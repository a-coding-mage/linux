// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022-2024 Red Hat */
/* Rust translation of testing/selftests/hid/hid_bpf.c.
 * C include dependencies intentionally remain external:
 * "hid.skel.h", "hid_common.h", and <bpf/bpf.h>.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u8 = u8;
type size_t = usize;
type va_list = *mut c_void;

#[repr(C)]
pub struct uhid_device {
    pub hid_id: c_int,
    pub tid: pthread_t,
}

#[repr(C)]
pub struct hid_hw_request_syscall_args {
    pub data: [__u8; 10],
    pub hid: c_uint,
    pub retval: c_int,
    pub size: size_t,
    pub type_: hid_report_type,
    pub request_type: __u8,
}

#[repr(C)]
pub struct hid_bpf {
    pub hid: uhid_device,
    pub hidraw_fd: c_int,
    pub skel: *mut hid,
    pub hid_links: [*mut bpf_link; 3], /* max number of programs loaded in a single test */
}

#[repr(C)]
pub struct test_program {
    pub name: *const c_char,
    pub insert_head: c_int,
}

#[repr(C)]
pub struct hidraw_report_descriptor {
    pub size: c_uint,
    pub value: [__u8; 4096],
}

pub enum __test_metadata {}
pub enum hid {}
pub enum hid_report_type {}
pub enum bpf_link {}
pub enum bpf_map {}
pub enum bpf_program {}
pub enum bpf_test_run_opts {}
pub enum timespec {}
pub enum pthread_mutex_t {}
pub enum pthread_cond_t {}
type pthread_t = usize;
type libbpf_print_level = c_uint;

const BUS_USB: c_uint = 0x03;
const EINVAL: c_int = 22;
const HID_FEATURE_REPORT: hid_report_type = unsafe { core::mem::transmute(0i32) };
const HID_REQ_GET_REPORT: __u8 = 0x01;
const BPF_MAP_TYPE_STRUCT_OPS: c_uint = 26;
const CLOCK_REALTIME: c_int = 0;
const LIBBPF_DEBUG: libbpf_print_level = 1;
const LIBBPF_STRICT_ALL: c_uint = 0xffffffff;
const HIDIOCGRDESCSIZE: c_uint = 0x80044801;
const HIDIOCGRDESC: c_uint = 0x90044802;

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        ($array).len()
    };
}

