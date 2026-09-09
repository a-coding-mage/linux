// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel/Rust translation:
// linux/net.h, linux/uio.h, net/sock.h, linux/nospec.h, and rsrc.h.

pub const IO_NOTIF_UBUF_FLAGS: u32 = SKBFL_ZEROCOPY_FRAG | SKBFL_DONT_ORPHAN;
pub const IO_NOTIF_SPLICE_BATCH: u32 = 32;

#[repr(C)]
pub struct io_notif_data {
    pub file: *mut file,
    pub uarg: ubuf_info,

    pub next: *mut io_notif_data,
    pub head: *mut io_notif_data,

    pub account_pages: u32,
    pub zc_report: bool,
    pub zc_used: bool,
    pub zc_copied: bool,
}

extern "C" {
    pub fn io_alloc_notif(ctx: *mut io_ring_ctx) -> *mut io_kiocb;
    pub fn io_tx_ubuf_complete(
        skb: *mut sk_buff,
        uarg: *mut ubuf_info,
        success: bool,
    );
}

#[inline]
pub unsafe fn io_notif_to_data(notif: *mut io_kiocb) -> *mut io_notif_data {
    // C macro equivalent: io_kiocb_to_cmd(notif, struct io_notif_data).
    io_kiocb_to_cmd(notif)
}

#[inline]
pub unsafe fn io_notif_flush(notif: *mut io_kiocb) {
    // C annotation: __must_hold(&notif->ctx->uring_lock).
    let nd: *mut io_notif_data = io_notif_to_data(notif);

    io_tx_ubuf_complete(core::ptr::null_mut(), &mut (*nd).uarg, true);
}

#[inline]
pub unsafe fn io_notif_account_mem(notif: *mut io_kiocb, len: u32) -> i32 {
    let ctx: *mut io_ring_ctx = (*notif).ctx;
    let nd: *mut io_notif_data = io_notif_to_data(notif);
    let nr_pages: u32 = (len >> PAGE_SHIFT) + 2;
    let mut ret: i32;

    if !(*ctx).user.is_null() {
        ret = __io_account_mem((*ctx).user, nr_pages);
        if ret != 0 {
            return ret;
        }
        (*nd).account_pages += nr_pages;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
