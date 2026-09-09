// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 SiFive
 *
 * Authors:
 *     Vincent Chen <vincent.chen@sifive.com>
 *     Greentime Hu <greentime.hu@sifive.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_RISCV_ISA_V)]
pub unsafe fn kvm_riscv_vcpu_vector_reset(vcpu: *mut kvm_vcpu) {
    let isa = (*vcpu).arch.isa;
    let cntx = &mut (*vcpu).arch.guest_context;

    cntx.sstatus &= !SR_VS;
    cntx.vector.vlenb = riscv_v_vsize / 32;

    if riscv_isa_extension_available(isa, V) {
        cntx.sstatus |= SR_VS_INITIAL;
        WARN_ON(cntx.vector.datap.is_null());
        memset(cntx.vector.datap, 0, riscv_v_vsize);
    } else {
        cntx.sstatus |= SR_VS_OFF;
    }
}

#[cfg(CONFIG_RISCV_ISA_V)]
unsafe fn kvm_riscv_vcpu_vector_clean(cntx: *mut kvm_cpu_context) {
    (*cntx).sstatus &= !SR_VS;
    (*cntx).sstatus |= SR_VS_CLEAN;
}

#[cfg(CONFIG_RISCV_ISA_V)]
pub unsafe fn kvm_riscv_vcpu_guest_vector_save(
    cntx: *mut kvm_cpu_context,
    isa: *mut c_ulong,
) {
    if ((*cntx).sstatus & SR_VS) == SR_VS_DIRTY {
        if riscv_isa_extension_available(isa, V) {
            __kvm_riscv_vector_save(cntx);
        }
        kvm_riscv_vcpu_vector_clean(cntx);
    }
}

#[cfg(CONFIG_RISCV_ISA_V)]
pub unsafe fn kvm_riscv_vcpu_guest_vector_restore(
    cntx: *mut kvm_cpu_context,
    isa: *mut c_ulong,
) {
    if ((*cntx).sstatus & SR_VS) != SR_VS_OFF {
        if riscv_isa_extension_available(isa, V) {
            riscv_v_flags_set(riscv_v_flags() | RISCV_V_VCPU_NEED_RESTORE);
        }
    }
}

#[cfg(CONFIG_RISCV_ISA_V)]
pub unsafe fn kvm_riscv_vcpu_host_vector_save(cntx: *mut kvm_cpu_context) {
    // No need to check host sstatus as it can be modified outside
    if !kvm_riscv_isa_check_host(V) {
        __kvm_riscv_vector_save(cntx);
    }
}

#[cfg(CONFIG_RISCV_ISA_V)]
pub unsafe fn kvm_riscv_vcpu_host_vector_restore(cntx: *mut kvm_cpu_context) {
    if !kvm_riscv_isa_check_host(V) {
        __kvm_riscv_vector_restore(cntx);
    }
    riscv_v_flags_set(riscv_v_flags() & !(RISCV_V_VCPU_CTX | RISCV_V_VCPU_NEED_RESTORE));
}

#[cfg(CONFIG_RISCV_ISA_V)]
pub unsafe fn kvm_riscv_vcpu_alloc_vector_context(vcpu: *mut kvm_vcpu) -> c_int {
    (*vcpu).arch.guest_context.vector.datap = kzalloc(riscv_v_vsize, GFP_KERNEL_ACCOUNT);
    if (*vcpu).arch.guest_context.vector.datap.is_null() {
        return -ENOMEM;
    }

    (*vcpu).arch.host_context.vector.datap = kzalloc(riscv_v_vsize, GFP_KERNEL_ACCOUNT);
    if (*vcpu).arch.host_context.vector.datap.is_null() {
        kfree((*vcpu).arch.guest_context.vector.datap);
        (*vcpu).arch.guest_context.vector.datap = core::ptr::null_mut();
        return -ENOMEM;
    }

    0
}

#[cfg(CONFIG_RISCV_ISA_V)]
pub unsafe fn kvm_riscv_vcpu_free_vector_context(vcpu: *mut kvm_vcpu) {
    kfree((*vcpu).arch.guest_context.vector.datap);
    kfree((*vcpu).arch.host_context.vector.datap);
}

#[cfg(CONFIG_RISCV_ISA_V)]
pub unsafe fn kvm_riscv_vcpu_flush_vector() {
    let vcpu = *this_cpu_ptr(kvm_get_running_vcpus());
    // Only reached from __riscv_flush_vector_context() when RISCV_V_VCPU_CTX is set, which
    // always have kvm_get_running_vcpus non-NULL.
    if WARN_ON_ONCE(vcpu.is_null()) {
        return;
    }

    kvm_riscv_vcpu_guest_vector_save(&mut (*vcpu).arch.guest_context, (*vcpu).arch.isa);
    if ((*vcpu).arch.guest_context.sstatus & SR_VS) != SR_VS_OFF {
        riscv_v_flags_set(riscv_v_flags() | RISCV_V_VCPU_NEED_RESTORE);
    }
}

