/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/***************************************************************************
 *    copyright            : (C) 2002 by Frank Mori Hess
 ***************************************************************************/

// Translated from gpib_ioctl.h. Linux ioctl encoding constants are supplied
// by the target platform in the original header.

pub const GPIB_CODE: u32 = 160;

#[repr(C)]
pub struct gpib_board_type_ioctl {
    pub name: [i8; 100],
}

/* argument for read/write/command ioctls */
#[repr(C)]
pub struct gpib_read_write_ioctl {
    pub buffer_ptr: u64,
    pub requested_transfer_count: u32,
    pub completed_transfer_count: u32,
    pub end: i32, /* end flag return for reads, end io suppression request for cmd */
    pub handle: i32,
}

#[repr(C)]
pub struct gpib_open_dev_ioctl {
    pub handle: u32,
    pub pad: u32,
    pub sad: i32,
    pub is_board: u32,
}

#[repr(C)]
pub struct gpib_close_dev_ioctl {
    pub handle: u32,
}

#[repr(C)]
pub struct gpib_serial_poll_ioctl {
    pub pad: u32,
    pub sad: i32,
    pub status_byte: u8,
    pub padding: [u8; 3], /* align to 32 bit boundary */
}

#[repr(C)]
pub struct gpib_eos_ioctl {
    pub eos: i32,
    pub eos_flags: i32,
}

#[repr(C)]
pub struct gpib_wait_ioctl {
    pub handle: i32,
    pub wait_mask: i32,
    pub clear_mask: i32,
    pub set_mask: i32,
    pub ibsta: i32,
    pub pad: i32,
    pub sad: i32,
    pub usec_timeout: u32,
}

#[repr(C)]
pub struct gpib_online_ioctl {
    pub init_data_ptr: u64,
    pub init_data_length: i32,
    pub online: i32,
}

#[repr(C)]
pub struct gpib_spoll_bytes_ioctl {
    pub num_bytes: u32,
    pub pad: u32,
    pub sad: i32,
}

#[repr(C)]
pub struct gpib_board_info_ioctl {
    pub pad: u32,
    pub sad: i32,
    pub parallel_poll_configuration: i32,
    pub autopolling: i32,
    pub is_system_controller: i32,
    pub t1_delay: u32,
    pub ist: u32,
    pub no_7_bit_eos: u32,
    pub padding: u32, /* align to 32 bit boundary */
}

#[repr(C)]
pub struct gpib_select_pci_ioctl {
    pub pci_bus: i32,
    pub pci_slot: i32,
}

#[repr(C)]
pub struct gpib_ppoll_config_ioctl {
    pub config: u8,
    pub _padding0: [u8; 3],
    pub set_ist: u32,
    pub clear_ist: u32,
    pub padding: u32, /* align to 32 bit boundary */
}

#[repr(C)]
pub struct gpib_pad_ioctl {
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct gpib_sad_ioctl {
    pub handle: u32,
    pub sad: i32,
}

/* select a piece of hardware to attach by its sysfs device path */
#[repr(C)]
pub struct gpib_select_device_path_ioctl {
    pub device_path: [i8; 0x1000],
}

/* update status byte and request service */
#[repr(C)]
pub struct gpib_request_service2 {
    pub status_byte: u8,
    pub padding: [u8; 3], /* align to 32 bit boundary */
    pub new_reason_for_service: i32,
}

// C bit-fields are represented by their containing 32-bit storage words.
// The ioctl encoding helpers below correspond to Linux _IO, _IOR, _IOW, _IOWR.
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    (dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT)
        | ((size as u32) << IOC_SIZESHIFT)
}
const fn io(ty: u32, nr: u32) -> u32 { ioc(IOC_NONE, ty, nr, 0) }
const fn ior<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_READ, ty, nr, core::mem::size_of::<T>()) }
const fn iow<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_WRITE, ty, nr, core::mem::size_of::<T>()) }
const fn iowr<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_READ | IOC_WRITE, ty, nr, core::mem::size_of::<T>()) }

