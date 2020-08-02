//! TODO: Investigate using wayland-sys for raw wayland, since it's generated from
//! the Wayland protocol definitions.  It may even be possible to make the wayland-server
//! crate work, which is safer bindings atop it, but I don't know how much wlroots
//! may or may not like either of those.  So we will see.
//!
//! TODO: References instead of pointers where possible.
//! Remove wl_list where possible.
//! Order functions properly.
//! Go through and look at other TODO's

use std::ffi;
use std::mem;
use std::pin::Pin;
use std::process;
use std::ptr;
use std::time::{Duration, Instant};

mod wlr;

use wlr::*;

use lazy_static::lazy_static;
lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CursorMode {
    Passthrough,
    Move,
    Resize,
}

#[repr(C)]
pub struct Server {
    display: *mut wl_display,
    backend: *mut wlr_backend,
    renderer: *mut wlr_renderer,

    xdg_shell: *mut wlr_xdg_shell,
    new_xdg_surface: wl_listener,
    /// These have to be boxed because they're full of
    /// things that pointers aim at, so we can't move them.
    views: Vec<Pin<Box<View>>>,
    views_idx: usize,

    cursor: *mut wlr_cursor,
    cursor_mgr: *mut wlr_xcursor_manager,
    cursor_motion: wl_listener,
    cursor_motion_absolute: wl_listener,
    cursor_button: wl_listener,
    cursor_axis: wl_listener,
    cursor_frame: wl_listener,

    seat: *mut wlr_seat,
    new_input: wl_listener,
    request_cursor: wl_listener,
    keyboards: Vec<Pin<Box<Keyboard>>>,
    cursor_mode: CursorMode,
    grabbed_view: *mut View,
    grab_x: f64,
    grab_y: f64,
    grab_width: i32,
    grab_height: i32,
    resize_edges: u32,

    output_layout: *mut wlr_output_layout,
    outputs: Vec<Pin<Box<Output>>>,
    new_output: wl_listener,
}

#[repr(C)]
pub struct View {
    link: wl_list,
    server: *mut Server,
    xdg_surface: *mut wlr_xdg_surface,
    map: wl_listener,
    unmap: wl_listener,
    destroy: wl_listener,
    request_move: wl_listener,
    request_resize: wl_listener,
    mapped: bool,
    x: i32,
    y: i32,
}

#[repr(C)]
pub struct Output {
    link: wl_list,
    server: *mut Server,
    wlr_output: *mut wlr_output,
    frame: wl_listener,
}

#[repr(C)]
pub struct Keyboard {
    server: *mut Server,
    device: *mut wlr_input_device,
    modifiers: wl_listener,
    key: wl_listener,
}

struct RenderData {
    output: *mut wlr_output,
    renderer: *mut wlr_renderer,
    view: *mut View,
    when: Duration,
}

/// Reimplementation of wl_container_of.  Horribly unsafe.
/// Probably unsound.  Can't use it on structs that aren't repr(C).
/// ...this might actually be impossible in defined Rust.
/// No, it's possible, but less easy than you think.  So let's use
/// the `memoffset` crate to do the hard part for us.
/// Note this returns a mut ptr, so if that's not what you want,
/// turn it to something else first thing.
macro_rules! conjure_heckin_ptr {
    ( $ptr: expr, $sample: path, $member: tt ) => {{
        use memoffset;
        let internal_ptr = $ptr as usize;
        let offset = memoffset::offset_of!($sample, $member);
        let pointer_to_parent = internal_ptr - offset;
        // SHAZAM!
        pointer_to_parent as *mut $sample
    }};
}

/// This is a redefinition of the `static inline` definition
/// in wayland-server-core.h.  Bindgen doesn't seem to like
/// such a pathological thing, but it's a one-liner so we'll
/// just do it here.  Hopefully we don't need many of them.
///
/// wayland-sys is theoretically the Right solution to this.
unsafe fn wl_signal_add(signal: *mut wl_signal, listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link)
}

