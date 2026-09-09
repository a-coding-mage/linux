// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Atish Patra <atish.patra@wdc.com>
 */

// Linux and architecture headers provide the constants and types referenced
// below; they are intentionally not reproduced here.

unsafe fn kvm_sbi_ext_base_handler(
    vcpu: *mut kvm_vcpu,
    run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> i32 {
    let cp: *mut kvm_cpu_context = &mut (*vcpu).arch.guest_context;
    let mut sbi_ext: *const kvm_vcpu_sbi_extension;
    let out_val: *mut libc::c_ulong = &mut (*retdata).out_val;

    match (*cp).a6 {
        SBI_EXT_BASE_GET_SPEC_VERSION => {
            *out_val = ((KVM_SBI_VERSION_MAJOR << SBI_SPEC_VERSION_MAJOR_SHIFT)
                | KVM_SBI_VERSION_MINOR) as libc::c_ulong;
        }
        SBI_EXT_BASE_GET_IMP_ID => {
            *out_val = KVM_SBI_IMPID as libc::c_ulong;
        }
        SBI_EXT_BASE_GET_IMP_VERSION => {
            *out_val = LINUX_VERSION_CODE as libc::c_ulong;
        }
        SBI_EXT_BASE_PROBE_EXT => {
            if ((*cp).a0 >= SBI_EXT_EXPERIMENTAL_START
                && (*cp).a0 <= SBI_EXT_EXPERIMENTAL_END)
                || ((*cp).a0 >= SBI_EXT_VENDOR_START && (*cp).a0 <= SBI_EXT_VENDOR_END)
            {
                /*
                 * For experimental/vendor extensions
                 * forward it to the userspace
                 */
                return kvm_riscv_vcpu_sbi_forward_handler(vcpu, run, retdata);
            } else {
                sbi_ext = kvm_vcpu_sbi_find_ext(vcpu, (*cp).a0);
                *out_val = if !sbi_ext.is_null() {
                    if let Some(probe) = (*sbi_ext).probe {
                        probe(vcpu) as libc::c_ulong
                    } else {
                        1
                    }
                } else {
                    0
                };
            }
        }
        SBI_EXT_BASE_GET_MVENDORID => {
            *out_val = (*vcpu).arch.mvendorid;
        }
        SBI_EXT_BASE_GET_MARCHID => {
            *out_val = (*vcpu).arch.marchid;
        }
        SBI_EXT_BASE_GET_MIMPID => {
            *out_val = (*vcpu).arch.mimpid;
        }
        _ => {
            (*retdata).err_val = SBI_ERR_NOT_SUPPORTED;
        }
    }

    0
}

const vcpu_sbi_ext_base: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_BASE,
    extid_end: SBI_EXT_BASE,
    handler: kvm_sbi_ext_base_handler,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
