/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not implemented here.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_arch_timers {
    TIMER_PTIMER,
    TIMER_VTIMER,
    NR_KVM_EL0_TIMERS,
    TIMER_HVTIMER = kvm_arch_timers::NR_KVM_EL0_TIMERS as isize,
    TIMER_HPTIMER,
    NR_KVM_TIMERS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_arch_timer_regs {
    TIMER_REG_CNT,
    TIMER_REG_CVAL,
    TIMER_REG_TVAL,
    TIMER_REG_CTL,
    TIMER_REG_VOFF,
}

#[repr(C)]
pub struct arch_timer_offset {
    /* If NULL, assume a zero offset. */
    pub vm_offset: *mut u64,
    /* If NULL, assume a zero offset. */
    pub vcpu_offset: *mut u64,
}

#[repr(C)]
pub struct arch_timer_vm_data {
    pub voffset: u64,
    pub poffset: u64,
    pub ppi: [u32; kvm_arch_timers::NR_KVM_TIMERS as usize],
}

#[repr(C)]
pub struct arch_timer_context {
    pub hrtimer: hrtimer,
    pub ns_frac: u64,
    pub offset: arch_timer_offset,
    pub loaded: bool,
    pub timer_id: kvm_arch_timers,
    pub host_timer_irq: u32,
}

#[repr(C)]
pub struct timer_map {
    pub direct_vtimer: *mut arch_timer_context,
    pub direct_ptimer: *mut arch_timer_context,
    pub emul_vtimer: *mut arch_timer_context,
    pub emul_ptimer: *mut arch_timer_context,
}

extern "C" {
    pub fn get_timer_map(vcpu: *mut kvm_vcpu, map: *mut timer_map);
}

#[repr(C)]
pub struct arch_timer_cpu {
    pub timers: [arch_timer_context; kvm_arch_timers::NR_KVM_TIMERS as usize],
    pub bg_timer: hrtimer,
    pub enabled: bool,
}

extern "C" {
    pub fn kvm_timer_hyp_init(has_gic: bool) -> i32;
    pub fn kvm_timer_enable(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_timer_vcpu_reset(vcpu: *mut kvm_vcpu);
    pub fn kvm_timer_vcpu_init(vcpu: *mut kvm_vcpu);
    pub fn kvm_timer_sync_nested(vcpu: *mut kvm_vcpu);
    pub fn kvm_timer_sync_user(vcpu: *mut kvm_vcpu);
    pub fn kvm_timer_should_notify_user(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_timer_update_run(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_timer_vcpu_terminate(vcpu: *mut kvm_vcpu);
    pub fn kvm_timer_init_vm(kvm: *mut kvm);
    pub fn kvm_arm_timer_set_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> i32;
    pub fn kvm_arm_timer_get_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> i32;
    pub fn kvm_arm_timer_has_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> i32;
    pub fn kvm_phys_timer_read() -> u64;
    pub fn kvm_timer_vcpu_load(vcpu: *mut kvm_vcpu);
    pub fn kvm_timer_vcpu_put(vcpu: *mut kvm_vcpu);
    pub fn kvm_timer_init_vhe();
    pub fn kvm_arm_timer_read_sysreg(vcpu: *mut kvm_vcpu, tmr: kvm_arch_timers, treg: kvm_arch_timer_regs) -> u64;
    pub fn kvm_arm_timer_write_sysreg(vcpu: *mut kvm_vcpu, tmr: kvm_arch_timers, treg: kvm_arch_timer_regs, val: u64);
    pub fn timer_get_ctl(ctxt: *mut arch_timer_context) -> u32;
    pub fn timer_get_cval(ctxt: *mut arch_timer_context) -> u64;
    pub fn kvm_timer_cpu_up();
    pub fn kvm_timer_cpu_down();
}

// CNTKCTL_EL1 valid bits as of DDI0487J.a
pub const CNTKCTL_VALID_BITS: u64 = (1u64 << 17) | ((1u64 << 10) - 1);

extern "C" {
    pub fn has_broken_cntvoff() -> bool;
    pub fn has_cntpoff() -> bool;
}

#[inline]
pub unsafe fn timer_set_offset(ctxt: *mut arch_timer_context, offset: u64) {
    if (*ctxt).offset.vm_offset.is_null() {
        // WARN(offset, "timer %d\n", arch_timer_ctx_index(ctxt));
        return;
    }
    core::ptr::write_volatile((*ctxt).offset.vm_offset, offset);
}

// The following C macros are represented as Rust macros to preserve their
// pointer-based interface and field access semantics.
#[macro_export]
macro_rules! arch_timer_ctx_index { ($ctx:expr) => { unsafe { (*$ctx).timer_id } }; }
#[macro_export]
macro_rules! timer_get_offset {
    ($ctxt:expr) => {{ let mut off: u64 = 0; let __ctxt = $ctxt; unsafe { if !__ctxt.is_null() { let ato = &(*__ctxt).offset; if !ato.vm_offset.is_null() { off = off.wrapping_add(core::ptr::read_volatile(ato.vm_offset)); } if !ato.vcpu_offset.is_null() { off = off.wrapping_add(core::ptr::read_volatile(ato.vcpu_offset)); } } } off }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
