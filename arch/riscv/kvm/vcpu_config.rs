// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2026 Qualcomm Technologies, Inc.
 */

// Dependencies supplied by the surrounding kernel/RISC-V translation unit.

const KVM_HEDELEG_DEFAULT: usize = (BIT(EXC_INST_MISALIGNED)
    | BIT(EXC_INST_ILLEGAL)
    | BIT(EXC_BREAKPOINT)
    | BIT(EXC_SYSCALL)
    | BIT(EXC_INST_PAGE_FAULT)
    | BIT(EXC_LOAD_PAGE_FAULT)
    | BIT(EXC_STORE_PAGE_FAULT));

const KVM_HIDELEG_DEFAULT: usize =
    BIT(IRQ_VS_SOFT) | BIT(IRQ_VS_TIMER) | BIT(IRQ_VS_EXT);

pub unsafe fn kvm_riscv_vcpu_config_init(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.cfg.hedeleg = KVM_HEDELEG_DEFAULT;
    (*vcpu).arch.cfg.hideleg = KVM_HIDELEG_DEFAULT;
}

pub unsafe fn kvm_riscv_vcpu_config_guest_debug(vcpu: *mut kvm_vcpu) {
    let cfg: *mut kvm_vcpu_config = &mut (*vcpu).arch.cfg;

    if (*vcpu).guest_debug {
        (*cfg).hedeleg &= !BIT(EXC_BREAKPOINT);
    } else {
        (*cfg).hedeleg |= BIT(EXC_BREAKPOINT);
    }

    (*vcpu).arch.csr_dirty = true;
}

pub unsafe fn kvm_riscv_vcpu_config_ran_once(vcpu: *mut kvm_vcpu) {
    let isa: *const c_ulong = (*vcpu).arch.isa;
    let cfg: *mut kvm_vcpu_config = &mut (*vcpu).arch.cfg;

    if riscv_isa_extension_available(isa, SVPBMT) {
        (*cfg).henvcfg |= ENVCFG_PBMTE;
    }

    if riscv_isa_extension_available(isa, SSTC) {
        (*cfg).henvcfg |= ENVCFG_STCE;
    }

    if riscv_isa_extension_available(isa, ZICBOM) {
        (*cfg).henvcfg |= ENVCFG_CBIE | ENVCFG_CBCFE;
    }

    if riscv_isa_extension_available(isa, ZICBOZ) {
        (*cfg).henvcfg |= ENVCFG_CBZE;
    }

    if riscv_isa_extension_available(isa, SVADU)
        && !riscv_isa_extension_available(isa, SVADE)
    {
        (*cfg).henvcfg |= ENVCFG_ADUE;
    }

    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SMSTATEEN) {
        (*cfg).hstateen0 |= SMSTATEEN0_HSENVCFG;
        if riscv_isa_extension_available(isa, SSAIA) {
            (*cfg).hstateen0 |= SMSTATEEN0_AIA_IMSIC | SMSTATEEN0_AIA | SMSTATEEN0_AIA_ISEL;
        }
        if riscv_isa_extension_available(isa, SMSTATEEN) {
            (*cfg).hstateen0 |= SMSTATEEN0_SSTATEEN0;
        }
    }

    if (*vcpu).guest_debug {
        (*cfg).hedeleg &= !BIT(EXC_BREAKPOINT);
    }

    kvm_riscv_vcpu_sbi_validate(vcpu);
}

pub unsafe fn kvm_riscv_vcpu_config_load(vcpu: *mut kvm_vcpu) {
    let cfg: *mut kvm_vcpu_config = &mut (*vcpu).arch.cfg;
    let mut nsh: *mut core::ffi::c_void;

    if kvm_riscv_nacl_sync_csr_available() {
        nsh = nacl_shmem();
        nacl_csr_write(nsh, CSR_HEDELEG, (*cfg).hedeleg);
        nacl_csr_write(nsh, CSR_HIDELEG, (*cfg).hideleg);
        nacl_csr_write(nsh, CSR_HENVCFG, (*cfg).henvcfg);
        if cfg!(target_pointer_width = "32") {
            nacl_csr_write(nsh, CSR_HENVCFGH, (*cfg).henvcfg >> 32);
        }
        if riscv_has_extension_unlikely(RISCV_ISA_EXT_SMSTATEEN) {
            nacl_csr_write(nsh, CSR_HSTATEEN0, (*cfg).hstateen0);
            if cfg!(target_pointer_width = "32") {
                nacl_csr_write(nsh, CSR_HSTATEEN0H, (*cfg).hstateen0 >> 32);
            }
        }
    } else {
        csr_write(CSR_HEDELEG, (*cfg).hedeleg);
        csr_write(CSR_HIDELEG, (*cfg).hideleg);
        csr_write(CSR_HENVCFG, (*cfg).henvcfg);
        if cfg!(target_pointer_width = "32") {
            csr_write(CSR_HENVCFGH, (*cfg).henvcfg >> 32);
        }
        if riscv_has_extension_unlikely(RISCV_ISA_EXT_SMSTATEEN) {
            csr_write(CSR_HSTATEEN0, (*cfg).hstateen0);
            if cfg!(target_pointer_width = "32") {
                csr_write(CSR_HSTATEEN0H, (*cfg).hstateen0 >> 32);
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