/* External selftest/libbpf/libc declarations supplied by translated dependencies. */
unsafe extern "C" {
    static rdesc: [__u8; 0];
    static mut errno: c_int;
    static mut stdout: *mut c_void;
    static mut output_report: [__u8; 10];
    static mut uhid_output_mtx: pthread_mutex_t;
    static mut uhid_output_cond: pthread_cond_t;

    fn close(fd: c_int) -> c_int;
    fn remove(path: *const c_char) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    fn ioctl(fd: c_int, request: c_uint, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn usleep(usec: c_uint) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut c_void, format: *const c_char, ap: va_list) -> c_int;

    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_timedwait(
        cond: *mut pthread_cond_t,
        mutex: *mut pthread_mutex_t,
        abstime: *const timespec,
    ) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;

    fn setup_uhid(
        metadata: *mut __test_metadata,
        hid: *mut uhid_device,
        bus: c_uint,
        vendor: c_uint,
        product: c_uint,
        rdesc: *const __u8,
        rdesc_size: size_t,
    ) -> c_int;
    fn uhid_destroy(metadata: *mut __test_metadata, hid: *mut uhid_device);
    fn uhid_send_event(
        metadata: *mut __test_metadata,
        hid: *mut uhid_device,
        buf: *mut __u8,
        size: size_t,
    );
    fn open_hidraw(hid: *mut uhid_device) -> c_int;

    fn hid__open() -> *mut hid;
    fn hid__detach(skel: *mut hid);
    fn hid__destroy(skel: *mut hid);
    fn hid__load(skel: *mut hid) -> c_int;
    fn hid__attach(skel: *mut hid) -> c_int;

    fn bpf_map__type(map: *mut bpf_map) -> c_uint;
    fn bpf_map__set_autocreate(map: *mut bpf_map, autocreate: bool) -> c_int;
    fn bpf_map__set_autoattach(map: *mut bpf_map, autoattach: bool);
    fn bpf_map__name(map: *mut bpf_map) -> *const c_char;
    fn bpf_map__initial_value(map: *mut bpf_map, size: *mut size_t) -> *mut c_int;
    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_obj_pin(fd: c_int, pathname: *const c_char) -> c_int;

    fn bpf_object__find_program_by_name(obj: *mut c_void, name: *const c_char) -> *mut bpf_program;
    fn bpf_object__find_map_by_name(obj: *mut c_void, name: *const c_char) -> *mut bpf_map;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn libbpf_set_strict_mode(mode: c_uint) -> c_int;
    fn libbpf_set_print(
        print_fn: unsafe extern "C" fn(libbpf_print_level, *const c_char, va_list) -> c_int,
    );
    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

/* Harness assertion/logging macros are supplied by the selftest framework. */

unsafe fn detach_bpf(self_: *mut hid_bpf) {
    let mut i: c_int;

    if (*self_).hidraw_fd != 0 {
        close((*self_).hidraw_fd);
    }
    (*self_).hidraw_fd = 0;

    if (*self_).skel.is_null() {
        return;
    }

    hid__detach((*self_).skel);

    i = 0;
    while (i as usize) < ARRAY_SIZE!((*self_).hid_links) {
        if !(*self_).hid_links[i as usize].is_null() {
            bpf_link__destroy((*self_).hid_links[i as usize]);
        }
        i += 1;
    }

    hid__destroy((*self_).skel);
    (*self_).skel = core::ptr::null_mut();
}

unsafe fn hid_bpf_teardown(
    _metadata: *mut __test_metadata,
    self_: *mut hid_bpf,
    variant: *const c_void,
) {
    let mut uhid_err: *mut c_void = core::ptr::null_mut();

    uhid_destroy(_metadata, &mut (*self_).hid);

    detach_bpf(self_);
    pthread_join((*self_).hid.tid, &mut uhid_err);
}

macro_rules! TEARDOWN_LOG {
    ($metadata:expr, $self_:expr, $variant:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        TH_LOG!($fmt $(, $arg)*);
        hid_bpf_teardown($metadata, $self_, $variant);
    }};
}

unsafe fn hid_bpf_setup(_metadata: *mut __test_metadata, self_: *mut hid_bpf) {
    let err: c_int;

    err = setup_uhid(
        _metadata,
        &mut (*self_).hid,
        BUS_USB,
        0x0001,
        0x0a36,
        rdesc.as_ptr(),
        core::mem::size_of_val(&rdesc),
    );
    ASSERT_OK!(err);
}

macro_rules! LOAD_PROGRAMS {
    ($progs:expr, $metadata:expr, $self_:expr, $variant:expr) => {
        load_programs(($progs).as_ptr(), ARRAY_SIZE!($progs), $metadata, $self_, $variant)
    };
}

macro_rules! LOAD_BPF {
    ($metadata:expr, $self_:expr, $variant:expr) => {
        load_programs(core::ptr::null(), 0, $metadata, $self_, $variant)
    };
}

