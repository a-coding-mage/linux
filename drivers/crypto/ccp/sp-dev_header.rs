/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Secure Processor driver
 *
 * Copyright (C) 2017-2019 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 * Author: Gary R Hook <gary.hook@amd.com>
 * Author: Brijesh Singh <brijesh.singh@amd.com>
 */

// Linux header dependencies are supplied by other translated units.

pub const SP_MAX_NAME_LEN: usize = 32;

pub const CACHE_NONE: u32 = 0x00;
pub const CACHE_WB_NO_ALLOC: u32 = 0xb7;

pub const PLATFORM_FEATURE_DBC: u32 = 0x1;
pub const PLATFORM_FEATURE_HSTI: u32 = 0x2;

// The original macro depends on the enclosing PSP device type's `vdata`
// member, which is supplied by another translated header.
#[macro_export]
macro_rules! PSP_FEATURE {
    ($psp:expr, DBC) => {
        unsafe {
            !($psp).vdata.is_null()
                && ((*($psp).vdata).platform_features & PLATFORM_FEATURE_DBC) != 0
        }
    };
    ($psp:expr, HSTI) => {
        unsafe {
            !($psp).vdata.is_null()
                && ((*($psp).vdata).platform_features & PLATFORM_FEATURE_HSTI) != 0
        }
    };
}

/* Structure to hold CCP device data */
#[repr(C)]
pub struct ccp_device;

#[repr(C)]
pub struct ccp_vdata {
    pub version: u32,
    pub dma_chan_attr: u32,
    pub setup: Option<unsafe extern "C" fn(*mut ccp_device)>,
    pub perform: *const ccp_actions,
    pub offset: u32,
    pub rsamax: u32,
}

#[repr(C)]
pub struct sev_vdata {
    pub cmdresp_reg: u32,
    pub cmdbuff_addr_lo_reg: u32,
    pub cmdbuff_addr_hi_reg: u32,
}

#[repr(C)]
pub struct tee_vdata {
    pub cmdresp_reg: u32,
    pub cmdbuff_addr_lo_reg: u32,
    pub cmdbuff_addr_hi_reg: u32,
    pub ring_wptr_reg: u32,
    pub ring_rptr_reg: u32,
    pub info_reg: u32,
}

#[repr(C)]
pub struct platform_access_vdata {
    pub cmdresp_reg: u32,
    pub cmdbuff_addr_lo_reg: u32,
    pub cmdbuff_addr_hi_reg: u32,
    pub doorbell_button_reg: u32,
    pub doorbell_cmd_reg: u32,
}

#[repr(C)]
pub struct psp_vdata {
    pub sev: *const sev_vdata,
    pub tee: *const tee_vdata,
    pub platform_access: *const platform_access_vdata,
    pub cmdresp_reg: u32,
    pub cmdbuff_addr_lo_reg: u32,
    pub cmdbuff_addr_hi_reg: u32,
    pub feature_reg: u32,
    pub inten_reg: u32,
    pub intsts_reg: u32,
    pub bootloader_info_reg: u32,
    pub platform_features: u32,
}

/* Structure to hold SP device data */
#[repr(C)]
pub struct sp_dev_vdata {
    pub bar: u32,
    pub ccp_vdata: *const ccp_vdata,
    pub psp_vdata: *const psp_vdata,
}

#[repr(C)]
pub struct sp_device {
    pub entry: list_head,
    pub dev: *mut device,
    pub dev_vdata: *const sp_dev_vdata,
    pub ord: u32,
    pub name: [::std::os::raw::c_char; SP_MAX_NAME_LEN],
    /* Bus specific device information */
    pub dev_specific: *mut ::std::ffi::c_void,
    /* I/O area used for device communication. */
    pub io_map: *mut ::std::ffi::c_void,
    /* DMA caching attribute support */
    pub axcache: u32,
    /* get and set master device */
    pub get_psp_master_device: Option<unsafe extern "C" fn() -> *mut sp_device>,
    pub set_psp_master_device: Option<unsafe extern "C" fn(*mut sp_device)>,
    pub clear_psp_master_device: Option<unsafe extern "C" fn(*mut sp_device)>,
    pub irq_registered: bool,
    pub use_tasklet: bool,
    pub ccp_irq: u32,
    pub ccp_irq_handler: irq_handler_t,
    pub ccp_irq_data: *mut ::std::ffi::c_void,
    pub psp_irq: u32,
    pub psp_irq_handler: irq_handler_t,
    pub psp_irq_data: *mut ::std::ffi::c_void,
    pub ccp_data: *mut ::std::ffi::c_void,
    pub psp_data: *mut ::std::ffi::c_void,
}

