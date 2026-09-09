// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * High speed xor_block operation for RAID4/5 utilizing the
 * ldd/std SPARC instructions.
 *
 * Copyright (C) 1999 Jakub Jelinek (jj@ultra.linux.cz)
 */

// C dependencies: xor_impl.h and xor_arch.h.

unsafe fn sparc_2(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
) {
    let mut lines = (bytes / core::mem::size_of::<usize>() / 8) as isize;
    loop {
        for i in 0..8 {
            let a = p1.add(i).read();
            let b = p2.add(i).read();
            p1.add(i).write(a ^ b);
        }
        p1 = p1.add(8);
        p2 = p2.add(8);
        lines -= 1;
        if lines <= 0 { break; }
    }
}

unsafe fn sparc_3(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
    mut p3: *const usize,
) {
    let mut lines = (bytes / core::mem::size_of::<usize>() / 8) as isize;
    loop {
        for i in 0..8 {
            let value = p1.add(i).read() ^ p2.add(i).read() ^ p3.add(i).read();
            p1.add(i).write(value);
        }
        p1 = p1.add(8); p2 = p2.add(8); p3 = p3.add(8);
        lines -= 1;
        if lines <= 0 { break; }
    }
}

unsafe fn sparc_4(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
    mut p3: *const usize,
    mut p4: *const usize,
) {
    let mut lines = (bytes / core::mem::size_of::<usize>() / 8) as isize;
    loop {
        for i in 0..8 {
            let value = p1.add(i).read() ^ p2.add(i).read() ^ p3.add(i).read() ^ p4.add(i).read();
            p1.add(i).write(value);
        }
        p1 = p1.add(8); p2 = p2.add(8); p3 = p3.add(8); p4 = p4.add(8);
        lines -= 1;
        if lines <= 0 { break; }
    }
}

unsafe fn sparc_5(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
    mut p3: *const usize,
    mut p4: *const usize,
    mut p5: *const usize,
) {
    let mut lines = (bytes / core::mem::size_of::<usize>() / 8) as isize;
    loop {
        for i in 0..8 {
            let value = p1.add(i).read()
                ^ p2.add(i).read() ^ p3.add(i).read()
                ^ p4.add(i).read() ^ p5.add(i).read();
            p1.add(i).write(value);
        }
        p1 = p1.add(8); p2 = p2.add(8); p3 = p3.add(8); p4 = p4.add(8); p5 = p5.add(8);
        lines -= 1;
        if lines <= 0 { break; }
    }
}

// The C macro expands the architecture-specific xor block entry points.
// DO_XOR_BLOCKS(sparc32, sparc_2, sparc_3, sparc_4, sparc_5);

// The C definition is supplied by the surrounding xor implementation:
// struct xor_block_template xor_block_SPARC = {
//     .name = "SPARC",
//     .xor_gen = xor_gen_sparc32,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
