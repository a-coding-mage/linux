// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of testing/selftests/bpf/progs/exceptions.c.
// Original C dependencies:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, <bpf/bpf_endian.h>, "bpf_misc.h",
// and "bpf_experimental.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u64 = u64;
type s64 = i64;
type __u32 = u32;

const ETH_P_IP: u64 = 0x0800;
const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct __sk_buff {
    pub protocol: u32,
    pub tstamp: u64,
}

#[repr(C)]
pub struct jmp_table_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut jmp_table: jmp_table_def = jmp_table_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 4,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u32>() as u32,
};

unsafe extern "C" {
    fn bpf_throw(cookie: u64);
    fn bpf_tail_call_static(ctx: *mut __sk_buff, map: *mut jmp_table_def, index: u32);
    fn bpf_ktime_get_ns() -> u64;
}

#[inline]
fn bpf_ntohs(x: u32) -> u64 {
    u16::from_be(x as u16) as u64
}

#[inline]
unsafe fn bpf_assert(cond: bool) {
    if !cond {
        unsafe { bpf_throw(0) };
    }
}

#[inline]
unsafe fn bpf_assert_with(cond: bool, cookie: u64) {
    if !cond {
        unsafe { bpf_throw(cookie) };
    }
}

#[inline]
unsafe fn bpf_assert_range(value: u64, lo: u64, hi: u64) {
    unsafe { bpf_assert(value >= lo && value <= hi) };
}

#[inline]
unsafe fn bpf_assert_range_with(value: u64, lo: u64, hi: u64, cookie: u64) {
    unsafe { bpf_assert_with(value >= lo && value <= hi, cookie) };
}

#[inline]
fn static_func(i: u64) -> i32 {
    unsafe { bpf_throw(32) };
    i as i32
}

#[no_mangle]
pub extern "C" fn global2static_simple(i: u64) -> i32 {
    static_func(i.wrapping_add(2));
    i.wrapping_sub(1) as i32
}

#[no_mangle]
pub extern "C" fn global2static(i: u64) -> i32 {
    if i == ETH_P_IP {
        unsafe { bpf_throw(16) };
    }
    static_func(i)
}

