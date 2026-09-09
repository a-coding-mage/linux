/* SPDX-License-Identifier: GPL-2.0 */

// The original declarations are conditional on CONFIG_NET_RX_BUSY_POLL.
// This feature name preserves that build-time condition for Rust consumers.

#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
extern "C" {
    pub fn io_napi_init(ctx: *mut io_ring_ctx);
    pub fn io_napi_free(ctx: *mut io_ring_ctx);

    pub fn io_register_napi(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void) -> i32;
    pub fn io_unregister_napi(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void) -> i32;

    pub fn __io_napi_add_id(
        ctx: *mut io_ring_ctx,
        napi_id: u32,
        mode: u32,
    ) -> i32;

    pub fn __io_napi_busy_loop(ctx: *mut io_ring_ctx, iowq: *mut io_wait_queue);
    pub fn io_napi_sqpoll_busy_poll(ctx: *mut io_ring_ctx) -> i32;
}

#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
#[inline]
pub unsafe fn io_napi(ctx: *mut io_ring_ctx) -> bool {
    !list_empty(core::ptr::addr_of_mut!((*ctx).napi_list))
}

#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
#[inline]
pub unsafe fn io_napi_busy_loop(ctx: *mut io_ring_ctx, iowq: *mut io_wait_queue) {
    if !io_napi(ctx) {
        return;
    }
    __io_napi_busy_loop(ctx, iowq);
}

/*
 * io_napi_add() - Add napi id to the busy poll list
 * @req: pointer to io_kiocb request
 *
 * Add the napi id of the socket to the napi busy poll list and hash table.
 */
#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
#[inline]
pub unsafe fn io_napi_add(req: *mut io_kiocb) {
    let ctx: *mut io_ring_ctx = (*req).ctx;
    let mut sock: *mut socket;
    let mode: u32 = IO_URING_NAPI_TRACKING_DYNAMIC;

    if READ_ONCE((*ctx).napi_track_mode) != mode {
        return;
    }

    sock = sock_from_file((*req).file);
    if !sock.is_null() && !(*sock).sk.is_null() {
        __io_napi_add_id(ctx, READ_ONCE((*(*sock).sk).sk_napi_id), mode);
    }
}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub unsafe fn io_napi_init(_ctx: *mut io_ring_ctx) {}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub unsafe fn io_napi_free(_ctx: *mut io_ring_ctx) {}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub unsafe fn io_register_napi(
    _ctx: *mut io_ring_ctx,
    _arg: *mut core::ffi::c_void,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub unsafe fn io_unregister_napi(
    _ctx: *mut io_ring_ctx,
    _arg: *mut core::ffi::c_void,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub unsafe fn io_napi(_ctx: *mut io_ring_ctx) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub unsafe fn io_napi_add(_req: *mut io_kiocb) {}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub unsafe fn io_napi_busy_loop(
    _ctx: *mut io_ring_ctx,
    _iowq: *mut io_wait_queue,
) {
}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub unsafe fn io_napi_sqpoll_busy_poll(_ctx: *mut io_ring_ctx) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
