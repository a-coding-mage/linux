// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2023 Google
//
// Translated from sample_filter.bpf.c. C include dependencies:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// <bpf/bpf_core_read.h>, and "sample-filter.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u32 = u32;
type __u64 = u64;

extern "C" {
    static mut filters: filters;
    static mut event_hash: event_hash;
    static mut idx_hash: idx_hash;
    static mut dropped: lost_count;

    static use_idx_hash: i32;

    fn bpf_cast_to_kern_ctx(ctx: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_current_uid_gid() -> __u64;
}

extern "C" {
    static MAX_FILTERS: i32;

    static PBF_TERM_SAMPLE_START: i32;
    static PBF_TERM_SAMPLE_END: i32;
    static PBF_TERM_IP: i32;
    static PBF_TERM_TID: i32;
    static PBF_TERM_TIME: i32;
    static PBF_TERM_ADDR: i32;
    static PBF_TERM_ID: i32;
    static PBF_TERM_CPU: i32;
    static PBF_TERM_PERIOD: i32;
    static PBF_TERM_WEIGHT: i32;
    static PBF_TERM_DATA_SRC: i32;
    static PBF_TERM_TRANSACTION: i32;
    static PBF_TERM_PHYS_ADDR: i32;
    static PBF_TERM_CGROUP: i32;
    static PBF_TERM_DATA_PAGE_SIZE: i32;
    static PBF_TERM_CODE_PAGE_SIZE: i32;
    static PBF_TERM_WEIGHT_STRUCT: i32;
    static PBF_TERM_UID: i32;
    static PBF_TERM_GID: i32;
    static PBF_TERM_NONE: i32;
    static __PBF_UNUSED_TERM4: i32;
    static __PBF_UNUSED_TERM5: i32;
    static __PBF_UNUSED_TERM9: i32;
    static __PBF_UNUSED_TERM10: i32;
    static __PBF_UNUSED_TERM11: i32;
    static __PBF_UNUSED_TERM12: i32;
    static __PBF_UNUSED_TERM13: i32;
    static __PBF_UNUSED_TERM16: i32;
    static __PBF_UNUSED_TERM18: i32;
    static __PBF_UNUSED_TERM20: i32;

    static PBF_OP_EQ: i32;
    static PBF_OP_NEQ: i32;
    static PBF_OP_GT: i32;
    static PBF_OP_GE: i32;
    static PBF_OP_LT: i32;
    static PBF_OP_LE: i32;
    static PBF_OP_AND: i32;
    static PBF_OP_GROUP_BEGIN: i32;
    static PBF_OP_GROUP_END: i32;
    static PBF_OP_DONE: i32;
}

/* BPF map that will be filled by user space */
#[repr(C)]
pub struct filters {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __type(key, int);
    // __type(value, struct perf_bpf_filter_entry[MAX_FILTERS]);
    // __uint(max_entries, 1);
    _private: [u8; 0],
}
// filters SEC(".maps")

/*
 * An evsel has multiple instances for each CPU or task but we need a single
 * id to be used as a key for the idx_hash.  This hashmap would translate the
 * instance's ID to a representative ID.
 */
#[repr(C)]
pub struct event_hash {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __type(key, __u64);
    // __type(value, __u64);
    // __uint(max_entries, 1);
    _private: [u8; 0],
}
// event_hash SEC(".maps")

/* tgid/evtid to filter index */
#[repr(C)]
pub struct idx_hash {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __type(key, struct idx_hash_key);
    // __type(value, int);
    // __uint(max_entries, 1);
    _private: [u8; 0],
}
// idx_hash SEC(".maps")

/* tgid to filter index */
#[repr(C)]
pub struct lost_count {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __type(key, int);
    // __type(value, int);
    // __uint(max_entries, 1);
    _private: [u8; 0],
}
// dropped SEC(".maps")

/* new kernel perf_sample_data definition */
#[repr(C)]
pub struct perf_sample_data___new {
    pub sample_flags: __u64,
}
// __attribute__((preserve_access_index))

/* new kernel perf_mem_data_src definition */
#[repr(C)]
pub union perf_mem_data_src___new {
    pub val: __u64,
    pub bits: perf_mem_data_src___new_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_mem_data_src___new_bits {
    pub val: __u64,
}

impl perf_mem_data_src___new_bits {
    #[inline]
    pub unsafe fn mem_hops(&self) -> __u64 {
        (self.val >> 43) & 0x7
    }
}

#[repr(C)]
pub struct idx_hash_key {
    pub tgid: __u64,
    pub evt_id: __u64,
}

#[repr(C)]
pub struct perf_bpf_filter_entry {
    pub term: i32,
    pub op: i32,
    pub part: i32,
    pub value: __u64,
}

#[repr(C)]
pub struct bpf_perf_event_data_kern {
    pub data: *mut perf_sample_data,
    pub event: *mut perf_event,
}

#[repr(C)]
pub struct perf_event {
    pub id: __u64,
    pub parent: *mut perf_event,
}

#[repr(C)]
pub struct perf_sample_data {
    pub ip: __u64,
    pub id: __u64,
    pub tid_entry: perf_sample_data_tid_entry,
    pub cpu_entry: perf_sample_data_cpu_entry,
    pub time: __u64,
    pub addr: __u64,
    pub period: __u64,
    pub txn: __u64,
    pub weight: perf_sample_weight,
    pub phys_addr: __u64,
    pub cgroup: __u64,
    pub code_page_size: __u64,
    pub data_page_size: __u64,
    pub data_src: perf_mem_data_src,
}

#[repr(C)]
pub struct perf_sample_data_tid_entry {
    pub pid: __u64,
    pub tid: __u64,
}

#[repr(C)]
pub struct perf_sample_data_cpu_entry {
    pub cpu: __u64,
}

#[repr(C)]
pub union perf_sample_weight {
    pub full: __u64,
    pub fields: perf_sample_weight_fields,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_sample_weight_fields {
    pub var1_dw: __u64,
    pub var2_w: __u32,
    pub var3_w: __u32,
}

#[repr(C)]
pub union perf_mem_data_src {
    pub val: __u64,
    pub bits: perf_mem_data_src_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_mem_data_src_bits {
    pub val: __u64,
}

impl perf_mem_data_src_bits {
    #[inline]
    pub unsafe fn mem_op(&self) -> __u64 {
        self.val & 0x1f
    }

    #[inline]
    pub unsafe fn mem_snoop(&self) -> __u64 {
        (self.val >> 19) & 0x1f
    }

    #[inline]
    pub unsafe fn mem_lock(&self) -> __u64 {
        (self.val >> 24) & 0x3
    }

    #[inline]
    pub unsafe fn mem_dtlb(&self) -> __u64 {
        (self.val >> 26) & 0x7f
    }

    #[inline]
    pub unsafe fn mem_lvl_num(&self) -> __u64 {
        (self.val >> 33) & 0xf
    }

    #[inline]
    pub unsafe fn mem_remote(&self) -> __u64 {
        (self.val >> 37) & 0x1
    }

    #[inline]
    pub unsafe fn mem_snoopx(&self) -> __u64 {
        (self.val >> 38) & 0x3
    }

    #[inline]
    pub unsafe fn mem_blk(&self) -> __u64 {
        (self.val >> 40) & 0x7
    }
}

#[inline]
unsafe fn check_result_failed(data: __u64, op: i32, val: __u64) -> bool {
    if op == PBF_OP_EQ {
        !(data == val)
    } else if op == PBF_OP_NEQ {
        !(data != val)
    } else if op == PBF_OP_GT {
        !(data > val)
    } else if op == PBF_OP_GE {
        !(data >= val)
    } else if op == PBF_OP_LT {
        !(data < val)
    } else if op == PBF_OP_LE {
        !(data <= val)
    } else {
        !(data & val != 0)
    }
}

/* helper function to return the given perf sample data */
#[inline]
unsafe fn perf_get_sample(
    kctx: *mut bpf_perf_event_data_kern,
    entry: *mut perf_bpf_filter_entry,
) -> __u64 {
    let data = (*kctx).data as *mut perf_sample_data___new;

    // bpf_core_field_exists(data->sample_flags)
    if data.is_null() {
        return 0;
    }

    // BUILD_CHECK_SAMPLE(...) static assertions are compile-time checks in C.

    /* For sample terms check the sample bit is set. */
    if (*entry).term >= PBF_TERM_SAMPLE_START
        && (*entry).term <= PBF_TERM_SAMPLE_END
        && ((*data).sample_flags
            & (1_u64 << ((*entry).term - PBF_TERM_SAMPLE_START) as __u64))
            == 0
    {
        return 0;
    }

    if (*entry).term == PBF_TERM_IP {
        return (*(*kctx).data).ip;
    }
    if (*entry).term == PBF_TERM_ID {
        return (*(*kctx).data).id;
    }
    if (*entry).term == PBF_TERM_TID {
        if (*entry).part != 0 {
            return (*(*kctx).data).tid_entry.pid;
        } else {
            return (*(*kctx).data).tid_entry.tid;
        }
    }
    if (*entry).term == PBF_TERM_CPU {
        return (*(*kctx).data).cpu_entry.cpu;
    }
    if (*entry).term == PBF_TERM_TIME {
        return (*(*kctx).data).time;
    }
    if (*entry).term == PBF_TERM_ADDR {
        return (*(*kctx).data).addr;
    }
    if (*entry).term == PBF_TERM_PERIOD {
        return (*(*kctx).data).period;
    }
    if (*entry).term == PBF_TERM_TRANSACTION {
        return (*(*kctx).data).txn;
    }
    if (*entry).term == PBF_TERM_WEIGHT_STRUCT {
        if (*entry).part == 1 {
            return (*(*kctx).data).weight.fields.var1_dw;
        }
        if (*entry).part == 2 {
            return (*(*kctx).data).weight.fields.var2_w as __u64;
        }
        if (*entry).part == 3 {
            return (*(*kctx).data).weight.fields.var3_w as __u64;
        }
        /* fall through */
        return (*(*kctx).data).weight.full;
    }
    if (*entry).term == PBF_TERM_WEIGHT {
        return (*(*kctx).data).weight.full;
    }
    if (*entry).term == PBF_TERM_PHYS_ADDR {
        return (*(*kctx).data).phys_addr;
    }
    if (*entry).term == PBF_TERM_CGROUP {
        return (*(*kctx).data).cgroup;
    }
    if (*entry).term == PBF_TERM_CODE_PAGE_SIZE {
        return (*(*kctx).data).code_page_size;
    }
    if (*entry).term == PBF_TERM_DATA_PAGE_SIZE {
        return (*(*kctx).data).data_page_size;
    }
    if (*entry).term == PBF_TERM_DATA_SRC {
        if (*entry).part == 1 {
            return (*(*kctx).data).data_src.bits.mem_op();
        }
        if (*entry).part == 2 {
            return (*(*kctx).data).data_src.bits.mem_lvl_num();
        }
        if (*entry).part == 3 {
            let snoop: __u32 = (*(*kctx).data).data_src.bits.mem_snoop() as __u32;
            let snoopx: __u32 = (*(*kctx).data).data_src.bits.mem_snoopx() as __u32;

            return ((snoopx << 5) | snoop) as __u64;
        }
        if (*entry).part == 4 {
            return (*(*kctx).data).data_src.bits.mem_remote();
        }
        if (*entry).part == 5 {
            return (*(*kctx).data).data_src.bits.mem_lock();
        }
        if (*entry).part == 6 {
            return (*(*kctx).data).data_src.bits.mem_dtlb();
        }
        if (*entry).part == 7 {
            return (*(*kctx).data).data_src.bits.mem_blk();
        }
        if (*entry).part == 8 {
            let data = &mut (*(*kctx).data).data_src as *mut perf_mem_data_src
                as *mut perf_mem_data_src___new;

            // __builtin_preserve_field_info(data->mem_hops, BPF_FIELD_EXISTS)
            return (*data).bits.mem_hops();
        }
        /* return the whole word */
        return (*(*kctx).data).data_src.val;
    }
    if (*entry).term == PBF_TERM_UID {
        return bpf_get_current_uid_gid() & 0xFFFFFFFF;
    }
    if (*entry).term == PBF_TERM_GID {
        return bpf_get_current_uid_gid() >> 32;
    }
    if (*entry).term == PBF_TERM_NONE
        || (*entry).term == __PBF_UNUSED_TERM4
        || (*entry).term == __PBF_UNUSED_TERM5
        || (*entry).term == __PBF_UNUSED_TERM9
        || (*entry).term == __PBF_UNUSED_TERM10
        || (*entry).term == __PBF_UNUSED_TERM11
        || (*entry).term == __PBF_UNUSED_TERM12
        || (*entry).term == __PBF_UNUSED_TERM13
        || (*entry).term == __PBF_UNUSED_TERM16
        || (*entry).term == __PBF_UNUSED_TERM18
        || (*entry).term == __PBF_UNUSED_TERM20
    {
        return 0;
    }
    0
}

/* BPF program to be called from perf event overflow handler */
// SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn perf_sample_filter(ctx: *mut core::ffi::c_void) -> i32 {
    let mut kctx: *mut bpf_perf_event_data_kern;
    let mut entry: *mut perf_bpf_filter_entry;
    let mut sample_data: __u64;
    let mut in_group: i32 = 0;
    let mut group_result: i32 = 0;
    let mut i: i32;
    let mut k: i32;
    let mut losts: *mut i32;

    kctx = bpf_cast_to_kern_ctx(ctx) as *mut bpf_perf_event_data_kern;

    k = 0;

    if use_idx_hash != 0 {
        let mut key = idx_hash_key {
            tgid: bpf_get_current_pid_tgid() >> 32,
            evt_id: 0,
        };
        let mut eid: __u64 = (*(*kctx).event).id;
        let mut key_id: *mut __u64;
        let mut idx: *mut i32;

        /* get primary_event_id */
        if !(*(*kctx).event).parent.is_null() {
            eid = (*(*(*kctx).event).parent).id;
        }

        key_id = bpf_map_lookup_elem(
            &mut event_hash as *mut _ as *mut core::ffi::c_void,
            &eid as *const _ as *const core::ffi::c_void,
        ) as *mut __u64;
        if key_id.is_null() {
            return drop(k);
        }

        key.evt_id = *key_id;

        idx = bpf_map_lookup_elem(
            &mut idx_hash as *mut _ as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut i32;
        if !idx.is_null() {
            k = *idx;
        } else {
            return drop(k);
        }
    }

    entry = bpf_map_lookup_elem(
        &mut filters as *mut _ as *mut core::ffi::c_void,
        &k as *const _ as *const core::ffi::c_void,
    ) as *mut perf_bpf_filter_entry;
    if entry.is_null() {
        return drop(k);
    }

    i = 0;
    while i < MAX_FILTERS {
        let cur = entry.offset(i as isize);
        sample_data = perf_get_sample(kctx, cur);

        if (*cur).op == PBF_OP_EQ
            || (*cur).op == PBF_OP_NEQ
            || (*cur).op == PBF_OP_GT
            || (*cur).op == PBF_OP_GE
            || (*cur).op == PBF_OP_LT
            || (*cur).op == PBF_OP_LE
            || (*cur).op == PBF_OP_AND
        {
            if check_result_failed(sample_data, (*cur).op, (*cur).value) {
                if in_group == 0 {
                    return drop(k);
                }
            } else if in_group != 0 {
                group_result = 1;
            }
        } else if (*cur).op == PBF_OP_GROUP_BEGIN {
            in_group = 1;
            group_result = 0;
        } else if (*cur).op == PBF_OP_GROUP_END {
            if group_result == 0 {
                return drop(k);
            }
            in_group = 0;
        } else if (*cur).op == PBF_OP_DONE {
            /* no failures so far, accept it */
            return 1;
        }

        i += 1;
    }
    /* generate sample data */
    1
}

#[inline]
unsafe fn drop(k: i32) -> i32 {
    let losts = bpf_map_lookup_elem(
        &mut dropped as *mut _ as *mut core::ffi::c_void,
        &k as *const _ as *const core::ffi::c_void,
    ) as *mut i32;
    if !losts.is_null() {
        core::intrinsics::atomic_xadd_seqcst(losts, 1);
    }

    0
}

// char LICENSE[] SEC("license") = "Dual BSD/GPL";
#[no_mangle]
#[link_section = "license"]
pub static LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
