/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of asm/atomic_lse.h. */

// External kernel types and the LSE preamble are supplied by dependent headers.

#[inline(always)]
pub unsafe fn __lse_atomic_andnot(i: i32, v: *mut atomic_t) {
    core::arch::asm!("stclr {i:w}, [{v}]", i = in(reg) i, v = in(reg) (*v).counter, options(nostack));
}
#[inline(always)] pub unsafe fn __lse_atomic_or(i: i32, v: *mut atomic_t) { core::arch::asm!("stset {i:w}, [{v}]", i=in(reg)i, v=in(reg)(*v).counter, options(nostack)); }
#[inline(always)] pub unsafe fn __lse_atomic_xor(i: i32, v: *mut atomic_t) { core::arch::asm!("steor {i:w}, [{v}]", i=in(reg)i, v=in(reg)(*v).counter, options(nostack)); }
#[inline(always)] pub unsafe fn __lse_atomic_add(i: i32, v: *mut atomic_t) { core::arch::asm!("stadd {i:w}, [{v}]", i=in(reg)i, v=in(reg)(*v).counter, options(nostack)); }
#[inline(always)] pub unsafe fn __lse_atomic_sub(i: i32, v: *mut atomic_t) { __lse_atomic_add(-i, v); }

macro_rules! atomic_fetch_op {
    ($name:ident, $op:literal, $suffix:literal) => {
        #[inline(always)] pub unsafe fn $name(i: i32, v: *mut atomic_t) -> i32 {
            let mut old: i32;
            core::arch::asm!(concat!($op, $suffix, " {i:w}, {old:w}, [{v}]"), i=in(reg)i, old=lateout(reg) old, v=in(reg)(*v).counter, options(nostack));
            old
        }
    };
}

atomic_fetch_op!(__lse_atomic_fetch_andnot_relaxed, "ldclr", "");
atomic_fetch_op!(__lse_atomic_fetch_andnot_acquire, "ldclra", "");
atomic_fetch_op!(__lse_atomic_fetch_andnot_release, "ldclrl", "");
atomic_fetch_op!(__lse_atomic_fetch_andnot, "ldclral", "");
atomic_fetch_op!(__lse_atomic_fetch_or_relaxed, "ldset", "");
atomic_fetch_op!(__lse_atomic_fetch_or_acquire, "ldseta", "");
atomic_fetch_op!(__lse_atomic_fetch_or_release, "ldsetl", "");
atomic_fetch_op!(__lse_atomic_fetch_or, "ldsetal", "");
atomic_fetch_op!(__lse_atomic_fetch_xor_relaxed, "ldeor", "");
atomic_fetch_op!(__lse_atomic_fetch_xor_acquire, "ldeora", "");
atomic_fetch_op!(__lse_atomic_fetch_xor_release, "ldeorl", "");
atomic_fetch_op!(__lse_atomic_fetch_xor, "ldeoral", "");
atomic_fetch_op!(__lse_atomic_fetch_add_relaxed, "ldadd", "");
atomic_fetch_op!(__lse_atomic_fetch_add_acquire, "ldadda", "");
atomic_fetch_op!(__lse_atomic_fetch_add_release, "ldaddl", "");
atomic_fetch_op!(__lse_atomic_fetch_add, "ldaddal", "");

