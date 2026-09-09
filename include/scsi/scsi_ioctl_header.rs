/* SPDX-License-Identifier: GPL-2.0 */

pub const SCSI_IOCTL_SEND_COMMAND: i32 = 1;
pub const SCSI_IOCTL_TEST_UNIT_READY: i32 = 2;
pub const SCSI_IOCTL_BENCHMARK_COMMAND: i32 = 3;
pub const SCSI_IOCTL_SYNC: i32 = 4; // Request synchronous parameters
pub const SCSI_IOCTL_START_UNIT: i32 = 5;
pub const SCSI_IOCTL_STOP_UNIT: i32 = 6;
// The door lock/unlock constants are compatible with Sun constants for
// the cdrom
pub const SCSI_IOCTL_DOORLOCK: i32 = 0x5380; // lock the eject mechanism
pub const SCSI_IOCTL_DOORUNLOCK: i32 = 0x5381; // unlock the mechanism

pub const SCSI_REMOVAL_PREVENT: i32 = 1;
pub const SCSI_REMOVAL_ALLOW: i32 = 0;

// The following declarations are present only when compiling the kernel
// portion of the original header (__KERNEL__).
#[cfg(feature = "__KERNEL__")]
pub mod kernel {
    use core::ffi::c_void;
    use core::ffi::c_int;

    #[repr(C)]
    pub struct gendisk {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct scsi_device {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct sg_io_hdr {
        _private: [u8; 0],
    }

    /*
     * Structures used for scsi_ioctl et al.
     */

    #[repr(C)]
    pub struct scsi_ioctl_command {
        pub inlen: u32,
        pub outlen: u32,
        pub data: [u8; 0],
    }

    pub type Scsi_Ioctl_Command = scsi_ioctl_command;

    #[repr(C)]
    pub struct scsi_idlun {
        pub dev_id: u32,
        pub host_unique_id: u32,
    }

    pub type Scsi_Idlun = scsi_idlun;

    /* Fibre Channel WWN, port_id struct */
    #[repr(C)]
    pub struct scsi_fctargaddress {
        pub host_port_id: u32,
        pub host_wwn: [u8; 8], // include NULL term.
    }

    pub type Scsi_FCTargAddress = scsi_fctargaddress;

    unsafe extern "C" {
        pub fn scsi_ioctl_block_when_processing_errors(
            sdev: *mut scsi_device,
            cmd: c_int,
            ndelay: bool,
        ) -> c_int;
        pub fn scsi_ioctl(
            sdev: *mut scsi_device,
            open_for_write: bool,
            cmd: c_int,
            arg: *mut c_void,
        ) -> c_int;
        pub fn get_sg_io_hdr(hdr: *mut sg_io_hdr, argp: *const c_void) -> c_int;
        pub fn put_sg_io_hdr(hdr: *const sg_io_hdr, argp: *mut c_void) -> c_int;
        pub fn scsi_cmd_allowed(cmd: *mut u8, open_for_write: bool) -> bool;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
