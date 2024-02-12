#!/usr/bin/env sh

set -e

mkdir zipped
for i in 1 2 3 4 5 6 7 8 9 10
do
  file_name="zipped/file_$i"
  dd if=/dev/random of=$file_name bs=1M count=5
done

zip -r static_file.zip zipped
mv static_file.zip static_file
rm -r zipped
