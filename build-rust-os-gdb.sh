#!/bin/sh
# Copyright 2014 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# Exit if anything fails
set -e

DIR=$( pwd )

git clone -b rust-os --depth 1 \
  https://github.com/phil-opp/binutils-gdb.git
cd binutils-gdb
./configure --prefix="$DIR/rust-os-gdb" --with-python=yes
make
make install
cd ..
rm -rf binutils-gdb
