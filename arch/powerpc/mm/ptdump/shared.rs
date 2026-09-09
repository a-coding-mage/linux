// SPDX-License-Identifier: GPL-2.0
/*
 * From split of dump_linuxpagetables.c
 * Copyright 2016, Rashmica Gupta, IBM Corp.
 */

// Dependencies supplied by the Linux kernel and ptdump.h are intentionally
// left external to this translation unit.

#[allow(non_upper_case_globals)]
static flag_array: [flag_info; 11] = [
    flag_info {
        mask: _PAGE_READ,
        val: 0,
        set: " ",
        clear: "r",
    },
    flag_info {
        mask: _PAGE_WRITE,
        val: 0,
        set: " ",
        clear: "w",
    },
    flag_info {
        mask: _PAGE_EXEC,
        val: _PAGE_EXEC,
        set: " X ",
        clear: "   ",
    },
    flag_info {
        mask: _PAGE_PRESENT,
        val: _PAGE_PRESENT,
        set: "present",
        clear: "       ",
    },
    flag_info {
        mask: _PAGE_COHERENT,
        val: _PAGE_COHERENT,
        set: "coherent",
        clear: "        ",
    },
    flag_info {
        mask: _PAGE_GUARDED,
        val: _PAGE_GUARDED,
        set: "guarded",
        clear: "       ",
    },
    flag_info {
        mask: _PAGE_DIRTY,
        val: _PAGE_DIRTY,
        set: "dirty",
        clear: "     ",
    },
    flag_info {
        mask: _PAGE_ACCESSED,
        val: _PAGE_ACCESSED,
        set: "accessed",
        clear: "        ",
    },
    flag_info {
        mask: _PAGE_WRITETHRU,
        val: _PAGE_WRITETHRU,
        set: "write through",
        clear: "             ",
    },
    flag_info {
        mask: _PAGE_NO_CACHE,
        val: _PAGE_NO_CACHE,
        set: "no cache",
        clear: "        ",
    },
    flag_info {
        mask: _PAGE_SPECIAL,
        val: _PAGE_SPECIAL,
        set: "special",
    },
];

#[allow(non_upper_case_globals)]
static mut pg_level: [ptdump_pg_level; 5] = [
    ptdump_pg_level {
        name: "PGD",
        flag: flag_array.as_ptr(),
        num: flag_array.len(),
    },
    ptdump_pg_level {
        name: "P4D",
        flag: flag_array.as_ptr(),
        num: flag_array.len(),
    },
    ptdump_pg_level {
        name: "PUD",
        flag: flag_array.as_ptr(),
        num: flag_array.len(),
    },
    ptdump_pg_level {
        name: "PMD",
        flag: flag_array.as_ptr(),
        num: flag_array.len(),
    },
    ptdump_pg_level {
        name: "PTE",
        flag: flag_array.as_ptr(),
        num: flag_array.len(),
    },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
