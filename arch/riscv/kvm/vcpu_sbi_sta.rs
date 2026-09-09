// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023 Ventana Micro Systems Inc.
 */

// Kernel headers and symbols are supplied by the surrounding Rust kernel environment.

unsafe fn kvm_riscv_vcpu_sbi_sta_reset(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.sta.shmem = INVALID_GPA;
    (*vcpu).arch.sta.last_steal = 0;
}

pub unsafe fn kvm_riscv_vcpu_record_steal_time(vcpu: *mut kvm_vcpu) {
    let shmem: gpa_t = (*vcpu).arch.sta.shmem;
    let last_steal: u64 = (*vcpu).arch.sta.last_steal;
    let mut sequence_le: __le32 = 0;
    let mut steal_le: __le64 = 0;
    let mut sequence: u32;
    let mut steal: u64;
    let hva: usize;
    let gfn: gfn_t;

    if shmem == INVALID_GPA {
        return;
    }

    /*
     * shmem is 64-byte aligned (see the enforcement in
     * kvm_sbi_sta_steal_time_set_shmem()) and the size of sbi_sta_struct
     * is 64 bytes, so we know all its offsets are in the same page.
     */
    gfn = shmem >> PAGE_SHIFT;
    hva = kvm_vcpu_gfn_to_hva(vcpu, gfn);

    if kvm_is_error_hva(hva) {
        (*vcpu).arch.sta.shmem = INVALID_GPA;
        return;
    }

    let sequence_ptr = (hva + offset_in_page(shmem) +
        core::mem::offset_of!(sbi_sta_struct, sequence)) as *mut __le32;
    let steal_ptr = (hva + offset_in_page(shmem) +
        core::mem::offset_of!(sbi_sta_struct, steal)) as *mut __le64;

    if WARN_ON(get_user(&mut sequence_le, sequence_ptr)) {
        return;
    }

    sequence = le32_to_cpu(sequence_le);
    sequence = sequence.wrapping_add(1);

    if WARN_ON(put_user(cpu_to_le32(sequence), sequence_ptr)) {
        return;
    }

    if !WARN_ON(get_user(&mut steal_le, steal_ptr)) {
        steal = le64_to_cpu(steal_le);
        (*vcpu).arch.sta.last_steal = READ_ONCE((*current).sched_info.run_delay);
        steal = steal.wrapping_add((*vcpu).arch.sta.last_steal.wrapping_sub(last_steal));
        WARN_ON(put_user(cpu_to_le64(steal), steal_ptr));
    }

    sequence = sequence.wrapping_add(1);
    WARN_ON(put_user(cpu_to_le32(sequence), sequence_ptr));

    kvm_vcpu_mark_page_dirty(vcpu, gfn);
}

unsafe fn kvm_sbi_sta_steal_time_set_shmem(vcpu: *mut kvm_vcpu) -> c_int {
    let cp: *mut kvm_cpu_context = &mut (*vcpu).arch.guest_context;
    let shmem_phys_lo: usize = (*cp).a0;
    let shmem_phys_hi: usize = (*cp).a1;
    let flags: u32 = (*cp).a2;
    let zero_sta: sbi_sta_struct = core::mem::zeroed();
    let mut shmem: gpa_t;
    let ret: c_int;

    if flags != 0 {
        return SBI_ERR_INVALID_PARAM;
    }

    if shmem_phys_lo == SBI_SHMEM_DISABLE && shmem_phys_hi == SBI_SHMEM_DISABLE {
        (*vcpu).arch.sta.shmem = INVALID_GPA;
        return 0;
    }

    if shmem_phys_lo & (SZ_64 - 1) != 0 {
        return SBI_ERR_INVALID_PARAM;
    }

    shmem = shmem_phys_lo as gpa_t;

    if shmem_phys_hi != 0 {
        if IS_ENABLED(CONFIG_32BIT) {
            shmem |= (shmem_phys_hi as gpa_t) << 32;
        } else {
            return SBI_ERR_INVALID_ADDRESS;
        }
    }

    /* No need to check writable slot explicitly as kvm_vcpu_write_guest does it internally */
    ret = kvm_vcpu_write_guest(vcpu, shmem, &zero_sta as *const sbi_sta_struct as *const u8,
        core::mem::size_of::<sbi_sta_struct>());
    if ret != 0 {
        return SBI_ERR_INVALID_ADDRESS;
    }

    (*vcpu).arch.sta.shmem = shmem;
    (*vcpu).arch.sta.last_steal = (*current).sched_info.run_delay;

    0
}

