#!/bin/bash
cat one.txt | awk 'BEGIN {count = 0;} {if ($0 == ""){print(count); count = 0} else {count = count + $0}}' | sort -n | tail -n 3
