#!/bin/sh

if [[ $(/usr/bin/id -u) -ne 0 ]]; then
  echo "uPico installer is not running as root. Try using sudo."
  exit 2
fi

cp upico /usr/local/bin/
cp upico.service /etc/systemd/system/
systemctl enable upico
systemctl start upico
echo "uPico installed."