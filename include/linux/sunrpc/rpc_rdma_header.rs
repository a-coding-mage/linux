/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * Copyright (c) 2015-2017 Oracle. All rights reserved.
 * Copyright (c) 2003-2007 Network Appliance, Inc. All rights reserved.
 *
 * This software is available under a choice of one of two
 * licenses. You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the BSD-type
 * license below.
 */

// Translated from the C header. Linux endian/XDR helpers are supplied externally.

pub const RPCRDMA_VERSION: u32 = 1;
pub const rpcrdma_version: u32 = cpu_to_be32(RPCRDMA_VERSION);

pub const RPCRDMA_V1_DEF_INLINE_SIZE: u32 = 1024;

pub const rpcrdma_fixed_maxsz: u32 = 4;
pub const rpcrdma_segment_maxsz: u32 = 4;
pub const rpcrdma_readseg_maxsz: u32 = 1 + rpcrdma_segment_maxsz;
pub const rpcrdma_readchunk_maxsz: u32 = 1 + rpcrdma_readseg_maxsz;

pub const RPCRDMA_HDRLEN_MIN: usize = core::mem::size_of::<u32>() * 7;
pub const RPCRDMA_HDRLEN_ERR: usize = core::mem::size_of::<u32>() * 5;

#[repr(u32)]
pub enum rpcrdma_errcode {
    ERR_VERS = 1,
    ERR_CHUNK = 2,
}

#[repr(u32)]
pub enum rpcrdma_proc {
    RDMA_MSG = 0,   // An RPC call or reply msg
    RDMA_NOMSG = 1, // An RPC call or reply msg - separate body
    RDMA_MSGP = 2,  // An RPC call or reply msg with padding
    RDMA_DONE = 3,  // Client signals reply completion
    RDMA_ERROR = 4, // An RPC RDMA encoding error
}

pub const rdma_msg: u32 = cpu_to_be32(rpcrdma_proc::RDMA_MSG as u32);
pub const rdma_nomsg: u32 = cpu_to_be32(rpcrdma_proc::RDMA_NOMSG as u32);
pub const rdma_msgp: u32 = cpu_to_be32(rpcrdma_proc::RDMA_MSGP as u32);
pub const rdma_done: u32 = cpu_to_be32(rpcrdma_proc::RDMA_DONE as u32);
pub const rdma_error: u32 = cpu_to_be32(rpcrdma_proc::RDMA_ERROR as u32);

pub const err_vers: u32 = cpu_to_be32(rpcrdma_errcode::ERR_VERS as u32);
pub const err_chunk: u32 = cpu_to_be32(rpcrdma_errcode::ERR_CHUNK as u32);

/*
 * Private extension to RPC-over-RDMA Version One.
 * Message passed during RDMA-CM connection set-up.
 *
 * Add new fields at the end, and don't permute existing fields.
 */
#[repr(C, packed)]
pub struct rpcrdma_connect_private {
    pub cp_magic: u32,
    pub cp_version: u8,
    pub cp_flags: u8,
    pub cp_send_size: u8,
    pub cp_recv_size: u8,
}

pub const rpcrdma_cmp_magic: u32 = 0xf6ab0e18;

pub const RPCRDMA_CMP_VERSION: u32 = 1;
pub const RPCRDMA_CMP_F_SND_W_INV_OK: u32 = 1 << 0;

#[inline]
pub fn rpcrdma_encode_buffer_size(size: u32) -> u8 {
    ((size >> 10) - 1) as u8
}

#[inline]
pub fn rpcrdma_decode_buffer_size(val: u8) -> u32 {
    ((val as u32) + 1) << 10
}

/**
 * xdr_encode_rdma_segment - Encode contents of an RDMA segment
 * @p: Pointer into a send buffer
 * @handle: The RDMA handle to encode
 * @length: The RDMA length to encode
 * @offset: The RDMA offset to encode
 *
 * Return value:
 *   Pointer to the XDR position that follows the encoded RDMA segment
 */
#[inline]
pub unsafe fn xdr_encode_rdma_segment(
    mut p: *mut u32,
    handle: u32,
    length: u32,
    offset: u64,
) -> *mut u32 {
    *p = cpu_to_be32(handle);
    p = p.add(1);
    *p = cpu_to_be32(length);
    p = p.add(1);
    xdr_encode_hyper(p, offset)
}

/** Encode contents of a Read segment. */
#[inline]
pub unsafe fn xdr_encode_read_segment(
    mut p: *mut u32,
    position: u32,
    handle: u32,
    length: u32,
    offset: u64,
) -> *mut u32 {
    *p = cpu_to_be32(position);
    p = p.add(1);
    xdr_encode_rdma_segment(p, handle, length, offset)
}

/** Decode contents of an RDMA segment. */
#[inline]
pub unsafe fn xdr_decode_rdma_segment(
    mut p: *const u32,
    handle: *mut u32,
    length: *mut u32,
    offset: *mut u64,
) -> *const u32 {
    *handle = be32_to_cpup(p);
    p = p.add(1);
    *length = be32_to_cpup(p);
    p = p.add(1);
    xdr_decode_hyper(p, offset)
}

/** Decode contents of a Read segment. */
#[inline]
pub unsafe fn xdr_decode_read_segment(
    mut p: *const u32,
    position: *mut u32,
    handle: *mut u32,
    length: *mut u32,
    offset: *mut u64,
) -> *const u32 {
    *position = be32_to_cpup(p);
    p = p.add(1);
    xdr_decode_rdma_segment(p, handle, length, offset)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
