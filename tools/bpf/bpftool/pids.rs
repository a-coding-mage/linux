// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2020 Facebook */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type ssize_t = isize;
type va_list = *mut c_void;
type json_writer_t = c_void;

const ENOTSUP: c_int = 95;
const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub key: c_ulonglong,
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct obj_ref {
    pub pid: c_int,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct obj_refs {
    pub refs: *mut obj_ref,
    pub ref_cnt: c_int,
    pub has_bpf_cookie: bool,
    pub bpf_cookie: __u64,
}

#[repr(C)]
pub struct pid_iter_entry {
    pub id: __u32,
    pub pid: c_int,
    pub comm: [c_char; 16],
    pub has_bpf_cookie: bool,
    pub bpf_cookie: __u64,
}

#[repr(C)]
pub struct pid_iter_bpf_rodata {
    pub obj_type: bpf_obj_type,
}

#[repr(C)]
pub struct pid_iter_bpf_links {
    pub iter: *mut bpf_link,
}

#[repr(C)]
pub struct pid_iter_bpf {
    pub rodata: *mut pid_iter_bpf_rodata,
    pub links: pid_iter_bpf_links,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bpf_obj_type {
    BPF_OBJ_UNKNOWN = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum libbpf_print_level {
    LIBBPF_WARN = 0,
    LIBBPF_INFO = 1,
    LIBBPF_DEBUG = 2,
}

type libbpf_print_fn_t = Option<
    unsafe extern "C" fn(level: libbpf_print_level, format: *const c_char, args: va_list) -> c_int,
>;

unsafe extern "C" {
    static mut errno: c_int;
    static mut verifier_logs: bool;

    static hash_fn_for_key_as_id: *mut c_void;
    static equal_fn_for_key_as_id: *mut c_void;

    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;

    fn p_err(format: *const c_char, ...);
    fn set_max_rlimit();

    fn hashmap__new(
        hash_fn: *const c_void,
        equal_fn: *const c_void,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    fn hashmap__append(map: *mut hashmap, key: c_ulonglong, value: *mut c_void) -> c_int;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__empty(map: *const hashmap) -> bool;
    fn hashmap__for_each_key_entry_first(
        map: *mut hashmap,
        key: c_ulonglong,
        entry: *mut *mut hashmap_entry,
    ) -> bool;
    fn hashmap__for_each_key_entry_next(
        map: *mut hashmap,
        key: c_ulonglong,
        entry: *mut *mut hashmap_entry,
    ) -> bool;
    fn hashmap__for_each_entry_first(
        map: *mut hashmap,
        entry: *mut *mut hashmap_entry,
        bkt: *mut size_t,
    ) -> bool;
    fn hashmap__for_each_entry_next(
        map: *mut hashmap,
        entry: *mut *mut hashmap_entry,
        bkt: *mut size_t,
    ) -> bool;

    fn pid_iter_bpf__open() -> *mut pid_iter_bpf;
    fn pid_iter_bpf__load(skel: *mut pid_iter_bpf) -> c_int;
    fn pid_iter_bpf__attach(skel: *mut pid_iter_bpf) -> c_int;
    fn pid_iter_bpf__destroy(skel: *mut pid_iter_bpf);
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;

    fn libbpf_set_print(fn_: libbpf_print_fn_t) -> libbpf_print_fn_t;

    fn jsonw_lluint_field(json_writer: *mut json_writer_t, prop: *const c_char, num: c_ulonglong);
    fn jsonw_name(json_writer: *mut json_writer_t, name: *const c_char);
    fn jsonw_start_array(json_writer: *mut json_writer_t);
    fn jsonw_start_object(json_writer: *mut json_writer_t);
    fn jsonw_int_field(json_writer: *mut json_writer_t, prop: *const c_char, num: c_int);
    fn jsonw_string_field(json_writer: *mut json_writer_t, prop: *const c_char, value: *const c_char);
    fn jsonw_end_object(json_writer: *mut json_writer_t);
    fn jsonw_end_array(json_writer: *mut json_writer_t);
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    let value = ptr as isize;
    value >= -4095isize && value < 0
}

/* BPFTOOL_WITHOUT_SKELETONS build condition:
 * int build_obj_refs_table(struct hashmap **map, enum bpf_obj_type type) { return -ENOTSUP; }
 * void delete_obj_refs_table(struct hashmap *map) {}
 * void emit_obj_refs_plain(struct hashmap *map, __u32 id, const char *prefix) {}
 * void emit_obj_refs_json(struct hashmap *map, __u32 id, json_writer_t *json_writer) {}
 */

unsafe fn add_ref(map: *mut hashmap, e: *mut pid_iter_entry) {
    let mut entry: *mut hashmap_entry = core::ptr::null_mut();
    let mut refs: *mut obj_refs;
    let mut ref_: *mut obj_ref;
    let mut err: c_int;
    let mut i: c_int;
    let mut tmp: *mut c_void;

    while hashmap__for_each_key_entry_first(map, (*e).id as c_ulonglong, &mut entry) {
        refs = (*entry).pvalue as *mut obj_refs;

        i = 0;
        while i < (*refs).ref_cnt {
            if (*(*refs).refs.add(i as usize)).pid == (*e).pid {
                return;
            }
            i += 1;
        }

        tmp = realloc(
            (*refs).refs as *mut c_void,
            (((*refs).ref_cnt + 1) as usize) * core::mem::size_of::<obj_ref>(),
        );
        if tmp.is_null() {
            p_err(
                c"failed to re-alloc memory for ID %u, PID %d, COMM %s...".as_ptr(),
                (*e).id,
                (*e).pid,
                (*e).comm.as_ptr(),
            );
            return;
        }
        (*refs).refs = tmp as *mut obj_ref;
        ref_ = (*refs).refs.add((*refs).ref_cnt as usize);
        (*ref_).pid = (*e).pid;
        memcpy(
            (*ref_).comm.as_mut_ptr() as *mut c_void,
            (*e).comm.as_ptr() as *const c_void,
            core::mem::size_of_val(&(*ref_).comm),
        );
        (*ref_).comm[core::mem::size_of_val(&(*ref_).comm) - 1] = 0;
        (*refs).ref_cnt += 1;

        return;
    }

    /* new ref */
    refs = calloc(1, core::mem::size_of::<obj_refs>()) as *mut obj_refs;
    if refs.is_null() {
        p_err(
            c"failed to alloc memory for ID %u, PID %d, COMM %s...".as_ptr(),
            (*e).id,
            (*e).pid,
            (*e).comm.as_ptr(),
        );
        return;
    }

    (*refs).refs = malloc(core::mem::size_of::<obj_ref>()) as *mut obj_ref;
    if (*refs).refs.is_null() {
        free(refs as *mut c_void);
        p_err(
            c"failed to alloc memory for ID %u, PID %d, COMM %s...".as_ptr(),
            (*e).id,
            (*e).pid,
            (*e).comm.as_ptr(),
        );
        return;
    }
    ref_ = (*refs).refs;
    (*ref_).pid = (*e).pid;
    memcpy(
        (*ref_).comm.as_mut_ptr() as *mut c_void,
        (*e).comm.as_ptr() as *const c_void,
        core::mem::size_of_val(&(*ref_).comm),
    );
    (*ref_).comm[core::mem::size_of_val(&(*ref_).comm) - 1] = 0;
    (*refs).ref_cnt = 1;
    (*refs).has_bpf_cookie = (*e).has_bpf_cookie;
    (*refs).bpf_cookie = (*e).bpf_cookie;

    err = hashmap__append(map, (*e).id as c_ulonglong, refs as *mut c_void);
    if err != 0 {
        p_err(
            c"failed to append entry to hashmap for ID %u: %s".as_ptr(),
            (*e).id,
            strerror(errno),
        );
    }
}

unsafe extern "C" fn libbpf_print_none(
    _level: libbpf_print_level,
    _format: *const c_char,
    _args: va_list,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_obj_refs_table(
    map: *mut *mut hashmap,
    type_: bpf_obj_type,
) -> c_int {
    let mut e: *mut pid_iter_entry;
    let mut buf: [u8; 4096 / core::mem::size_of::<pid_iter_entry>() * core::mem::size_of::<pid_iter_entry>()] =
        [0; 4096 / core::mem::size_of::<pid_iter_entry>() * core::mem::size_of::<pid_iter_entry>()];
    let mut skel: *mut pid_iter_bpf;
    let mut err: c_int;
    let mut ret: c_int;
    let mut fd: c_int = -1;
    let mut i: c_int;

    *map = hashmap__new(
        hash_fn_for_key_as_id as *const c_void,
        equal_fn_for_key_as_id as *const c_void,
        core::ptr::null_mut(),
    );
    if IS_ERR(*map as *const c_void) {
        p_err(c"failed to create hashmap for PID references".as_ptr());
        return -1;
    }
    set_max_rlimit();

    skel = pid_iter_bpf__open();
    if skel.is_null() {
        p_err(c"failed to open PID iterator skeleton".as_ptr());
        return -1;
    }

    (*(*skel).rodata).obj_type = type_;

    if !verifier_logs {
        let mut default_print: libbpf_print_fn_t;

        /* Unless debug information is on, we don't want the output to
         * be polluted with libbpf errors if bpf_iter is not supported.
         */
        default_print = libbpf_set_print(Some(libbpf_print_none));
        err = pid_iter_bpf__load(skel);
        libbpf_set_print(default_print);
    } else {
        err = pid_iter_bpf__load(skel);
    }
    if err != 0 {
        /* too bad, kernel doesn't support BPF iterators yet */
        err = 0;
        goto_out(fd, skel);
        return err;
    }
    err = pid_iter_bpf__attach(skel);
    if err != 0 {
        /* if we loaded above successfully, attach has to succeed */
        p_err(c"failed to attach PID iterator: %d".as_ptr(), err);
        goto_out(fd, skel);
        return err;
    }

    fd = bpf_iter_create(bpf_link__fd((*skel).links.iter));
    if fd < 0 {
        err = -errno;
        p_err(c"failed to create PID iterator session: %d".as_ptr(), err);
        goto_out(fd, skel);
        return err;
    }

    loop {
        ret = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) as c_int;
        if ret < 0 {
            if errno == EAGAIN {
                continue;
            }
            err = -errno;
            p_err(c"failed to read PID iterator output: %d".as_ptr(), err);
            goto_out(fd, skel);
            return err;
        }
        if ret == 0 {
            break;
        }
        if (ret as usize) % core::mem::size_of::<pid_iter_entry>() != 0 {
            err = -EINVAL;
            p_err(c"invalid PID iterator output format".as_ptr());
            goto_out(fd, skel);
            return err;
        }
        ret /= core::mem::size_of::<pid_iter_entry>() as c_int;

        e = buf.as_mut_ptr() as *mut pid_iter_entry;
        i = 0;
        while i < ret {
            add_ref(*map, e);
            i += 1;
            e = e.add(1);
        }
    }
    err = 0;
    goto_out(fd, skel);
    err
}

unsafe fn goto_out(fd: c_int, skel: *mut pid_iter_bpf) {
    if fd >= 0 {
        close(fd);
    }
    pid_iter_bpf__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_obj_refs_table(map: *mut hashmap) {
    let mut entry: *mut hashmap_entry = core::ptr::null_mut();
    let mut bkt: size_t = 0;

    if map.is_null() {
        return;
    }

    while hashmap__for_each_entry_first(map, &mut entry, &mut bkt) {
        let refs: *mut obj_refs = (*entry).pvalue as *mut obj_refs;

        free((*refs).refs as *mut c_void);
        free(refs as *mut c_void);

        if !hashmap__for_each_entry_next(map, &mut entry, &mut bkt) {
            break;
        }
    }

    hashmap__free(map);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emit_obj_refs_json(
    map: *mut hashmap,
    id: __u32,
    json_writer: *mut json_writer_t,
) {
    let mut entry: *mut hashmap_entry = core::ptr::null_mut();

    if hashmap__empty(map) {
        return;
    }

    while hashmap__for_each_key_entry_first(map, id as c_ulonglong, &mut entry) {
        let refs: *mut obj_refs = (*entry).pvalue as *mut obj_refs;
        let mut i: c_int;

        if (*refs).ref_cnt == 0 {
            break;
        }

        if (*refs).has_bpf_cookie {
            jsonw_lluint_field(
                json_writer,
                c"bpf_cookie".as_ptr(),
                (*refs).bpf_cookie as c_ulonglong,
            );
        }

        jsonw_name(json_writer, c"pids".as_ptr());
        jsonw_start_array(json_writer);
        i = 0;
        while i < (*refs).ref_cnt {
            let ref_: *mut obj_ref = (*refs).refs.add(i as usize);

            jsonw_start_object(json_writer);
            jsonw_int_field(json_writer, c"pid".as_ptr(), (*ref_).pid);
            jsonw_string_field(json_writer, c"comm".as_ptr(), (*ref_).comm.as_ptr());
            jsonw_end_object(json_writer);
            i += 1;
        }
        jsonw_end_array(json_writer);
        break;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emit_obj_refs_plain(map: *mut hashmap, id: __u32, prefix: *const c_char) {
    let mut entry: *mut hashmap_entry = core::ptr::null_mut();

    if hashmap__empty(map) {
        return;
    }

    while hashmap__for_each_key_entry_first(map, id as c_ulonglong, &mut entry) {
        let refs: *mut obj_refs = (*entry).pvalue as *mut obj_refs;
        let mut i: c_int;

        if (*refs).ref_cnt == 0 {
            break;
        }

        if (*refs).has_bpf_cookie {
            printf(
                c"\n\tbpf_cookie %llu".as_ptr(),
                (*refs).bpf_cookie as c_ulonglong,
            );
        }

        printf(c"%s".as_ptr(), prefix);
        i = 0;
        while i < (*refs).ref_cnt {
            let ref_: *mut obj_ref = (*refs).refs.add(i as usize);

            printf(
                c"%s%s(%d)".as_ptr(),
                if i == 0 {
                    c"".as_ptr()
                } else {
                    c", ".as_ptr()
                },
                (*ref_).comm.as_ptr(),
                (*ref_).pid,
            );
            i += 1;
        }
        break;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