unsafe fn load_programs(
    programs: *const test_program,
    progs_count: size_t,
    _metadata: *mut __test_metadata,
    self_: *mut hid_bpf,
    variant: *const c_void,
) {
    let mut iter_map: *mut bpf_map;
    let mut err: c_int = -EINVAL;

    ASSERT_LE!(progs_count, ARRAY_SIZE!((*self_).hid_links));
    TH_LOG!("too many programs are to be loaded");

    /* open the bpf file */
    (*self_).skel = hid__open();
    ASSERT_OK_PTR!((*self_).skel);
    TEARDOWN_LOG!(_metadata, self_, variant, "Error while calling hid__open");

    /*
     * Disable all struct_ops maps by default so libbpf does not autoload
     * programs referenced by maps that are unrelated to the current test.
     */
    /* bpf_object__for_each_map(iter_map, *self->skel->skeleton->obj) */
    while bpf_object_for_each_map_next(&mut iter_map, (*self_).skel) {
        if bpf_map__type(iter_map) == BPF_MAP_TYPE_STRUCT_OPS {
            err = bpf_map__set_autocreate(iter_map, false);
            ASSERT_OK!(err);
            TH_LOG!("can not disable struct_ops map '%s'", bpf_map__name(iter_map));
        }

        bpf_map__set_autoattach(iter_map, false);
    }

    let mut i: c_int = 0;
    while (i as size_t) < progs_count {
        let mut prog: *mut bpf_program;
        let mut map: *mut bpf_map;
        let mut ops_hid_id: *mut c_int;
        let program = &*programs.add(i as usize);

        prog = bpf_object__find_program_by_name(hid_skel_obj((*self_).skel), program.name);
        ASSERT_OK_PTR!(prog);
        TH_LOG!("can not find program by name '%s'", program.name);

        bpf_program__set_autoload(prog, true);

        map = bpf_object__find_map_by_name(hid_skel_obj((*self_).skel), program.name.add(4));
        ASSERT_OK_PTR!(map);
        TH_LOG!("can not find struct_ops by name '%s'", program.name.add(4));

        err = bpf_map__set_autocreate(map, true);
        ASSERT_OK!(err);
        TH_LOG!("can not enable struct_ops map '%s'", program.name.add(4));

        /* hid_id is the first field of struct hid_bpf_ops */
        ops_hid_id = bpf_map__initial_value(map, core::ptr::null_mut());
        ASSERT_OK_PTR!(ops_hid_id);
        TH_LOG!("unable to retrieve struct_ops data");

        *ops_hid_id = (*self_).hid.hid_id;
        i += 1;
    }

    err = hid__load((*self_).skel);
    ASSERT_OK!(err);
    TH_LOG!("hid_skel_load failed: %d", err);

    i = 0;
    while (i as size_t) < progs_count {
        let mut map: *mut bpf_map;
        let program = &*programs.add(i as usize);

        map = bpf_object__find_map_by_name(hid_skel_obj((*self_).skel), program.name.add(4));
        ASSERT_OK_PTR!(map);
        TH_LOG!("can not find struct_ops by name '%s'", program.name.add(4));

        (*self_).hid_links[i as usize] = bpf_map__attach_struct_ops(map);
        ASSERT_OK_PTR!((*self_).hid_links[i as usize]);
        TH_LOG!("failed to attach struct ops '%s'", program.name.add(4));
        i += 1;
    }

    hid__attach((*self_).skel);

    (*self_).hidraw_fd = open_hidraw(&mut (*self_).hid);
    ASSERT_GE!((*self_).hidraw_fd, 0);
    TH_LOG!("open_hidraw");
}

/* Placeholders for C skeleton field/macro access supplied by generated bindings. */
unsafe fn hid_skel_obj(_skel: *mut hid) -> *mut c_void {
    TODO!("external generated hid skeleton object access")
}

unsafe fn bpf_object_for_each_map_next(_iter_map: *mut *mut bpf_map, _skel: *mut hid) -> bool {
    TODO!("external bpf_object__for_each_map iteration")
}

/*
 * A simple test to see if the fixture is working fine.
 * If this fails, none of the other tests will pass.
 */
unsafe fn test_create_uhid(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {}

/*
 * Attach hid_first_event to the given uhid device,
 * retrieve and open the matching hidraw node,
 * inject one event in the uhid device,
 * check that the program sees it and can change the data
 */
unsafe fn raw_event(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_first_event".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* check that the program is correctly loaded */
    ASSERT_EQ!(hid_data_callback_check((*self_).skel), 52);
    TH_LOG!("callback_check1");
    ASSERT_EQ!(hid_data_callback2_check((*self_).skel), 52);
    TH_LOG!("callback2_check1");

    /* inject one event */
    buf[0] = 1;
    buf[1] = 42;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* check that hid_first_event() was executed */
    ASSERT_EQ!(hid_data_callback_check((*self_).skel), 42);
    TH_LOG!("callback_check1");

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[2], 47);

    /* inject another event */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    buf[0] = 1;
    buf[1] = 47;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* check that hid_first_event() was executed */
    ASSERT_EQ!(hid_data_callback_check((*self_).skel), 47);
    TH_LOG!("callback_check1");

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[2], 52);
}

