#!/bin/bash

if [ -z "$CLOUDFLARE_TOKEN" ]; then
    echo "CLOUDFLARE_TOKEN is not set"
    exit 1
fi

cloudflared tunnel run --token $CLOUDFLARE_TOKEN
