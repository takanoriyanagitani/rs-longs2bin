#!/bin/sh

wsm="./target/wasm32-wasip1/release-wasi/longs2bin_std.wasm"

help(){
  wazero run "${wsm}" -h |
    bat --language=help
}

ex1(){
  printf '%d\n' 2 3 5 7 |
    wazero run "${wsm}" -l |
    xxd
}

ex2(){
  printf '%d\n' 2 3 5 7 |
    wazero run "${wsm}" -b |
    xxd
}

#help

ex1
#ex2
