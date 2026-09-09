// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Atish Patra <atish.patra@wdc.com>
 */

unsafe fn kvm_sbi_ext_time_handler(
    vcpu: *mut kvm_vcpu,
    _run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> i32 {
    let cp = &mut (*vcpu).arch.guest_context;
    let next_cycle: u64;

    if cp.a6 != SBI_EXT_TIME_SET_TIMER {
        (*retdata).err_val = SBI_ERR_NOT_SUPPORTED;
        return 0;
    }

    kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_SET_TIMER);
    #[cfg(target_arch = "riscv32")]
    {
        next_cycle = ((cp.a1 as u64) << 32) | (cp.a0 as u64);
    }
    #[cfg(not(target_arch = "riscv32"))]
    {
        next_cycle = cp.a0 as u64;
    }
    kvm_riscv_vcpu_timer_next_event(vcpu, next_cycle);

    0
}

pub static vcpu_sbi_ext_time: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_TIME,
    extid_end: SBI_EXT_TIME,
    handler: Some(kvm_sbi_ext_time_handler),
};

unsafe fn kvm_sbi_ext_ipi_handler(
    vcpu: *mut kvm_vcpu,
    _run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> i32 {
    let mut ret: i32 = 0;
    let mut i: usize;
    let mut tmp: *mut kvm_vcpu;
    let cp = &mut (*vcpu).arch.guest_context;
    let hmask = cp.a0 as usize;
    let hbase = cp.a1 as usize;
    let mut hart_bit: usize = 0;
    let mut sentmask: usize = 0;

    if cp.a6 != SBI_EXT_IPI_SEND_IPI {
        (*retdata).err_val = SBI_ERR_NOT_SUPPORTED;
        return 0;
    }

    kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_IPI_SENT);
    // Translation of the kernel kvm_for_each_vcpu macro; its supplied Rust
    // dependency provides the iteration primitive and loop variables.
    kvm_for_each_vcpu!(i, tmp, (*vcpu).kvm, {
        if hbase != usize::MAX {
            if (*tmp).vcpu_id < hbase {
                continue;
            }
            hart_bit = (*tmp).vcpu_id - hbase;
            if hart_bit >= __riscv_xlen as usize {
                break;
            }
            if (hmask & (1usize << hart_bit)) == 0 {
                continue;
            }
        }
        ret = kvm_riscv_vcpu_set_interrupt(tmp, IRQ_VS_SOFT);
        if ret < 0 {
            break;
        }
        sentmask |= 1usize << hart_bit;
        kvm_riscv_vcpu_pmu_incr_fw(tmp, SBI_PMU_FW_IPI_RCVD);
    });

    if hbase != usize::MAX && (hmask ^ sentmask) != 0 {
        (*retdata).err_val = SBI_ERR_INVALID_PARAM;
    }

    ret
}

pub static vcpu_sbi_ext_ipi: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_IPI,
    extid_end: SBI_EXT_IPI,
    handler: Some(kvm_sbi_ext_ipi_handler),
};

unsafe fn kvm_sbi_ext_rfence_handler(
    vcpu: *mut kvm_vcpu,
    _run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> i32 {
    let cp = &mut (*vcpu).arch.guest_context;
    let hmask = cp.a0 as usize;
    let hbase = cp.a1 as usize;
    let funcid = cp.a6 as usize;
    let vmid: _;

    match funcid {
        SBI_EXT_RFENCE_REMOTE_FENCE_I => {
            kvm_riscv_fence_i((*vcpu).kvm, hbase, hmask);
            kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_FENCE_I_SENT);
        }
        SBI_EXT_RFENCE_REMOTE_SFENCE_VMA => {
            vmid = READ_ONCE!((*vcpu).kvm.arch.vmid.vmid);
            if (cp.a2 == 0 && cp.a3 == 0) || cp.a3 == usize::MAX {
                kvm_riscv_hfence_vvma_all((*vcpu).kvm, hbase, hmask, vmid);
            } else {
                kvm_riscv_hfence_vvma_gva((*vcpu).kvm, hbase, hmask, cp.a2, cp.a3, PAGE_SHIFT, vmid);
            }
            kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_HFENCE_VVMA_SENT);
        }
        SBI_EXT_RFENCE_REMOTE_SFENCE_VMA_ASID => {
            vmid = READ_ONCE!((*vcpu).kvm.arch.vmid.vmid);
            if (cp.a2 == 0 && cp.a3 == 0 || cp.a3 == usize::MAX) {
                kvm_riscv_hfence_vvma_asid_all((*vcpu).kvm, hbase, hmask, cp.a4, vmid);
            } else {
                kvm_riscv_hfence_vvma_asid_gva((*vcpu).kvm, hbase, hmask, cp.a2, cp.a3, PAGE_SHIFT, cp.a4, vmid);
            }
            kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_HFENCE_VVMA_ASID_SENT);
        }
        SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA
        | SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA_VMID
        | SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA
        | SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA_ASID => {
            // Until nested virtualization is implemented, SBI HFENCE calls are unsupported.
            (*retdata).err_val = SBI_ERR_NOT_SUPPORTED;
        }
        _ => {
            (*retdata).err_val = SBI_ERR_NOT_SUPPORTED;
        }
    }
    0
}

pub static vcpu_sbi_ext_rfence: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_RFENCE,
    extid_end: SBI_EXT_RFENCE,
    handler: Some(kvm_sbi_ext_rfence_handler),
};

unsafe fn kvm_sbi_ext_srst_handler(
    vcpu: *mut kvm_vcpu,
    run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> i32 {
    let cp = &mut (*vcpu).arch.guest_context;
    let funcid = cp.a6 as usize;
    let reason = cp.a1 as u32;
    let reset_type = cp.a0 as u32;

    match funcid {
        SBI_EXT_SRST_RESET => match reset_type {
            SBI_SRST_RESET_TYPE_SHUTDOWN => {
                kvm_riscv_vcpu_sbi_system_reset(vcpu, run, KVM_SYSTEM_EVENT_SHUTDOWN, reason);
                (*retdata).uexit = true;
            }
            SBI_SRST_RESET_TYPE_COLD_REBOOT | SBI_SRST_RESET_TYPE_WARM_REBOOT => {
                kvm_riscv_vcpu_sbi_system_reset(vcpu, run, KVM_SYSTEM_EVENT_RESET, reason);
                (*retdata).uexit = true;
            }
            _ => (*retdata).err_val = SBI_ERR_NOT_SUPPORTED,
        },
        _ => (*retdata).err_val = SBI_ERR_NOT_SUPPORTED,
    }
    0
}

pub static vcpu_sbi_ext_srst: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_SRST,
    extid_end: SBI_EXT_SRST,
    handler: Some(kvm_sbi_ext_srst_handler),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
