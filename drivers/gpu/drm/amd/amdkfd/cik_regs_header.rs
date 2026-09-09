/*
 * Copyright 2014 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// if PTR32, these are the bases for scratch and lds
pub const fn PRIVATE_BASE(x: u32) -> u32 { x << 0 } // scratch
pub const fn SHARED_BASE(x: u32) -> u32 { x << 16 } // LDS
pub const PTR32: u32 = 1 << 0;
pub const fn ALIGNMENT_MODE(x: u32) -> u32 { x << 2 }
pub const SH_MEM_ALIGNMENT_MODE_UNALIGNED: u32 = 3;
pub const fn DEFAULT_MTYPE(x: u32) -> u32 { x << 4 }
pub const fn APE1_MTYPE(x: u32) -> u32 { x << 7 }

// valid for both DEFAULT_MTYPE and APE1_MTYPE
pub const MTYPE_CACHED_NV: u32 = 0;
pub const MTYPE_CACHED: u32 = 1;
pub const MTYPE_NONCACHED: u32 = 3;

pub const DEFAULT_CP_HQD_PERSISTENT_STATE: u32 = 0x33u32 << 8;
pub const PRELOAD_REQ: u32 = 1 << 0;

pub const MQD_CONTROL_PRIV_STATE_EN: u32 = 1u32 << 8;

pub const DEFAULT_MIN_IB_AVAIL_SIZE: u32 = 3u32 << 20;

pub const IB_ATC_EN: u32 = 1 << 23;

pub const QUANTUM_EN: u32 = 1u32;
pub const QUANTUM_SCALE_1MS: u32 = 1u32 << 4;
pub const fn QUANTUM_DURATION(x: u32) -> u32 { x << 8 }

pub const fn RPTR_BLOCK_SIZE(x: u32) -> u32 { x << 8 }
pub const fn MIN_AVAIL_SIZE(x: u32) -> u32 { x << 20 }
pub const DEFAULT_RPTR_BLOCK_SIZE: u32 = RPTR_BLOCK_SIZE(5);
pub const DEFAULT_MIN_AVAIL_SIZE: u32 = MIN_AVAIL_SIZE(3);

pub const PQ_ATC_EN: u32 = 1 << 23;
pub const NO_UPDATE_RPTR: u32 = 1 << 27;

pub const fn DOORBELL_OFFSET(x: u32) -> u32 { x << 2 }
pub const DOORBELL_EN: u32 = 1 << 30;

pub const PRIV_STATE: u32 = 1 << 30;
pub const KMD_QUEUE: u32 = 1 << 31;

pub const AQL_ENABLE: u32 = 1;

pub const GRBM_GFX_INDEX: u32 = 0x30800;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
