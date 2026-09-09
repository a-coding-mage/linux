/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* This header will be used by application too. */
/*
 * The mmaped area is divided into the mailbox, command ring, and data beyond
 * the command ring. The mailbox describes the command ring, while command
 * entries contain offsets into the shared area.
 */

pub const TCMU_VERSION: &str = "2.0";

pub const TCMU_MAILBOX_VERSION: u16 = 2;
pub const ALIGN_SIZE: usize = 64;
pub const TCMU_MAILBOX_FLAG_CAP_OOOC: u32 = 1 << 0;
pub const TCMU_MAILBOX_FLAG_CAP_READ_LEN: u32 = 1 << 1;
pub const TCMU_MAILBOX_FLAG_CAP_TMR: u32 = 1 << 2;
pub const TCMU_MAILBOX_FLAG_CAP_KEEP_BUF: u32 = 1 << 3;

#[repr(C, packed)]
pub struct tcmu_mailbox {
    pub version: __u16,
    pub flags: __u16,
    pub cmdr_off: __u32,
    pub cmdr_size: __u32,
    pub cmd_head: __u32,
    pub cmd_tail: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcmu_opcode {
    TCMU_OP_PAD = 0,
    TCMU_OP_CMD = 1,
    TCMU_OP_TMR = 2,
}

#[repr(C, packed)]
pub struct tcmu_cmd_entry_hdr {
    pub len_op: __u32,
    pub cmd_id: __u16,
    pub kflags: __u8,
    pub uflags: __u8,
}

pub const TCMU_UFLAG_UNKNOWN_OP: __u8 = 0x1;
pub const TCMU_UFLAG_READ_LEN: __u8 = 0x2;
pub const TCMU_UFLAG_KEEP_BUF: __u8 = 0x4;
pub const TCMU_OP_MASK: __u32 = 0x7;

#[inline]
pub unsafe fn tcmu_hdr_get_op(len_op: __u32) -> tcmu_opcode {
    core::mem::transmute((len_op & TCMU_OP_MASK) as i32)
}

#[inline]
pub unsafe fn tcmu_hdr_set_op(len_op: *mut __u32, op: tcmu_opcode) {
    *len_op &= !TCMU_OP_MASK;
    *len_op |= (op as __u32) & TCMU_OP_MASK;
}

#[inline]
pub fn tcmu_hdr_get_len(len_op: __u32) -> __u32 {
    len_op & !TCMU_OP_MASK
}

#[inline]
pub unsafe fn tcmu_hdr_set_len(len_op: *mut __u32, len: __u32) {
    *len_op &= TCMU_OP_MASK;
    *len_op |= len;
}

pub const TCMU_SENSE_BUFFERSIZE: usize = 96;

#[repr(C)]
pub struct tcmu_cmd_entry_req {
    pub iov_cnt: __u32,
    pub iov_bidi_cnt: __u32,
    pub iov_dif_cnt: __u32,
    pub cdb_off: __u64,
    pub __pad1: __u64,
    pub __pad2: __u64,
    pub iov: [iovec; 0],
}

#[repr(C)]
pub struct tcmu_cmd_entry_rsp {
    pub scsi_status: __u8,
    pub __pad1: __u8,
    pub __pad2: __u16,
    pub read_len: __u32,
    pub sense_buffer: [i8; TCMU_SENSE_BUFFERSIZE],
}

#[repr(C)]
pub union tcmu_cmd_entry_union {
    pub req: tcmu_cmd_entry_req,
    pub rsp: tcmu_cmd_entry_rsp,
}

#[repr(C, packed)]
pub struct tcmu_cmd_entry {
    pub hdr: tcmu_cmd_entry_hdr,
    pub __bindgen_anon_1: tcmu_cmd_entry_union,
}

pub const TCMU_TMR_UNKNOWN: __u8 = 0;
pub const TCMU_TMR_ABORT_TASK: __u8 = 1;
pub const TCMU_TMR_ABORT_TASK_SET: __u8 = 2;
pub const TCMU_TMR_CLEAR_ACA: __u8 = 3;
pub const TCMU_TMR_CLEAR_TASK_SET: __u8 = 4;
pub const TCMU_TMR_LUN_RESET: __u8 = 5;
pub const TCMU_TMR_TARGET_WARM_RESET: __u8 = 6;
pub const TCMU_TMR_TARGET_COLD_RESET: __u8 = 7;
pub const TCMU_TMR_LUN_RESET_PRO: __u8 = 128;

#[repr(C, packed)]
pub struct tcmu_tmr_entry {
    pub hdr: tcmu_cmd_entry_hdr,
    pub tmr_type: __u8,
    pub __pad1: __u8,
    pub __pad2: __u16,
    pub cmd_cnt: __u32,
    pub __pad3: __u64,
    pub __pad4: __u64,
    pub cmd_ids: [__u16; 0],
}

pub const TCMU_OP_ALIGN_SIZE: usize = core::mem::size_of::<__u64>();

#[repr(C)]
pub enum tcmu_genl_cmd {
    TCMU_CMD_UNSPEC,
    TCMU_CMD_ADDED_DEVICE,
    TCMU_CMD_REMOVED_DEVICE,
    TCMU_CMD_RECONFIG_DEVICE,
    TCMU_CMD_ADDED_DEVICE_DONE,
    TCMU_CMD_REMOVED_DEVICE_DONE,
    TCMU_CMD_RECONFIG_DEVICE_DONE,
    TCMU_CMD_SET_FEATURES,
    __TCMU_CMD_MAX,
}
pub const TCMU_CMD_MAX: u32 = tcmu_genl_cmd::__TCMU_CMD_MAX as u32 - 1;

#[repr(C)]
pub enum tcmu_genl_attr {
    TCMU_ATTR_UNSPEC,
    TCMU_ATTR_DEVICE,
    TCMU_ATTR_MINOR,
    TCMU_ATTR_PAD,
    TCMU_ATTR_DEV_CFG,
    TCMU_ATTR_DEV_SIZE,
    TCMU_ATTR_WRITECACHE,
    TCMU_ATTR_CMD_STATUS,
    TCMU_ATTR_DEVICE_ID,
    TCMU_ATTR_SUPP_KERN_CMD_REPLY,
    __TCMU_ATTR_MAX,
}
pub const TCMU_ATTR_MAX: u32 = tcmu_genl_attr::__TCMU_ATTR_MAX as u32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