#[inline]
fn static2global(i: u64) -> i32 {
    global2static(i).wrapping_add(i as i32)
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_throw_always_1(_ctx: *mut __sk_buff) -> i32 {
    unsafe { bpf_throw(64) };
    0
}

/* In this case, the global func will never be seen executing after call to
 * static subprog, hence verifier will DCE the remaining instructions. Ensure we
 * are resilient to that.
 */
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn exception_throw_always_2(ctx: *mut __sk_buff) -> i32 {
    unsafe { global2static_simple((*ctx).protocol as u64) }
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn exception_throw_unwind_1(ctx: *mut __sk_buff) -> i32 {
    unsafe { static2global(bpf_ntohs((*ctx).protocol)) }
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn exception_throw_unwind_2(ctx: *mut __sk_buff) -> i32 {
    unsafe { static2global(bpf_ntohs((*ctx).protocol).wrapping_sub(1)) }
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_throw_default(_ctx: *mut __sk_buff) -> i32 {
    unsafe { bpf_throw(0) };
    1
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_throw_default_value(_ctx: *mut __sk_buff) -> i32 {
    unsafe { bpf_throw(5) };
    1
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_tail_call_target(_ctx: *mut __sk_buff) -> i32 {
    unsafe { bpf_throw(16) };
    0
}

#[inline]
unsafe fn exception_tail_call_subprog(ctx: *mut __sk_buff) -> i32 {
    let ret: i32 = 10;

    unsafe { bpf_tail_call_static(ctx, core::ptr::addr_of_mut!(jmp_table), 0) };
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(ret)) }
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn exception_tail_call(ctx: *mut __sk_buff) -> i32 {
    let mut ret: i32 = 0;

    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(ret), exception_tail_call_subprog(ctx));
        core::ptr::read_volatile(core::ptr::addr_of!(ret)).wrapping_add(8)
    }
}

#[no_mangle]
pub extern "C" fn throw_11() {
    unsafe { bpf_throw(11) };
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_throw_from_void_global(_ctx: *mut __sk_buff) -> i32 {
    throw_11();

    0
}

#[no_mangle]
pub extern "C" fn exception_ext_global(_ctx: *mut __sk_buff) -> i32 {
    let ret: i32 = 0;

    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(ret)) }
}

#[inline]
fn exception_ext_static(ctx: *mut __sk_buff) -> i32 {
    exception_ext_global(ctx)
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_ext(ctx: *mut __sk_buff) -> i32 {
    exception_ext_static(ctx)
}

#[no_mangle]
pub extern "C" fn exception_cb_mod_global(_cookie: u64) -> i32 {
    let ret: i32 = 0;

    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(ret)) }
}

/* Example of how the exception callback supplied during verification can still
 * introduce extensions by calling to dummy global functions, and alter runtime
 * behavior.
 *
 * Right now we don't allow freplace attachment to exception callback itself,
 * but if the need arises this restriction is technically feasible to relax in
 * the future.
 */
#[no_mangle]
pub extern "C" fn exception_cb_mod(cookie: u64) -> i32 {
    exception_cb_mod_global(cookie)
        .wrapping_add(cookie as i32)
        .wrapping_add(10)
}

#[link_section = "tc"]
#[no_mangle]
/* __exception_cb(exception_cb_mod) */
pub extern "C" fn exception_ext_mod_cb_runtime(_ctx: *mut __sk_buff) -> i32 {
    unsafe { bpf_throw(25) };
    0
}

#[inline]
fn subprog(_ctx: *mut __sk_buff) -> i32 {
    unsafe { bpf_ktime_get_ns() as i32 }
}

#[inline]
unsafe fn throwing_subprog(ctx: *mut __sk_buff) -> i32 {
    if unsafe { (*ctx).tstamp } != 0 {
        unsafe { bpf_throw(0) };
    }
    unsafe { bpf_ktime_get_ns() as i32 }
}

#[no_mangle]
pub extern "C" fn global_subprog(_ctx: *mut __sk_buff) -> i32 {
    unsafe { bpf_ktime_get_ns() as i32 }
}

#[no_mangle]
pub unsafe extern "C" fn throwing_global_subprog(ctx: *mut __sk_buff) -> i32 {
    if unsafe { (*ctx).tstamp } != 0 {
        unsafe { bpf_throw(0) };
    }
    unsafe { bpf_ktime_get_ns() as i32 }
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn exception_throw_subprog(ctx: *mut __sk_buff) -> i32 {
    match unsafe { (*ctx).protocol } {
        1 => return subprog(ctx),
        2 => return global_subprog(ctx),
        3 => return unsafe { throwing_subprog(ctx) },
        4 => return unsafe { throwing_global_subprog(ctx) },
        _ => {}
    }
    unsafe { bpf_throw(1) };
    0
}

#[no_mangle]
pub extern "C" fn assert_nz_gfunc(c: u64) -> i32 {
    let cookie: u64 = c;

    unsafe { bpf_assert(core::ptr::read_volatile(core::ptr::addr_of!(cookie)) != 0) };
    0
}

#[no_mangle]
pub extern "C" fn assert_zero_gfunc(c: u64) -> i32 {
    let cookie: u64 = c;

    unsafe { bpf_assert(core::ptr::read_volatile(core::ptr::addr_of!(cookie)) == 0) };
    0
}

#[no_mangle]
pub extern "C" fn assert_neg_gfunc(c: s64) -> i32 {
    let cookie: s64 = c;

    unsafe { bpf_assert(core::ptr::read_volatile(core::ptr::addr_of!(cookie)) < 0) };
    0
}

#[no_mangle]
pub extern "C" fn assert_pos_gfunc(c: s64) -> i32 {
    let cookie: s64 = c;

    unsafe { bpf_assert(core::ptr::read_volatile(core::ptr::addr_of!(cookie)) > 0) };
    0
}

#[no_mangle]
pub extern "C" fn assert_negeq_gfunc(c: s64) -> i32 {
    let cookie: s64 = c;

    unsafe { bpf_assert(core::ptr::read_volatile(core::ptr::addr_of!(cookie)) <= -1) };
    0
}

#[no_mangle]
pub extern "C" fn assert_poseq_gfunc(c: s64) -> i32 {
    let cookie: s64 = c;

    unsafe { bpf_assert(core::ptr::read_volatile(core::ptr::addr_of!(cookie)) >= 1) };
    0
}

#[no_mangle]
pub extern "C" fn assert_nz_gfunc_with(c: u64) -> i32 {
    let cookie: u64 = c;
    let cookie_val = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(cookie)) };

    unsafe { bpf_assert_with(cookie_val != 0, cookie_val.wrapping_add(100)) };
    0
}