unsafe fn focus_view(view: &mut View, surface: &mut wlr_surface) {
    // Note this only deals with keyboard focus
    let server = view.server;
    let seat = (*server).seat;
    let prev_surface = (*seat).keyboard_state.focused_surface;
    if prev_surface == surface {
        // Don't re-focus already focussed surface
        return;
    }
    // TODO: Make prev_surface an Option non-nullable pointer?
    if prev_surface != ptr::null_mut() {
        let previous = wlr_xdg_surface_from_wlr_surface((*seat).keyboard_state.focused_surface);
        wlr_xdg_toplevel_set_activated(previous, false);
    }

    let keyboard = wlr_seat_get_keyboard(seat);
    // move view to the front
    if let Some((idx, _)) = (*(*view).server)
        .views
        .iter()
        .enumerate()
        .find(|(_idx, v)| &*(v.as_ref()) as *const View == view)
    {
        let v = (*server).views.remove(idx);
        (*server).views.insert(0, v);
    }
    wlr_xdg_toplevel_set_activated((*view).xdg_surface, true);
    wlr_seat_keyboard_notify_enter(
        seat,
        (*(*view).xdg_surface).surface,
        (*keyboard).keycodes.as_mut_ptr(),
        (*keyboard).num_keycodes,
        &mut (*keyboard).modifiers,
    );
}

unsafe extern "C" fn keyboard_handle_modifiers(
    listener: *mut wl_listener,
    _data: *mut ffi::c_void,
) {
    let keyboard = &mut *conjure_heckin_ptr!(listener, Keyboard, modifiers);
    wlr_seat_set_keyboard((*keyboard.server).seat, (*keyboard).device);
    wlr_seat_keyboard_notify_modifiers(
        (*keyboard.server).seat,
        &mut (*(*keyboard.device).__bindgen_anon_1.keyboard).modifiers,
    );
}

unsafe fn handle_keybinding(server: &mut Server, sym: xkb_keysym_t) -> bool {
    #[allow(non_upper_case_globals)]
    match sym {
        XKB_KEY_Escape => {
            wl_display_terminate(server.display);
        }
        XKB_KEY_F1 => {
            // Cycle to next view.
            if server.views.len() > 2 {
                server.views_idx = (server.views_idx + 1) % server.views.len();
                let current_view = &mut server.views[server.views_idx];
                // TODO: This lifetime-break-with-pointer
                // is TOTALLY FINE.  Nothing to see here, move along.
                let current_surface = (*current_view.xdg_surface).surface;
                focus_view(current_view, &mut *current_surface);
            }
        }
        _ => {
            return false;
        }
    }
    true
}

unsafe extern "C" fn keyboard_handle_key(listener: *mut wl_listener, data: *mut ffi::c_void) {
    let keyboard = &mut *(conjure_heckin_ptr!(listener, Keyboard, key));
    let server = keyboard.server;
    let event = &*(data as *const wlr_event_keyboard_key);
    let seat = (*server).seat;
    let kb = (*keyboard.device).__bindgen_anon_1.keyboard;

    // Translate libinput keycode -> xkbcommon
    let keycode: u32 = event.keycode + 8;
    // This needs a pointer-to-pointer, yay
    let syms: *mut *const xkb_keysym_t = &mut ptr::null();
    let nsyms = xkb_state_key_get_syms((*kb).xkb_state, keycode, syms);
    let mut handled = false;
    let modifiers = wlr_keyboard_get_modifiers(kb);
    if modifiers & (wlr_keyboard_modifier_WLR_MODIFIER_ALT) != 0
        && event.state == wlr_key_state_WLR_KEY_PRESSED
    {
        for i in 0..nsyms {
            // TODO: Figure out what the heck is up with syms, actually
            handled = handle_keybinding(&mut *server, **syms.offset(i as isize));
        }
    }

    if !handled {
        // Pass it along to the client
        wlr_seat_set_keyboard(seat, keyboard.device);
        wlr_seat_keyboard_notify_key(seat, event.time_msec, event.keycode, event.state);
    }
}

