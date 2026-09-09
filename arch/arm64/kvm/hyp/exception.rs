// SPDX-License-Identifier: GPL-2.0-only
/*
 * Fault injection for both 32 and 64bit guests.
 *
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * Based on arch/arm/kvm/emulate.c
 * Copyright (C) 2012 - Virtual Open Systems and Columbia University
 * Author: Christoffer Dall <c.dall@virtualopensystems.com>
 */

// Hypervisor code only: __KVM_NVHE_HYPERVISOR__ or __KVM_VHE_HYPERVISOR__

unsafe fn __vcpu_write_spsr(vcpu: *mut kvm_vcpu, target_mode: c_ulong, val: u64) {
    if has_vhe() {
        if target_mode == PSR_MODE_EL1h {
            vcpu_write_sys_reg(vcpu, val, SPSR_EL1);
        } else {
            vcpu_write_sys_reg(vcpu, val, SPSR_EL2);
        }
    } else {
        __vcpu_assign_sys_reg(vcpu, SPSR_EL1, val);
    }
}

unsafe fn __vcpu_write_spsr_abt(vcpu: *mut kvm_vcpu, val: u64) {
    if has_vhe() && vcpu_get_flag(vcpu, SYSREGS_ON_CPU) {
        write_sysreg!(val, spsr_abt);
    } else {
        (*vcpu).arch.ctxt.spsr_abt = val;
    }
}

unsafe fn __vcpu_write_spsr_und(vcpu: *mut kvm_vcpu, val: u64) {
    if has_vhe() && vcpu_get_flag(vcpu, SYSREGS_ON_CPU) {
        write_sysreg!(val, spsr_und);
    } else {
        (*vcpu).arch.ctxt.spsr_und = val;
    }
}

unsafe fn enter_exception64(vcpu: *mut kvm_vcpu, target_mode: c_ulong, type_: exception_type) {
    let mut sctlr: c_ulong;
    let mut vbar: c_ulong;
    let old: c_ulong;
    let mut new: c_ulong;
    let mode: c_ulong;
    let exc_offset: u64;

    mode = *vcpu_cpsr(vcpu) & (PSR_MODE_MASK | PSR_MODE32_BIT);

    if mode == target_mode {
        exc_offset = CURRENT_EL_SP_ELx_VECTOR;
    } else if (mode | PSR_MODE_THREAD_BIT) == target_mode {
        exc_offset = CURRENT_EL_SP_EL0_VECTOR;
    } else if (mode & PSR_MODE32_BIT) == 0 {
        exc_offset = LOWER_EL_AArch64_VECTOR;
    } else {
        exc_offset = LOWER_EL_AArch32_VECTOR;
    }

    match target_mode {
        PSR_MODE_EL1h => {
            vbar = vcpu_read_sys_reg(vcpu, VBAR_EL1);
            sctlr = vcpu_read_sys_reg(vcpu, SCTLR_EL1);
            vcpu_write_sys_reg(vcpu, *vcpu_pc(vcpu), ELR_EL1);
        }
        PSR_MODE_EL2h => {
            vbar = vcpu_read_sys_reg(vcpu, VBAR_EL2);
            sctlr = vcpu_read_sys_reg(vcpu, SCTLR_EL2);
            vcpu_write_sys_reg(vcpu, *vcpu_pc(vcpu), ELR_EL2);
        }
        _ => BUG!(),
    }

    *vcpu_pc(vcpu) = vbar + exc_offset + type_ as u64;

    old = *vcpu_cpsr(vcpu);
    new = 0;
    new |= old & PSR_N_BIT;
    new |= old & PSR_Z_BIT;
    new |= old & PSR_C_BIT;
    new |= old & PSR_V_BIT;

    if kvm_has_mte(kern_hyp_va((*vcpu).kvm)) {
        new |= PSR_TCO_BIT;
    }
    new |= old & PSR_DIT_BIT;

    // PSTATE.UAO is set to zero upon any exception to AArch64.
    // PSTATE.PAN is unchanged unless SCTLR_ELx.SPAN == 0b0.
    new |= old & PSR_PAN_BIT;
    if (sctlr & SCTLR_EL1_SPAN) == 0 {
        new |= PSR_PAN_BIT;
    }
    // PSTATE.SS and PSTATE.IL are set to zero upon any exception to AArch64.
    if sctlr & SCTLR_ELx_DSSBS != 0 {
        new |= PSR_SSBS_BIT;
    }
    // PSTATE.BTYPE is set to zero upon any exception to AArch64.
    new |= PSR_D_BIT | PSR_A_BIT | PSR_I_BIT | PSR_F_BIT;
    new |= target_mode;

    *vcpu_cpsr(vcpu) = new;
    __vcpu_write_spsr(vcpu, target_mode, old);
}

