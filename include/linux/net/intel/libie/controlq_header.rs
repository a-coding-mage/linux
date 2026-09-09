/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2025 Intel Corporation */

// Translated from the C header. Included kernel types and helpers are supplied
// by the surrounding Rust environment.

pub const LIBIE_CTLQ_MBX_ID: i32 = -1;
pub const LIBIE_CTLQ_MAX_BUF_LEN: usize = SZ_4K;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum libie_ctlq_type {
    LIBIE_CTLQ_TYPE_TX = 0,
    LIBIE_CTLQ_TYPE_RX = 1,
}

pub const LIBIE_CTLQ_SEND_MSG_TO_CP: u16 = 0x801;
pub const LIBIE_CTLQ_SEND_MSG_TO_PEER: u16 = 0x804;
pub const LIBIE_CP_TX_COPYBREAK: u32 = 128;

#[repr(C)]
pub struct libie_ctlq_ctx {
    pub mmio_info: libie_mmio_info,
    pub ctlqs: list_head,
    pub ctlqs_lock: spinlock_t,
}

#[repr(C)]
pub struct libie_ctlq_reg {
    pub head: *mut core::ffi::c_void,
    pub tail: *mut core::ffi::c_void,
    pub len: *mut core::ffi::c_void,
    pub addr_high: *mut core::ffi::c_void,
    pub addr_low: *mut core::ffi::c_void,
    pub len_mask: u32,
    pub len_ena_mask: u32,
    pub head_mask: u32,
}

#[repr(C)]
pub struct libie_cp_dma_mem {
    pub va: *mut core::ffi::c_void,
    pub pa: dma_addr_t,
    pub size: usize,
    pub direction: i32,
}

#[repr(C)]
pub union libie_ctlq_msg_func_or_retval {
    pub func_id: u16,
    pub hw_retval: u16,
}

#[repr(C)]
pub union libie_ctlq_msg_mem {
    pub recv_mem: kvec,
    pub send_mem: libie_cp_dma_mem,
}

#[repr(C)]
pub struct libie_ctlq_msg {
    pub flags: u16,
    pub opcode: u16,
    pub data_len: u16,
    pub func_or_retval: libie_ctlq_msg_func_or_retval,
    pub chnl_opcode: u32,
    pub chnl_retval: u32,
    pub param0: u32,
    pub sw_cookie: u16,
    pub virt_flags: u16,
    pub addr_param: u64,
    pub mem: libie_ctlq_msg_mem,
}

#[repr(C)]
pub struct libie_ctlq_create_info {
    pub type_: libie_ctlq_type,
    pub id: i32,
    pub reg: libie_ctlq_reg,
    pub len: u16,
}

#[repr(C)]
pub union libie_ctlq_info_rx_or_tx {
    pub rx_fqes: *mut libeth_fqe,
    pub tx_msg: *mut *mut libie_ctlq_msg,
}

#[repr(C)]
pub union libie_ctlq_info_next {
    pub next_to_use: u32,
    pub next_to_post: u32,
}

#[repr(C)]
pub struct libie_ctlq_info {
    pub list: list_head,
    pub type_: libie_ctlq_type,
    pub qid: i32,
    pub lock: spinlock_t,
    pub ring_mem: libie_cp_dma_mem,
    pub descs: *mut libie_ctlq_desc,
    pub rx_or_tx: libie_ctlq_info_rx_or_tx,
    pub reg: libie_ctlq_reg,
    pub dev: *mut device,
    pub pp: *mut page_pool,
    pub truesize: u32,
    pub next_to_clean: u32,
    pub next: libie_ctlq_info_next,
    pub ring_len: u32,
}

