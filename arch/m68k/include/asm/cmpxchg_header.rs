/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/irqflags.h, linux/minmax.h, asm-generic/cmpxchg-local.h,
// and asm-generic/cmpxchg.h.

#[inline]
pub unsafe fn __xg<T>(x: *mut T) -> *mut T {
    x
}

unsafe extern "C" {
    pub fn __invalid_xchg_size(x: usize, ptr: *mut core::ffi::c_void, size: i32) -> usize;
}

// When CONFIG_RMW_INSNS is not enabled, this is the interrupt-disabled swap
// implementation. The configuration condition is preserved here for the
// build system to select the corresponding definition.
#[cfg(not(CONFIG_RMW_INSNS))]
#[inline]
pub unsafe fn __arch_xchg(mut x: usize, ptr: *mut core::ffi::c_void, size: i32) -> usize {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);

    match size {
        1 => {
            let p = ptr as *mut u8;
            core::ptr::swap(p, &mut x as *mut usize as *mut u8);
        }
        2 => {
            let p = ptr as *mut u16;
            core::ptr::swap(p, &mut x as *mut usize as *mut u16);
        }
        4 => {
            let p = ptr as *mut u32;
            core::ptr::swap(p, &mut x as *mut usize as *mut u32);
        }
        _ => x = __invalid_xchg_size(x, ptr, size),
    }

    local_irq_restore(flags);
    x
}

// CONFIG_RMW_INSNS selects the m68k CAS instruction implementation. The
// inline assembly strings and operands are retained as the direct low-level
// equivalent for the target architecture.
#[cfg(CONFIG_RMW_INSNS)]
#[inline]
pub unsafe fn __arch_xchg(mut x: usize, ptr: *mut core::ffi::c_void, size: i32) -> usize {
    match size {
        1 => core::arch::asm!(
            "moveb {x}, {tmp}\n\t1:\n\tcasb {tmp}, {x}, [{ptr}]\n\tjne 1b",
            x = inout(reg) x,
            tmp = lateout(reg) _,
            ptr = in(reg) ptr,
            options(nostack)
        ),
        2 => core::arch::asm!(
            "movew {x}, {tmp}\n\t1:\n\tcasw {tmp}, {x}, [{ptr}]\n\tjne 1b",
            x = inout(reg) x,
            tmp = lateout(reg) _,
            ptr = in(reg) ptr,
            options(nostack)
        ),
        4 => core::arch::asm!(
            "movel {x}, {tmp}\n\t1:\n\tcasl {tmp}, {x}, [{ptr}]\n\tjne 1b",
            x = inout(reg) x,
            tmp = lateout(reg) _,
            ptr = in(reg) ptr,
            options(nostack)
        ),
        _ => x = __invalid_xchg_size(x, ptr, size),
    }
    x
}

#[macro_export]
macro_rules! arch_xchg {
    ($ptr:expr, $x:expr) => {{
        let value = $x;
        $crate::__arch_xchg(
            value as usize,
            ($ptr) as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of_val(&*($ptr)) as i32,
        ) as _
    }};
}

#[macro_export]
macro_rules! arch_cmpxchg64_local {
    ($ptr:expr, $o:expr, $n:expr) => {
        __generic_cmpxchg64_local(($ptr), ($o), ($n))
    };
}

unsafe extern "C" {
    pub fn __invalid_cmpxchg_size(
        ptr: *mut core::ffi::c_void,
        old: usize,
        new: usize,
        size: i32,
    ) -> usize;
}

#[cfg(CONFIG_RMW_INSNS)]
#[inline]
pub unsafe fn __cmpxchg(
    mut p: *mut core::ffi::c_void,
    mut old: usize,
    new: usize,
    size: i32,
) -> usize {
    match size {
        1 => core::arch::asm!("casb {old}, {new}, [{p}]", old = inout(reg) old, new = in(reg) new, p = in(reg) p),
        2 => core::arch::asm!("casw {old}, {new}, [{p}]", old = inout(reg) old, new = in(reg) new, p = in(reg) p),
        4 => core::arch::asm!("casl {old}, {new}, [{p}]", old = inout(reg) old, new = in(reg) new, p = in(reg) p),
        _ => old = __invalid_cmpxchg_size(p, old, new, size),
    }
    old
}

#[macro_export]
macro_rules! arch_cmpxchg {
    ($ptr:expr, $o:expr, $n:expr) => {
        $crate::__cmpxchg(($ptr) as *mut _ as *mut core::ffi::c_void, ($o) as usize, ($n) as usize, core::mem::size_of_val(&*($ptr)) as i32) as _
    };
}

#[macro_export]
macro_rules! arch_cmpxchg_local {
    ($ptr:expr, $o:expr, $n:expr) => {
        $crate::__cmpxchg(($ptr) as *mut _ as *mut core::ffi::c_void, ($o) as usize, ($n) as usize, core::mem::size_of_val(&*($ptr)) as i32) as _
    };
}

#[cfg(CONFIG_RMW_INSNS)]
#[macro_export]
macro_rules! arch_cmpxchg64 {
    ($ptr:expr, $o:expr, $n:expr) => { arch_cmpxchg64_local!(($ptr), ($o), ($n)) };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
