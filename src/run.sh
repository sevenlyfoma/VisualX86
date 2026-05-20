docker build -t my-rust-node-app .
docker run --name my-running-app --publish 8080:8080 my-rust-node-app