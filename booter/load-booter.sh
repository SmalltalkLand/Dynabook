mkinitcpio -c ./mkinitcpio.conf -g ./boot/initramfs.img
mkdir res
cd res
lsinitcpio -x ../boot/initramfs.img

find -mindepth 1 -printf '%P\0' | LANG=C bsdcpio -0 -o -H newc --quiet | gzip > ../boot/initramfs-linux.img
