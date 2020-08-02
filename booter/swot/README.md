# Swot

A Rust port of the `wlroots` `tinywl` Wayland compositor example.

For more info, see <https://wiki.alopex.li/WaylandAndRust>

# Building

Install deps:

 * apt install llvm-dev libclang-dev clang libwlroots-dev

wlroots seems to want an "xdg-shell-protocol.h" for some reason, see
https://github.com/swaywm/wlroots/blob/master/tinywl/Makefile
for some explaination of it.  Essentially you have to generate a header
from the Wayland XML specifications:

```
WAYLAND_PROTOCOLS=$(shell pkg-config --variable=pkgdatadir wayland-protocols)
WAYLAND_SCANNER=$(shell pkg-config --variable=wayland_scanner wayland-scanner)
LIBS=\
	 $(shell pkg-config --cflags --libs wlroots) \
	 $(shell pkg-config --cflags --libs wayland-server) \
	 $(shell pkg-config --cflags --libs xkbcommon)

# wayland-scanner is a tool which generates C headers and rigging for Wayland
# protocols, which are specified in XML. wlroots requires you to rig these up
# to your build system yourself and provide them in the include path.
xdg-shell-protocol.h:
	$(WAYLAND_SCANNER) server-header \
		$(WAYLAND_PROTOCOLS)/stable/xdg-shell/xdg-shell.xml $@

xdg-shell-protocol.c: xdg-shell-protocol.h
	$(WAYLAND_SCANNER) private-code \
		$(WAYLAND_PROTOCOLS)/stable/xdg-shell/xdg-shell.xml $@

```

To do this on Debian I had to:

```
/usr/bin/wayland-scanner private-code /usr/share/wayland-protocols/stable/xdg-shell/xdg-shell.xml xdg-shell-protocol.c
/usr/bin/wayland-scanner server-header /usr/share/wayland-protocols/stable/xdg-shell/xdg-shell.xml
```

This is tedious enough to do as part of a build.rs that for now I don't
bother.

# License

MIT
