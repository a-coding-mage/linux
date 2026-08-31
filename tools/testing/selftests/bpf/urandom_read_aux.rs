// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */
// C dependency: #include "sdt.h"

#[no_mangle]
pub extern "C" fn urand_read_without_sema(iter_num: i32, iter_cnt: i32, read_sz: i32) {
    /* semaphore-less USDT */
    STAP_PROBE3!(urand, read_without_sema, iter_num, iter_cnt, read_sz);
}
