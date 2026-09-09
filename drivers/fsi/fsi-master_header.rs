/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * FSI master definitions. These comprise the core <--> master interface,
 * to allow the core to interact with the (hardware-specific) masters.
 *
 * Copyright (C) IBM Corporation 2016
 */

/* Dependency intent: linux::device::device and linux::mutex::mutex are
 * supplied by the surrounding kernel translation. */

/* Master registers */
pub const FSI_MMODE: u32 = 0x0;
pub const FSI_MDLYR: u32 = 0x4;
pub const FSI_MCRSP: u32 = 0x8;
pub const FSI_MENP0: u32 = 0x10;
pub const FSI_MLEVP0: u32 = 0x18;
pub const FSI_MSENP0: u32 = 0x18;
pub const FSI_MCENP0: u32 = 0x20;
pub const FSI_MAEB: u32 = 0x70;
pub const FSI_MVER: u32 = 0x74;
pub const FSI_MSTAP0: u32 = 0xd0;
pub const FSI_MRESP0: u32 = 0xd0;
pub const FSI_MESRB0: u32 = 0x1d0;
pub const FSI_MRESB0: u32 = 0x1d0;
pub const FSI_MSCSB0: u32 = 0x1d4;
pub const FSI_MATRB0: u32 = 0x1d8;
pub const FSI_MDTRB0: u32 = 0x1dc;
pub const FSI_MECTRL: u32 = 0x2e0;

pub const FSI_MMODE_EIP: u32 = 0x80000000;
pub const FSI_MMODE_ECRC: u32 = 0x40000000;
pub const FSI_MMODE_RELA: u32 = 0x20000000;
pub const FSI_MMODE_EPC: u32 = 0x10000000;
pub const FSI_MMODE_P8_TO_LSB: u32 = 0x00000010;
pub const FSI_MMODE_CRS0SHFT: u32 = 18;
pub const FSI_MMODE_CRS0MASK: u32 = 0x3ff;
pub const FSI_MMODE_CRS1SHFT: u32 = 8;
pub const FSI_MMODE_CRS1MASK: u32 = 0x3ff;

pub const FSI_MRESB_RST_GEN: u32 = 0x80000000;
pub const FSI_MRESB_RST_ERR: u32 = 0x40000000;
pub const FSI_MRESP_RST_ALL_MASTER: u32 = 0x20000000;
pub const FSI_MRESP_RST_ALL_LINK: u32 = 0x10000000;
pub const FSI_MRESP_RST_MCR: u32 = 0x08000000;
pub const FSI_MRESP_RST_PYE: u32 = 0x04000000;
pub const FSI_MRESP_RST_ALL: u32 = 0xfc000000;
pub const FSI_MECTRL_EOAE: u16 = 0x8000;
pub const FSI_MECTRL_P8_AUTO_TERM: u16 = 0x4000;

pub const FSI_HUB_LINK_OFFSET: u32 = 0x80000;
pub const FSI_HUB_LINK_SIZE: u32 = 0x80000;
pub const FSI_HUB_MASTER_MAX_LINKS: u32 = 8;

pub const FSI_ECHO_DELAY_CLOCKS: u32 = 16;
pub const FSI_SEND_DELAY_CLOCKS: u32 = 16;
pub const FSI_PRE_BREAK_CLOCKS: u32 = 50;
pub const FSI_BREAK_CLOCKS: u32 = 256;
pub const FSI_POST_BREAK_CLOCKS: u32 = 16000;
pub const FSI_INIT_CLOCKS: u32 = 5000;
pub const FSI_MASTER_DPOLL_CLOCKS: u32 = 50;
pub const FSI_MASTER_EPOLL_CLOCKS: u32 = 50;
pub const FSI_CRC_ERR_RETRIES: u32 = 10;
pub const FSI_MASTER_MAX_BUSY: u32 = 200;
pub const FSI_MASTER_MTOE_COUNT: u32 = 1000;

pub const FSI_CMD_DPOLL: u8 = 0x2;
pub const FSI_CMD_EPOLL: u8 = 0x3;
pub const FSI_CMD_TERM: u8 = 0x3f;
pub const FSI_CMD_ABS_AR: u8 = 0x4;
pub const FSI_CMD_REL_AR: u8 = 0x5;
pub const FSI_CMD_SAME_AR: u8 = 0x3;
pub const FSI_RESP_ACK: u8 = 0;
pub const FSI_RESP_BUSY: u8 = 1;
pub const FSI_RESP_ERRA: u8 = 2;
pub const FSI_RESP_ERRC: u8 = 3;
pub const FSI_CRC_SIZE: u32 = 4;
pub const FSI_MASTER_FLAG_SWCLOCK: u32 = 0x1;

#[repr(C)]
pub struct fsi_master {
    pub dev: device,
    pub idx: core::ffi::c_int,
    pub n_links: core::ffi::c_int,
    pub flags: core::ffi::c_int,
    pub scan_lock: mutex,
    pub read: Option<unsafe extern "C" fn(*mut fsi_master, core::ffi::c_int, u8, u32, *mut core::ffi::c_void, usize) -> core::ffi::c_int>,
    pub write: Option<unsafe extern "C" fn(*mut fsi_master, core::ffi::c_int, u8, u32, *const core::ffi::c_void, usize) -> core::ffi::c_int>,
    pub term: Option<unsafe extern "C" fn(*mut fsi_master, core::ffi::c_int, u8) -> core::ffi::c_int>,
    pub send_break: Option<unsafe extern "C" fn(*mut fsi_master, core::ffi::c_int) -> core::ffi::c_int>,
    pub link_enable: Option<unsafe extern "C" fn(*mut fsi_master, core::ffi::c_int, bool) -> core::ffi::c_int>,
    pub link_config: Option<unsafe extern "C" fn(*mut fsi_master, core::ffi::c_int, u8, u8) -> core::ffi::c_int>,
}

/* Equivalent of: container_of(d, struct fsi_master, dev). */
#[macro_export]
macro_rules! to_fsi_master {
    ($d:expr) => {
        unsafe { &mut *((($d as *mut device) as *mut u8).sub(core::mem::offset_of!(fsi_master, dev)) as *mut fsi_master) }
    };
}

extern "C" {
    pub fn fsi_master_register(master: *mut fsi_master) -> core::ffi::c_int;
    pub fn fsi_master_unregister(master: *mut fsi_master);
    pub fn fsi_master_rescan(master: *mut fsi_master) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
