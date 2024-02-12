#!/usr/bin/env bash

# URL="http://10.10.0.2:8080/hello"
URL="http://172.17.0.2:8080/hello"
N=32768 # 2^15

rm -f data.out
for i in $(seq 0 12)
do
  C=$(echo "2 ^ $i" | bc)
  hey -n $N -c $C $URL > out.tmp
  E=$(grep "Error distribution" out.tmp)
  if [[ -n $E ]]
  then
    printf "C=$C\tERROR\n"
    break
  fi

  R=$(grep "Requests/sec" out.tmp | awk '{ print $2 }')

  printf "C=$C\tR=$R\n"
  echo "$R" >> data.out
done

# rm -f out.tmp