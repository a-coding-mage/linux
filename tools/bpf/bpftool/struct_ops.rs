// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2020 Facebook */

/* Translated from bpf/bpftool/struct_ops.c.
 * C include dependencies are expected to be provided by the surrounding build:
 * errno, stdio, unistd, linux/err, bpf/bpf, bpf/btf, bpf/libbpf,
 * json_writer, and bpftool main declarations.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type __s32 = i32;
type size_t = usize;
type bool_t = bool;
type json_writer_t = json_writer;

const STRUCT_OPS_VALUE_PREFIX: &[u8] = b"bpf_struct_ops_\0";
const BTF_KIND_STRUCT: c_uint = 4;
const BPF_MAP_TYPE_STRUCT_OPS: c_uint = 26;
const BPF_F_LINK: __u32 = 1 << 13;
const ENOENT: c_int = 2;
const UINT32_MAX_: c_ulong = u32::MAX as c_ulong;
const PATH_MAX: usize = 4096;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size: __u32,
}

#[repr(C)]
pub struct bpf_map_info {
    pub type_: __u32,
    pub id: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
    pub name: [c_char; 16],
    pub ifindex: __u32,
    pub btf_vmlinux_value_type_id: __u32,
}

#[repr(C)]
pub struct bpf_link_info {
    pub type_: __u32,
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_object_open_opts {
    pub sz: size_t,
    pub kernel_log_level: c_uint,
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct json_writer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_dumper {
    pub btf: *const btf,
    pub jw: *mut json_writer,
    pub is_plain_text: bool_t,
    pub prog_id_as_func_ptr: bool_t,
}

#[repr(C)]
pub struct cmd {
    pub cmd: *const c_char,
    pub func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

#[repr(C)]
struct res {
    nr_maps: c_uint,
    nr_errs: c_uint,
}

type work_func = Option<
    unsafe extern "C" fn(
        c_int,
        *const bpf_map_info,
        *mut c_void,
        *mut json_writer,
    ) -> c_int,
>;

unsafe extern "C" {
    static mut errno: c_int;
    static mut json_output: bool_t;
    static mut json_wtr: *mut json_writer;
    static mut verifier_logs: bool_t;
    static mut bin_name: *const c_char;
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;

    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    fn p_err(format: *const c_char, ...);
    fn p_info(format: *const c_char, ...);
    fn usage() -> !;
    fn is_prefix(str: *const c_char, prefix: *const c_char) -> bool_t;
    fn cmd_select(
        cmds: *const cmd,
        argc: c_int,
        argv: *mut *mut c_char,
        help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int,
    ) -> c_int;
    fn set_max_rlimit();
    fn create_and_mount_bpffs_dir(dir: *const c_char) -> c_int;
    fn pathname_concat(
        buf: *mut c_char,
        buf_sz: size_t,
        path: *const c_char,
        name: *const c_char,
    ) -> c_int;

    fn libbpf_find_kernel_btf() -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__type_by_id(btf: *const btf, id: __u32) -> *const btf_type;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__find_by_name_kind(btf: *const btf, name: *const c_char, kind: c_uint) -> __s32;
    fn btf_dumper_type(d: *mut btf_dumper, type_id: __s32, data: *mut c_void);

    fn bpf_map_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *mut __u32) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_link_get_info_by_fd(fd: c_int, info: *mut bpf_link_info, info_len: *mut __u32) -> c_int;

    fn bpf_object__open_file(file: *const c_char, opts: *const bpf_object_open_opts) -> *mut bpf_object;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_object__next_map(obj: *const bpf_object, map: *mut bpf_map) -> *mut bpf_map;
    fn bpf_map__type(map: *const bpf_map) -> c_uint;
    fn bpf_map__attach_struct_ops(map: *const bpf_map) -> *mut bpf_link;
    fn bpf_map__name(map: *const bpf_map) -> *const c_char;
    fn bpf_map__fd(map: *const bpf_map) -> c_int;
    fn bpf_map__map_flags(map: *const bpf_map) -> __u32;
    fn bpf_link__pin(link: *mut bpf_link, path: *const c_char) -> c_int;
    fn bpf_link__fd(link: *const bpf_link) -> c_int;
    fn bpf_link__disconnect(link: *mut bpf_link);
    fn bpf_link__destroy(link: *mut bpf_link);

    fn jsonw_start_array(wtr: *mut json_writer);
    fn jsonw_end_array(wtr: *mut json_writer);
    fn jsonw_start_object(wtr: *mut json_writer);
    fn jsonw_end_object(wtr: *mut json_writer);
    fn jsonw_uint_field(wtr: *mut json_writer, name: *const c_char, value: c_uint);
    fn jsonw_string_field(wtr: *mut json_writer, name: *const c_char, value: *const c_char);
    fn jsonw_name(wtr: *mut json_writer, name: *const c_char);
    fn jsonw_null(wtr: *mut json_writer);
    fn jsonw_new(stream: *mut c_void) -> *mut json_writer;
    fn jsonw_pretty(wtr: *mut json_writer, pretty: bool_t);
    fn jsonw_destroy(wtr: *mut *mut json_writer);
}

static mut map_info_type: *const btf_type = ptr::null();
static mut map_info_alloc_len: __u32 = 0;
static mut btf_vmlinux: *mut btf = ptr::null_mut();
static mut map_info_type_id: __s32 = 0;

unsafe fn get_arg(argc: *mut c_int, argv: *mut *mut *mut c_char) -> *const c_char {
    let arg = **argv;
    *argv = (*argv).add(1);
    *argc -= 1;
    arg as *const c_char
}

unsafe fn get_btf_vmlinux() -> *const btf {
    if !btf_vmlinux.is_null() {
        return btf_vmlinux;
    }

    btf_vmlinux = libbpf_find_kernel_btf();
    if btf_vmlinux.is_null() {
        p_err(b"struct_ops requires kernel CONFIG_DEBUG_INFO_BTF=y\0".as_ptr() as *const c_char);
    }

    btf_vmlinux
}

unsafe fn get_kern_struct_ops_name(info: *const bpf_map_info) -> *const c_char {
    let kern_btf: *const btf;
    let t: *const btf_type;
    let mut st_ops_name: *const c_char;

    kern_btf = get_btf_vmlinux();
    if kern_btf.is_null() {
        return b"<btf_vmlinux_not_found>\0".as_ptr() as *const c_char;
    }

    t = btf__type_by_id(kern_btf, (*info).btf_vmlinux_value_type_id);
    st_ops_name = btf__name_by_offset(kern_btf, (*t).name_off);
    st_ops_name = st_ops_name.add(strlen(STRUCT_OPS_VALUE_PREFIX.as_ptr() as *const c_char));

    st_ops_name
}

unsafe fn get_map_info_type_id() -> __s32 {
    let kern_btf: *const btf;

    if map_info_type_id != 0 {
        return map_info_type_id;
    }

    kern_btf = get_btf_vmlinux();
    if kern_btf.is_null() {
        return 0;
    }

    map_info_type_id = btf__find_by_name_kind(
        kern_btf,
        b"bpf_map_info\0".as_ptr() as *const c_char,
        BTF_KIND_STRUCT,
    );
    if map_info_type_id < 0 {
        p_err(b"can't find bpf_map_info from btf_vmlinux\0".as_ptr() as *const c_char);
        return map_info_type_id;
    }
    map_info_type = btf__type_by_id(kern_btf, map_info_type_id as __u32);

    /* Ensure map_info_alloc() has at least what the bpftool needs */
    map_info_alloc_len = (*map_info_type).size;
    if (map_info_alloc_len as usize) < size_of::<bpf_map_info>() {
        map_info_alloc_len = size_of::<bpf_map_info>() as __u32;
    }

    map_info_type_id
}

