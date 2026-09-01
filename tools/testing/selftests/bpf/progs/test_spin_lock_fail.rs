// SPDX-License-Identifier: GPL-2.0
// Translated from C source using vmlinux.h, bpf_tracing.h, bpf_helpers.h,
// and bpf_experimental.h dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_void};
use core::ptr;

#[repr(C)]
pub struct bpf_spin_lock {
    pub val: u32,
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub pkt_type: u32,
    pub mark: u32,
    pub queue_mapping: u32,
    pub protocol: u32,
}

#[repr(C)]
pub struct foo {
    pub lock: bpf_spin_lock,
    pub data: i32,
}

#[repr(C)]
pub struct array_map {
    pub _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_obj_new(arg: *const c_void) -> *mut foo;
    fn bpf_obj_drop(ptr: *mut foo);
    fn bpf_this_cpu_ptr(ptr: *mut c_void) -> *mut c_void;
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_printk(fmt: *const c_char, ...) -> i32;
    fn bpf_copy_from_user(dst: *mut c_void, size: u32, src: *const c_void) -> i32;
    fn bpf_copy_from_user_str(dst: *mut c_void, size: u32, src: *const c_void, flags: u64) -> i32;

    static mut array_map: array_map;
    static mut map_of_maps: c_void;
}

// static struct bpf_spin_lock lockA SEC(".data.A");
#[unsafe(link_section = ".data.A")]
static mut lockA: bpf_spin_lock = bpf_spin_lock { val: 0 };

// static struct bpf_spin_lock lockB SEC(".data.B");
#[unsafe(link_section = ".data.B")]
static mut lockB: bpf_spin_lock = bpf_spin_lock { val: 0 };

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_id_kptr_preserve(ctx: *mut c_void) -> i32 {
    let mut f: *mut foo;

    f = unsafe { bpf_obj_new(ptr::null()) };
    if f.is_null() {
        return 0;
    }
    unsafe { bpf_this_cpu_ptr(f as *mut c_void) };
    0
}

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_id_global_zero(ctx: *mut c_void) -> i32 {
    unsafe { bpf_this_cpu_ptr(&raw mut lockA as *mut c_void) };
    0
}

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_id_mapval_preserve(ctx: *mut c_void) -> i32 {
    let mut f: *mut foo;
    let mut key: i32 = 0;

    f = unsafe { bpf_map_lookup_elem(&raw mut array_map as *const c_void, &raw const key as *const c_void) as *mut foo };
    if f.is_null() {
        return 0;
    }
    unsafe { bpf_this_cpu_ptr(f as *mut c_void) };
    0
}

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_id_innermapval_preserve(ctx: *mut c_void) -> i32 {
    let mut f: *mut foo;
    let mut key: i32 = 0;
    let mut map: *mut c_void;

    map = unsafe { bpf_map_lookup_elem(&raw mut map_of_maps as *const c_void, &raw const key as *const c_void) };
    if map.is_null() {
        return 0;
    }
    f = unsafe { bpf_map_lookup_elem(map as *const c_void, &raw const key as *const c_void) as *mut foo };
    if f.is_null() {
        return 0;
    }
    unsafe { bpf_this_cpu_ptr(f as *mut c_void) };
    0
}

macro_rules! check {
    ($name:ident, $A:expr, $B:expr) => {
        // SEC("?tc")
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(ctx: *mut c_void) -> i32 {
            let mut f1: *mut foo;
            let mut f2: *mut foo;
            let mut v: *mut foo;
            let mut iv: *mut foo;
            let mut key: i32 = 0;
            let mut map: *mut c_void;

            map = unsafe { bpf_map_lookup_elem(&raw mut map_of_maps as *const c_void, &raw const key as *const c_void) };
            if map.is_null() {
                return 0;
            }
            iv = unsafe { bpf_map_lookup_elem(map as *const c_void, &raw const key as *const c_void) as *mut foo };
            if iv.is_null() {
                return 0;
            }
            v = unsafe { bpf_map_lookup_elem(&raw mut array_map as *const c_void, &raw const key as *const c_void) as *mut foo };
            if v.is_null() {
                return 0;
            }
            f1 = unsafe { bpf_obj_new(ptr::null()) };
            if f1.is_null() {
                return 0;
            }
            f2 = unsafe { bpf_obj_new(ptr::null()) };
            if f2.is_null() {
                unsafe { bpf_obj_drop(f1) };
                return 0;
            }
            unsafe { bpf_spin_lock($A) };
            unsafe { bpf_spin_unlock($B) };
            0
        }
    };
}

