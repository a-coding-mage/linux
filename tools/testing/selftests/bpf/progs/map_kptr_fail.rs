// SPDX-License-Identifier: GPL-2.0
// C dependencies omitted from executable Rust:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, "bpf_misc.h",
// "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct __sk_buff {
    pub protocol: i32,
}

#[repr(C)]
pub struct prog_test_member {
    _private: [u8; 0],
}

#[repr(C)]
pub struct prog_test_ref_kfunc {
    pub next: *mut prog_test_ref_kfunc,
    pub memb: prog_test_member,
}

#[repr(C)]
pub struct map_value {
    pub buf: [i8; 8],
    pub unref_ptr: *mut prog_test_ref_kfunc,
    pub ref_ptr: *mut prog_test_ref_kfunc,
    pub ref_memb_ptr: *mut prog_test_member,
}

#[repr(C)]
pub struct array_map {
    _private: [u8; 0],
}

// Original C map definition:
// struct array_map {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __type(key, int);
//     __type(value, struct map_value);
//     __uint(max_entries, 1);
// } array_map SEC(".maps");
#[link_section = ".maps"]
#[no_mangle]
pub static mut array_map: array_map = array_map { _private: [] };

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut array_map, key: *const i32) -> *mut map_value;
    fn bpf_kptr_xchg(kptr: *mut core::ffi::c_void, ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_this_cpu_ptr(ptr: *mut prog_test_ref_kfunc) -> *mut core::ffi::c_void;
    fn bpf_get_current_comm(buf: *mut core::ffi::c_void, size_of_buf: u32) -> i32;
    fn bpf_kfunc_call_test_acquire(arg: *mut u64) -> *mut prog_test_ref_kfunc;
    fn bpf_kfunc_call_test_release(p: *mut prog_test_ref_kfunc);
}

// SEC("?tc")
// __failure __msg("kptr access size must be BPF_DW")
#[no_mangle]
pub unsafe extern "C" fn size_not_bpf_dw(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    *((&raw mut (*v).unref_ptr) as *mut u32) = 0;
    return 0;
}

// SEC("?tc")
// __failure __msg("kptr access cannot have variable offset")
#[no_mangle]
pub unsafe extern "C" fn non_const_var_off(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;
    let id: i32;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    id = (*ctx).protocol;
    if id < 4 || id > 12 {
        return 0;
    }
    *((v as *mut core::ffi::c_void).byte_add(id as usize) as *mut u64) = 0;

    return 0;
}

// SEC("?tc")
// __failure __msg("R1 doesn't have constant offset. kptr has to be")
#[no_mangle]
pub unsafe extern "C" fn non_const_var_off_kptr_xchg(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;
    let id: i32;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    id = (*ctx).protocol;
    if id < 4 || id > 12 {
        return 0;
    }
    bpf_kptr_xchg((v as *mut core::ffi::c_void).byte_add(id as usize), core::ptr::null_mut());

    return 0;
}

// SEC("?tc")
// __failure __msg("kptr access misaligned expected=8 off=7")
#[no_mangle]
pub unsafe extern "C" fn misaligned_access_write(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    *((v as *mut core::ffi::c_void).byte_add(7) as *mut *mut core::ffi::c_void) = core::ptr::null_mut();

    return 0;
}

// SEC("?tc")
// __failure __msg("kptr access misaligned expected=8 off=1")
#[no_mangle]
pub unsafe extern "C" fn misaligned_access_read(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    return *((v as *mut core::ffi::c_void).byte_add(1) as *mut u64) as i32;
}

// SEC("?tc")
// __failure __msg("variable untrusted_ptr_ access var_off=(0x0; 0x1e0)")
#[no_mangle]
pub unsafe extern "C" fn reject_var_off_store(ctx: *mut __sk_buff) -> i32 {
    let mut unref_ptr: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;
    let id: i32;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    unref_ptr = (*v).unref_ptr;
    if unref_ptr.is_null() {
        return 0;
    }
    id = (*ctx).protocol;
    if id < 4 || id > 12 {
        return 0;
    }
    unref_ptr = unref_ptr.offset(id as isize);
    (*v).unref_ptr = unref_ptr;

    return 0;
}