/* If the subcmd needs to print out the bpf_map_info,
 * it should always call map_info_alloc to allocate
 * a bpf_map_info object instead of allocating it
 * on the stack.
 *
 * map_info_alloc() will take the running kernel's btf
 * into account.  i.e. it will consider the
 * sizeof(struct bpf_map_info) of the running kernel.
 *
 * It will enable the "struct_ops" cmd to print the latest
 * "struct bpf_map_info".
 *
 * [ Recall that "struct_ops" requires the kernel's btf to
 *   be available ]
 */
unsafe fn map_info_alloc(alloc_len: *mut __u32) -> *mut bpf_map_info {
    let info: *mut bpf_map_info;

    if get_map_info_type_id() < 0 {
        return ptr::null_mut();
    }

    info = calloc(1, map_info_alloc_len as size_t) as *mut bpf_map_info;
    if info.is_null() {
        p_err(b"mem alloc failed\0".as_ptr() as *const c_char);
    } else {
        *alloc_len = map_info_alloc_len;
    }

    info
}

/* It iterates all struct_ops maps of the system.
 * It returns the fd in "*res_fd" and map_info in "*info".
 * In the very first iteration, info->id should be 0.
 * An optional map "*name" filter can be specified.
 * The filter can be made more flexible in the future.
 * e.g. filter by kernel-struct-ops-name, regex-name, glob-name, ...etc.
 *
 * Return value:
 *     1: A struct_ops map found.  It is returned in "*res_fd" and "*info".
 *        The caller can continue to call get_next in the future.
 *     0: No struct_ops map is returned.
 *        All struct_ops map has been found.
 *    -1: Error and the caller should abort the iteration.
 */
