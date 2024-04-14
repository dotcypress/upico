#!/bin/sh

cp -f upico /usr/local/bin/
cp -f upico.service /etc/systemd/system/
systemctl enable upico
systemctl start upico
echo 'SUBSYSTEM=="usb",ATTRS{idVendor}=="1209",ATTRS{idProduct}=="bc07",MODE="0660",GROUP="plugdev"' > /etc/udev/rules.d/50-upico-permissions.rules
udevadm control --reload-rules
echo "uPico installed"