unsafe fn server_new_keyboard(server: *mut Server, device: *mut wlr_input_device) {
    let mut modifiers: wl_listener = mem::zeroed();
    modifiers.notify = Some(keyboard_handle_modifiers);

    let mut key: wl_listener = mem::zeroed();
    key.notify = Some(keyboard_handle_key);

    let mut keyboard = Box::pin(Keyboard {
        server,
        device,
        modifiers,
        key,
    });

    // Prepare a default keymap and assign it to the keyboard
    let mut rules: xkb_rule_names = mem::zeroed();
    let context = xkb_context_new(xkb_context_flags_XKB_CONTEXT_NO_FLAGS);
    // Apparently xkb_map_new_from_names got renamed at some point?
    let keymap = xkb_keymap_new_from_names(
        context,
        &mut rules,
        xkb_keymap_compile_flags_XKB_KEYMAP_COMPILE_NO_FLAGS,
    );

    wlr_keyboard_set_keymap((*device).__bindgen_anon_1.keyboard, keymap);
    xkb_keymap_unref(keymap);
    xkb_context_unref(context);
    wlr_keyboard_set_repeat_info((*device).__bindgen_anon_1.keyboard, 25, 600);

    wl_signal_add(
        &mut (*(*device).__bindgen_anon_1.keyboard).events.modifiers,
        &mut (*keyboard).modifiers,
    );
    wl_signal_add(
        &mut (*(*device).__bindgen_anon_1.keyboard).events.key,
        &mut (*keyboard).key,
    );
    wlr_seat_set_keyboard((*server).seat, device);
    (*server).keyboards.push(keyboard);
}

unsafe fn server_new_pointer(server: *mut Server, device: *mut wlr_input_device) {
    wlr_cursor_attach_input_device((*server).cursor, device);
}

unsafe extern "C" fn server_new_input(listener: *mut wl_listener, data: *mut ffi::c_void) {
    let server = &mut *(conjure_heckin_ptr!(listener, Server, new_input));
    let device = &mut *(data as *mut wlr_input_device);
    #[allow(non_upper_case_globals)]
    match device.type_ {
        wlr_input_device_type_WLR_INPUT_DEVICE_KEYBOARD => server_new_keyboard(server, device),
        wlr_input_device_type_WLR_INPUT_DEVICE_POINTER => server_new_pointer(server, device),
        _ => (),
    }
    // Let the wlr_seat know what our capabilities are,
    // we assume we always have a pointer.
    let caps: u32 = if server.keyboards.is_empty() {
        wl_seat_capability_WL_SEAT_CAPABILITY_POINTER
    } else {
        wl_seat_capability_WL_SEAT_CAPABILITY_POINTER
            | wl_seat_capability_WL_SEAT_CAPABILITY_KEYBOARD
    };
    wlr_seat_set_capabilities(server.seat, caps);
}

unsafe fn begin_interactive(view: &mut View, mode: CursorMode, edges: u32) {
    let server = &mut *view.server;
    let focused_surface = (*server.seat).pointer_state.focused_surface;
    if (*view.xdg_surface).surface != focused_surface {
        // Unfocused clients cannot request move/resize
        return;
    }
    server.grabbed_view = view;
    server.cursor_mode = mode;
    let mut geo_box = mem::zeroed();
    wlr_xdg_surface_get_geometry(view.xdg_surface, &mut geo_box);
    if mode == CursorMode::Move {
        server.grab_x = (*server.cursor).x - (view.x as f64);
        server.grab_y = (*server.cursor).y - (view.y as f64);
    } else {
        server.grab_x = (*server.cursor).x + (geo_box.x as f64);
        server.grab_y = (*server.cursor).y + (geo_box.y as f64);
    }
    server.grab_width = geo_box.width;
    server.grab_height = geo_box.height;
    server.resize_edges = edges;
}

