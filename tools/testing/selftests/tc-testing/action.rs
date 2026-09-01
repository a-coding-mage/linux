/* SPDX-License-Identifier: GPL-2.0
 * Copyright (c) 2018 Davide Caratti, Red Hat inc.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

/* Dependency intent from C includes:
 * #include <linux/bpf.h>
 * #include <linux/pkt_cls.h>
 *
 * This translation expects Rust declarations for `__sk_buff` and `TC_ACT_OK`
 * to be supplied by the surrounding build/bindings.
 */

#[no_mangle]
#[link_section = "action-ok"]
#[used]
pub unsafe extern "C" fn action_ok(s: *mut __sk_buff) -> i32 {
    TC_ACT_OK
}

#[no_mangle]
#[link_section = "action-ko"]
#[used]
pub unsafe extern "C" fn action_ko(s: *mut __sk_buff) -> i32 {
    (*s).data = 0x0;
    TC_ACT_OK
}

#[no_mangle]
#[link_section = "license"]
#[used]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
