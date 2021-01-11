#!/bin/bash

# Remember to run this script from parent directory like this:
#   docker/run_docker.sh

GID=$(id -g $USER)
UID=$(id -u $USER)



docker run -it \
 --volume "${PWD}":"${PWD}" \
 -p 3000:3000 \
 --privileged \
 --name first_pwa_node.v.1.0 first_pwa_node:1.0 \
 bash # -c "sudo chown -R $UID:$GID /home/$USER;bash"

