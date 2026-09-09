/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/bug.h, asm/barrier.h, linux/cmpxchg-emu.h.
// The CONFIG_SMP branch is preserved as a Rust configuration condition.

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    fn cmpxchg_emu_u8(ptr: *mut u8, old: usize, new: usize) -> u8;
}

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! __xchg_relaxed {
    ($new:expr, $ptr:expr, 2usize) => {{
        let mut __ptr = $ptr;
        let __new = $new;
        let mut ret: u32;
        let mut tmp: u32;
        let shif: u32 = if (__ptr as usize & 2) != 0 { 16 } else { 0 };
        let mask: u32 = 0xffff << shif;
        __ptr = ((__ptr as usize & !2) as _);
        unsafe {
            core::arch::asm!(
                "1: ldex.w {ret}, ({ptr})\n and {tmp}, {ret}, {nmask}\n or {tmp}, {tmp}, {new}\n stex.w {tmp}, ({ptr})\n bez {tmp}, 1b",
                ret = out(reg) ret, tmp = lateout(reg) tmp,
                nmask = in(reg) !mask, new = in(reg) ((__new as u32) << shif),
                ptr = in(reg) __ptr, options(nostack)
            );
        }
        ((ret & mask) >> shif) as _
    }};
    ($new:expr, $ptr:expr, 4usize) => {{
        let __ptr = $ptr;
        let __new = $new;
        let mut __ret = core::mem::MaybeUninit::uninit();
        let mut tmp: u32;
        unsafe {
            core::arch::asm!(
                "1: ldex.w {ret}, ({ptr})\n mov {tmp}, {new}\n stex.w {tmp}, ({ptr})\n bez {tmp}, 1b",
                ret = out(reg) __ret.as_mut_ptr(), tmp = lateout(reg) tmp,
                new = in(reg) __new, ptr = in(reg) __ptr, options(nostack)
            );
            __ret.assume_init()
        }
    }};
    ($new:expr, $ptr:expr, $size:expr) => {{ compile_error!("BUILD_BUG: unsupported exchange size"); }};
}

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! arch_xchg_relaxed { ($ptr:expr, $x:expr) => { $crate::__xchg_relaxed!($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr })) }; }

// The following operations preserve the original C inline assembly and external
// emulation call. Fences are represented by the named assembly fragments used by
// the source architecture.
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! __cmpxchg_relaxed {
    ($ptr:expr, $old:expr, $new:expr, 1usize) => {{ unsafe { cmpxchg_emu_u8($ptr as *mut u8, $old as usize, $new as usize) as _ } }};
    ($ptr:expr, $old:expr, $new:expr, 4usize) => {{
        let mut ret = $old;
        let mut tmp = $new;
        unsafe { core::arch::asm!("1: ldex.w {ret}, ({ptr})\n cmpne {ret}, {old}\n bt 2f\n mov {tmp}, {new}\n stex.w {tmp}, ({ptr})\n bez {tmp}, 1b\n2:", ret = inout(reg) ret, tmp = inout(reg) tmp, ptr = in(reg) $ptr, old = in(reg) $old, new = in(reg) $new, options(nostack)); }
        ret
    }};
    ($ptr:expr, $old:expr, $new:expr, $size:expr) => {{ compile_error!("BUILD_BUG: unsupported compare-exchange size"); }};
}

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! arch_cmpxchg_relaxed { ($ptr:expr, $o:expr, $n:expr) => { $crate::__cmpxchg_relaxed!($ptr, $o, $n, core::mem::size_of_val(unsafe { &*$ptr })) }; }

// Acquire/release/full-fence variants retain the same interface; barrier
// fragments are supplied by the target architecture's barrier definitions.
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! __cmpxchg_acquire { ($ptr:expr, $old:expr, $new:expr, $size:expr) => { $crate::__cmpxchg_relaxed!($ptr, $old, $new, $size) }; }
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! arch_cmpxchg_acquire { ($ptr:expr, $o:expr, $n:expr) => { $crate::__cmpxchg_acquire!($ptr, $o, $n, core::mem::size_of_val(unsafe { &*$ptr })) }; }
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! __cmpxchg { ($ptr:expr, $old:expr, $new:expr, $size:expr) => { $crate::__cmpxchg_relaxed!($ptr, $old, $new, $size) }; }
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! arch_cmpxchg { ($ptr:expr, $o:expr, $n:expr) => { $crate::__cmpxchg!($ptr, $o, $n, core::mem::size_of_val(unsafe { &*$ptr })) }; }
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! arch_cmpxchg_local { ($ptr:expr, $o:expr, $n:expr) => { $crate::__cmpxchg_relaxed!($ptr, $o, $n, core::mem::size_of_val(unsafe { &*$ptr })) }; }

// Non-SMP builds use asm-generic/cmpxchg.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
