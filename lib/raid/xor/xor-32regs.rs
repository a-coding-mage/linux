// SPDX-License-Identifier: GPL-2.0-or-later
// Dependency: xor_impl.h

unsafe fn xor_32regs_2(
    bytes: usize,
    mut p1: *mut libc::c_ulong,
    mut p2: *const libc::c_ulong,
) {
    let mut lines = (bytes / core::mem::size_of::<libc::c_long>() / 8) as libc::c_long;

    loop {
        let mut d0: libc::c_long = *p1.add(0) as libc::c_long;
        let mut d1: libc::c_long = *p1.add(1) as libc::c_long;
        let mut d2: libc::c_long = *p1.add(2) as libc::c_long;
        let mut d3: libc::c_long = *p1.add(3) as libc::c_long;
        let mut d4: libc::c_long = *p1.add(4) as libc::c_long;
        let mut d5: libc::c_long = *p1.add(5) as libc::c_long;
        let mut d6: libc::c_long = *p1.add(6) as libc::c_long;
        let mut d7: libc::c_long = *p1.add(7) as libc::c_long;
        d0 ^= *p2.add(0) as libc::c_long; d1 ^= *p2.add(1) as libc::c_long;
        d2 ^= *p2.add(2) as libc::c_long; d3 ^= *p2.add(3) as libc::c_long;
        d4 ^= *p2.add(4) as libc::c_long; d5 ^= *p2.add(5) as libc::c_long;
        d6 ^= *p2.add(6) as libc::c_long; d7 ^= *p2.add(7) as libc::c_long;
        *p1.add(0) = d0 as libc::c_ulong; *p1.add(1) = d1 as libc::c_ulong;
        *p1.add(2) = d2 as libc::c_ulong; *p1.add(3) = d3 as libc::c_ulong;
        *p1.add(4) = d4 as libc::c_ulong; *p1.add(5) = d5 as libc::c_ulong;
        *p1.add(6) = d6 as libc::c_ulong; *p1.add(7) = d7 as libc::c_ulong;
        p1 = p1.add(8); p2 = p2.add(8);
        lines -= 1;
        if lines <= 0 { break; }
    }
}

unsafe fn xor_32regs_3(bytes: usize, p1: *mut libc::c_ulong, p2: *const libc::c_ulong, p3: *const libc::c_ulong) {
    xor_32regs_n(bytes, p1, &[p2, p3]);
}

unsafe fn xor_32regs_4(bytes: usize, p1: *mut libc::c_ulong, p2: *const libc::c_ulong, p3: *const libc::c_ulong, p4: *const libc::c_ulong) {
    xor_32regs_n(bytes, p1, &[p2, p3, p4]);
}

unsafe fn xor_32regs_5(bytes: usize, p1: *mut libc::c_ulong, p2: *const libc::c_ulong, p3: *const libc::c_ulong, p4: *const libc::c_ulong, p5: *const libc::c_ulong) {
    xor_32regs_n(bytes, p1, &[p2, p3, p4, p5]);
}

unsafe fn xor_32regs_n(bytes: usize, mut p1: *mut libc::c_ulong, ps: &[*const libc::c_ulong]) {
    let mut lines = bytes / core::mem::size_of::<libc::c_long>() / 8;
    loop {
        for i in 0..8 {
            let mut d = *p1.add(i) as libc::c_long;
            for &p in ps { d ^= *p.add(i) as libc::c_long; }
            *p1.add(i) = d as libc::c_ulong;
        }
        p1 = p1.add(8);
        lines -= 1;
        if lines == 0 { break; }
    }
}

// DO_XOR_BLOCKS(32regs, xor_32regs_2, xor_32regs_3, xor_32regs_4, xor_32regs_5);

pub static mut xor_block_32regs: xor_block_template = xor_block_template {
    name: "32regs",
    xor_gen: xor_gen_32regs,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
