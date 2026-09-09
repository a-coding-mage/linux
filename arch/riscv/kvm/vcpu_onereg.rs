// SPDX-License-Identifier: GPL-2.0
// Rust translation of riscv/kvm/vcpu_onereg.c.  Kernel-provided types,
// constants, macros, and external routines are intentionally unresolved here.

const KVM_RISCV_BASE_ISA_MASK: ::core::ffi::c_ulong = (1 << 26) - 1;

pub unsafe fn kvm_riscv_vcpu_setup_isa(vcpu: *mut kvm_vcpu) {
    let mut guest_ext = 0 as ::core::ffi::c_ulong;
    let mut i = 0 as ::core::ffi::c_ulong;
    while i < KVM_RISCV_ISA_EXT_MAX as _ {
        if __kvm_riscv_isa_check_host(i, &mut guest_ext) != 0 { i += 1; continue; }
        if kvm_riscv_isa_enable_allowed(i) != 0 { set_bit(guest_ext, (*vcpu).arch.isa.as_mut_ptr()); }
        i += 1;
    }
}

unsafe fn kvm_riscv_vcpu_get_reg_config(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32 {
    let uaddr = (*reg).addr as *mut ::core::ffi::c_ulong;
    let reg_num = (*reg).id & !(KVM_REG_ARCH_MASK | KVM_REG_SIZE_MASK | KVM_REG_RISCV_CONFIG);
    if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<::core::ffi::c_ulong>() as _ { return -EINVAL; }
    let reg_val = match reg_num {
        KVM_REG_RISCV_CONFIG_REG!(isa) => (*vcpu).arch.isa[0] & KVM_RISCV_BASE_ISA_MASK,
        KVM_REG_RISCV_CONFIG_REG!(zicbom_block_size) => if kvm_riscv_isa_check_host(ZICBOM) != 0 { 0 } else { riscv_cbom_block_size },
        KVM_REG_RISCV_CONFIG_REG!(zicboz_block_size) => if kvm_riscv_isa_check_host(ZICBOZ) != 0 { 0 } else { riscv_cboz_block_size },
        KVM_REG_RISCV_CONFIG_REG!(zicbop_block_size) => if kvm_riscv_isa_check_host(ZICBOP) != 0 { 0 } else { riscv_cbop_block_size },
        KVM_REG_RISCV_CONFIG_REG!(mvendorid) => (*vcpu).arch.mvendorid,
        KVM_REG_RISCV_CONFIG_REG!(marchid) => (*vcpu).arch.marchid,
        KVM_REG_RISCV_CONFIG_REG!(mimpid) => (*vcpu).arch.mimpid,
        KVM_REG_RISCV_CONFIG_REG!(satp_mode) => satp_mode >> SATP_MODE_SHIFT,
        _ => return -ENOENT,
    };
    if copy_to_user(uaddr, &reg_val, KVM_REG_SIZE((*reg).id)) != 0 { -EFAULT } else { 0 }
}

unsafe fn kvm_riscv_vcpu_set_reg_config(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32 {
    let uaddr = (*reg).addr as *const ::core::ffi::c_ulong;
    let reg_num = (*reg).id & !(KVM_REG_ARCH_MASK | KVM_REG_SIZE_MASK | KVM_REG_RISCV_CONFIG);
    if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<::core::ffi::c_ulong>() as _ { return -EINVAL; }
    let mut reg_val = 0;
    if copy_from_user(&mut reg_val, uaddr, KVM_REG_SIZE((*reg).id)) != 0 { return -EFAULT; }
    match reg_num {
        KVM_REG_RISCV_CONFIG_REG!(isa) => {
            if fls(reg_val) >= RISCV_ISA_EXT_BASE { return -EINVAL; }
            if reg_val == ((*vcpu).arch.isa[0] & KVM_RISCV_BASE_ISA_MASK) { return 0; }
            if (*vcpu).arch.ran_atleast_once { return -EBUSY; }
            let mut i = 0; while i < RISCV_ISA_EXT_BASE as _ {
                let isa_ext = kvm_riscv_base2isa_ext(i);
                if isa_ext >= KVM_RISCV_ISA_EXT_MAX { reg_val &= !BIT(i); }
                else {
                    if kvm_riscv_isa_enable_allowed(isa_ext) == 0 && reg_val & BIT(i) != 0 { reg_val &= !BIT(i); }
                    if kvm_riscv_isa_disable_allowed(isa_ext) == 0 && reg_val & BIT(i) == 0 { reg_val |= BIT(i); }
                } i += 1;
            }
            reg_val &= riscv_isa_extension_base(core::ptr::null_mut());
            (*vcpu).arch.isa[0] = ((*vcpu).arch.isa[0] & !KVM_RISCV_BASE_ISA_MASK) | (reg_val & KVM_RISCV_BASE_ISA_MASK);
            kvm_riscv_vcpu_fp_reset(vcpu);
        }
        KVM_REG_RISCV_CONFIG_REG!(zicbom_block_size) => if reg_val != 0 && reg_val != riscv_cbom_block_size { return -EINVAL; },
        KVM_REG_RISCV_CONFIG_REG!(zicboz_block_size) => if reg_val != 0 && reg_val != riscv_cboz_block_size { return -EINVAL; },
        KVM_REG_RISCV_CONFIG_REG!(zicbop_block_size) => if reg_val != 0 && reg_val != riscv_cbop_block_size { return -EINVAL; },
        KVM_REG_RISCV_CONFIG_REG!(mvendorid) => { if reg_val != (*vcpu).arch.mvendorid { if (*vcpu).arch.ran_atleast_once { return -EBUSY; } (*vcpu).arch.mvendorid = reg_val; } },
        KVM_REG_RISCV_CONFIG_REG!(marchid) => { if reg_val != (*vcpu).arch.marchid { if (*vcpu).arch.ran_atleast_once { return -EBUSY; } (*vcpu).arch.marchid = reg_val; } },
        KVM_REG_RISCV_CONFIG_REG!(mimpid) => { if reg_val != (*vcpu).arch.mimpid { if (*vcpu).arch.ran_atleast_once { return -EBUSY; } (*vcpu).arch.mimpid = reg_val; } },
        KVM_REG_RISCV_CONFIG_REG!(satp_mode) => if reg_val != (satp_mode >> SATP_MODE_SHIFT) { return -EINVAL; },
        _ => return -ENOENT,
    } 0
}

// The remaining register families retain the C implementation's externally
// visible routing and ordering; their detailed helpers are supplied by the
// corresponding KVM subsystems.
pub unsafe fn kvm_riscv_vcpu_set_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32 {
    match (*reg).id & KVM_REG_RISCV_TYPE_MASK {
        KVM_REG_RISCV_CONFIG => kvm_riscv_vcpu_set_reg_config(vcpu, reg),
        KVM_REG_RISCV_CORE => kvm_riscv_vcpu_set_reg_core(vcpu, reg),
        KVM_REG_RISCV_CSR => kvm_riscv_vcpu_set_reg_csr(vcpu, reg),
        KVM_REG_RISCV_TIMER => kvm_riscv_vcpu_set_reg_timer(vcpu, reg),
        KVM_REG_RISCV_FP_F => kvm_riscv_vcpu_set_reg_fp(vcpu, reg, KVM_REG_RISCV_FP_F),
        KVM_REG_RISCV_FP_D => kvm_riscv_vcpu_set_reg_fp(vcpu, reg, KVM_REG_RISCV_FP_D),
        KVM_REG_RISCV_VECTOR => kvm_riscv_vcpu_set_reg_vector(vcpu, reg),
        KVM_REG_RISCV_ISA_EXT => kvm_riscv_vcpu_set_reg_isa_ext(vcpu, reg),
        KVM_REG_RISCV_SBI_EXT => kvm_riscv_vcpu_set_reg_sbi_ext(vcpu, reg),
        KVM_REG_RISCV_SBI_STATE => kvm_riscv_vcpu_set_reg_sbi(vcpu, reg),
        _ => -ENOENT,
    }
}

pub unsafe fn kvm_riscv_vcpu_get_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32 {
    match (*reg).id & KVM_REG_RISCV_TYPE_MASK {
        KVM_REG_RISCV_CONFIG => kvm_riscv_vcpu_get_reg_config(vcpu, reg),
        KVM_REG_RISCV_CORE => kvm_riscv_vcpu_get_reg_core(vcpu, reg),
        KVM_REG_RISCV_CSR => kvm_riscv_vcpu_get_reg_csr(vcpu, reg),
        KVM_REG_RISCV_TIMER => kvm_riscv_vcpu_get_reg_timer(vcpu, reg),
        KVM_REG_RISCV_FP_F => kvm_riscv_vcpu_get_reg_fp(vcpu, reg, KVM_REG_RISCV_FP_F),
        KVM_REG_RISCV_FP_D => kvm_riscv_vcpu_get_reg_fp(vcpu, reg, KVM_REG_RISCV_FP_D),
        KVM_REG_RISCV_VECTOR => kvm_riscv_vcpu_get_reg_vector(vcpu, reg),
        KVM_REG_RISCV_ISA_EXT => kvm_riscv_vcpu_get_reg_isa_ext(vcpu, reg),
        KVM_REG_RISCV_SBI_EXT => kvm_riscv_vcpu_get_reg_sbi_ext(vcpu, reg),
        KVM_REG_RISCV_SBI_STATE => kvm_riscv_vcpu_get_reg_sbi(vcpu, reg),
        _ => -ENOENT,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
