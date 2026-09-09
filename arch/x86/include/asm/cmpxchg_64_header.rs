/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header _ASM_X86_CMPXCHG_64_H.

#[macro_export]
macro_rules! arch_cmpxchg64 {
    ($ptr:expr, $o:expr, $n:expr) => {{
        BUILD_BUG_ON!(core::mem::size_of_val(unsafe { &*$ptr }) != 8);
        arch_cmpxchg!($ptr, $o, $n)
    }};
}

#[macro_export]
macro_rules! arch_cmpxchg64_local {
    ($ptr:expr, $o:expr, $n:expr) => {{
        BUILD_BUG_ON!(core::mem::size_of_val(unsafe { &*$ptr }) != 8);
        arch_cmpxchg_local!($ptr, $o, $n)
    }};
}

#[macro_export]
macro_rules! arch_try_cmpxchg64 {
    ($ptr:expr, $po:expr, $n:expr) => {{
        BUILD_BUG_ON!(core::mem::size_of_val(unsafe { &*$ptr }) != 8);
        arch_try_cmpxchg!($ptr, $po, $n)
    }};
}

#[macro_export]
macro_rules! arch_try_cmpxchg64_local {
    ($ptr:expr, $po:expr, $n:expr) => {{
        BUILD_BUG_ON!(core::mem::size_of_val(unsafe { &*$ptr }) != 8);
        arch_try_cmpxchg_local!($ptr, $po, $n)
    }};
}

#[repr(C)]
pub union __u128_halves {
    pub full: u128,
    pub halves: __u128_halves_struct,
}

#[repr(C)]
pub struct __u128_halves_struct {
    pub low: u64,
    pub high: u64,
}

#[macro_export]
macro_rules! __arch_cmpxchg128 {
    ($ptr:expr, $old:expr, $new:expr, $lock:expr) => {{
        let mut o = __u128_halves { full: $old };
        let n = __u128_halves { full: $new };
        let (o_low, o_high) = unsafe {
            let h = o.halves;
            (h.low, h.high)
        };
        let (n_low, n_high) = unsafe {
            let h = n.halves;
            (h.low, h.high)
        };
        let mut low = o_low;
        let mut high = o_high;
        unsafe {
            core::arch::asm!(
                concat!($lock, "cmpxchg16b [{ptr}]") ,
                ptr = in(reg) $ptr,
                inout("rax") low,
                inout("rdx") high,
                in("rbx") n_low,
                in("rcx") n_high,
                options nostack, preserves_flags
            );
        }
        o = __u128_halves { halves: __u128_halves_struct { low, high } };
        unsafe { o.full }
    }};
}

#[inline(always)]
pub unsafe fn arch_cmpxchg128(ptr: *mut u128, old: u128, new: u128) -> u128 {
    __arch_cmpxchg128!(ptr, old, new, LOCK_PREFIX)
}

#[inline(always)]
pub unsafe fn arch_cmpxchg128_local(ptr: *mut u128, old: u128, new: u128) -> u128 {
    __arch_cmpxchg128!(ptr, old, new, "")
}

#[macro_export]
macro_rules! __arch_try_cmpxchg128 {
    ($ptr:expr, $oldp:expr, $new:expr, $lock:expr) => {{
        let mut o = __u128_halves { full: unsafe { *$oldp } };
        let n = __u128_halves { full: $new };
        let (mut low, mut high) = unsafe {
            let h = o.halves;
            (h.low, h.high)
        };
        let (n_low, n_high) = unsafe {
            let h = n.halves;
            (h.low, h.high)
        };
        let mut ret: u8;
        unsafe {
            core::arch::asm!(
                concat!($lock, "cmpxchg16b [{ptr}]") ,
                ptr = in(reg) $ptr,
                inout("rax") low,
                inout("rdx") high,
                in("rbx") n_low,
                in("rcx") n_high,
                setz ret,
                options nostack
            );
        }
        if unlikely!(!(ret != 0)) {
            o = __u128_halves { halves: __u128_halves_struct { low, high } };
            unsafe { *$oldp = o.full; }
        }
        likely!(ret != 0)
    }};
}

#[inline(always)]
pub unsafe fn arch_try_cmpxchg128(ptr: *mut u128, oldp: *mut u128, new: u128) -> bool {
    __arch_try_cmpxchg128!(ptr, oldp, new, LOCK_PREFIX)
}

#[inline(always)]
pub unsafe fn arch_try_cmpxchg128_local(ptr: *mut u128, oldp: *mut u128, new: u128) -> bool {
    __arch_try_cmpxchg128!(ptr, oldp, new, "")
}

#[macro_export]
macro_rules! system_has_cmpxchg128 {
    () => { boot_cpu_has!(X86_FEATURE_CX16) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
