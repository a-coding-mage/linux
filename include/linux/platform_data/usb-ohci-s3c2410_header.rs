/* SPDX-License-Identifier: GPL-2.0-only */
/* arch/arm/plat-samsung/include/plat/usb-control.h
 *
 * Copyright (c) 2004 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C - USB host port information
 */

// __ASM_ARCH_USBCONTROL_H

pub const S3C_HCDFLG_USED: ::core::ffi::c_int = 1;

#[repr(C)]
pub struct s3c2410_hcd_port {
	pub flags: u8,
	pub power: u8,
	pub oc_status: u8,
	pub oc_changed: u8,
}

#[repr(C)]
pub struct s3c2410_hcd_info {
	pub hcd: *mut usb_hcd,
	pub port: [s3c2410_hcd_port; 2],

	pub power_control: Option<unsafe extern "C" fn(port: ::core::ffi::c_int, to: ::core::ffi::c_int)>,
	pub enable_oc: Option<unsafe extern "C" fn(info: *mut s3c2410_hcd_info, on: ::core::ffi::c_int)>,
	pub report_oc: Option<unsafe extern "C" fn(info: *mut s3c2410_hcd_info, ports: ::core::ffi::c_int)>,
}

#[inline]
pub unsafe fn s3c2410_usb_report_oc(info: *mut s3c2410_hcd_info, ports: ::core::ffi::c_int) {
	if let Some(report_oc) = (*info).report_oc {
		report_oc(info, ports);
	}
}

unsafe extern "C" {
	pub fn s3c_ohci_set_platdata(info: *mut s3c2410_hcd_info);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