macro_rules! atomic_sub_fetch {
    ($name:ident, $add:ident) => { #[inline(always)] pub unsafe fn $name(i:i32,v:*mut atomic_t)->i32 { $add(-i,v) } };
}
atomic_sub_fetch!(__lse_atomic_fetch_sub_relaxed, __lse_atomic_fetch_add_relaxed);
atomic_sub_fetch!(__lse_atomic_fetch_sub_acquire, __lse_atomic_fetch_add_acquire);
atomic_sub_fetch!(__lse_atomic_fetch_sub_release, __lse_atomic_fetch_add_release);
atomic_sub_fetch!(__lse_atomic_fetch_sub, __lse_atomic_fetch_add);

macro_rules! atomic_returns {
    ($add:ident,$sub:ident,$fa:ident,$fs:ident) => {
        #[inline(always)] pub unsafe fn $add(i:i32,v:*mut atomic_t)->i32 { $fa(i,v).wrapping_add(i) }
        #[inline(always)] pub unsafe fn $sub(i:i32,v:*mut atomic_t)->i32 { $fs(i,v).wrapping_sub(i) }
    };
}
atomic_returns!(__lse_atomic_add_return_relaxed,__lse_atomic_sub_return_relaxed,__lse_atomic_fetch_add_relaxed,__lse_atomic_fetch_sub_relaxed);
atomic_returns!(__lse_atomic_add_return_acquire,__lse_atomic_sub_return_acquire,__lse_atomic_fetch_add_acquire,__lse_atomic_fetch_sub_acquire);
atomic_returns!(__lse_atomic_add_return_release,__lse_atomic_sub_return_release,__lse_atomic_fetch_add_release,__lse_atomic_fetch_sub_release);
atomic_returns!(__lse_atomic_add_return,__lse_atomic_sub_return,__lse_atomic_fetch_add,__lse_atomic_fetch_sub);

#[inline(always)] pub unsafe fn __lse_atomic_and(i:i32,v:*mut atomic_t){ __lse_atomic_andnot(!i,v); }
macro_rules! atomic_and_fetch { ($n:ident,$f:ident) => { #[inline(always)] pub unsafe fn $n(i:i32,v:*mut atomic_t)->i32{$f(!i,v)} }; }
atomic_and_fetch!(__lse_atomic_fetch_and_relaxed,__lse_atomic_fetch_andnot_relaxed);
atomic_and_fetch!(__lse_atomic_fetch_and_acquire,__lse_atomic_fetch_andnot_acquire);
atomic_and_fetch!(__lse_atomic_fetch_and_release,__lse_atomic_fetch_andnot_release);
atomic_and_fetch!(__lse_atomic_fetch_and,__lse_atomic_fetch_andnot);

// 64-bit variants mirror the 32-bit operations and preserve signed 64-bit intent.
macro_rules! atomic64_op { ($n:ident,$ins:literal) => { #[inline(always)] pub unsafe fn $n(i:i64,v:*mut atomic64_t){core::arch::asm!(concat!($ins," {i}, [{v}]"),i=in(reg)i,v=in(reg)(*v).counter,options(nostack));} }; }
atomic64_op!(__lse_atomic64_andnot,"stclr"); atomic64_op!(__lse_atomic64_or,"stset"); atomic64_op!(__lse_atomic64_xor,"steor"); atomic64_op!(__lse_atomic64_add,"stadd");
#[inline(always)] pub unsafe fn __lse_atomic64_sub(i:i64,v:*mut atomic64_t){__lse_atomic64_add(-i,v);}
macro_rules! atomic64_fetch { ($n:ident,$ins:literal) => { #[inline(always)] pub unsafe fn $n(i:i64,v:*mut atomic64_t)->i64{let mut old:i64;core::arch::asm!(concat!($ins," {i}, {old}, [{v}]"),i=in(reg)i,old=lateout(reg)old,v=in(reg)(*v).counter,options(nostack));old} }; }
atomic64_fetch!(__lse_atomic64_fetch_andnot_relaxed,"ldclr"); atomic64_fetch!(__lse_atomic64_fetch_andnot_acquire,"ldclra"); atomic64_fetch!(__lse_atomic64_fetch_andnot_release,"ldclrl"); atomic64_fetch!(__lse_atomic64_fetch_andnot,"ldclral");
atomic64_fetch!(__lse_atomic64_fetch_or_relaxed,"ldset"); atomic64_fetch!(__lse_atomic64_fetch_or_acquire,"ldseta"); atomic64_fetch!(__lse_atomic64_fetch_or_release,"ldsetl"); atomic64_fetch!(__lse_atomic64_fetch_or,"ldsetal");
atomic64_fetch!(__lse_atomic64_fetch_xor_relaxed,"ldeor"); atomic64_fetch!(__lse_atomic64_fetch_xor_acquire,"ldeora"); atomic64_fetch!(__lse_atomic64_fetch_xor_release,"ldeorl"); atomic64_fetch!(__lse_atomic64_fetch_xor,"ldeoral");
atomic64_fetch!(__lse_atomic64_fetch_add_relaxed,"ldadd"); atomic64_fetch!(__lse_atomic64_fetch_add_acquire,"ldadda"); atomic64_fetch!(__lse_atomic64_fetch_add_release,"ldaddl"); atomic64_fetch!(__lse_atomic64_fetch_add,"ldaddal");
macro_rules! atomic64_sub {($n:ident,$a:ident)=>{#[inline(always)]pub unsafe fn $n(i:i64,v:*mut atomic64_t)->i64{$a(-i,v)}};}
atomic64_sub!(__lse_atomic64_fetch_sub_relaxed,__lse_atomic64_fetch_add_relaxed); atomic64_sub!(__lse_atomic64_fetch_sub_acquire,__lse_atomic64_fetch_add_acquire); atomic64_sub!(__lse_atomic64_fetch_sub_release,__lse_atomic64_fetch_add_release); atomic64_sub!(__lse_atomic64_fetch_sub,__lse_atomic64_fetch_add);
macro_rules! atomic64_ret {($a:ident,$s:ident,$fa:ident,$fs:ident)=>{#[inline(always)]pub unsafe fn $a(i:i64,v:*mut atomic64_t)->i64{$fa(i,v).wrapping_add(i)} #[inline(always)]pub unsafe fn $s(i:i64,v:*mut atomic64_t)->i64{$fs(i,v).wrapping_sub(i)}};}
atomic64_ret!(__lse_atomic64_add_return_relaxed,__lse_atomic64_sub_return_relaxed,__lse_atomic64_fetch_add_relaxed,__lse_atomic64_fetch_sub_relaxed); atomic64_ret!(__lse_atomic64_add_return_acquire,__lse_atomic64_sub_return_acquire,__lse_atomic64_fetch_add_acquire,__lse_atomic64_fetch_sub_acquire); atomic64_ret!(__lse_atomic64_add_return_release,__lse_atomic64_sub_return_release,__lse_atomic64_fetch_add_release,__lse_atomic64_fetch_sub_release); atomic64_ret!(__lse_atomic64_add_return,__lse_atomic64_sub_return,__lse_atomic64_fetch_add,__lse_atomic64_fetch_sub);
#[inline(always)]pub unsafe fn __lse_atomic64_and(i:i64,v:*mut atomic64_t){__lse_atomic64_andnot(!i,v)}
macro_rules! atomic64_and {($n:ident,$f:ident)=>{#[inline(always)]pub unsafe fn $n(i:i64,v:*mut atomic64_t)->i64{$f(!i,v)}};}
atomic64_and!(__lse_atomic64_fetch_and_relaxed,__lse_atomic64_fetch_andnot_relaxed); atomic64_and!(__lse_atomic64_fetch_and_acquire,__lse_atomic64_fetch_andnot_acquire); atomic64_and!(__lse_atomic64_fetch_and_release,__lse_atomic64_fetch_andnot_release); atomic64_and!(__lse_atomic64_fetch_and,__lse_atomic64_fetch_andnot);

macro_rules! cmpxchg_case {
    ($n:ident,$t:ty,$w:literal,$s:literal) => {
        #[inline(always)] pub unsafe fn $n(ptr:*mut core::ffi::c_void,mut old:$t,new:$t)->$t {
            core::arch::asm!(concat!("cas",$s," {old:",$w,"}, {new:",$w,"}, [{ptr}]"), old=inout(reg)old, new=in(reg)new, ptr=in(reg)ptr, options(nostack)); old
        }
    };
}
cmpxchg_case!(__lse__cmpxchg_case_8,u8,"w","b"); cmpxchg_case!(__lse__cmpxchg_case_16,u16,"w","h"); cmpxchg_case!(__lse__cmpxchg_case_32,u32,"w",""); cmpxchg_case!(__lse__cmpxchg_case_64,u64,"x","");
cmpxchg_case!(__lse__cmpxchg_case_acq_8,u8,"w","b"); cmpxchg_case!(__lse__cmpxchg_case_acq_16,u16,"w","h"); cmpxchg_case!(__lse__cmpxchg_case_acq_32,u32,"w",""); cmpxchg_case!(__lse__cmpxchg_case_acq_64,u64,"x","");
cmpxchg_case!(__lse__cmpxchg_case_rel_8,u8,"w","b"); cmpxchg_case!(__lse__cmpxchg_case_rel_16,u16,"w","h"); cmpxchg_case!(__lse__cmpxchg_case_rel_32,u32,"w",""); cmpxchg_case!(__lse__cmpxchg_case_rel_64,u64,"x","");
cmpxchg_case!(__lse__cmpxchg_case_mb_8,u8,"w","b"); cmpxchg_case!(__lse__cmpxchg_case_mb_16,u16,"w","h"); cmpxchg_case!(__lse__cmpxchg_case_mb_32,u32,"w",""); cmpxchg_case!(__lse__cmpxchg_case_mb_64,u64,"x","");

#[inline(always)] pub unsafe fn __lse_atomic64_dec_if_positive(v:*mut atomic64_t)->i64 {
    loop { let old = core::ptr::read_volatile(&(*v).counter); let ret=old.wrapping_sub(1); if ret<0{return ret;} if __lse__cmpxchg_case_64((&mut (*v).counter) as *mut _ as *mut core::ffi::c_void,old,ret)==old{return ret;} }
}

// The 128-bit CAS entry points use the externally supplied u128 representation.
extern "C" { pub fn __lse__cmpxchg128(ptr:*mut u128, old:u128, new:u128)->u128; pub fn __lse__cmpxchg128_mb(ptr:*mut u128, old:u128, new:u128)->u128; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
