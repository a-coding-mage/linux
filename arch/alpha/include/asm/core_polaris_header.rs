/* SPDX-License-Identifier: GPL-2.0 */

/*
 * POLARIS is the internal name for a core logic chipset which provides
 * memory controller and PCI access for the 21164PC chip based systems.
 *
 * This file is based on:
 *
 * Polaris System Controller
 * Device Functional Specification
 * 22-Jan-98
 * Rev. 4.2
 */

/* Polaris memory regions. `IDENT_ADDR` is supplied by the surrounding Alpha
 * address-space definitions. */
pub const POLARIS_SPARSE_MEM_BASE: u64 = IDENT_ADDR + 0xf800_0000_00;
pub const POLARIS_DENSE_MEM_BASE: u64 = IDENT_ADDR + 0xf900_0000_00;
pub const POLARIS_SPARSE_IO_BASE: u64 = IDENT_ADDR + 0xf980_0000_00;
pub const POLARIS_SPARSE_CONFIG_BASE: u64 = IDENT_ADDR + 0xf9c0_0000_00;
pub const POLARIS_IACK_BASE: u64 = IDENT_ADDR + 0xf9f8_0000_00;
pub const POLARIS_DENSE_IO_BASE: u64 = IDENT_ADDR + 0xf9fc_0000_00;
pub const POLARIS_DENSE_CONFIG_BASE: u64 = IDENT_ADDR + 0xf9fe_0000_00;

pub const POLARIS_IACK_SC: u64 = POLARIS_IACK_BASE;

/* The Polaris command/status registers live in PCI Config space for
 * bus 0/device 0. As such, they may be bytes, words, or doublewords.
 */
pub const POLARIS_W_VENID: u64 = POLARIS_DENSE_CONFIG_BASE;
pub const POLARIS_W_DEVID: u64 = POLARIS_DENSE_CONFIG_BASE + 2;
pub const POLARIS_W_CMD: u64 = POLARIS_DENSE_CONFIG_BASE + 4;
pub const POLARIS_W_STATUS: u64 = POLARIS_DENSE_CONFIG_BASE + 6;

/* Data structure for handling POLARIS machine checks. */
#[repr(C)]
pub struct el_POLARIS_sysdata_mcheck {
    pub psc_status: u64,
    pub psc_pcictl0: u64,
    pub psc_pcictl1: u64,
    pub psc_pcictl2: u64,
}

/* The following declarations are present only when building the kernel. */
#[cfg(feature = "kernel")]
pub unsafe fn polaris_ioportmap(addr: u64) -> *mut core::ffi::c_void {
    (addr + POLARIS_DENSE_IO_BASE) as *mut core::ffi::c_void
}

#[cfg(feature = "kernel")]
pub unsafe fn polaris_ioremap(addr: u64, _size: u64) -> *mut core::ffi::c_void {
    (addr + POLARIS_DENSE_MEM_BASE) as *mut core::ffi::c_void
}

#[cfg(feature = "kernel")]
pub unsafe fn polaris_is_ioaddr(addr: u64) -> i32 {
    (addr >= POLARIS_SPARSE_MEM_BASE) as i32
}

#[cfg(feature = "kernel")]
pub unsafe fn polaris_is_mmio(addr: *const core::ffi::c_void) -> i32 {
    ((addr as u64) < POLARIS_SPARSE_IO_BASE) as i32
}

/*
 * The C header includes asm/io_trivial.h with these prefix and capability
 * macros. Its generated trivial BWX/LQ I/O declarations are supplied by the
 * surrounding Rust translation and are intentionally not reimplemented here.
 */
#[cfg(feature = "kernel")]
pub const POLARIS_TRIVIAL_RW_BW: i32 = 1;
#[cfg(feature = "kernel")]
pub const POLARIS_TRIVIAL_RW_LQ: i32 = 1;
#[cfg(feature = "kernel")]
pub const POLARIS_TRIVIAL_IO_BW: i32 = 1;
#[cfg(feature = "kernel")]
pub const POLARIS_TRIVIAL_IO_LQ: i32 = 1;
#[cfg(feature = "kernel")]
pub const POLARIS_TRIVIAL_IOUNMAP: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
