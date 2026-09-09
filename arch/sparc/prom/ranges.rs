// SPDX-License-Identifier: GPL-2.0
/*
 * ranges.c: Handle ranges in newer proms for obio/sbus.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

static mut promlib_obio_ranges: [linux_prom_ranges; PROMREG_MAX] =
    [linux_prom_ranges::default(); PROMREG_MAX];
static mut num_obio_ranges: i32 = 0;

/* Adjust register values based upon the ranges parameters. */
unsafe fn prom_adjust_regs(
    regp: *mut linux_prom_registers,
    nregs: i32,
    rangep: *mut linux_prom_ranges,
    nranges: i32,
) {
    let mut regc: i32;
    let mut rngc: i32;

    regc = 0;
    while regc < nregs {
        rngc = 0;
        while rngc < nranges {
            if (*regp.add(regc as usize)).which_io
                == (*rangep.add(rngc as usize)).ot_child_space
            {
                break; /* Fount it */
            }
            rngc += 1;
        }
        if rngc == nranges {
            /* oops */
            prom_printf(
                b"adjust_regs: Could not find range with matching bus type...\n\0"
                    .as_ptr() as *const i8,
            );
        }
        (*regp.add(regc as usize)).which_io =
            (*rangep.add(rngc as usize)).ot_parent_space;
        (*regp.add(regc as usize)).phys_addr -=
            (*rangep.add(rngc as usize)).ot_child_base;
        (*regp.add(regc as usize)).phys_addr +=
            (*rangep.add(rngc as usize)).ot_parent_base;
        regc += 1;
    }
}

unsafe fn prom_adjust_ranges(
    ranges1: *mut linux_prom_ranges,
    nranges1: i32,
    ranges2: *mut linux_prom_ranges,
    nranges2: i32,
) {
    let mut rng1c: i32;
    let mut rng2c: i32;

    rng1c = 0;
    while rng1c < nranges1 {
        rng2c = 0;
        while rng2c < nranges2 {
            let r1 = &*ranges1.add(rng1c as usize);
            let r2 = &*ranges2.add(rng2c as usize);
            if r1.ot_parent_space == r2.ot_child_space
                && r1.ot_parent_base >= r2.ot_child_base
                && r2.ot_child_base + r2.or_size - r1.ot_parent_base > 0
            {
                break;
            }
            rng2c += 1;
        }
        if rng2c == nranges2 {
            /* oops */
            prom_printf(
                b"adjust_ranges: Could not find matching bus type...\n\0"
                    .as_ptr() as *const i8,
            );
        } else if (*ranges1.add(rng1c as usize)).ot_parent_base
            + (*ranges1.add(rng1c as usize)).or_size
            > (*ranges2.add(rng2c as usize)).ot_child_base
                + (*ranges2.add(rng2c as usize)).or_size
        {
            (*ranges1.add(rng1c as usize)).or_size =
                (*ranges2.add(rng2c as usize)).ot_child_base
                    + (*ranges2.add(rng2c as usize)).or_size
                    - (*ranges1.add(rng1c as usize)).ot_parent_base;
        }
        (*ranges1.add(rng1c as usize)).ot_parent_space =
            (*ranges2.add(rng2c as usize)).ot_parent_space;
        (*ranges1.add(rng1c as usize)).ot_parent_base +=
            (*ranges2.add(rng2c as usize)).ot_parent_base;
        rng1c += 1;
    }
}

/* Apply probed obio ranges to registers passed, if no ranges return. */
pub unsafe fn prom_apply_obio_ranges(regs: *mut linux_prom_registers, nregs: i32) {
    if num_obio_ranges != 0 {
        prom_adjust_regs(regs, nregs, promlib_obio_ranges.as_mut_ptr(), num_obio_ranges);
    }
}

pub unsafe fn prom_ranges_init() {
    let node: phandle;
    let obio_node: phandle;
    let mut success: i32;

    num_obio_ranges = 0;

    /* Check for obio and sbus ranges. */
    node = prom_getchild(prom_root_node);
    obio_node = prom_searchsiblings(node, b"obio\0".as_ptr() as *const i8);

    if obio_node != 0 {
        success = prom_getproperty(
            obio_node,
            b"ranges\0".as_ptr() as *const i8,
            promlib_obio_ranges.as_mut_ptr() as *mut i8,
            core::mem::size_of_val(&promlib_obio_ranges) as i32,
        );
        if success != -1 {
            num_obio_ranges =
                success / core::mem::size_of::<linux_prom_ranges>() as i32;
        }
    }

    if num_obio_ranges != 0 {
        prom_printf(b"PROMLIB: obio_ranges %d\n\0".as_ptr() as *const i8, num_obio_ranges);
    }
}

pub unsafe fn prom_apply_generic_ranges(
    node: phandle,
    parent: phandle,
    regs: *mut linux_prom_registers,
    nregs: i32,
) {
    let mut success: i32;
    let mut num_ranges: i32;
    let mut ranges: [linux_prom_ranges; PROMREG_MAX] =
        [linux_prom_ranges::default(); PROMREG_MAX];

    success = prom_getproperty(
        node,
        b"ranges\0".as_ptr() as *const i8,
        ranges.as_mut_ptr() as *mut i8,
        core::mem::size_of_val(&ranges) as i32,
    );
    if success != -1 {
        num_ranges = success / core::mem::size_of::<linux_prom_ranges>() as i32;
        if parent != 0 {
            let mut parent_ranges: [linux_prom_ranges; PROMREG_MAX] =
                [linux_prom_ranges::default(); PROMREG_MAX];
            let mut num_parent_ranges: i32;

            success = prom_getproperty(
                parent,
                b"ranges\0".as_ptr() as *const i8,
                parent_ranges.as_mut_ptr() as *mut i8,
                core::mem::size_of_val(&parent_ranges) as i32,
            );
            if success != -1 {
                num_parent_ranges =
                    success / core::mem::size_of::<linux_prom_ranges>() as i32;
                prom_adjust_ranges(
                    ranges.as_mut_ptr(),
                    num_ranges,
                    parent_ranges.as_mut_ptr(),
                    num_parent_ranges,
                );
            }
        }
        prom_adjust_regs(regs, nregs, ranges.as_mut_ptr(), num_ranges);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