unsafe fn get_next_struct_ops_map(
    name: *const c_char,
    res_fd: *mut c_int,
    info: *mut bpf_map_info,
    mut info_len: __u32,
) -> c_int {
    let mut id: __u32 = (*info).id;
    let mut err: c_int;
    let fd: c_int;

    loop {
        err = bpf_map_get_next_id(id, &mut id);
        if err != 0 {
            if errno == ENOENT {
                return 0;
            }
            p_err(b"can't get next map: %s\0".as_ptr() as *const c_char, strerror(errno));
            return -1;
        }

        fd = bpf_map_get_fd_by_id(id);
        if fd < 0 {
            if errno == ENOENT {
                continue;
            }
            p_err(
                b"can't get map by id (%u): %s\0".as_ptr() as *const c_char,
                id,
                strerror(errno),
            );
            return -1;
        }

        err = bpf_map_get_info_by_fd(fd, info, &mut info_len);
        if err != 0 {
            p_err(b"can't get map info: %s\0".as_ptr() as *const c_char, strerror(errno));
            close(fd);
            return -1;
        }

        if (*info).type_ == BPF_MAP_TYPE_STRUCT_OPS
            && (name.is_null() || strcmp(name, (*info).name.as_ptr()) == 0)
        {
            *res_fd = fd;
            return 1;
        }
        close(fd);
    }
}

unsafe fn cmd_retval(res: *const res, must_have_one_map: bool_t) -> c_int {
    if (*res).nr_errs != 0 || ((*res).nr_maps == 0) && must_have_one_map {
        return -1;
    }

    0
}

/* "data" is the work_func private storage */

/* Find all struct_ops map in the system.
 * Filter out by "name" (if specified).
 * Then call "func(fd, info, data, wtr)" on each struct_ops map found.
 */
unsafe fn do_search(
    name: *const c_char,
    func: work_func,
    data: *mut c_void,
    wtr: *mut json_writer,
) -> res {
    let info: *mut bpf_map_info;
    let mut res = res { nr_maps: 0, nr_errs: 0 };
    let mut info_len: __u32 = 0;
    let mut fd: c_int = 0;
    let mut err: c_int;

    info = map_info_alloc(&mut info_len);
    if info.is_null() {
        res.nr_errs += 1;
        return res;
    }

    if !wtr.is_null() {
        jsonw_start_array(wtr);
    }
    loop {
        err = get_next_struct_ops_map(name, &mut fd, info, info_len);
        if err != 1 {
            break;
        }
        res.nr_maps += 1;
        err = func.unwrap()(fd, info, data, wtr);
        if err != 0 {
            res.nr_errs += 1;
        }
        close(fd);
    }
    if !wtr.is_null() {
        jsonw_end_array(wtr);
    }

    if err != 0 {
        res.nr_errs += 1;
    }

    if wtr.is_null() && !name.is_null() && res.nr_errs == 0 && res.nr_maps == 0 {
        /* It is not printing empty [].
         * Thus, needs to specifically say nothing found
         * for "name" here.
         */
        p_err(b"no struct_ops found for %s\0".as_ptr() as *const c_char, name);
    } else if wtr.is_null() && json_output && res.nr_errs == 0 {
        /* The "func()" above is not writing any json (i.e. !wtr
         * test here).
         *
         * However, "-j" is enabled and there is no errs here,
         * so call json_null() as the current convention of
         * other cmds.
         */
        jsonw_null(json_wtr);
    }

    free(info as *mut c_void);
    res
}

