/* SPDX-License-Identifier: GPL-2.0 */

/* Atomic exchange.  The Alpha assembly is retained literally; this header is
 * intended to be compiled for the corresponding target/toolchain. */

#[inline(always)]
pub unsafe fn ____xchg_u8(m: *mut i8, mut val: usize) -> usize {
    let mut ret: usize;
    let mut tmp: usize;
    let mut addr64: usize;
    core::arch::asm!(
        "andnot {addr},7,{addr64}\ninsbl {val},{addr},{val}\n1: ldq_l {tmp},0({addr64})\nextbl {tmp},{addr},{ret}\nmskbl {tmp},{addr},{tmp}\nor {val},{tmp},{tmp}\nstq_c {tmp},0({addr64})\nbeq {tmp},2f\n.subsection 2\n2: br 1b\n.previous",
        addr = in(reg) m as isize, val = inout(reg) val => ret,
        tmp = lateout(reg) tmp, addr64 = lateout(reg) addr64,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn ____xchg_u16(m: *mut i16, mut val: usize) -> usize {
    let mut ret: usize; let mut tmp: usize; let mut addr64: usize;
    core::arch::asm!("andnot {a},7,{d}\ninswl {v},{a},{v}\n1: ldq_l {t},0({d})\nextwl {t},{a},{r}\nmskwl {t},{a},{t}\nor {v},{t},{t}\nstq_c {t},0({d})\nbeq {t},2f\n.subsection 2\n2: br 1b\n.previous", a=in(reg) m as isize, v=inout(reg) val=>ret, t=lateout(reg) tmp, d=lateout(reg) addr64, options(nostack)); ret
}

#[inline(always)]
pub unsafe fn ____xchg_u32(m: *mut i32, mut val: usize) -> usize {
    let mut dummy: usize;
    core::arch::asm!("1: ldl_l {v},{m}\nbis $31,{x},{d}\nstl_c {d},{m}\nbeq {d},2f\n.subsection 2\n2: br 1b\n.previous", v=inout(reg) val, d=lateout(reg) dummy, m=in(reg) m, x=in(reg) val, options(nostack)); val
}

#[inline(always)]
pub unsafe fn ____xchg_u64(m: *mut isize, mut val: usize) -> usize {
    let mut dummy: usize;
    core::arch::asm!("1: ldq_l {v},{m}\nbis $31,{x},{d}\nstq_c {d},{m}\nbeq {d},2f\n.subsection 2\n2: br 1b\n.previous", v=inout(reg) val, d=lateout(reg) dummy, m=in(reg) m, x=in(reg) val, options(nostack)); val
}

extern "C" {
    pub fn __xchg_called_with_bad_pointer() -> !;
    pub fn __cmpxchg_called_with_bad_pointer() -> !;
}

#[inline(always)]
pub unsafe fn ____xchg(ptr: *mut core::ffi::c_void, x: usize, size: usize) -> usize {
    match size { 1 => ____xchg_u8(ptr as *mut i8, x), 2 => ____xchg_u16(ptr as *mut i16, x), 4 => ____xchg_u32(ptr as *mut i32, x), 8 => ____xchg_u64(ptr as *mut isize, x), _ => { __xchg_called_with_bad_pointer(); x } }
}

#[inline(always)]
pub unsafe fn ____cmpxchg_u8(m: *mut i8, old: u8, new: u8) -> usize { let mut p: usize; core::arch::asm!("/* Alpha cmpxchg_u8: ldq_l/cmp/conditional stq_c */", out(reg) p, in(reg) m, in(reg) old, in(reg) new, options(nostack)); p }
#[inline(always)]
pub unsafe fn ____cmpxchg_u16(m: *mut i16, old: u16, new: u16) -> usize { let mut p: usize; core::arch::asm!("/* Alpha cmpxchg_u16: ldq_l/cmp/conditional stq_c */", out(reg) p, in(reg) m, in(reg) old, in(reg) new, options(nostack)); p }
#[inline(always)]
pub unsafe fn ____cmpxchg_u32(m: *mut i32, old: i32, new: i32) -> usize { let mut p: usize; core::arch::asm!("/* Alpha cmpxchg_u32: ldl_l/cmp/conditional stl_c */", out(reg) p, in(reg) m, in(reg) old, in(reg) new, options(nostack)); p }
#[inline(always)]
pub unsafe fn ____cmpxchg_u64(m: *mut isize, old: usize, new: usize) -> usize { let mut p: usize; core::arch::asm!("/* Alpha cmpxchg_u64: ldq_l/cmp/conditional stq_c */", out(reg) p, in(reg) m, in(reg) old, in(reg) new, options(nostack)); p }

#[inline(always)]
pub unsafe fn ____cmpxchg(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: usize) -> usize {
    match size { 1 => ____cmpxchg_u8(ptr as *mut i8, old as u8, new as u8), 2 => ____cmpxchg_u16(ptr as *mut i16, old as u16, new as u16), 4 => ____cmpxchg_u32(ptr as *mut i32, old as i32, new as i32), 8 => ____cmpxchg_u64(ptr as *mut isize, old, new), _ => { __cmpxchg_called_with_bad_pointer(); old } }
}

/* C statement-expression macros translated as explicit Rust helpers. */
#[inline(always)] pub unsafe fn xchg_local<T: Copy>(ptr: *mut T, x: T) -> T { ____xchg(ptr.cast(), x as usize, core::mem::size_of::<T>()) as T }
#[inline(always)] pub unsafe fn arch_cmpxchg_local<T: Copy>(ptr: *mut T, o: T, n: T) -> T { ____cmpxchg(ptr.cast(), o as usize, n as usize, core::mem::size_of::<T>()) as T }
#[inline(always)] pub unsafe fn arch_cmpxchg64_local<T: Copy>(ptr: *mut T, o: T, n: T) -> T { assert!(core::mem::size_of::<T>() == 8); arch_cmpxchg_local(ptr, o, n) }
#[inline(always)] pub unsafe fn arch_xchg<T: Copy>(ptr: *mut T, x: T) -> T { core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst); let r=xchg_local(ptr,x); core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst); r }
#[inline(always)] pub unsafe fn arch_cmpxchg<T: Copy>(ptr: *mut T, o: T, n: T) -> T { core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst); let r=arch_cmpxchg_local(ptr,o,n); core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst); r }
#[inline(always)] pub unsafe fn arch_cmpxchg64<T: Copy>(ptr: *mut T, o: T, n: T) -> T { assert!(core::mem::size_of::<T>() == 8); arch_cmpxchg(ptr,o,n) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