/*
 * Attach hid_first_event to the given uhid device,
 * retrieve and open the matching hidraw node,
 * inject one event in the uhid device,
 * check that the program sees it and can change the data
 */
unsafe fn subprog_raw_event(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_subprog_first_event".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* inject one event */
    buf[0] = 1;
    buf[1] = 42;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[2], 47);

    /* inject another event */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    buf[0] = 1;
    buf[1] = 47;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[2], 52);
}

/* Additional external generated skeleton field accessors. */
unsafe fn hid_data_callback_check(_skel: *mut hid) -> c_int {
    TODO!("external generated hid skeleton data access")
}
unsafe fn hid_data_callback2_check(_skel: *mut hid) -> c_int {
    TODO!("external generated hid skeleton data access")
}
unsafe fn hid_bss_get_data_overflow_check(_skel: *mut hid) -> c_int {
    TODO!("external generated hid skeleton bss access")
}
unsafe fn hid_prog_fd(_skel: *mut hid, _name: *const c_char) -> c_int {
    TODO!("external generated hid skeleton program fd access")
}
unsafe fn hid_map_first_event(_skel: *mut hid) -> *mut bpf_map {
    TODO!("external generated hid skeleton map access")
}

/*
 * Attach hid_first_event to the given uhid device,
 * attempt at re-attaching it, we should not lock and
 * return an invalid struct bpf_link
 */
unsafe fn multiple_attach(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_first_event".as_ptr(), insert_head: 0 }];
    let link: *mut bpf_link;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    link = bpf_map__attach_struct_ops(hid_map_first_event((*self_).skel));
    ASSERT_NULL!(link);
    TH_LOG!("unexpected return value when re-attaching the struct_ops");
}

/*
 * Ensures that we can attach/detach programs
 */
unsafe fn test_attach_detach(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [
        test_program { name: c"hid_first_event".as_ptr(), insert_head: 0 },
        test_program { name: c"hid_second_event".as_ptr(), insert_head: 0 },
    ];
    let mut link: *mut bpf_link;
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;
    let mut link_fd: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    link = (*self_).hid_links[0];
    ASSERT_OK_PTR!(link);
    TH_LOG!("HID-BPF link not created");

    link_fd = bpf_link__fd(link);
    ASSERT_GE!(link_fd, 0);
    TH_LOG!("HID-BPF link FD not valid");

    /* inject one event */
    buf[0] = 1;
    buf[1] = 42;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[2], 47);

    /* make sure both programs are run */
    ASSERT_EQ!(buf[3], 52);

    /* pin the first program and immediately unpin it */
    const PIN_PATH: *const c_char = c"/sys/fs/bpf/hid_first_event".as_ptr();
    err = bpf_obj_pin(link_fd, PIN_PATH);
    ASSERT_OK!(err);
    TH_LOG!("error while calling bpf_obj_pin");
    remove(PIN_PATH);
    usleep(100000);

    /* detach the program */
    detach_bpf(self_);

    (*self_).hidraw_fd = open_hidraw(&mut (*self_).hid);
    ASSERT_GE!((*self_).hidraw_fd, 0);
    TH_LOG!("open_hidraw");

    /* inject another event */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    buf[0] = 1;
    buf[1] = 47;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw_no_bpf");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[1], 47);
    ASSERT_EQ!(buf[2], 0);
    ASSERT_EQ!(buf[3], 0);

    /* re-attach our program */
    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* inject one event */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    buf[0] = 1;
    buf[1] = 42;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[2], 47);
    ASSERT_EQ!(buf[3], 52);
}

