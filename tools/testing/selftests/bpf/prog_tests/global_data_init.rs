// SPDX-License-Identifier: GPL-2.0
// Translated from global_data_init.c. C includes were:
// <test_progs.h>, "bpf/libbpf_internal.h",
// "test_global_percpu_data.skel.h", and "test_global_percpu_data.lskel.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type ssize_t = isize;

const ENOMEM: c_int = 12;
const BPF_F_TEST_RUN_ON_CPU: __u32 = 1 << 0;
const BPF_F_CPU: __u64 = 1 << 1;
const BPF_F_RDONLY_PROG: __u32 = 1 << 3;
const BPF_F_ALL_CPUS: __u64 = 1 << 8;
const BPF_MAP_TYPE_PERCPU_ARRAY: c_uint = 6;
const BPF_PROG_TYPE_SOCKET_FILTER: c_uint = 1;
const BPF_DW: c_uint = 0x18;
const BPF_REG_0: c_uint = 0;
const BPF_REG_1: c_uint = 1;
const _SC_PAGE_SIZE: c_int = 30;
const FEAT_PERCPU_DATA: c_int = 0;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: size_t,
    pub ctx_in: *mut c_void,
    pub ctx_size_in: __u32,
    pub flags: __u32,
    pub cpu: c_int,
    pub retval: __u32,
}

#[repr(C)]
pub struct bpf_map_create_opts {
    pub sz: size_t,
    pub map_flags: __u32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub sz: size_t,
    pub log_buf: *mut c_char,
    pub log_size: __u32,
    pub log_level: __u32,
}

#[repr(C)]
pub struct bpf_iter_attach_opts {
    pub sz: size_t,
    pub link_info: *mut bpf_iter_link_info,
    pub link_info_len: __u32,
}

#[repr(C)]
pub struct bpf_iter_link_info_map {
    pub map_fd: c_int,
}

