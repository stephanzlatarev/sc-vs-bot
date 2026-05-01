#!/bin/bash

socat TCP-LISTEN:10044,bind=0.0.0.0,fork,reuseaddr TCP:127.0.0.1:10004 &
node host.js
