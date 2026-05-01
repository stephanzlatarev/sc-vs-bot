#!/bin/bash

/StarCraftII/Versions/Base75689/SC2_x64 -listen 0.0.0.0 -port 10001 &

#socat TCP-LISTEN:10004,bind=0.0.0.0,fork,reuseaddr TCP:host.docker.internal:10044 &
socat TCP-LISTEN:10004,bind=0.0.0.0,fork,reuseaddr TCP:129.212.171.186:10044 &
node client.js &
wait