unsafe fn do_one_id(
    id_str: *const c_char,
    func: work_func,
    data: *mut c_void,
    wtr: *mut json_writer,
) -> res {
    let info: *mut bpf_map_info;
    let mut res = res { nr_maps: 0, nr_errs: 0 };
    let id: c_ulong;
    let mut info_len: __u32 = 0;
    let mut endptr: *mut c_char = ptr::null_mut();
    let fd: c_int;

    id = strtoul(id_str, &mut endptr, 0);
    if *endptr != 0 || id == 0 || id > UINT32_MAX_ {
        p_err(b"invalid id %s\0".as_ptr() as *const c_char, id_str);
        res.nr_errs += 1;
        return res;
    }

    fd = bpf_map_get_fd_by_id(id as __u32);
    if fd < 0 {
        p_err(b"can't get map by id (%lu): %s\0".as_ptr() as *const c_char, id, strerror(errno));
        res.nr_errs += 1;
        return res;
    }

    info = map_info_alloc(&mut info_len);
    if info.is_null() {
        res.nr_errs += 1;
        free(info as *mut c_void);
        close(fd);
        return res;
    }

    if bpf_map_get_info_by_fd(fd, info, &mut info_len) != 0 {
        p_err(b"can't get map info: %s\0".as_ptr() as *const c_char, strerror(errno));
        res.nr_errs += 1;
        free(info as *mut c_void);
        close(fd);
        return res;
    }

    if (*info).type_ != BPF_MAP_TYPE_STRUCT_OPS {
        p_err(
            b"%s id %u is not a struct_ops map\0".as_ptr() as *const c_char,
            (*info).name.as_ptr(),
            (*info).id,
        );
        res.nr_errs += 1;
        free(info as *mut c_void);
        close(fd);
        return res;
    }

    res.nr_maps += 1;

    if !wtr.is_null() {
        jsonw_start_array(wtr);
    }

    if func.unwrap()(fd, info, data, wtr) != 0 {
        res.nr_errs += 1;
    } else if wtr.is_null() && json_output {
        /* The "func()" above is not writing any json (i.e. !wtr
         * test here).
         *
         * However, "-j" is enabled and there is no errs here,
         * so call json_null() as the current convention of
         * other cmds.
         */
        jsonw_null(json_wtr);
    }

    if !wtr.is_null() {
        jsonw_end_array(wtr);
    }

    free(info as *mut c_void);
    close(fd);

    res
}

unsafe fn do_work_on_struct_ops(
    search_type: *const c_char,
    search_term: *const c_char,
    func: work_func,
    data: *mut c_void,
    wtr: *mut json_writer,
) -> res {
    if !search_type.is_null() {
        if is_prefix(search_type, b"id\0".as_ptr() as *const c_char) {
            return do_one_id(search_term, func, data, wtr);
        } else if !is_prefix(search_type, b"name\0".as_ptr() as *const c_char) {
            usage();
        }
    }

    do_search(search_term, func, data, wtr)
}

