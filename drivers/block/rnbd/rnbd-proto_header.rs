/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RDMA Network Block Driver
 *
 * Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
 * Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
 * Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const RNBD_PROTO_VER_MAJOR: u32 = 2;
pub const RNBD_PROTO_VER_MINOR: u32 = 2;

/* The default port number the RTRS server is listening on. */
pub const RTRS_PORT: u32 = 1234;

/// enum rnbd_msg_type - RNBD message types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rnbd_msg_type {
    RNBD_MSG_SESS_INFO,
    RNBD_MSG_SESS_INFO_RSP,
    RNBD_MSG_OPEN,
    RNBD_MSG_OPEN_RSP,
    RNBD_MSG_IO,
    RNBD_MSG_CLOSE,
}

/// struct rnbd_msg_hdr - header of RNBD messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_hdr {
    pub r#type: __le16,
    pub __padding: __le16,
}

/*
 * We allow to map RO many times and RW only once. We allow to map yet another
 * time RW, if MIGRATION is provided (second RW export can be required for
 * example for VM migration)
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rnbd_access_mode {
    RNBD_ACCESS_RO,
    RNBD_ACCESS_RW,
    RNBD_ACCESS_MIGRATION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_access_mode_entry {
    pub mode: rnbd_access_mode,
    pub str_: *const core::ffi::c_char,
}

pub static rnbd_access_modes: [rnbd_access_mode_entry; 3] = [
    rnbd_access_mode_entry { mode: rnbd_access_mode::RNBD_ACCESS_RO, str_: b"ro\0".as_ptr() as *const _ },
    rnbd_access_mode_entry { mode: rnbd_access_mode::RNBD_ACCESS_RW, str_: b"rw\0".as_ptr() as *const _ },
    rnbd_access_mode_entry { mode: rnbd_access_mode::RNBD_ACCESS_MIGRATION, str_: b"migration\0".as_ptr() as *const _ },
];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_sess_info {
    pub hdr: rnbd_msg_hdr,
    pub ver: u8,
    pub reserved: [u8; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_sess_info_rsp {
    pub hdr: rnbd_msg_hdr,
    pub ver: u8,
    pub reserved: [u8; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_open {
    pub hdr: rnbd_msg_hdr,
    pub access_mode: u8,
    pub resv1: u8,
    pub dev_name: [i8; NAME_MAX],
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_close {
    pub hdr: rnbd_msg_hdr,
    pub device_id: __le32,
}

pub const RNBD_FUA: u32 = 1 << 0;
pub const RNBD_WRITEBACK: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_open_rsp {
    pub hdr: rnbd_msg_hdr,
    pub device_id: __le32,
    pub nsectors: __le64,
    pub max_hw_sectors: __le32,
    pub max_write_zeroes_sectors: __le32,
    pub max_discard_sectors: __le32,
    pub discard_granularity: __le32,
    pub discard_alignment: __le32,
    pub physical_block_size: __le16,
    pub logical_block_size: __le16,
    pub max_segments: __le16,
    pub secure_discard: __le16,
    pub obsolete_rotational: u8,
    pub cache_policy: u8,
    pub reserved: [u8; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_io {
    pub hdr: rnbd_msg_hdr,
    pub device_id: __le32,
    pub sector: __le64,
    pub rw: __le32,
    pub bi_size: __le32,
    pub prio: __le16,
}

pub const RNBD_OP_BITS: u32 = 8;
pub const RNBD_OP_MASK: u32 = (1 << RNBD_OP_BITS) - 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rnbd_io_flags {
    RNBD_OP_READ = 0,
    RNBD_OP_WRITE = 1,
    RNBD_OP_FLUSH = 2,
    RNBD_OP_DISCARD = 3,
    RNBD_OP_SECURE_ERASE = 4,
    RNBD_OP_WRITE_ZEROES = 5,
    RNBD_F_SYNC = 1 << (RNBD_OP_BITS + 0),
    RNBD_F_FUA = 1 << (RNBD_OP_BITS + 1),
    RNBD_F_PREFLUSH = 1 << (RNBD_OP_BITS + 2),
    RNBD_F_NOUNMAP = 1 << (RNBD_OP_BITS + 3),
}

#[inline]
pub const fn rnbd_op(flags: u32) -> u32 {
    flags & RNBD_OP_MASK
}

#[inline]
pub const fn rnbd_flags(flags: u32) -> u32 {
    flags & !RNBD_OP_MASK
}

#[inline]
pub unsafe fn rnbd_to_bio_flags(rnbd_opf: u32) -> blk_opf_t {
    let mut bio_opf: blk_opf_t;
    match rnbd_op(rnbd_opf) {
        RNBD_OP_READ => bio_opf = REQ_OP_READ,
        RNBD_OP_WRITE => bio_opf = REQ_OP_WRITE,
        RNBD_OP_FLUSH => bio_opf = REQ_OP_WRITE | REQ_PREFLUSH,
        RNBD_OP_DISCARD => bio_opf = REQ_OP_DISCARD,
        RNBD_OP_SECURE_ERASE => bio_opf = REQ_OP_SECURE_ERASE,
        RNBD_OP_WRITE_ZEROES => {
            bio_opf = REQ_OP_WRITE_ZEROES;
            if rnbd_opf & RNBD_F_NOUNMAP != 0 { bio_opf |= REQ_NOUNMAP; }
        }
        _ => {
            WARN(1, "Unknown RNBD type: %d (flags %d)\n", rnbd_op(rnbd_opf), rnbd_opf);
            bio_opf = 0;
        }
    }
    if rnbd_opf & RNBD_F_SYNC != 0 { bio_opf |= REQ_SYNC; }
    if rnbd_opf & RNBD_F_FUA != 0 { bio_opf |= REQ_FUA; }
    if rnbd_opf & RNBD_F_PREFLUSH != 0 { bio_opf |= REQ_PREFLUSH; }
    bio_opf
}

#[inline]
pub unsafe fn rq_to_rnbd_flags(rq: *mut request) -> u32 {
    let mut rnbd_opf: u32;
    match req_op(rq) {
        REQ_OP_READ => rnbd_opf = RNBD_OP_READ,
        REQ_OP_WRITE => rnbd_opf = RNBD_OP_WRITE,
        REQ_OP_DISCARD => rnbd_opf = RNBD_OP_DISCARD,
        REQ_OP_SECURE_ERASE => rnbd_opf = RNBD_OP_SECURE_ERASE,
        REQ_OP_WRITE_ZEROES => {
            rnbd_opf = RNBD_OP_WRITE_ZEROES;
            if (*rq).cmd_flags & REQ_NOUNMAP != 0 { rnbd_opf |= RNBD_F_NOUNMAP; }
        }
        REQ_OP_FLUSH => rnbd_opf = RNBD_OP_FLUSH,
        _ => {
            WARN(1, "Unknown request type %d (flags %llu)\n", req_op(rq) as u32, (*rq).cmd_flags as u64);
            rnbd_opf = 0;
        }
    }
    if op_is_sync((*rq).cmd_flags) { rnbd_opf |= RNBD_F_SYNC; }
    if op_is_flush((*rq).cmd_flags) { rnbd_opf |= RNBD_F_FUA; }
    if (*rq).cmd_flags & REQ_PREFLUSH != 0 { rnbd_opf |= RNBD_F_PREFLUSH; }
    rnbd_opf
}

extern "C" {
    pub fn rnbd_access_mode_str(mode: rnbd_access_mode) -> *const core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
