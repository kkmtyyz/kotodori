#!/bin/bash

riscv32-unknown-linux-gnu-gcc -c main.o main.c
riscv32-unknown-linux-gnu-objdump -d main.o > main.dump
