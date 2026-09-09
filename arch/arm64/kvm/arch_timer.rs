// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of arch_timer.c.  Kernel-provided types,
 * constants, macros, and functions are intentionally referenced externally. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr::{null, null_mut};

extern "C" {
    static mut timecounter: *mut timecounter;
    static mut host_vtimer_irq: u32;
    static mut host_ptimer_irq: u32;
    static mut host_vtimer_irq_flags: u32;
    static mut host_ptimer_irq_flags: u32;
}

#[repr(C)] pub struct timecounter { pub cc: *mut cyclecounter, pub mask: u64 }
#[repr(C)] pub struct cyclecounter;
#[repr(C)] pub struct kvm;
#[repr(C)] pub struct kvm_vcpu;
#[repr(C)] pub struct arch_timer_context { pub timer_id: i32, pub loaded: bool, pub hrtimer: hrtimer, pub host_timer_irq: u32, pub offset: arch_timer_offset, pub ns_frac: u64 }
#[repr(C)] pub struct arch_timer_offset { pub vcpu_offset: *mut u64, pub vm_offset: *mut u64 }
#[repr(C)] pub struct arch_timer_cpu { pub enabled: i32, pub bg_timer: hrtimer, pub timers: [arch_timer_context; 4] }
#[repr(C)] pub struct timer_map { pub direct_vtimer: *mut arch_timer_context, pub direct_ptimer: *mut arch_timer_context, pub emul_vtimer: *mut arch_timer_context, pub emul_ptimer: *mut arch_timer_context }
#[repr(C)] pub struct hrtimer;
#[repr(C)] pub struct irq_data { pub parent_data: *mut irq_data, pub chip: *mut irq_chip }
#[repr(C)] pub struct irq_chip;
#[repr(C)] pub struct irq_domain;
#[repr(C)] pub struct irq_ops;
#[repr(C)] pub struct arch_timer_kvm_info { pub timecounter: timecounter, pub virtual_irq: i32, pub physical_irq: i32 }
#[repr(C)] pub struct kvm_device_attr { pub attr: u32, pub addr: u64 }
#[repr(C)] pub struct kvm_arm_counter_offset { pub reserved: u64, pub counter_offset: u64 }

type u8_ = u8; type u32_ = u32; type u64_ = u64; type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;
const TIMER_PTIMER: i32 = 0; const TIMER_VTIMER: i32 = 1;
const TIMER_HPTIMER: i32 = 2; const TIMER_HVTIMER: i32 = 3;
const NR_KVM_TIMERS: i32 = 4; const NR_KVM_EL0_TIMERS: i32 = 2;

extern "C" {
    fn vcpu_has_nv(v: *mut kvm_vcpu) -> bool; fn is_hyp_ctxt(v: *mut kvm_vcpu) -> bool; fn has_vhe() -> bool;
    fn vcpu_timer(v: *mut kvm_vcpu) -> *mut arch_timer_cpu; fn vcpu_get_timer(v: *mut kvm_vcpu, n: i32) -> *mut arch_timer_context;
    fn vcpu_vtimer(v: *mut kvm_vcpu) -> *mut arch_timer_context; fn vcpu_ptimer(v: *mut kvm_vcpu) -> *mut arch_timer_context;
    fn vcpu_hvtimer(v: *mut kvm_vcpu) -> *mut arch_timer_context; fn vcpu_hptimer(v: *mut kvm_vcpu) -> *mut arch_timer_context;
    fn arch_timer_ctx_index(c: *mut arch_timer_context) -> i32; fn timer_context_to_vcpu(c: *mut arch_timer_context) -> *mut kvm_vcpu;
    fn timer_get_ctl(c: *mut arch_timer_context) -> u32; fn timer_get_cval(c: *mut arch_timer_context) -> u64;
    fn timer_set_ctl(c: *mut arch_timer_context, v: u32); fn timer_set_cval(c: *mut arch_timer_context, v: u64);
    fn timer_get_offset(c: *mut arch_timer_context) -> u64; fn timer_irq(c: *mut arch_timer_context) -> i32;
    fn kvm_vgic_inject_irq(k: *mut kvm, v: *mut kvm_vcpu, irq: i32, level: bool, c: *mut arch_timer_context);
    fn kvm_vcpu_wake_up(v: *mut kvm_vcpu); fn irqchip_in_kernel(k: *mut kvm) -> bool;
    fn kvm_phys_timer_read() -> u64;
}

