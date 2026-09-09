// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependencies supplied by the corresponding kernel headers:
// clocksource/arm_arch_timer.h, linux/compiler.h, linux/kvm_host.h,
// asm/kvm_hyp.h, and asm/kvm_mmu.h.

#[repr(C)]
pub struct KvmVcpu {
    _opaque: [u8; 0],
}

extern "C" {
    fn has_hvhe() -> bool;
    fn vcpu_is_protected(vcpu: *mut KvmVcpu) -> bool;
    fn vcpu_ptimer(vcpu: *mut KvmVcpu) -> *mut core::ffi::c_void;
    fn vcpu_vtimer(vcpu: *mut KvmVcpu) -> *mut core::ffi::c_void;
    fn timer_get_offset(timer: *mut core::ffi::c_void) -> u64;
    fn has_broken_cntvoff() -> bool;
    fn sysreg_clear_set(reg: u64, clear: u64, set: u64);
}

// These constants and system-register operations are provided by the target
// architecture bindings.
extern "C" {
    static mut cntvoff_el2: u64;
    static mut cnthctl_el2: u64;
}

const CNTHCTL_EL1PCTEN: u64 = 1 << 0;
const CNTHCTL_EL1PCEN: u64 = 1 << 1;
const CNTHCTL_EL1TVT: u64 = 1 << 8;
const CNTHCTL_EL1TVCT: u64 = 1 << 9;

#[inline]
unsafe fn write_sysreg(value: u64, reg: *mut u64) {
    core::ptr::write_volatile(reg, value);
}

pub unsafe fn __kvm_timer_set_cntvoff(cntvoff: u64) {
    write_sysreg(cntvoff, core::ptr::addr_of_mut!(cntvoff_el2));
}

/*
 * Should only be called on non-VHE or hVHE setups.
 * VHE systems use EL2 timers and configure EL1 timers in kvm_timer_init_vhe().
 */
pub unsafe fn __timer_disable_traps(vcpu: *mut KvmVcpu) {
    let mut set: u64;
    let mut clr: u64;
    let mut shift: u32 = 0;

    if has_hvhe() {
        shift = 10;
    }

    /* Allow physical timer/counter access for the host */
    set = (CNTHCTL_EL1PCTEN | CNTHCTL_EL1PCEN) << shift;
    clr = CNTHCTL_EL1TVT | CNTHCTL_EL1TVCT;

    let _ = vcpu;
    sysreg_clear_set(cnthctl_el2, clr, set);
}

/*
 * Should only be called on non-VHE or hVHE setups.
 * VHE systems use EL2 timers and configure EL1 timers in kvm_timer_init_vhe().
 */
pub unsafe fn __timer_enable_traps(vcpu: *mut KvmVcpu) {
    let mut clr: u64 = 0;
    let mut set: u64 = 0;

    /*
     * Disallow physical timer access for the guest
     * Physical counter access is allowed if no offset is enforced
     * or running a protected VM (we don't offset anything in this case).
     */
    clr = CNTHCTL_EL1PCEN;
    if vcpu_is_protected(vcpu) || timer_get_offset(vcpu_ptimer(vcpu)) == 0 {
        set |= CNTHCTL_EL1PCTEN;
    } else {
        clr |= CNTHCTL_EL1PCTEN;
    }

    if has_hvhe() {
        clr <<= 10;
        set <<= 10;
    }

    /*
     * Trap the virtual counter/timer if we have a broken cntvoff
     * implementation and non zero offset as in timer_set_traps()
     */
    if has_broken_cntvoff() && timer_get_offset(vcpu_vtimer(vcpu)) != 0 {
        set |= CNTHCTL_EL1TVT | CNTHCTL_EL1TVCT;
    }

    sysreg_clear_set(cnthctl_el2, clr, set);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
