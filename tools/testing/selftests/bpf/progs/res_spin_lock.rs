// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024-2025 Meta Platforms, Inc. and affiliates. */
// C includes translated as external dependencies:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

pub const EDEADLK: i32 = 35;
pub const ETIMEDOUT: i32 = 110;

#[repr(C)]
pub struct arr_elem {
    pub lock: bpf_res_spin_lock,
}

// Original C BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 64);
//     __type(key, int);
//     __type(value, struct arr_elem);
// } arrmap SEC(".maps");
#[repr(C)]
pub struct arrmap_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: arr_elem,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut arrmap: arrmap_def = arrmap_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 64,
    key: 0,
    value: arr_elem {
        lock: unsafe { core::mem::zeroed() },
    },
};

#[no_mangle]
#[link_section = ".data.A"]
pub static mut lockA: bpf_res_spin_lock = unsafe { core::mem::zeroed() };

#[no_mangle]
#[link_section = ".data.B"]
pub static mut lockB: bpf_res_spin_lock = unsafe { core::mem::zeroed() };

// Original section: SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_test(ctx: *mut __sk_buff) -> i32 {
    let mut elem1: *mut arr_elem;
    let mut elem2: *mut arr_elem;
    let mut r: i32;

    let key0: i32 = 0;
    elem1 = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(arrmap).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(key0).cast::<core::ffi::c_void>(),
    )
    .cast::<arr_elem>();
    if elem1.is_null() {
        return -1;
    }
    let key0_2: i32 = 0;
    elem2 = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(arrmap).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(key0_2).cast::<core::ffi::c_void>(),
    )
    .cast::<arr_elem>();
    if elem2.is_null() {
        return -1;
    }

    r = bpf_res_spin_lock(core::ptr::addr_of_mut!((*elem1).lock));
    if r != 0 {
        return r;
    }
    r = bpf_res_spin_lock(core::ptr::addr_of_mut!((*elem2).lock));
    if r == 0 {
        bpf_res_spin_unlock(core::ptr::addr_of_mut!((*elem2).lock));
        bpf_res_spin_unlock(core::ptr::addr_of_mut!((*elem1).lock));
        return -1;
    }
    bpf_res_spin_unlock(core::ptr::addr_of_mut!((*elem1).lock));
    (r != -EDEADLK) as i32
}

// Original section: SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_test_AB(ctx: *mut __sk_buff) -> i32 {
    let mut r: i32;

    r = bpf_res_spin_lock(core::ptr::addr_of_mut!(lockA));
    if r != 0 {
        return (r == 0) as i32;
    }
    /* Only unlock if we took the lock. */
    if bpf_res_spin_lock(core::ptr::addr_of_mut!(lockB)) == 0 {
        bpf_res_spin_unlock(core::ptr::addr_of_mut!(lockB));
    }
    bpf_res_spin_unlock(core::ptr::addr_of_mut!(lockA));
    0
}

#[no_mangle]
pub static mut err: i32 = 0;

// Original section: SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_test_BA(ctx: *mut __sk_buff) -> i32 {
    let mut r: i32;

    r = bpf_res_spin_lock(core::ptr::addr_of_mut!(lockB));
    if r != 0 {
        return (r == 0) as i32;
    }
    if bpf_res_spin_lock(core::ptr::addr_of_mut!(lockA)) == 0 {
        bpf_res_spin_unlock(core::ptr::addr_of_mut!(lockA));
    } else {
        err = -EDEADLK;
    }
    bpf_res_spin_unlock(core::ptr::addr_of_mut!(lockB));
    if err != 0 { err } else { 0 }
}

// Original section: SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_test_held_lock_max(ctx: *mut __sk_buff) -> i32 {
    let mut locks: [*mut bpf_res_spin_lock; 48] = [core::ptr::null_mut(); 48];
    let mut e: *mut arr_elem;
    let mut time_beg: u64;
    let mut time: u64;
    let mut ret: i32 = 0;
    let mut i: i32;

    // C static assertion:
    // _Static_assert(ARRAY_SIZE(((struct rqspinlock_held){}).locks) == 31,
    //                "RES_NR_HELD assumed to be 31");
    const _: [(); 31] = [(); core::mem::size_of::<rqspinlock_held_locks_len_marker>()];

    i = 0;
    while i < 34 {
        let key: i32 = i;

        /* We cannot pass in i as it will get spilled/filled by the compiler and
         * loses bounds in verifier state.
         */
        e = bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(arrmap).cast::<core::ffi::c_void>(),
            core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
        )
        .cast::<arr_elem>();
        if e.is_null() {
            return 1;
        }
        locks[i as usize] = core::ptr::addr_of_mut!((*e).lock);
        i += 1;
    }

    while i < 48 {
        let key: i32 = i - 2;

        /* We cannot pass in i as it will get spilled/filled by the compiler and
         * loses bounds in verifier state.
         */
        e = bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(arrmap).cast::<core::ffi::c_void>(),
            core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
        )
        .cast::<arr_elem>();
        if e.is_null() {
            return 1;
        }
        locks[i as usize] = core::ptr::addr_of_mut!((*e).lock);
        i += 1;
    }

    time_beg = bpf_ktime_get_ns();
    i = 0;
    while i < 34 {
        if bpf_res_spin_lock(locks[i as usize]) != 0 {
            goto_end(&mut i, &locks, time_beg, ret);
            return end(i, &locks, time_beg, ret);
        }
        i += 1;
    }

    /* Trigger AA, after exhausting entries in the held lock table. This
     * time, only the timeout can save us, as AA detection won't succeed.
     */
    ret = bpf_res_spin_lock(locks[34]);
    if ret == 0 {
        bpf_res_spin_unlock(locks[34]);
        ret = 1;
        return end(i, &locks, time_beg, ret);
    }

    ret = if ret != -ETIMEDOUT { 2 } else { 0 };

    end(i, &locks, time_beg, ret)
}

unsafe fn end(
    mut i: i32,
    locks: &[*mut bpf_res_spin_lock; 48],
    time_beg: u64,
    ret: i32,
) -> i32 {
    while {
        i -= 1;
        i >= 0
    } {
        bpf_res_spin_unlock(locks[i as usize]);
    }
    let time: u64 = bpf_ktime_get_ns().wrapping_sub(time_beg);
    /* Time spent should be easily above our limit (1/4 s), since AA
     * detection won't be expedited due to lack of held lock entry.
     */
    if ret != 0 {
        ret
    } else if time > 1000000000 / 4 {
        0
    } else {
        1
    }
}

unsafe fn goto_end(
    _i: &mut i32,
    _locks: &[*mut bpf_res_spin_lock; 48],
    _time_beg: u64,
    _ret: i32,
) {
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

extern "C" {
    pub type bpf_res_spin_lock;
    pub type __sk_buff;
    pub type rqspinlock_held;
    pub type rqspinlock_held_locks_len_marker;

    pub static BPF_MAP_TYPE_ARRAY: u32;

    pub fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_res_spin_lock(lock: *mut bpf_res_spin_lock) -> i32;
    pub fn bpf_res_spin_unlock(lock: *mut bpf_res_spin_lock);
    pub fn bpf_ktime_get_ns() -> u64;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
