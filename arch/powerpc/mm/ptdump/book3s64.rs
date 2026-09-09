// SPDX-License-Identifier: GPL-2.0
/*
 * From split of dump_linuxpagetables.c
 * Copyright 2016, Rashmica Gupta, IBM Corp.
 *
 */

// Dependencies supplied by the corresponding kernel headers and ptdump.h.

static FLAG_ARRAY: [flag_info; 17] = [
    flag_info {
        mask: _PAGE_PRIVILEGED,
        val: 0,
        set: "user",
        clear: "    ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_READ,
        val: _PAGE_READ,
        set: "r",
        clear: " ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_WRITE,
        val: _PAGE_WRITE,
        set: "w",
        clear: " ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_EXEC,
        val: _PAGE_EXEC,
        set: " X ",
        clear: "   ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_PTE,
        val: _PAGE_PTE,
        set: "pte",
        clear: "   ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_PRESENT,
        val: _PAGE_PRESENT,
        set: "valid",
        clear: "     ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_PRESENT | _PAGE_INVALID,
        val: 0,
        set: "       ",
        clear: "present",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: H_PAGE_HASHPTE,
        val: H_PAGE_HASHPTE,
        set: "hpte",
        clear: "    ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_DIRTY,
        val: _PAGE_DIRTY,
        set: "dirty",
        clear: "     ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_ACCESSED,
        val: _PAGE_ACCESSED,
        set: "accessed",
        clear: "        ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_NON_IDEMPOTENT,
        val: _PAGE_NON_IDEMPOTENT,
        set: "non-idempotent",
        clear: "              ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_TOLERANT,
        val: _PAGE_TOLERANT,
        set: "tolerant",
        clear: "        ",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: H_PAGE_BUSY,
        val: H_PAGE_BUSY,
        set: "busy",
        ..flag_info::DEFAULT
    },
    #[cfg(feature = "CONFIG_PPC_64K_PAGES")]
    flag_info {
        mask: H_PAGE_COMBO,
        val: H_PAGE_COMBO,
        set: "combo",
        ..flag_info::DEFAULT
    },
    #[cfg(feature = "CONFIG_PPC_64K_PAGES")]
    flag_info {
        mask: H_PAGE_4K_PFN,
        val: H_PAGE_4K_PFN,
        set: "4K_pfn",
        ..flag_info::DEFAULT
    },
    #[cfg(not(feature = "CONFIG_PPC_64K_PAGES"))]
    flag_info {
        mask: H_PAGE_F_GIX,
        val: H_PAGE_F_GIX,
        set: "f_gix",
        is_val: true,
        shift: H_PAGE_F_GIX_SHIFT,
        ..flag_info::DEFAULT
    },
    #[cfg(not(feature = "CONFIG_PPC_64K_PAGES"))]
    flag_info {
        mask: H_PAGE_F_SECOND,
        val: H_PAGE_F_SECOND,
        set: "f_second",
        ..flag_info::DEFAULT
    },
    flag_info {
        mask: _PAGE_SPECIAL,
        val: _PAGE_SPECIAL,
        set: "special",
        ..flag_info::DEFAULT
    },
];

pub static mut pg_level: [ptdump_pg_level; 5] = [
    ptdump_pg_level { name: "PGD", flag: FLAG_ARRAY.as_ptr(), num: FLAG_ARRAY.len() },
    ptdump_pg_level { name: "P4D", flag: FLAG_ARRAY.as_ptr(), num: FLAG_ARRAY.len() },
    ptdump_pg_level { name: "PUD", flag: FLAG_ARRAY.as_ptr(), num: FLAG_ARRAY.len() },
    ptdump_pg_level { name: "PMD", flag: FLAG_ARRAY.as_ptr(), num: FLAG_ARRAY.len() },
    ptdump_pg_level { name: "PTE", flag: FLAG_ARRAY.as_ptr(), num: FLAG_ARRAY.len() },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
