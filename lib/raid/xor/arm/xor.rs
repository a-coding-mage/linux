// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2001 Russell King
 */

// Dependencies supplied by xor_impl.h and xor_arch.h are intentionally not
// reimplemented here.

unsafe fn xor_arm4regs_2(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
) {
    let mut lines = bytes / core::mem::size_of::<usize>() / 4;

    loop {
        let mut a1 = p1.read();
        let mut a2 = p1.add(1).read();
        let mut a3 = p1.add(2).read();
        let mut a4 = p1.add(3).read();
        p1 = p1.add(4);

        a1 ^= p2.read();
        a2 ^= p2.add(1).read();
        a3 ^= p2.add(2).read();
        a4 ^= p2.add(3).read();
        p2 = p2.add(4);

        p1.sub(4).write(a1);
        p1.sub(3).write(a2);
        p1.sub(2).write(a3);
        p1.sub(1).write(a4);

        lines = lines.wrapping_sub(1);
        if lines == 0 {
            break;
        }
    }
}

unsafe fn xor_arm4regs_3(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
    mut p3: *const usize,
) {
    let mut lines = bytes / core::mem::size_of::<usize>() / 4;

    loop {
        let mut a1 = p1.read();
        let mut a2 = p1.add(1).read();
        let mut a3 = p1.add(2).read();
        let mut a4 = p1.add(3).read();
        p1 = p1.add(4);

        a1 ^= p2.read();
        a2 ^= p2.add(1).read();
        a3 ^= p2.add(2).read();
        a4 ^= p2.add(3).read();
        p2 = p2.add(4);

        a1 ^= p3.read();
        a2 ^= p3.add(1).read();
        a3 ^= p3.add(2).read();
        a4 ^= p3.add(3).read();
        p3 = p3.add(4);

        p1.sub(4).write(a1);
        p1.sub(3).write(a2);
        p1.sub(2).write(a3);
        p1.sub(1).write(a4);

        lines = lines.wrapping_sub(1);
        if lines == 0 {
            break;
        }
    }
}

unsafe fn xor_arm4regs_4(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
    mut p3: *const usize,
    mut p4: *const usize,
) {
    let mut lines = bytes / core::mem::size_of::<usize>() / 2;

    loop {
        let mut a1 = p1.read();
        let mut a2 = p1.add(1).read();
        p1 = p1.add(2);

        a1 ^= p2.read();
        a2 ^= p2.add(1).read();
        p2 = p2.add(2);
        a1 ^= p3.read();
        a2 ^= p3.add(1).read();
        p3 = p3.add(2);
        a1 ^= p4.read();
        a2 ^= p4.add(1).read();
        p4 = p4.add(2);

        p1.sub(2).write(a1);
        p1.sub(1).write(a2);

        lines = lines.wrapping_sub(1);
        if lines == 0 {
            break;
        }
    }
}

unsafe fn xor_arm4regs_5(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
    mut p3: *const usize,
    mut p4: *const usize,
    mut p5: *const usize,
) {
    let mut lines = bytes / core::mem::size_of::<usize>() / 2;

    loop {
        let mut a1 = p1.read();
        let mut a2 = p1.add(1).read();
        p1 = p1.add(2);

        a1 ^= p2.read();
        a2 ^= p2.add(1).read();
        p2 = p2.add(2);
        a1 ^= p3.read();
        a2 ^= p3.add(1).read();
        p3 = p3.add(2);
        a1 ^= p4.read();
        a2 ^= p4.add(1).read();
        p4 = p4.add(2);
        a1 ^= p5.read();
        a2 ^= p5.add(1).read();
        p5 = p5.add(2);

        p1.sub(2).write(a1);
        p1.sub(1).write(a2);

        lines = lines.wrapping_sub(1);
        if lines == 0 {
            break;
        }
    }
}

// DO_XOR_BLOCKS(arm4regs, xor_arm4regs_2, xor_arm4regs_3, xor_arm4regs_4,
//               xor_arm4regs_5);
// The macro-generated xor_gen_arm4regs declaration and xor_block_template
// definition are supplied by the corresponding external dependencies.
extern "C" {
    fn xor_gen_arm4regs();
}

pub static mut xor_block_arm4regs: xor_block_template = xor_block_template {
    name: "arm4regs",
    xor_gen: xor_gen_arm4regs,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