#[no_mangle]
pub extern "C" fn assert_zero_gfunc_with(c: u64) -> i32 {
    let cookie: u64 = c;
    let cookie_val = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(cookie)) };

    unsafe { bpf_assert_with(cookie_val == 0, cookie_val.wrapping_add(100)) };
    0
}

#[no_mangle]
pub extern "C" fn assert_neg_gfunc_with(c: s64) -> i32 {
    let cookie: s64 = c;
    let cookie_val = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(cookie)) };

    unsafe { bpf_assert_with(cookie_val < 0, (cookie_val + 100) as u64) };
    0
}

#[no_mangle]
pub extern "C" fn assert_pos_gfunc_with(c: s64) -> i32 {
    let cookie: s64 = c;
    let cookie_val = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(cookie)) };

    unsafe { bpf_assert_with(cookie_val > 0, (cookie_val + 100) as u64) };
    0
}

#[no_mangle]
pub extern "C" fn assert_negeq_gfunc_with(c: s64) -> i32 {
    let cookie: s64 = c;
    let cookie_val = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(cookie)) };

    unsafe { bpf_assert_with(cookie_val <= -1, (cookie_val + 100) as u64) };
    0
}

#[no_mangle]
pub extern "C" fn assert_poseq_gfunc_with(c: s64) -> i32 {
    let cookie: s64 = c;
    let cookie_val = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(cookie)) };

    unsafe { bpf_assert_with(cookie_val >= 1, (cookie_val + 100) as u64) };
    0
}

macro_rules! check_assert {
    ($fn_name:ident, $cookie:expr, $generated:ident) => {
        #[link_section = "tc"]
        #[no_mangle]
        pub extern "C" fn $generated(_ctx: *mut __sk_buff) -> i32 {
            $fn_name($cookie).wrapping_add(1)
        }
    };
}

check_assert!(assert_nz_gfunc, 5, exception_assert_nz_gfunc);
check_assert!(assert_zero_gfunc, 0, exception_assert_zero_gfunc);
check_assert!(assert_neg_gfunc, -100, exception_assert_neg_gfunc);
check_assert!(assert_pos_gfunc, 100, exception_assert_pos_gfunc);
check_assert!(assert_negeq_gfunc, -1, exception_assert_negeq_gfunc);
check_assert!(assert_poseq_gfunc, 1, exception_assert_poseq_gfunc);

check_assert!(assert_nz_gfunc_with, 5, exception_assert_nz_gfunc_with);
check_assert!(assert_zero_gfunc_with, 0, exception_assert_zero_gfunc_with);
check_assert!(assert_neg_gfunc_with, -100, exception_assert_neg_gfunc_with);
check_assert!(assert_pos_gfunc_with, 100, exception_assert_pos_gfunc_with);
check_assert!(assert_negeq_gfunc_with, -1, exception_assert_negeq_gfunc_with);
check_assert!(assert_poseq_gfunc_with, 1, exception_assert_poseq_gfunc_with);

check_assert!(assert_nz_gfunc, 0, exception_bad_assert_nz_gfunc);
check_assert!(assert_zero_gfunc, 5, exception_bad_assert_zero_gfunc);
check_assert!(assert_neg_gfunc, 100, exception_bad_assert_neg_gfunc);
check_assert!(assert_pos_gfunc, -100, exception_bad_assert_pos_gfunc);
check_assert!(assert_negeq_gfunc, 1, exception_bad_assert_negeq_gfunc);
check_assert!(assert_poseq_gfunc, -1, exception_bad_assert_poseq_gfunc);

