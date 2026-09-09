// SPDX-License-Identifier: GPL-2.0
// Linux kernel headers and export/trace macros are supplied by external dependencies.

use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct msr {
    pub q: u64,
}

extern "C" {
    fn alloc_percpu(size: usize) -> *mut msr;
    fn free_percpu(ptr: *mut msr);
    fn rdmsrq_safe(msr: u32, val: *mut u64) -> i32;
    fn wrmsrq_safe(msr: u32, val: u64) -> i32;
    fn pr_warn(fmt: *const core::ffi::c_char, ...);

    #[cfg(feature = "config_tracepoints")]
    fn trace_write_msr(msr: u32, val: u64, failed: i32);
    #[cfg(feature = "config_tracepoints")]
    fn trace_read_msr(msr: u32, val: u64, failed: i32);
    #[cfg(feature = "config_tracepoints")]
    fn trace_rdpmc(msr: u32, val: u64, failed: i32);
}

pub unsafe fn msrs_alloc() -> *mut msr {
    let mut msrs: *mut msr = ptr::null_mut();

    msrs = alloc_percpu(size_of::<msr>());
    if msrs.is_null() {
        // pr_warn("%s: error allocating msrs\n", __func__);
        return ptr::null_mut();
    }

    msrs
}

// EXPORT_SYMBOL(msrs_alloc);

pub unsafe fn msrs_free(msrs: *mut msr) {
    free_percpu(msrs);
}

// EXPORT_SYMBOL(msrs_free);

/**
 * msr_read - Read an MSR with error handling
 * @msr: MSR to read
 * @m: value to read into
 *
 * It returns read data only on success, otherwise it doesn't change the output
 * argument @m.
 *
 * Return: %0 for success, otherwise an error code
 */
unsafe fn msr_read(msr: u32, m: *mut msr) -> i32 {
    let mut val: u64 = 0;
    let err = rdmsrq_safe(msr, &mut val);
    if err == 0 {
        (*m).q = val;
    }
    err
}

/**
 * msr_write - Write an MSR with error handling
 *
 * @msr: MSR to write
 * @m: value to write
 *
 * Return: %0 for success, otherwise an error code
 */
unsafe fn msr_write(msr: u32, m: *mut msr) -> i32 {
    wrmsrq_safe(msr, (*m).q)
}

unsafe fn __flip_bit(msr: u32, bit: u8, set: bool) -> i32 {
    let mut m = msr { q: 0 };
    let mut m1: msr;
    let mut err: i32 = -22; // -EINVAL

    if bit > 63 {
        return err;
    }

    err = msr_read(msr, &mut m);
    if err != 0 {
        return err;
    }

    m1 = m;
    if set {
        m1.q |= 1u64.wrapping_shl(bit as u32);
    } else {
        m1.q &= !(1u64.wrapping_shl(bit as u32));
    }

    if m1.q == m.q {
        return 0;
    }

    err = msr_write(msr, &mut m1);
    if err != 0 {
        return err;
    }

    1
}

/**
 * msr_set_bit - Set @bit in a MSR @msr.
 * @msr: MSR to write
 * @bit: bit number to set
 */
pub unsafe fn msr_set_bit(msr: u32, bit: u8) -> i32 {
    __flip_bit(msr, bit, true)
}

// EXPORT_SYMBOL_FOR_KVM(msr_set_bit);

/**
 * msr_clear_bit - Clear @bit in a MSR @msr.
 * @msr: MSR to write
 * @bit: bit number to clear
 */
pub unsafe fn msr_clear_bit(msr: u32, bit: u8) -> i32 {
    __flip_bit(msr, bit, false)
}

// EXPORT_SYMBOL_FOR_KVM(msr_clear_bit);

#[cfg(feature = "config_tracepoints")]
pub unsafe fn do_trace_write_msr(msr: u32, val: u64, failed: i32) {
    trace_write_msr(msr, val, failed);
}

#[cfg(feature = "config_tracepoints")]
pub unsafe fn do_trace_read_msr(msr: u32, val: u64, failed: i32) {
    trace_read_msr(msr, val, failed);
}

#[cfg(feature = "config_tracepoints")]
pub unsafe fn do_trace_rdpmc(msr: u32, val: u64, failed: i32) {
    trace_rdpmc(msr, val, failed);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
