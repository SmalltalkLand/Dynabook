THEDISPLAY=${RANDOM}2
x11docker --display $THEDISPLAY \
    --clean-xhost \
    --runfromhost 'DISPLAY=$THEDISPLAY launchy &' \
    --share /tmp/.X11-unix \
    --wayland \
    --weston \
    --gpu \
    x11docker/xwayland
