/* SPDX-License-Identifier: GPL-2.0 */

// Translated from x86/include/asm/msr.h. Included C headers and build-time
// configuration are supplied by the surrounding translation unit.

#[repr(C)]
pub struct msr_info {
    pub msr_no: u32,
    pub reg: msr,
    pub msrs: *mut msr,
    pub err: i32,
}

#[repr(C)]
pub struct msr_regs_info {
    pub regs: *mut u32,
    pub err: i32,
}

#[repr(C)]
pub struct saved_msr {
    pub valid: bool,
    pub info: msr_info,
}

#[repr(C)]
pub struct saved_msrs {
    pub num: u32,
    pub array: *mut saved_msr,
}

#[cfg(feature = "CONFIG_TRACEPOINTS")]
extern "C" {
    pub fn do_trace_write_msr(msr: u32, val: u64, failed: i32);
    pub fn do_trace_read_msr(msr: u32, val: u64, failed: i32);
    pub fn do_trace_rdpmc(msr: u32, val: u64, failed: i32);
}

#[cfg(not(feature = "CONFIG_TRACEPOINTS"))]
#[inline]
pub unsafe fn do_trace_write_msr(_msr: u32, _val: u64, _failed: i32) {}
#[cfg(not(feature = "CONFIG_TRACEPOINTS"))]
#[inline]
pub unsafe fn do_trace_read_msr(_msr: u32, _val: u64, _failed: i32) {}
#[cfg(not(feature = "CONFIG_TRACEPOINTS"))]
#[inline]
pub unsafe fn do_trace_rdpmc(_msr: u32, _val: u64, _failed: i32) {}

#[inline(always)]
pub unsafe fn __rdmsr(_msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    core::arch::asm!("rdmsr", in("ecx") _msr, lateout("eax") low, lateout("edx") high);
    (low as u64) | ((high as u64) << 32)
}

#[inline(always)]
pub unsafe fn __wrmsrq(msr: u32, val: u64) {
    core::arch::asm!("wrmsr", in("ecx") msr, in("eax") val as u32, in("edx") (val >> 32) as u32);
}

#[inline]
pub unsafe fn native_rdmsr(msr: u32, val1: *mut u32, val2: *mut u32) {
    let val = __rdmsr(msr);
    *val1 = val as u32;
    *val2 = (val >> 32) as u32;
}

#[inline(always)]
pub unsafe fn native_rdmsrq(msr: u32) -> u64 { __rdmsr(msr) }

#[inline(always)]
pub unsafe fn native_wrmsr(msr: u32, low: u32, high: u32) {
    __wrmsrq(msr, ((high as u64) << 32) | low as u64)
}

#[inline(always)]
pub unsafe fn native_wrmsrq(msr: u32, val: u64) { __wrmsrq(msr, val) }

extern "C" {
    pub fn rdmsr_safe_regs(regs: *mut u32) -> i32;
    pub fn wrmsr_safe_regs(regs: *mut u32) -> i32;
    pub fn msrs_alloc() -> *mut msr;
    pub fn msrs_free(msrs: *mut msr);
    pub fn msr_set_bit(msr: u32, bit: u8) -> i32;
    pub fn msr_clear_bit(msr: u32, bit: u8) -> i32;
}

#[inline]
pub unsafe fn native_read_msr(msr: u32) -> u64 { __rdmsr(msr) }

#[inline]
pub unsafe fn native_read_msr_safe(msr: u32, p: *mut u64) -> i32 {
    *p = __rdmsr(msr);
    0
}

#[inline]
pub unsafe fn native_write_msr(msr: u32, val: u64) { __wrmsrq(msr, val); }

#[inline]
pub unsafe fn native_write_msr_safe(msr: u32, val: u64) { __wrmsrq(msr, val); }

#[inline]
pub unsafe fn rdmsr(msr: u32, low: *mut u32, high: *mut u32) {
    native_rdmsr(msr, low, high)
}
#[inline]
pub unsafe fn wrmsr(msr: u32, low: u32, high: u32) { native_write_msr(msr, ((high as u64) << 32) | low as u64); }
#[inline]
pub unsafe fn rdmsrq(msr: u32, val: *mut u64) { *val = native_read_msr(msr); }
#[inline]
pub unsafe fn wrmsrq(msr: u32, val: u64) { native_write_msr(msr, val); }
#[inline]
pub unsafe fn rdmsr_safe(msr: u32, low: *mut u32, high: *mut u32) -> i32 {
    let mut val = 0u64;
    let err = native_read_msr_safe(msr, &mut val);
    *low = val as u32;
    *high = (val >> 32) as u32;
    err
}

#[inline]
pub unsafe fn native_read_pmc(counter: i32) -> u64 {
    let low: u32;
    let high: u32;
    core::arch::asm!("rdpmc", in("ecx") counter, lateout("eax") low, lateout("edx") high);
    (low as u64) | ((high as u64) << 32)
}

#[inline]
pub unsafe fn rdmsrq_safe(msr: u32, p: *mut u64) -> i32 { native_read_msr_safe(msr, p) }
#[inline]
pub unsafe fn wrmsrq_safe(msr: u32, val: u64) { native_write_msr_safe(msr, val); }
#[inline(always)]
pub unsafe fn rdpmc(counter: i32) -> u64 { native_read_pmc(counter) }

/* WRMSRNS is selected by the original build's X86_FEATURE_WRMSRNS alternative. */
#[inline(always)]
pub unsafe fn wrmsrns(msr: u32, val: u64) { __wrmsrq(msr, val); }

#[inline]
pub unsafe fn wrmsr_safe(msr: u32, low: u32, high: u32) {
    wrmsrq_safe(msr, ((high as u64) << 32) | low as u64)
}

extern "C" {
    pub fn rdmsr_on_cpus(mask: *const cpumask, msr_no: u32, msrs: *mut msr);
    pub fn wrmsr_on_cpus(mask: *const cpumask, msr_no: u32, msrs: *mut msr);
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn rdmsrq_on_cpu(cpu: u32, msr_no: u32, q: *mut u64) -> i32;
    pub fn wrmsrq_on_cpu(cpu: u32, msr_no: u32, q: u64) -> i32;
    pub fn rdmsrq_safe_on_cpu(cpu: u32, msr_no: u32, q: *mut u64) -> i32;
    pub fn wrmsrq_safe_on_cpu(cpu: u32, msr_no: u32, q: u64) -> i32;
    pub fn rdmsr_safe_regs_on_cpu(cpu: u32, regs: *mut u32) -> i32;
    pub fn wrmsr_safe_regs_on_cpu(cpu: u32, regs: *mut u32) -> i32;
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn rdmsrq_on_cpu(_cpu: u32, msr_no: u32, q: *mut u64) -> i32 { rdmsrq_safe(msr_no, q) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn wrmsrq_on_cpu(_cpu: u32, msr_no: u32, q: u64) -> i32 { wrmsrq_safe(msr_no, q); 0 }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn rdmsrq_safe_on_cpu(_cpu: u32, msr_no: u32, q: *mut u64) -> i32 { rdmsrq_safe(msr_no, q) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn wrmsrq_safe_on_cpu(_cpu: u32, msr_no: u32, q: u64) -> i32 { wrmsrq_safe(msr_no, q) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn rdmsr_safe_regs_on_cpu(_cpu: u32, regs: *mut u32) -> i32 { rdmsr_safe_regs(regs) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn wrmsr_safe_regs_on_cpu(_cpu: u32, regs: *mut u32) -> i32 { wrmsr_safe_regs(regs) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
