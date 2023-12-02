#!/usr/bin/env sh

DATE=$(date +%d)

if [ ! -d "$DATE" ]; then
  echo "Creating new cargo project"
  cargo new --quiet --bin --vcs none --name "aoc$DATE" $DATE

  cp template.rs $DATE/src/main.rs
  touch $DATE/data.txt
  touch $DATE/data_test1.txt
  touch $DATE/data_test2.txt
fi

firefox /persist/home/vawvaw/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/std/index.html https://adventofcode.com/ 2>&1 >/dev/null&

nvim $DATE/src/main.rs