unsafe extern "C" fn __do_show(
    _fd: c_int,
    info: *const bpf_map_info,
    _data: *mut c_void,
    wtr: *mut json_writer,
) -> c_int {
    if !wtr.is_null() {
        jsonw_start_object(wtr);
        jsonw_uint_field(wtr, b"id\0".as_ptr() as *const c_char, (*info).id);
        jsonw_string_field(wtr, b"name\0".as_ptr() as *const c_char, (*info).name.as_ptr());
        jsonw_string_field(
            wtr,
            b"kernel_struct_ops\0".as_ptr() as *const c_char,
            get_kern_struct_ops_name(info),
        );
        jsonw_end_object(wtr);
    } else {
        printf(
            b"%u: %-15s %-32s\n\0".as_ptr() as *const c_char,
            (*info).id,
            (*info).name.as_ptr(),
            get_kern_struct_ops_name(info),
        );
    }

    0
}

unsafe extern "C" fn do_show(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut search_type: *const c_char = ptr::null();
    let mut search_term: *const c_char = ptr::null();
    let res: res;

    if argc != 0 && argc != 2 {
        usage();
    }

    if argc == 2 {
        search_type = get_arg(&mut argc, &mut argv);
        search_term = get_arg(&mut argc, &mut argv);
    }

    res = do_work_on_struct_ops(search_type, search_term, Some(__do_show), ptr::null_mut(), json_wtr);

    cmd_retval(&res, !search_term.is_null())
}

unsafe extern "C" fn __do_dump(
    fd: c_int,
    info: *const bpf_map_info,
    data: *mut c_void,
    wtr: *mut json_writer,
) -> c_int {
    let d: *mut btf_dumper = data as *mut btf_dumper;
    let struct_ops_type: *const btf_type;
    let mut kern_btf: *const btf = (*d).btf;
    let struct_ops_name: *const c_char;
    let zero: c_int = 0;
    let value: *mut c_void;

    /* note: d->jw == wtr */

    kern_btf = (*d).btf;

    /* The kernel supporting BPF_MAP_TYPE_STRUCT_OPS must have
     * btf_vmlinux_value_type_id.
     */
    struct_ops_type = btf__type_by_id(kern_btf, (*info).btf_vmlinux_value_type_id);
    struct_ops_name = btf__name_by_offset(kern_btf, (*struct_ops_type).name_off);
    value = calloc(1, (*info).value_size as size_t);
    if value.is_null() {
        p_err(b"mem alloc failed\0".as_ptr() as *const c_char);
        return -1;
    }

    if bpf_map_lookup_elem(fd, &zero as *const c_int as *const c_void, value) != 0 {
        p_err(
            b"can't lookup struct_ops map %s id %u\0".as_ptr() as *const c_char,
            (*info).name.as_ptr(),
            (*info).id,
        );
        free(value);
        return -1;
    }

    jsonw_start_object(wtr);
    jsonw_name(wtr, b"bpf_map_info\0".as_ptr() as *const c_char);
    btf_dumper_type(d, map_info_type_id, info as *mut c_void);
    jsonw_end_object(wtr);

    jsonw_start_object(wtr);
    jsonw_name(wtr, struct_ops_name);
    btf_dumper_type(d, (*info).btf_vmlinux_value_type_id as __s32, value);
    jsonw_end_object(wtr);

    free(value);

    0
}

unsafe extern "C" fn do_dump(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut search_type: *const c_char = ptr::null();
    let mut search_term: *const c_char = ptr::null();
    let mut wtr: *mut json_writer_t = json_wtr;
    let kern_btf: *const btf;
    let mut d = btf_dumper {
        btf: ptr::null(),
        jw: ptr::null_mut(),
        is_plain_text: false,
        prog_id_as_func_ptr: false,
    };
    let res: res;

    if argc != 0 && argc != 2 {
        usage();
    }

    if argc == 2 {
        search_type = get_arg(&mut argc, &mut argv);
        search_term = get_arg(&mut argc, &mut argv);
    }

    kern_btf = get_btf_vmlinux();
    if kern_btf.is_null() {
        return -1;
    }

    if !json_output {
        wtr = jsonw_new(stdout);
        if wtr.is_null() {
            p_err(b"can't create json writer\0".as_ptr() as *const c_char);
            return -1;
        }
        jsonw_pretty(wtr, true);
    }

    d.btf = kern_btf;
    d.jw = wtr;
    d.is_plain_text = !json_output;
    d.prog_id_as_func_ptr = true;

    res = do_work_on_struct_ops(search_type, search_term, Some(__do_dump), &mut d as *mut _ as *mut c_void, wtr);

    if !json_output {
        jsonw_destroy(&mut wtr);
    }

    cmd_retval(&res, !search_term.is_null())
}

