/* SPDX-License-Identifier: GPL-2.0-only */

/* Declarations from <uapi/asm/setup.h> are supplied externally. */

/*
 * We leave one page for the initial stack page, and one page for
 * the initial process structure. Also, the console eats 3 MB for
 * the initial bootloader (one of which we can reclaim later).
 */
pub const BOOT_PCB: usize = 0x20000000;
pub const BOOT_ADDR: usize = 0x20000000;
/* Remove when official MILO sources have ELF support: */
pub const BOOT_SIZE: usize = 16 * 1024;

#[cfg(feature = "CONFIG_ALPHA_LEGACY_START_ADDRESS")]
pub const KERNEL_START_PHYS: usize = 0x300000; /* Old bootloaders hardcoded this. */

#[cfg(not(feature = "CONFIG_ALPHA_LEGACY_START_ADDRESS"))]
pub const KERNEL_START_PHYS: usize = 0x1000000; /* required: Wildfire/Titan/Marvel */

/* PAGE_OFFSET is supplied by the target environment. */
pub const KERNEL_START: usize = PAGE_OFFSET + KERNEL_START_PHYS;
pub const SWAPPER_PGD: usize = KERNEL_START;
pub const INIT_STACK: usize = PAGE_OFFSET + KERNEL_START_PHYS + 0x02000;
pub const EMPTY_PGT: usize = PAGE_OFFSET + KERNEL_START_PHYS + 0x04000;
pub const EMPTY_PGE: usize = PAGE_OFFSET + KERNEL_START_PHYS + 0x08000;
pub const ZERO_PGE: usize = PAGE_OFFSET + KERNEL_START_PHYS + 0x0A000;

pub const START_ADDR: usize = PAGE_OFFSET + KERNEL_START_PHYS + 0x10000;

/*
 * This is setup by the secondary bootstrap loader. Because
 * the zero page is zeroed out as soon as the vm system is
 * initialized, we need to copy things out into a more permanent
 * place.
 */
pub const PARAM: usize = ZERO_PGE;

/* absolute_pointer is supplied by the target environment. */
#[macro_export]
macro_rules! COMMAND_LINE {
    () => {
        absolute_pointer($crate::PARAM + 0x0000) as *mut core::ffi::c_char
    };
}

#[macro_export]
macro_rules! INITRD_START {
    () => {
        *(($crate::PARAM + 0x100) as *const ::core::ffi::c_ulong)
    };
}

#[macro_export]
macro_rules! INITRD_SIZE {
    () => {
        *(($crate::PARAM + 0x108) as *const ::core::ffi::c_ulong)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