unsafe fn kvm_sbi_ext_sta_handler(
    vcpu: *mut kvm_vcpu,
    _run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> c_int {
    let cp: *mut kvm_cpu_context = &mut (*vcpu).arch.guest_context;
    let funcid: usize = (*cp).a6;
    let ret: c_int;

    match funcid {
        SBI_EXT_STA_STEAL_TIME_SET_SHMEM => {
            ret = kvm_sbi_sta_steal_time_set_shmem(vcpu);
        }
        _ => {
            ret = SBI_ERR_NOT_SUPPORTED;
        }
    }

    (*retdata).err_val = ret;

    0
}

unsafe fn kvm_sbi_ext_sta_probe(_vcpu: *mut kvm_vcpu) -> c_ulong {
    !!sched_info_on()
}

unsafe fn kvm_sbi_ext_sta_get_state_reg_count(_vcpu: *mut kvm_vcpu) -> c_ulong {
    (core::mem::size_of::<kvm_riscv_sbi_sta>() / core::mem::size_of::<c_ulong>()) as c_ulong
}

unsafe fn kvm_sbi_ext_sta_get_reg(
    vcpu: *mut kvm_vcpu,
    reg_num: c_ulong,
    reg_size: c_ulong,
    reg_val: *mut core::ffi::c_void,
) -> c_int {
    if reg_size != core::mem::size_of::<c_ulong>() as c_ulong {
        return -EINVAL;
    }
    let value = reg_val as *mut c_ulong;

    match reg_num {
        KVM_REG_RISCV_SBI_STA_REG_shmem_lo => {
            *value = (*vcpu).arch.sta.shmem as c_ulong;
        }
        KVM_REG_RISCV_SBI_STA_REG_shmem_hi => {
            if IS_ENABLED(CONFIG_32BIT) {
                *value = upper_32_bits((*vcpu).arch.sta.shmem) as c_ulong;
            } else {
                *value = 0;
            }
        }
        _ => return -ENOENT,
    }

    0
}

unsafe fn kvm_sbi_ext_sta_set_reg(
    vcpu: *mut kvm_vcpu,
    reg_num: c_ulong,
    reg_size: c_ulong,
    reg_val: *const core::ffi::c_void,
) -> c_int {
    let value: c_ulong;
    let mut new_shmem: gpa_t = INVALID_GPA;

    if reg_size != core::mem::size_of::<c_ulong>() as c_ulong {
        return -EINVAL;
    }
    value = *(reg_val as *const c_ulong);

    match reg_num {
        KVM_REG_RISCV_SBI_STA_REG_shmem_lo => {
            if IS_ENABLED(CONFIG_32BIT) {
                let hi: gpa_t = upper_32_bits((*vcpu).arch.sta.shmem);
                new_shmem = value as gpa_t;
                new_shmem |= hi << 32;
            } else {
                new_shmem = value as gpa_t;
            }
        }
        KVM_REG_RISCV_SBI_STA_REG_shmem_hi => {
            if IS_ENABLED(CONFIG_32BIT) {
                let lo: gpa_t = lower_32_bits((*vcpu).arch.sta.shmem);
                new_shmem = (value as gpa_t) << 32;
                new_shmem |= lo;
            } else if value != 0 {
                return -EINVAL;
            }
        }
        _ => return -ENOENT,
    }

    if new_shmem != INVALID_GPA && !IS_ALIGNED(new_shmem, 64) {
        return -EINVAL;
    }

    (*vcpu).arch.sta.shmem = new_shmem;

    0
}

#[no_mangle]
pub static mut vcpu_sbi_ext_sta: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_STA,
    extid_end: SBI_EXT_STA,
    handler: Some(kvm_sbi_ext_sta_handler),
    probe: Some(kvm_sbi_ext_sta_probe),
    reset: Some(kvm_riscv_vcpu_sbi_sta_reset),
    state_reg_subtype: KVM_REG_RISCV_SBI_STA,
    get_state_reg_count: Some(kvm_sbi_ext_sta_get_state_reg_count),
    get_state_reg: Some(kvm_sbi_ext_sta_get_reg),
    set_state_reg: Some(kvm_sbi_ext_sta_set_reg),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