unsafe extern "C" fn __do_unregister(
    fd: c_int,
    info: *const bpf_map_info,
    _data: *mut c_void,
    _wtr: *mut json_writer,
) -> c_int {
    let zero: c_int = 0;

    if bpf_map_delete_elem(fd, &zero as *const c_int as *const c_void) != 0 {
        p_err(
            b"can't unload %s %s id %u: %s\0".as_ptr() as *const c_char,
            get_kern_struct_ops_name(info),
            (*info).name.as_ptr(),
            (*info).id,
            strerror(errno),
        );
        return -1;
    }

    p_info(
        b"Unregistered %s %s id %u\0".as_ptr() as *const c_char,
        get_kern_struct_ops_name(info),
        (*info).name.as_ptr(),
        (*info).id,
    );

    0
}

unsafe extern "C" fn do_unregister(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let search_type: *const c_char;
    let search_term: *const c_char;
    let res: res;

    if argc != 2 {
        usage();
    }

    search_type = get_arg(&mut argc, &mut argv);
    search_term = get_arg(&mut argc, &mut argv);

    res = do_work_on_struct_ops(search_type, search_term, Some(__do_unregister), ptr::null_mut(), ptr::null_mut());

    cmd_retval(&res, true)
}

unsafe fn pin_link(link: *mut bpf_link, pindir: *const c_char, name: *const c_char) -> c_int {
    let mut pinfile = [0 as c_char; PATH_MAX];
    let err: c_int;

    err = pathname_concat(pinfile.as_mut_ptr(), size_of::<[c_char; PATH_MAX]>(), pindir, name);
    if err != 0 {
        return -1;
    }

    bpf_link__pin(link, pinfile.as_ptr())
}

