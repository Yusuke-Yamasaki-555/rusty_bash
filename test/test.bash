#!/bin/bash
# SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
# SPDX-License-Identifier: BSD-3-Clause

cargo build --release || exit 1

cd $(dirname $0)

com=../target/release/rusty_bash

{

./test_simple_command.bash 2> /dev/null | tail -n 1 &
./test_others.bash 2> /dev/null | tail -n 1         &
./test_jobs.bash 2> /dev/null | tail -n 1           &
./test_builtins.bash 2> /dev/null | tail -n 1       &

wait

} | awk '{print}/ERROR/{exit 1}'
