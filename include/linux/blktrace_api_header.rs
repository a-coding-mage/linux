/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/blk-mq.h, linux/relay.h, linux/compat.h, uapi/linux/blktrace_api.h,
// linux/list.h, linux/blk_types.h, and (when enabled) linux/sysfs.h.

#[cfg(CONFIG_BLK_DEV_IO_TRACE)]
#[repr(C)]
pub struct blk_trace {
    pub version: ::core::ffi::c_int,
    pub trace_state: ::core::ffi::c_int,
    pub rchan: *mut rchan,
    pub sequence: *mut ::core::ffi::c_ulong,
    pub msg_data: *mut u8,
    pub act_mask: u64,
    pub start_lba: u64,
    pub end_lba: u64,
    pub pid: u32,
    pub dev: u32,
    pub dir: *mut dentry,
    pub running_list: list_head,
    pub dropped: atomic_t,
}

#[cfg(CONFIG_BLK_DEV_IO_TRACE)]
extern "C" {
    pub fn blk_trace_ioctl(
        bdev: *mut block_device,
        cmd: ::core::ffi::c_uint,
        arg: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn blk_trace_shutdown(q: *mut request_queue);
    pub fn __blk_trace_note_message(
        bt: *mut blk_trace,
        css: *mut cgroup_subsys_state,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    pub fn blk_add_driver_data(rq: *mut request, data: *mut ::core::ffi::c_void, len: usize);
    pub fn blk_trace_setup(
        q: *mut request_queue,
        name: *mut ::core::ffi::c_char,
        dev: dev_t,
        bdev: *mut block_device,
        arg: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn blk_trace_startstop(q: *mut request_queue, start: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    pub fn blk_trace_remove(q: *mut request_queue) -> ::core::ffi::c_int;
}

// The C variadic macros use RCU locking and are retained as source-level
// macros; their referenced kernel helpers are supplied by other translations.
#[cfg(CONFIG_BLK_DEV_IO_TRACE)]
#[macro_export]
macro_rules! blk_add_cgroup_trace_msg {
    ($q:expr, $css:expr, $fmt:expr $(, $arg:expr)*) => {{
        let mut bt;
        unsafe {
            rcu_read_lock();
            bt = rcu_dereference(($q).blk_trace);
            if unlikely(!bt.is_null()) {
                __blk_trace_note_message(bt, $css, $fmt $(, $arg)*);
            }
            rcu_read_unlock();
        }
    }};
}

#[cfg(CONFIG_BLK_DEV_IO_TRACE)]
#[macro_export]
macro_rules! blk_add_trace_msg {
    ($q:expr, $fmt:expr $(, $arg:expr)*) => {
        blk_add_cgroup_trace_msg!($q, core::ptr::null_mut(), $fmt $(, $arg)*)
    };
}

pub const BLK_TN_MAX_MSG: usize = 128;

#[cfg(CONFIG_BLK_DEV_IO_TRACE)]
#[inline]
pub unsafe fn blk_trace_note_message_enabled(q: *mut request_queue) -> bool {
    let bt;
    let ret;
    rcu_read_lock();
    bt = rcu_dereference((*q).blk_trace);
    ret = !bt.is_null() && ((*bt).act_mask & BLK_TC_NOTIFY) != 0;
    rcu_read_unlock();
    ret
}

#[cfg(not(CONFIG_BLK_DEV_IO_TRACE))]
#[inline]
pub unsafe fn blk_trace_ioctl(
    _bdev: *mut block_device,
    _cmd: ::core::ffi::c_uint,
    _arg: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int { -ENOTTY }

#[cfg(not(CONFIG_BLK_DEV_IO_TRACE))]
#[inline]
pub unsafe fn blk_trace_shutdown(_q: *mut request_queue) {}

#[cfg(not(CONFIG_BLK_DEV_IO_TRACE))]
#[inline]
pub unsafe fn blk_add_driver_data(
    _rq: *mut request, _data: *mut ::core::ffi::c_void, _len: usize,
) {}

#[cfg(not(CONFIG_BLK_DEV_IO_TRACE))]
#[inline]
pub unsafe fn blk_trace_setup(
    _q: *mut request_queue, _name: *mut ::core::ffi::c_char, _dev: dev_t,
    _bdev: *mut block_device, _arg: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int { -ENOTTY }

#[cfg(not(CONFIG_BLK_DEV_IO_TRACE))]
#[inline]
pub unsafe fn blk_trace_startstop(_q: *mut request_queue, _start: ::core::ffi::c_int)
    -> ::core::ffi::c_int { -ENOTTY }

#[cfg(not(CONFIG_BLK_DEV_IO_TRACE))]
#[inline]
pub unsafe fn blk_trace_remove(_q: *mut request_queue) -> ::core::ffi::c_int { -ENOTTY }

#[cfg(not(CONFIG_BLK_DEV_IO_TRACE))]
#[inline]
pub unsafe fn blk_trace_note_message_enabled(_q: *mut request_queue) -> bool { false }

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct compat_blk_user_trace_setup {
    pub name: [::core::ffi::c_char; BLKTRACE_BDEV_SIZE],
    pub act_mask: u16,
    pub buf_size: u32,
    pub buf_nr: u32,
    pub start_lba: compat_u64,
    pub end_lba: compat_u64,
    pub pid: u32,
}

// #define BLKTRACESETUP32 _IOWR(0x12, 115, struct compat_blk_user_trace_setup)

extern "C" {
    pub fn blk_fill_rwbs(rwbs: *mut ::core::ffi::c_char, opf: blk_opf_t);
}

#[inline]
pub unsafe fn blk_rq_trace_sector(rq: *mut request) -> sector_t {
    // Tracing ignores the starting sector for passthrough requests and for
    // requests where the starting sector was not set.
    if blk_rq_is_passthrough(rq) || blk_rq_pos(rq) == (!0 as sector_t) {
        0
    } else {
        blk_rq_pos(rq)
    }
}

#[inline]
pub unsafe fn blk_rq_trace_nr_sectors(rq: *mut request) -> ::core::ffi::c_uint {
    if blk_rq_is_passthrough(rq) { 0 } else { blk_rq_sectors(rq) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
