// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

type __u32 = u32;
type error_t = c_int;

#[repr(C)]
pub struct argp_option {
    pub name: *const c_char,
    pub key: c_int,
    pub arg: *const c_char,
    pub flags: c_int,
    pub doc: *const c_char,
    pub group: c_int,
}

#[repr(C)]
pub struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
}

#[repr(C)]
pub struct bench_res {
    pub hits: u64,
    pub important_hits: u64,
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub argp: *const argp,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_progress: Option<unsafe extern "C" fn(*mut bench_res, *mut bench_res)>,
    pub report_final: Option<unsafe extern "C" fn(*mut bench_res)>,
}

#[repr(C)]
pub struct env_t {
    pub producer_cnt: c_int,
    pub consumer_cnt: c_int,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct local_storage_bench_rodata {
    pub num_maps: __u32,
    pub hashmap_num_keys: __u32,
    pub use_hashmap: c_int,
    pub interleave: c_int,
}

#[repr(C)]
pub struct local_storage_bench_bss {
    pub hits: u64,
    pub important_hits: u64,
}

#[repr(C)]
pub struct local_storage_bench_maps {
    pub array_of_hash_maps: *mut bpf_map,
    pub array_of_local_storage_maps: *mut bpf_map,
}

#[repr(C)]
pub struct local_storage_bench_progs {
    pub get_local: *mut bpf_program,
}

#[repr(C)]
pub struct local_storage_bench {
    pub obj: *mut bpf_object,
    pub maps: local_storage_bench_maps,
    pub progs: local_storage_bench_progs,
    pub rodata: *mut local_storage_bench_rodata,
    pub bss: *mut local_storage_bench_bss,
}

#[repr(C)]
pub struct bpf_map_create_opts {
    pub sz: usize,
    pub btf_fd: c_int,
    pub btf_key_type_id: u32,
    pub btf_value_type_id: u32,
    pub map_flags: u32,
}

unsafe extern "C" {
    static mut env: env_t;
    static mut stderr: *mut FILE;
    static __NR_getpgid: c_long;

    static ARGP_ERR_UNKNOWN: error_t;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn argp_usage(state: *mut argp_state);
    fn exit(status: c_int) -> !;
    fn syscall(number: c_long, ...) -> c_long;

    fn setup_libbpf();
    fn local_storage_bench__open() -> *mut local_storage_bench;
    fn local_storage_bench__load(obj: *mut local_storage_bench) -> c_int;

    fn bpf_map__inner_map(map: *mut bpf_map) -> *mut bpf_map;
    fn bpf_map__btf_key_type_id(map: *mut bpf_map) -> u32;
    fn bpf_map__btf_value_type_id(map: *mut bpf_map) -> u32;
    fn bpf_object__btf_fd(obj: *mut bpf_object) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_create(
        map_type: c_uint,
        map_name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut c_void;

    fn atomic_swap(ptr: *mut u64, val: u64) -> u64;
    fn local_storage_report_progress(res: *mut bench_res, prev: *mut bench_res);
    fn local_storage_report_final(res: *mut bench_res);
}

#[repr(C)]
struct Args {
    nr_maps: __u32,
    hashmap_nr_keys_used: __u32,
}

static mut args: Args = Args {
    nr_maps: 1000,
    hashmap_nr_keys_used: 1000,
};

const ARG_NR_MAPS: c_int = 6000;
const ARG_HASHMAP_NR_KEYS_USED: c_int = 6001;

static opts: [argp_option; 3] = [
    argp_option {
        name: b"nr_maps\0".as_ptr() as *const c_char,
        key: ARG_NR_MAPS,
        arg: b"NR_MAPS\0".as_ptr() as *const c_char,
        flags: 0,
        doc: b"Set number of local_storage maps\0".as_ptr() as *const c_char,
        group: 0,
    },
    argp_option {
        name: b"hashmap_nr_keys_used\0".as_ptr() as *const c_char,
        key: ARG_HASHMAP_NR_KEYS_USED,
        arg: b"NR_KEYS\0".as_ptr() as *const c_char,
        flags: 0,
        doc: b"When doing hashmap test, set number of hashmap keys test uses\0".as_ptr()
            as *const c_char,
        group: 0,
    },
    argp_option {
        name: ptr::null(),
        key: 0,
        arg: ptr::null(),
        flags: 0,
        doc: ptr::null(),
        group: 0,
    },
];

unsafe extern "C" fn parse_arg(
    key: c_int,
    arg: *mut c_char,
    state: *mut argp_state,
) -> error_t {
    let ret: c_long;

    match key {
        ARG_NR_MAPS => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > c_uint::MAX as c_long {
                fprintf(stderr, b"invalid nr_maps\0".as_ptr() as *const c_char);
                argp_usage(state);
            }
            args.nr_maps = ret as __u32;
        }
        ARG_HASHMAP_NR_KEYS_USED => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > c_uint::MAX as c_long {
                fprintf(
                    stderr,
                    b"invalid hashmap_nr_keys_used\0".as_ptr() as *const c_char,
                );
                argp_usage(state);
            }
            args.hashmap_nr_keys_used = ret as __u32;
        }
        _ => {
            return ARGP_ERR_UNKNOWN;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub static bench_local_storage_argp: argp = argp {
    options: opts.as_ptr(),
    parser: Some(parse_arg),
};

/* Keep in sync w/ array of maps in bpf */
const MAX_NR_MAPS: __u32 = 1000;
/* keep in sync w/ same define in bpf */
const HASHMAP_SZ: __u32 = 4194304;

unsafe extern "C" fn validate() {
    if env.producer_cnt != 1 {
        fprintf(
            stderr,
            b"benchmark doesn't support multi-producer!\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }
    if env.consumer_cnt != 0 {
        fprintf(
            stderr,
            b"benchmark doesn't support consumer!\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }

    if args.nr_maps > MAX_NR_MAPS {
        fprintf(
            stderr,
            b"nr_maps must be <= 1000\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }

    if args.hashmap_nr_keys_used > HASHMAP_SZ {
        fprintf(
            stderr,
            b"hashmap_nr_keys_used must be <= %u\n\0".as_ptr() as *const c_char,
            HASHMAP_SZ,
        );
        exit(1);
    }
}

#[repr(C)]
struct Ctx {
    skel: *mut local_storage_bench,
    bpf_obj: *mut c_void,
    array_of_maps: *mut bpf_map,
}

static mut ctx: Ctx = Ctx {
    skel: ptr::null_mut(),
    bpf_obj: ptr::null_mut(),
    array_of_maps: ptr::null_mut(),
};

unsafe extern "C" fn prepopulate_hashmap(fd: c_int) {
    let mut i: c_int;
    let mut key: c_int;
    let mut val: c_int;

    /* local_storage gets will have BPF_LOCAL_STORAGE_GET_F_CREATE flag set, so
     * populate the hashmap for a similar comparison
     */
    i = 0;
    while i < HASHMAP_SZ as c_int {
        key = i;
        val = i;
        if bpf_map_update_elem(
            fd,
            &key as *const c_int as *const c_void,
            &val as *const c_int as *const c_void,
            0,
        ) != 0
        {
            fprintf(
                stderr,
                b"Error prepopulating hashmap (key %d)\n\0".as_ptr() as *const c_char,
                key,
            );
            exit(1);
        }
        i += 1;
    }
}

const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_MAP_TYPE_HASH: c_uint = 1;
const BPF_MAP_TYPE_TASK_STORAGE: c_uint = 29;

unsafe extern "C" fn __setup(prog: *mut bpf_program, hashmap: bool) {
    let inner_map: *mut bpf_map;
    let mut i: c_int;
    let mut fd: c_int;
    let mim_fd: c_int;
    let mut err: c_int;

    let mut create_opts: bpf_map_create_opts = core::mem::zeroed();
    create_opts.sz = core::mem::size_of::<bpf_map_create_opts>();

    if !hashmap {
        create_opts.map_flags = BPF_F_NO_PREALLOC;
    }

    (*(*ctx.skel).rodata).num_maps = args.nr_maps;
    (*(*ctx.skel).rodata).hashmap_num_keys = args.hashmap_nr_keys_used;
    inner_map = bpf_map__inner_map(ctx.array_of_maps);
    create_opts.btf_key_type_id = bpf_map__btf_key_type_id(inner_map);
    create_opts.btf_value_type_id = bpf_map__btf_value_type_id(inner_map);

    err = local_storage_bench__load(ctx.skel);
    if err != 0 {
        fprintf(
            stderr,
            b"Error loading skeleton\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }

    create_opts.btf_fd = bpf_object__btf_fd((*ctx.skel).obj);

    mim_fd = bpf_map__fd(ctx.array_of_maps);
    if mim_fd < 0 {
        fprintf(
            stderr,
            b"Error getting map_in_map fd\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }

    i = 0;
    while i < args.nr_maps as c_int {
        if hashmap {
            fd = bpf_map_create(
                BPF_MAP_TYPE_HASH,
                ptr::null(),
                core::mem::size_of::<c_int>() as c_uint,
                core::mem::size_of::<c_int>() as c_uint,
                HASHMAP_SZ,
                &create_opts,
            );
        } else {
            fd = bpf_map_create(
                BPF_MAP_TYPE_TASK_STORAGE,
                ptr::null(),
                core::mem::size_of::<c_int>() as c_uint,
                core::mem::size_of::<c_int>() as c_uint,
                0,
                &create_opts,
            );
        }
        if fd < 0 {
            fprintf(
                stderr,
                b"Error creating map %d: %d\n\0".as_ptr() as *const c_char,
                i,
                fd,
            );
            exit(1);
        }

        if hashmap {
            prepopulate_hashmap(fd);
        }

        err = bpf_map_update_elem(
            mim_fd,
            &i as *const c_int as *const c_void,
            &fd as *const c_int as *const c_void,
            0,
        );
        if err != 0 {
            fprintf(
                stderr,
                b"Error updating array-of-maps w/ map %d\n\0".as_ptr() as *const c_char,
                i,
            );
            exit(1);
        }

        i += 1;
    }

    if bpf_program__attach(prog).is_null() {
        fprintf(
            stderr,
            b"Error attaching bpf program\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }
}

unsafe extern "C" fn hashmap_setup() {
    let skel: *mut local_storage_bench;

    setup_libbpf();

    skel = local_storage_bench__open();
    ctx.skel = skel;
    ctx.array_of_maps = (*skel).maps.array_of_hash_maps;
    (*(*skel).rodata).use_hashmap = 1;
    (*(*skel).rodata).interleave = 0;

    __setup((*skel).progs.get_local, true);
}

unsafe extern "C" fn local_storage_cache_get_setup() {
    let skel: *mut local_storage_bench;

    setup_libbpf();

    skel = local_storage_bench__open();
    ctx.skel = skel;
    ctx.array_of_maps = (*skel).maps.array_of_local_storage_maps;
    (*(*skel).rodata).use_hashmap = 0;
    (*(*skel).rodata).interleave = 0;

    __setup((*skel).progs.get_local, false);
}

unsafe extern "C" fn local_storage_cache_get_interleaved_setup() {
    let skel: *mut local_storage_bench;

    setup_libbpf();

    skel = local_storage_bench__open();
    ctx.skel = skel;
    ctx.array_of_maps = (*skel).maps.array_of_local_storage_maps;
    (*(*skel).rodata).use_hashmap = 0;
    (*(*skel).rodata).interleave = 1;

    __setup((*skel).progs.get_local, false);
}

unsafe extern "C" fn measure(res: *mut bench_res) {
    (*res).hits = atomic_swap(&mut (*(*ctx.skel).bss).hits, 0);
    (*res).important_hits = atomic_swap(&mut (*(*ctx.skel).bss).important_hits, 0);
}

#[inline]
unsafe extern "C" fn trigger_bpf_program() {
    syscall(__NR_getpgid);
}

unsafe extern "C" fn producer(_input: *mut c_void) -> *mut c_void {
    while true {
        trigger_bpf_program();
    }

    ptr::null_mut()
}

/* cache sequential and interleaved get benchs test local_storage get
 * performance, specifically they demonstrate performance cliff of
 * current list-plus-cache local_storage model.
 *
 * cache sequential get: call bpf_task_storage_get on n maps in order
 * cache interleaved get: like "sequential get", but interleave 4 calls to the
 *	'important' map (idx 0 in array_of_maps) for every 10 calls. Goal
 *	is to mimic environment where many progs are accessing their local_storage
 *	maps, with 'our' prog needing to access its map more often than others
 */
#[unsafe(no_mangle)]
pub static bench_local_storage_cache_seq_get: bench = bench {
    name: b"local-storage-cache-seq-get\0".as_ptr() as *const c_char,
    argp: &bench_local_storage_argp,
    validate: Some(validate),
    setup: Some(local_storage_cache_get_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(local_storage_report_progress),
    report_final: Some(local_storage_report_final),
};

#[unsafe(no_mangle)]
pub static bench_local_storage_cache_interleaved_get: bench = bench {
    name: b"local-storage-cache-int-get\0".as_ptr() as *const c_char,
    argp: &bench_local_storage_argp,
    validate: Some(validate),
    setup: Some(local_storage_cache_get_interleaved_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(local_storage_report_progress),
    report_final: Some(local_storage_report_final),
};

#[unsafe(no_mangle)]
pub static bench_local_storage_cache_hashmap_control: bench = bench {
    name: b"local-storage-cache-hashmap-control\0".as_ptr() as *const c_char,
    argp: &bench_local_storage_argp,
    validate: Some(validate),
    setup: Some(hashmap_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(local_storage_report_progress),
    report_final: Some(local_storage_report_final),
};
