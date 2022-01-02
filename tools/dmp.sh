#!/bin/bash

riscv64-unknown-linux-gnu-gcc -c main.o main.c
riscv64-unknown-linux-gnu-objdump -d main.o > main.dump
