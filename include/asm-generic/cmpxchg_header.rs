/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generic UP xchg and cmpxchg using interrupt disablement.  Does not
 * support SMP.
 */

// CONFIG_SMP is rejected by the original header: generic cmpxchg cannot be
// used on SMP systems.

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    pub fn __generic_xchg_called_with_bad_pointer();
    pub fn local_irq_save(flags: *mut usize);
    pub fn local_irq_restore(flags: usize);
    pub fn __generic_cmpxchg_local(
        ptr: *mut core::ffi::c_void,
        old: usize,
        new: usize,
        size: usize,
    ) -> usize;
    pub fn __generic_cmpxchg64_local(
        ptr: *mut core::ffi::c_void,
        old: u64,
        new: u64,
    ) -> u64;
}

/*
 * This function doesn't exist, so you'll get a linker error if
 * something tries to do an invalidly-sized xchg().
 */
#[inline]
pub unsafe fn __generic_xchg<T: Copy>(x: usize, ptr: *mut T, size: i32) -> usize {
    let mut flags: usize = 0;
    match size {
        1 => {
            // __xchg_u8, when supplied by the architecture, is used here.
            local_irq_save(&mut flags as *mut usize);
            let ret = core::ptr::read_volatile(ptr as *const u8) as usize;
            core::ptr::write_volatile(ptr as *mut u8, (x & 0xff) as u8);
            local_irq_restore(flags);
            ret
        }
        2 => {
            // __xchg_u16, when supplied by the architecture, is used here.
            local_irq_save(&mut flags as *mut usize);
            let ret = core::ptr::read_volatile(ptr as *const u16) as usize;
            core::ptr::write_volatile(ptr as *mut u16, (x & 0xffff) as u16);
            local_irq_restore(flags);
            ret
        }
        4 => {
            // __xchg_u32, when supplied by the architecture, is used here.
            local_irq_save(&mut flags as *mut usize);
            let ret = core::ptr::read_volatile(ptr as *const u32) as usize;
            core::ptr::write_volatile(ptr as *mut u32, (x & 0xffff_ffff) as u32);
            local_irq_restore(flags);
            ret
        }
        // CONFIG_64BIT
        8 => {
            // __xchg_u64, when supplied by the architecture, is used here.
            local_irq_save(&mut flags as *mut usize);
            let ret = core::ptr::read_volatile(ptr as *const u64) as usize;
            core::ptr::write_volatile(ptr as *mut u64, x as u64);
            local_irq_restore(flags);
            ret
        }
        _ => {
            __generic_xchg_called_with_bad_pointer();
            x
        }
    }
}

#[inline]
pub unsafe fn generic_xchg<T: Copy>(ptr: *mut T, x: T) -> T {
    let raw = __generic_xchg(x_as_usize(&x), ptr, core::mem::size_of::<T>() as i32);
    core::mem::transmute_copy(&raw)
}

#[inline]
unsafe fn x_as_usize<T>(value: &T) -> usize {
    let mut raw = 0usize;
    core::ptr::copy_nonoverlapping(
        value as *const T as *const u8,
        &mut raw as *mut usize as *mut u8,
        core::mem::size_of::<T>().min(core::mem::size_of::<usize>()),
    );
    raw
}

/* Atomic compare and exchange. */
#[inline]
pub unsafe fn generic_cmpxchg_local<T: Copy>(ptr: *mut T, old: T, new: T) -> T {
    let ret = __generic_cmpxchg_local(
        ptr as *mut core::ffi::c_void,
        x_as_usize(&old),
        x_as_usize(&new),
        core::mem::size_of::<T>(),
    );
    core::mem::transmute_copy(&ret)
}

#[inline]
pub unsafe fn generic_cmpxchg64_local(ptr: *mut u64, old: u64, new: u64) -> u64 {
    __generic_cmpxchg64_local(ptr as *mut core::ffi::c_void, old, new)
}

// arch_xchg defaults to generic_xchg when not supplied by the architecture.
// arch_cmpxchg_local defaults to generic_cmpxchg_local.
// arch_cmpxchg64_local defaults to generic_cmpxchg64_local.
// arch_cmpxchg and arch_cmpxchg64 alias their corresponding local forms.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
