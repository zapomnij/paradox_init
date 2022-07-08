#!/bin/sh

if [ ! "$(id -u)" = "0" ]; then
    echo "This script has to be run as root" > /dev/stderr
    exit 1
fi

cd "$(dirname ${0})"

set -e

mkdir -p /etc/init

install -m644 *.json /etc/init
rm /etc/init/endscript.json

install -m644 endscript.json /etc

install -m644 init.list /etc