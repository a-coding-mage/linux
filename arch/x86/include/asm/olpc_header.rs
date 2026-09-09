/* SPDX-License-Identifier: GPL-2.0 */
/* OLPC machine specific definitions */

/* Dependency supplied by asm/geode.h in the C source. */

#[repr(C)]
pub struct olpc_platform_t {
    pub flags: ::core::ffi::c_int,
    pub boardrev: u32,
}

pub const OLPC_F_PRESENT: ::core::ffi::c_int = 0x01;
pub const OLPC_F_DCON: ::core::ffi::c_int = 0x02;

/* CONFIG_OLPC conditional preserved from the C header. */
#[cfg(feature = "CONFIG_OLPC")]
extern "C" {
    pub static mut olpc_platform_info: olpc_platform_t;
}

#[cfg(feature = "CONFIG_OLPC")]
#[inline]
pub fn olpc_board(id: u8) -> u32 {
    ((id as u32) << 4) | 0x8
}

#[cfg(feature = "CONFIG_OLPC")]
#[inline]
pub fn olpc_board_pre(id: u8) -> u32 {
    (id as u32) << 4
}

#[cfg(feature = "CONFIG_OLPC")]
#[inline]
pub unsafe fn machine_is_olpc() -> ::core::ffi::c_int {
    if (olpc_platform_info.flags & OLPC_F_PRESENT) != 0 { 1 } else { 0 }
}

#[cfg(feature = "CONFIG_OLPC")]
#[inline]
pub unsafe fn olpc_has_dcon() -> ::core::ffi::c_int {
    if (olpc_platform_info.flags & OLPC_F_DCON) != 0 { 1 } else { 0 }
}

#[cfg(feature = "CONFIG_OLPC")]
#[inline]
pub unsafe fn olpc_board_at_least(rev: u32) -> ::core::ffi::c_int {
    if olpc_platform_info.boardrev >= rev { 1 } else { 0 }
}

#[cfg(not(feature = "CONFIG_OLPC"))]
#[inline]
pub fn machine_is_olpc() -> ::core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_OLPC"))]
#[inline]
pub fn olpc_has_dcon() -> ::core::ffi::c_int { 0 }

/* CONFIG_OLPC_XO1_PM conditional preserved from the C header. */
#[cfg(feature = "CONFIG_OLPC_XO1_PM")]
extern "C" {
    pub fn do_olpc_suspend_lowlevel();
    pub fn olpc_xo1_pm_wakeup_set(value: u16);
    pub fn olpc_xo1_pm_wakeup_clear(value: u16);
}

extern "C" {
    pub fn pci_olpc_init() -> ::core::ffi::c_int;
}

/* GPIO assignments */
pub const OLPC_GPIO_MIC_AC: u32 = 1;
pub const OLPC_GPIO_DCON_STAT0: u32 = 5;
pub const OLPC_GPIO_DCON_STAT1: u32 = 6;
pub const OLPC_GPIO_DCON_IRQ: u32 = 7;
/* geode_gpio is supplied by asm/geode.h in the C source. */
pub const OLPC_GPIO_THRM_ALRM: u32 = geode_gpio(10);
pub const OLPC_GPIO_DCON_LOAD: u32 = 11;
pub const OLPC_GPIO_DCON_BLANK: u32 = 12;
pub const OLPC_GPIO_SMB_CLK: u32 = 14;
pub const OLPC_GPIO_SMB_DATA: u32 = 15;
pub const OLPC_GPIO_WORKAUX: u32 = geode_gpio(24);
pub const OLPC_GPIO_LID: u32 = 26;
pub const OLPC_GPIO_ECSCI: u32 = 27;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
