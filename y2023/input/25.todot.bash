#!/usr/bin/env bash

printf 'digraph {\n'
while read -r v val; do
  for va in $val; do
    printf '  %s -> %s [dir=none]\n' "$v" "$va"
  done
done < <(sed 's/://' "${1:-y2023/input/25}")
printf '}\n'
