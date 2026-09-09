/* SPDX-License-Identifier: GPL-2.0 */

/* for Loongson-3 smp support */
extern "C" {
    pub static mut smp_group: [u64; 4];
}

/* 4 groups(nodes) in maximum in numa case */
#[macro_export]
macro_rules! SMP_CORE_GROUP0_BASE {
    () => {
        unsafe { $crate::smp_group[0] }
    };
}

#[macro_export]
macro_rules! SMP_CORE_GROUP1_BASE {
    () => {
        unsafe { $crate::smp_group[1] }
    };
}

#[macro_export]
macro_rules! SMP_CORE_GROUP2_BASE {
    () => {
        unsafe { $crate::smp_group[2] }
    };
}

#[macro_export]
macro_rules! SMP_CORE_GROUP3_BASE {
    () => {
        unsafe { $crate::smp_group[3] }
    };
}

/* 4 cores in each group(node) */
pub const SMP_CORE0_OFFSET: u32 = 0x000;
pub const SMP_CORE1_OFFSET: u32 = 0x100;
pub const SMP_CORE2_OFFSET: u32 = 0x200;
pub const SMP_CORE3_OFFSET: u32 = 0x300;

/* ipi registers offsets */
pub const STATUS0: u32 = 0x00;
pub const EN0: u32 = 0x04;
pub const SET0: u32 = 0x08;
pub const CLEAR0: u32 = 0x0c;
pub const STATUS1: u32 = 0x10;
pub const MASK1: u32 = 0x14;
pub const SET1: u32 = 0x18;
pub const CLEAR1: u32 = 0x1c;
pub const BUF: u32 = 0x20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
