// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/io_uring translation.

use core::ffi::c_void;

extern "C" {
    fn io_notif_to_data(notif: *mut io_kiocb) -> *mut io_notif_data;
    fn cmd_to_io_kiocb(nd: *mut io_notif_data) -> *mut io_kiocb;
    fn io_req_task_complete(req: io_tw_req, tw: io_tw_token_t);
    fn __io_unaccount_mem(user: *mut c_void, pages: u64);
    fn io_tx_ubuf_complete(skb: *mut sk_buff, uarg: *mut ubuf_info, success: bool);
    fn io_link_skb(skb: *mut sk_buff, uarg: *mut ubuf_info) -> i32;
    fn io_alloc_req(ctx: *mut io_ring_ctx, notif: *mut *mut io_kiocb) -> bool;
    fn current_io_uring() -> *mut c_void;
    fn io_get_task_refs(refs: i32);
    fn net_zcopy_get(uarg: *mut ubuf_info);
    fn skb_zcopy(skb: *mut sk_buff) -> *mut ubuf_info;
    fn skb_zcopy_init(skb: *mut sk_buff, uarg: *mut ubuf_info);
    fn __io_req_task_work_add(notif: *mut io_kiocb, flags: u32);
    fn refcount_set(refcnt: *mut u32, value: u32);
}

#[repr(C)]
pub struct io_tw_req {
    pub req: *mut io_kiocb,
}

pub type io_tw_token_t = usize;

#[repr(C)]
pub struct ubuf_info_ops {
    pub complete: Option<unsafe extern "C" fn(*mut sk_buff, *mut ubuf_info, bool)>,
    pub link_skb: Option<unsafe extern "C" fn(*mut sk_buff, *mut ubuf_info) -> i32>,
}

#[repr(C)]
pub struct ubuf_info {
    pub flags: u32,
    pub ops: *const ubuf_info_ops,
    pub refcnt: u32,
}

#[repr(C)]
pub struct io_notif_data {
    pub uarg: ubuf_info,
    pub zc_report: bool,
    pub zc_copied: bool,
    pub zc_used: bool,
    pub account_pages: u64,
    pub next: *mut io_notif_data,
    pub head: *mut io_notif_data,
}

#[repr(C)]
pub struct io_ring_ctx {
    pub uring_lock: c_void,
    pub user: *mut c_void,
}

#[repr(C)]
pub struct io_kiocb {
    pub ctx: *mut io_ring_ctx,
    pub opcode: u32,
    pub flags: u32,
    pub file: *mut c_void,
    pub tctx: *mut c_void,
    pub io_task_work: io_task_work,
    pub file_node: *mut c_void,
    pub buf_node: *mut c_void,
    pub cqe: io_cqe,
}

#[repr(C)]
pub struct io_task_work {
    pub func: Option<unsafe extern "C" fn(io_tw_req, io_tw_token_t)>,
}

