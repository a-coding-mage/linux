/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/sched.h supplies `task_struct`, `current`, and `PF_BLOCK_TS`.

#[repr(C)]
pub struct blk_plug_cb;

pub type blk_plug_cb_fn = Option<unsafe extern "C" fn(
    cb: *mut blk_plug_cb,
    from_schedule: bool,
)>;

#[repr(C)]
pub struct rq_list {
    pub head: *mut request,
    pub tail: *mut request,
}

// CONFIG_BLOCK
#[cfg(feature = "CONFIG_BLOCK")]
#[repr(C)]
pub struct blk_plug {
    pub mq_list: rq_list, // blk-mq requests

    // if ios_left is > 1, we can batch tag/rq allocations
    pub cached_rqs: rq_list,
    pub cur_ktime: u64,
    pub nr_ios: u16,

    pub rq_count: u16,

    pub multiple_queues: bool,
    pub has_elevator: bool,

    pub cb_list: list_head, // md requires an unplug callback
}

#[cfg(feature = "CONFIG_BLOCK")]
extern "C" {
    pub fn blk_start_plug(plug: *mut blk_plug);
    pub fn blk_start_plug_nr_ios(plug: *mut blk_plug, nr_ios: u16);
    pub fn blk_finish_plug(plug: *mut blk_plug);

    pub fn __blk_flush_plug(plug: *mut blk_plug, from_schedule: bool);

    pub fn blk_check_plugged(
        unplug: blk_plug_cb_fn,
        data: *mut core::ffi::c_void,
        size: i32,
    ) -> *mut blk_plug_cb;
}

#[cfg(feature = "CONFIG_BLOCK")]
#[inline]
pub unsafe fn blk_flush_plug(plug: *mut blk_plug, async_: bool) {
    if !plug.is_null() {
        __blk_flush_plug(plug, async_);
    }
}

#[cfg(feature = "CONFIG_BLOCK")]
#[inline(always)]
pub unsafe fn blk_plug_invalidate_ts() {
    // Equivalent to: if (unlikely(current->flags & PF_BLOCK_TS))
    if (*current).flags & PF_BLOCK_TS != 0 {
        (*(*current).plug).cur_ktime = 0;
        (*current).flags &= !PF_BLOCK_TS;
    }
}

#[cfg(feature = "CONFIG_BLOCK")]
#[repr(C)]
pub struct blk_plug_cb {
    pub list: list_head,
    pub callback: blk_plug_cb_fn,
    pub data: *mut core::ffi::c_void,
}

// CONFIG_BLOCK disabled
#[cfg(not(feature = "CONFIG_BLOCK"))]
#[repr(C)]
pub struct blk_plug {}

#[cfg(not(feature = "CONFIG_BLOCK"))]
#[inline]
pub unsafe fn blk_start_plug(_plug: *mut blk_plug) {}

#[cfg(not(feature = "CONFIG_BLOCK"))]
#[inline]
pub unsafe fn blk_start_plug_nr_ios(_plug: *mut blk_plug, _nr_ios: u16) {}

#[cfg(not(feature = "CONFIG_BLOCK"))]
#[inline]
pub unsafe fn blk_finish_plug(_plug: *mut blk_plug) {}

#[cfg(not(feature = "CONFIG_BLOCK"))]
#[inline]
pub unsafe fn blk_flush_plug(_plug: *mut blk_plug, _async_: bool) {}

#[cfg(not(feature = "CONFIG_BLOCK"))]
#[inline]
pub unsafe fn blk_plug_invalidate_ts() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
