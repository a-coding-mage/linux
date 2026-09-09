/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by linux/mmc/host.h and linux/interrupt.h are
// intentionally referenced here rather than reimplemented.

pub enum device {}
pub enum mmc_host {}
pub enum property_entry {}

#[repr(C)]
pub struct pxamci_platform_data {
    pub ocr_mask: ::core::ffi::c_uint, // available voltages
    pub detect_delay_ms: ::core::ffi::c_ulong, // delay in millisecond before detecting cards after interrupt
    pub init: Option<unsafe extern "C" fn(*mut device, irq_handler_t, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub get_ro: Option<unsafe extern "C" fn(*mut device) -> ::core::ffi::c_int>,
    pub setpower: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut device, *mut ::core::ffi::c_void)>,
    pub gpio_card_ro_invert: bool, // gpio ro is inverted
}

extern "C" {
    pub fn pxa_set_mci_info(
        info: *const pxamci_platform_data,
        props: *const property_entry,
    );
    pub fn pxa3xx_set_mci2_info(info: *mut pxamci_platform_data);
    pub fn pxa3xx_set_mci3_info(info: *mut pxamci_platform_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
