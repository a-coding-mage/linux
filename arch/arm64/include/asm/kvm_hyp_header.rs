/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// #include <linux/compiler.h>
// #include <linux/kvm_host.h>
// #include <asm/alternative.h>
// #include <asm/sysreg.h>

extern "C" {
    // DECLARE_PER_CPU(struct kvm_cpu_context, kvm_hyp_ctxt);
    pub static mut kvm_hyp_ctxt: kvm_cpu_context;
    // DECLARE_PER_CPU(unsigned long, kvm_hyp_vector);
    pub static mut kvm_hyp_vector: c_ulong;
    // DECLARE_PER_CPU(struct kvm_nvhe_init_params, kvm_init_params);
    pub static mut kvm_init_params: kvm_nvhe_init_params;
}

/*
 * Unified accessors for registers that have a different encoding
 * between VHE and non-VHE. They must be specified without their "ELx"
 * encoding, but with the SYS_ prefix, as defined in asm/sysreg.h.
 */

#[cfg(__KVM_VHE_HYPERVISOR__)]
macro_rules! read_sysreg_el0 { ($r:ident) => { read_sysreg_s(concat_idents!($r, _EL02)) }; }
#[cfg(__KVM_VHE_HYPERVISOR__)]
macro_rules! write_sysreg_el0 { ($v:expr, $r:ident) => { write_sysreg_s($v, concat_idents!($r, _EL02)) }; }
#[cfg(__KVM_VHE_HYPERVISOR__)]
macro_rules! read_sysreg_el1 { ($r:ident) => { read_sysreg_s(concat_idents!($r, _EL12)) }; }
#[cfg(__KVM_VHE_HYPERVISOR__)]
macro_rules! write_sysreg_el1 { ($v:expr, $r:ident) => { write_sysreg_s($v, concat_idents!($r, _EL12)) }; }
#[cfg(__KVM_VHE_HYPERVISOR__)]
macro_rules! read_sysreg_el2 { ($r:ident) => { read_sysreg_s(concat_idents!($r, _EL1)) }; }
#[cfg(__KVM_VHE_HYPERVISOR__)]
macro_rules! write_sysreg_el2 { ($v:expr, $r:ident) => { write_sysreg_s($v, concat_idents!($r, _EL1)) }; }

// !__KVM_VHE_HYPERVISOR__
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
#[cfg(__KVM_NVHE_HYPERVISOR__)]
const VHE_ALT_KEY: u32 = ARM64_KVM_HVHE;
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
#[cfg(not(__KVM_NVHE_HYPERVISOR__))]
const VHE_ALT_KEY: u32 = ARM64_HAS_VIRT_HOST_EXTN;

// The C read_sysreg_elx/write_sysreg_elx macros contain architecture-specific
// inline assembly and ALTERNATIVE patching; preserve their dependency here.
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
macro_rules! read_sysreg_elx {
    ($r:ident, $nvh:ident, $vh:ident) => {{
        let mut reg: u64;
        unsafe {
            core::arch::asm!("mrs {0}, {1}", out(reg) reg, const VHE_ALT_KEY);
        }
        reg
    }};
}
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
macro_rules! write_sysreg_elx {
    ($v:expr, $r:ident, $nvh:ident, $vh:ident) => {{
        let __val: u64 = ($v) as u64;
        unsafe { core::arch::asm!("", in(reg) __val); }
    }};
}
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
macro_rules! read_sysreg_el0 { ($r:ident) => { read_sysreg_elx!($r, _EL0, _EL02) }; }
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
macro_rules! write_sysreg_el0 { ($v:expr, $r:ident) => { write_sysreg_elx!($v, $r, _EL0, _EL02) }; }
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
macro_rules! read_sysreg_el1 { ($r:ident) => { read_sysreg_elx!($r, _EL1, _EL12) }; }
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
macro_rules! write_sysreg_el1 { ($v:expr, $r:ident) => { write_sysreg_elx!($v, $r, _EL1, _EL12) }; }
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
macro_rules! read_sysreg_el2 { ($r:ident) => { read_sysreg_elx!($r, _EL2, _EL1) }; }
#[cfg(not(__KVM_VHE_HYPERVISOR__))]
macro_rules! write_sysreg_el2 { ($v:expr, $r:ident) => { write_sysreg_elx!($v, $r, _EL2, _EL1) }; }

/*
 * Without an __arch_swab32(), we fall back to ___constant_swab32(), but the
 * static inline can allow the compiler to out-of-line this. KVM always wants
 * the macro version as it's always inlined.
 */
macro_rules! __kvm_swab32 { ($x:expr) => { ___constant_swab32!($x) }; }

