 VERSION=0.0.4

 docker build -t unexist/rust:$VERSION .circleci/images/primary/
 docker login
 docker push unexist/rust:$VERSION
