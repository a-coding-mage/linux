// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Atish Patra <atish.patra@wdc.com>
 *     Anup Patel <anup.patel@wdc.com>
 */

// Linux and architecture dependencies are supplied by other translation units.

// CONFIG_FPU
pub unsafe fn kvm_riscv_vcpu_fp_reset(vcpu: *mut kvm_vcpu) {
    let cntx: *mut kvm_cpu_context = &mut (*vcpu).arch.guest_context;

    (*cntx).sstatus &= !SR_FS;
    if riscv_isa_extension_available((*vcpu).arch.isa, F)
        || riscv_isa_extension_available((*vcpu).arch.isa, D)
    {
        (*cntx).sstatus |= SR_FS_INITIAL;
    } else {
        (*cntx).sstatus |= SR_FS_OFF;
    }
}

// CONFIG_FPU
unsafe fn kvm_riscv_vcpu_fp_clean(cntx: *mut kvm_cpu_context) {
    (*cntx).sstatus &= !SR_FS;
    (*cntx).sstatus |= SR_FS_CLEAN;
}

// CONFIG_FPU
pub unsafe fn kvm_riscv_vcpu_guest_fp_save(
    cntx: *mut kvm_cpu_context,
    isa: *const c_ulong,
) {
    if ((*cntx).sstatus & SR_FS) == SR_FS_DIRTY {
        if riscv_isa_extension_available(isa, D) {
            __kvm_riscv_fp_d_save(cntx);
        } else if riscv_isa_extension_available(isa, F) {
            __kvm_riscv_fp_f_save(cntx);
        }
        kvm_riscv_vcpu_fp_clean(cntx);
    }
}

// CONFIG_FPU
pub unsafe fn kvm_riscv_vcpu_guest_fp_restore(
    cntx: *mut kvm_cpu_context,
    isa: *const c_ulong,
) {
    if ((*cntx).sstatus & SR_FS) != SR_FS_OFF {
        if riscv_isa_extension_available(isa, D) {
            __kvm_riscv_fp_d_restore(cntx);
        } else if riscv_isa_extension_available(isa, F) {
            __kvm_riscv_fp_f_restore(cntx);
        }
        kvm_riscv_vcpu_fp_clean(cntx);
    }
}

// CONFIG_FPU
pub unsafe fn kvm_riscv_vcpu_host_fp_save(cntx: *mut kvm_cpu_context) {
    /* No need to check host sstatus as it can be modified outside */
    if !kvm_riscv_isa_check_host(D) {
        __kvm_riscv_fp_d_save(cntx);
    } else if !kvm_riscv_isa_check_host(F) {
        __kvm_riscv_fp_f_save(cntx);
    }
}

// CONFIG_FPU
pub unsafe fn kvm_riscv_vcpu_host_fp_restore(cntx: *mut kvm_cpu_context) {
    if !kvm_riscv_isa_check_host(D) {
        __kvm_riscv_fp_d_restore(cntx);
    } else if !kvm_riscv_isa_check_host(F) {
        __kvm_riscv_fp_f_restore(cntx);
    }
}

