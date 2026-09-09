/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ioctl interface for /dev/chsc
 *
 * Copyright IBM Corp. 2008, 2012
 * Author(s): Cornelia Huck <cornelia.huck@de.ibm.com>
 */

// Dependencies supplied by the surrounding kernel bindings:
// linux types, ioctl encoding, asm/chpid.h, and asm/schid.h.

pub const CHSC_SIZE: usize = 0x1000;

#[repr(C)]
pub struct chsc_async_header {
    pub length: u16,
    pub code: u16,
    pub cmd_dependend: u32,
    // C bitfield: key occupies the low 4 bits; the remaining 28 bits are unused.
    pub key_and_reserved: u32,
    pub sid: subchannel_id,
}

#[repr(C)]
pub struct chsc_async_area {
    pub header: chsc_async_header,
    pub data: [u8; CHSC_SIZE - core::mem::size_of::<chsc_async_header>()],
}

#[repr(C)]
pub struct chsc_header {
    pub length: u16,
    pub code: u16,
}

#[repr(C)]
pub struct chsc_sync_area {
    pub header: chsc_header,
    pub data: [u8; CHSC_SIZE - core::mem::size_of::<chsc_header>()],
}

#[repr(C)]
pub struct chsc_response_struct {
    pub length: u16,
    pub code: u16,
    pub parms: u32,
    pub data: [u8; CHSC_SIZE - 2 * core::mem::size_of::<u16>() - core::mem::size_of::<u32>()],
}

#[repr(C)]
pub struct chsc_chp_cd {
    pub chpid: chp_id,
    pub m: i32,
    pub fmt: i32,
    pub cpcb: chsc_response_struct,
}

#[repr(C)]
pub struct chsc_cu_cd {
    pub cun: u16,
    pub cssid: u8,
    pub m: i32,
    pub fmt: i32,
    pub cucb: chsc_response_struct,
}

#[repr(C)]
pub struct chsc_sch_cud {
    pub schid: subchannel_id,
    pub fmt: i32,
    pub scub: chsc_response_struct,
}

#[repr(C)]
pub struct conf_id {
    pub m: i32,
    pub cssid: u8,
    pub ssid: u8,
}

#[repr(C)]
pub struct chsc_conf_info {
    pub id: conf_id,
    pub fmt: i32,
    pub scid: chsc_response_struct,
}

#[repr(C)]
pub struct ccl_parm_chpid {
    pub m: i32,
    pub chp: chp_id,
}

#[repr(C)]
pub struct ccl_parm_cssids {
    pub f_cssid: u8,
    pub l_cssid: u8,
}

#[repr(C)]
pub struct chsc_comp_list_req {
    pub ctype: i32,
    pub fmt: i32,
    pub chpid: ccl_parm_chpid,
    pub cssids: ccl_parm_cssids,
}

pub const CCL_CU_ON_CHP: i32 = 1;
pub const CCL_CHP_TYPE_CAP: i32 = 2;
pub const CCL_CSS_IMG: i32 = 4;
pub const CCL_CSS_IMG_CONF_CHAR: i32 = 5;
pub const CCL_IOP_CHP: i32 = 6;

#[repr(C)]
pub struct chsc_comp_list {
    pub req: chsc_comp_list_req,
    pub sccl: chsc_response_struct,
}

#[repr(C)]
pub struct chsc_dcal_req {
    pub atype: i32,
    pub list_parm: [u32; 2],
    pub fmt: i32,
}

pub const DCAL_CSS_IID_PN: i32 = 4;

#[repr(C)]
pub struct chsc_dcal {
    pub req: chsc_dcal_req,
    pub sdcal: chsc_response_struct,
}

#[repr(C)]
pub struct chsc_cpd_info {
    pub chpid: chp_id,
    pub m: i32,
    pub fmt: i32,
    pub rfmt: i32,
    pub c: i32,
    pub chpdb: chsc_response_struct,
}

pub const CHSC_IOCTL_MAGIC: u8 = b'c';

const IOC_NRBITS: usize = 8;
const IOC_TYPEBITS: usize = 8;
const IOC_SIZEBITS: usize = 14;
const IOC_NRSHIFT: usize = 0;
const IOC_TYPESHIFT: usize = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: usize = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: usize = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: usize = 1;
const IOC_READ: usize = 2;

const fn ioc(dir: usize, magic: u8, nr: u8, size: usize) -> usize {
    (dir << IOC_DIRSHIFT) | ((size & ((1usize << IOC_SIZEBITS) - 1)) << IOC_SIZESHIFT)
        | ((magic as usize) << IOC_TYPESHIFT) | ((nr as usize) << IOC_NRSHIFT)
}
const fn iowr<T>(magic: u8, nr: u8) -> usize {
    ioc(IOC_READ | IOC_WRITE, magic, nr, core::mem::size_of::<T>())
}
const fn io(magic: u8, nr: u8) -> usize { ioc(0, magic, nr, 0) }

pub const CHSC_START: usize = iowr::<chsc_async_area>(CHSC_IOCTL_MAGIC, 0x81);
pub const CHSC_INFO_CHANNEL_PATH: usize = iowr::<chsc_chp_cd>(CHSC_IOCTL_MAGIC, 0x82);
pub const CHSC_INFO_CU: usize = iowr::<chsc_cu_cd>(CHSC_IOCTL_MAGIC, 0x83);
pub const CHSC_INFO_SCH_CU: usize = iowr::<chsc_sch_cud>(CHSC_IOCTL_MAGIC, 0x84);
pub const CHSC_INFO_CI: usize = iowr::<chsc_conf_info>(CHSC_IOCTL_MAGIC, 0x85);
pub const CHSC_INFO_CCL: usize = iowr::<chsc_comp_list>(CHSC_IOCTL_MAGIC, 0x86);
pub const CHSC_INFO_CPD: usize = iowr::<chsc_cpd_info>(CHSC_IOCTL_MAGIC, 0x87);
pub const CHSC_INFO_DCAL: usize = iowr::<chsc_dcal>(CHSC_IOCTL_MAGIC, 0x88);
pub const CHSC_START_SYNC: usize = iowr::<chsc_sync_area>(CHSC_IOCTL_MAGIC, 0x89);
pub const CHSC_ON_CLOSE_SET: usize = iowr::<chsc_async_area>(CHSC_IOCTL_MAGIC, 0x8a);
pub const CHSC_ON_CLOSE_REMOVE: usize = io(CHSC_IOCTL_MAGIC, 0x8b);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
