/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. Linux annotations such as __percpu and __rcu
// are represented by comments; the referenced types are supplied externally.

#[repr(C, align(16))]
pub struct gnet_stats_basic_sync {
    pub bytes: u64_stats_t,
    pub packets: u64_stats_t,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
pub struct gnet_dump {
    pub lock: *mut spinlock_t,
    pub skb: *mut sk_buff,
    pub tail: *mut nlattr,

    // Backward compatibility
    pub compat_tc_stats: ::core::ffi::c_int,
    pub compat_xstats: ::core::ffi::c_int,
    pub padattr: ::core::ffi::c_int,
    pub xstats: *mut ::core::ffi::c_void,
    pub xstats_len: ::core::ffi::c_int,
    pub tc_stats: tc_stats,
}

pub struct net_rate_estimator;

unsafe extern "C" {
    pub fn gnet_stats_basic_sync_init(b: *mut gnet_stats_basic_sync);
    pub fn gnet_stats_start_copy(
        skb: *mut sk_buff,
        r#type: ::core::ffi::c_int,
        lock: *mut spinlock_t,
        d: *mut gnet_dump,
        padattr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn gnet_stats_start_copy_compat(
        skb: *mut sk_buff,
        r#type: ::core::ffi::c_int,
        tc_stats_type: ::core::ffi::c_int,
        xstats_type: ::core::ffi::c_int,
        lock: *mut spinlock_t,
        d: *mut gnet_dump,
        padattr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn gnet_stats_copy_basic(
        d: *mut gnet_dump,
        cpu: *const gnet_stats_basic_sync,
        b: *const gnet_stats_basic_sync,
        running: bool,
    ) -> ::core::ffi::c_int;
    pub fn gnet_stats_add_basic(
        bstats: *mut gnet_stats_basic_sync,
        cpu: *const gnet_stats_basic_sync,
        b: *const gnet_stats_basic_sync,
        running: bool,
    );
    pub fn gnet_stats_copy_basic_hw(
        d: *mut gnet_dump,
        cpu: *const gnet_stats_basic_sync,
        b: *const gnet_stats_basic_sync,
        running: bool,
    ) -> ::core::ffi::c_int;
    pub fn gnet_stats_copy_rate_est(
        d: *mut gnet_dump,
        ptr: *mut *mut net_rate_estimator,
    ) -> ::core::ffi::c_int;
    pub fn gnet_stats_copy_queue(
        d: *mut gnet_dump,
        cpu_q: *const gnet_stats_queue,
        q: *const gnet_stats_queue,
        qlen: __u32,
    ) -> ::core::ffi::c_int;
    pub fn gnet_stats_add_queue(
        qstats: *mut gnet_stats_queue,
        cpu_q: *const gnet_stats_queue,
        q: *const gnet_stats_queue,
    );
    pub fn gnet_stats_copy_app(
        d: *mut gnet_dump,
        st: *mut ::core::ffi::c_void,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn gnet_stats_finish_copy(d: *mut gnet_dump) -> ::core::ffi::c_int;
    pub fn gen_new_estimator(
        bstats: *mut gnet_stats_basic_sync,
        cpu_bstats: *mut gnet_stats_basic_sync,
        rate_est: *mut *mut net_rate_estimator,
        lock: *mut spinlock_t,
        running: bool,
        opt: *mut nlattr,
    ) -> ::core::ffi::c_int;
    pub fn gen_kill_estimator(ptr: *mut *mut net_rate_estimator);
    pub fn gen_replace_estimator(
        bstats: *mut gnet_stats_basic_sync,
        cpu_bstats: *mut gnet_stats_basic_sync,
        ptr: *mut *mut net_rate_estimator,
        lock: *mut spinlock_t,
        running: bool,
        opt: *mut nlattr,
    ) -> ::core::ffi::c_int;
    pub fn gen_estimator_active(ptr: *mut *mut net_rate_estimator) -> bool;
    pub fn gen_estimator_read(
        ptr: *mut *mut net_rate_estimator,
        sample: *mut gnet_stats_rate_est64,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
