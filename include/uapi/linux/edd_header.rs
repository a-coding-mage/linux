/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * linux/include/linux/edd.h
 *
 * Structures and definitions for the INT 13h BIOS Enhanced Disk Drive
 * Services.
 */

// Dependency supplied by the Linux types translation.

pub const EDDNR: u32 = 0x1e9;
pub const EDDBUF: u32 = 0xd00;
pub const EDDMAXNR: usize = 6;
pub const EDDEXTSIZE: usize = 8;
pub const EDDPARMSIZE: usize = 74;
pub const CHECKEXTENSIONSPRESENT: u8 = 0x41;
pub const GETDEVICEPARAMETERS: u8 = 0x48;
pub const LEGACYGETDEVICEPARAMETERS: u8 = 0x08;
pub const EDDMAGIC1: u16 = 0x55AA;
pub const EDDMAGIC2: u16 = 0xAA55;

pub const READ_SECTORS: u8 = 0x02;
pub const EDD_MBR_SIG_OFFSET: u32 = 0x1B8;
pub const EDD_MBR_SIG_BUF: u32 = 0x290;
pub const EDD_MBR_SIG_MAX: usize = 16;
pub const EDD_MBR_SIG_NR_BUF: u32 = 0x1ea;

pub const EDD_EXT_FIXED_DISK_ACCESS: u16 = 1 << 0;
pub const EDD_EXT_DEVICE_LOCKING_AND_EJECTING: u16 = 1 << 1;
pub const EDD_EXT_ENHANCED_DISK_DRIVE_SUPPORT: u16 = 1 << 2;
pub const EDD_EXT_64BIT_EXTENSIONS: u16 = 1 << 3;

pub const EDD_INFO_DMA_BOUNDARY_ERROR_TRANSPARENT: u16 = 1 << 0;
pub const EDD_INFO_GEOMETRY_VALID: u16 = 1 << 1;
pub const EDD_INFO_REMOVABLE: u16 = 1 << 2;
pub const EDD_INFO_WRITE_VERIFY: u16 = 1 << 3;
pub const EDD_INFO_MEDIA_CHANGE_NOTIFICATION: u16 = 1 << 4;
pub const EDD_INFO_LOCKABLE: u16 = 1 << 5;
pub const EDD_INFO_NO_MEDIA_PRESENT: u16 = 1 << 6;
pub const EDD_INFO_USE_INT13_FN50: u16 = 1 << 7;

#[repr(C, packed)]
pub struct edd_device_params_interface_path_isa {
    pub base_address: u16,
    pub reserved1: u16,
    pub reserved2: u32,
}

#[repr(C, packed)]
pub struct edd_device_params_interface_path_pci {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
    pub channel: u8,
    pub reserved: u32,
}

#[repr(C, packed)]
pub struct edd_device_params_interface_path_u64 {
    pub reserved: u64,
}

#[repr(C)]
pub union edd_device_params_interface_path {
    pub isa: edd_device_params_interface_path_isa,
    pub pci: edd_device_params_interface_path_pci,
    pub ibnd: edd_device_params_interface_path_u64,
    pub xprs: edd_device_params_interface_path_u64,
    pub htpt: edd_device_params_interface_path_u64,
    pub unknown: edd_device_params_interface_path_u64,
}

#[repr(C, packed)]
pub struct edd_device_params_device_path_ata {
    pub device: u8,
    pub reserved1: u8,
    pub reserved2: u16,
    pub reserved3: u32,
    pub reserved4: u64,
}

#[repr(C, packed)]
pub struct edd_device_params_device_path_atapi {
    pub device: u8,
    pub lun: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub reserved3: u32,
    pub reserved4: u64,
}

#[repr(C, packed)]
pub struct edd_device_params_device_path_scsi {
    pub id: u16,
    pub lun: u64,
    pub reserved1: u16,
    pub reserved2: u32,
}

#[repr(C, packed)]
pub struct edd_device_params_device_path_u64_pair {
    pub first: u64,
    pub second: u64,
}

#[repr(C, packed)]
pub struct edd_device_params_device_path_raid {
    pub array_number: u32,
    pub reserved1: u32,
    pub reserved2: u64,
}

#[repr(C)]
pub union edd_device_params_device_path {
    pub ata: edd_device_params_device_path_ata,
    pub atapi: edd_device_params_device_path_atapi,
    pub scsi: edd_device_params_device_path_scsi,
    pub usb: edd_device_params_device_path_u64_pair,
    pub i1394: edd_device_params_device_path_u64_pair,
    pub fibre: edd_device_params_device_path_u64_pair,
    pub i2o: edd_device_params_device_path_u64_pair,
    pub raid: edd_device_params_device_path_raid,
    pub sata: edd_device_params_device_path_ata,
    pub unknown: edd_device_params_device_path_u64_pair,
}

#[repr(C, packed)]
pub struct edd_device_params {
    pub length: u16,
    pub info_flags: u16,
    pub num_default_cylinders: u32,
    pub num_default_heads: u32,
    pub sectors_per_track: u32,
    pub number_of_sectors: u64,
    pub bytes_per_sector: u16,
    pub dpte_ptr: u32,
    pub key: u16,
    pub device_path_info_length: u8,
    pub reserved2: u8,
    pub reserved3: u16,
    pub host_bus_type: [u8; 4],
    pub interface_type: [u8; 8],
    pub interface_path: edd_device_params_interface_path,
    pub device_path: edd_device_params_device_path,
    pub reserved4: u8,
    pub checksum: u8,
}

#[repr(C, packed)]
pub struct edd_info {
    pub device: u8,
    pub version: u8,
    pub interface_support: u16,
    pub legacy_max_cylinder: u16,
    pub legacy_max_head: u8,
    pub legacy_sectors_per_track: u8,
    pub params: edd_device_params,
}

#[repr(C)]
pub struct edd {
    pub mbr_signature: [core::ffi::c_uint; EDD_MBR_SIG_MAX],
    pub edd_info: [edd_info; EDDMAXNR],
    pub mbr_signature_nr: u8,
    pub edd_info_nr: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
