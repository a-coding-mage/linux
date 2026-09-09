// SPDX-License-Identifier: GPL-2.0
/*
 * USB CDC common helpers
 *
 * Copyright (c) 2015 Oliver Neukum <oneukum@suse.com>
 */

// Dependency declarations supplied by the uapi/linux/usb/cdc.h translation.

/*
 * inofficial magic numbers
 */

pub const CDC_PHONET_MAGIC_NUMBER: u8 = 0xAB;

/*
 * parsing CDC headers
 */

#[repr(C)]
pub struct usb_cdc_parsed_header {
    pub usb_cdc_union_desc: *mut usb_cdc_union_desc,
    pub usb_cdc_header_desc: *mut usb_cdc_header_desc,

    pub usb_cdc_call_mgmt_descriptor: *mut usb_cdc_call_mgmt_descriptor,
    pub usb_cdc_acm_descriptor: *mut usb_cdc_acm_descriptor,
    pub usb_cdc_country_functional_desc: *mut usb_cdc_country_functional_desc,
    pub usb_cdc_network_terminal_desc: *mut usb_cdc_network_terminal_desc,
    pub usb_cdc_ether_desc: *mut usb_cdc_ether_desc,
    pub usb_cdc_dmm_desc: *mut usb_cdc_dmm_desc,
    pub usb_cdc_mdlm_desc: *mut usb_cdc_mdlm_desc,
    pub usb_cdc_mdlm_detail_desc: *mut usb_cdc_mdlm_detail_desc,
    pub usb_cdc_obex_desc: *mut usb_cdc_obex_desc,
    pub usb_cdc_ncm_desc: *mut usb_cdc_ncm_desc,
    pub usb_cdc_mbim_desc: *mut usb_cdc_mbim_desc,
    pub usb_cdc_mbim_extended_desc: *mut usb_cdc_mbim_extended_desc,

    pub phonet_magic_present: bool,
}

pub struct usb_interface;

unsafe extern "C" {
    pub fn cdc_parse_cdc_header(
        hdr: *mut usb_cdc_parsed_header,
        intf: *mut usb_interface,
        buffer: *mut u8,
        buflen: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