/*
 * Attach hid_change_report_id to the given uhid device,
 * retrieve and open the matching hidraw node,
 * inject one event in the uhid device,
 * check that the program sees it and can change the data
 */
unsafe fn test_hid_change_report(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_change_report_id".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* inject one event */
    buf[0] = 1;
    buf[1] = 42;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 9);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 2);
    ASSERT_EQ!(buf[1], 42);
    ASSERT_EQ!(buf[2], 0);
    TH_LOG!("leftovers_from_previous_test");
}

macro_rules! DECLARE_LIBBPF_OPTS_BPF_TEST_RUN {
    ($name:ident, $args:expr) => {
        let mut $name: bpf_test_run_opts = core::mem::zeroed();
        let _ = &$args;
    };
}

/*
 * Call hid_bpf_input_report against the given uhid device,
 * check that the program is called and does the expected.
 */
unsafe fn test_hid_user_input_report_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let mut args = hid_hw_request_syscall_args {
        data: [0; 10],
        hid: 0,
        retval: -1,
        size: 10,
        type_: core::mem::zeroed(),
        request_type: 0,
    };
    DECLARE_LIBBPF_OPTS_BPF_TEST_RUN!(tattrs, args);
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;
    let mut prog_fd: c_int;

    LOAD_BPF!(_metadata, self_, variant);

    args.hid = (*self_).hid.hid_id as c_uint;
    args.data[0] = 1; /* report ID */
    args.data[1] = 2; /* report ID */
    args.data[2] = 42; /* report ID */

    prog_fd = hid_prog_fd((*self_).skel, c"hid_user_input_report".as_ptr());

    /* check that there is no data to read from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, -1);
    TH_LOG!("read_hidraw");

    err = bpf_prog_test_run_opts(prog_fd, &mut tattrs);

    ASSERT_OK!(err);
    TH_LOG!("error while calling bpf_prog_test_run_opts");

    ASSERT_EQ!(args.retval, 0);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[1], 2);
    ASSERT_EQ!(buf[2], 42);
}

/*
 * Call hid_bpf_hw_output_report against the given uhid device,
 * check that the program is called and does the expected.
 */
unsafe fn test_hid_user_output_report_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let mut args = hid_hw_request_syscall_args {
        data: [0; 10],
        hid: 0,
        retval: -1,
        size: 10,
        type_: core::mem::zeroed(),
        request_type: 0,
    };
    DECLARE_LIBBPF_OPTS_BPF_TEST_RUN!(tattrs, args);
    let mut err: c_int;
    let mut cond_err: c_int;
    let mut prog_fd: c_int;
    let mut time_to_wait: timespec = core::mem::zeroed();

    LOAD_BPF!(_metadata, self_, variant);

    args.hid = (*self_).hid.hid_id as c_uint;
    args.data[0] = 1; /* report ID */
    args.data[1] = 2; /* report ID */
    args.data[2] = 42; /* report ID */

    prog_fd = hid_prog_fd((*self_).skel, c"hid_user_output_report".as_ptr());

    pthread_mutex_lock(&mut uhid_output_mtx);

    memset(output_report.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&output_report));
    clock_gettime(CLOCK_REALTIME, &mut time_to_wait);
    timespec_add_tv_sec(&mut time_to_wait, 2);

    err = bpf_prog_test_run_opts(prog_fd, &mut tattrs);
    cond_err = pthread_cond_timedwait(&mut uhid_output_cond, &mut uhid_output_mtx, &time_to_wait);

    ASSERT_OK!(err);
    TH_LOG!("error while calling bpf_prog_test_run_opts");
    ASSERT_OK!(cond_err);
    TH_LOG!("error while calling waiting for the condition");

    ASSERT_EQ!(args.retval, 3);

    ASSERT_EQ!(output_report[0], 1);
    ASSERT_EQ!(output_report[1], 2);
    ASSERT_EQ!(output_report[2], 42);

    pthread_mutex_unlock(&mut uhid_output_mtx);
}

unsafe fn timespec_add_tv_sec(_ts: *mut timespec, _delta: i64) {
    TODO!("external timespec tv_sec field access")
}