extern "C" {
    pub fn __vgic_v2_perform_cpuif_access(vcpu: *mut kvm_vcpu) -> c_int;
    pub fn __gic_v3_get_lr(lr: c_uint) -> u64;
    pub fn __gic_v3_set_lr(val: u64, lr: c_int);
    pub fn __vgic_v3_save_state(cpu_if: *mut vgic_v3_cpu_if);
    pub fn __vgic_v3_restore_state(cpu_if: *mut vgic_v3_cpu_if);
    pub fn __vgic_v3_activate_traps(cpu_if: *mut vgic_v3_cpu_if);
    pub fn __vgic_v3_deactivate_traps(cpu_if: *mut vgic_v3_cpu_if);
    pub fn __vgic_v3_save_aprs(cpu_if: *mut vgic_v3_cpu_if);
    pub fn __vgic_v3_restore_vmcr_aprs(cpu_if: *mut vgic_v3_cpu_if);
    pub fn __vgic_v3_perform_cpuif_access(vcpu: *mut kvm_vcpu) -> c_int;

    /* GICv5 */
    pub fn __vgic_v5_save_apr(cpu_if: *mut vgic_v5_cpu_if);
    pub fn __vgic_v5_restore_vmcr_apr(cpu_if: *mut vgic_v5_cpu_if);
    /* No hypercalls for the following */
    pub fn __vgic_v5_save_ppi_state(cpu_if: *mut vgic_v5_cpu_if);
    pub fn __vgic_v5_restore_ppi_state(cpu_if: *mut vgic_v5_cpu_if);
    pub fn __vgic_v5_save_state(cpu_if: *mut vgic_v5_cpu_if);
    pub fn __vgic_v5_restore_state(cpu_if: *mut vgic_v5_cpu_if);
}

#[cfg(__KVM_NVHE_HYPERVISOR__)]
extern "C" {
    pub fn __timer_enable_traps(vcpu: *mut kvm_vcpu);
    pub fn __timer_disable_traps(vcpu: *mut kvm_vcpu);
    pub fn __sysreg_save_state_nvhe(ctxt: *mut kvm_cpu_context);
    pub fn __sysreg_restore_state_nvhe(ctxt: *mut kvm_cpu_context);
}
#[cfg(not(__KVM_NVHE_HYPERVISOR__))]
extern "C" {
    pub fn __vcpu_load_switch_sysregs(vcpu: *mut kvm_vcpu);
    pub fn __vcpu_put_switch_sysregs(vcpu: *mut kvm_vcpu);
    pub fn sysreg_save_host_state_vhe(ctxt: *mut kvm_cpu_context);
    pub fn sysreg_restore_host_state_vhe(ctxt: *mut kvm_cpu_context);
    pub fn sysreg_save_guest_state_vhe(ctxt: *mut kvm_cpu_context);
    pub fn sysreg_restore_guest_state_vhe(ctxt: *mut kvm_cpu_context);
}

extern "C" {
    pub fn __debug_switch_to_guest(vcpu: *mut kvm_vcpu);
    pub fn __debug_switch_to_host(vcpu: *mut kvm_vcpu);
}
#[cfg(__KVM_NVHE_HYPERVISOR__)]
extern "C" {
    pub fn __debug_save_host_buffers_nvhe(vcpu: *mut kvm_vcpu);
    pub fn __debug_restore_host_buffers_nvhe(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn __guest_enter(vcpu: *mut kvm_vcpu) -> u64;
    pub fn kvm_host_psci_handler(host_ctxt: *mut kvm_cpu_context, func_id: u32) -> bool;
}

#[cfg(__KVM_NVHE_HYPERVISOR__)]
extern "C" {
    pub fn __hyp_do_panic(host_ctxt: *mut kvm_cpu_context, spsr: u64, elr: u64, par: u64) -> !;
    pub fn __pkvm_init_switch_pgd(pgd: phys_addr_t, sp: c_ulong, f: Option<unsafe extern "C" fn()>);
    pub fn __pkvm_init(phys: phys_addr_t, size: c_ulong, per_cpu_base: *mut c_ulong, hyp_va_bits: u32) -> c_int;
    pub fn __host_enter(host_ctxt: *mut kvm_cpu_context) -> !;
}

extern "C" {
    pub static mut id_aa64pfr0_el1_sys_val: u64;
    pub static mut id_aa64pfr1_el1_sys_val: u64;
    pub static mut id_aa64pfr2_el1_sys_val: u64;
    pub static mut id_aa64isar0_el1_sys_val: u64;
    pub static mut id_aa64isar1_el1_sys_val: u64;
    pub static mut id_aa64isar2_el1_sys_val: u64;
    pub static mut id_aa64mmfr0_el1_sys_val: u64;
    pub static mut id_aa64mmfr1_el1_sys_val: u64;
    pub static mut id_aa64mmfr2_el1_sys_val: u64;
    pub static mut id_aa64smfr0_el1_sys_val: u64;
    pub static mut __icache_flags: c_ulong;
    pub static mut kvm_arm_vmid_bits: c_uint;
    pub static mut kvm_host_sve_max_vl: c_uint;
    pub static mut hyp_nr_cpus: c_ulong;
    pub static mut hyp_gicv3_nr_lr: c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
