docker rm -f bob2
docker run --name bob2 -v /var/run/docker.sock:/var/run/docker.sock "$@" viomckinney/bob:latest