/*
 * Call hid_hw_raw_request against the given uhid device,
 * check that the program is called and does the expected.
 */
unsafe fn test_hid_user_raw_request_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let mut args = hid_hw_request_syscall_args {
        data: [0; 10],
        hid: 0,
        retval: -1,
        size: 10,
        type_: HID_FEATURE_REPORT,
        request_type: HID_REQ_GET_REPORT,
    };
    DECLARE_LIBBPF_OPTS_BPF_TEST_RUN!(tattrs, args);
    let mut err: c_int;
    let mut prog_fd: c_int;

    LOAD_BPF!(_metadata, self_, variant);

    args.hid = (*self_).hid.hid_id as c_uint;
    args.data[0] = 1; /* report ID */

    prog_fd = hid_prog_fd((*self_).skel, c"hid_user_raw_request".as_ptr());

    err = bpf_prog_test_run_opts(prog_fd, &mut tattrs);
    ASSERT_OK!(err);
    TH_LOG!("error while calling bpf_prog_test_run_opts");

    ASSERT_EQ!(args.retval, 2);

    ASSERT_EQ!(args.data[1], 2);
}

fn HIDIOCGFEATURE(len: size_t) -> c_uint {
    0x80000000u32 | ((len as c_uint) << 16) | 0x4807
}

/*
 * Call hid_hw_raw_request against the given uhid device,
 * check that the program is called and prevents the
 * call to uhid.
 */
