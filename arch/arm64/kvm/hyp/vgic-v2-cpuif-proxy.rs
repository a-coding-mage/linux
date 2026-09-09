// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

type PhysAddr = usize;

const KVM_VGIC_V2_CPU_SIZE: PhysAddr = 0x2000;
const GIC_CPU_DEACTIVATE: PhysAddr = 0x1000;
const PSR_AA32_E_BIT: u64 = 1 << 9;
const SCTLR_ELx_EE: u64 = 1 << 25;

#[repr(C)]
pub struct KvmVcpu {
    pub kvm: *mut Kvm,
}

#[repr(C)]
pub struct Kvm {
    pub arch: KvmArch,
}

#[repr(C)]
pub struct KvmArch {
    pub vgic: VgicDist,
}

#[repr(C)]
pub struct VgicDist {
    pub vgic_cpu_base: PhysAddr,
}

#[repr(C)]
pub struct KvmVgicGlobalState {
    pub vcpu_hyp_va: *mut c_void,
}

extern "C" {
    static mut kvm_vgic_global_state: KvmVgicGlobalState;

    fn kern_hyp_va<T>(ptr: *mut T) -> *mut T;
    fn vcpu_mode_is_32bit(vcpu: *mut KvmVcpu) -> bool;
    fn read_sysreg_el2(reg: u32) -> u64;
    fn read_sysreg_el1(reg: u32) -> u64;
    fn kvm_vcpu_get_fault_ipa(vcpu: *mut KvmVcpu) -> PhysAddr;
    fn kvm_vcpu_get_hfar(vcpu: *mut KvmVcpu) -> u64;
    fn kvm_vcpu_dabt_get_as(vcpu: *mut KvmVcpu) -> usize;
    fn kvm_vcpu_dabt_get_rd(vcpu: *mut KvmVcpu) -> i32;
    fn kvm_vcpu_dabt_iswrite(vcpu: *mut KvmVcpu) -> bool;
    fn vcpu_get_reg(vcpu: *mut KvmVcpu, reg: i32) -> u32;
    fn vcpu_set_reg(vcpu: *mut KvmVcpu, reg: i32, value: u32);
    fn __kvm_skip_instr(vcpu: *mut KvmVcpu);
    fn __kvm_swab32(value: u32) -> u32;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn readl_relaxed(addr: *mut c_void) -> u32;
}

const SYS_SPSR: u32 = 0;
const SYS_SCTLR: u32 = 1;

#[inline]
unsafe fn far_to_fipa_offset(value: u64) -> PhysAddr {
    value as PhysAddr
}

unsafe fn __is_be(vcpu: *mut KvmVcpu) -> bool {
    if vcpu_mode_is_32bit(vcpu) {
        return (read_sysreg_el2(SYS_SPSR) & PSR_AA32_E_BIT) != 0;
    }

    (read_sysreg_el1(SYS_SCTLR) & SCTLR_ELx_EE) != 0
}

/*
 * __vgic_v2_perform_cpuif_access -- perform a GICV access on behalf of the
 *                                     guest.
 *
 * @vcpu: the offending vcpu
 *
 * Returns:
 *  1: GICV access successfully performed
 *  0: Not a GICV access
 * -1: Illegal GICV access successfully performed
 */
pub unsafe fn __vgic_v2_perform_cpuif_access(vcpu: *mut KvmVcpu) -> i32 {
    let kvm = kern_hyp_va((*vcpu).kvm);
    let vgic = &(*kvm).arch.vgic;
    let mut fault_ipa: PhysAddr;
    let addr: *mut c_void;
    let rd: i32;

    /* Build the full address */
    fault_ipa = kvm_vcpu_get_fault_ipa(vcpu);
    fault_ipa |= far_to_fipa_offset(kvm_vcpu_get_hfar(vcpu));

    /* If not for GICV, move on */
    if fault_ipa < vgic.vgic_cpu_base
        || fault_ipa >= vgic.vgic_cpu_base + KVM_VGIC_V2_CPU_SIZE
    {
        return 0;
    }

    /* Reject anything but a 32bit access */
    if kvm_vcpu_dabt_get_as(vcpu) != core::mem::size_of::<u32>() {
        __kvm_skip_instr(vcpu);
        return -1;
    }

    /* Not aligned? Don't bother */
    if fault_ipa & 3 != 0 {
        __kvm_skip_instr(vcpu);
        return -1;
    }

    /* Handle deactivation as a normal exit */
    if fault_ipa - vgic.vgic_cpu_base >= GIC_CPU_DEACTIVATE {
        return 0;
    }

    rd = kvm_vcpu_dabt_get_rd(vcpu);
    addr = (kvm_vgic_global_state.vcpu_hyp_va as *mut u8)
        .add(fault_ipa - vgic.vgic_cpu_base) as *mut c_void;

    if kvm_vcpu_dabt_iswrite(vcpu) {
        let mut data = vcpu_get_reg(vcpu, rd);
        if __is_be(vcpu) {
            /* guest pre-swabbed data, undo this for writel() */
            data = __kvm_swab32(data);
        }
        writel_relaxed(data, addr);
    } else {
        let mut data = readl_relaxed(addr);
        if __is_be(vcpu) {
            /* guest expects swabbed data */
            data = __kvm_swab32(data);
        }
        vcpu_set_reg(vcpu, rd, data);
    }

    __kvm_skip_instr(vcpu);

    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
