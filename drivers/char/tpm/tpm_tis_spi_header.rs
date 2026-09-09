/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Infineon Technologies AG
 * Copyright (C) 2016 STMicroelectronics SAS
 */

// Dependency supplied by tpm_tis_core.h.

#[repr(C)]
pub struct tpm_tis_spi_phy {
    pub r#priv: tpm_tis_data,
    pub spi_device: *mut spi_device,
    pub flow_control: Option<unsafe extern "C" fn(
        phy: *mut tpm_tis_spi_phy,
        xfer: *mut spi_transfer,
    ) -> ::core::ffi::c_int>,
    pub ready: completion,
    pub wake_after: ::core::ffi::c_ulong,
    pub iobuf: *mut u8,
}

#[inline]
pub unsafe fn to_tpm_tis_spi_phy(data: *mut tpm_tis_data) -> *mut tpm_tis_spi_phy {
    // Equivalent to Linux's container_of(data, struct tpm_tis_spi_phy, priv).
    let uninit = ::core::mem::MaybeUninit::<tpm_tis_spi_phy>::uninit();
    let base = uninit.as_ptr();
    let member = ::core::ptr::addr_of!((*base).r#priv);
    (data as *mut u8).sub(member as usize - base as usize) as *mut tpm_tis_spi_phy
}

extern "C" {
    pub fn tpm_tis_spi_init(
        spi: *mut spi_device,
        phy: *mut tpm_tis_spi_phy,
        irq: ::core::ffi::c_int,
        phy_ops: *const tpm_tis_phy_ops,
    ) -> ::core::ffi::c_int;

    pub fn tpm_tis_spi_transfer(
        data: *mut tpm_tis_data,
        addr: u32,
        len: u16,
        r#in: *mut u8,
        out: *const u8,
    ) -> ::core::ffi::c_int;
}

// CONFIG_TCG_TIS_SPI_CR50 controls whether this external implementation is available.
#[cfg(feature = "CONFIG_TCG_TIS_SPI_CR50")]
extern "C" {
    pub fn cr50_spi_probe(spi: *mut spi_device) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_TCG_TIS_SPI_CR50"))]
#[inline]
pub unsafe fn cr50_spi_probe(_spi: *mut spi_device) -> ::core::ffi::c_int {
    -19 // -ENODEV
}

// CONFIG_PM_SLEEP and CONFIG_TCG_TIS_SPI_CR50 select the resume interface.
#[cfg(all(feature = "CONFIG_PM_SLEEP", feature = "CONFIG_TCG_TIS_SPI_CR50"))]
extern "C" {
    pub fn tpm_tis_spi_resume(dev: *mut device) -> ::core::ffi::c_int;
}

#[cfg(all(feature = "CONFIG_PM_SLEEP", not(feature = "CONFIG_TCG_TIS_SPI_CR50")))]
pub use tpm_tis_resume as tpm_tis_spi_resume;

#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
pub const tpm_tis_spi_resume: Option<unsafe extern "C" fn(*mut device) -> ::core::ffi::c_int> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
