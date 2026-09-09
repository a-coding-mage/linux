/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of `fuse/dev.h`.

use core::ffi::c_void;

/// Maximum number of outstanding background requests.
pub const FUSE_DEFAULT_MAX_BACKGROUND: u32 = 12;

#[repr(C)]
pub struct fuse_conn {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fuse_chan {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fuse_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fuse_args {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fuse_copy_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fuse_backing_map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fuse_forget_link {
    _private: [u8; 0],
}

pub type gfp_t = usize;
pub type dev_t = u64;
pub type fuse_notify_code = core::ffi::c_int;

#[repr(C)]
pub struct fuse_chan_param {
    pub minor: u32,
    pub max_write: u32,
    pub max_pages: u32,
    pub io_uring_enabled: bool,
}

extern "C" {
    pub fn fuse_chan_new() -> *mut fuse_chan;
    pub fn fuse_dev_chan_new() -> *mut fuse_chan;
    pub fn fuse_chan_release(fch: *mut fuse_chan);
    pub fn fuse_chan_free(fch: *mut fuse_chan);
    pub fn fuse_chan_num_background(fch: *mut fuse_chan) -> u32;
    pub fn fuse_chan_max_background(fch: *mut fuse_chan) -> u32;
    pub fn fuse_chan_max_background_set(fch: *mut fuse_chan, val: u32);
    pub fn fuse_chan_num_waiting(fch: *mut fuse_chan) -> u32;
    pub fn fuse_chan_set_fc(fch: *mut fuse_chan, fc: *mut fuse_conn);
    pub fn fuse_chan_set_initialized(fch: *mut fuse_chan, param: *mut fuse_chan_param);
    pub fn fuse_chan_send(fch: *mut fuse_chan, args: *mut fuse_args) -> isize;
    pub fn fuse_chan_send_bg(fch: *mut fuse_chan, args: *mut fuse_args, gfp_flags: gfp_t) -> i32;
    pub fn fuse_chan_send_notify_reply(
        fch: *mut fuse_chan,
        args: *mut fuse_args,
        unique: u64,
    ) -> i32;
    pub fn fuse_chan_resend(fch: *mut fuse_chan);

    pub fn fuse_alloc_forget() -> *mut fuse_forget_link;
    pub fn fuse_chan_queue_forget(
        fch: *mut fuse_chan,
        forget: *mut fuse_forget_link,
        nodeid: u64,
        nlookup: u64,
    );

    /// Initialize the client device.
    pub fn fuse_dev_init() -> i32;
    /// Cleanup the client device.
    pub fn fuse_dev_cleanup();
    pub fn fuse_dev_install(fud: *mut fuse_dev, fch: *mut fuse_chan);
    pub fn fuse_dev_verify(fud: *mut fuse_dev, fch: *mut fuse_chan) -> bool;
    pub fn fuse_dev_put(fud: *mut fuse_dev);
    pub fn fuse_dev_is_installed(fud: *mut fuse_dev) -> bool;
    pub fn fuse_dev_is_sync_init(fud: *mut fuse_dev) -> bool;
    pub fn fuse_dev_grab(file: *mut file) -> *mut fuse_dev;

    pub fn fuse_init_server_timeout(fch: *mut fuse_chan, timeout: u32);
    pub fn fuse_chan_abort(fch: *mut fuse_chan, abort_with_err: bool);
    pub fn fuse_chan_wait_aborted(fch: *mut fuse_chan);
    pub fn fuse_conn_get(fc: *mut fuse_conn) -> *mut fuse_conn;
    pub fn fuse_conn_put(fc: *mut fuse_conn);
    pub fn fuse_conn_get_id(fc: *mut fuse_conn) -> dev_t;
    pub fn fuse_end_polls(fc: *mut fuse_conn);
    pub fn fuse_notify(
        fc: *mut fuse_conn,
        code: fuse_notify_code,
        size: u32,
        cs: *mut fuse_copy_state,
    ) -> i32;
    pub fn fuse_backing_open(fc: *mut fuse_conn, map: *mut fuse_backing_map) -> i32;
    pub fn fuse_backing_close(fc: *mut fuse_conn, backing_id: i32) -> i32;
    pub fn fuse_copy_one(cs: *mut fuse_copy_state, val: *mut c_void, size: u32) -> i32;
    pub fn fuse_copy_folio(
        cs: *mut fuse_copy_state,
        foliop: *mut *mut folio,
        offset: u32,
        count: u32,
        zeroing: i32,
    ) -> i32;
    pub fn fuse_copy_finish(cs: *mut fuse_copy_state);

    #[cfg(CONFIG_FUSE_IO_URING)]
    pub fn fuse_uring_enabled() -> bool;
    #[cfg(CONFIG_FUSE_IO_URING)]
    pub fn fuse_uring_destruct(fch: *mut fuse_chan);
}

#[cfg(not(CONFIG_FUSE_IO_URING))]
#[inline]
pub fn fuse_uring_enabled() -> bool {
    false
}

#[cfg(not(CONFIG_FUSE_IO_URING))]
#[inline]
pub fn fuse_uring_destruct(_fch: *mut fuse_chan) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