check!(lock_id_mismatch_kptr_kptr, unsafe { &raw mut (*f1).lock }, unsafe { &raw mut (*f2).lock });
check!(lock_id_mismatch_kptr_global, unsafe { &raw mut (*f1).lock }, &raw mut lockA);
check!(lock_id_mismatch_kptr_mapval, unsafe { &raw mut (*f1).lock }, unsafe { &raw mut (*v).lock });
check!(lock_id_mismatch_kptr_innermapval, unsafe { &raw mut (*f1).lock }, unsafe { &raw mut (*iv).lock });

check!(lock_id_mismatch_global_global, &raw mut lockA, &raw mut lockB);
check!(lock_id_mismatch_global_kptr, &raw mut lockA, unsafe { &raw mut (*f1).lock });
check!(lock_id_mismatch_global_mapval, &raw mut lockA, unsafe { &raw mut (*v).lock });
check!(lock_id_mismatch_global_innermapval, &raw mut lockA, unsafe { &raw mut (*iv).lock });

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_id_mismatch_mapval_mapval(ctx: *mut c_void) -> i32 {
    let mut f1: *mut foo;
    let mut f2: *mut foo;
    let mut key: i32 = 0;

    f1 = unsafe { bpf_map_lookup_elem(&raw mut array_map as *const c_void, &raw const key as *const c_void) as *mut foo };
    if f1.is_null() {
        return 0;
    }
    f2 = unsafe { bpf_map_lookup_elem(&raw mut array_map as *const c_void, &raw const key as *const c_void) as *mut foo };
    if f2.is_null() {
        return 0;
    }

    unsafe { bpf_spin_lock(&raw mut (*f1).lock) };
    unsafe { (*f1).data = 42 };
    unsafe { bpf_spin_unlock(&raw mut (*f2).lock) };

    0
}

check!(lock_id_mismatch_mapval_kptr, unsafe { &raw mut (*v).lock }, unsafe { &raw mut (*f1).lock });
check!(lock_id_mismatch_mapval_global, unsafe { &raw mut (*v).lock }, &raw mut lockB);
check!(lock_id_mismatch_mapval_innermapval, unsafe { &raw mut (*v).lock }, unsafe { &raw mut (*iv).lock });

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_id_mismatch_innermapval_innermapval1(ctx: *mut c_void) -> i32 {
    let mut f1: *mut foo;
    let mut f2: *mut foo;
    let mut key: i32 = 0;
    let mut map: *mut c_void;

    map = unsafe { bpf_map_lookup_elem(&raw mut map_of_maps as *const c_void, &raw const key as *const c_void) };
    if map.is_null() {
        return 0;
    }
    f1 = unsafe { bpf_map_lookup_elem(map as *const c_void, &raw const key as *const c_void) as *mut foo };
    if f1.is_null() {
        return 0;
    }
    f2 = unsafe { bpf_map_lookup_elem(map as *const c_void, &raw const key as *const c_void) as *mut foo };
    if f2.is_null() {
        return 0;
    }

    unsafe { bpf_spin_lock(&raw mut (*f1).lock) };
    unsafe { (*f1).data = 42 };
    unsafe { bpf_spin_unlock(&raw mut (*f2).lock) };

    0
}

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_id_mismatch_innermapval_innermapval2(ctx: *mut c_void) -> i32 {
    let mut f1: *mut foo;
    let mut f2: *mut foo;
    let mut key: i32 = 0;
    let mut map: *mut c_void;

    map = unsafe { bpf_map_lookup_elem(&raw mut map_of_maps as *const c_void, &raw const key as *const c_void) };
    if map.is_null() {
        return 0;
    }
    f1 = unsafe { bpf_map_lookup_elem(map as *const c_void, &raw const key as *const c_void) as *mut foo };
    if f1.is_null() {
        return 0;
    }
    map = unsafe { bpf_map_lookup_elem(&raw mut map_of_maps as *const c_void, &raw const key as *const c_void) };
    if map.is_null() {
        return 0;
    }
    f2 = unsafe { bpf_map_lookup_elem(map as *const c_void, &raw const key as *const c_void) as *mut foo };
    if f2.is_null() {
        return 0;
    }

    unsafe { bpf_spin_lock(&raw mut (*f1).lock) };
    unsafe { (*f1).data = 42 };
    unsafe { bpf_spin_unlock(&raw mut (*f2).lock) };

    0
}

