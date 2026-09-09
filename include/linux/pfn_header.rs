/* SPDX-License-Identifier: GPL-2.0 */

// C header dependency: <linux/types.h>
// PAGE_SIZE, PAGE_MASK, and PAGE_SHIFT are supplied by the surrounding
// translation environment.

#[macro_export]
macro_rules! PFN_ALIGN {
    ($x:expr) => {
        (((($x) as usize) + (PAGE_SIZE - 1)) & PAGE_MASK)
    };
}

#[macro_export]
macro_rules! PFN_UP {
    ($x:expr) => {
        ((($x) + PAGE_SIZE - 1) >> PAGE_SHIFT)
    };
}

#[macro_export]
macro_rules! PFN_DOWN {
    ($x:expr) => {
        (($x) >> PAGE_SHIFT)
    };
}

#[macro_export]
macro_rules! PFN_PHYS {
    ($x:expr) => {
        (($x as phys_addr_t) << PAGE_SHIFT)
    };
}

#[macro_export]
macro_rules! PHYS_PFN {
    ($x:expr) => {
        (($x >> PAGE_SHIFT) as usize)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
