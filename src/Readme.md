To run the server use

./run.sh

To access the web page go to 

http://127.0.0.1:24508

If you want to change the port change the port const in /web/index.js

To remove all docker/podman containers after you are done use

./clean.sh


Try 
docker build -t my-rust-node-app .

docker run --name my-running-app --publish 8080:8080 my-rust-node-app

docker rm my-running-app 

docker image rm my-rust-node-app:latest 