check!(lock_id_mismatch_innermapval_kptr, unsafe { &raw mut (*iv).lock }, unsafe { &raw mut (*f1).lock });
check!(lock_id_mismatch_innermapval_global, unsafe { &raw mut (*iv).lock }, &raw mut lockA);
check!(lock_id_mismatch_innermapval_mapval, unsafe { &raw mut (*iv).lock }, unsafe { &raw mut (*v).lock });

#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_subprog(ctx: *mut __sk_buff) -> i32 {
    let mut ret: i32 = 0;

    if unsafe { (*ctx).protocol } != 0 {
        unsafe { ptr::write_volatile(&mut ret, ptr::read_volatile(&ret).wrapping_add((*ctx).protocol as i32)) };
    }
    unsafe { ptr::read_volatile(&ret).wrapping_add((*ctx).mark as i32) }
}

unsafe extern "C" fn static_subprog_call_global(ctx: *mut __sk_buff) -> i32 {
    let mut ret: i32 = 0;

    if unsafe { (*ctx).protocol } != 0 {
        return unsafe { ptr::read_volatile(&ret) };
    }
    unsafe { ptr::read_volatile(&ret).wrapping_add((*ctx).len as i32).wrapping_add(global_subprog(ctx)) }
}

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_global_subprog_call1(ctx: *mut __sk_buff) -> i32 {
    let mut ret: i32 = 0;

    unsafe { bpf_spin_lock(&raw mut lockA) };
    if unsafe { (*ctx).mark } == 42 {
        ret = unsafe { global_subprog(ctx) };
    }
    unsafe { bpf_spin_unlock(&raw mut lockA) };
    ret
}

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_global_subprog_call2(ctx: *mut __sk_buff) -> i32 {
    let mut ret: i32 = 0;

    unsafe { bpf_spin_lock(&raw mut lockA) };
    if unsafe { (*ctx).mark } == 42 {
        ret = unsafe { static_subprog_call_global(ctx) };
    }
    unsafe { bpf_spin_unlock(&raw mut lockA) };
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_subprog_int(i: i32) -> i32 {
    let mut i = i;

    if i != 0 {
        unsafe { bpf_printk(c"%p".as_ptr(), &raw mut i) };
    }
    i
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_sleepable_helper_subprog(i: i32) -> i32 {
    let mut i = i;

    if i != 0 {
        unsafe { bpf_copy_from_user(&raw mut i as *mut c_void, core::mem::size_of_val(&i) as u32, ptr::null()) };
    }
    i
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_sleepable_kfunc_subprog(i: i32) -> i32 {
    let mut i = i;

    if i != 0 {
        unsafe {
            bpf_copy_from_user_str(
                &raw mut i as *mut c_void,
                core::mem::size_of_val(&i) as u32,
                ptr::null(),
                0,
            )
        };
    }
    unsafe { global_subprog_int(i) };
    i
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_subprog_calling_sleepable_global(i: i32) -> i32 {
    let mut i = i;

    if i == 0 {
        unsafe { global_sleepable_kfunc_subprog(i) };
    }
    i
}

// SEC("?syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_global_sleepable_helper_subprog(ctx: *mut __sk_buff) -> i32 {
    let mut ret: i32 = 0;

    unsafe { bpf_spin_lock(&raw mut lockA) };
    if unsafe { (*ctx).mark } == 42 {
        ret = unsafe { global_sleepable_helper_subprog((*ctx).mark as i32) };
    }
    unsafe { bpf_spin_unlock(&raw mut lockA) };
    ret
}

// SEC("?syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_global_sleepable_kfunc_subprog(ctx: *mut __sk_buff) -> i32 {
    let mut ret: i32 = 0;

    unsafe { bpf_spin_lock(&raw mut lockA) };
    if unsafe { (*ctx).mark } == 42 {
        ret = unsafe { global_sleepable_kfunc_subprog((*ctx).mark as i32) };
    }
    unsafe { bpf_spin_unlock(&raw mut lockA) };
    ret
}

// SEC("?syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_global_sleepable_subprog_indirect(ctx: *mut __sk_buff) -> i32 {
    let mut ret: i32 = 0;

    unsafe { bpf_spin_lock(&raw mut lockA) };
    if unsafe { (*ctx).mark } == 42 {
        ret = unsafe { global_subprog_calling_sleepable_global((*ctx).mark as i32) };
    }
    unsafe { bpf_spin_unlock(&raw mut lockA) };
    ret
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
