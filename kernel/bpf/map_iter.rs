// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */

// Dependencies supplied by the surrounding kernel/BPF translation unit.

#[repr(C)]
pub struct bpf_iter_seq_map_info {
    pub map_id: u32,
}

unsafe fn bpf_map_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let info = (*seq).private as *mut bpf_iter_seq_map_info;
    let map = bpf_map_get_curr_or_next(&mut (*info).map_id);
    if map.is_null() {
        return core::ptr::null_mut();
    }
    if *pos == 0 {
        *pos += 1;
    }
    map as *mut core::ffi::c_void
}

unsafe fn bpf_map_seq_next(
    seq: *mut seq_file,
    v: *mut core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    let info = (*seq).private as *mut bpf_iter_seq_map_info;
    *pos += 1;
    (*info).map_id += 1;
    bpf_map_put(v as *mut bpf_map);
    bpf_map_get_curr_or_next(&mut (*info).map_id) as *mut core::ffi::c_void
}

#[repr(C)]
pub struct bpf_iter__bpf_map {
    pub meta: *mut bpf_iter_meta,
    pub map: *mut bpf_map,
}

unsafe fn __bpf_map_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void, in_stop: bool) -> i32 {
    let mut ctx = bpf_iter__bpf_map { meta: core::ptr::null_mut(), map: v as *mut bpf_map };
    let mut meta = bpf_iter_meta { seq };
    let mut ret = 0;
    ctx.meta = &mut meta;
    let prog = bpf_iter_get_info(&mut meta, in_stop);
    if !prog.is_null() {
        ret = bpf_iter_run_prog(prog, &mut ctx);
    }
    ret
}

unsafe fn bpf_map_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    __bpf_map_seq_show(seq, v, false)
}

unsafe fn bpf_map_seq_stop(seq: *mut seq_file, v: *mut core::ffi::c_void) {
    if v.is_null() {
        let _ = __bpf_map_seq_show(seq, v, true);
    } else {
        bpf_map_put(v as *mut bpf_map);
    }
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe fn(*mut seq_file, *mut loff_t) -> *mut core::ffi::c_void>,
    pub next: Option<unsafe fn(*mut seq_file, *mut core::ffi::c_void, *mut loff_t) -> *mut core::ffi::c_void>,
    pub stop: Option<unsafe fn(*mut seq_file, *mut core::ffi::c_void)>,
    pub show: Option<unsafe fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
}

static bpf_map_seq_ops: seq_operations = seq_operations {
    start: Some(bpf_map_seq_start),
    next: Some(bpf_map_seq_next),
    stop: Some(bpf_map_seq_stop),
    show: Some(bpf_map_seq_show),
};

// DEFINE_BPF_ITER_FUNC(bpf_map, struct bpf_iter_meta *meta, struct bpf_map *map)
// BTF_ID_LIST_GLOBAL_SINGLE(btf_bpf_map_id, struct, bpf_map)
// Registration and kfunc-list macros are supplied by the kernel build environment.

unsafe fn bpf_iter_attach_map(
    prog: *mut bpf_prog,
    linfo: *mut bpf_iter_link_info,
    aux: *mut bpf_iter_aux_info,
) -> i32 {
    let mut err = -EINVAL;
    let mut is_percpu = false;
    if (*linfo).map.map_fd == 0 {
        return -EBADF;
    }
    let map = bpf_map_get_with_uref((*linfo).map.map_fd);
    if IS_ERR(map) {
        return PTR_ERR(map);
    }
    if (*map).excl_prog_sha != 0 {
        err = -EPERM;
        bpf_map_put_with_uref(map);
        return err;
    }
    if (*map).map_type == BPF_MAP_TYPE_PERCPU_HASH
        || (*map).map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH
        || (*map).map_type == BPF_MAP_TYPE_PERCPU_ARRAY
    {
        is_percpu = true;
    } else if (*map).map_type != BPF_MAP_TYPE_HASH
        && (*map).map_type != BPF_MAP_TYPE_LRU_HASH
        && (*map).map_type != BPF_MAP_TYPE_ARRAY
        && (*map).map_type != BPF_MAP_TYPE_RHASH
    {
        bpf_map_put_with_uref(map);
        return err;
    }
    let key_acc_size = (*(*prog).aux).max_rdonly_access;
    let value_acc_size = (*(*prog).aux).max_rdwr_access;
    let key_size = (*map).key_size;
    let value_size = if !is_percpu {
        (*map).value_size
    } else {
        round_up((*map).value_size, 8) * num_possible_cpus()
    };
    if key_acc_size > key_size || value_acc_size > value_size {
        err = -EACCES;
        bpf_map_put_with_uref(map);
        return err;
    }
    (*aux).map = map;
    0
}

unsafe fn bpf_iter_detach_map(aux: *mut bpf_iter_aux_info) {
    bpf_map_put_with_uref((*aux).map);
}

pub unsafe fn bpf_iter_map_show_fdinfo(aux: *const bpf_iter_aux_info, seq: *mut seq_file) {
    seq_printf(seq, "map_id:\t%u\n", (*(*aux).map).id);
}

pub unsafe fn bpf_iter_map_fill_link_info(aux: *const bpf_iter_aux_info, info: *mut bpf_link_info) -> i32 {
    (*info).iter.map.map_id = (*(*aux).map).id;
    0
}

// DEFINE_BPF_ITER_FUNC(bpf_map_elem, struct bpf_iter_meta *meta,
//                      struct bpf_map *map, void *key, void *value)

unsafe fn bpf_map_iter_init() -> i32 {
    (*bpf_map_reg_info.ctx_arg_info.add(0)).btf_id = *btf_bpf_map_id;
    let ret = bpf_iter_reg_target(&bpf_map_reg_info);
    if ret != 0 {
        return ret;
    }
    bpf_iter_reg_target(&bpf_map_elem_reg_info)
}

// late_initcall(bpf_map_iter_init)
// __bpf_kfunc_start_defs__ / __bpf_kfunc_end_defs__

pub unsafe fn bpf_map_sum_elem_count(map: *const bpf_map) -> i64 {
    if map.is_null() || (*map).elem_count.is_null() {
        return 0;
    }
    let mut ret = 0i64;
    for_each_possible_cpu!(cpu {
        let pcount = per_cpu_ptr((*map).elem_count, cpu);
        ret += READ_ONCE!(*pcount);
    });
    ret
}

// BTF_KFUNCS_START(bpf_map_iter_kfunc_ids)
// BTF_ID_FLAGS(func, bpf_map_sum_elem_count)
// BTF_KFUNCS_END(bpf_map_iter_kfunc_ids)
// late_initcall(init_subsystem)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
