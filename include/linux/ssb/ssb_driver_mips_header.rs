/* SPDX-License-Identifier: GPL-2.0 */
// Translated from ssb_driver_mips.h.
// The CONFIG_* conditions are represented as Rust crate features.

pub struct ssb_device;

#[cfg(feature = "CONFIG_SSB_DRIVER_MIPS")]
#[repr(C)]
pub struct ssb_serial_port {
    pub regs: *mut core::ffi::c_void,
    pub clockspeed: core::ffi::c_ulong,
    pub irq: core::ffi::c_uint,
    pub baud_base: core::ffi::c_uint,
    pub reg_shift: core::ffi::c_uint,
}

#[cfg(feature = "CONFIG_SSB_DRIVER_MIPS")]
#[repr(C)]
pub struct ssb_pflash {
    pub present: bool,
    pub buswidth: u8,
    pub window: u32,
    pub window_size: u32,
}

#[cfg(all(feature = "CONFIG_SSB_DRIVER_MIPS", feature = "CONFIG_SSB_SFLASH"))]
#[repr(C)]
pub struct ssb_sflash {
    pub present: bool,
    pub window: u32,
    pub blocksize: u32,
    pub numblocks: u16,
    pub size: u32,
    pub priv_: *mut core::ffi::c_void,
}

#[cfg(feature = "CONFIG_SSB_DRIVER_MIPS")]
#[repr(C)]
pub struct ssb_mipscore {
    pub dev: *mut ssb_device,
    pub nr_serial_ports: core::ffi::c_int,
    pub serial_ports: [ssb_serial_port; 4],
    pub pflash: ssb_pflash,
    #[cfg(feature = "CONFIG_SSB_SFLASH")]
    pub sflash: ssb_sflash,
}

#[cfg(feature = "CONFIG_SSB_DRIVER_MIPS")]
unsafe extern "C" {
    pub fn ssb_mipscore_init(mcore: *mut ssb_mipscore);
    pub fn ssb_cpu_clock(mcore: *mut ssb_mipscore) -> u32;
    pub fn ssb_mips_irq(dev: *mut ssb_device) -> core::ffi::c_uint;
}

#[cfg(not(feature = "CONFIG_SSB_DRIVER_MIPS"))]
#[repr(C)]
pub struct ssb_mipscore {}

#[cfg(not(feature = "CONFIG_SSB_DRIVER_MIPS"))]
#[inline]
pub unsafe fn ssb_mipscore_init(_mcore: *mut ssb_mipscore) {}

#[cfg(not(feature = "CONFIG_SSB_DRIVER_MIPS"))]
#[inline]
pub unsafe fn ssb_mips_irq(_dev: *mut ssb_device) -> core::ffi::c_uint {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
