#!/bin/sh

cp upico /usr/local/bin/
cp upico.service /etc/systemd/system/
systemctl enable upico
systemctl start upico
echo "uPico installed"
