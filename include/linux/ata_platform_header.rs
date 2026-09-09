/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __LINUX_ATA_PLATFORM_H

#[repr(C)]
pub struct pata_platform_info {
    /*
     * I/O port shift, for platforms with ports that are
     * constantly spaced and need larger than the 1-byte
     * spacing used by ata_std_ports().
     */
    pub ioport_shift: ::core::ffi::c_uint,
}

// External type supplied by another dependency.
#[repr(C)]
pub struct scsi_host_template {
    _private: [u8; 0],
}

// External types supplied by another dependency.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn __pata_platform_probe(
        dev: *mut device,
        io_res: *mut resource,
        ctl_res: *mut resource,
        irq_res: *mut resource,
        ioport_shift: ::core::ffi::c_uint,
        __pio_mask: ::core::ffi::c_int,
        sht: *const scsi_host_template,
        use16bit: bool,
    ) -> ::core::ffi::c_int;
}

/*
 * Marvell SATA private data
 */
#[repr(C)]
pub struct mv_sata_platform_data {
    pub n_ports: ::core::ffi::c_int, /* number of sata ports */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
