#!/bin/sh

cp -f upico /usr/local/bin/
cp -f upico.service /etc/systemd/system/
systemctl enable upico
systemctl start upico
echo "uPico installed"