unsafe extern "C" fn seat_request_cursor(listener: *mut wl_listener, data: *mut ffi::c_void) {
    let server = &mut *(conjure_heckin_ptr!(listener, Server, request_cursor));
    let event = &mut *(data as *mut wlr_seat_pointer_request_set_cursor_event);
    let focused_client = (*server.seat).pointer_state.focused_client;
    // Check that the client we got it from is the one that actually has focus
    if focused_client == event.seat_client {
        wlr_cursor_set_surface(
            server.cursor,
            event.surface,
            event.hotspot_x,
            event.hotspot_y,
        );
    }
}

unsafe fn view_at(view: &mut View, lx: f64, ly: f64) -> Option<(*mut wlr_surface, f64, f64)> {
    let view_sx = lx - (view.x as f64);
    let view_sy = ly - (view.y as f64);
    //let state = &mut (*(*view.xdg_surface).surface).current;
    let sx = &mut 0.0;
    let sy = &mut 0.0;
    let surf = wlr_xdg_surface_surface_at(view.xdg_surface, view_sx, view_sy, sx, sy);
    if surf != ptr::null_mut() {
        Some((surf, *sx, *sy))
    } else {
        None
    }
}

unsafe fn desktop_view_at(
    server: &mut Server,
    lx: f64,
    ly: f64,
) -> Option<(*mut View, *mut wlr_surface, f64, f64)> {
    for view in server.views.iter_mut() {
        match view_at(&mut *view, lx, ly) {
            None => (),
            Some((s, sx, sy)) => {
                return Some(((&mut **view) as *mut View, s, sx, sy));
            }
        }
    }
    None
}

unsafe fn process_cursor_move(server: &mut Server, _time: u32) {
    (*server.grabbed_view).x = ((*server.cursor).x - server.grab_x) as i32;
    (*server.grabbed_view).y = ((*server.cursor).y - server.grab_y) as i32;
}

/// Resize the grabbed view
unsafe fn process_cursor_resize(server: &mut Server, _time: u32) {
    //TODO: Verify this is all actually right.
    let view = server.grabbed_view;
    let dx = (*server.cursor).x - server.grab_x;
    let dy = (*server.cursor).y - server.grab_y;
    let mut x: f64 = (*view).x as _;
    let mut y: f64 = (*view).y as _;
    let mut width: f64 = server.grab_width as _;
    let mut height: f64 = server.grab_height as _;

    if server.resize_edges & wlr_edges_WLR_EDGE_TOP != 0 {
        y = server.grab_y + dy;
        height -= dy;
        if height < 1.0 {
            y += height
        }
    } else if server.resize_edges & wlr_edges_WLR_EDGE_BOTTOM != 0 {
        height += dy;
    }

    if server.resize_edges & wlr_edges_WLR_EDGE_LEFT != 0 {
        x = server.grab_x + dx;
        width -= dx;
        if width < 1.0 {
            x += width;
        } else if server.resize_edges & wlr_edges_WLR_EDGE_RIGHT != 0 {
            width += dx
        }
    }
    (*view).x = x as i32;
    (*view).y = y as i32;
    wlr_xdg_toplevel_set_size((*view).xdg_surface, width as u32, height as u32);
}

unsafe fn process_cursor_motion(server: &mut Server, time: u32) {
    if server.cursor_mode == CursorMode::Move {
        process_cursor_move(server, time);
        return;
    } else if server.cursor_mode == CursorMode::Resize {
        process_cursor_resize(server, time);
        return;
    }

    // otherwise, find the view under the pointer and send the event along.
    let seat = server.seat;
    if let Some((_view, surface, sx, sy)) =
        desktop_view_at(server, (*server.cursor).x, (*server.cursor).y)
    {
        if surface != ptr::null_mut() {
            let focus_changed = (*seat).pointer_state.focused_surface != surface;
            wlr_seat_pointer_notify_enter(seat, surface, sx, sy);
            if !focus_changed {
                wlr_seat_pointer_notify_motion(seat, time, sx, sy);
            }
        } else {
            wlr_seat_pointer_clear_focus(seat);
        }
    } else {
        // There's no view under the cursor, set default.
        //let focus_changed = (*seat).pointer_state.focused_surface != surface;
        let cursor_name = &ffi::CStr::from_bytes_with_nul_unchecked(b"left_ptr\0");
        wlr_xcursor_manager_set_cursor_image(
            server.cursor_mgr,
            cursor_name.as_ptr(),
            server.cursor,
        );
    }
}

