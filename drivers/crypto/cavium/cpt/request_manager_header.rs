/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Cavium, Inc.
 */

// Dependency declarations from cpt_common.h are intentionally not reproduced here.

pub const TIME_IN_RESET_COUNT: u32 = 5;
pub const COMPLETION_CODE_SIZE: u32 = 8;
pub const COMPLETION_CODE_INIT: u32 = 0;
pub const PENDING_THOLD: u32 = 100;
pub const MAX_SG_IN_CNT: u32 = 12;
pub const MAX_SG_OUT_CNT: u32 = 13;
pub const SG_LIST_HDR_SIZE: u32 = 8;
pub const MAX_BUF_CNT: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub union ctrl_info {
    pub flags: u32,
    pub s: ctrl_info_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctrl_info_s {
    // Bitfield ordering follows __BIG_ENDIAN_BITFIELD when that build condition is enabled.
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union opcode_info {
    pub flags: u16,
    pub s: opcode_info_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opcode_info_s {
    pub major: u8,
    pub minor: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cptvf_request {
    pub opcode: opcode_info,
    pub param1: u16,
    pub param2: u16,
    pub dlen: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buf_ptr {
    pub vptr: *mut u8,
    pub dma_addr: dma_addr_t,
    pub size: u16,
}

#[repr(C)]
pub struct cpt_request_info {
    pub incnt: u8,
    pub outcnt: u8,
    pub rlen: u16,
    pub ctrl: ctrl_info,
    pub req: cptvf_request,
    pub may_sleep: bool,
    pub r#in: [buf_ptr; MAX_BUF_CNT],
    pub out: [buf_ptr; MAX_BUF_CNT],
    pub callback: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void)>,
    pub callback_arg: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sglist_component_u {
    pub len: u64,
    pub s: sglist_component_u_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sglist_component_u_s {
    pub len0: u16,
    pub len1: u16,
    pub len2: u16,
    pub len3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sglist_component {
    pub u: sglist_component_u,
    pub ptr0: u64,
    pub ptr1: u64,
    pub ptr2: u64,
    pub ptr3: u64,
}

#[repr(C)]
pub struct cpt_info_buffer {
    pub cptvf: *mut cpt_vf,
    pub time_in: core::ffi::c_ulong,
    pub extra_time: u8,
    pub req: *mut cpt_request_info,
    pub dptr_baddr: dma_addr_t,
    pub dlen: u32,
    pub rptr_baddr: dma_addr_t,
    pub comp_baddr: dma_addr_t,
    pub in_buffer: *mut u8,
    pub out_buffer: *mut u8,
    pub gather_components: *mut u8,
    pub scatter_components: *mut u8,
    pub pentry: *mut pending_entry,
    pub completion_addr: *mut u64,
    pub alternate_caddr: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vq_cmd_word0 {
    pub u64: u64,
    pub s: vq_cmd_word0_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vq_cmd_word0_s {
    pub opcode: u16,
    pub param1: u16,
    pub param2: u16,
    pub dlen: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vq_cmd_word3 {
    pub u64: u64,
    pub s: vq_cmd_word3_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vq_cmd_word3_s {
    // Bitfield ordering follows __BIG_ENDIAN_BITFIELD when that build condition is enabled.
    pub bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_vq_command {
    pub cmd: vq_cmd_word0,
    pub dptr: u64,
    pub rptr: u64,
    pub cptr: vq_cmd_word3,
}

extern "C" {
    pub fn vq_post_process(cptvf: *mut cpt_vf, qno: u32);
    pub fn process_request(cptvf: *mut cpt_vf, req: *mut cpt_request_info) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
