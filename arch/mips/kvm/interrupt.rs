/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * KVM/MIPS: Interrupt delivery
 *
 * Copyright (C) 2012  MIPS Technologies, Inc.  All rights reserved.
 * Authors: Sanjay Lal <sanjayl@kymasys.com>
 */

// The declarations supplied by the Linux and MIPS headers are dependencies of
// this translation.

extern "C" {
    static mut kvm_mips_callbacks: *mut KvmMipsCallbacks;
}

#[repr(C)]
pub struct KvmMipsCallbacks {
    pub irq_clear: unsafe extern "C" fn(*mut KvmVcpu, u32, u32),
    pub irq_deliver: unsafe extern "C" fn(*mut KvmVcpu, u32, u32),
}

#[repr(C)]
pub struct KvmVcpuArch {
    pub pending_exceptions: usize,
    pub pending_exceptions_clr: usize,
}

#[repr(C)]
pub struct KvmVcpu {
    pub arch: KvmVcpuArch,
}

// MIPS_EXC_MAX and MIPS_EXC_INT_TIMER are supplied by interrupt.h.
extern "C" {
    static MIPS_EXC_MAX: u32;
    static MIPS_EXC_INT_TIMER: u32;
}

pub unsafe extern "C" fn kvm_mips_deliver_interrupts(
    vcpu: *mut KvmVcpu,
    cause: u32,
) {
    let pending: *mut usize = &mut (*vcpu).arch.pending_exceptions;
    let pending_clr: *mut usize = &mut (*vcpu).arch.pending_exceptions_clr;
    let mut priority: u32;

    priority = 0;
    while priority <= MIPS_EXC_MAX {
        if (*pending_clr & (1usize << priority)) != 0 {
            ((*kvm_mips_callbacks).irq_clear)(vcpu, priority, cause);
        }
        priority = priority.wrapping_add(1);
    }

    priority = 0;
    while priority <= MIPS_EXC_MAX {
        if (*pending & (1usize << priority)) != 0 {
            ((*kvm_mips_callbacks).irq_deliver)(vcpu, priority, cause);
        }
        priority = priority.wrapping_add(1);
    }
}

pub unsafe extern "C" fn kvm_mips_pending_timer(vcpu: *mut KvmVcpu) -> i32 {
    if ((*vcpu).arch.pending_exceptions & (1usize << MIPS_EXC_INT_TIMER)) != 0 {
        1
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
