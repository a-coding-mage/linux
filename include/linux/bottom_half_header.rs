/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(any(feature = "CONFIG_PREEMPT_RT", feature = "CONFIG_TRACE_IRQFLAGS"))]
extern "C" {
    pub fn __local_bh_disable_ip(ip: ::core::ffi::c_ulong, cnt: u32);
}

#[cfg(not(any(feature = "CONFIG_PREEMPT_RT", feature = "CONFIG_TRACE_IRQFLAGS")))]
#[inline(always)]
unsafe fn __local_bh_disable_ip(ip: ::core::ffi::c_ulong, cnt: u32) {
    let _ = ip;
    preempt_count_add(cnt);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

extern "C" {
    pub fn _local_bh_enable();
    pub fn __local_bh_enable_ip(ip: ::core::ffi::c_ulong, cnt: u32);
}

extern "C" {
    fn preempt_count_add(cnt: u32);
}

pub const SOFTIRQ_DISABLE_OFFSET: u32 = 0;

#[inline]
pub unsafe fn local_bh_disable() {
    // _THIS_IP_ is a compiler-provided instruction-pointer expression in C;
    // its file-local equivalent is unavailable here.
    __local_bh_disable_ip(0, SOFTIRQ_DISABLE_OFFSET);
}

#[inline]
pub unsafe fn local_bh_enable_ip(ip: ::core::ffi::c_ulong) {
    __local_bh_enable_ip(ip, SOFTIRQ_DISABLE_OFFSET);
}

#[inline]
pub unsafe fn local_bh_enable() {
    // _THIS_IP_ is a compiler-provided instruction-pointer expression in C;
    // its file-local equivalent is unavailable here.
    __local_bh_enable_ip(0, SOFTIRQ_DISABLE_OFFSET);
}

#[cfg(feature = "CONFIG_PREEMPT_RT")]
extern "C" {
    pub fn local_bh_blocked() -> bool;
}

#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
#[inline]
pub fn local_bh_blocked() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
