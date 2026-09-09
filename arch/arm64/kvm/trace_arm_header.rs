/* SPDX-License-Identifier: GPL-2.0 */
// Translation of trace_arm.h.  C tracepoint registration/formatting is kept
// as payload types and unsafe recorder functions; referenced kernel symbols
// are supplied by the surrounding translation unit.

#[repr(C)]
pub struct KvmEntry { pub vcpu_pc: ::core::ffi::c_ulong }
pub unsafe fn kvm_entry(vcpu_pc: ::core::ffi::c_ulong) -> KvmEntry { KvmEntry { vcpu_pc } }

#[repr(C)]
pub struct KvmExit { pub ret: ::core::ffi::c_int, pub esr_ec: u32, pub vcpu_pc: ::core::ffi::c_ulong }
pub unsafe fn kvm_exit(ret: ::core::ffi::c_int, esr_ec: u32, vcpu_pc: ::core::ffi::c_ulong) -> KvmExit {
    KvmExit { ret: ARM_EXCEPTION_CODE(ret), esr_ec: if ARM_EXCEPTION_IS_TRAP(ret) { esr_ec } else { 0 }, vcpu_pc }
}

#[repr(C)]
pub struct KvmGuestFault { pub vcpu_pc: ::core::ffi::c_ulong, pub hsr: ::core::ffi::c_ulong, pub hxfar: ::core::ffi::c_ulong, pub ipa: u64 }
pub unsafe fn kvm_guest_fault(vcpu_pc: ::core::ffi::c_ulong, hsr: ::core::ffi::c_ulong, hxfar: ::core::ffi::c_ulong, ipa: u64) -> KvmGuestFault { KvmGuestFault { vcpu_pc, hsr, hxfar, ipa } }

#[repr(C)]
pub struct KvmAccessFault { pub ipa: ::core::ffi::c_ulong }
pub unsafe fn kvm_access_fault(ipa: ::core::ffi::c_ulong) -> KvmAccessFault { KvmAccessFault { ipa } }

