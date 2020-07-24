sudo docker build -t smalltalkland/squeak .
sudo docker run --rm -it -v $PWD:/app smalltalkland/squeak bash -c "/sqbin/squeak $@"
