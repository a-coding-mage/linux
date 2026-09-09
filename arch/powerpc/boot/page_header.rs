/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2001 PPC64 Team, IBM Corp
 */

/*
 * In the assembler build ASM_CONST(x) is just x; in the C build it appends
 * the unsigned-long suffix. Rust integer literals are inferred at use sites.
 */

/* PAGE_SHIFT determines the page size */
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1usize << PAGE_SHIFT;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

/* align addr on a size boundary - adjust address up/down if needed */
#[macro_export]
macro_rules! _ALIGN_UP {
    ($addr:expr, $size:expr) => {{
        let addr = $addr;
        let size = $size;
        (addr + (size - 1)) & !(size - 1)
    }};
}

#[macro_export]
macro_rules! _ALIGN_DOWN {
    ($addr:expr, $size:expr) => {{
        let addr = $addr;
        let size = $size;
        addr & !(size - 1)
    }};
}

/* align addr on a size boundary - adjust address up if needed */
#[macro_export]
macro_rules! _ALIGN {
    ($addr:expr, $size:expr) => {
        $crate::_ALIGN_UP!($addr, $size)
    };
}

/* to align the pointer to the (next) page boundary */
#[macro_export]
macro_rules! PAGE_ALIGN {
    ($addr:expr) => {
        $crate::_ALIGN!($addr, $crate::PAGE_SIZE)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