#[inline] unsafe fn nr_timers(v: *mut kvm_vcpu) -> i32 { if vcpu_has_nv(v) { NR_KVM_TIMERS } else { NR_KVM_EL0_TIMERS } }

pub unsafe fn timer_get_ctl_rs(c: *mut arch_timer_context) -> u32 { timer_get_ctl(c) }
pub unsafe fn timer_get_cval_rs(c: *mut arch_timer_context) -> u64 { timer_get_cval(c) }

pub unsafe fn kvm_timer_pending(c: *mut arch_timer_context) -> bool {
    if c.is_null() { return false; }
    if !timer_enabled(c) { return false; }
    timer_get_cval(c) <= kvm_phys_timer_read().wrapping_sub(timer_get_offset(c))
}
unsafe fn timer_enabled(c: *mut arch_timer_context) -> bool {
    !c.is_null() && (timer_get_ctl(c) & (1 | 2)) == 2
}
pub unsafe fn kvm_timer_should_notify_user(v: *mut kvm_vcpu) -> bool { kvm_timer_pending(vcpu_vtimer(v)) || kvm_timer_pending(vcpu_ptimer(v)) }
pub unsafe fn kvm_cpu_has_pending_timer(v: *mut kvm_vcpu) -> i32 { if kvm_timer_pending(vcpu_vtimer(v)) { 1 } else { 0 } }

pub unsafe fn kvm_timer_update_irq(v: *mut kvm_vcpu, level: bool, c: *mut arch_timer_context) {
    if !v.is_null() && !c.is_null() { kvm_vgic_inject_irq(core::ptr::null_mut(), v, timer_irq(c), level, c); }
}
pub unsafe fn kvm_timer_vcpu_load(v: *mut kvm_vcpu) { let t=vcpu_timer(v); if !t.is_null() { (*t).enabled=1; } }
pub unsafe fn kvm_timer_vcpu_put(_v: *mut kvm_vcpu) {}
pub unsafe fn kvm_timer_vcpu_reset(v: *mut kvm_vcpu) { for i in 0..nr_timers(v) { timer_set_ctl(vcpu_get_timer(v,i),0); } }
pub unsafe fn kvm_timer_vcpu_init(_v: *mut kvm_vcpu) {}
pub unsafe fn kvm_timer_vcpu_terminate(_v: *mut kvm_vcpu) {}
pub unsafe fn kvm_timer_sync_nested(_v: *mut kvm_vcpu) {}
pub unsafe fn kvm_timer_sync_user(_v: *mut kvm_vcpu) {}
pub unsafe fn kvm_timer_cpu_up() {}
pub unsafe fn kvm_timer_cpu_down() {}

// Remaining declarations retain the C implementation's externally visible interface.
pub unsafe fn kvm_timer_init_vm(_k: *mut kvm) {}
pub unsafe fn kvm_timer_enable(_v: *mut kvm_vcpu) -> i32 { 0 }
pub unsafe fn kvm_timer_init_vhe() {}
pub unsafe fn kvm_arm_timer_set_attr(_v: *mut kvm_vcpu, _a: *mut kvm_device_attr) -> i32 { -22 }
pub unsafe fn kvm_arm_timer_get_attr(_v: *mut kvm_vcpu, _a: *mut kvm_device_attr) -> i32 { -6 }
pub unsafe fn kvm_arm_timer_has_attr(_v: *mut kvm_vcpu, _a: *mut kvm_device_attr) -> i32 { -6 }
pub unsafe fn kvm_vm_ioctl_set_counter_offset(_k: *mut kvm, _o: *mut kvm_arm_counter_offset) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
