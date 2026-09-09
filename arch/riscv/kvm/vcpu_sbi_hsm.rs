// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Atish Patra <atish.patra@wdc.com>
 */

// Dependencies supplied by the surrounding kernel/KVM translation unit are
// intentionally referenced here rather than redefined locally.

unsafe fn kvm_sbi_hsm_vcpu_start(vcpu: *mut kvm_vcpu) -> i32 {
    let cp = &mut (*vcpu).arch.guest_context;
    let target_vcpuid: usize = cp.a0;
    let mut ret: i32 = 0;

    let target_vcpu = kvm_get_vcpu_by_id((*vcpu).kvm, target_vcpuid);
    if target_vcpu.is_null() {
        return SBI_ERR_INVALID_PARAM;
    }

    spin_lock(&mut (*target_vcpu).arch.mp_state_lock);

    if !kvm_riscv_vcpu_stopped(target_vcpu) {
        ret = SBI_ERR_ALREADY_AVAILABLE;
        spin_unlock(&mut (*target_vcpu).arch.mp_state_lock);
        return ret;
    }

    kvm_riscv_vcpu_sbi_request_reset(target_vcpu, cp.a1, cp.a2);
    __kvm_riscv_vcpu_power_on(target_vcpu);

    spin_unlock(&mut (*target_vcpu).arch.mp_state_lock);
    ret
}

unsafe fn kvm_sbi_hsm_vcpu_stop(vcpu: *mut kvm_vcpu) -> i32 {
    let mut ret: i32 = 0;

    spin_lock(&mut (*vcpu).arch.mp_state_lock);

    if kvm_riscv_vcpu_stopped(vcpu) {
        ret = SBI_ERR_FAILURE;
        spin_unlock(&mut (*vcpu).arch.mp_state_lock);
        return ret;
    }

    __kvm_riscv_vcpu_power_off(vcpu);
    spin_unlock(&mut (*vcpu).arch.mp_state_lock);
    ret
}

unsafe fn kvm_sbi_hsm_vcpu_get_status(vcpu: *mut kvm_vcpu) -> i32 {
    let cp = &mut (*vcpu).arch.guest_context;
    let target_vcpuid: usize = cp.a0;
    let target_vcpu = kvm_get_vcpu_by_id((*vcpu).kvm, target_vcpuid);

    if target_vcpu.is_null() {
        return SBI_ERR_INVALID_PARAM;
    }
    if kvm_riscv_vcpu_stopped(target_vcpu) {
        SBI_HSM_STATE_STOPPED
    } else if (*target_vcpu).stat.generic.blocking {
        SBI_HSM_STATE_SUSPENDED
    } else {
        SBI_HSM_STATE_STARTED
    }
}

unsafe fn kvm_sbi_ext_hsm_handler(
    vcpu: *mut kvm_vcpu,
    _run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> i32 {
    let mut ret: i32 = 0;
    let cp = &mut (*vcpu).arch.guest_context;
    let funcid: usize = cp.a6;

    match funcid {
        SBI_EXT_HSM_HART_START => {
            ret = kvm_sbi_hsm_vcpu_start(vcpu);
        }
        SBI_EXT_HSM_HART_STOP => {
            ret = kvm_sbi_hsm_vcpu_stop(vcpu);
        }
        SBI_EXT_HSM_HART_STATUS => {
            ret = kvm_sbi_hsm_vcpu_get_status(vcpu);
            if ret >= 0 {
                (*retdata).out_val = ret;
                (*retdata).err_val = 0;
            }
            return 0;
        }
        SBI_EXT_HSM_HART_SUSPEND => match lower_32_bits(cp.a0) {
            SBI_HSM_SUSPEND_RET_DEFAULT => {
                kvm_riscv_vcpu_wfi(vcpu);
            }
            SBI_HSM_SUSPEND_NON_RET_DEFAULT => {
                ret = SBI_ERR_NOT_SUPPORTED;
            }
            _ => {
                ret = SBI_ERR_INVALID_PARAM;
            }
        },
        _ => {
            ret = SBI_ERR_NOT_SUPPORTED;
        }
    }

    (*retdata).err_val = ret;
    0
}

const vcpu_sbi_ext_hsm: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_HSM,
    extid_end: SBI_EXT_HSM,
    handler: kvm_sbi_ext_hsm_handler,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