#[repr(C)]
pub struct io_cqe {
    pub res: u32,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

const IORING_NOTIF_USAGE_ZC_COPIED: u32 = 1 << 0;
const IOU_F_TWQ_LAZY_WAKE: u32 = 1 << 0;
const IO_NOTIF_UBUF_FLAGS: u32 = 0;
const IORING_OP_NOP: u32 = 0;

unsafe extern "C" fn io_notif_tw_complete(mut tw_req: io_tw_req, tw: io_tw_token_t) {
    let mut notif = tw_req.req;
    let mut nd = io_notif_to_data(notif);
    let ctx = (*notif).ctx;

    loop {
        notif = cmd_to_io_kiocb(nd);
        if (*ctx as *mut io_ring_ctx) != (*notif).ctx {
            return;
        }

        if (*nd).zc_report && ((*nd).zc_copied || !(*nd).zc_used) {
            (*notif).cqe.res |= IORING_NOTIF_USAGE_ZC_COPIED;
        }
        if (*nd).account_pages != 0 && !(*notif).ctx.is_null() && !(*(*notif).ctx).user.is_null() {
            __io_unaccount_mem((*(*notif).ctx).user, (*nd).account_pages);
            (*nd).account_pages = 0;
        }
        nd = (*nd).next;
        io_req_task_complete(io_tw_req { req: notif }, tw);
        if nd.is_null() { break; }
    }
}

#[no_mangle]
pub unsafe extern "C" fn io_tx_ubuf_complete(skb: *mut sk_buff, uarg: *mut ubuf_info, success: bool) {
    let nd = (uarg as *mut u8).sub(core::mem::offset_of!(io_notif_data, uarg)) as *mut io_notif_data;
    let notif = cmd_to_io_kiocb(nd);
    if (*nd).zc_report {
        if success && !(*nd).zc_used && !skb.is_null() { (*nd).zc_used = true; }
        else if !success && !(*nd).zc_copied { (*nd).zc_copied = true; }
    }
    if (*uarg).refcnt != 1 { (*uarg).refcnt -= 1; return; }
    (*uarg).refcnt = 0;
    if (*nd).head != nd { io_tx_ubuf_complete(skb, &mut (*(*nd).head).uarg, success); return; }
    let tw_flags = if (*nd).next.is_null() { IOU_F_TWQ_LAZY_WAKE } else { 0 };
    (*notif).io_task_work.func = Some(io_notif_tw_complete);
    __io_req_task_work_add(notif, tw_flags);
}

unsafe extern "C" fn io_link_skb(skb: *mut sk_buff, uarg: *mut ubuf_info) -> i32 {
    let nd = (uarg as *mut u8).sub(core::mem::offset_of!(io_notif_data, uarg)) as *mut io_notif_data;
    let notif = cmd_to_io_kiocb(nd);
    let prev_uarg = skb_zcopy(skb);
    if prev_uarg.is_null() { net_zcopy_get(&mut (*nd).uarg); skb_zcopy_init(skb, &mut (*nd).uarg); return 0; }
    if prev_uarg == uarg { return 0; }
    if (*nd).head != nd || !(*nd).next.is_null() { return -17; }
    if (*prev_uarg).ops != &IO_UBUF_OPS { return -17; }
    let prev_nd = (prev_uarg as *mut u8).sub(core::mem::offset_of!(io_notif_data, uarg)) as *mut io_notif_data;
    let prev_notif = cmd_to_io_kiocb(prev_nd);
    if (*notif).ctx != (*prev_notif).ctx || (*notif).tctx != (*prev_notif).tctx { return -17; }
    (*nd).head = (*prev_nd).head; (*nd).next = (*prev_nd).next; (*prev_nd).next = nd;
    net_zcopy_get(&mut (*(*nd).head).uarg); 0
}

static IO_UBUF_OPS: ubuf_info_ops = ubuf_info_ops { complete: Some(io_tx_ubuf_complete), link_skb: Some(io_link_skb) };

#[no_mangle]
pub unsafe extern "C" fn io_alloc_notif(ctx: *mut io_ring_ctx) -> *mut io_kiocb {
    let mut notif: *mut io_kiocb = core::ptr::null_mut();
    if !io_alloc_req(ctx, &mut notif) { return core::ptr::null_mut(); }
    (*notif).ctx = ctx; (*notif).opcode = IORING_OP_NOP; (*notif).flags = 0; (*notif).file = core::ptr::null_mut();
    (*notif).tctx = current_io_uring(); io_get_task_refs(1); (*notif).file_node = core::ptr::null_mut(); (*notif).buf_node = core::ptr::null_mut();
    let nd = io_notif_to_data(notif); (*nd).zc_report = false; (*nd).account_pages = 0; (*nd).next = core::ptr::null_mut(); (*nd).head = nd;
    (*nd).uarg.flags = IO_NOTIF_UBUF_FLAGS; (*nd).uarg.ops = &IO_UBUF_OPS; (*nd).uarg.refcnt = 1; notif
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