unsafe fn get_except32_cpsr(vcpu: *mut kvm_vcpu, mode: u32) -> c_ulong {
    let sctlr: u32 = vcpu_read_sys_reg(vcpu, SCTLR_EL1);
    let old = *vcpu_cpsr(vcpu);
    let mut new: c_ulong = 0;
    new |= old & PSR_AA32_N_BIT;
    new |= old & PSR_AA32_Z_BIT;
    new |= old & PSR_AA32_C_BIT;
    new |= old & PSR_AA32_V_BIT;
    new |= old & PSR_AA32_Q_BIT;
    new |= old & PSR_AA32_DIT_BIT;
    if sctlr & BIT(31) != 0 { new |= PSR_AA32_SSBS_BIT; }
    new |= old & PSR_AA32_PAN_BIT;
    if sctlr & BIT(23) == 0 { new |= PSR_AA32_PAN_BIT; }
    new |= old & PSR_AA32_GE_MASK;
    if sctlr & BIT(25) != 0 { new |= PSR_AA32_E_BIT; }
    new |= old & PSR_AA32_A_BIT;
    if mode != PSR_AA32_MODE_UND && mode != PSR_AA32_MODE_SVC { new |= PSR_AA32_A_BIT; }
    new |= PSR_AA32_I_BIT;
    new |= old & PSR_AA32_F_BIT;
    if mode == PSR_AA32_MODE_FIQ { new |= PSR_AA32_F_BIT; }
    if sctlr & BIT(30) != 0 { new |= PSR_AA32_T_BIT; }
    new |= mode as c_ulong;
    new
}

// Table taken from ARMv8 ARM DDI0487B-B, table G1-10.
static RETURN_OFFSETS: [[u8; 2]; 8] = [
    [0, 0], [4, 2], [0, 0], [4, 4], [8, 8], [0, 0], [4, 4], [4, 4],
];

unsafe fn enter_exception32(vcpu: *mut kvm_vcpu, mode: u32, vect_offset: u32) {
    let spsr = *vcpu_cpsr(vcpu);
    let is_thumb = (spsr & PSR_AA32_T_BIT) != 0;
    let sctlr: u32 = vcpu_read_sys_reg(vcpu, SCTLR_EL1);
    let mut return_address = *vcpu_pc(vcpu);
    return_address += RETURN_OFFSETS[(vect_offset >> 2) as usize][if is_thumb { 1 } else { 0 }] as c_ulong;
    *vcpu_cpsr(vcpu) = get_except32_cpsr(vcpu, mode);
    match mode {
        PSR_AA32_MODE_ABT => {
            __vcpu_write_spsr_abt(vcpu, host_spsr_to_spsr32(spsr));
            vcpu_gp_regs(vcpu).compat_lr_abt = return_address;
        }
        PSR_AA32_MODE_UND => {
            __vcpu_write_spsr_und(vcpu, host_spsr_to_spsr32(spsr));
            vcpu_gp_regs(vcpu).compat_lr_und = return_address;
        }
        _ => {}
    }
    let vector = if sctlr & (1 << 13) != 0 {
        vect_offset.wrapping_add(0xffff0000)
    } else {
        vect_offset.wrapping_add(vcpu_read_sys_reg(vcpu, VBAR_EL1))
    };
    *vcpu_pc(vcpu) = vector;
}

unsafe fn kvm_inject_exception(vcpu: *mut kvm_vcpu) {
    if vcpu_el1_is_32bit(vcpu) {
        match vcpu_get_flag(vcpu, EXCEPT_MASK) {
            unpack_vcpu_flag!(EXCEPT_AA32_UND) => enter_exception32(vcpu, PSR_AA32_MODE_UND, 4),
            unpack_vcpu_flag!(EXCEPT_AA32_IABT) => enter_exception32(vcpu, PSR_AA32_MODE_ABT, 12),
            unpack_vcpu_flag!(EXCEPT_AA32_DABT) => enter_exception32(vcpu, PSR_AA32_MODE_ABT, 16),
            _ => {}
        }
    } else {
        match vcpu_get_flag(vcpu, EXCEPT_MASK) {
            unpack_vcpu_flag!(EXCEPT_AA64_EL1_SYNC) => enter_exception64(vcpu, PSR_MODE_EL1h, except_type_sync),
            unpack_vcpu_flag!(EXCEPT_AA64_EL1_SERR) => enter_exception64(vcpu, PSR_MODE_EL1h, except_type_serror),
            unpack_vcpu_flag!(EXCEPT_AA64_EL2_SYNC) => enter_exception64(vcpu, PSR_MODE_EL2h, except_type_sync),
            unpack_vcpu_flag!(EXCEPT_AA64_EL2_IRQ) => enter_exception64(vcpu, PSR_MODE_EL2h, except_type_irq),
            unpack_vcpu_flag!(EXCEPT_AA64_EL2_SERR) => enter_exception64(vcpu, PSR_MODE_EL2h, except_type_serror),
            _ => {}
        }
    }
}

pub unsafe fn __kvm_adjust_pc(vcpu: *mut kvm_vcpu) {
    if vcpu_get_flag(vcpu, PENDING_EXCEPTION) {
        kvm_inject_exception(vcpu);
        vcpu_clear_flag(vcpu, PENDING_EXCEPTION);
        vcpu_clear_flag(vcpu, EXCEPT_MASK);
    } else if vcpu_get_flag(vcpu, INCREMENT_PC) {
        kvm_skip_instr(vcpu);
        vcpu_clear_flag(vcpu, INCREMENT_PC);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
