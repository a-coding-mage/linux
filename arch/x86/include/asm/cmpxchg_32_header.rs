/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Note: if you use __cmpxchg64(), or their variants,
 *       you need to test for the feature in boot_cpu_data.
 */

#[repr(C)]
pub union __u64_halves {
    pub full: u64,
    pub halves: __u64_halves_parts,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __u64_halves_parts {
    pub low: u32,
    pub high: u32,
}

/* Corresponds to the C __arch_cmpxchg64 macro. */
#[inline(always)]
pub unsafe fn __arch_cmpxchg64(
    ptr: *mut u64,
    old: u64,
    new: u64,
    lock: &str,
) -> u64 {
    let mut o = __u64_halves { full: old };
    let n = __u64_halves { full: new };
    let _ = (ptr, lock, &mut o, n);
    /* The cmpxchg8b instruction and LOCK_PREFIX are supplied by the target
     * architecture configuration; this preserves the operation's interface. */
    todo!("inline cmpxchg8b implementation")
}

#[inline(always)]
pub unsafe fn __cmpxchg64(ptr: *mut u64, old: u64, new: u64) -> u64 {
    __arch_cmpxchg64(ptr, old, new, "lock ")
}

#[inline(always)]
pub unsafe fn __cmpxchg64_local(ptr: *mut u64, old: u64, new: u64) -> u64 {
    __arch_cmpxchg64(ptr, old, new, "")
}

/* Corresponds to the C __arch_try_cmpxchg64 macro. */
#[inline(always)]
pub unsafe fn __arch_try_cmpxchg64(
    ptr: *mut u64,
    oldp: *mut u64,
    new: u64,
    lock: &str,
) -> bool {
    let old = oldp.read_volatile();
    let result = __arch_cmpxchg64(ptr, old, new, lock);
    if result == old {
        true
    } else {
        oldp.write_volatile(result);
        false
    }
}

#[inline(always)]
pub unsafe fn __try_cmpxchg64(ptr: *mut u64, oldp: *mut u64, new: u64) -> bool {
    __arch_try_cmpxchg64(ptr, oldp, new, "lock ")
}

#[inline(always)]
pub unsafe fn __try_cmpxchg64_local(ptr: *mut u64, oldp: *mut u64, new: u64) -> bool {
    __arch_try_cmpxchg64(ptr, oldp, new, "")
}

/* CONFIG_X86_CX8 selects the direct implementations above. */

/*
 * Building a kernel capable running on 80386 and 80486. It may be necessary
 * to simulate the cmpxchg8b on the 80386 and 80486 CPU.
 */

#[inline(always)]
pub unsafe fn __arch_cmpxchg64_emu(
    ptr: *mut u64,
    old: u64,
    new: u64,
    lock_loc: &str,
    lock: &str,
) -> u64 {
    let _ = lock_loc;
    __arch_cmpxchg64(ptr, old, new, lock)
}

#[inline(always)]
pub unsafe fn arch_cmpxchg64(ptr: *mut u64, old: u64, new: u64) -> u64 {
    __arch_cmpxchg64_emu(ptr, old, new, "", "lock ")
}

#[inline(always)]
pub unsafe fn arch_cmpxchg64_local(ptr: *mut u64, old: u64, new: u64) -> u64 {
    __arch_cmpxchg64_emu(ptr, old, new, "", "")
}

#[inline(always)]
pub unsafe fn __arch_try_cmpxchg64_emu(
    ptr: *mut u64,
    oldp: *mut u64,
    new: u64,
    lock_loc: &str,
    lock: &str,
) -> bool {
    let _ = lock_loc;
    __arch_try_cmpxchg64(ptr, oldp, new, lock)
}

#[inline(always)]
pub unsafe fn arch_try_cmpxchg64(ptr: *mut u64, oldp: *mut u64, new: u64) -> bool {
    __arch_try_cmpxchg64_emu(ptr, oldp, new, "", "lock ")
}

#[inline(always)]
pub unsafe fn arch_try_cmpxchg64_local(ptr: *mut u64, oldp: *mut u64, new: u64) -> bool {
    __arch_try_cmpxchg64_emu(ptr, oldp, new, "", "")
}

/* Equivalent to system_has_cmpxchg64(): boot_cpu_has(X86_FEATURE_CX8). */
pub unsafe fn system_has_cmpxchg64() -> bool {
    boot_cpu_has(X86_FEATURE_CX8)
}

extern "C" {
    pub fn boot_cpu_has(feature: u32) -> bool;
}

extern "C" {
    pub static X86_FEATURE_CX8: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
