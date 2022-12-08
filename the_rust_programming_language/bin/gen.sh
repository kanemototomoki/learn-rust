#!/bin/bash

if [ -z "$1" ]; then
  echo "番号を入力してね！"
  exit 0
fi

cargo new ch$1
touch ch$1/readme.md
code ch$1
