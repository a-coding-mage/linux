// SPDX-License-Identifier: GPL-2.0
/*
 * From split of dump_linuxpagetables.c
 * Copyright 2016, Rashmica Gupta, IBM Corp.
 *
 */

// Dependency declarations from <linux/kernel.h>, <linux/pgtable.h>, and
// "ptdump.h" are supplied by the surrounding translation unit.

static flag_array: [flag_info; 11] = [
    flag_info {
        #[cfg(feature = "CONFIG_PPC_16K_PAGES")]
        mask: _PAGE_HUGE,
        #[cfg(feature = "CONFIG_PPC_16K_PAGES")]
        val: _PAGE_HUGE,
        #[cfg(not(feature = "CONFIG_PPC_16K_PAGES"))]
        mask: _PAGE_SPS,
        #[cfg(not(feature = "CONFIG_PPC_16K_PAGES"))]
        val: _PAGE_SPS,
        set: "huge\0".as_ptr() as *const i8,
        clear: "    \0".as_ptr() as *const i8,
    },
    flag_info {
        mask: _PAGE_RO | _PAGE_NA,
        val: 0,
        set: "rw\0".as_ptr() as *const i8,
        clear: core::ptr::null(),
    },
    flag_info {
        mask: _PAGE_RO | _PAGE_NA,
        val: _PAGE_RO,
        set: "r \0".as_ptr() as *const i8,
        clear: core::ptr::null(),
    },
    flag_info {
        mask: _PAGE_RO | _PAGE_NA,
        val: _PAGE_NA,
        set: "  \0".as_ptr() as *const i8,
        clear: core::ptr::null(),
    },
    flag_info {
        mask: _PAGE_EXEC,
        val: _PAGE_EXEC,
        set: " X \0".as_ptr() as *const i8,
        clear: "   \0".as_ptr() as *const i8,
    },
    flag_info {
        mask: _PAGE_PRESENT,
        val: _PAGE_PRESENT,
        set: "present\0".as_ptr() as *const i8,
        clear: "       \0".as_ptr() as *const i8,
    },
    flag_info {
        mask: _PAGE_GUARDED,
        val: _PAGE_GUARDED,
        set: "guarded\0".as_ptr() as *const i8,
        clear: "       \0".as_ptr() as *const i8,
    },
    flag_info {
        mask: _PAGE_DIRTY,
        val: _PAGE_DIRTY,
        set: "dirty\0".as_ptr() as *const i8,
        clear: "     \0".as_ptr() as *const i8,
    },
    flag_info {
        mask: _PAGE_ACCESSED,
        val: _PAGE_ACCESSED,
        set: "accessed\0".as_ptr() as *const i8,
        clear: "        \0".as_ptr() as *const i8,
    },
    flag_info {
        mask: _PAGE_NO_CACHE,
        val: _PAGE_NO_CACHE,
        set: "no cache\0".as_ptr() as *const i8,
        clear: "        \0".as_ptr() as *const i8,
    },
    flag_info {
        mask: _PAGE_SPECIAL,
        val: _PAGE_SPECIAL,
        set: "special\0".as_ptr() as *const i8,
        clear: core::ptr::null(),
    },
];

#[no_mangle]
pub static mut pg_level: [ptdump_pg_level; 5] = [
    ptdump_pg_level {
        name: "PGD\0".as_ptr() as *const i8,
        flag: flag_array.as_ptr(),
        num: 11,
    },
    ptdump_pg_level {
        name: "P4D\0".as_ptr() as *const i8,
        flag: flag_array.as_ptr(),
        num: 11,
    },
    ptdump_pg_level {
        name: "PUD\0".as_ptr() as *const i8,
        flag: flag_array.as_ptr(),
        num: 11,
    },
    ptdump_pg_level {
        name: "PMD\0".as_ptr() as *const i8,
        flag: flag_array.as_ptr(),
        num: 11,
    },
    ptdump_pg_level {
        name: "PTE\0".as_ptr() as *const i8,
        flag: flag_array.as_ptr(),
        num: 11,
    },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