unsafe fn test_hid_filter_raw_request_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_test_filter_raw_request".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* first check that we did not attach to device_event */

    /* inject one event */
    buf[0] = 1;
    buf[1] = 42;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[1], 42);
    ASSERT_EQ!(buf[2], 0);
    TH_LOG!("leftovers_from_previous_test");

    /* now check that our program is preventing hid_hw_raw_request() */

    /* emit hid_hw_raw_request from hidraw */
    /* Get Feature */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    buf[0] = 0x1; /* Report Number */
    err = ioctl((*self_).hidraw_fd, HIDIOCGFEATURE(core::mem::size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
    ASSERT_LT!(err, 0);
    TH_LOG!("unexpected success while reading HIDIOCGFEATURE: %d", err);
    ASSERT_EQ!(errno, 20);
    TH_LOG!("unexpected error code while reading HIDIOCGFEATURE: %d", errno);

    /* remove our bpf program and check that we can now emit commands */

    /* detach the program */
    detach_bpf(self_);

    (*self_).hidraw_fd = open_hidraw(&mut (*self_).hid);
    ASSERT_GE!((*self_).hidraw_fd, 0);
    TH_LOG!("open_hidraw");

    err = ioctl((*self_).hidraw_fd, HIDIOCGFEATURE(core::mem::size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
    ASSERT_GE!(err, 0);
    TH_LOG!("error while reading HIDIOCGFEATURE: %d", err);
}

/*
 * Call hid_hw_raw_request against the given uhid device,
 * check that the program is called and can issue the call
 * to uhid and transform the answer.
 */
unsafe fn test_hid_change_raw_request_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_test_hidraw_raw_request".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* emit hid_hw_raw_request from hidraw */
    /* Get Feature */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    buf[0] = 0x1; /* Report Number */
    err = ioctl((*self_).hidraw_fd, HIDIOCGFEATURE(core::mem::size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
    ASSERT_EQ!(err, 3);
    TH_LOG!("unexpected returned size while reading HIDIOCGFEATURE: %d", err);

    ASSERT_EQ!(buf[0], 2);
    ASSERT_EQ!(buf[1], 3);
    ASSERT_EQ!(buf[2], 4);
}

/*
 * Call hid_hw_raw_request against the given uhid device,
 * check that the program is not making infinite loops.
 */
unsafe fn test_hid_infinite_loop_raw_request_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_test_infinite_loop_raw_request".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* emit hid_hw_raw_request from hidraw */
    /* Get Feature */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    buf[0] = 0x1; /* Report Number */
    err = ioctl((*self_).hidraw_fd, HIDIOCGFEATURE(core::mem::size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
    ASSERT_EQ!(err, 3);
    TH_LOG!("unexpected returned size while reading HIDIOCGFEATURE: %d", err);
}

/*
 * Call hid_hw_output_report against the given uhid device,
 * check that the program is called and prevents the
 * call to uhid.
 */
unsafe fn test_hid_filter_output_report_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_test_filter_output_report".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* first check that we did not attach to device_event */

    /* inject one event */
    buf[0] = 1;
    buf[1] = 42;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[1], 42);
    ASSERT_EQ!(buf[2], 0);
    TH_LOG!("leftovers_from_previous_test");

    /* now check that our program is preventing hid_hw_output_report() */

    buf[0] = 1; /* report ID */
    buf[1] = 2;
    buf[2] = 42;

    err = write((*self_).hidraw_fd, buf.as_ptr() as *const c_void, 3) as c_int;
    ASSERT_LT!(err, 0);
    TH_LOG!("unexpected success while sending hid_hw_output_report: %d", err);
    ASSERT_EQ!(errno, 25);
    TH_LOG!("unexpected error code while sending hid_hw_output_report: %d", errno);

    /* remove our bpf program and check that we can now emit commands */

    /* detach the program */
    detach_bpf(self_);

    (*self_).hidraw_fd = open_hidraw(&mut (*self_).hid);
    ASSERT_GE!((*self_).hidraw_fd, 0);
    TH_LOG!("open_hidraw");

    err = write((*self_).hidraw_fd, buf.as_ptr() as *const c_void, 3) as c_int;
    ASSERT_GE!(err, 0);
    TH_LOG!("error while sending hid_hw_output_report: %d", err);
}

/*
 * Call hid_hw_output_report against the given uhid device,
 * check that the program is called and can issue the call
 * to uhid and transform the answer.
 */
unsafe fn test_hid_change_output_report_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_test_hidraw_output_report".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* emit hid_hw_output_report from hidraw */
    buf[0] = 1; /* report ID */
    buf[1] = 2;
    buf[2] = 42;

    err = write((*self_).hidraw_fd, buf.as_ptr() as *const c_void, 10) as c_int;
    ASSERT_EQ!(err, 2);
    TH_LOG!("unexpected returned size while sending hid_hw_output_report: %d", err);
}

/*
 * Call hid_hw_output_report against the given uhid device,
 * check that the program is not making infinite loops.
 */
unsafe fn test_hid_infinite_loop_output_report_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_test_infinite_loop_output_report".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* emit hid_hw_output_report from hidraw */
    buf[0] = 1; /* report ID */
    buf[1] = 2;
    buf[2] = 42;

    err = write((*self_).hidraw_fd, buf.as_ptr() as *const c_void, 8) as c_int;
    ASSERT_EQ!(err, 2);
    TH_LOG!("unexpected returned size while sending hid_hw_output_report: %d", err);
}

/*
 * Attach hid_multiply_event_wq to the given uhid device,
 * retrieve and open the matching hidraw node,
 * inject one event in the uhid device,
 * check that the program sees it and can add extra data
 */
unsafe fn test_multiply_events_wq(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_test_multiply_events_wq".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* inject one event */
    buf[0] = 1;
    buf[1] = 42;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[1], 47);

    usleep(100000);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 9);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 2);
    ASSERT_EQ!(buf[1], 3);
}

/*
 * Attach hid_multiply_event to the given uhid device,
 * retrieve and open the matching hidraw node,
 * inject one event in the uhid device,
 * check that the program sees it and can add extra data
 */
unsafe fn test_multiply_events(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_test_multiply_events".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* inject one event */
    buf[0] = 1;
    buf[1] = 42;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 9);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 2);
    ASSERT_EQ!(buf[1], 47);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 9);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 2);
    ASSERT_EQ!(buf[1], 52);
}

/*
 * Call hid_bpf_input_report against the given uhid device,
 * check that the program is not making infinite loops.
 */
