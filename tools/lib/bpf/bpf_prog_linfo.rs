// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2018 Facebook */

use core::ffi::c_void;
use core::mem::{offset_of, size_of};
use core::ptr;

pub type __u32 = u32;
pub type __u64 = u64;

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ENOENT: i32 = 2;

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn __errno_location() -> *mut i32;
}

#[inline]
unsafe fn set_errno(err: i32) {
    unsafe {
        *__errno_location() = err;
    }
}

#[repr(C)]
pub struct bpf_line_info {
    pub insn_off: __u32,
    pub file_name_off: __u32,
    pub line_off: __u32,
    pub line_col: __u32,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub nr_line_info: __u32,
    pub line_info_rec_size: __u32,
    pub line_info: __u64,
    pub nr_jited_ksyms: __u32,
    pub jited_line_info: __u64,
    pub nr_jited_line_info: __u32,
    pub jited_line_info_rec_size: __u32,
    pub nr_jited_func_lens: __u32,
    pub jited_ksyms: __u64,
    pub jited_func_lens: __u64,
}

#[repr(C)]
pub struct bpf_prog_linfo {
    raw_linfo: *mut c_void,
    raw_jited_linfo: *mut c_void,
    nr_jited_linfo_per_func: *mut __u32,
    jited_linfo_func_idx: *mut __u32,
    nr_linfo: __u32,
    nr_jited_func: __u32,
    rec_size: __u32,
    jited_rec_size: __u32,
}

