bwrap --ro-bind / / --bind . . --dev /dev --proc /proc --bind /dev/disk /dev/disk/ ./squeak_base/squeak.sh $1