unsafe fn test_hid_infinite_loop_input_report_call(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_test_infinite_loop_input_report".as_ptr(), insert_head: 0 }];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* emit hid_hw_output_report from hidraw */
    buf[0] = 1; /* report ID */
    buf[1] = 2;
    buf[2] = 42;

    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[1], 3);

    /* read the data from hidraw: hid_bpf_try_input_report should work exactly one time */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[0], 1);
    ASSERT_EQ!(buf[1], 4);

    /* read the data from hidraw: there should be none */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, -1);
    TH_LOG!("read_hidraw");
}

/*
 * Attach hid_insert{0,1,2} to the given uhid device,
 * retrieve and open the matching hidraw node,
 * inject one event in the uhid device,
 * check that the programs have been inserted in the correct order.
 */
unsafe fn test_hid_attach_flags(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [
        test_program { name: c"hid_test_insert2".as_ptr(), insert_head: 0 },
        test_program { name: c"hid_test_insert1".as_ptr(), insert_head: 1 },
        test_program { name: c"hid_test_insert3".as_ptr(), insert_head: 0 },
    ];
    let mut buf: [__u8; 10] = [0; 10];
    let mut err: c_int;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* inject one event */
    buf[0] = 1;
    uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

    /* read the data from hidraw */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) as c_int;
    ASSERT_EQ!(err, 6);
    TH_LOG!("read_hidraw");
    ASSERT_EQ!(buf[1], 1);
    ASSERT_EQ!(buf[2], 2);
    ASSERT_EQ!(buf[3], 3);
}

/*
 * Attach hid_rdesc_fixup to the given uhid device,
 * retrieve and open the matching hidraw node,
 * check that the hidraw report descriptor has been updated.
 */
unsafe fn test_rdesc_fixup(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let mut rpt_desc: hidraw_report_descriptor = core::mem::zeroed();
    let progs = [test_program { name: c"hid_rdesc_fixup".as_ptr(), insert_head: 0 }];
    let mut err: c_int;
    let mut desc_size: c_int = 0;

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    /* check that hid_rdesc_fixup() was executed */
    ASSERT_EQ!(hid_data_callback2_check((*self_).skel), 0x21);

    /* read the exposed report descriptor from hidraw */
    err = ioctl((*self_).hidraw_fd, HIDIOCGRDESCSIZE, &mut desc_size) as c_int;
    ASSERT_GE!(err, 0);
    TH_LOG!("error while reading HIDIOCGRDESCSIZE: %d", err);

    /* ensure the new size of the rdesc is bigger than the old one */
    ASSERT_GT!(desc_size as size_t, core::mem::size_of_val(&rdesc));

    rpt_desc.size = desc_size as c_uint;
    err = ioctl((*self_).hidraw_fd, HIDIOCGRDESC, &mut rpt_desc) as c_int;
    ASSERT_GE!(err, 0);
    TH_LOG!("error while reading HIDIOCGRDESC: %d", err);

    ASSERT_EQ!(rpt_desc.value[4], 0x42);
}

unsafe fn test_rdesc_fixup_get_data_overflow(_metadata: *mut __test_metadata, self_: *mut hid_bpf, variant: *const c_void) {
    let progs = [test_program { name: c"hid_rdesc_fixup_get_data_overflow".as_ptr(), insert_head: 0 }];

    LOAD_PROGRAMS!(progs, _metadata, self_, variant);

    ASSERT_EQ!(hid_bss_get_data_overflow_check((*self_).skel), 1);
}

unsafe extern "C" fn libbpf_print_fn(
    level: libbpf_print_level,
    format: *const c_char,
    args: va_list,
) -> c_int {
    let mut buf: [c_char; 1024] = [0; 1024];

    if level == LIBBPF_DEBUG {
        return 0;
    }

    snprintf(buf.as_mut_ptr(), core::mem::size_of_val(&buf), c"# %s".as_ptr(), format);

    vfprintf(stdout, buf.as_ptr(), args);
    return 0;
}

unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    /* Use libbpf 1.0 API mode */
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    libbpf_set_print(libbpf_print_fn);

    return test_harness_run(argc, argv);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
