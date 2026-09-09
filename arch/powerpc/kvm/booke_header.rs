/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright IBM Corp. 2008
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/kvm_host.h, asm/kvm_ppc.h, asm/switch_to.h, and timing.h.

/* interrupt priortity ordering */
pub const BOOKE_IRQPRIO_DATA_STORAGE: i32 = 0;
pub const BOOKE_IRQPRIO_INST_STORAGE: i32 = 1;
pub const BOOKE_IRQPRIO_ALIGNMENT: i32 = 2;
pub const BOOKE_IRQPRIO_PROGRAM: i32 = 3;
pub const BOOKE_IRQPRIO_FP_UNAVAIL: i32 = 4;
#[cfg(CONFIG_SPE_POSSIBLE)]
pub const BOOKE_IRQPRIO_SPE_UNAVAIL: i32 = 5;
#[cfg(CONFIG_SPE_POSSIBLE)]
pub const BOOKE_IRQPRIO_SPE_FP_DATA: i32 = 6;
#[cfg(CONFIG_SPE_POSSIBLE)]
pub const BOOKE_IRQPRIO_SPE_FP_ROUND: i32 = 7;
#[cfg(CONFIG_PPC_E500MC)]
pub const BOOKE_IRQPRIO_ALTIVEC_UNAVAIL: i32 = 5;
#[cfg(CONFIG_PPC_E500MC)]
pub const BOOKE_IRQPRIO_ALTIVEC_ASSIST: i32 = 6;
pub const BOOKE_IRQPRIO_SYSCALL: i32 = 8;
pub const BOOKE_IRQPRIO_AP_UNAVAIL: i32 = 9;
pub const BOOKE_IRQPRIO_DTLB_MISS: i32 = 10;
pub const BOOKE_IRQPRIO_ITLB_MISS: i32 = 11;
pub const BOOKE_IRQPRIO_MACHINE_CHECK: i32 = 12;
pub const BOOKE_IRQPRIO_DEBUG: i32 = 13;
pub const BOOKE_IRQPRIO_CRITICAL: i32 = 14;
pub const BOOKE_IRQPRIO_WATCHDOG: i32 = 15;
pub const BOOKE_IRQPRIO_EXTERNAL: i32 = 16;
pub const BOOKE_IRQPRIO_FIT: i32 = 17;
pub const BOOKE_IRQPRIO_DECREMENTER: i32 = 18;
pub const BOOKE_IRQPRIO_PERFORMANCE_MONITOR: i32 = 19;
/* Internal pseudo-irqprio for level triggered externals */
pub const BOOKE_IRQPRIO_EXTERNAL_LEVEL: i32 = 20;
pub const BOOKE_IRQPRIO_DBELL: i32 = 21;
pub const BOOKE_IRQPRIO_DBELL_CRIT: i32 = 22;
pub const BOOKE_IRQPRIO_MAX: i32 = 23;

pub const BOOKE_IRQMASK_EE: i32 = (1 << BOOKE_IRQPRIO_EXTERNAL_LEVEL)
    | (1 << BOOKE_IRQPRIO_PERFORMANCE_MONITOR)
    | (1 << BOOKE_IRQPRIO_DBELL)
    | (1 << BOOKE_IRQPRIO_DECREMENTER)
    | (1 << BOOKE_IRQPRIO_FIT)
    | (1 << BOOKE_IRQPRIO_EXTERNAL);

pub const BOOKE_IRQMASK_CE: i32 = (1 << BOOKE_IRQPRIO_DBELL_CRIT)
    | (1 << BOOKE_IRQPRIO_WATCHDOG)
    | (1 << BOOKE_IRQPRIO_CRITICAL);

extern "C" {
    pub static mut kvmppc_booke_handlers: ::core::ffi::c_ulong;
    pub static mut kvmppc_booke_handler_addr: [::core::ffi::c_ulong; 0];

    pub fn kvmppc_set_msr(vcpu: *mut kvm_vcpu, new_msr: u32);
    pub fn kvmppc_mmu_msr_notify(vcpu: *mut kvm_vcpu, old_msr: u32);

    pub fn kvmppc_set_epcr(vcpu: *mut kvm_vcpu, new_epcr: u32);
    pub fn kvmppc_set_tcr(vcpu: *mut kvm_vcpu, new_tcr: u32);
    pub fn kvmppc_set_tsr_bits(vcpu: *mut kvm_vcpu, tsr_bits: u32);
    pub fn kvmppc_clr_tsr_bits(vcpu: *mut kvm_vcpu, tsr_bits: u32);

    pub fn kvmppc_booke_emulate_op(vcpu: *mut kvm_vcpu, inst: ::core::ffi::c_uint, advance: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn kvmppc_booke_emulate_mfspr(vcpu: *mut kvm_vcpu, sprn: ::core::ffi::c_int, spr_val: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn kvmppc_booke_emulate_mtspr(vcpu: *mut kvm_vcpu, sprn: ::core::ffi::c_int, spr_val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;

    pub fn kvmppc_load_guest_spe(vcpu: *mut kvm_vcpu);
    pub fn kvmppc_save_guest_spe(vcpu: *mut kvm_vcpu);
    pub fn kvmppc_vcpu_disable_spe(vcpu: *mut kvm_vcpu);

    pub fn kvmppc_booke_vcpu_load(vcpu: *mut kvm_vcpu, cpu: ::core::ffi::c_int);
    pub fn kvmppc_booke_vcpu_put(vcpu: *mut kvm_vcpu);

    pub fn kvmppc_set_pending_interrupt(vcpu: *mut kvm_vcpu, type_: int_class);

    pub fn kvmppc_core_emulate_op_e500(vcpu: *mut kvm_vcpu, inst: ::core::ffi::c_uint, advance: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn kvmppc_core_emulate_mtspr_e500(vcpu: *mut kvm_vcpu, sprn: ::core::ffi::c_int, spr_val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn kvmppc_core_emulate_mfspr_e500(vcpu: *mut kvm_vcpu, sprn: ::core::ffi::c_int, spr_val: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn kvmppc_core_emulate_mtspr_e500(vcpu: *mut kvm_vcpu, sprn: ::core::ffi::c_int, spr_val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn kvmppc_core_emulate_mfspr_e500(vcpu: *mut kvm_vcpu, sprn: ::core::ffi::c_int, spr_val: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;

    pub fn mtspr(sprn: ::core::ffi::c_int, value: ::core::ffi::c_ulong);
    pub fn mfspr(sprn: ::core::ffi::c_int) -> ::core::ffi::c_ulong;

    pub fn kvmppc_handle_exit(vcpu: *mut kvm_vcpu, exit_nr: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum int_class {
    INT_CLASS_NONCRIT,
    INT_CLASS_CRIT,
    INT_CLASS_MC,
    INT_CLASS_DBG,
}

#[inline]
pub unsafe fn kvmppc_clear_dbsr() {
    mtspr(SPRN_DBSR, mfspr(SPRN_DBSR));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
