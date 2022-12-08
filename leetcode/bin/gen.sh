#!/bin/bash

if [ -z "$1" ]; then
  echo "問題番号を入力してね！"
  exit 0
fi

cargo new p_$1
touch p_$1/readme.md
code p_$1
