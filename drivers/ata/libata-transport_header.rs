/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _LIBATA_TRANSPORT_H

// External dependency: defined by the surrounding libata translation unit.
#[repr(C)]
pub struct scsi_transport_template {
    _private: [u8; 0],
}

extern "C" {
    pub static mut ata_scsi_transportt: scsi_transport_template;

    pub fn ata_tlink_add(link: *mut ata_link) -> ::core::ffi::c_int;
    pub fn ata_tlink_delete(link: *mut ata_link);

    // C __init annotation preserved as intent; supplied by the build environment.
    pub fn libata_transport_init() -> ::core::ffi::c_int;
    // C __exit annotation preserved as intent; supplied by the build environment.
    pub fn libata_transport_exit();
}

// External dependency: struct ata_link is defined by the surrounding libata translation unit.
#[repr(C)]
pub struct ata_link {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
