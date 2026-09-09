/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (c) 2016 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// Dependency supplied by <asm/bpf_perf_event.h>.

#[repr(C)]
pub struct bpf_perf_event_data {
    pub regs: bpf_user_pt_regs_t,
    pub sample_period: u64,
    pub addr: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
