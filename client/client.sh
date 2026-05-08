#!/bin/bash

/StarCraftII/Versions/Base75689/SC2_x64 -listen 0.0.0.0 -port 10001 &

node play.js &

wait
