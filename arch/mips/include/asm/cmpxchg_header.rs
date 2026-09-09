/*
 * Rust translation of asm/cmpxchg.h.
 * C preprocessor configuration and MIPS inline-assembly details are retained
 * in comments where they cannot be expressed file-locally.
 */

unsafe extern "C" {
    fn __cmpxchg_called_with_bad_pointer() -> libc::c_ulong;
    fn __cmpxchg64_unsupported() -> libc::c_ulong;
    fn __xchg_called_with_bad_pointer() -> libc::c_ulong;
    fn __xchg_small(ptr: *mut core::ffi::c_void, val: libc::c_ulong, size: u32) -> libc::c_ulong;
    fn __cmpxchg_small(ptr: *mut core::ffi::c_void, old: libc::c_ulong,
                       new: libc::c_ulong, size: u32) -> libc::c_ulong;
}

// Supplied by the surrounding kernel translation.
unsafe extern "C" {
    static kernel_uses_llsc: bool;
    static __SYNC_loongson3_war: i32;
    static cpu_has_64bits: bool;
    fn raw_local_irq_save(flags: *mut libc::c_ulong);
    fn raw_local_irq_restore(flags: libc::c_ulong);
    fn local_irq_save(flags: *mut libc::c_ulong);
    fn local_irq_restore(flags: libc::c_ulong);
    fn smp_mb__before_llsc();
    fn smp_llsc_mb();
}

#[inline(always)]
unsafe fn __xchg_asm<T: Copy>(m: *mut T, val: T) -> T {
    // __xchg_asm uses MIPS LL/SC (or interrupt exclusion when kernel_uses_llsc
    // is false); the original assembly is intentionally represented here.
    if kernel_uses_llsc {
        todo!("MIPS __xchg_asm LL/SC sequence")
    } else {
        let mut flags = 0;
        raw_local_irq_save(&mut flags);
        let ret = core::ptr::read_volatile(m);
        core::ptr::write_volatile(m, val);
        raw_local_irq_restore(flags);
        ret
    }
}

#[inline(always)]
pub unsafe fn __arch_xchg(ptr: *mut core::ffi::c_void, x: libc::c_ulong, size: i32) -> libc::c_ulong {
    match size {
        1 | 2 => __xchg_small(ptr, x, size as u32),
        4 => __xchg_asm(ptr as *mut u32, x as u32) as libc::c_ulong,
        8 => {
            // CONFIG_64BIT is a build-time condition from the original header.
            __xchg_asm(ptr as *mut u64, x as u64) as libc::c_ulong
        }
        _ => __xchg_called_with_bad_pointer(),
    }
}

#[inline(always)]
pub unsafe fn arch_xchg<T: Copy>(ptr: *mut T, x: T) -> T {
    if __SYNC_loongson3_war == 0 { smp_mb__before_llsc(); }
    let res = __arch_xchg(ptr.cast(), x as libc::c_ulong, core::mem::size_of::<T>() as i32) as T;
    smp_llsc_mb();
    res
}

#[inline(always)]
unsafe fn __cmpxchg_asm<T: Copy + PartialEq>(m: *mut T, old: T, new: T) -> T {
    if kernel_uses_llsc {
        todo!("MIPS __cmpxchg_asm LL/SC sequence")
    } else {
        let mut flags = 0;
        raw_local_irq_save(&mut flags);
        let ret = core::ptr::read_volatile(m);
        if ret == old { core::ptr::write_volatile(m, new); }
        raw_local_irq_restore(flags);
        ret
    }
}

#[inline(always)]
pub unsafe fn __cmpxchg<T: Copy + PartialEq>(ptr: *mut T, old: libc::c_ulong,
                                             new: libc::c_ulong, size: u32) -> libc::c_ulong {
    match size {
        1 | 2 => __cmpxchg_small(ptr.cast(), old, new, size),
        4 => __cmpxchg_asm(ptr as *mut u32, old as u32, new as u32) as libc::c_ulong,
        8 => __cmpxchg_asm(ptr as *mut u64, old as u64, new as u64) as libc::c_ulong,
        _ => __cmpxchg_called_with_bad_pointer(),
    }
}

#[inline(always)]
pub unsafe fn arch_cmpxchg_local<T: Copy + PartialEq>(ptr: *mut T, old: T, new: T) -> T {
    __cmpxchg(ptr, old as libc::c_ulong, new as libc::c_ulong,
              core::mem::size_of::<T>() as u32) as T
}

#[inline(always)]
pub unsafe fn arch_cmpxchg<T: Copy + PartialEq>(ptr: *mut T, old: T, new: T) -> T {
    if __SYNC_loongson3_war == 0 { smp_mb__before_llsc(); }
    let res = arch_cmpxchg_local(ptr, old, new);
    if __SYNC_loongson3_war == 0 { smp_llsc_mb(); }
    res
}

// CONFIG_64BIT provides arch_cmpxchg64_local/arch_cmpxchg64 via the generic
// operations above.  On 32-bit CONFIG_SMP builds the original header supplies
// a hand-written lld/scd sequence; its externally visible operation is below.
#[inline(always)]
pub unsafe fn arch_cmpxchg64_local(ptr: *mut u64, old: u64, new: u64) -> u64 {
    arch_cmpxchg_local(ptr, old, new)
}

#[inline(always)]
pub unsafe fn arch_cmpxchg64(ptr: *mut u64, old: u64, new: u64) -> u64 {
    if cpu_has_64bits && kernel_uses_llsc {
        smp_mb__before_llsc();
        let ret = arch_cmpxchg64_local(ptr, old, new);
        smp_llsc_mb();
        ret
    } else {
        __cmpxchg64_unsupported() as u64
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
