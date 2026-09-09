/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * KVM/MIPS: Interrupts
 * Copyright (C) 2012  MIPS Technologies, Inc.  All rights reserved.
 * Authors: Sanjay Lal <sanjayl@kymasys.com>
 */

/*
 * MIPS Exception Priorities, exceptions (including interrupts) are queued up
 * for the guest in the order specified by their priorities
 */

pub const MIPS_EXC_RESET: u32 = 0;
pub const MIPS_EXC_SRESET: u32 = 1;
pub const MIPS_EXC_DEBUG_ST: u32 = 2;
pub const MIPS_EXC_DEBUG: u32 = 3;
pub const MIPS_EXC_DDB: u32 = 4;
pub const MIPS_EXC_NMI: u32 = 5;
pub const MIPS_EXC_MCHK: u32 = 6;
pub const MIPS_EXC_INT_TIMER: u32 = 7;
pub const MIPS_EXC_INT_IO_1: u32 = 8;
pub const MIPS_EXC_INT_IO_2: u32 = 9;
pub const MIPS_EXC_EXECUTE: u32 = 10;
pub const MIPS_EXC_INT_IPI_1: u32 = 11;
pub const MIPS_EXC_INT_IPI_2: u32 = 12;
pub const MIPS_EXC_MAX: u32 = 13;
/* XXXSL More to follow */

pub const C_TI: u64 = 1u64 << 30;

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm {
    _private: [u8; 0],
}

extern "C" {
    pub static mut kvm_priority_to_irq: *mut u32;

    pub fn kvm_irq_to_priority(irq: u32) -> u32;

    pub fn kvm_mips_pending_timer(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_int;

    pub fn kvm_mips_deliver_interrupts(vcpu: *mut kvm_vcpu, cause: u32);

    #[cfg(CONFIG_CPU_LOONGSON64)]
    pub fn kvm_init_loongson_ipi(kvm: *mut kvm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