#[repr(C)]
pub struct KvmIrqLine { pub r#type: u32, pub vcpu_idx: ::core::ffi::c_int, pub irq_num: ::core::ffi::c_int, pub level: ::core::ffi::c_int }
pub unsafe fn kvm_irq_line(r#type: u32, vcpu_idx: ::core::ffi::c_int, irq_num: ::core::ffi::c_int, level: ::core::ffi::c_int) -> KvmIrqLine { KvmIrqLine { r#type, vcpu_idx, irq_num, level } }

#[repr(C)]
pub struct KvmMmioEmulate { pub vcpu_pc: ::core::ffi::c_ulong, pub instr: ::core::ffi::c_ulong, pub cpsr: ::core::ffi::c_ulong }
pub unsafe fn kvm_mmio_emulate(vcpu_pc: ::core::ffi::c_ulong, instr: ::core::ffi::c_ulong, cpsr: ::core::ffi::c_ulong) -> KvmMmioEmulate { KvmMmioEmulate { vcpu_pc, instr, cpsr } }

#[repr(C)]
pub struct KvmMmioNisv { pub vcpu_pc: ::core::ffi::c_ulong, pub esr: ::core::ffi::c_ulong, pub far: ::core::ffi::c_ulong, pub ipa: ::core::ffi::c_ulong }
pub unsafe fn kvm_mmio_nisv(vcpu_pc: ::core::ffi::c_ulong, esr: ::core::ffi::c_ulong, far: ::core::ffi::c_ulong, ipa: ::core::ffi::c_ulong) -> KvmMmioNisv { KvmMmioNisv { vcpu_pc, esr, far, ipa } }

#[repr(C)]
pub struct KvmSetWayFlush { pub vcpu_pc: ::core::ffi::c_ulong, pub cache: bool }
pub unsafe fn kvm_set_way_flush(vcpu_pc: ::core::ffi::c_ulong, cache: bool) -> KvmSetWayFlush { KvmSetWayFlush { vcpu_pc, cache } }

#[repr(C)]
pub struct KvmToggleCache { pub vcpu_pc: ::core::ffi::c_ulong, pub was: bool, pub now: bool }
pub unsafe fn kvm_toggle_cache(vcpu_pc: ::core::ffi::c_ulong, was: bool, now: bool) -> KvmToggleCache { KvmToggleCache { vcpu_pc, was, now } }

#[repr(C)]
pub struct KvmTimerUpdateIrq { pub vcpu_id: ::core::ffi::c_ulong, pub irq: u32, pub level: ::core::ffi::c_int }
pub unsafe fn kvm_timer_update_irq(vcpu_id: ::core::ffi::c_ulong, irq: u32, level: ::core::ffi::c_int) -> KvmTimerUpdateIrq { KvmTimerUpdateIrq { vcpu_id, irq, level } }

#[repr(C)]
pub struct KvmGetTimerMap { pub vcpu_id: ::core::ffi::c_ulong, pub direct_vtimer: ::core::ffi::c_int, pub direct_ptimer: ::core::ffi::c_int, pub emul_vtimer: ::core::ffi::c_int, pub emul_ptimer: ::core::ffi::c_int }
pub unsafe fn kvm_get_timer_map(vcpu_id: ::core::ffi::c_ulong, map: *mut timer_map) -> KvmGetTimerMap {
    KvmGetTimerMap { vcpu_id, direct_vtimer: arch_timer_ctx_index((*map).direct_vtimer), direct_ptimer: if !(*map).direct_ptimer.is_null() { arch_timer_ctx_index((*map).direct_ptimer) } else { -1 }, emul_vtimer: if !(*map).emul_vtimer.is_null() { arch_timer_ctx_index((*map).emul_vtimer) } else { -1 }, emul_ptimer: if !(*map).emul_ptimer.is_null() { arch_timer_ctx_index((*map).emul_ptimer) } else { -1 } }
}

#[repr(C)]
pub struct KvmTimerState { pub ctl: ::core::ffi::c_ulong, pub cval: u64, pub timer_idx: ::core::ffi::c_int }
pub unsafe fn kvm_timer_save_state(ctx: *mut arch_timer_context) -> KvmTimerState { KvmTimerState { ctl: timer_get_ctl(ctx), cval: timer_get_cval(ctx), timer_idx: arch_timer_ctx_index(ctx) } }
pub unsafe fn kvm_timer_restore_state(ctx: *mut arch_timer_context) -> KvmTimerState { KvmTimerState { ctl: timer_get_ctl(ctx), cval: timer_get_cval(ctx), timer_idx: arch_timer_ctx_index(ctx) } }

#[repr(C)]
pub struct KvmTimerHrtimerExpire { pub timer_idx: ::core::ffi::c_int }
pub unsafe fn kvm_timer_hrtimer_expire(ctx: *mut arch_timer_context) -> KvmTimerHrtimerExpire { KvmTimerHrtimerExpire { timer_idx: arch_timer_ctx_index(ctx) } }

#[repr(C)]
pub struct KvmTimerEmulate { pub timer_idx: ::core::ffi::c_int, pub should_fire: bool }
pub unsafe fn kvm_timer_emulate(ctx: *mut arch_timer_context, should_fire: bool) -> KvmTimerEmulate { KvmTimerEmulate { timer_idx: arch_timer_ctx_index(ctx), should_fire } }

#[repr(C)]
pub struct KvmNestedEret { pub vcpu: *mut kvm_vcpu, pub elr_el2: ::core::ffi::c_ulong, pub spsr_el2: ::core::ffi::c_ulong, pub target_mode: ::core::ffi::c_ulong, pub hcr_el2: ::core::ffi::c_ulong }
pub unsafe fn kvm_nested_eret(vcpu: *mut kvm_vcpu, elr_el2: ::core::ffi::c_ulong, spsr_el2: ::core::ffi::c_ulong) -> KvmNestedEret { KvmNestedEret { vcpu, elr_el2, spsr_el2, target_mode: spsr_el2 & (PSR_MODE_MASK | PSR_MODE32_BIT), hcr_el2: __vcpu_sys_reg(vcpu, HCR_EL2) } }

#[repr(C)]
pub struct KvmInjectNestedException { pub vcpu: *mut kvm_vcpu, pub esr_el2: u64, pub r#type: ::core::ffi::c_int, pub spsr_el2: ::core::ffi::c_ulong, pub pc: ::core::ffi::c_ulong, pub source_mode: ::core::ffi::c_ulong, pub hcr_el2: ::core::ffi::c_ulong }
pub unsafe fn kvm_inject_nested_exception(vcpu: *mut kvm_vcpu, esr_el2: u64, r#type: ::core::ffi::c_int) -> KvmInjectNestedException { let cpsr = *vcpu_cpsr(vcpu); KvmInjectNestedException { vcpu, esr_el2, r#type, spsr_el2: cpsr, pc: *vcpu_pc(vcpu), source_mode: cpsr & (PSR_MODE_MASK | PSR_MODE32_BIT), hcr_el2: __vcpu_sys_reg(vcpu, HCR_EL2) } }

#[repr(C)]
pub struct KvmForwardSysregTrap { pub pc: u64, pub sysreg: u32, pub is_read: bool }
pub unsafe fn kvm_forward_sysreg_trap(vcpu: *mut kvm_vcpu, sysreg: u32, is_read: bool) -> KvmForwardSysregTrap { KvmForwardSysregTrap { pc: *vcpu_pc(vcpu), sysreg, is_read } }

// External kernel declarations referenced by this header.
extern "C" {
    fn ARM_EXCEPTION_CODE(ret: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ARM_EXCEPTION_IS_TRAP(ret: ::core::ffi::c_int) -> bool;
    fn arch_timer_ctx_index(ctx: *mut arch_timer_context) -> ::core::ffi::c_int;
    fn timer_get_ctl(ctx: *mut arch_timer_context) -> ::core::ffi::c_ulong;
    fn timer_get_cval(ctx: *mut arch_timer_context) -> u64;
    fn __vcpu_sys_reg(vcpu: *mut kvm_vcpu, reg: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn vcpu_cpsr(vcpu: *mut kvm_vcpu) -> *mut ::core::ffi::c_ulong;
    fn vcpu_pc(vcpu: *mut kvm_vcpu) -> *mut ::core::ffi::c_ulong;
}
extern "C" { type timer_map; type arch_timer_context; type kvm_vcpu; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
