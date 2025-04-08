#!/usr/bin/env bash

if [ "$#" -ne 1 ]; then
    printf "Illegal number of parameters\n"
    printf "USAGE: aoc.sh [1, 25]\n"
    exit 1
fi
if [ ! -f .cookie ]; then
    printf "Error: File '.cookie' not found.\n" >&2
    exit 1
fi
if [ ! -d ./inputs ]; then
    printf "Error: Folder './inputs' not found.\n" >&2
    exit 1
fi

day=$1
output_file=$(printf "./inputs/day%02d.input" "$1")
cookie=$(cat .cookie)

curl "https://adventofcode.com/2024/day/$day/input" \
    --compressed \
    -H "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0" \
    -H "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8" \
    -H "Accept-Language: en-US,en;q=0.5" \
    -H "Accept-Encoding: gzip, deflate, br, zstd" \
    -H "Referer: https://adventofcode.com/2024/day/$day" \
    -H "DNT: 1" \
    -H "Sec-GPC: 1" \
    -H "Connection: keep-alive" \
    -H "Upgrade-Insecure-Requests: 1" \
    -H "Sec-Fetch-Dest: document" \
    -H "Sec-Fetch-Mode: navigate" \
    -H "Sec-Fetch-Site: same-origin" \
    -H "Sec-Fetch-User: ?1" \
    -H "Priority: u=0, i" \
    -H "Pragma: no-cache" \
    -H "Cache-Control: no-cache" \
    -H "Cookie: $cookie" \
    --silent \
    --output "$output_file"

printf "Wrote $output_file succesfully\n"
