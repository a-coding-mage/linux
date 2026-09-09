// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1999, 2000 Ralf Baechle (ralf@gnu.org)
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/sched.h, linux/interrupt.h, linux/kernel_stat.h,
// linux/param.h, linux/timex.h, linux/mm.h, asm/sn/klconfig.h,
// asm/sn/arch.h, and asm/sn/gda.h.

pub unsafe fn find_component(
    brd: *mut lboard_t,
    mut kli: *mut klinfo_t,
    struct_type: u8,
) -> *mut klinfo_t {
    let mut index: i32;
    let mut j: i32;

    if kli.is_null() {
        index = 0;
    } else {
        j = 0;
        while j < KLCF_NUM_COMPS(brd) {
            if kli == KLCF_COMP(brd, j) {
                break;
            }
            j += 1;
        }
        index = j;
        if index == KLCF_NUM_COMPS(brd) {
            printk(c"find_component: Bad pointer: 0x%p\n", kli);
            return core::ptr::null_mut();
        }
        index += 1; // next component
    }

    while index < KLCF_NUM_COMPS(brd) {
        kli = KLCF_COMP(brd, index);
        if KLCF_COMP_TYPE(kli) == struct_type {
            return kli;
        }
        index += 1;
    }

    /* Didn't find it. */
    core::ptr::null_mut()
}

pub unsafe fn find_first_component(
    brd: *mut lboard_t,
    struct_type: u8,
) -> *mut klinfo_t {
    find_component(brd, core::ptr::null_mut(), struct_type)
}

pub unsafe fn find_lboard(
    mut start: *mut lboard_t,
    brd_type: u8,
) -> *mut lboard_t {
    /* Search all boards stored on this node. */
    while !start.is_null() {
        if (*start).brd_type == brd_type {
            return start;
        }
        start = KLCF_NEXT(start);
    }
    /* Didn't find it. */
    core::ptr::null_mut()
}

pub unsafe fn find_lboard_class(
    mut start: *mut lboard_t,
    brd_type: u8,
) -> *mut lboard_t {
    /* Search all boards stored on this node. */
    while !start.is_null() {
        if KLCLASS((*start).brd_type) == KLCLASS(brd_type) {
            return start;
        }
        start = KLCF_NEXT(start);
    }

    /* Didn't find it. */
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
