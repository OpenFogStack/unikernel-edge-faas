#!/usr/bin/env bash

# URL="http://10.10.0.2:8080/static"
URL="http://172.17.0.2:8080/static"

# fetching the file 204 sums up to 9.96 ~ 10 GiB
N=204

# fetch once to give a chance for filling page cache
hey -c 1 -n 1 $URL &> /dev/null

hey -c 4 -n 204 $URL

