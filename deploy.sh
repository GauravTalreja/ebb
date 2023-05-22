#!/bin/bash
git pull
sudo pkill server
perseus deploy
# Styles need to be copied over to the pkg
cp dist/tailwind.css pkg/dist/
sudo bash -c './pkg/server &'
