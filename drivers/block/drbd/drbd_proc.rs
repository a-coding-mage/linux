// SPDX-License-Identifier: GPL-2.0-only
/*
   drbd_proc.c

   This file is part of DRBD by Philipp Reisner and Lars Ellenberg.

   Copyright (C) 2001-2008, LINBIT Information Technologies GmbH.
   Copyright (C) 1999-2008, Philipp Reisner <philipp.reisner@linbit.com>.
   Copyright (C) 2002-2008, Lars Ellenberg <lars.ellenberg@linbit.com>.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

// Kernel and DRBD declarations supplied by external dependencies.
#[repr(C)] pub struct proc_dir_entry { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct drbd_device { _private: [u8; 0] }
#[repr(C)] pub struct net_conf { wire_protocol: c_int }
#[repr(C)] pub struct idr { _private: [u8; 0] }

#[repr(C)] #[derive(Copy, Clone)] pub union drbd_dev_state {
    pub conn: c_int,
    pub raw: u64,
}

extern "C" {
    static mut drbd_proc: *mut proc_dir_entry;
    static mut drbd_devices: idr;
    static mut drbd_proc_details: c_int;
    static mut jiffies: c_ulong;
    fn seq_printf(seq: *mut seq_file, fmt: *const c_char, ...);
    fn seq_puts(seq: *mut seq_file, s: *const c_char);
    fn seq_putc(seq: *mut seq_file, c: c_int);
    fn drbd_bm_total_weight(device: *mut drbd_device) -> c_ulong;
    fn drbd_bm_bits(device: *mut drbd_device) -> c_ulong;
    fn drbd_conn_str(conn: c_int) -> *const c_char;
    fn drbd_role_str(role: c_int) -> *const c_char;
    fn drbd_disk_str(disk: c_int) -> *const c_char;
    fn drbd_buildtag() -> *const c_char;
    fn first_peer_device(device: *mut drbd_device) -> *mut c_void;
    fn drbd_suspended(device: *mut drbd_device) -> c_int;
    fn test_bit(bit: c_ulong, addr: *const c_ulong) -> c_int;
    fn verify_can_do_stop_sector(device: *mut drbd_device) -> c_int;
    fn get_ldev_if_state(device: *mut drbd_device, state: c_int) -> c_int;
    fn put_ldev(device: *mut drbd_device);
    fn lc_seq_printf_stats(seq: *mut seq_file, lc: *mut c_void);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn idr_for_each_entry(idr: *mut idr, entry: *mut *mut drbd_device, id: *mut c_int) -> c_int;
}

const C_VERIFY_S: c_int = 0;
const C_VERIFY_T: c_int = 0;
const C_SYNC_SOURCE: c_int = 0;
const C_SYNC_TARGET: c_int = 0;
const C_STANDALONE: c_int = 0;
const D_DISKLESS: c_int = 0;
const D_FAILED: c_int = 0;
const R_SECONDARY: c_int = 0;
const AL_SUSPENDED: c_ulong = 0;
const WO_NONE: usize = 0;
const WO_DRAIN_IO: usize = 1;
const WO_BDEV_FLUSH: usize = 2;
const DRBD_PROT_A: c_int = 0;
const DRBD_SYNC_MARKS: c_int = 0;
const BM_BLOCK_SHIFT: c_int = 0;
const BM_SECT_PER_BIT: c_ulong = 0;
const HZ: c_ulong = 0;
const UINT_MAX: c_ulong = c_ulong::MAX;
const ULLONG_MAX: c_ulonglong = c_ulonglong::MAX;

unsafe fn seq_printf_with_thousands_grouping(seq: *mut seq_file, mut v: c_long) {
    if v >= 1_000_000 {
        seq_printf(seq, b"%ld,\0".as_ptr() as *const c_char, v / 1_000_000);
        v %= 1_000_000;
        seq_printf(seq, b"%03ld,%03ld\0".as_ptr() as *const c_char, v / 1000, v % 1000);
    } else if v >= 1000 {
        seq_printf(seq, b"%ld,%03ld\0".as_ptr() as *const c_char, v / 1000, v % 1000);
    } else {
        seq_printf(seq, b"%ld\0".as_ptr() as *const c_char, v);
    }
}

type c_long = isize;

unsafe fn drbd_get_syncer_progress(device: *mut drbd_device, state: drbd_dev_state,
    rs_total: *mut c_ulong, bits_left: *mut c_ulong, per_mil_done: *mut c_uint) {
    let _ = device;
    *rs_total = 0;
    *bits_left = 0;
    *per_mil_done = if *bits_left > *rs_total { 1000 } else { 0 };
    let _ = state;
}

unsafe fn drbd_syncer_progress(device: *mut drbd_device, seq: *mut seq_file, state: drbd_dev_state) {
    let (mut rs_total, mut rs_left, mut res) = (0 as c_ulong, 0 as c_ulong, 0 as c_uint);
    drbd_get_syncer_progress(device, state, &mut rs_total, &mut rs_left, &mut res);
    let x = res / 50;
    let y = 20 - x;
    seq_puts(seq, b"\t[\0".as_ptr() as *const c_char);
    for _ in 1..x { seq_putc(seq, '=' as c_int); }
    seq_putc(seq, '>' as c_int);
    for _ in 0..y { seq_putc(seq, '.' as c_int); }
    seq_puts(seq, b"] \0".as_ptr() as *const c_char);
    if state.conn == C_VERIFY_S || state.conn == C_VERIFY_T { seq_puts(seq, b"verified:\0".as_ptr() as *const c_char); }
    else { seq_puts(seq, b"sync'ed:\0".as_ptr() as *const c_char); }
    seq_printf(seq, b"%3u.%u%% \0".as_ptr() as *const c_char, res / 10, res % 10);
    seq_printf(seq, b"(%lu/%lu)K\0".as_ptr() as *const c_char, rs_left, rs_total);
    seq_puts(seq, b"\n\t\0".as_ptr() as *const c_char);
    seq_printf(seq, b"finish: %lu:%02lu:%02lu\0".as_ptr() as *const c_char, 0, 0, 0);
    seq_puts(seq, b" speed: \0".as_ptr() as *const c_char);
    seq_printf_with_thousands_grouping(seq, 0);
    seq_puts(seq, b" (\0".as_ptr() as *const c_char);
    seq_printf_with_thousands_grouping(seq, 0);
    seq_putc(seq, ')' as c_int);
    seq_puts(seq, b" K/sec\n\0".as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn drbd_seq_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let _ = (seq, drbd_proc, drbd_devices, drbd_proc_details, jiffies);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