pub const LIBIE_CTLQ_MBX_ATQ_LEN: u32 = 0x3ff;
pub const LIBIE_CTLQ_DESC_FLAG_DD: u16 = 1 << 0;
pub const LIBIE_CTLQ_DESC_FLAG_CMP: u16 = 1 << 1;
pub const LIBIE_CTLQ_DESC_FLAG_ERR: u16 = 1 << 2;
pub const LIBIE_CTLQ_DESC_FLAG_FTYPE_VM: u16 = 1 << 6;
pub const LIBIE_CTLQ_DESC_FLAG_FTYPE_PF: u16 = 1 << 7;
pub const LIBIE_CTLQ_DESC_FLAG_FTYPE: u16 = 0xC0;
pub const LIBIE_CTLQ_DESC_FLAG_RD: u16 = 1 << 10;
pub const LIBIE_CTLQ_DESC_FLAG_VFC: u16 = 1 << 11;
pub const LIBIE_CTLQ_DESC_FLAG_BUF: u16 = 1 << 12;
pub const LIBIE_CTLQ_DESC_FLAG_HOST_ID: u16 = 0xE000;
pub const LIBIE_CTLQ_DESC_FLAGS: u64 = 0xFFFF;
pub const LIBIE_CTLQ_DESC_INFRA_OPCODE: u64 = 0xFFFF_0000;
pub const LIBIE_CTLQ_DESC_DATA_LEN: u64 = 0xFFFF_0000_0000;
pub const LIBIE_CTLQ_DESC_HW_RETVAL: u64 = 0xFFFF_0000_0000_0000;
pub const LIBIE_CTLQ_DESC_PFID_VFID: u64 = 0xFFFF_0000_0000_0000;
pub const LIBIE_CTLQ_DESC_VIRTCHNL_OPCODE: u64 = 0x0FFF_FFFF;
pub const LIBIE_CTLQ_DESC_VIRTCHNL_DESC_TYPE: u64 = 0xF000_0000;
pub const LIBIE_CTLQ_DESC_VIRTCHNL_MSG_RET_VAL: u64 = 0xFFFF_FFFF_0000_0000;
pub const LIBIE_CTLQ_DESC_MSG_PARAM0: u64 = 0xFFFF_FFFF;
pub const LIBIE_CTLQ_DESC_SW_COOKIE: u64 = 0xFFFF_0000_0000;
pub const LIBIE_CTLQ_DESC_VIRTCHNL_FLAGS: u64 = 0xFFFF_0000_0000_0000;
pub const LIBIE_CTLQ_DESC_DATA_ADDR_HIGH: u64 = 0xFFFF_FFFF;
pub const LIBIE_CTLQ_DESC_DATA_ADDR_LOW: u64 = 0xFFFF_FFFF_0000_0000;

#[repr(C)]
pub struct libie_ctlq_desc {
    pub qword0: __le64,
    pub qword1: __le64,
    pub qword2: __le64,
    pub qword3: __le64,
}

#[repr(C)]
pub struct libie_ctlq_clean_params {
    pub rel_dma_mem: Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut libie_cp_dma_mem)>,
    pub rel_ctx: *const core::ffi::c_void,
    pub ctlq: *mut libie_ctlq_info,
    pub num_msgs: u16,
    pub force: bool,
}

pub unsafe fn libie_ctlq_release_rx_buf(rx_buf: *mut kvec) {
    if (*rx_buf).iov_base.is_null() {
        return;
    }
    let netmem = virt_to_netmem((*rx_buf).iov_base);
    page_pool_put_full_netmem(netmem_get_pp(netmem), netmem, false);
}

extern "C" {
    pub fn libie_ctlq_init(ctx: *mut libie_ctlq_ctx, qinfo: *const libie_ctlq_create_info, numq: u32) -> i32;
    pub fn libie_ctlq_deinit(ctx: *mut libie_ctlq_ctx);
    pub fn libie_find_ctlq(ctx: *mut libie_ctlq_ctx, type_: libie_ctlq_type, id: i32) -> *mut libie_ctlq_info;
    pub fn libie_ctlq_send_desc_avail(ctlq: *const libie_ctlq_info) -> u32;
    pub fn libie_ctlq_send(ctlq: *mut libie_ctlq_info, num_q_msg: u32);
    pub fn libie_ctlq_send_clean(params: *const libie_ctlq_clean_params) -> u32;
    pub fn libie_ctlq_recv(ctlq: *mut libie_ctlq_info, msg: *mut libie_ctlq_msg, num_q_msg: u32) -> u32;
    pub fn libie_ctlq_post_rx_buffs(ctlq: *mut libie_ctlq_info) -> i32;
}

