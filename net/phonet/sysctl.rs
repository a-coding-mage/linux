// SPDX-License-Identifier: GPL-2.0-only
/*
 * File: sysctl.c
 *
 * Phonet /proc/sys/net/phonet interface implementation
 *
 * Copyright (C) 2008 Nokia Corporation.
 *
 * Author: Rémi Denis-Courmont
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/seqlock.h, linux/sysctl.h, linux/errno.h, linux/init.h,
// net/sock.h, linux/phonet.h, and net/phonet/phonet.h.

const DYNAMIC_PORT_MIN: ::core::ffi::c_int = 0x40;
const DYNAMIC_PORT_MAX: ::core::ffi::c_int = 0x7f;

static mut local_port_range_lock: seqlock_t = DEFINE_SEQLOCK!();
static mut local_port_range_min: [::core::ffi::c_int; 2] = [0, 0];
static mut local_port_range_max: [::core::ffi::c_int; 2] = [1023, 1023];
static mut local_port_range: [::core::ffi::c_int; 2] =
    [DYNAMIC_PORT_MIN, DYNAMIC_PORT_MAX];
static mut phonet_table_hrd: *mut ctl_table_header = ::core::ptr::null_mut();

unsafe fn set_local_port_range(range: *mut ::core::ffi::c_int) {
    write_seqlock(&raw mut local_port_range_lock);
    local_port_range[0] = *range.add(0);
    local_port_range[1] = *range.add(1);
    write_sequnlock(&raw mut local_port_range_lock);
}

pub unsafe fn phonet_get_local_port_range(
    min: *mut ::core::ffi::c_int,
    max: *mut ::core::ffi::c_int,
) {
    let mut seq: ::core::ffi::c_uint;

    loop {
        seq = read_seqbegin(&raw mut local_port_range_lock);
        if !min.is_null() {
            *min = local_port_range[0];
        }
        if !max.is_null() {
            *max = local_port_range[1];
        }
        if !read_seqretry(&raw mut local_port_range_lock, seq) {
            break;
        }
    }
}

unsafe fn proc_local_port_range(
    table: *const ctl_table,
    write: ::core::ffi::c_int,
    buffer: *mut ::core::ffi::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> ::core::ffi::c_int {
    let mut range: [::core::ffi::c_int; 2] =
        [local_port_range[0], local_port_range[1]];
    let mut tmp: ctl_table = ::core::mem::zeroed();
    tmp.data = (&mut range as *mut [::core::ffi::c_int; 2]).cast();
    tmp.maxlen = ::core::mem::size_of_val(&range);
    tmp.mode = (*table).mode;
    tmp.extra1 = (&raw mut local_port_range_min).cast();
    tmp.extra2 = (&raw mut local_port_range_max).cast();

    let mut ret = proc_dointvec_minmax(&mut tmp, write, buffer, lenp, ppos);

    if write != 0 && ret == 0 {
        if range[1] < range[0] {
            ret = -EINVAL;
        } else {
            set_local_port_range(range.as_mut_ptr());
        }
    }

    ret
}

static mut phonet_table: [ctl_table; 2] = [
    ctl_table {
        procname: b"local_port_range\0".as_ptr().cast(),
        data: (&raw mut local_port_range).cast(),
        maxlen: ::core::mem::size_of::<[::core::ffi::c_int; 2]>(),
        mode: 0o644,
        proc_handler: Some(proc_local_port_range),
        ..unsafe { ::core::mem::zeroed() }
    },
    unsafe { ::core::mem::zeroed() },
];

pub unsafe fn phonet_sysctl_init() -> ::core::ffi::c_int {
    phonet_table_hrd = register_net_sysctl(
        &raw mut init_net,
        b"net/phonet\0".as_ptr().cast(),
        (&raw mut phonet_table).cast(),
    );
    if phonet_table_hrd.is_null() { -ENOMEM } else { 0 }
}

pub unsafe fn phonet_sysctl_exit() {
    unregister_net_sysctl_table(phonet_table_hrd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
