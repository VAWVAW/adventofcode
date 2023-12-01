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

nix-shell -p librewolf --run "librewolf $(nix-shell -p rustup --run "rustup doc --path --std")" 2>/dev/null &

nvim $DATE/src/main.rs