unsafe extern "C" fn server_cursor_motion(listener: *mut wl_listener, data: *mut ffi::c_void) {
    let server = &mut *conjure_heckin_ptr!(listener, Server, cursor_motion);
    let event = &mut *(data as *mut wlr_event_pointer_motion);

    wlr_cursor_move(server.cursor, event.device, event.delta_x, event.delta_y);
    process_cursor_motion(server, event.time_msec);
}

unsafe extern "C" fn render_surface(
    surface: *mut wlr_surface,
    sx: i32,
    sy: i32,
    data: *mut ffi::c_void,
) {
    let rdata = &mut *(data as *mut RenderData);
    let view = rdata.view;
    let output = rdata.output;

    let texture = wlr_surface_get_texture(surface);
    if texture.is_null() {
        return;
    }

    let ox = &mut 0.0;
    let oy = &mut 0.0;
    wlr_output_layout_output_coords((*(*view).server).output_layout, output, ox, oy);
    *ox += ((*view).x + sx) as f64;
    *oy += ((*view).y + sy) as f64;

    let bx = wlr_box {
        x: (*ox * (*output).scale as f64) as i32,
        y: (*oy * (*output).scale as f64) as i32,
        width: ((*surface).current.width as f32 * (*output).scale) as i32,
        height: ((*surface).current.height as f32 * (*output).scale) as i32,
    };

    let matrix = [0.0; 9].as_mut_ptr();
    let transform = wlr_output_transform_invert((*surface).current.transform);
    wlr_matrix_project_box(
        matrix,
        &bx,
        transform,
        0.0,
        (*output).transform_matrix.as_mut_ptr(),
    );
    wlr_render_texture_with_matrix(rdata.renderer, texture, matrix, 1.0);
    let ts = &mut timespec {
        tv_sec: rdata.when.as_secs() as i64,
        tv_nsec: rdata.when.subsec_nanos() as i64,
    };
    wlr_surface_send_frame_done(surface, ts);
}

unsafe extern "C" fn output_frame(listener: *mut wl_listener, _data: *mut ffi::c_void) {
    let output = &mut *(conjure_heckin_ptr!(listener, Output, frame));
    let renderer = (*output.server).renderer;
    if !wlr_output_attach_render(output.wlr_output, ptr::null_mut()) {
        return;
    }
    let width = &mut 0;
    let height = &mut 0;
    wlr_output_effective_resolution(output.wlr_output, width, height);
    wlr_renderer_begin(renderer, *width, *height);
    let color = [0.1, 0.2, 0.3, 1.0];
    wlr_renderer_clear(renderer, color.as_ptr());

    // render back to front
    for view in (*output.server).views.iter_mut().rev() {
        if !view.mapped {
            continue;
        }
        let mut rdata = RenderData {
            output: output.wlr_output,
            view: &mut **view,
            renderer: renderer,
            when: START_TIME.elapsed(),
        };
        wlr_xdg_surface_for_each_surface(
            (*view).xdg_surface,
            Some(render_surface),
            (&mut rdata) as *mut _ as *mut ffi::c_void,
        );
    }

    wlr_output_render_software_cursors(output.wlr_output, ptr::null_mut());
    wlr_renderer_end(renderer);
    wlr_output_commit(output.wlr_output);
}