unsafe fn kvm_riscv_vcpu_vreg_addr(
    vcpu: *mut kvm_vcpu,
    reg_num: c_ulong,
    reg_size: usize,
    reg_addr: *mut *mut c_void,
) -> c_int {
    let cntx = &mut (*vcpu).arch.guest_context;
    let vlenb = riscv_v_vsize / 32;

    if reg_num < KVM_REG_RISCV_VECTOR_REG(0) {
        if reg_size != core::mem::size_of::<c_ulong>() { return -EINVAL; }
        match reg_num {
            KVM_REG_RISCV_VECTOR_CSR_REG(vstart) => *reg_addr = &mut cntx.vector.vstart as *mut _ as *mut c_void,
            KVM_REG_RISCV_VECTOR_CSR_REG(vl) => *reg_addr = &mut cntx.vector.vl as *mut _ as *mut c_void,
            KVM_REG_RISCV_VECTOR_CSR_REG(vtype) => *reg_addr = &mut cntx.vector.vtype as *mut _ as *mut c_void,
            KVM_REG_RISCV_VECTOR_CSR_REG(vcsr) => *reg_addr = &mut cntx.vector.vcsr as *mut _ as *mut c_void,
            KVM_REG_RISCV_VECTOR_CSR_REG(vlenb) => *reg_addr = &mut cntx.vector.vlenb as *mut _ as *mut c_void,
            KVM_REG_RISCV_VECTOR_CSR_REG(datap) => return -ENOENT,
            _ => return -ENOENT,
        }
    } else if reg_num <= KVM_REG_RISCV_VECTOR_REG(31) {
        if reg_size != vlenb { return -EINVAL; }
        WARN_ON(cntx.vector.datap.is_null());
        // Sanitize the userspace-derived register index against speculative out-of-bounds access.
        let reg_offset = array_index_nospec(reg_num - KVM_REG_RISCV_VECTOR_REG(0), 32);
        *reg_addr = cntx.vector.datap.add(reg_offset * vlenb) as *mut c_void;
    } else {
        return -ENOENT;
    }
    0
}

pub unsafe fn kvm_riscv_vcpu_get_reg_vector(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int {
    let isa = (*vcpu).arch.isa;
    let uaddr = (*reg).addr as c_ulong as *mut c_ulong;
    let reg_num = (*reg).id & !(KVM_REG_ARCH_MASK | KVM_REG_SIZE_MASK | KVM_REG_RISCV_VECTOR);
    let reg_size = KVM_REG_SIZE((*reg).id);
    let mut reg_addr: *mut c_void = core::ptr::null_mut();
    if !riscv_isa_extension_available(isa, V) { return -ENOENT; }
    let rc = kvm_riscv_vcpu_vreg_addr(vcpu, reg_num, reg_size, &mut reg_addr);
    if rc != 0 { return rc; }
    if copy_to_user(uaddr as *mut c_void, reg_addr, reg_size) != 0 { return -EFAULT; }
    0
}

pub unsafe fn kvm_riscv_vcpu_set_reg_vector(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int {
    let isa = (*vcpu).arch.isa;
    let uaddr = (*reg).addr as c_ulong as *mut c_ulong;
    let reg_num = (*reg).id & !(KVM_REG_ARCH_MASK | KVM_REG_SIZE_MASK | KVM_REG_RISCV_VECTOR);
    let reg_size = KVM_REG_SIZE((*reg).id);
    let mut reg_addr: *mut c_void = core::ptr::null_mut();
    if !riscv_isa_extension_available(isa, V) { return -ENOENT; }

    if reg_num == KVM_REG_RISCV_VECTOR_CSR_REG(vlenb) {
        let cntx = &mut (*vcpu).arch.guest_context;
        let mut reg_val: c_ulong = 0;
        if reg_size != core::mem::size_of_val(&reg_val) { return -EINVAL; }
        if copy_from_user(&mut reg_val as *mut _ as *mut c_void, uaddr as *mut c_void, reg_size) != 0 { return -EFAULT; }
        if reg_val != cntx.vector.vlenb { return -EINVAL; }
        return 0;
    }

    let rc = kvm_riscv_vcpu_vreg_addr(vcpu, reg_num, reg_size, &mut reg_addr);
    if rc != 0 { return rc; }
    if copy_from_user(reg_addr, uaddr as *mut c_void, reg_size) != 0 { return -EFAULT; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
