#!/bin/bash
make clean
make 
sudo insmod hello.ko
sudo rmmod hello.ko
dmesg | tail -2