/// Callback for getting a new output thingamathang.
unsafe extern "C" fn server_new_output(listener: *mut wl_listener, data: *mut ffi::c_void) {
    let server = &mut *(conjure_heckin_ptr!(listener, Server, new_output));
    let wlr_output = &mut *(data as *mut wlr_output);

    if !(wl_list_empty(&wlr_output.modes) != 0) {
        let mode = wlr_output_preferred_mode(wlr_output);
        wlr_output_set_mode(wlr_output, mode);
        wlr_output_enable(wlr_output, true);
        if !wlr_output_commit(wlr_output) {
            return;
        }
    }

    let mut output: Pin<Box<Output>> = Box::pin(mem::zeroed());
    output.wlr_output = wlr_output;
    output.server = server;
    output.frame.notify = Some(output_frame);
    wl_signal_add(&mut wlr_output.events.frame, &mut output.frame);
    server.outputs.push(output);

    wlr_output_layout_add_auto(server.output_layout, wlr_output);
}

unsafe extern "C" fn xdg_surface_map(listener: *mut wl_listener, _data: *mut ffi::c_void) {
    // TODO: This is Hard to turn into a reference because then you get a double-borrow
    // in focus_view(), and that is not trivial to untangle.
    let view = conjure_heckin_ptr!(listener, View, map);
    (*view).mapped = true;
    focus_view(&mut *view, &mut *(*(*view).xdg_surface).surface);
}

unsafe extern "C" fn xdg_surface_unmap(listener: *mut wl_listener, _data: *mut ffi::c_void) {
    let view = &mut *conjure_heckin_ptr!(listener, View, unmap);
    view.mapped = false;
}

/// TODO: This is kinda awful, since we reach through the View's parent pointer
/// to the Server to remove the View itself from said Server.  If the Server moves
/// or the View moves or such this will no longer work.
/// In related news, I'm not sure it's safe to make `view` a reference rather than
/// a raw pointer here, due to said parent pointer.
unsafe extern "C" fn xdg_surface_destroy(listener: *mut wl_listener, _data: *mut ffi::c_void) {
    let view = &mut *conjure_heckin_ptr!(listener, View, destroy);
    // TODO: Improve this by having the View store its idx?
    if let Some((idx, _)) = (*view.server)
        .views
        .iter()
        .enumerate()
        .find(|(_idx, v)| &*(v.as_ref()) as *const View == view)
    {
        // TODO: This *shouldn't* move all the other View's, verify.
        (*view.server).views.remove(idx);
    } else {
        panic!("Should never happen.");
    }
}

unsafe extern "C" fn server_cursor_motion_absolute(
    listener: *mut wl_listener,
    data: *mut ffi::c_void,
) {
    let server = &mut *conjure_heckin_ptr!(listener, Server, cursor_motion_absolute);
    let event = &mut *(data as *mut wlr_event_pointer_motion_absolute);
    wlr_cursor_warp_absolute(server.cursor, event.device, event.x, event.y);
    process_cursor_motion(server, event.time_msec);
}

unsafe extern "C" fn server_cursor_button(listener: *mut wl_listener, data: *mut ffi::c_void) {
    let server = &mut *conjure_heckin_ptr!(listener, Server, cursor_button);
    let event = &mut *(data as *mut wlr_event_pointer_button);
    wlr_seat_pointer_notify_button(server.seat, event.time_msec, event.button, event.state);
    if let Some((view, surface, _sx, _sy)) =
        desktop_view_at(server, (*server.cursor).x, (*server.cursor).y)
    {
        if event.state == wlr_button_state_WLR_BUTTON_RELEASED {
            server.cursor_mode = CursorMode::Passthrough;
        } else {
            focus_view(&mut *view, &mut *surface);
        }
    }
}

unsafe extern "C" fn server_cursor_axis(listener: *mut wl_listener, data: *mut ffi::c_void) {
    let server = &mut *conjure_heckin_ptr!(listener, Server, cursor_axis);
    let event = &mut *(data as *mut wlr_event_pointer_axis);
    wlr_seat_pointer_notify_axis(
        server.seat,
        event.time_msec,
        event.orientation,
        event.delta,
        event.delta_discrete,
        event.source,
    )
}

unsafe extern "C" fn server_cursor_frame(listener: *mut wl_listener, _data: *mut ffi::c_void) {
    let server = &mut *conjure_heckin_ptr!(listener, Server, cursor_frame);
    wlr_seat_pointer_notify_frame(server.seat);
}

