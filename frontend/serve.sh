#!/usr/bin/env bash

set -e
cert_path="$HOME/.localhost/127.0.0.1"
http-server -S -C "$cert_path.pem" -K "$cert_path-key.pem"
