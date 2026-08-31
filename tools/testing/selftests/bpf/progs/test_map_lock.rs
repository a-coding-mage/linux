// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C includes translated as external dependencies:
// linux/bpf.h, linux/version.h, bpf/bpf_helpers.h

pub const VAR_NUM: usize = 16;
pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct bpf_spin_lock {
    pub val: u32,
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hmap_elem {
    pub lock: bpf_spin_lock,
    pub var: [i32; VAR_NUM],
}

#[repr(C)]
pub struct bpf_map_def_hash_map {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut hash_map: bpf_map_def_hash_map = bpf_map_def_hash_map {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<hmap_elem>() as u32,
};

#[repr(C)]
pub struct array_elem {
    pub lock: bpf_spin_lock,
    pub var: [i32; VAR_NUM],
}

#[repr(C)]
pub struct bpf_map_def_array_map {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut array_map: bpf_map_def_array_map = bpf_map_def_array_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<array_elem>() as u32,
};

unsafe extern "C" {
    pub fn bpf_get_prandom_u32() -> u32;
    pub fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    pub fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
}

#[unsafe(link_section = "cgroup/skb")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_map_lock_test(skb: *mut __sk_buff) -> i32 {
    let mut val: *mut hmap_elem;
    let rnd: i32 = unsafe { bpf_get_prandom_u32() as i32 };
    let key: i32 = 0;
    let mut err: i32 = 1;
    let mut i: i32;
    let mut q: *mut array_elem;

    let _ = skb;

    val = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(hash_map) as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut hmap_elem
    };
    if val.is_null() {
        return err;
    }

    /* spin_lock in hash map */
    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!((*val).lock));
    }
    i = 0;
    while i < VAR_NUM as i32 {
        unsafe {
            (*val).var[i as usize] = rnd;
        }
        i += 1;
    }
    unsafe {
        bpf_spin_unlock(core::ptr::addr_of_mut!((*val).lock));
    }

    /* spin_lock in array */
    q = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(array_map) as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut array_elem
    };
    if q.is_null() {
        return err;
    }
    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!((*q).lock));
    }
    i = 0;
    while i < VAR_NUM as i32 {
        unsafe {
            (*q).var[i as usize] = rnd;
        }
        i += 1;
    }
    unsafe {
        bpf_spin_unlock(core::ptr::addr_of_mut!((*q).lock));
    }
    err = 0;

    err
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
