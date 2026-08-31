// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024-2025 Meta Platforms, Inc. and affiliates. */
// Dependencies from the original C file:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, "bpf_misc.h", "bpf_experimental.h"

#[repr(C)]
pub struct arr_elem {
    pub lock: bpf_res_spin_lock,
}

// Original: arrmap SEC(".maps") with type BPF_MAP_TYPE_ARRAY, max_entries 1,
// key int, value struct arr_elem.
#[no_mangle]
#[link_section = ".maps"]
pub static mut arrmap: bpf_map_def_arrmap = bpf_map_def_arrmap {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<arr_elem>() as u32,
};

#[no_mangle]
pub static mut value: i64 = 0;

#[no_mangle]
#[link_section = ".data.A"]
pub static mut lock: bpf_spin_lock = bpf_spin_lock {};

#[no_mangle]
#[link_section = ".data.B"]
pub static mut res_lock: bpf_res_spin_lock = bpf_res_spin_lock {};

// SEC("?tc")
// __failure __msg("point to map value or allocated object")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_arg(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    bpf_res_spin_lock(bpf_core_cast(
        &raw mut (*elem).lock as *mut bpf_res_spin_lock as *mut core::ffi::c_void,
    ) as *mut bpf_res_spin_lock);
    bpf_res_spin_lock(&raw mut (*elem).lock);
    let _ = ctx;
    0
}

// SEC("?tc")
// __failure __msg("AA deadlock detected")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_AA(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    bpf_res_spin_lock(&raw mut (*elem).lock);
    bpf_res_spin_lock(&raw mut (*elem).lock);
    let _ = ctx;
    0
}

// SEC("?tc")
// __failure __msg("AA deadlock detected")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_cond_AA(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    if bpf_res_spin_lock(&raw mut (*elem).lock) != 0 {
        return 0;
    }
    bpf_res_spin_lock(&raw mut (*elem).lock);
    let _ = ctx;
    0
}

// SEC("?tc")
// __failure __msg("unlock of different lock")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_mismatch_1(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    if bpf_res_spin_lock(&raw mut (*elem).lock) != 0 {
        return 0;
    }
    bpf_res_spin_unlock(&raw mut res_lock);
    let _ = ctx;
    0
}

// SEC("?tc")
// __failure __msg("unlock of different lock")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_mismatch_2(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    if bpf_res_spin_lock(&raw mut res_lock) != 0 {
        return 0;
    }
    bpf_res_spin_unlock(&raw mut (*elem).lock);
    let _ = ctx;
    0
}

// SEC("?tc")
// __failure __msg("unlock of different lock")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_irq_mismatch_1(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;
    let mut f1: core::ffi::c_ulong = 0;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    bpf_local_irq_save(&mut f1);
    if bpf_res_spin_lock(&raw mut res_lock) != 0 {
        return 0;
    }
    bpf_res_spin_unlock_irqrestore(&raw mut res_lock, &mut f1);
    let _ = ctx;
    0
}

// SEC("?tc")
// __failure __msg("unlock of different lock")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_irq_mismatch_2(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;
    let mut f1: core::ffi::c_ulong = 0;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    if bpf_res_spin_lock_irqsave(&raw mut res_lock, &mut f1) != 0 {
        return 0;
    }
    bpf_res_spin_unlock(&raw mut res_lock);
    let _ = ctx;
    0
}

// SEC("?tc")
// __success
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_ooo(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    if bpf_res_spin_lock(&raw mut res_lock) != 0 {
        return 0;
    }
    if bpf_res_spin_lock(&raw mut (*elem).lock) != 0 {
        bpf_res_spin_unlock(&raw mut res_lock);
        return 0;
    }
    bpf_res_spin_unlock(&raw mut (*elem).lock);
    bpf_res_spin_unlock(&raw mut res_lock);
    let _ = ctx;
    0
}