/* Standard functions. */
pub const IBRD: u32 = iowr::<gpib_read_write_ioctl>(GPIB_CODE, 100);
pub const IBWRT: u32 = iowr::<gpib_read_write_ioctl>(GPIB_CODE, 101);
pub const IBCMD: u32 = iowr::<gpib_read_write_ioctl>(GPIB_CODE, 102);
pub const IBOPENDEV: u32 = iowr::<gpib_open_dev_ioctl>(GPIB_CODE, 3);
pub const IBCLOSEDEV: u32 = iow::<gpib_close_dev_ioctl>(GPIB_CODE, 4);
pub const IBWAIT: u32 = iowr::<gpib_wait_ioctl>(GPIB_CODE, 5);
pub const IBRPP: u32 = iowr::<u8>(GPIB_CODE, 6);
pub const IBSIC: u32 = iow::<u32>(GPIB_CODE, 9);
pub const IBSRE: u32 = iow::<i32>(GPIB_CODE, 10);
pub const IBGTS: u32 = io(GPIB_CODE, 11);
pub const IBCAC: u32 = iow::<i32>(GPIB_CODE, 12);
pub const IBLINES: u32 = ior::<i16>(GPIB_CODE, 14);
pub const IBPAD: u32 = iow::<gpib_pad_ioctl>(GPIB_CODE, 15);
pub const IBSAD: u32 = iow::<gpib_sad_ioctl>(GPIB_CODE, 16);
pub const IBTMO: u32 = iow::<u32>(GPIB_CODE, 17);
pub const IBRSP: u32 = iowr::<gpib_serial_poll_ioctl>(GPIB_CODE, 18);
pub const IBEOS: u32 = iow::<gpib_eos_ioctl>(GPIB_CODE, 19);
pub const IBRSV: u32 = iow::<u8>(GPIB_CODE, 20);
pub const CFCBASE: u32 = iow::<u64>(GPIB_CODE, 21);
pub const CFCIRQ: u32 = iow::<u32>(GPIB_CODE, 22);
pub const CFCDMA: u32 = iow::<u32>(GPIB_CODE, 23);
pub const CFCBOARDTYPE: u32 = iow::<gpib_board_type_ioctl>(GPIB_CODE, 24);
pub const IBMUTEX: u32 = iow::<i32>(GPIB_CODE, 26);
pub const IBSPOLL_BYTES: u32 = iowr::<gpib_spoll_bytes_ioctl>(GPIB_CODE, 27);
pub const IBPPC: u32 = iow::<gpib_ppoll_config_ioctl>(GPIB_CODE, 28);
pub const IBBOARD_INFO: u32 = ior::<gpib_board_info_ioctl>(GPIB_CODE, 29);
pub const IBQUERY_BOARD_RSV: u32 = ior::<i32>(GPIB_CODE, 31);
pub const IBSELECT_PCI: u32 = iowr::<gpib_select_pci_ioctl>(GPIB_CODE, 32);
pub const IBEVENT: u32 = ior::<i16>(GPIB_CODE, 33);
pub const IBRSC: u32 = iow::<i32>(GPIB_CODE, 34);
pub const IB_T1_DELAY: u32 = iow::<u32>(GPIB_CODE, 35);
pub const IBLOC: u32 = io(GPIB_CODE, 36);
pub const IBAUTOSPOLL: u32 = iow::<i16>(GPIB_CODE, 38);
pub const IBONL: u32 = iow::<gpib_online_ioctl>(GPIB_CODE, 39);
pub const IBPP2_SET: u32 = iow::<i16>(GPIB_CODE, 40);
pub const IBPP2_GET: u32 = ior::<i16>(GPIB_CODE, 41);
pub const IBSELECT_DEVICE_PATH: u32 = iow::<gpib_select_device_path_ioctl>(GPIB_CODE, 43);
/* 44 was IBSELECT_SERIAL_NUMBER */
pub const IBRSV2: u32 = iow::<gpib_request_service2>(GPIB_CODE, 45);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
