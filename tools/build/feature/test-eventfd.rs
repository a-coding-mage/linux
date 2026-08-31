// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>

// C dependency intent: #include <sys/eventfd.h>

const EFD_NONBLOCK: i32 = 0o0004000;

unsafe extern "C" {
    fn eventfd(initval: u32, flags: i32) -> i32;
}

fn main() -> i32 {
    unsafe { eventfd(0, EFD_NONBLOCK) }
}