// SEC("?tc")
// __success
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_ooo_irq(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;
    let mut f1: core::ffi::c_ulong = 0;
    let mut f2: core::ffi::c_ulong = 0;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    if bpf_res_spin_lock_irqsave(&raw mut res_lock, &mut f1) != 0 {
        return 0;
    }
    if bpf_res_spin_lock_irqsave(&raw mut (*elem).lock, &mut f2) != 0 {
        bpf_res_spin_unlock_irqrestore(&raw mut res_lock, &mut f1);
        /* We won't have a unreleased IRQ flag error here. */
        return 0;
    }
    bpf_res_spin_unlock_irqrestore(&raw mut (*elem).lock, &mut f2);
    bpf_res_spin_unlock_irqrestore(&raw mut res_lock, &mut f1);
    let _ = ctx;
    0
}

#[no_mangle]
#[link_section = ".data.OO1"]
pub static mut lock1: bpf_res_spin_lock = bpf_res_spin_lock {};

#[no_mangle]
#[link_section = ".data.OO2"]
pub static mut lock2: bpf_res_spin_lock = bpf_res_spin_lock {};

// SEC("?tc")
// __failure __msg("bpf_res_spin_unlock cannot be out of order")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_ooo_unlock(ctx: *mut __sk_buff) -> i32 {
    if bpf_res_spin_lock(&raw mut lock1) != 0 {
        return 0;
    }
    if bpf_res_spin_lock(&raw mut lock2) != 0 {
        bpf_res_spin_unlock(&raw mut lock1);
        return 0;
    }
    bpf_res_spin_unlock(&raw mut lock1);
    bpf_res_spin_unlock(&raw mut lock2);
    let _ = ctx;
    0
}

// SEC("?tc")
// __failure __msg("off 1 doesn't point to 'struct bpf_res_spin_lock' that is at 0")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_bad_off(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        return 0;
    }
    bpf_res_spin_lock((&raw mut (*elem).lock as *mut core::ffi::c_void).add(1) as *mut bpf_res_spin_lock);
    let _ = ctx;
    0
}

// SEC("?tc")
// __failure __msg("R1 doesn't have constant offset. bpf_res_spin_lock has to be at the constant offset")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_var_off(ctx: *mut __sk_buff) -> i32 {
    let mut elem: *mut arr_elem;
    let val: u64 = value as u64;

    elem = bpf_map_lookup_elem(
        &raw mut arrmap as *mut _ as *mut core::ffi::c_void,
        &0i32 as *const _ as *const core::ffi::c_void,
    ) as *mut arr_elem;
    if elem.is_null() {
        // FIXME: Only inline assembly use in assert macro doesn't emit
        //        BTF definition.
        bpf_throw(0);
        return 0;
    }
    bpf_assert_range(val, 0, 40);
    bpf_res_spin_lock((&raw mut value as *mut core::ffi::c_void).add(val as usize) as *mut bpf_res_spin_lock);
    let _ = ctx;
    0
}

// SEC("?tc")
// __failure __msg("map 'res_spin.bss' has no valid bpf_res_spin_lock")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_no_lock_map(ctx: *mut __sk_buff) -> i32 {
    bpf_res_spin_lock((&raw mut value as *mut core::ffi::c_void).add(1) as *mut bpf_res_spin_lock);
    let _ = ctx;
    0
}

#[repr(C)]
pub struct res_spin_lock_no_lock_kptr_anon {
    pub i: i32,
}

// SEC("?tc")
// __failure __msg("local 'kptr' has no valid bpf_res_spin_lock")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn res_spin_lock_no_lock_kptr(ctx: *mut __sk_buff) -> i32 {
    let p: *mut res_spin_lock_no_lock_kptr_anon = bpf_obj_new::<res_spin_lock_no_lock_kptr_anon>();

    if p.is_null() {
        return 0;
    }
    bpf_res_spin_lock(p as *mut core::ffi::c_void as *mut bpf_res_spin_lock);
    let _ = ctx;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
