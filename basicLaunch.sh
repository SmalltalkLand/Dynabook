mkdir -p /_st
mkfifo /app/squeak.sock
bwrap --bind / / --bind /app/squeak.sock /squeak.sock --bind /_st /parent --tmpfs /dev sh native/sq_runtime_socket.sh & 
bwrap --bind / / --bind /app/squeak.sock /squeak.sock --tmpfs /_st --tmpfs /dev --bind /dev/disk /dev/disk sh squeak/squeak.sh squeak/target.image
