// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependencies supplied by the corresponding kernel headers:
// #include <hyp/debug-sr.h>
// #include <linux/kvm_host.h>
// #include <asm/kvm_hyp.h>

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn __debug_switch_to_guest_common(vcpu: *mut kvm_vcpu);
    fn __debug_switch_to_host_common(vcpu: *mut kvm_vcpu);
}

pub unsafe fn __debug_switch_to_guest(vcpu: *mut kvm_vcpu) {
    unsafe {
        __debug_switch_to_guest_common(vcpu);
    }
}

pub unsafe fn __debug_switch_to_host(vcpu: *mut kvm_vcpu) {
    unsafe {
        __debug_switch_to_host_common(vcpu);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
