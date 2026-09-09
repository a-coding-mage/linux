// SPDX-License-Identifier: GPL-2.0-only
/*
 * vMTRR implementation
 *
 * Copyright (C) 2006 Qumranet, Inc.
 * Copyright 2010 Red Hat, Inc. and/or its affiliates.
 * Copyright(C) 2015 Intel Corporation.
 *
 * Authors:
 *   Yaniv Kamay  <yaniv@qumranet.com>
 *   Avi Kivity   <avi@qumranet.com>
 *   Marcelo Tosatti <mtosatti@redhat.com>
 *   Paolo Bonzini <pbonzini@redhat.com>
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 */

// Dependencies supplied by the surrounding kernel/KVM translation.

unsafe fn find_mtrr(vcpu: *mut kvm_vcpu, msr: u32) -> *mut u64 {
    let mut index: usize;

    if msr >= MTRRphysBase_MSR(0) && msr <= MTRRphysMask_MSR(KVM_NR_VAR_MTRR - 1) {
        index = (msr - MTRRphysBase_MSR(0)) as usize;
        return &mut (*vcpu).arch.mtrr_state.var[index];
    }

    match msr {
        MSR_MTRRfix64K_00000 => &mut (*vcpu).arch.mtrr_state.fixed_64k,
        MSR_MTRRfix16K_80000 | MSR_MTRRfix16K_A0000 => {
            index = (msr - MSR_MTRRfix16K_80000) as usize;
            &mut (*vcpu).arch.mtrr_state.fixed_16k[index]
        }
        MSR_MTRRfix4K_C0000
        | MSR_MTRRfix4K_C8000
        | MSR_MTRRfix4K_D0000
        | MSR_MTRRfix4K_D8000
        | MSR_MTRRfix4K_E0000
        | MSR_MTRRfix4K_E8000
        | MSR_MTRRfix4K_F0000
        | MSR_MTRRfix4K_F8000 => {
            index = (msr - MSR_MTRRfix4K_C0000) as usize;
            &mut (*vcpu).arch.mtrr_state.fixed_4k[index]
        }
        MSR_MTRRdefType => &mut (*vcpu).arch.mtrr_state.deftype,
        _ => core::ptr::null_mut(),
    }
}

fn valid_mtrr_type(t: u32) -> bool {
    t < 8 && ((1u32 << t) & 0x73) != 0 // 0, 1, 4, 5, 6
}

unsafe fn kvm_mtrr_valid(vcpu: *mut kvm_vcpu, msr: u32, data: u64) -> bool {
    let mut i: u32;
    let mut mask: u64;

    if msr == MSR_MTRRdefType {
        if data & !0xcff != 0 {
            return false;
        }
        return valid_mtrr_type((data & 0xff) as u32);
    } else if msr >= MSR_MTRRfix64K_00000 && msr <= MSR_MTRRfix4K_F8000 {
        i = 0;
        while i < 8 {
            if !valid_mtrr_type(((data >> (i * 8)) & 0xff) as u32) {
                return false;
            }
            i += 1;
        }
        return true;
    }

    // variable MTRRs
    if !(msr >= MTRRphysBase_MSR(0) && msr <= MTRRphysMask_MSR(KVM_NR_VAR_MTRR - 1)) {
        // WARN_ON_ONCE(!(msr >= MTRRphysBase_MSR(0) &&
        //                 msr <= MTRRphysMask_MSR(KVM_NR_VAR_MTRR - 1)))
        return false;
    }

    mask = kvm_vcpu_reserved_gpa_bits_raw(vcpu);
    if (msr & 1 == 0) {
        // MTRR base
        if !valid_mtrr_type((data & 0xff) as u32) {
            return false;
        }
        mask |= 0xf00;
    } else {
        // MTRR mask
        mask |= 0x7ff;
    }

    (data & mask) == 0
}

pub unsafe fn kvm_mtrr_set_msr(vcpu: *mut kvm_vcpu, msr: u32, data: u64) -> i32 {
    let mtrr = find_mtrr(vcpu, msr);
    if mtrr.is_null() {
        return 1;
    }

    if !kvm_mtrr_valid(vcpu, msr, data) {
        return 1;
    }

    *mtrr = data;
    0
}

pub unsafe fn kvm_mtrr_get_msr(vcpu: *mut kvm_vcpu, msr: u32, pdata: *mut u64) -> i32 {
    // MSR_MTRRcap is a readonly MSR.
    if msr == MSR_MTRRcap {
        /*
         * SMRR = 0
         * WC = 1
         * FIX = 1
         * VCNT = KVM_NR_VAR_MTRR
         */
        *pdata = 0x500 | KVM_NR_VAR_MTRR;
        return 0;
    }

    let mtrr = find_mtrr(vcpu, msr);
    if mtrr.is_null() {
        return 1;
    }

    *pdata = *mtrr;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