unsafe fn dissect_jited_func(
    prog_linfo: *mut bpf_prog_linfo,
    ksym_func: *const __u64,
    ksym_len: *const __u32,
) -> i32 {
    let nr_jited_func: __u32;
    let nr_linfo: __u32;
    let mut raw_jited_linfo: *const c_void;
    let mut jited_linfo: *const __u64;
    let mut last_jited_linfo: __u64;
    /*
     * Index to raw_jited_linfo:
     *      i: Index for searching the next ksym_func
     * prev_i: Index to the last found ksym_func
     */
    let mut i: __u32;
    let mut prev_i: __u32;
    let mut f: __u32; /* Index to ksym_func */

    unsafe {
        raw_jited_linfo = (*prog_linfo).raw_jited_linfo as *const c_void;
        jited_linfo = raw_jited_linfo as *const __u64;
        if *ksym_func.add(0) != *jited_linfo {
            return -EINVAL;
        }

        *(*prog_linfo).jited_linfo_func_idx.add(0) = 0;
        nr_jited_func = (*prog_linfo).nr_jited_func;
        nr_linfo = (*prog_linfo).nr_linfo;

        prev_i = 0;
        i = 1;
        f = 1;
        while i < nr_linfo && f < nr_jited_func {
            raw_jited_linfo =
                (raw_jited_linfo as *const u8).add((*prog_linfo).jited_rec_size as usize)
                    as *const c_void;
            last_jited_linfo = *jited_linfo;
            jited_linfo = raw_jited_linfo as *const __u64;

            if *ksym_func.add(f as usize) == *jited_linfo {
                *(*prog_linfo).jited_linfo_func_idx.add(f as usize) = i;

                /* Sanity check */
                if last_jited_linfo
                    .wrapping_sub(*ksym_func.add(f.wrapping_sub(1) as usize))
                    .wrapping_add(1)
                    > *ksym_len.add(f.wrapping_sub(1) as usize) as __u64
                {
                    return -EINVAL;
                }

                *(*prog_linfo)
                    .nr_jited_linfo_per_func
                    .add(f.wrapping_sub(1) as usize) = i.wrapping_sub(prev_i);
                prev_i = i;

                /*
                 * The ksym_func[f] is found in jited_linfo.
                 * Look for the next one.
                 */
                f = f.wrapping_add(1);
            } else if *jited_linfo <= last_jited_linfo {
                /* Ensure the addr is increasing _within_ a func */
                return -EINVAL;
            }

            i = i.wrapping_add(1);
        }

        if f != nr_jited_func {
            return -EINVAL;
        }

        *(*prog_linfo)
            .nr_jited_linfo_per_func
            .add(nr_jited_func.wrapping_sub(1) as usize) = nr_linfo.wrapping_sub(prev_i);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_prog_linfo__free(prog_linfo: *mut bpf_prog_linfo) {
    unsafe {
        if prog_linfo.is_null() {
            return;
        }

        free((*prog_linfo).raw_linfo);
        free((*prog_linfo).raw_jited_linfo);
        free((*prog_linfo).nr_jited_linfo_per_func as *mut c_void);
        free((*prog_linfo).jited_linfo_func_idx as *mut c_void);
        free(prog_linfo as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_prog_linfo__new(
    info: *const bpf_prog_info,
) -> *mut bpf_prog_linfo {
    let mut prog_linfo: *mut bpf_prog_linfo;
    let nr_linfo: __u32;
    let nr_jited_func: __u32;
    let mut data_sz: __u64;

    unsafe {
        nr_linfo = (*info).nr_line_info;

        if nr_linfo == 0 {
            set_errno(EINVAL);
            return ptr::null_mut();
        }

        /*
         * The min size that bpf_prog_linfo has to access for
         * searching purpose.
         */
        if (*info).line_info_rec_size < offset_of!(bpf_line_info, file_name_off) as __u32 {
            set_errno(EINVAL);
            return ptr::null_mut();
        }

        prog_linfo = calloc(1, size_of::<bpf_prog_linfo>()) as *mut bpf_prog_linfo;
        if prog_linfo.is_null() {
            set_errno(ENOMEM);
            return ptr::null_mut();
        }

        /* Copy xlated line_info */
        (*prog_linfo).nr_linfo = nr_linfo;
        (*prog_linfo).rec_size = (*info).line_info_rec_size;
        data_sz = nr_linfo as __u64 * (*prog_linfo).rec_size as __u64;
        (*prog_linfo).raw_linfo = malloc(data_sz as usize);
        if (*prog_linfo).raw_linfo.is_null() {
            bpf_prog_linfo__free(prog_linfo);
            set_errno(EINVAL);
            return ptr::null_mut();
        }
        memcpy(
            (*prog_linfo).raw_linfo,
            (*info).line_info as usize as *const c_void,
            data_sz as usize,
        );

        nr_jited_func = (*info).nr_jited_ksyms;
        if nr_jited_func == 0
            || (*info).jited_line_info == 0
            || (*info).nr_jited_line_info != nr_linfo
            || (*info).jited_line_info_rec_size < size_of::<__u64>() as __u32
            || (*info).nr_jited_func_lens != nr_jited_func
            || (*info).jited_ksyms == 0
            || (*info).jited_func_lens == 0
        {
            /* Not enough info to provide jited_line_info */
            return prog_linfo;
        }

        /* Copy jited_line_info */
        (*prog_linfo).nr_jited_func = nr_jited_func;
        (*prog_linfo).jited_rec_size = (*info).jited_line_info_rec_size;
        data_sz = nr_linfo as __u64 * (*prog_linfo).jited_rec_size as __u64;
        (*prog_linfo).raw_jited_linfo = malloc(data_sz as usize);
        if (*prog_linfo).raw_jited_linfo.is_null() {
            bpf_prog_linfo__free(prog_linfo);
            set_errno(EINVAL);
            return ptr::null_mut();
        }
        memcpy(
            (*prog_linfo).raw_jited_linfo,
            (*info).jited_line_info as usize as *const c_void,
            data_sz as usize,
        );

        /* Number of jited_line_info per jited func */
        (*prog_linfo).nr_jited_linfo_per_func =
            malloc(nr_jited_func as usize * size_of::<__u32>()) as *mut __u32;
        if (*prog_linfo).nr_jited_linfo_per_func.is_null() {
            bpf_prog_linfo__free(prog_linfo);
            set_errno(EINVAL);
            return ptr::null_mut();
        }

        /*
         * For each jited func,
         * the start idx to the "linfo" and "jited_linfo" array,
         */
        (*prog_linfo).jited_linfo_func_idx =
            malloc(nr_jited_func as usize * size_of::<__u32>()) as *mut __u32;
        if (*prog_linfo).jited_linfo_func_idx.is_null() {
            bpf_prog_linfo__free(prog_linfo);
            set_errno(EINVAL);
            return ptr::null_mut();
        }

        if dissect_jited_func(
            prog_linfo,
            (*info).jited_ksyms as usize as *const __u64,
            (*info).jited_func_lens as usize as *const __u32,
        ) != 0
        {
            bpf_prog_linfo__free(prog_linfo);
            set_errno(EINVAL);
            return ptr::null_mut();
        }

        prog_linfo
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_prog_linfo__lfind_addr_func(
    prog_linfo: *const bpf_prog_linfo,
    addr: __u64,
    func_idx: __u32,
    nr_skip: __u32,
) -> *const bpf_line_info {
    let jited_rec_size: __u32;
    let rec_size: __u32;
    let mut nr_linfo: __u32;
    let start: __u32;
    let mut i: __u32;
    let mut raw_jited_linfo: *const c_void;
    let mut raw_linfo: *const c_void;
    let mut jited_linfo: *const __u64;

    unsafe {
        if func_idx >= (*prog_linfo).nr_jited_func {
            set_errno(ENOENT);
            return ptr::null();
        }

        nr_linfo = *(*prog_linfo)
            .nr_jited_linfo_per_func
            .add(func_idx as usize);
        if nr_skip >= nr_linfo {
            set_errno(ENOENT);
            return ptr::null();
        }

        start = *(*prog_linfo)
            .jited_linfo_func_idx
            .add(func_idx as usize)
            + nr_skip;
        jited_rec_size = (*prog_linfo).jited_rec_size;
        raw_jited_linfo = ((*prog_linfo).raw_jited_linfo as *const u8)
            .add(start as usize * jited_rec_size as usize)
            as *const c_void;
        jited_linfo = raw_jited_linfo as *const __u64;
        if addr < *jited_linfo {
            set_errno(ENOENT);
            return ptr::null();
        }

        nr_linfo = nr_linfo.wrapping_sub(nr_skip);
        rec_size = (*prog_linfo).rec_size;
        raw_linfo = ((*prog_linfo).raw_linfo as *const u8)
            .add(start as usize * rec_size as usize)
            as *const c_void;
        i = 0;
        while i < nr_linfo {
            if addr < *jited_linfo {
                break;
            }

            raw_linfo = (raw_linfo as *const u8).add(rec_size as usize) as *const c_void;
            raw_jited_linfo =
                (raw_jited_linfo as *const u8).add(jited_rec_size as usize) as *const c_void;
            jited_linfo = raw_jited_linfo as *const __u64;
            i = i.wrapping_add(1);
        }

        (raw_linfo as *const u8).sub(rec_size as usize) as *const bpf_line_info
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_prog_linfo__lfind(
    prog_linfo: *const bpf_prog_linfo,
    insn_off: __u32,
    nr_skip: __u32,
) -> *const bpf_line_info {
    let mut linfo: *const bpf_line_info;
    let rec_size: __u32;
    let mut nr_linfo: __u32;
    let mut i: __u32;
    let mut raw_linfo: *const c_void;

    unsafe {
        nr_linfo = (*prog_linfo).nr_linfo;
        if nr_skip >= nr_linfo {
            set_errno(ENOENT);
            return ptr::null();
        }

        rec_size = (*prog_linfo).rec_size;
        raw_linfo = ((*prog_linfo).raw_linfo as *const u8)
            .add(nr_skip as usize * rec_size as usize)
            as *const c_void;
        linfo = raw_linfo as *const bpf_line_info;
        if insn_off < (*linfo).insn_off {
            set_errno(ENOENT);
            return ptr::null();
        }

        nr_linfo = nr_linfo.wrapping_sub(nr_skip);
        i = 0;
        while i < nr_linfo {
            if insn_off < (*linfo).insn_off {
                break;
            }

            raw_linfo = (raw_linfo as *const u8).add(rec_size as usize) as *const c_void;
            linfo = raw_linfo as *const bpf_line_info;
            i = i.wrapping_add(1);
        }

        (raw_linfo as *const u8).sub(rec_size as usize) as *const bpf_line_info
    }
}
