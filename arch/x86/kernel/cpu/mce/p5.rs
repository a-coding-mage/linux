// SPDX-License-Identifier: GPL-2.0
/*
 * P5 specific Machine Check Exception Reporting
 * (C) Copyright 2002 Alan Cox <alan@lxorguk.ukuu.org.uk>
 */

// Kernel and architecture dependencies supplied by the surrounding tree.

/* By default disabled */
#[no_mangle]
pub static mut mce_p5_enabled: i32 = 0;

extern "C" {
    pub fn rdmsrq(msr: u32, value: *mut u64);
    pub fn smp_processor_id() -> i32;
    pub fn add_taint(taint: u32, lockdep: u32);
    pub fn cr4_set_bits(bits: u32);
    pub fn cpu_has(c: *const cpuinfo_x86, feature: u32) -> bool;
    pub fn pr_emerg(format: *const core::ffi::c_char, ...);
    pub fn pr_info(format: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpuinfo_x86 {
    _private: [u8; 0],
}

const MSR_IA32_P5_MC_ADDR: u32 = 0x0000_0000;
const MSR_IA32_P5_MC_TYPE: u32 = 0x0000_0001;
const X86_FEATURE_MCE: u32 = 0;
const X86_CR4_MCE: u32 = 1 << 6;
const TAINT_MACHINE_CHECK: u32 = 0;
const LOCKDEP_NOW_UNRELIABLE: u32 = 0;

/* Machine check handler for Pentium class Intel CPUs: */
#[no_mangle]
pub unsafe extern "C" fn pentium_machine_check(regs: *mut pt_regs) {
    let _ = regs;
    let mut addr: u64 = 0;
    let mut machine_type: u64 = 0;

    // instrumentation_begin();
    rdmsrq(MSR_IA32_P5_MC_ADDR, &mut addr as *mut u64);
    rdmsrq(MSR_IA32_P5_MC_TYPE, &mut machine_type as *mut u64);

    pr_emerg(
        b"CPU#%d: Machine Check Exception:  0x%8X (type 0x%8X).\n\0".as_ptr() as *const core::ffi::c_char,
        smp_processor_id(),
        addr as u32,
        machine_type as u32,
    );

    if machine_type & (1 << 5) != 0 {
        pr_emerg(
            b"CPU#%d: Possible thermal failure (CPU on fire ?).\n\0".as_ptr()
                as *const core::ffi::c_char,
            smp_processor_id(),
        );
    }

    add_taint(TAINT_MACHINE_CHECK, LOCKDEP_NOW_UNRELIABLE);
    // instrumentation_end();
}

/* Set up machine check reporting for processors with Intel style MCE: */
#[no_mangle]
pub unsafe extern "C" fn intel_p5_mcheck_init(c: *mut cpuinfo_x86) {
    let mut q: u64 = 0;

    /* Default P5 to off as its often misconnected: */
    if mce_p5_enabled == 0 {
        return;
    }

    /* Check for MCE support: */
    if !cpu_has(c as *const cpuinfo_x86, X86_FEATURE_MCE) {
        return;
    }

    /* Read registers before enabling: */
    rdmsrq(MSR_IA32_P5_MC_ADDR, &mut q as *mut u64);
    rdmsrq(MSR_IA32_P5_MC_TYPE, &mut q as *mut u64);
    pr_info(
        b"Intel old style machine check architecture supported.\n\0".as_ptr()
            as *const core::ffi::c_char,
    );

    /* Enable MCE: */
    cr4_set_bits(X86_CR4_MCE);
    pr_info(
        b"Intel old style machine check reporting enabled on CPU#%d.\n\0".as_ptr()
            as *const core::ffi::c_char,
        smp_processor_id(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