unsafe extern "C" fn xdg_toplevel_request_move(
    listener: *mut wl_listener,
    _data: *mut ffi::c_void,
) {
    let view = &mut *conjure_heckin_ptr!(listener, View, request_move);
    begin_interactive(view, CursorMode::Move, 0);
}

unsafe extern "C" fn xdg_toplevel_request_resize(
    listener: *mut wl_listener,
    data: *mut ffi::c_void,
) {
    dbg!("XDG surface requested resize!");
    let view = &mut *conjure_heckin_ptr!(listener, View, request_resize);
    let event = &mut *(data as *mut wlr_xdg_toplevel_resize_event);
    begin_interactive(view, CursorMode::Resize, event.edges);
}

/// Handle getting a new xdg surface, such as a new application window.
unsafe extern "C" fn server_new_xdg_surface(listener: *mut wl_listener, data: *mut ffi::c_void) {
    // These stay as raw pointers instead of references because we need to store them
    // as such in the View.
    let server = conjure_heckin_ptr!(listener, Server, new_xdg_surface);
    let xdg_surface = data as *mut wlr_xdg_surface;
    dbg!(xdg_surface);
    if (*xdg_surface).role != wlr_xdg_surface_role_WLR_XDG_SURFACE_ROLE_TOPLEVEL {
        return;
    }

    let mut map: wl_listener = mem::zeroed();
    map.notify = Some(xdg_surface_map);
    let mut unmap: wl_listener = mem::zeroed();
    unmap.notify = Some(xdg_surface_unmap);
    let mut destroy: wl_listener = mem::zeroed();
    destroy.notify = Some(xdg_surface_destroy);

    let mut request_move: wl_listener = mem::zeroed();
    request_move.notify = Some(xdg_toplevel_request_move);

    let mut request_resize: wl_listener = mem::zeroed();
    request_resize.notify = Some(xdg_toplevel_request_resize);

    let link = mem::zeroed();
    // Gotta box this so it stays on the heap...
    let mut view = Box::pin(View {
        link,
        // TODO: aiee, parent pointer.  This is hazardous in Rust.
        server,
        xdg_surface,
        map,
        unmap,
        destroy,
        request_move,
        request_resize,

        mapped: false,
        x: 0,
        y: 0,
    });
    dbg!(view.xdg_surface);
    wl_signal_add(&mut (*xdg_surface).events.map, &mut view.map);
    wl_signal_add(&mut (*xdg_surface).events.unmap, &mut view.unmap);
    wl_signal_add(&mut (*xdg_surface).events.destroy, &mut view.destroy);

    // bindgen doesn't handle anonymous unions terribly gracefully,
    // but anonymous unions are not terribly graceful anyway, so.
    let toplevel = (*xdg_surface).__bindgen_anon_1.toplevel;
    wl_signal_add(&mut (*toplevel).events.request_move, &mut view.request_move);
    wl_signal_add(
        &mut (*toplevel).events.request_resize,
        &mut view.request_resize,
    );

    // Add to the list of views
    (*server).views.push(view);
}

