// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// Dependency includes and the preserve_access_index attribute are supplied by
// the eBPF build environment.

use core::ffi::c_char;

#[repr(C)]
pub struct seq_file;

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub session_id: __u64,
    pub seq_num: __u64,
}

#[repr(C)]
pub struct bpf_map {
    pub id: __u32,
    pub name: [c_char; 16],
    pub max_entries: __u32,
}

#[repr(C)]
pub struct bpf_iter__bpf_map {
    pub meta: *mut bpf_iter_meta,
    pub map: *mut bpf_map,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
}

#[repr(C)]
pub struct btf_header {
    pub str_len: __u32,
}

#[repr(C)]
pub struct btf {
    pub strings: *const c_char,
    pub types: *mut *mut btf_type,
    pub hdr: btf_header,
}

#[repr(C)]
pub struct bpf_prog_aux {
    pub id: __u32,
    pub name: [c_char; 16],
    pub attach_func_name: *const c_char,
    pub dst_prog: *mut bpf_prog,
    pub func_info: *mut bpf_func_info,
    pub btf: *mut btf,
}

#[repr(C)]
pub struct bpf_prog {
    pub aux: *mut bpf_prog_aux,
}

#[repr(C)]
pub struct bpf_iter__bpf_prog {
    pub meta: *mut bpf_iter_meta,
    pub prog: *mut bpf_prog,
}

// Supplied by the kernel/eBPF dependency headers.
#[repr(C)]
pub struct bpf_func_info;

type __u32 = u32;
type __u64 = u64;
type __s64 = i64;

unsafe fn get_name(btf: *mut btf, btf_id: isize, fallback: *const c_char) -> *const c_char {
    let mut t: *mut btf_type = core::ptr::null_mut();
    let name_off: __u32;

    if btf.is_null() {
        return fallback;
    }
    let str_ = (*btf).strings;
    let types = (*btf).types;
    bpf_probe_read_kernel(
        &mut t as *mut *mut btf_type as *mut core::ffi::c_void,
        core::mem::size_of::<*mut btf_type>(),
        types.offset(btf_id),
    );
    name_off = BPF_CORE_READ!(t, name_off);
    if name_off >= (*btf).hdr.str_len {
        return fallback;
    }
    str_.add(name_off as usize)
}

extern "C" {
    fn bpf_map_sum_elem_count(map: *mut bpf_map) -> __s64;
}

#[no_mangle]
pub unsafe extern "C" fn dump_bpf_map(ctx: *mut bpf_iter__bpf_map) -> i32 {
    let seq = (*(*ctx).meta).seq;
    let seq_num = (*(*ctx).meta).seq_num;
    let map = (*ctx).map;

    if map.is_null() {
        return 0;
    }
    if seq_num == 0 {
        BPF_SEQ_PRINTF!(seq, "  id name             max_entries  cur_entries\n");
    }
    BPF_SEQ_PRINTF!(
        seq,
        "%4u %-16s  %10d   %10lld\n",
        (*map).id,
        (*map).name.as_ptr(),
        (*map).max_entries,
        bpf_map_sum_elem_count(map)
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn dump_bpf_prog(ctx: *mut bpf_iter__bpf_prog) -> i32 {
    let seq = (*(*ctx).meta).seq;
    let seq_num = (*(*ctx).meta).seq_num;
    let prog = (*ctx).prog;

    if prog.is_null() {
        return 0;
    }
    let aux = (*prog).aux;
    if seq_num == 0 {
        BPF_SEQ_PRINTF!(seq, "  id name             attached\n");
    }
    let type_id = *( (*aux).func_info as *const __u32 );
    BPF_SEQ_PRINTF!(
        seq,
        "%4u %-16s %s %s\n",
        (*aux).id,
        get_name((*aux).btf, type_id as isize, (*aux).name.as_ptr()),
        (*aux).attach_func_name,
        (*(*aux).dst_prog).aux.as_ref().unwrap().name.as_ptr()
    );
    0
}

// External helper supplied by the eBPF dependency headers.
extern "C" {
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: usize, src: *const core::ffi::c_void) -> i64;
}

#[no_mangle]
pub static mut LICENSE: [c_char; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
