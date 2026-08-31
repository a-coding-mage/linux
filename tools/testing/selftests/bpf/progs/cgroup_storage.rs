// SPDX-License-Identifier: GPL-2.0

// Source dependencies: <linux/bpf.h> and <bpf/bpf_helpers.h>

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_CGROUP_STORAGE: __u32 = 19;
const BPF_MAP_TYPE_LRU_PERCPU_HASH: __u32 = 10;
const BPF_ANY: __u64 = 0;

#[repr(C)]
pub struct bpf_cgroup_storage_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CgroupStorageMap {
    type_: __u32,
    key: *const bpf_cgroup_storage_key,
    value: *const __u64,
}

#[repr(C)]
pub struct CgroupStorageOobMap {
    type_: __u32,
    key: *const bpf_cgroup_storage_key,
    value: *const __u32,
}

#[repr(C)]
pub struct LruMap {
    type_: __u32,
    max_entries: __u32,
    key: *const __u32,
    value: *const __u32,
}

unsafe extern "C" {
    fn bpf_get_local_storage(map: *mut core::ffi::c_void, flags: __u64) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut cgroup_storage: CgroupStorageMap = CgroupStorageMap {
    type_: BPF_MAP_TYPE_CGROUP_STORAGE,
    key: core::ptr::null(),
    value: core::ptr::null(),
};

#[unsafe(link_section = "cgroup_skb/egress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_prog(skb: *mut __sk_buff) -> i32 {
    let counter: *mut __u64;

    counter = unsafe {
        bpf_get_local_storage(
            &raw mut cgroup_storage as *mut core::ffi::c_void,
            0,
        ) as *mut __u64
    };
    unsafe {
        core::intrinsics::atomic_xadd_seqcst(counter, 1);
    }

    /* Drop one out of every two packets */
    unsafe { (*counter & 1) as i32 }
}

/* Maps for OOB test */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut cgroup_storage_oob: CgroupStorageOobMap = CgroupStorageOobMap {
    type_: BPF_MAP_TYPE_CGROUP_STORAGE,
    key: core::ptr::null(),
    value: core::ptr::null(), /* 4-byte value - not 8-byte aligned */
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut lru_map: LruMap = LruMap {
    type_: BPF_MAP_TYPE_LRU_PERCPU_HASH,
    max_entries: 1,
    key: core::ptr::null(),
    value: core::ptr::null(), /* 4-byte value - same as cgroup storage */
};

#[unsafe(link_section = "cgroup/sock_create")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger_oob(sk: *mut bpf_sock) -> i32 {
    let key: __u32 = 0;
    let cgroup_val: *mut __u32;
    let value: __u32 = 0x12345678;

    /* Get cgroup storage value */
    cgroup_val = unsafe {
        bpf_get_local_storage(
            &raw mut cgroup_storage_oob as *mut core::ffi::c_void,
            0,
        ) as *mut __u32
    };
    if cgroup_val.is_null() {
        return 0;
    }

    /* Initialize cgroup storage */
    unsafe {
        *cgroup_val = value;
    }

    /* This triggers the OOB read:
     * bpf_map_update_elem() -> htab_map_update_elem() ->
     * pcpu_init_value() -> copy_map_value_long() ->
     * bpf_obj_memcpy(..., long_memcpy=true) ->
     * bpf_long_memcpy(dst, src, round_up(4, 8))
     *
     * The copy size is rounded up to 8 bytes, but cgroup_val
     * points to a 4-byte buffer, causing a 4-byte OOB read.
     */
    unsafe {
        bpf_map_update_elem(
            &raw mut lru_map as *mut core::ffi::c_void,
            &key as *const __u32 as *const core::ffi::c_void,
            cgroup_val as *const core::ffi::c_void,
            BPF_ANY,
        );
    }

    1
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
