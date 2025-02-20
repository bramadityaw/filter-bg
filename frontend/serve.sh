#!/usr/bin/env bash

set -e
cert_path="$HOME/.localhost/localhost"
http-server -S -C "$cert_path.pem" -K "$cert_path-key.pem"