check_assert!(assert_nz_gfunc_with, 0, exception_bad_assert_nz_gfunc_with);
check_assert!(assert_zero_gfunc_with, 5, exception_bad_assert_zero_gfunc_with);
check_assert!(assert_neg_gfunc_with, 100, exception_bad_assert_neg_gfunc_with);
check_assert!(assert_pos_gfunc_with, -100, exception_bad_assert_pos_gfunc_with);
check_assert!(assert_negeq_gfunc_with, 1, exception_bad_assert_negeq_gfunc_with);
check_assert!(assert_poseq_gfunc_with, -1, exception_bad_assert_poseq_gfunc_with);

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_assert_range(_ctx: *mut __sk_buff) -> i32 {
    let time: u64 = unsafe { bpf_ktime_get_ns() };

    unsafe { bpf_assert_range(time, 0, !0u64) };
    1
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_assert_range_with(_ctx: *mut __sk_buff) -> i32 {
    let time: u64 = unsafe { bpf_ktime_get_ns() };

    unsafe { bpf_assert_range_with(time, 0, !0u64, 10) };
    1
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_bad_assert_range(_ctx: *mut __sk_buff) -> i32 {
    let time: u64 = unsafe { bpf_ktime_get_ns() };

    unsafe { bpf_assert_range(time, (-100i64) as u64, 100) };
    1
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_bad_assert_range_with(_ctx: *mut __sk_buff) -> i32 {
    let time: u64 = unsafe { bpf_ktime_get_ns() };

    unsafe { bpf_assert_range_with(time, (-1000i64) as u64, 1000, 10) };
    1
}

// C conditional:
// #if (defined(__TARGET_ARCH_x86) || defined(__TARGET_ARCH_arm64)) \
//     && defined(__BPF_FEATURE_STACK_ARGUMENT)
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
pub const has_stack_arg: bool = true;

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
)))]
pub const has_stack_arg: bool = false;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg1: i64 = 1;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg2: i64 = 2;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg3: i64 = 3;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg4: i64 = 4;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg5: i64 = 5;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg6: i64 = 6;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg7: i64 = 7;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg8: i64 = 8;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg9: i64 = 9;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub static mut arg10: i64 = 10;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[inline]
fn throwing_many_args(
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
    g: i64,
    h: i64,
    i: i64,
    j: i64,
) -> i64 {
    unsafe { bpf_throw((a + b + c + d + e + f + g + h + i + j) as u64) };
    0
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub extern "C" fn exception_cb_sa(cookie: u64) -> i32 {
    cookie.wrapping_add(1) as i32
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[link_section = "tc"]
#[no_mangle]
/* __exception_cb(exception_cb_sa) */
pub unsafe extern "C" fn exception_throw_stack_arg(_ctx: *mut __sk_buff) -> i32 {
    unsafe {
        throwing_many_args(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
    }
    0
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[inline]
fn no_throw_many_args(
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
    g: i64,
    h: i64,
    i: i64,
    j: i64,
) -> i64 {
    a + b + c + d + e + f + g + h + i + j
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[link_section = "tc"]
#[no_mangle]
/* __exception_cb(exception_cb_sa) */
pub unsafe extern "C" fn exception_throw_after_stack_arg(_ctx: *mut __sk_buff) -> i32 {
    let ret: i64;

    unsafe {
        ret = no_throw_many_args(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
    }
    if ret > 0 {
        unsafe { bpf_throw(ret as u64) };
    }
    0
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[inline]
fn subprog_throw_sa(val: i64) -> i64 {
    throwing_many_args(
        val,
        val + 1,
        val + 2,
        val + 3,
        val + 4,
        val + 5,
        val + 6,
        val + 7,
        val + 8,
        val + 9,
    );
    0
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[link_section = "tc"]
#[no_mangle]
/* __exception_cb(exception_cb_sa) */
pub unsafe extern "C" fn exception_throw_subprog_stack_arg(_ctx: *mut __sk_buff) -> i32 {
    unsafe {
        subprog_throw_sa(arg1);
    }
    0
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[inline]
fn subprog_throw_after_sa(val: i64) -> i64 {
    let ret: i64;

    ret = no_throw_many_args(
        val,
        val + 1,
        val + 2,
        val + 3,
        val + 4,
        val + 5,
        val + 6,
        val + 7,
        val + 8,
        val + 9,
    );
    if ret > 0 {
        unsafe { bpf_throw(ret as u64) };
    }
    0
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[link_section = "tc"]
#[no_mangle]
/* __exception_cb(exception_cb_sa) */
pub unsafe extern "C" fn exception_throw_subprog_after_stack_arg(_ctx: *mut __sk_buff) -> i32 {
    unsafe {
        subprog_throw_after_sa(arg1);
    }
    0
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
)))]
#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_throw_stack_arg(_ctx: *mut __sk_buff) -> i32 {
    0
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
)))]
#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_throw_after_stack_arg(_ctx: *mut __sk_buff) -> i32 {
    0
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
)))]
#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_throw_subprog_stack_arg(_ctx: *mut __sk_buff) -> i32 {
    0
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
)))]
#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn exception_throw_subprog_after_stack_arg(_ctx: *mut __sk_buff) -> i32 {
    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
