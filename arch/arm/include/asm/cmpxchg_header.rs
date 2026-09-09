/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux::irqflags, linux::prefetch, asm::barrier, and cmpxchg emulation.

#[cfg(any(CONFIG_CPU_SA1100, CONFIG_CPU_SA110))]
// On StrongARM, SWP bypasses the cache; exchange is therefore emulated with
// interrupts disabled.  This is not suitable for SMP.
const SWP_IS_BUGGY: bool = true;

extern "C" {
    fn __bad_xchg(ptr: *mut core::ffi::c_void, size: i32);
    fn __bad_cmpxchg(ptr: *mut core::ffi::c_void, size: i32);
    fn prefetchw(ptr: *const core::ffi::c_void);
    fn raw_local_irq_save(flags: *mut usize);
    fn raw_local_irq_restore(flags: usize);
    fn __generic_cmpxchg_local(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: usize) -> usize;
    fn __generic_cmpxchg64_local(ptr: *mut core::ffi::c_void, old: u64, new: u64) -> u64;
    fn cmpxchg_emu_u8(ptr: *mut u8, old: usize, new: usize) -> usize;
}

#[inline]
pub unsafe fn __arch_xchg(x: usize, ptr: *mut core::ffi::c_void, size: i32) -> usize {
    prefetchw(ptr as *const _);
    match size {
        1 => {
            #[cfg(any(CONFIG_CPU_SA1100, CONFIG_CPU_SA110))]
            {
                let mut flags = 0usize;
                raw_local_irq_save(&mut flags);
                let ret = core::ptr::read_volatile(ptr as *const u8) as usize;
                core::ptr::write_volatile(ptr as *mut u8, x as u8);
                raw_local_irq_restore(flags);
                ret
            }
            #[cfg(not(any(CONFIG_CPU_SA1100, CONFIG_CPU_SA110)))]
            { core::ptr::swap(ptr as *mut u8, &mut (x as u8)) as usize }
        }
        4 => {
            #[cfg(any(CONFIG_CPU_SA1100, CONFIG_CPU_SA110))]
            {
                let mut flags = 0usize;
                raw_local_irq_save(&mut flags);
                let ret = core::ptr::read_volatile(ptr as *const u32) as usize;
                core::ptr::write_volatile(ptr as *mut u32, x as u32);
                raw_local_irq_restore(flags);
                ret
            }
            #[cfg(not(any(CONFIG_CPU_SA1100, CONFIG_CPU_SA110)))]
            { core::ptr::replace(ptr as *mut u32, x as u32) as usize }
        }
        _ => { __bad_xchg(ptr, size); 0 }
    }
}

#[inline]
pub unsafe fn arch_xchg_relaxed<T: Copy>(ptr: *mut T, x: T) -> T {
    let ret = __arch_xchg(x_as_usize(x), ptr as *mut _, core::mem::size_of::<T>() as i32);
    usize_as_value(ret)
}

// These casts stand for the C __typeof__ conversions used by the header.
unsafe fn x_as_usize<T: Copy>(x: T) -> usize { core::mem::transmute_copy(&x) }
unsafe fn usize_as_value<T: Copy>(x: usize) -> T { core::mem::transmute_copy(&x) }

#[cfg(not(any(ARM_ARCH_GE_6)))]
#[inline]
pub unsafe fn arch_cmpxchg_local<T: Copy>(ptr: *mut T, old: T, new: T) -> T {
    usize_as_value(__generic_cmpxchg_local(ptr as *mut _, x_as_usize(old), x_as_usize(new), core::mem::size_of::<T>()))
}

#[cfg(ARM_ARCH_GE_6)]
#[inline]
pub unsafe fn __cmpxchg(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: i32) -> usize {
    prefetchw(ptr as *const _);
    match size {
        1 => {
            #[cfg(CONFIG_CPU_V6)]
            { cmpxchg_emu_u8(ptr as *mut u8, old, new) }
            #[cfg(not(CONFIG_CPU_V6))]
            { let _ = (old, new); core::ptr::read_volatile(ptr as *const u8) as usize }
        }
        2 => { let _ = (old, new); core::ptr::read_volatile(ptr as *const u16) as usize }
        4 => { let _ = (old, new); core::ptr::read_volatile(ptr as *const u32) as usize }
        _ => { __bad_cmpxchg(ptr, size); 0 }
    }
}

#[cfg(ARM_ARCH_GE_6)]
#[inline]
pub unsafe fn arch_cmpxchg_relaxed<T: Copy>(ptr: *mut T, old: T, new: T) -> T {
    usize_as_value(__cmpxchg(ptr as *mut _, x_as_usize(old), x_as_usize(new), core::mem::size_of::<T>() as i32))
}

#[cfg(ARM_ARCH_GE_6)]
#[inline]
pub unsafe fn __cmpxchg_local(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: i32) -> usize {
    __cmpxchg(ptr, old, new, size)
}

#[cfg(ARM_ARCH_GE_6)]
#[inline]
pub unsafe fn arch_cmpxchg_local<T: Copy>(ptr: *mut T, old: T, new: T) -> T {
    arch_cmpxchg_relaxed(ptr, old, new)
}

#[cfg(ARM_ARCH_GE_6)]
#[inline]
pub unsafe fn __cmpxchg64(ptr: *mut u64, old: u64, new: u64) -> u64 {
    prefetchw(ptr as *const _);
    let oldval = core::ptr::read_volatile(ptr);
    if oldval == old { core::ptr::write_volatile(ptr, new); }
    oldval
}

#[cfg(ARM_ARCH_GE_6)]
#[inline]
pub unsafe fn arch_cmpxchg64_relaxed(ptr: *mut u64, old: u64, new: u64) -> u64 { __cmpxchg64(ptr, old, new) }

#[cfg(ARM_ARCH_GE_6)]
#[inline]
pub unsafe fn arch_cmpxchg64_local(ptr: *mut u64, old: u64, new: u64) -> u64 { arch_cmpxchg64_relaxed(ptr, old, new) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
