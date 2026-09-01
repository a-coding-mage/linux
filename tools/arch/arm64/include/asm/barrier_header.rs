/* SPDX-License-Identifier: GPL-2.0 */

/*
 * From tools/perf/perf-sys.h, last modified in:
 * f428ebd184c82a7914b2aa7e9f868918aaf7ea78 perf tools: Fix AAAAARGH64 memory barriers
 *
 * XXX: arch/arm64/include/asm/barrier.h in the kernel sources use dsb, is this
 * a case like for arm32 where we do things differently in userspace?
 */

#[inline(always)]
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!("dmb ish", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn wmb() {
    unsafe {
        core::arch::asm!("dmb ishst", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn rmb() {
    unsafe {
        core::arch::asm!("dmb ishld", options(nostack, preserves_flags));
    }
}

/*
 * Kernel uses dmb variants on arm64 for smp_*() barriers. Pretty much the same
 * implementation as above mb()/wmb()/rmb(), though for the latter kernel uses
 * dsb. In any case, should above mb()/wmb()/rmb() change, make sure the below
 * smp_*() don't.
 */
#[inline(always)]
pub unsafe fn smp_mb() {
    unsafe {
        core::arch::asm!("dmb ish", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn smp_wmb() {
    unsafe {
        core::arch::asm!("dmb ishst", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn smp_rmb() {
    unsafe {
        core::arch::asm!("dmb ishld", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn smp_store_release<T: Copy>(p: *mut T, v: T) {
    match core::mem::size_of::<T>() {
        1 => {
            let __val = *(&v as *const T as *const __u8_alias_t);
            unsafe {
                core::arch::asm!(
                    "stlrb {1:w}, [{0}]",
                    in(reg) p,
                    in(reg) __val,
                    options(nostack, preserves_flags),
                );
            }
        }
        2 => {
            let __val = *(&v as *const T as *const __u16_alias_t);
            unsafe {
                core::arch::asm!(
                    "stlrh {1:w}, [{0}]",
                    in(reg) p,
                    in(reg) __val,
                    options(nostack, preserves_flags),
                );
            }
        }
        4 => {
            let __val = *(&v as *const T as *const __u32_alias_t);
            unsafe {
                core::arch::asm!(
                    "stlr {1:w}, [{0}]",
                    in(reg) p,
                    in(reg) __val,
                    options(nostack, preserves_flags),
                );
            }
        }
        8 => {
            let __val = *(&v as *const T as *const __u64_alias_t);
            unsafe {
                core::arch::asm!(
                    "stlr {1}, [{0}]",
                    in(reg) p,
                    in(reg) __val,
                    options(nostack, preserves_flags),
                );
            }
        }
        _ => {
            /* Only to shut up gcc ... */
            unsafe {
                mb();
            }
        }
    }
}

#[inline(always)]
pub unsafe fn smp_load_acquire<T: Copy>(p: *const T) -> T {
    let mut __u = core::mem::MaybeUninit::<T>::zeroed();

    match core::mem::size_of::<T>() {
        1 => unsafe {
            let mut __val: __u8_alias_t;
            core::arch::asm!(
                "ldarb {0:w}, [{1}]",
                out(reg) __val,
                in(reg) p,
                options(nostack, preserves_flags),
            );
            *(__u.as_mut_ptr() as *mut __u8_alias_t) = __val;
        },
        2 => unsafe {
            let mut __val: __u16_alias_t;
            core::arch::asm!(
                "ldarh {0:w}, [{1}]",
                out(reg) __val,
                in(reg) p,
                options(nostack, preserves_flags),
            );
            *(__u.as_mut_ptr() as *mut __u16_alias_t) = __val;
        },
        4 => unsafe {
            let mut __val: __u32_alias_t;
            core::arch::asm!(
                "ldar {0:w}, [{1}]",
                out(reg) __val,
                in(reg) p,
                options(nostack, preserves_flags),
            );
            *(__u.as_mut_ptr() as *mut __u32_alias_t) = __val;
        },
        8 => unsafe {
            let mut __val: __u64_alias_t;
            core::arch::asm!(
                "ldar {0}, [{1}]",
                out(reg) __val,
                in(reg) p,
                options(nostack, preserves_flags),
            );
            *(__u.as_mut_ptr() as *mut __u64_alias_t) = __val;
        },
        _ => {
            /* Only to shut up gcc ... */
            unsafe {
                mb();
            }
        }
    }

    unsafe { __u.assume_init() }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
