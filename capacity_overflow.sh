#!/usr/bin/env bash

. ~/.profile

export RUST_LIB_DIR=$(rustup run stable rustc --print sysroot)/lib
export JAVA_HOME="${JAVA_HOME:-$(mvn --version | grep 'Java home' | sed 's/.*: //')}"
export JAVA_LIB_DIR="$(find ${JAVA_HOME} -type f -name libjvm.\* | xargs -n1 dirname)"

export LD_LIBRARY_PATH=$JAVA_LIB_DIR:$RUST_LIB_DIR
#export LD_LIBRARY_PATH=/Library/Java/JavaVirtualMachines/jdk1.8.0_152.jdk/Contents/Home/jre/lib/server/:$RUST_LIB_DIR

env

cargo run --example capacity_overflow