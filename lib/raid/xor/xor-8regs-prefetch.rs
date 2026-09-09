// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from the C implementation. The declarations below are supplied
// by the corresponding kernel headers and xor implementation support.

unsafe extern "C" {
    fn prefetchw(ptr: *const core::ffi::c_void);
    fn prefetch(ptr: *const core::ffi::c_void);
    fn xor_gen_8regs_p();
}

unsafe fn xor_8regs_p_2(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
) {
    let mut lines: isize = (bytes / core::mem::size_of::<usize>() / 8) as isize - 1;
    prefetchw(p1.cast());
    prefetch(p2.cast());

    loop {
        prefetchw(p1.add(8).cast());
        prefetch(p2.add(8).cast());
        for i in 0..8 {
            *p1.add(i) ^= *p2.add(i);
        }
        p1 = p1.add(8);
        p2 = p2.add(8);
        lines -= 1;
        if lines <= 0 {
            if lines == 0 {
                for i in 0..8 {
                    *p1.add(i) ^= *p2.add(i);
                }
                p1 = p1.add(8);
                p2 = p2.add(8);
            }
            break;
        }
    }
}

unsafe fn xor_8regs_p_3(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
    mut p3: *const usize,
) {
    let mut lines: isize = (bytes / core::mem::size_of::<usize>() / 8) as isize - 1;
    prefetchw(p1.cast());
    prefetch(p2.cast());
    prefetch(p3.cast());

    loop {
        prefetchw(p1.add(8).cast());
        prefetch(p2.add(8).cast());
        prefetch(p3.add(8).cast());
        for i in 0..8 {
            *p1.add(i) ^= *p2.add(i) ^ *p3.add(i);
        }
        p1 = p1.add(8);
        p2 = p2.add(8);
        p3 = p3.add(8);
        lines -= 1;
        if lines <= 0 {
            if lines == 0 {
                for i in 0..8 {
                    *p1.add(i) ^= *p2.add(i) ^ *p3.add(i);
                }
                p1 = p1.add(8);
                p2 = p2.add(8);
                p3 = p3.add(8);
            }
            break;
        }
    }
}

unsafe fn xor_8regs_p_4(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
    mut p3: *const usize,
    mut p4: *const usize,
) {
    let mut lines: isize = (bytes / core::mem::size_of::<usize>() / 8) as isize - 1;
    prefetchw(p1.cast()); prefetch(p2.cast()); prefetch(p3.cast()); prefetch(p4.cast());
    loop {
        prefetchw(p1.add(8).cast()); prefetch(p2.add(8).cast());
        prefetch(p3.add(8).cast()); prefetch(p4.add(8).cast());
        for i in 0..8 { *p1.add(i) ^= *p2.add(i) ^ *p3.add(i) ^ *p4.add(i); }
        p1 = p1.add(8); p2 = p2.add(8); p3 = p3.add(8); p4 = p4.add(8);
        lines -= 1;
        if lines <= 0 { if lines == 0 { for i in 0..8 { *p1.add(i) ^= *p2.add(i) ^ *p3.add(i) ^ *p4.add(i); } } break; }
    }
}

unsafe fn xor_8regs_p_5(
    bytes: usize,
    mut p1: *mut usize,
    mut p2: *const usize,
    mut p3: *const usize,
    mut p4: *const usize,
    mut p5: *const usize,
) {
    let mut lines: isize = (bytes / core::mem::size_of::<usize>() / 8) as isize - 1;
    prefetchw(p1.cast()); prefetch(p2.cast()); prefetch(p3.cast()); prefetch(p4.cast()); prefetch(p5.cast());
    loop {
        prefetchw(p1.add(8).cast()); prefetch(p2.add(8).cast()); prefetch(p3.add(8).cast());
        prefetch(p4.add(8).cast()); prefetch(p5.add(8).cast());
        for i in 0..8 { *p1.add(i) ^= *p2.add(i) ^ *p3.add(i) ^ *p4.add(i) ^ *p5.add(i); }
        p1 = p1.add(8); p2 = p2.add(8); p3 = p3.add(8); p4 = p4.add(8); p5 = p5.add(8);
        lines -= 1;
        if lines <= 0 { if lines == 0 { for i in 0..8 { *p1.add(i) ^= *p2.add(i) ^ *p3.add(i) ^ *p4.add(i) ^ *p5.add(i); } } break; }
    }
}

// DO_XOR_BLOCKS(8regs_p, xor_8regs_p_2, xor_8regs_p_3, xor_8regs_p_4,
//               xor_8regs_p_5) is provided by xor_impl.h.
// struct xor_block_template xor_block_8regs_p = {
//     .name = "8regs_prefetch",
//     .xor_gen = xor_gen_8regs_p,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
