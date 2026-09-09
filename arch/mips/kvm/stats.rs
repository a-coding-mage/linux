/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * KVM/MIPS: COP0 access histogram
 *
 * Copyright (C) 2012  MIPS Technologies, Inc.  All rights reserved.
 * Authors: Sanjay Lal <sanjayl@kymasys.com>
 */

use core::ffi::{c_char, c_int, c_ulong};

/* Supplied by the KVM/MIPS headers. */
extern "C" {
    static mut N_MIPS_COPROC_REGS: c_int;
    static mut N_MIPS_COPROC_SEL: c_int;
    fn kvm_info(fmt: *const c_char, ...);
}

#[no_mangle]
pub static mut kvm_cop0_str: [*const c_char; 32] = [
    b"Index\0".as_ptr() as *const c_char,
    b"Random\0".as_ptr() as *const c_char,
    b"EntryLo0\0".as_ptr() as *const c_char,
    b"EntryLo1\0".as_ptr() as *const c_char,
    b"Context\0".as_ptr() as *const c_char,
    b"PG Mask\0".as_ptr() as *const c_char,
    b"Wired\0".as_ptr() as *const c_char,
    b"HWREna\0".as_ptr() as *const c_char,
    b"BadVAddr\0".as_ptr() as *const c_char,
    b"Count\0".as_ptr() as *const c_char,
    b"EntryHI\0".as_ptr() as *const c_char,
    b"Compare\0".as_ptr() as *const c_char,
    b"Status\0".as_ptr() as *const c_char,
    b"Cause\0".as_ptr() as *const c_char,
    b"EXC PC\0".as_ptr() as *const c_char,
    b"PRID\0".as_ptr() as *const c_char,
    b"Config\0".as_ptr() as *const c_char,
    b"LLAddr\0".as_ptr() as *const c_char,
    b"Watch Lo\0".as_ptr() as *const c_char,
    b"Watch Hi\0".as_ptr() as *const c_char,
    b"X Context\0".as_ptr() as *const c_char,
    b"Reserved\0".as_ptr() as *const c_char,
    b"Impl Dep\0".as_ptr() as *const c_char,
    b"Debug\0".as_ptr() as *const c_char,
    b"DEPC\0".as_ptr() as *const c_char,
    b"PerfCnt\0".as_ptr() as *const c_char,
    b"ErrCtl\0".as_ptr() as *const c_char,
    b"CacheErr\0".as_ptr() as *const c_char,
    b"TagLo\0".as_ptr() as *const c_char,
    b"TagHi\0".as_ptr() as *const c_char,
    b"ErrorEPC\0".as_ptr() as *const c_char,
    b"DESAVE\0".as_ptr() as *const c_char,
];

/* The layout is supplied by the KVM/MIPS headers in the containing tree. */
#[repr(C)]
pub struct kvm_vcpu {
    pub vcpu_id: c_int,
    pub arch: kvm_vcpu_arch,
}

#[repr(C)]
pub struct kvm_vcpu_arch {
    pub cop0: kvm_mips_cop0,
}

#[repr(C)]
pub struct kvm_mips_cop0 {
    pub stat: [[c_ulong; 8]; 32],
}

pub unsafe fn kvm_mips_dump_stats(vcpu: *mut kvm_vcpu) {
    /* CONFIG_KVM_MIPS_DEBUG_COP0_COUNTERS is a build-time condition. */
    #[cfg(feature = "CONFIG_KVM_MIPS_DEBUG_COP0_COUNTERS")]
    {
        kvm_info(
            b"\nKVM VCPU[%d] COP0 Access Profile:\n\0".as_ptr() as *const c_char,
            (*vcpu).vcpu_id,
        );
        let mut i: c_int = 0;
        while i < N_MIPS_COPROC_REGS {
            let mut j: c_int = 0;
            while j < N_MIPS_COPROC_SEL {
                if (*vcpu).arch.cop0.stat[i as usize][j as usize] != 0 {
                    kvm_info(
                        b"%s[%d]: %lu\n\0".as_ptr() as *const c_char,
                        kvm_cop0_str[i as usize],
                        j,
                        (*vcpu).arch.cop0.stat[i as usize][j as usize],
                    );
                }
                j += 1;
            }
            i += 1;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