pub const LIBIE_CTLQ_MAX_XN_ENTRIES: usize = 256;
pub const LIBIE_CTLQ_XN_COOKIE_M: u16 = 0xFF00;
pub const LIBIE_CTLQ_XN_INDEX_M: u16 = 0x00FF;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum libie_ctlq_xn_state {
    LIBIE_CTLQ_XN_IDLE = 0,
    LIBIE_CTLQ_XN_WAITING,
    LIBIE_CTLQ_XN_COMPLETED_SUCCESS,
    LIBIE_CTLQ_XN_COMPLETED_FAILED,
    LIBIE_CTLQ_XN_ASYNC,
    LIBIE_CTLQ_XN_SHUTDOWN,
}

#[repr(C)]
pub struct libie_ctlq_xn {
    pub resp_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut kvec, i32)>,
    pub xn_lock: spinlock_t,
    pub cmd_completion_event: completion,
    pub small_dma_mem: libie_cp_dma_mem,
    pub send_dma_mem: libie_cp_dma_mem,
    pub recv_mem: kvec,
    pub send_ctx: *mut core::ffi::c_void,
    pub timeout_ms: u64,
    pub timestamp: ktime_t,
    pub tx_msg: *mut libie_ctlq_msg,
    pub virtchnl_opcode: u32,
    pub state: libie_ctlq_xn_state,
    pub cookie: u8,
    pub index: u8,
}

#[repr(C)]
pub struct libie_ctlq_xn_manager {
    pub ctx: *mut libie_ctlq_ctx,
    pub free_xns_bm_lock: spinlock_t,
    pub free_xns_bm: [usize; 4],
    pub ring: [libie_ctlq_xn; LIBIE_CTLQ_MAX_XN_ENTRIES],
    pub small_buff_pool: *mut dma_pool,
    pub can_destroy: completion,
    pub shutdown: bool,
}

#[repr(C)]
pub struct libie_ctlq_xn_send_params {
    pub resp_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut kvec, i32)>,
    pub rel_tx_buf: Option<unsafe extern "C" fn(*const core::ffi::c_void)>,
    pub xnm: *mut libie_ctlq_xn_manager,
    pub ctlq: *mut libie_ctlq_info,
    pub ctlq_msg: *mut libie_ctlq_msg,
    pub send_buf: kvec,
    pub recv_mem: kvec,
    pub send_ctx: *mut core::ffi::c_void,
    pub timeout_ms: u64,
    pub chnl_opcode: u32,
}

#[inline]
pub fn libie_cp_can_send_onstack(size: u32) -> bool {
    size <= LIBIE_CP_TX_COPYBREAK
}

#[repr(C)]
pub struct libie_ctlq_xn_recv_params {
    pub ctlq_msg_handler: Option<unsafe extern "C" fn(*mut libie_ctlq_ctx, *mut libie_ctlq_msg)>,
    pub xnm: *mut libie_ctlq_xn_manager,
    pub ctlq: *mut libie_ctlq_info,
    pub budget: u32,
}

#[repr(C)]
pub struct libie_ctlq_xn_init_params {
    pub cctlq_info: *mut libie_ctlq_create_info,
    pub ctx: *mut libie_ctlq_ctx,
    pub xnm: *mut libie_ctlq_xn_manager,
    pub num_qs: u32,
}

extern "C" {
    pub fn libie_ctlq_xn_init(params: *mut libie_ctlq_xn_init_params) -> i32;
    pub fn libie_ctlq_xn_deinit(xnm: *mut libie_ctlq_xn_manager, ctx: *mut libie_ctlq_ctx);
    pub fn libie_ctlq_xn_shutdown(xnm: *mut libie_ctlq_xn_manager);
    pub fn libie_ctlq_xn_send(params: *mut libie_ctlq_xn_send_params) -> i32;
    pub fn libie_ctlq_xn_recv(params: *mut libie_ctlq_xn_recv_params) -> u32;
    pub fn libie_ctlq_xn_send_clean(ctlq: *mut libie_ctlq_info, rel_tx_buf: Option<unsafe extern "C" fn(*const core::ffi::c_void)>, force: bool) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