pub unsafe fn kvm_riscv_vcpu_get_reg_fp(
    vcpu: *mut kvm_vcpu,
    reg: *const kvm_one_reg,
    rtype: c_ulong,
) -> c_int {
    let cntx: *mut kvm_cpu_context = &mut (*vcpu).arch.guest_context;
    let uaddr = (*reg).addr as *mut c_ulong;
    let mut reg_num = (*reg).id & !(KVM_REG_ARCH_MASK | KVM_REG_SIZE_MASK | rtype);
    let mut reg_val: *mut c_void;

    if (rtype == KVM_REG_RISCV_FP_F)
        && riscv_isa_extension_available((*vcpu).arch.isa, F)
    {
        if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<u32>() {
            return -EINVAL;
        }
        if reg_num == KVM_REG_RISCV_FP_F_REG!(fcsr) {
            reg_val = &mut (*cntx).fp.f.fcsr as *mut _ as *mut c_void;
        } else if (KVM_REG_RISCV_FP_F_REG!(f[0]) <= reg_num)
            && (reg_num <= KVM_REG_RISCV_FP_F_REG!(f[31]))
        {
            reg_num = array_index_nospec(reg_num, core::mem::size_of_val(&(*cntx).fp.f.f));
            reg_val = &mut (*cntx).fp.f.f[reg_num as usize] as *mut _ as *mut c_void;
        } else {
            return -ENOENT;
        }
    } else if (rtype == KVM_REG_RISCV_FP_D)
        && riscv_isa_extension_available((*vcpu).arch.isa, D)
    {
        if reg_num == KVM_REG_RISCV_FP_D_REG!(fcsr) {
            if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<u32>() {
                return -EINVAL;
            }
            reg_val = &mut (*cntx).fp.d.fcsr as *mut _ as *mut c_void;
        } else if (KVM_REG_RISCV_FP_D_REG!(f[0]) <= reg_num)
            && (reg_num <= KVM_REG_RISCV_FP_D_REG!(f[31]))
        {
            if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<u64>() {
                return -EINVAL;
            }
            reg_num = array_index_nospec(reg_num, core::mem::size_of_val(&(*cntx).fp.d.f));
            reg_val = &mut (*cntx).fp.d.f[reg_num as usize] as *mut _ as *mut c_void;
        } else {
            return -ENOENT;
        }
    } else {
        return -ENOENT;
    }

    if copy_to_user(uaddr, reg_val, KVM_REG_SIZE((*reg).id)) != 0 {
        return -EFAULT;
    }
    0
}

pub unsafe fn kvm_riscv_vcpu_set_reg_fp(
    vcpu: *mut kvm_vcpu,
    reg: *const kvm_one_reg,
    rtype: c_ulong,
) -> c_int {
    let cntx: *mut kvm_cpu_context = &mut (*vcpu).arch.guest_context;
    let uaddr = (*reg).addr as *const c_ulong;
    let mut reg_num = (*reg).id & !(KVM_REG_ARCH_MASK | KVM_REG_SIZE_MASK | rtype);
    let mut reg_val: *mut c_void;

    if (rtype == KVM_REG_RISCV_FP_F)
        && riscv_isa_extension_available((*vcpu).arch.isa, F)
    {
        if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<u32>() {
            return -EINVAL;
        }
        if reg_num == KVM_REG_RISCV_FP_F_REG!(fcsr) {
            reg_val = &mut (*cntx).fp.f.fcsr as *mut _ as *mut c_void;
        } else if (KVM_REG_RISCV_FP_F_REG!(f[0]) <= reg_num)
            && (reg_num <= KVM_REG_RISCV_FP_F_REG!(f[31]))
        {
            reg_num = array_index_nospec(reg_num, core::mem::size_of_val(&(*cntx).fp.f.f));
            reg_val = &mut (*cntx).fp.f.f[reg_num as usize] as *mut _ as *mut c_void;
        } else {
            return -ENOENT;
        }
    } else if (rtype == KVM_REG_RISCV_FP_D)
        && riscv_isa_extension_available((*vcpu).arch.isa, D)
    {
        if reg_num == KVM_REG_RISCV_FP_D_REG!(fcsr) {
            if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<u32>() {
                return -EINVAL;
            }
            reg_val = &mut (*cntx).fp.d.fcsr as *mut _ as *mut c_void;
        } else if (KVM_REG_RISCV_FP_D_REG!(f[0]) <= reg_num)
            && (reg_num <= KVM_REG_RISCV_FP_D_REG!(f[31]))
        {
            if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<u64>() {
                return -EINVAL;
            }
            reg_num = array_index_nospec(reg_num, core::mem::size_of_val(&(*cntx).fp.d.f));
            reg_val = &mut (*cntx).fp.d.f[reg_num as usize] as *mut _ as *mut c_void;
        } else {
            return -ENOENT;
        }
    } else {
        return -ENOENT;
    }

    if copy_from_user(reg_val, uaddr, KVM_REG_SIZE((*reg).id)) != 0 {
        return -EFAULT;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
