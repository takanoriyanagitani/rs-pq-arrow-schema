#!/bin/bash

set -u

bin="./target/release/pq-show-arrow-schema"

ischema="./sample.d/psch.txt"
icsv="./sample.d/input.csv"
iparquet="./sample.d/input.parquet"

export ENV_PQ_FILENAME="${iparquet}"

export ENV_SHOW_MODE=detailed
export ENV_SHOW_MODE=typeonly
export ENV_SHOW_MODE=array

geninput(){
  echo generating the input file...

  mkdir -p "./sample.d"
  parquet-fromcsv \
    --has-header \
    --schema "${ischema}" \
    --input-file "${icsv}" \
    --output-file "${iparquet}"
}

run_native(){
  "${bin}"
}

test -f "${iparquet}" || geninput

echo array mode
run_native
echo

echo simple mode
ENV_SHOW_MODE=typeonly run_native
echo