extern "C" {
    pub fn sp_pci_init() -> ::std::os::raw::c_int;
    pub fn sp_pci_exit();
    pub fn sp_platform_init() -> ::std::os::raw::c_int;
    pub fn sp_platform_exit();
    pub fn sp_alloc_struct(dev: *mut device) -> *mut sp_device;
    pub fn sp_init(sp: *mut sp_device) -> ::std::os::raw::c_int;
    pub fn sp_destroy(sp: *mut sp_device);
    pub fn sp_suspend(sp: *mut sp_device) -> ::std::os::raw::c_int;
    pub fn sp_resume(sp: *mut sp_device) -> ::std::os::raw::c_int;
    pub fn sp_restore(sp: *mut sp_device) -> ::std::os::raw::c_int;
    pub fn sp_request_ccp_irq(sp: *mut sp_device, handler: irq_handler_t, name: *const ::std::os::raw::c_char, data: *mut ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn sp_free_ccp_irq(sp: *mut sp_device, data: *mut ::std::ffi::c_void);
    pub fn sp_request_psp_irq(sp: *mut sp_device, handler: irq_handler_t, name: *const ::std::os::raw::c_char, data: *mut ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn sp_free_psp_irq(sp: *mut sp_device, data: *mut ::std::ffi::c_void);
    pub fn sp_get_psp_master_device() -> *mut sp_device;
}

// CONFIG_CRYPTO_DEV_SP_CCP and CONFIG_CRYPTO_DEV_SP_PSP are build-time
// conditions from the original header and are preserved by these cfg gates.
#[cfg(feature = "CONFIG_CRYPTO_DEV_SP_CCP")]
extern "C" {
    pub fn ccp_dev_init(sp: *mut sp_device) -> ::std::os::raw::c_int;
    pub fn ccp_dev_destroy(sp: *mut sp_device);
    pub fn ccp_dev_suspend(sp: *mut sp_device);
    pub fn ccp_dev_resume(sp: *mut sp_device);
}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_CCP"))]
pub unsafe fn ccp_dev_init(_sp: *mut sp_device) -> ::std::os::raw::c_int { 0 }
#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_CCP"))]
pub unsafe fn ccp_dev_destroy(_sp: *mut sp_device) {}
#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_CCP"))]
pub unsafe fn ccp_dev_suspend(_sp: *mut sp_device) {}
#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_CCP"))]
pub unsafe fn ccp_dev_resume(_sp: *mut sp_device) {}

#[cfg(feature = "CONFIG_CRYPTO_DEV_SP_PSP")]
extern "C" {
    pub fn psp_dev_init(sp: *mut sp_device) -> ::std::os::raw::c_int;
    pub fn psp_pci_init();
    pub fn psp_dev_destroy(sp: *mut sp_device);
    pub fn psp_pci_exit();
    pub fn psp_restore(sp: *mut sp_device) -> ::std::os::raw::c_int;
}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_PSP"))]
pub unsafe fn psp_dev_init(_sp: *mut sp_device) -> ::std::os::raw::c_int { 0 }
#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_PSP"))]
pub unsafe fn psp_pci_init() {}
#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_PSP"))]
pub unsafe fn psp_dev_destroy(_sp: *mut sp_device) {}
#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_PSP"))]
pub unsafe fn psp_pci_exit() {}
#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_PSP"))]
pub unsafe fn psp_restore(_sp: *mut sp_device) -> ::std::os::raw::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
