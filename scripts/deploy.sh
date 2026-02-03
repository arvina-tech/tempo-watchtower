#!/bin/bash

HOST=${HOST:-"watchtower.temprano.io"}
REMOTE_USER=${REMOTE_USER:-"temprano"}
BASE_DIR=${BASE_DIR:-"/home/temprano"}

OUTPUT_FILE="$BASE_DIR/tempo-watchtower"

CURRENT_DIR=$(dirname $(realpath $0))

scp -q $CURRENT_DIR/install.sh $REMOTE_USER@$HOST:/tmp/install.sh

ssh $REMOTE_USER@$HOST bash << EOF
bash /tmp/install.sh --output $OUTPUT_FILE
supervisorctl restart tempo-watchtower
rm /tmp/install.sh
supervisorctl status tempo-watchtower
EOF