fn main() {
    lazy_static::initialize(&START_TIME);
    // TODO: structopt or such, take a startup command.
    //let startup_command = "alacritty";
    unsafe {
        wlr_log_init(wlr_log_importance_WLR_DEBUG, None);

        let display = wl_display_create();
        let backend = wlr_backend_autocreate(display, None);
        let renderer = wlr_backend_get_renderer(backend);

        let views = Vec::new();
        let xdg_shell = wlr_xdg_shell_create(display);
        let new_xdg_surface = mem::zeroed();

        let cursor = wlr_cursor_create();
        let cursor_axis = mem::zeroed();
        let cursor_button = mem::zeroed();
        let cursor_frame = mem::zeroed();
        let cursor_motion = mem::zeroed();
        let cursor_motion_absolute = mem::zeroed();
        let cursor_mgr = wlr_xcursor_manager_create(ptr::null(), 24);

        let seat = ptr::null_mut();
        let request_cursor = mem::zeroed();
        let new_input = mem::zeroed();

        let output_layout = wlr_output_layout_create();
        let new_output = mem::zeroed();

        let mut server = Server {
            display,
            backend,
            renderer,

            views,
            views_idx: 0,
            xdg_shell,
            new_xdg_surface,

            cursor,
            cursor_axis,
            cursor_button,
            cursor_frame,
            cursor_motion,
            cursor_motion_absolute,
            cursor_mgr,

            seat,
            keyboards: Vec::new(),
            new_input,
            request_cursor,
            cursor_mode: CursorMode::Passthrough,
            grab_height: 0,
            grab_width: 0,
            grab_x: 0.0,
            grab_y: 0.0,
            resize_edges: 0,
            grabbed_view: ptr::null_mut(),

            output_layout,
            outputs: Vec::new(),
            new_output,
        };

        wlr_renderer_init_wl_display(server.renderer, server.display);
        wlr_compositor_create(server.display, server.renderer);
        wlr_data_device_manager_create(server.display);

        server.new_output.notify = Some(server_new_output);
        wl_signal_add(
            &mut (*server.backend).events.new_output,
            &mut server.new_output,
        );

        server.new_xdg_surface.notify = Some(server_new_xdg_surface);
        wl_signal_add(
            &mut (*server.xdg_shell).events.new_surface,
            &mut server.new_xdg_surface,
        );

        wlr_cursor_attach_output_layout(server.cursor, server.output_layout);
        wlr_xcursor_manager_load(server.cursor_mgr, 1.0);

        // Hook up all the mouse event handlers
        server.cursor_motion.notify = Some(server_cursor_motion);
        wl_signal_add(
            &mut (*server.cursor).events.motion,
            &mut server.cursor_motion,
        );
        server.cursor_motion_absolute.notify = Some(server_cursor_motion_absolute);
        wl_signal_add(
            &mut (*server.cursor).events.motion_absolute,
            &mut server.cursor_motion_absolute,
        );
        server.cursor_button.notify = Some(server_cursor_button);
        wl_signal_add(
            &mut (*server.cursor).events.button,
            &mut server.cursor_button,
        );
        server.cursor_axis.notify = Some(server_cursor_axis);
        wl_signal_add(&mut (*server.cursor).events.axis, &mut server.cursor_axis);
        server.cursor_frame.notify = Some(server_cursor_frame);
        wl_signal_add(&mut (*server.cursor).events.frame, &mut server.cursor_frame);

        // Configure seat
        server.new_input.notify = Some(server_new_input);
        wl_signal_add(
            &mut (*server.backend).events.new_input,
            &mut server.new_input,
        );

        let seat_name = ffi::CString::new("seat0").expect("Never happens");
        server.seat = wlr_seat_create(server.display, seat_name.as_ptr());

        server.request_cursor.notify = Some(seat_request_cursor);
        wl_signal_add(&mut (*server.cursor).events.frame, &mut server.cursor_frame);

        // Add unix socket to the wayland display
        let socket = wl_display_add_socket_auto(server.display);
        if socket.is_null() {
            wlr_backend_destroy(server.backend);
            process::exit(1);
        }
        let socket_str: &str = ffi::CStr::from_ptr(socket).to_str().expect("Can't happen");

        // Start backend
        if !wlr_backend_start(server.backend) {
            wlr_backend_destroy(server.backend);
            wl_display_destroy(server.display);
        }

        // Set the env var and run the startup command
        // TODO: run command eventually
        std::env::set_var("WAYLAND_DISPLAY", socket_str);
        // Run event loop
        let log_str = ffi::CString::new("Running Wayland compositor on display %s").unwrap();
        // FUCKING MACROS
        //wlr_log(wlr_log_importance_WLR_INFO, Some(log_str.as_ptr()));
        _wlr_log(wlr_log_importance_WLR_INFO, log_str.as_ptr(), socket);
        wl_display_run(server.display);

        // Cleanup once wl_display_run() returns
        wl_display_destroy_clients(server.display);
        wl_display_destroy(server.display);
    }
}