// SEC("?tc")
// __failure __msg("invalid kptr access, R1 type=untrusted_ptr_prog_test_ref_kfunc")
#[no_mangle]
pub unsafe extern "C" fn reject_bad_type_match(ctx: *mut __sk_buff) -> i32 {
    let mut unref_ptr: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    unref_ptr = (*v).unref_ptr;
    if unref_ptr.is_null() {
        return 0;
    }
    unref_ptr = (unref_ptr as *mut core::ffi::c_void).byte_add(4) as *mut prog_test_ref_kfunc;
    (*v).unref_ptr = unref_ptr;

    return 0;
}

// SEC("?tc")
// __failure __msg("R1 type=untrusted_ptr_or_null_ expected=percpu_ptr_")
#[no_mangle]
pub unsafe extern "C" fn marked_as_untrusted_or_null(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    bpf_this_cpu_ptr((*v).unref_ptr);
    return 0;
}

// SEC("?tc")
// __failure __msg("access beyond struct prog_test_ref_kfunc at off 32 size 4")
#[no_mangle]
pub unsafe extern "C" fn correct_btf_id_check_size(ctx: *mut __sk_buff) -> i32 {
    let mut p: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    p = (*v).unref_ptr;
    if p.is_null() {
        return 0;
    }
    return *((p as *mut core::ffi::c_void).byte_add(core::mem::size_of::<prog_test_ref_kfunc>()) as *mut i32);
}

// SEC("?tc")
// __failure __msg("R1 type=untrusted_ptr_ expected=percpu_ptr_")
#[no_mangle]
pub unsafe extern "C" fn inherit_untrusted_on_walk(ctx: *mut __sk_buff) -> i32 {
    let mut unref_ptr: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    unref_ptr = (*v).unref_ptr;
    if unref_ptr.is_null() {
        return 0;
    }
    unref_ptr = (*unref_ptr).next;
    bpf_this_cpu_ptr(unref_ptr);
    return 0;
}

// SEC("?tc")
// __failure __msg("off=8 kptr isn't referenced kptr")
#[no_mangle]
pub unsafe extern "C" fn reject_kptr_xchg_on_unref(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    bpf_kptr_xchg((&raw mut (*v).unref_ptr) as *mut core::ffi::c_void, core::ptr::null_mut());
    return 0;
}

// SEC("?tc")
// __failure __msg("R1 type=rcu_ptr_or_null_ expected=percpu_ptr_")
#[no_mangle]
pub unsafe extern "C" fn mark_ref_as_untrusted_or_null(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    bpf_this_cpu_ptr((*v).ref_ptr);
    return 0;
}

// SEC("?tc")
// __failure __msg("store to referenced kptr disallowed")
#[no_mangle]
pub unsafe extern "C" fn reject_untrusted_store_to_ref(ctx: *mut __sk_buff) -> i32 {
    let mut p: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    p = (*v).ref_ptr;
    if p.is_null() {
        return 0;
    }
    /* Checkmate, clang */
    core::ptr::write_volatile((&raw mut (*v).ref_ptr) as *mut *mut prog_test_ref_kfunc, p);
    return 0;
}

// SEC("?tc")
// __failure __msg("release helper bpf_kptr_xchg expects referenced PTR_TO_BTF_ID passed to R2")
#[no_mangle]
pub unsafe extern "C" fn reject_untrusted_xchg(ctx: *mut __sk_buff) -> i32 {
    let mut p: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    p = (*v).ref_ptr;
    if p.is_null() {
        return 0;
    }
    bpf_kptr_xchg((&raw mut (*v).ref_ptr) as *mut core::ffi::c_void, p as *mut core::ffi::c_void);
    return 0;
}

// SEC("?tc")
// __failure
// __msg("invalid kptr access, R2 type=trusted_ptr_prog_test_ref_kfunc expected=ptr_prog_test_member")
#[no_mangle]
pub unsafe extern "C" fn reject_bad_type_xchg(ctx: *mut __sk_buff) -> i32 {
    let mut ref_ptr: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;
    let mut compound_literal: u64 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    ref_ptr = bpf_kfunc_call_test_acquire(&mut compound_literal);
    if ref_ptr.is_null() {
        return 0;
    }
    bpf_kptr_xchg((&raw mut (*v).ref_memb_ptr) as *mut core::ffi::c_void, ref_ptr as *mut core::ffi::c_void);
    return 0;
}

