// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Rust translation of dependencies from:
 *   #include <vmlinux.h>
 *   #include <bpf/bpf_helpers.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_HASH: __u32 = 1;

#[used]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct key_t {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub seq_num: __u32,
}

#[repr(C)]
pub struct bpf_iter__bpf_map_elem {
    pub meta: *mut bpf_iter_meta,
    pub map: *mut bpf_map,
    pub key: *mut core::ffi::c_void,
    pub value: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct bpf_map_def_key_t_u64 {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct bpf_map_def_u64_u64 {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct bpf_map_def_key_t_u32 {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[used]
#[unsafe(link_section = ".maps")]
pub static mut hashmap1: bpf_map_def_key_t_u64 = bpf_map_def_key_t_u64 {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 3,
    key_size: core::mem::size_of::<key_t>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[used]
#[unsafe(link_section = ".maps")]
pub static mut hashmap2: bpf_map_def_u64_u64 = bpf_map_def_u64_u64 {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 3,
    key_size: core::mem::size_of::<__u64>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[used]
#[unsafe(link_section = ".maps")]
pub static mut hashmap3: bpf_map_def_key_t_u32 = bpf_map_def_key_t_u32 {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 3,
    key_size: core::mem::size_of::<key_t>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

/* will set before prog run */
pub static mut in_test_mode: bool = false;

/* will collect results during prog run */
pub static mut key_sum_a: __u32 = 0;
pub static mut key_sum_b: __u32 = 0;
pub static mut key_sum_c: __u32 = 0;
pub static mut val_sum: __u64 = 0;

unsafe extern "C" {
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i32;
    fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i32;
    fn bpf_seq_printf(seq: *mut seq_file, fmt: *const u8, fmt_size: __u32, ...) -> i32;
}

macro_rules! BPF_SEQ_PRINTF {
    ($seq:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        const FMT: &[u8] = concat!($fmt, "\0").as_bytes();
        unsafe { bpf_seq_printf($seq, FMT.as_ptr(), FMT.len() as __u32, $($arg),*) }
    }};
}

#[unsafe(link_section = "iter/bpf_map_elem")]
pub unsafe extern "C" fn dump_bpf_hash_map(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    let seq: *mut seq_file = unsafe { (*(*ctx).meta).seq };
    let seq_num: __u32 = unsafe { (*(*ctx).meta).seq_num };
    let map: *mut bpf_map = unsafe { (*ctx).map };
    let key: *mut key_t = unsafe { (*ctx).key as *mut key_t };
    let mut tmp_key: key_t = key_t { a: 0, b: 0, c: 0 };
    let val: *mut __u64 = unsafe { (*ctx).value as *mut __u64 };
    let tmp_val: __u64 = 0;
    let mut ret: i32;

    unsafe {
        if in_test_mode {
            /* test mode is used by selftests to
             * test functionality of bpf_hash_map iter.
             *
             * the above hashmap1 will have correct size
             * and will be accepted, hashmap2 and hashmap3
             * should be rejected due to smaller key/value
             * size.
             */
            if key == core::ptr::null_mut() || val == core::ptr::null_mut() {
                return 0;
            }

            /* update the value and then delete the <key, value> pair.
             * it should not impact the existing 'val' which is still
             * accessible under rcu.
             */
            core::ptr::copy_nonoverlapping(key, &mut tmp_key, 1);
            ret = bpf_map_update_elem(
                &raw mut hashmap1 as *mut core::ffi::c_void,
                &tmp_key as *const key_t as *const core::ffi::c_void,
                &tmp_val as *const __u64 as *const core::ffi::c_void,
                0,
            );
            if ret != 0 {
                return 0;
            }
            ret = bpf_map_delete_elem(
                &raw mut hashmap1 as *mut core::ffi::c_void,
                &tmp_key as *const key_t as *const core::ffi::c_void,
            );
            if ret != 0 {
                return 0;
            }

            key_sum_a = key_sum_a.wrapping_add((*key).a as __u32);
            key_sum_b = key_sum_b.wrapping_add((*key).b as __u32);
            key_sum_c = key_sum_c.wrapping_add((*key).c as __u32);
            val_sum = val_sum.wrapping_add(*val);
            return 0;
        }
    }

    /* non-test mode, the map is prepared with the
     * below bpftool command sequence:
     *   bpftool map create /sys/fs/bpf/m1 type hash \
     *   	key 12 value 8 entries 3 name map1
     *   bpftool map update id 77 key 0 0 0 1 0 0 0 0 0 0 0 1 \
     *   	value 0 0 0 1 0 0 0 1
     *   bpftool map update id 77 key 0 0 0 1 0 0 0 0 0 0 0 2 \
     *   	value 0 0 0 1 0 0 0 2
     * The bpftool iter command line:
     *   bpftool iter pin ./bpf_iter_bpf_hash_map.o /sys/fs/bpf/p1 \
     *   	map id 77
     * The below output will be:
     *   map dump starts
     *   77: (1000000 0 2000000) (200000001000000)
     *   77: (1000000 0 1000000) (100000001000000)
     *   map dump ends
     */
    if seq_num == 0 {
        BPF_SEQ_PRINTF!(seq, "map dump starts\n");
    }

    if key == core::ptr::null_mut() || val == core::ptr::null_mut() {
        BPF_SEQ_PRINTF!(seq, "map dump ends\n");
        return 0;
    }

    unsafe {
        BPF_SEQ_PRINTF!(
            seq,
            "%d: (%x %d %x) (%llx)\n",
            (*map).id,
            (*key).a,
            (*key).b,
            (*key).c,
            *val
        );
    }

    return 0;
}

#[unsafe(link_section = "iter.s/bpf_map_elem")]
pub unsafe extern "C" fn sleepable_dummy_dump(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    unsafe {
        if (*(*ctx).meta).seq_num == 0 {
            BPF_SEQ_PRINTF!((*(*ctx).meta).seq, "map dump starts\n");
        }
    }

    return 0;
}
