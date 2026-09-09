/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by <uapi/asm/setup_data.h>. */

#[repr(C)]
pub struct pci_setup_rom {
    pub data: setup_data,
    pub vendor: u16,
    pub devid: u16,
    pub pcilen: u64,
    pub segment: core::ffi::c_ulong,
    pub bus: core::ffi::c_ulong,
    pub device: core::ffi::c_ulong,
    pub function: core::ffi::c_ulong,
    pub romdata: [u8; 0],
}

/* kexec external ABI */
#[repr(C)]
pub struct efi_setup_data {
    pub fw_vendor: u64,
    pub __unused: u64,
    pub tables: u64,
    pub smbios: u64,
    pub reserved: [u64; 8],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
