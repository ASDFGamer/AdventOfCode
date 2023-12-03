#!/bin/sh

#TODO convert to rust

if [ $# -ne 1 ]; then
    echo "Please provide the day as the only argument"
    exit 1
fi
day=$1
session_id_file="./.aoc_session_id"
if [ ! -f $session_id_file ]; then
    echo "Please create a aoc_session_id file which contains your aoc session id"
    exit 1
fi
session_id=$(cat $session_id_file)
curl -H "cookie: session=$session_id" https://adventofcode.com/2023/day/${day}/input -o "./day-${day}/src/bin/input" > /dev/null