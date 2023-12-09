#!/bin/bash

sudo chmod 666 /dev/ttyUSB*
sudo chmod 666 /dev/ttyACM*
ls -l /dev/ttyUSB*
ls -l /dev/ttyACM*

DEV="/dev/ttyUSB0"

clear
sudo stty -F $DEV raw echo 115200
sudo stty -F $DEV -a
cat $DEV # | hexdump -C

