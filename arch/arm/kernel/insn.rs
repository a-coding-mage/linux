// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

unsafe extern "C" {
    fn WARN_ON_ONCE(condition: bool);
    fn __opcode_thumb32_compose(first: usize, second: usize) -> usize;
}

unsafe fn __arm_gen_branch_thumb2(
    pc: usize,
    addr: usize,
    link: bool,
    warn: bool,
) -> usize {
    let (s, j1, j2, i1, i2, imm10, imm11): (usize, usize, usize, usize, usize, usize, usize);
    let (first, mut second): (usize, usize);
    let offset: isize;

    offset = (addr as isize).wrapping_sub((pc.wrapping_add(4)) as isize);
    if offset < -16777216 || offset > 16777214 {
        WARN_ON_ONCE(warn);
        return 0;
    }

    s = ((offset >> 24) & 0x1) as usize;
    i1 = ((offset >> 23) & 0x1) as usize;
    i2 = ((offset >> 22) & 0x1) as usize;
    imm10 = ((offset >> 12) & 0x3ff) as usize;
    imm11 = ((offset >> 1) & 0x7ff) as usize;

    j1 = ((!i1) ^ s) & 1;
    j2 = ((!i2) ^ s) & 1;

    first = 0xf000 | (s << 10) | imm10;
    second = 0x9000 | (j1 << 13) | (j2 << 11) | imm11;
    if link {
        second |= 1 << 14;
    }

    __opcode_thumb32_compose(first, second)
}

unsafe fn __arm_gen_branch_arm(
    pc: usize,
    addr: usize,
    link: bool,
    warn: bool,
) -> usize {
    let mut opcode: usize = 0xea000000;
    let mut offset: isize;

    if link {
        opcode |= 1 << 24;
    }

    offset = (addr as isize).wrapping_sub((pc.wrapping_add(8)) as isize);
    if offset < -33554432 || offset > 33554428 {
        WARN_ON_ONCE(warn);
        return 0;
    }

    offset = (offset >> 2) & 0x00ffffff;

    opcode | offset as usize
}

pub unsafe fn __arm_gen_branch(pc: usize, addr: usize, link: bool, warn: bool) -> usize {
    // Build-time CONFIG_THUMB2_KERNEL selects the implementation, as in IS_ENABLED().
    if cfg!(CONFIG_THUMB2_KERNEL) {
        __arm_gen_branch_thumb2(pc, addr, link, warn)
    } else {
        __arm_gen_branch_arm(pc, addr, link, warn)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
