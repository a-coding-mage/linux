// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependency expectations:
// vmlinux.h, limits.h, bpf/bpf_tracing.h, bpf/bpf_helpers.h,
// bpf/bpf_core_read.h, bpf/bpf_endian.h, bpf_misc.h, bpf_experimental.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type s32 = i32;
type s64 = i64;
type u8 = u8;
type u32 = u32;
type u64 = u64;

const INT_MIN: s64 = i32::MIN as s64;
const INT_MAX: s64 = i32::MAX as s64;
const LLONG_MIN: s64 = i64::MIN;
const LLONG_MAX: s64 = i64::MAX;

#[repr(C)]
pub struct bpf_sock {
    pub rx_queue_mapping: s32,
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub data: u32,
    pub data_end: u32,
    pub sk: *mut bpf_sock,
}

unsafe extern "C" {
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_assert(_: bool);
    fn bpf_assert_range<T>(num: T, min: T, max: T);
    fn bpf_assert_with(_: bool, _: i32);
}

#[inline(always)]
fn bpf_cmp_unlikely<T: PartialOrd>(left: T, op_result: bool, _right: T) -> bool {
    op_result
}

// __msg("R{{.}}=0xffffffff80000000")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_eq_int_min(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num == INT_MIN, INT_MIN)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=0x7fffffff")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_eq_int_max(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num == INT_MAX, INT_MAX)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=0")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_eq_zero(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num == 0, 0)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=0x8000000000000000")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_eq_llong_min(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num == LLONG_MIN, LLONG_MIN)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=0x7fffffffffffffff")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_eq_llong_max(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num == LLONG_MAX, LLONG_MAX)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smax=0x7ffffffe)")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_lt_pos(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num < INT_MAX, INT_MAX)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smax=-1,umin=0x8000000000000000,var_off=(0x8000000000000000; 0x7fffffffffffffff))")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_lt_zero(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num < 0, 0)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smax=0xffffffff7fffffff")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_lt_neg(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num < INT_MIN, INT_MIN)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smax=0x7fffffff)")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_le_pos(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num <= INT_MAX, INT_MAX)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smax=0)")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_le_zero(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num <= 0, 0)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smax=0xffffffff80000000")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_le_neg(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num <= INT_MIN, INT_MIN)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smin=umin=0x80000000,umax=0x7fffffffffffffff,var_off=(0x0; 0x7fffffffffffffff))")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_gt_pos(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num > INT_MAX, INT_MAX)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smin=umin=1,umax=0x7fffffffffffffff,var_off=(0x0; 0x7fffffffffffffff))")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_gt_zero(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num > 0, 0)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smin=0xffffffff80000001")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_gt_neg(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num > INT_MIN, INT_MIN)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smin=umin=0x7fffffff,umax=0x7fffffffffffffff,var_off=(0x0; 0x7fffffffffffffff))")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_ge_pos(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num >= INT_MAX, INT_MAX)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smin=0,umax=0x7fffffffffffffff,var_off=(0x0; 0x7fffffffffffffff))")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_ge_zero(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num >= 0, 0)) };
    unsafe { *(num as *const u64) as i32 }
}

// __msg("R{{.}}=scalar(id=1,smin=0xffffffff80000000")
// SEC("?tc") __log_level(2) __failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_ge_neg(_ctx: *mut core::ffi::c_void) -> i32 {
    let num: s64 = unsafe { bpf_ktime_get_ns() } as s64;
    unsafe { bpf_assert(bpf_cmp_unlikely(num, num >= INT_MIN, INT_MIN)) };
    unsafe { *(num as *const u64) as i32 }
}

// SEC("?tc") __log_level(2) __failure
// __msg(": R1=ctx() R2=scalar(smin=0xffffffff80000002,smax=smax32=0x7ffffffd,smin32=0x80000002) R10=fp0")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_range_s64(ctx: *mut __sk_buff) -> i32 {
    let sk: *mut bpf_sock = unsafe { (*ctx).sk };
    let num: s64;

    // _Static_assert(_Generic((sk->rx_queue_mapping), s32: 1, default: 0), "type match");
    if sk.is_null() {
        return 0;
    }
    num = unsafe { (*sk).rx_queue_mapping as s64 };
    unsafe { bpf_assert_range(num, INT_MIN + 2, INT_MAX - 2) };
    unsafe { *((ctx as *mut u8).offset(num as isize)) as i32 }
}

// SEC("?tc") __log_level(2) __failure
// __msg(": R1=ctx() R2=scalar(smin=umin=smin32=umin32=4096,smax=umax=smax32=umax32=8192,var_off=(0x0; 0x3fff))")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_range_u64(ctx: *mut __sk_buff) -> i32 {
    let num: u64 = unsafe { (*ctx).len as u64 };

    unsafe { bpf_assert_range(num, 4096, 8192) };
    unsafe { *((ctx as *mut u8).offset(num as isize)) as i32 }
}

// SEC("?tc") __log_level(2) __failure
// __msg(": R1=ctx() R2=4096 R10=fp0")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_single_range_s64(ctx: *mut __sk_buff) -> i32 {
    let sk: *mut bpf_sock = unsafe { (*ctx).sk };
    let num: s64;

    // _Static_assert(_Generic((sk->rx_queue_mapping), s32: 1, default: 0), "type match");
    if sk.is_null() {
        return 0;
    }
    num = unsafe { (*sk).rx_queue_mapping as s64 };

    unsafe { bpf_assert_range(num, 4096, 4096) };
    unsafe { *((ctx as *mut u8).offset(num as isize)) as i32 }
}

// SEC("?tc") __log_level(2) __failure
// __msg(": R1=ctx() R2=4096 R10=fp0")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_single_range_u64(ctx: *mut __sk_buff) -> i32 {
    let num: u64 = unsafe { (*ctx).len as u64 };

    unsafe { bpf_assert_range(num, 4096, 4096) };
    unsafe { *((ctx as *mut u8).offset(num as isize)) as i32 }
}

// SEC("?tc") __log_level(2) __failure
// __msg(": R6=pkt(r=64) R10=fp0")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_generic(ctx: *mut __sk_buff) -> i32 {
    let data_end: *mut u8 = unsafe { (*ctx).data_end as usize as *mut u8 };
    let data: *mut u8 = unsafe { (*ctx).data as usize as *mut u8 };

    unsafe { bpf_assert(data.offset(64) <= data_end) };
    unsafe { *data.offset(128) as i32 }
}

// SEC("?fentry/bpf_check")
// __failure __msg("At program exit the register R1 has smin=64 smax=64")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_assert_with_return(ctx: *mut core::ffi::c_void) -> i32 {
    unsafe { bpf_assert_with(ctx.is_null(), 64) };
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