#[repr(C)]
pub union bpf_iter_link_info {
    pub map: bpf_iter_link_info_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_src_reg: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_global_percpu_data__struct_data {
    pub i: c_int,
    pub set: bool,
    pub nums: [c_int; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_global_percpu_data__percpu {
    pub cpu_id: [c_int; 1],
    pub data: c_int,
    pub set: bool,
    pub nums: [c_int; 7],
    pub struct_data: test_global_percpu_data__struct_data,
}

impl Default for test_global_percpu_data__struct_data {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

impl Default for test_global_percpu_data__percpu {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
pub struct test_global_percpu_data__bss {
    pub run: c_int,
    pub run_iter: bool,
    pub sum: c_int,
}

#[repr(C)]
pub struct test_global_percpu_data__rodata {
    pub num_cpus: c_int,
    pub num_off: size_t,
    pub elem_sz: size_t,
}

#[repr(C)]
pub struct test_global_percpu_data__maps {
    pub percpu_data: *mut bpf_map,
    pub data_percpu: *mut bpf_map,
    pub percpu_looooooooong: *mut bpf_map,
    pub percpu: *mut bpf_map,
}

#[repr(C)]
pub struct test_global_percpu_data__progs {
    pub update_percpu_data: *mut bpf_program,
    pub dump_percpu_data: *mut bpf_program,
}

#[repr(C)]
pub struct test_global_percpu_data {
    pub percpu: *mut test_global_percpu_data__percpu,
    pub data_percpu: *mut c_void,
    pub percpu_data: *mut c_void,
    pub percpu_looooooooong: *mut c_void,
    pub maps: test_global_percpu_data__maps,
    pub progs: test_global_percpu_data__progs,
    pub bss: *mut test_global_percpu_data__bss,
    pub rodata: *mut test_global_percpu_data__rodata,
}

#[repr(C)]
pub struct test_global_percpu_data_lskel__map {
    pub map_fd: c_int,
}

#[repr(C)]
pub struct test_global_percpu_data_lskel__prog {
    pub prog_fd: c_int,
}

#[repr(C)]
pub struct test_global_percpu_data_lskel__maps {
    pub percpu: test_global_percpu_data_lskel__map,
}

#[repr(C)]
pub struct test_global_percpu_data_lskel__progs {
    pub update_percpu_data: test_global_percpu_data_lskel__prog,
}

#[repr(C)]
pub struct test_global_percpu_data_lskel {
    pub maps: test_global_percpu_data_lskel__maps,
    pub progs: test_global_percpu_data_lskel__progs,
    pub bss: *mut test_global_percpu_data__bss,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn sysconf(name: c_int) -> isize;

    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__is_internal(map: *const bpf_map) -> bool;
    fn bpf_map__value_size(map: *const bpf_map) -> size_t;
    fn bpf_map__set_initial_value(map: *mut bpf_map, data: *const c_void, size: size_t) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_map__fd(map: *const bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn parse_cpu_mask_file(path: *const c_char, mask: *mut *mut bool, n: *mut c_int) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_map__lookup_elem(
        map: *const bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *mut c_void,
        value_sz: size_t,
        flags: __u64,
    ) -> c_int;
    fn bpf_map_lookup_elem_flags(fd: c_int, key: *const c_void, value: *mut c_void, flags: __u64) -> c_int;
    fn bpf_map__name(map: *const bpf_map) -> *const c_char;
    fn bpf_map__type(map: *const bpf_map) -> c_uint;
    fn bpf_map__initial_value(map: *const bpf_map, psize: *mut size_t) -> *mut c_void;
    fn bpf_map__set_value_size(map: *mut bpf_map, size: size_t) -> c_int;
    fn bpf_map__btf_value_type_id(map: *const bpf_map) -> c_int;
    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_map_create(
        map_type: c_uint,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map_freeze(fd: c_int) -> c_int;
    fn bpf_prog_load(
        prog_type: c_uint,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: size_t,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_program__attach_iter(prog: *const bpf_program, opts: *const bpf_iter_attach_opts) -> *mut bpf_link;
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_link__fd(link: *const bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn test_global_percpu_data__open() -> *mut test_global_percpu_data;
    fn test_global_percpu_data__load(skel: *mut test_global_percpu_data) -> c_int;
    fn test_global_percpu_data__destroy(skel: *mut test_global_percpu_data);
    fn test_global_percpu_data_lskel__open_and_load() -> *mut test_global_percpu_data_lskel;
    fn test_global_percpu_data_lskel__destroy(skel: *mut test_global_percpu_data_lskel);

    fn CHECK_FAIL(cond: bool) -> bool;
    fn CHECK(cond: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_NEQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_GT<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_GE<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_LT<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_FALSE(cond: bool, name: *const c_char) -> bool;
    fn ASSERT_TRUE(cond: bool, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
    fn RUN_TESTS(name: *const c_char);
    fn feat_supported(arg: *const c_void, feat: c_int) -> bool;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
}

const fn bpf_ld_map_value(dst: c_uint, src: c_uint, off: i16) -> bpf_insn {
    bpf_insn {
        code: 0,
        dst_src_reg: ((src as u8) << 4) | (dst as u8),
        off,
        imm: 0,
    }
}

const fn bpf_ldx_mem(_size: c_uint, dst: c_uint, src: c_uint, off: i16) -> bpf_insn {
    bpf_insn {
        code: 0,
        dst_src_reg: ((src as u8) << 4) | (dst as u8),
        off,
        imm: 0,
    }
}

const fn bpf_st_mem(_size: c_uint, dst: c_uint, off: i16, imm: i32) -> bpf_insn {
    bpf_insn {
        code: 0,
        dst_src_reg: dst as u8,
        off,
        imm,
    }
}

const fn bpf_exit_insn() -> bpf_insn {
    bpf_insn {
        code: 0,
        dst_src_reg: 0,
        off: 0,
        imm: 0,
    }
}

const fn roundup(x: size_t, y: size_t) -> size_t {
    ((x + y - 1) / y) * y
}

#[no_mangle]
pub unsafe extern "C" fn test_global_data_init() {
    let file = c"./test_global_data.bpf.o".as_ptr();
    let mut err: c_int = -ENOMEM;
    let mut map_fd: c_int;
    let zero: c_int = 0;
    let mut buff: *mut __u8 = core::ptr::null_mut();
    let mut newval: *mut __u8 = core::ptr::null_mut();
    let obj: *mut bpf_object;
    let map: *mut bpf_map;
    let _duration: __u32 = 0;
    let sz: size_t;

    obj = bpf_object__open_file(file, core::ptr::null());
    err = libbpf_get_error(obj as *const c_void);
    if CHECK_FAIL(err != 0) {
        return;
    }

    map = bpf_object__find_map_by_name(obj, c".rodata".as_ptr());
    if CHECK_FAIL(map.is_null() || !bpf_map__is_internal(map)) {
        goto_out(buff, newval, obj);
        return;
    }

    sz = bpf_map__value_size(map);
    newval = malloc(sz) as *mut __u8;
    if CHECK_FAIL(newval.is_null()) {
        goto_out(buff, newval, obj);
        return;
    }

    memset(newval as *mut c_void, 0, sz);
    /* wrong size, should fail */
    err = bpf_map__set_initial_value(map, newval as *const c_void, sz - 1);
    if CHECK(
        err == 0,
        c"reject set initial value wrong size".as_ptr(),
        c"err %d\n".as_ptr(),
        err,
    ) {
        goto_out(buff, newval, obj);
        return;
    }

    err = bpf_map__set_initial_value(map, newval as *const c_void, sz);
    if CHECK(err != 0, c"set initial value".as_ptr(), c"err %d\n".as_ptr(), err) {
        goto_out(buff, newval, obj);
        return;
    }

    err = bpf_object__load(obj);
    if CHECK_FAIL(err != 0) {
        goto_out(buff, newval, obj);
        return;
    }

    map_fd = bpf_map__fd(map);
    if CHECK_FAIL(map_fd < 0) {
        goto_out(buff, newval, obj);
        return;
    }

    buff = malloc(sz) as *mut __u8;
    if !buff.is_null() {
        err = bpf_map_lookup_elem(map_fd, &zero as *const _ as *const c_void, buff as *mut c_void);
    }
    if CHECK(
        buff.is_null() || err != 0 || memcmp(buff as *const c_void, newval as *const c_void, sz) != 0,
        c"compare .rodata map data override".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno,
    ) {
        goto_out(buff, newval, obj);
        return;
    }

    memset(newval as *mut c_void, 1, sz);
    /* object loaded - should fail */
    err = bpf_map__set_initial_value(map, newval as *const c_void, sz);
    CHECK(
        err == 0,
        c"reject set initial value after load".as_ptr(),
        c"err %d\n".as_ptr(),
        err,
    );

    goto_out(buff, newval, obj);
}

unsafe fn goto_out(buff: *mut __u8, newval: *mut __u8, obj: *mut bpf_object) {
    free(buff as *mut c_void);
    free(newval as *mut c_void);
    bpf_object__close(obj);
}

unsafe fn test_percpu_data_on_cpus(
    map: *mut bpf_map,
    map_fd: c_int,
    prog_fd: c_int,
    runp: *mut c_int,
) {
    let mut data: *mut test_global_percpu_data__percpu = core::ptr::null_mut();
    let mut err: c_int;
    let mut key: c_int = 0;
    let mut num_online: c_int = 0;
    let mut run: c_int = 0;
    let mut args: [__u64; 2] = [0x1234u64, 0x5678u64];
    let data_sz: size_t;
    let mut online: *mut bool = core::ptr::null_mut();
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        ctx_in: args.as_mut_ptr() as *mut c_void,
        ctx_size_in: core::mem::size_of_val(&args) as __u32,
        flags: BPF_F_TEST_RUN_ON_CPU,
        cpu: 0,
        retval: 0,
    };

    err = parse_cpu_mask_file(
        c"/sys/devices/system/cpu/online".as_ptr(),
        &mut online,
        &mut num_online,
    );
    if !ASSERT_OK(err, c"parse_cpu_mask_file".as_ptr()) {
        return;
    }

    data_sz = if !map.is_null() {
        bpf_map__value_size(map)
    } else {
        core::mem::size_of::<test_global_percpu_data__percpu>()
    };
    data = calloc(1, data_sz) as *mut test_global_percpu_data__percpu;
    if !ASSERT_OK_PTR(data as *const c_void, c"calloc percpu data".as_ptr()) {
        free(online as *mut c_void);
        return;
    }

    /* run on every online-CPU */
    for i in 0..num_online {
        let flags: __u64;

        if !*online.add(i as usize) {
            continue;
        }

        topts.cpu = i;
        topts.retval = -1i32 as __u32;
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
        ASSERT_EQ(topts.retval, 0u32, c"bpf_prog_test_run_opts retval".as_ptr());

        memset(data as *mut c_void, 0, data_sz);
        flags = ((i as __u64) << 32) | BPF_F_CPU;
        if !map.is_null() {
            err = bpf_map__lookup_elem(
                map,
                &key as *const _ as *const c_void,
                core::mem::size_of_val(&key),
                data as *mut c_void,
                data_sz,
                flags,
            );
        } else {
            err = bpf_map_lookup_elem_flags(map_fd, &key as *const _ as *const c_void, data as *mut c_void, flags);
        }
        if !ASSERT_OK(err, c"lookup_elem on cpu".as_ptr()) {
            break;
        }

        run += 1;
        ASSERT_EQ(*runp, run, c"run".as_ptr());
        ASSERT_EQ((*data).cpu_id[0], i, c"cpu_id".as_ptr());
        ASSERT_EQ((*data).data, 1, c"data".as_ptr());
        ASSERT_TRUE((*data).set, c"set".as_ptr());
        ASSERT_EQ((*data).nums[6], 0xc0de, c"nums[6]".as_ptr());
        ASSERT_EQ((*data).struct_data.i, 1, c"struct_data.i".as_ptr());
        ASSERT_TRUE((*data).struct_data.set, c"struct_data.set".as_ptr());
        ASSERT_EQ((*data).struct_data.nums[6], 0xc0de, c"struct_data.nums[6]".as_ptr());
    }

    free(data as *mut c_void);
    free(online as *mut c_void);
}

unsafe fn test_global_percpu_data_init() {
    let mut init_value: test_global_percpu_data__percpu = Default::default();
    let mut init_data: *mut test_global_percpu_data__percpu;
    let desired_sz: __u32 = sysconf(_SC_PAGE_SIZE) as __u32;
    let mut skel: *mut test_global_percpu_data = core::ptr::null_mut();
    let mut init_data_sz: size_t = 0;
    let map: *mut bpf_map;
    let prog_fd: c_int;
    let mut err: c_int;

    skel = test_global_percpu_data__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_global_percpu_data__open".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }
    if !ASSERT_OK_PTR((*skel).percpu as *const c_void, c"skel->percpu".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }
    if !ASSERT_OK_PTR((*skel).data_percpu as *const c_void, c"skel->data_percpu".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }
    if !ASSERT_OK_PTR((*skel).percpu_data as *const c_void, c"skel->percpu_data".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }
    if !ASSERT_OK_PTR((*skel).percpu_looooooooong as *const c_void, c"skel->percpu_looooooooong".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }

    ASSERT_STREQ(bpf_map__name((*skel).maps.percpu_data), c".percpu.data".as_ptr(), c".percpu.data map name".as_ptr());
    ASSERT_STREQ(bpf_map__name((*skel).maps.data_percpu), c".data.percpu".as_ptr(), c".data.percpu map name".as_ptr());
    ASSERT_STREQ(bpf_map__name((*skel).maps.percpu_looooooooong), c".percpu.looooooooong".as_ptr(), c"long map name".as_ptr());
    ASSERT_STREQ(bpf_map__name((*skel).maps.percpu), c".percpu".as_ptr(), c"map name".as_ptr());
    ASSERT_EQ((*(*skel).percpu).data, -1, c"skel->percpu->data".as_ptr());
    ASSERT_FALSE((*(*skel).percpu).set, c"skel->percpu->set".as_ptr());
    ASSERT_EQ((*(*skel).percpu).nums[6], 0, c"skel->percpu->nums[6]".as_ptr());
    ASSERT_EQ((*(*skel).percpu).struct_data.i, -1, c"struct_data.i".as_ptr());
    ASSERT_FALSE((*(*skel).percpu).struct_data.set, c"struct_data.set".as_ptr());
    ASSERT_EQ((*(*skel).percpu).struct_data.nums[6], 0, c"struct_data.nums[6]".as_ptr());

    map = (*skel).maps.percpu;
    if !ASSERT_EQ(bpf_map__type(map), BPF_MAP_TYPE_PERCPU_ARRAY, c"bpf_map__type".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }

    init_value.data = 2;
    init_value.nums[6] = -1;
    init_value.struct_data.i = 2;
    init_value.struct_data.nums[6] = -1;
    err = bpf_map__set_initial_value(
        map,
        &init_value as *const _ as *const c_void,
        core::mem::size_of_val(&init_value),
    );
    if !ASSERT_OK(err, c"bpf_map__set_initial_value".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }

    init_data = bpf_map__initial_value(map, &mut init_data_sz) as *mut test_global_percpu_data__percpu;
    if !ASSERT_OK_PTR(init_data as *const c_void, c"bpf_map__initial_value".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }

    ASSERT_EQ((*init_data).data, init_value.data, c"init_value data".as_ptr());
    ASSERT_EQ((*init_data).set, init_value.set, c"init_value set".as_ptr());
    ASSERT_EQ((*init_data).struct_data.i, init_value.struct_data.i, c"init_value struct_data.i".as_ptr());
    ASSERT_EQ((*init_data).struct_data.nums[6], init_value.struct_data.nums[6], c"init_value struct_data.nums[6]".as_ptr());
    ASSERT_EQ(init_data_sz, core::mem::size_of_val(&init_value), c"init_value size".as_ptr());
    ASSERT_EQ(init_data as *mut c_void, (*skel).percpu as *mut c_void, c"skel->percpu eq init_data".as_ptr());
    ASSERT_EQ((*(*skel).percpu).data, init_value.data, c"skel->percpu->data".as_ptr());
    ASSERT_EQ((*(*skel).percpu).set, init_value.set, c"skel->percpu->set".as_ptr());
    ASSERT_EQ((*(*skel).percpu).struct_data.i, init_value.struct_data.i, c"skel->percpu->struct_data.i".as_ptr());
    ASSERT_EQ((*(*skel).percpu).struct_data.nums[6], init_value.struct_data.nums[6], c"skel->percpu->struct_data.nums[6]".as_ptr());

    ASSERT_GT(desired_sz as size_t, core::mem::size_of_val(&init_value), c"desired_sz".as_ptr());
    err = bpf_map__set_value_size(map, desired_sz as size_t);
    if !ASSERT_OK(err, c"bpf_map__set_value_size".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }
    if !ASSERT_EQ(bpf_map__value_size(map), desired_sz as size_t, c"percpu value size".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }
    if !ASSERT_NEQ(bpf_map__btf_value_type_id(map), 0, c"percpu BTF value type".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }

    init_data = bpf_map__initial_value(map, &mut init_data_sz) as *mut test_global_percpu_data__percpu;
    if !ASSERT_OK_PTR(init_data as *const c_void, c"resized bpf_map__initial_value".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }
    if !ASSERT_EQ(init_data_sz, desired_sz as size_t, c"resized initial value size".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }
    if !ASSERT_EQ((*init_data).data, init_value.data, c"resized initial value data".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }

    err = test_global_percpu_data__load(skel);
    if !ASSERT_OK(err, c"test_global_percpu_data__load".as_ptr()) {
        test_global_percpu_data__destroy(skel);
        return;
    }

    ASSERT_OK_PTR((*skel).percpu as *const c_void, c"skel->percpu".as_ptr());

    prog_fd = bpf_program__fd((*skel).progs.update_percpu_data);
    test_percpu_data_on_cpus(map, bpf_map__fd(map), prog_fd, &mut (*(*skel).bss).run);

    test_global_percpu_data__destroy(skel);
}

unsafe fn test_global_percpu_data_lskel() {
    let mut lskel: *mut test_global_percpu_data_lskel = core::ptr::null_mut();
    let prog_fd: c_int;
    let map_fd: c_int;

    lskel = test_global_percpu_data_lskel__open_and_load();
    if !ASSERT_OK_PTR(lskel as *const c_void, c"test_global_percpu_data_lskel__open_and_load".as_ptr()) {
        test_global_percpu_data_lskel__destroy(lskel);
        return;
    }

    map_fd = (*lskel).maps.percpu.map_fd;
    prog_fd = (*lskel).progs.update_percpu_data.prog_fd;
    test_percpu_data_on_cpus(core::ptr::null_mut(), map_fd, prog_fd, &mut (*(*lskel).bss).run);

    test_global_percpu_data_lskel__destroy(lskel);
}

unsafe fn create_rdonly_percpu_array() -> c_int {
    let map_opts = bpf_map_create_opts {
        sz: core::mem::size_of::<bpf_map_create_opts>(),
        map_flags: BPF_F_RDONLY_PROG,
    };
    let key: c_int = 0;
    let map_fd: c_int;
    let mut err: c_int;
    let value: __u64 = 0;

    map_fd = bpf_map_create(
        BPF_MAP_TYPE_PERCPU_ARRAY,
        c"percpu_ro_map".as_ptr(),
        core::mem::size_of::<c_int>() as __u32,
        core::mem::size_of::<__u64>() as __u32,
        1,
        &map_opts,
    );
    if !ASSERT_GE(map_fd, 0, c"bpf_map_create".as_ptr()) {
        return -1;
    }

    err = bpf_map_update_elem(
        map_fd,
        &key as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_F_ALL_CPUS,
    );
    if !ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
        close(map_fd);
        return -1;
    }

    err = bpf_map_freeze(map_fd);
    if !ASSERT_OK(err, c"bpf_map_freeze".as_ptr()) {
        close(map_fd);
        return -1;
    }

    map_fd
}

unsafe fn test_global_percpu_data_rdonly_direct_read() {
    /*
     * Raw instructions with manually prepared rdonly percpu_array map
     * for testing direct-read global percpu data, because libbpf
     * doesn't have rdonly internal percpu_array map support for
     * global percpu data.
     */
    let mut insns = [
        bpf_ld_map_value(BPF_REG_1, 0, 0),
        bpf_ldx_mem(BPF_DW, BPF_REG_0, BPF_REG_1, 0),
        bpf_exit_insn(),
    ];
    let map_fd: c_int;
    let prog_fd: c_int;

    map_fd = create_rdonly_percpu_array();
    if map_fd < 0 {
        return;
    }

    insns[0].imm = map_fd;
    prog_fd = bpf_prog_load(
        BPF_PROG_TYPE_SOCKET_FILTER,
        c"percpu_ro_prog".as_ptr(),
        c"GPL".as_ptr(),
        insns.as_ptr(),
        insns.len(),
        core::ptr::null(),
    );
    if ASSERT_GE(prog_fd, 0, c"bpf_prog_load".as_ptr()) {
        close(prog_fd);
    }
    close(map_fd);
}

unsafe fn test_global_percpu_data_rdonly_direct_write() {
    let mut prog_opts = bpf_prog_load_opts {
        sz: core::mem::size_of::<bpf_prog_load_opts>(),
        log_buf: core::ptr::null_mut(),
        log_size: 0,
        log_level: 0,
    };
    /* See the comment in test_global_percpu_data_rdonly_direct_read() */
    let mut insns = [
        bpf_ld_map_value(BPF_REG_1, 0, 0),
        bpf_ldx_mem(BPF_DW, BPF_REG_0, BPF_REG_1, 0),
        bpf_st_mem(BPF_DW, BPF_REG_1, 0, 0),
        bpf_exit_insn(),
    ];
    let mut log_buf = [0 as c_char; 256];
    let map_fd: c_int;
    let prog_fd: c_int;

    prog_opts.log_buf = log_buf.as_mut_ptr();
    prog_opts.log_size = core::mem::size_of_val(&log_buf) as __u32;
    prog_opts.log_level = 1;

    map_fd = create_rdonly_percpu_array();
    if map_fd < 0 {
        return;
    }

    insns[0].imm = map_fd;
    prog_fd = bpf_prog_load(
        BPF_PROG_TYPE_SOCKET_FILTER,
        c"percpu_ro_prog".as_ptr(),
        c"GPL".as_ptr(),
        insns.as_ptr(),
        insns.len(),
        &prog_opts,
    );
    if !ASSERT_LT(prog_fd, 0, c"bpf_prog_load".as_ptr()) {
        close(prog_fd);
    } else {
        ASSERT_HAS_SUBSTR(log_buf.as_ptr(), c"write into map forbidden".as_ptr(), c"verifier log".as_ptr());
    }
    close(map_fd);
}

unsafe fn test_global_percpu_data_verifier_log() {
    RUN_TESTS(c"test_global_percpu_data".as_ptr());
}

unsafe fn test_global_percpu_data_iter() {
    let mut opts = bpf_iter_attach_opts {
        sz: core::mem::size_of::<bpf_iter_attach_opts>(),
        link_info: core::ptr::null_mut(),
        link_info_len: 0,
    };
    let skel: *mut test_global_percpu_data;
    let mut linfo: bpf_iter_link_info = core::mem::zeroed();
    let mut link: *mut bpf_link = core::ptr::null_mut();
    let fd: c_int;
    let num_cpus: c_int;
    let mut len: ssize_t;
    let mut err: c_int;
    let mut buf = [0 as c_char; 16];

    num_cpus = libbpf_num_possible_cpus();
    if !ASSERT_GT(num_cpus, 0, c"libbpf_num_possible_cpus".as_ptr()) {
        return;
    }

    skel = test_global_percpu_data__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_global_percpu_data__open".as_ptr()) {
        return;
    }

    (*(*skel).rodata).num_cpus = num_cpus;
    (*(*skel).rodata).num_off = core::mem::offset_of!(test_global_percpu_data__percpu, struct_data)
        + core::mem::offset_of!(test_global_percpu_data__struct_data, nums)
        + 6 * core::mem::size_of::<c_int>();
    (*(*skel).rodata).elem_sz = roundup(core::mem::size_of::<test_global_percpu_data__percpu>(), 8);
    (*(*skel).percpu).struct_data.nums[6] = 0xc0de;

    err = test_global_percpu_data__load(skel);
    if !ASSERT_OK(err, c"test_global_percpu_data__load".as_ptr()) {
        bpf_link__destroy(link);
        test_global_percpu_data__destroy(skel);
        return;
    }

    linfo.map.map_fd = bpf_map__fd((*skel).maps.percpu);
    opts.link_info = &mut linfo;
    opts.link_info_len = core::mem::size_of_val(&linfo) as __u32;
    link = bpf_program__attach_iter((*skel).progs.dump_percpu_data, &opts);
    if !ASSERT_OK_PTR(link as *const c_void, c"bpf_program__attach_iter".as_ptr()) {
        bpf_link__destroy(link);
        test_global_percpu_data__destroy(skel);
        return;
    }

    fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_GE(fd, 0, c"bpf_iter_create".as_ptr()) {
        bpf_link__destroy(link);
        test_global_percpu_data__destroy(skel);
        return;
    }

    loop {
        len = read(fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf));
        if len <= 0 {
            break;
        }
        loop {
            break;
        }
    }
    ASSERT_EQ(len, 0, c"read iter".as_ptr());
    ASSERT_TRUE((*(*skel).bss).run_iter, c"run_iter".as_ptr());
    ASSERT_EQ((*(*skel).bss).sum, 0xc0de * num_cpus, c"sum".as_ptr());

    close(fd);
    bpf_link__destroy(link);
    test_global_percpu_data__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_global_percpu_data() {
    if !feat_supported(core::ptr::null(), FEAT_PERCPU_DATA) {
        test__skip();
        return;
    }

    if test__start_subtest(c"init".as_ptr()) {
        test_global_percpu_data_init();
    }
    if test__start_subtest(c"lskel".as_ptr()) {
        test_global_percpu_data_lskel();
    }
    if test__start_subtest(c"rdonly_direct_read".as_ptr()) {
        test_global_percpu_data_rdonly_direct_read();
    }
    if test__start_subtest(c"rdonly_direct_write".as_ptr()) {
        test_global_percpu_data_rdonly_direct_write();
    }
    test_global_percpu_data_verifier_log();
    if test__start_subtest(c"iter".as_ptr()) {
        test_global_percpu_data_iter();
    }
}
