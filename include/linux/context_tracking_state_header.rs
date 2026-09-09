/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external.  CONFIG_* conditionals below preserve the source build intent.

pub const CT_NESTING_IRQ_NONIDLE: i64 = (i64::MAX / 2) + 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ctx_state {
    CT_STATE_DISABLED = -1,
    CT_STATE_KERNEL = 0,
    CT_STATE_IDLE = 1,
    CT_STATE_USER = 2,
    CT_STATE_GUEST = 3,
    CT_STATE_MAX = 4,
}

#[repr(C)]
pub struct context_tracking {
    #[cfg(feature = "CONFIG_CONTEXT_TRACKING_USER")]
    pub active: bool,
    #[cfg(feature = "CONFIG_CONTEXT_TRACKING_USER")]
    pub recursion: i32,
    #[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
    pub state: atomic_t,
    #[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
    pub nesting: i64,
    #[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
    pub nmi_nesting: i64,
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_SIZE: usize = core::mem::size_of::<atomic_t>() * 8;
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_STATE_WIDTH: usize = bits_per((CT_STATE_MAX as i32) - 1);
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_STATE_START: usize = 0;
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_STATE_END: usize = CT_STATE_START + CT_STATE_WIDTH - 1;
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_RCU_WATCHING_MAX_WIDTH: usize = CT_SIZE - CT_STATE_WIDTH;
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_RCU_WATCHING_WIDTH: usize = if cfg!(feature = "CONFIG_RCU_DYNTICKS_TORTURE") { 2 } else { CT_RCU_WATCHING_MAX_WIDTH };
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_RCU_WATCHING_START: usize = CT_STATE_END + 1;
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_RCU_WATCHING_END: usize = CT_RCU_WATCHING_START + CT_RCU_WATCHING_WIDTH - 1;
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_RCU_WATCHING: usize = 1usize << CT_RCU_WATCHING_START;
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_STATE_MASK: usize = genmask(CT_STATE_END, CT_STATE_START);
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_RCU_WATCHING_MASK: usize = genmask(CT_RCU_WATCHING_END, CT_RCU_WATCHING_START);
#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
pub const CT_UNUSED_WIDTH: usize = CT_RCU_WATCHING_MAX_WIDTH - CT_RCU_WATCHING_WIDTH;

#[cfg(feature = "CONFIG_CONTEXT_TRACKING")]
extern "Rust" {
    pub static mut context_tracking: PerCpu<context_tracking>;
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_USER")]
#[inline(always)]
pub unsafe fn __ct_state() -> i32 {
    raw_atomic_read(this_cpu_ptr(core::ptr::addr_of_mut!(context_tracking.state))) & CT_STATE_MASK as i32
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
#[inline(always)]
pub unsafe fn ct_rcu_watching() -> i32 {
    atomic_read(this_cpu_ptr(core::ptr::addr_of_mut!(context_tracking.state))) & CT_RCU_WATCHING_MASK as i32
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
#[inline(always)]
pub unsafe fn ct_rcu_watching_cpu(cpu: i32) -> i32 {
    let ct = per_cpu_ptr(core::ptr::addr_of_mut!(context_tracking), cpu);
    atomic_read(core::ptr::addr_of_mut!((*ct).state)) & CT_RCU_WATCHING_MASK as i32
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
#[inline(always)]
pub unsafe fn ct_rcu_watching_cpu_acquire(cpu: i32) -> i32 {
    let ct = per_cpu_ptr(core::ptr::addr_of_mut!(context_tracking), cpu);
    atomic_read_acquire(core::ptr::addr_of_mut!((*ct).state)) & CT_RCU_WATCHING_MASK as i32
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
#[inline(always)]
pub unsafe fn ct_nesting() -> i64 { __this_cpu_read(context_tracking.nesting) }

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
#[inline(always)]
pub unsafe fn ct_nesting_cpu(cpu: i32) -> i64 {
    (*per_cpu_ptr(core::ptr::addr_of_mut!(context_tracking), cpu)).nesting
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
#[inline(always)]
pub unsafe fn ct_nmi_nesting() -> i64 { __this_cpu_read(context_tracking.nmi_nesting) }

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
#[inline(always)]
pub unsafe fn ct_nmi_nesting_cpu(cpu: i32) -> i64 {
    (*per_cpu_ptr(core::ptr::addr_of_mut!(context_tracking), cpu)).nmi_nesting
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_USER")]
extern "Rust" { pub static mut context_tracking_key: static_key_false; }

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_USER")]
#[inline(always)]
pub unsafe fn context_tracking_enabled() -> bool { static_branch_unlikely(&context_tracking_key) }

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_USER")]
#[inline(always)]
pub unsafe fn context_tracking_enabled_cpu(cpu: i32) -> bool {
    context_tracking_enabled() && per_cpu(context_tracking.active, cpu)
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_USER")]
#[inline(always)]
pub unsafe fn context_tracking_enabled_this_cpu() -> bool {
    context_tracking_enabled() && __this_cpu_read(context_tracking.active)
}

#[cfg(feature = "CONFIG_CONTEXT_TRACKING_USER")]
#[inline(always)]
pub unsafe fn ct_state() -> i32 {
    if !context_tracking_enabled() { return CT_STATE_DISABLED as i32; }
    preempt_disable();
    let ret = __ct_state();
    preempt_enable();
    ret
}

#[cfg(not(feature = "CONFIG_CONTEXT_TRACKING_USER"))]
#[inline(always)]
pub const fn context_tracking_enabled() -> bool { false }
#[cfg(not(feature = "CONFIG_CONTEXT_TRACKING_USER"))]
#[inline(always)]
pub const fn context_tracking_enabled_cpu(_cpu: i32) -> bool { false }
#[cfg(not(feature = "CONFIG_CONTEXT_TRACKING_USER"))]
#[inline(always)]
pub const fn context_tracking_enabled_this_cpu() -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
