// SPDX-License-Identifier: GPL-2.0-or-later
// Dependency supplied by the original include: xor_impl.h

use core::ffi::{c_long, c_ulong};

unsafe fn xor_8regs_2(
    bytes: c_ulong,
    mut p1: *mut c_ulong,
    mut p2: *const c_ulong,
) {
    let mut lines = (bytes / core::mem::size_of::<c_long>() as c_ulong / 8) as c_long;

    loop {
        *p1.add(0) ^= *p2.add(0);
        *p1.add(1) ^= *p2.add(1);
        *p1.add(2) ^= *p2.add(2);
        *p1.add(3) ^= *p2.add(3);
        *p1.add(4) ^= *p2.add(4);
        *p1.add(5) ^= *p2.add(5);
        *p1.add(6) ^= *p2.add(6);
        *p1.add(7) ^= *p2.add(7);
        p1 = p1.add(8);
        p2 = p2.add(8);
        lines = lines.wrapping_sub(1);
        if lines <= 0 { break; }
    }
}

unsafe fn xor_8regs_3(
    bytes: c_ulong,
    mut p1: *mut c_ulong,
    mut p2: *const c_ulong,
    mut p3: *const c_ulong,
) {
    let mut lines = (bytes / core::mem::size_of::<c_long>() as c_ulong / 8) as c_long;
    loop {
        for i in 0..8 { *p1.add(i) ^= *p2.add(i) ^ *p3.add(i); }
        p1 = p1.add(8); p2 = p2.add(8); p3 = p3.add(8);
        lines = lines.wrapping_sub(1); if lines <= 0 { break; }
    }
}

unsafe fn xor_8regs_4(
    bytes: c_ulong,
    mut p1: *mut c_ulong,
    mut p2: *const c_ulong,
    mut p3: *const c_ulong,
    mut p4: *const c_ulong,
) {
    let mut lines = (bytes / core::mem::size_of::<c_long>() as c_ulong / 8) as c_long;
    loop {
        for i in 0..8 { *p1.add(i) ^= *p2.add(i) ^ *p3.add(i) ^ *p4.add(i); }
        p1 = p1.add(8); p2 = p2.add(8); p3 = p3.add(8); p4 = p4.add(8);
        lines = lines.wrapping_sub(1); if lines <= 0 { break; }
    }
}

unsafe fn xor_8regs_5(
    bytes: c_ulong,
    mut p1: *mut c_ulong,
    mut p2: *const c_ulong,
    mut p3: *const c_ulong,
    mut p4: *const c_ulong,
    mut p5: *const c_ulong,
) {
    let mut lines = (bytes / core::mem::size_of::<c_long>() as c_ulong / 8) as c_long;
    loop {
        for i in 0..8 { *p1.add(i) ^= *p2.add(i) ^ *p3.add(i) ^ *p4.add(i) ^ *p5.add(i); }
        p1 = p1.add(8); p2 = p2.add(8); p3 = p3.add(8); p4 = p4.add(8); p5 = p5.add(8);
        lines = lines.wrapping_sub(1); if lines <= 0 { break; }
    }
}

// DO_XOR_BLOCKS(8regs, xor_8regs_2, xor_8regs_3, xor_8regs_4, xor_8regs_5);
// The macro expansion and these types/functions are supplied by xor_impl.h.
#[allow(non_camel_case_types)]
pub struct xor_block_template;
extern "C" {
    fn xor_gen_8regs();
    pub static mut xor_block_8regs: xor_block_template;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
