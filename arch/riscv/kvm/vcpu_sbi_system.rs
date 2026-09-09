// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2024 Ventana Micro Systems Inc.
 */

// Dependencies supplied by the surrounding kernel/KVM and RISC-V interfaces
// are intentionally left external to this translation unit.

unsafe fn kvm_sbi_ext_susp_handler(
    vcpu: *mut kvm_vcpu,
    run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> i32 {
    let cp: *mut kvm_cpu_context = unsafe { &mut (*vcpu).arch.guest_context };
    let funcid: libc::c_ulong = unsafe { (*cp).a6 };
    let mut hva: libc::c_ulong;
    let mut i: libc::c_ulong;
    let mut tmp: *mut kvm_vcpu;

    match funcid {
        SBI_EXT_SUSP_SYSTEM_SUSPEND => {
            if lower_32_bits(unsafe { (*cp).a0 }) != SBI_SUSP_SLEEP_TYPE_SUSPEND_TO_RAM {
                unsafe { (*retdata).err_val = SBI_ERR_INVALID_PARAM };
                return 0;
            }

            if (unsafe { (*cp).sstatus } & SR_SPP) == 0 {
                unsafe { (*retdata).err_val = SBI_ERR_FAILURE };
                return 0;
            }

            hva = kvm_vcpu_gfn_to_hva_prot(
                vcpu,
                unsafe { (*cp).a1 } >> PAGE_SHIFT,
                core::ptr::null_mut(),
            );
            if kvm_is_error_hva(hva) {
                unsafe { (*retdata).err_val = SBI_ERR_INVALID_ADDRESS };
                return 0;
            }

            /*
             * Check that all other vCPUs are stopped before entering
             * system suspend.
             *
             * There is a known TOCTOU race here: a concurrent HSM
             * HART_START on another vCPU can start a vCPU after it
             * has already passed this check, violating the invariant.
             *
             * We do not fix this because:
             * 1. Triggering the race requires a pathological guest.
             * 2. Only guest state is at risk, not host integrity.
             * 3. Userspace can double-check vCPU states before
             *    proceeding with suspend.
             */
            kvm_for_each_vcpu!(i, tmp, unsafe { (*vcpu).kvm }, {
                if tmp == vcpu {
                    continue;
                }
                if !kvm_riscv_vcpu_stopped(tmp) {
                    unsafe { (*retdata).err_val = SBI_ERR_DENIED };
                    return 0;
                }
            });

            kvm_riscv_vcpu_sbi_request_reset(
                vcpu,
                unsafe { (*cp).a1 },
                unsafe { (*cp).a2 },
            );

            /* userspace provides the suspend implementation */
            return kvm_riscv_vcpu_sbi_forward_handler(vcpu, run, retdata);
        }
        _ => {
            unsafe { (*retdata).err_val = SBI_ERR_NOT_SUPPORTED };
        }
    }

    0
}

const vcpu_sbi_ext_susp: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_SUSP,
    extid_end: SBI_EXT_SUSP,
    default_disabled: true,
    handler: kvm_sbi_ext_susp_handler,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
