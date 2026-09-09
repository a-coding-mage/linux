/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2019 NXP */

/* DPDMAI Version */
pub const DPDMAI_VER_MAJOR: u32 = 3;
pub const DPDMAI_VER_MINOR: u32 = 3;

pub const DPDMAI_CMD_BASE_VERSION: u32 = 1;
pub const DPDMAI_CMD_ID_OFFSET: u32 = 4;

/* Maximum number of Tx/Rx queues per DPDMAI object */
pub const DPDMAI_MAX_QUEUE_NUM: u32 = 8;

pub const fn DPDMAI_CMDID_FORMAT_V(x: u32, v: u32) -> u32 {
    (x << DPDMAI_CMD_ID_OFFSET) | v
}
pub const fn DPDMAI_CMDID_FORMAT(x: u32) -> u32 {
    DPDMAI_CMDID_FORMAT_V(x, DPDMAI_CMD_BASE_VERSION)
}

/* Command IDs */
pub const DPDMAI_CMDID_CLOSE: u32 = DPDMAI_CMDID_FORMAT(0x800);
pub const DPDMAI_CMDID_OPEN: u32 = DPDMAI_CMDID_FORMAT(0x80E);
pub const DPDMAI_CMDID_CREATE: u32 = DPDMAI_CMDID_FORMAT(0x90E);
pub const DPDMAI_CMDID_DESTROY: u32 = DPDMAI_CMDID_FORMAT(0x900);

pub const DPDMAI_CMDID_ENABLE: u32 = DPDMAI_CMDID_FORMAT(0x002);
pub const DPDMAI_CMDID_DISABLE: u32 = DPDMAI_CMDID_FORMAT(0x003);
pub const DPDMAI_CMDID_GET_ATTR: u32 = DPDMAI_CMDID_FORMAT(0x004);
pub const DPDMAI_CMDID_RESET: u32 = DPDMAI_CMDID_FORMAT(0x005);
pub const DPDMAI_CMDID_IS_ENABLED: u32 = DPDMAI_CMDID_FORMAT(0x006);

pub const DPDMAI_CMDID_SET_RX_QUEUE: u32 = DPDMAI_CMDID_FORMAT_V(0x1A0, 2);
pub const DPDMAI_CMDID_GET_RX_QUEUE: u32 = DPDMAI_CMDID_FORMAT_V(0x1A1, 2);
pub const DPDMAI_CMDID_GET_TX_QUEUE: u32 = DPDMAI_CMDID_FORMAT_V(0x1A2, 2);

pub const MC_CMD_HDR_TOKEN_O: u32 = 32; /* Token field offset */
pub const MC_CMD_HDR_TOKEN_S: u32 = 16; /* Token field size */

pub const fn MAKE_UMASK64(width: u32) -> u64 {
    if width < 64 { (1u64 << width).wrapping_sub(1) } else { u64::MAX }
}

/* Data Path DMA Interface API
 * Contains initialization APIs and runtime control APIs for DPDMAI
 */

/* Maximum number of Tx/Rx priorities per DPDMAI object */
pub const DPDMAI_PRIO_NUM: usize = 2;

/* DPDMAI queue modification options */
/* Select to modify the user's context associated with the queue */
pub const DPDMAI_QUEUE_OPT_USER_CTX: u32 = 0x1;
/* Select to modify the queue's destination */
pub const DPDMAI_QUEUE_OPT_DEST: u32 = 0x2;

#[repr(C)]
pub struct dpdmai_cfg {
    pub num_queues: u8,
    pub priorities: [u8; DPDMAI_PRIO_NUM],
}

#[repr(C)]
pub struct dpdmai_attr {
    pub id: i32,
    pub version: dpdmai_attr_version,
    pub num_of_priorities: u8,
    pub num_of_queues: u8,
}

#[repr(C)]
pub struct dpdmai_attr_version {
    pub major: u16,
    pub minor: u16,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpdmai_dest {
    DPDMAI_DEST_NONE = 0,
    DPDMAI_DEST_DPIO = 1,
    DPDMAI_DEST_DPCON = 2,
}

#[repr(C)]
pub struct dpdmai_dest_cfg {
    pub dest_type: dpdmai_dest,
    pub dest_id: i32,
    pub priority: u8,
}

#[repr(C)]
pub struct dpdmai_rx_queue_cfg {
    pub dest_cfg: dpdmai_dest_cfg,
    pub options: u32,
    pub user_ctx: u64,
}

#[repr(C)]
pub struct dpdmai_rx_queue_attr {
    pub dest_cfg: dpdmai_dest_cfg,
    pub user_ctx: u64,
    pub fqid: u32,
}

#[repr(C)]
pub struct dpdmai_tx_queue_attr {
    pub fqid: u32,
}

/* Supplied by the surrounding dependency environment. */
#[repr(C)]
pub struct fsl_mc_io {
    _private: [u8; 0],
}

extern "C" {
    pub fn dpdmai_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, dpdmai_id: i32, token: *mut u16) -> i32;
    pub fn dpdmai_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpdmai_destroy(mc_io: *mut fsl_mc_io, cmd_flags: u32, dpdmai_id: u32, token: u16) -> i32;
    pub fn dpdmai_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpdmai_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpdmai_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpdmai_get_attributes(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, attr: *mut dpdmai_attr) -> i32;
    pub fn dpdmai_set_rx_queue(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, queue_idx: u8, priority: u8, cfg: *const dpdmai_rx_queue_cfg) -> i32;
    pub fn dpdmai_get_rx_queue(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, queue_idx: u8, priority: u8, attr: *mut dpdmai_rx_queue_attr) -> i32;
    pub fn dpdmai_get_tx_queue(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, queue_idx: u8, priority: u8, attr: *mut dpdmai_tx_queue_attr) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
