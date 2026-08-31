// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
// C source defined _GNU_SOURCE and included <unistd.h> for gettid().

unsafe extern "C" {
    fn gettid() -> i32;
}

fn main() -> i32 {
    unsafe { gettid() }
}