// SEC("?tc")
// __failure __msg("invalid kptr access, R2 type=trusted_ptr_prog_test_ref_kfunc")
#[no_mangle]
pub unsafe extern "C" fn reject_member_of_ref_xchg(ctx: *mut __sk_buff) -> i32 {
    let mut ref_ptr: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;
    let mut compound_literal: u64 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    ref_ptr = bpf_kfunc_call_test_acquire(&mut compound_literal);
    if ref_ptr.is_null() {
        return 0;
    }
    bpf_kptr_xchg((&raw mut (*v).ref_memb_ptr) as *mut core::ffi::c_void, (&raw mut (*ref_ptr).memb) as *mut core::ffi::c_void);
    return 0;
}

// SEC("?syscall")
// __failure __msg("kptr cannot be accessed indirectly by helper")
#[no_mangle]
pub unsafe extern "C" fn reject_indirect_helper_access(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    bpf_get_current_comm(v as *mut core::ffi::c_void, (core::mem::size_of_val(&(*v).buf) + 1) as u32);
    return 0;
}

// __noinline
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn write_func(p: *mut i32) -> i32 {
    return if !p.is_null() {
        *p = 42;
        *p
    } else {
        0
    };
}

// SEC("?tc")
// __failure __msg("kptr cannot be accessed indirectly by helper")
#[no_mangle]
pub unsafe extern "C" fn reject_indirect_global_func_access(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    return write_func((v as *mut core::ffi::c_void).byte_add(5) as *mut i32);
}

// SEC("?tc")
// __failure __msg("Unreleased reference id=4 alloc_insn=")
#[no_mangle]
pub unsafe extern "C" fn kptr_xchg_ref_state(ctx: *mut __sk_buff) -> i32 {
    let mut p: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;
    let mut compound_literal: u64 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    p = bpf_kfunc_call_test_acquire(&mut compound_literal);
    if p.is_null() {
        return 0;
    }
    bpf_kptr_xchg((&raw mut (*v).ref_ptr) as *mut core::ffi::c_void, p as *mut core::ffi::c_void);
    return 0;
}

// SEC("?tc")
// __failure __msg("Possibly NULL pointer passed to helper R2")
#[no_mangle]
pub unsafe extern "C" fn kptr_xchg_possibly_null(ctx: *mut __sk_buff) -> i32 {
    let mut p: *mut prog_test_ref_kfunc;
    let mut v: *mut map_value;
    let key: i32 = 0;
    let mut compound_literal: u64 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    p = bpf_kfunc_call_test_acquire(&mut compound_literal);

    /* PTR_TO_BTF_ID | PTR_MAYBE_NULL passed to bpf_kptr_xchg() */
    p = bpf_kptr_xchg((&raw mut (*v).ref_ptr) as *mut core::ffi::c_void, p as *mut core::ffi::c_void) as *mut prog_test_ref_kfunc;
    if !p.is_null() {
        bpf_kfunc_call_test_release(p);
    }

    return 0;
}

// SEC("?tc")
/*
 * A compiler with BPF_ST folds the constant into a store-immediate, which the
 * verifier rejects on a different path (and with a different message) than the
 * BPF_STX form.
 */
// Original build-time condition:
// #ifdef __BPF_FEATURE_ST
// __failure __msg("BPF_ST imm must be 0 when storing to kptr at off=8")
// #else
// __failure __msg("invalid kptr access, R")
// #endif
#[no_mangle]
pub unsafe extern "C" fn reject_scalar_store_to_kptr(ctx: *mut __sk_buff) -> i32 {
    let mut v: *mut map_value;
    let key: i32 = 0;

    v = bpf_map_lookup_elem(&raw mut array_map, &key);
    if v.is_null() {
        return 0;
    }

    core::ptr::write_volatile((&raw mut (*v).unref_ptr) as *mut u64, 0xBADC0DE);
    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [i8; 4] = [b'G' as i8, b'P' as i8, b'L' as i8, 0];