unsafe extern "C" fn do_register(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut open_opts = bpf_object_open_opts {
        sz: size_of::<bpf_object_open_opts>(),
        kernel_log_level: 0,
    };
    let mut link_info_len: __u32 = size_of::<bpf_link_info>() as __u32;
    let mut link_info = bpf_link_info { type_: 0, id: 0 };
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut info_len: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut nr_errs: c_int = 0;
    let mut nr_maps: c_int = 0;
    let mut linkdir: *const c_char = ptr::null();
    let obj: *mut bpf_object;
    let mut link: *mut bpf_link;
    let mut map: *mut bpf_map;
    let file: *const c_char;

    if argc != 1 && argc != 2 {
        usage();
    }

    file = get_arg(&mut argc, &mut argv);
    if argc == 1 {
        linkdir = get_arg(&mut argc, &mut argv);
    }

    if !linkdir.is_null() && create_and_mount_bpffs_dir(linkdir) != 0 {
        p_err(b"can't mount bpffs for pinning\0".as_ptr() as *const c_char);
        return -1;
    }

    if verifier_logs {
        /* log_level1 + log_level2 + stats, but not stable UAPI */
        open_opts.kernel_log_level = 1 + 2 + 4;
    }

    obj = bpf_object__open_file(file, &open_opts);
    if obj.is_null() {
        return -1;
    }

    set_max_rlimit();

    if bpf_object__load(obj) != 0 {
        bpf_object__close(obj);
        return -1;
    }

    map = ptr::null_mut();
    loop {
        map = bpf_object__next_map(obj, map);
        if map.is_null() {
            break;
        }
        if bpf_map__type(map) != BPF_MAP_TYPE_STRUCT_OPS {
            continue;
        }

        link = bpf_map__attach_struct_ops(map);
        if link.is_null() {
            p_err(
                b"can't register struct_ops %s: %s\0".as_ptr() as *const c_char,
                bpf_map__name(map),
                strerror(errno),
            );
            nr_errs += 1;
            continue;
        }
        nr_maps += 1;

        if bpf_map_get_info_by_fd(bpf_map__fd(map), &mut info, &mut info_len) != 0 {
            /* Not p_err.  The struct_ops was attached
             * successfully.
             */
            p_info(
                b"Registered %s but can't find id: %s\0".as_ptr() as *const c_char,
                bpf_map__name(map),
                strerror(errno),
            );
            bpf_link__disconnect(link);
            bpf_link__destroy(link);
            continue;
        }
        if (bpf_map__map_flags(map) & BPF_F_LINK) == 0 {
            p_info(
                b"Registered %s %s id %u\0".as_ptr() as *const c_char,
                get_kern_struct_ops_name(&info),
                info.name.as_ptr(),
                info.id,
            );
            bpf_link__disconnect(link);
            bpf_link__destroy(link);
            continue;
        }
        if bpf_link_get_info_by_fd(bpf_link__fd(link), &mut link_info, &mut link_info_len) != 0 {
            p_err(
                b"Registered %s but can't find link id: %s\0".as_ptr() as *const c_char,
                bpf_map__name(map),
                strerror(errno),
            );
            nr_errs += 1;
            bpf_link__disconnect(link);
            bpf_link__destroy(link);
            continue;
        }
        if !linkdir.is_null() && pin_link(link, linkdir, info.name.as_ptr()) != 0 {
            p_err(
                b"can't pin link %u for %s: %s\0".as_ptr() as *const c_char,
                link_info.id,
                info.name.as_ptr(),
                strerror(errno),
            );
            nr_errs += 1;
            bpf_link__disconnect(link);
            bpf_link__destroy(link);
            continue;
        }
        p_info(
            b"Registered %s %s map id %u link id %u\0".as_ptr() as *const c_char,
            get_kern_struct_ops_name(&info),
            info.name.as_ptr(),
            info.id,
            link_info.id,
        );

        bpf_link__disconnect(link);
        bpf_link__destroy(link);
    }

    bpf_object__close(obj);

    if nr_errs != 0 {
        return -1;
    }

    if nr_maps == 0 {
        p_err(b"no struct_ops found in %s\0".as_ptr() as *const c_char, file);
        return -1;
    }

    if json_output {
        jsonw_null(json_wtr);
    }

    0
}

unsafe extern "C" fn do_help(_argc: c_int, argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }

    fprintf(
        stderr,
        b"Usage: %1$s %2$s { show | list } [STRUCT_OPS_MAP]\n       %1$s %2$s dump [STRUCT_OPS_MAP]\n       %1$s %2$s register OBJ [LINK_DIR]\n       %1$s %2$s unregister STRUCT_OPS_MAP\n       %1$s %2$s help\n\n       STRUCT_OPS_MAP := [ id STRUCT_OPS_MAP_ID | name STRUCT_OPS_MAP_NAME ]\n       \0".as_ptr() as *const c_char,
        bin_name,
        *argv.offset(-2),
    );
    fprintf(stderr, b"%s }\n\0".as_ptr() as *const c_char, b"HELP_SPEC_OPTIONS\0".as_ptr() as *const c_char);

    0
}

static cmds: [cmd; 7] = [
    cmd { cmd: b"show\0".as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: b"list\0".as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: b"register\0".as_ptr() as *const c_char, func: Some(do_register) },
    cmd { cmd: b"unregister\0".as_ptr() as *const c_char, func: Some(do_unregister) },
    cmd { cmd: b"dump\0".as_ptr() as *const c_char, func: Some(do_dump) },
    cmd { cmd: b"help\0".as_ptr() as *const c_char, func: Some(do_help) },
    cmd { cmd: ptr::null(), func: None },
];

#[no_mangle]
pub unsafe extern "C" fn do_struct_ops(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let err: c_int;

    err = cmd_select(cmds.as_ptr(), argc, argv, do_help);

    btf__free(btf_vmlinux);
    btf_vmlinux = ptr::null_mut();
    map_info_type = ptr::null();
    map_info_alloc_len = 0;
    map_info_type_id = 0;

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
