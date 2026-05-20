docker build -t cs4099-2 . --file DockerfileQuick

docker run --name mv54_cs4099 --publish 24508:24508 cs4099-2

docker rm mv54_cs4099

docker image rm cs4099-2