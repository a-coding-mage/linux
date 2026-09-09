/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: the C header includes <linux/types.h>, which supplies __u8
// and __u32. These names are left as external dependencies for the translated
// header.

pub const VMCOREDD_NOTE_NAME: &str = "LINUX";
pub const VMCOREDD_MAX_NAME_BYTES: usize = 44;

#[repr(C)]
pub struct vmcoredd_header {
	pub n_namesz: __u32, /* Name size */
	pub n_descsz: __u32, /* Content size */
	pub n_type: __u32,   /* NT_VMCOREDD */
	pub name: [__u8; 8], /* LINUX\0\0\0 */
	pub dump_name: [__u8; VMCOREDD_MAX_NAME_BYTES], /* Device dump's name */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hwerr_error_type {
	HWERR_RECOV_CPU,
	HWERR_RECOV_MEMORY,
	HWERR_RECOV_PCI,
	HWERR_RECOV_CXL,
	HWERR_RECOV_OTHERS,
	HWERR_RECOV_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
