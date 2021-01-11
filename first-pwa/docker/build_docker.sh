#!/bin/bash

# GID=$(id -g $USER)
# UID=$(id -u $USER)

# docker build --build-arg USER=$USER \
#              --build-arg GID=$GID \
#              --build-arg UID=$UID \
#              -t "first_pwa_node:1.0" . 


docker build -t "first_pwa_node:1.0" -f ./docker/Dockerfile . 
