// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023 Rivos Inc
 *
 * Authors:
 *     Atish Patra <atishp@rivosinc.com>
 */

// Linux and architecture dependencies are supplied by the surrounding kernel
// translation unit.

unsafe fn kvm_sbi_ext_pmu_handler(
    vcpu: *mut kvm_vcpu,
    _run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> i32 {
    let mut ret: i32 = 0;
    let cp: *mut kvm_cpu_context = &mut (*vcpu).arch.guest_context;
    let kvpmu: *mut kvm_pmu = vcpu_to_pmu(vcpu);
    let funcid: ::core::ffi::c_ulong = (*cp).a6;
    let mut temp: u64;

    if !(*kvpmu).init_done {
        (*retdata).err_val = SBI_ERR_NOT_SUPPORTED;
        return 0;
    }

    match funcid {
        SBI_EXT_PMU_NUM_COUNTERS => {
            ret = kvm_riscv_vcpu_pmu_num_ctrs(vcpu, retdata);
        }
        SBI_EXT_PMU_COUNTER_GET_INFO => {
            ret = kvm_riscv_vcpu_pmu_ctr_info(vcpu, (*cp).a0, retdata);
        }
        SBI_EXT_PMU_COUNTER_CFG_MATCH => {
            // CONFIG_32BIT is a build-time kernel configuration condition.
            temp = ((*cp).a4 as u64);
            ret = kvm_riscv_vcpu_pmu_ctr_cfg_match(
                vcpu, (*cp).a0, (*cp).a1, (*cp).a2, (*cp).a3, temp, retdata,
            );
        }
        SBI_EXT_PMU_COUNTER_START => {
            // CONFIG_32BIT is a build-time kernel configuration condition.
            temp = ((*cp).a3 as u64);
            ret = kvm_riscv_vcpu_pmu_ctr_start(
                vcpu, (*cp).a0, (*cp).a1, (*cp).a2, temp, retdata,
            );
        }
        SBI_EXT_PMU_COUNTER_STOP => {
            ret = kvm_riscv_vcpu_pmu_ctr_stop(
                vcpu, (*cp).a0, (*cp).a1, (*cp).a2, retdata,
            );
        }
        SBI_EXT_PMU_COUNTER_FW_READ => {
            ret = kvm_riscv_vcpu_pmu_fw_ctr_read(vcpu, (*cp).a0, retdata);
        }
        SBI_EXT_PMU_COUNTER_FW_READ_HI => {
            // CONFIG_32BIT is a build-time kernel configuration condition.
            (*retdata).out_val = 0;
        }
        SBI_EXT_PMU_SNAPSHOT_SET_SHMEM => {
            ret = kvm_riscv_vcpu_pmu_snapshot_set_shmem(
                vcpu, (*cp).a0, (*cp).a1, (*cp).a2, retdata,
            );
        }
        SBI_EXT_PMU_EVENT_GET_INFO => {
            ret = kvm_riscv_vcpu_pmu_event_info(
                vcpu, (*cp).a0, (*cp).a1, (*cp).a2, (*cp).a3, retdata,
            );
        }
        _ => {
            (*retdata).err_val = SBI_ERR_NOT_SUPPORTED;
        }
    }

    ret
}

unsafe fn kvm_sbi_ext_pmu_probe(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_ulong {
    let kvpmu: *mut kvm_pmu = vcpu_to_pmu(vcpu);

    (*kvpmu).init_done
}

const VCPU_SBI_EXT_PMU: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_PMU,
    extid_end: SBI_EXT_PMU,
    handler: Some(kvm_sbi_ext_pmu_handler),
    probe: Some(kvm_sbi_ext_pmu_probe),
};

#[no_mangle]
pub static vcpu_sbi_ext_pmu: kvm_vcpu_sbi_extension = VCPU_SBI_EXT_PMU;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
