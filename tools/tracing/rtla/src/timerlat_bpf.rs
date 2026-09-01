// SPDX-License-Identifier: GPL-2.0
// C source was compiled only when HAVE_BPF_SKEL was defined.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct timerlat_params {
    pub common: timerlat_common_params,
}

#[repr(C)]
pub struct timerlat_common_params {
    pub output_divisor: c_ulonglong,
    pub hist: timerlat_hist_params,
    pub stop_us: c_ulonglong,
    pub stop_total_us: c_ulonglong,
    pub aa_only: bool,
}

#[repr(C)]
pub struct timerlat_hist_params {
    pub entries: c_uint,
    pub bucket_size: c_ulonglong,
}

#[repr(C)]
pub struct timerlat_bpf {
    pub rodata: *mut timerlat_bpf_rodata,
    pub maps: timerlat_bpf_maps,
}

#[repr(C)]
pub struct timerlat_bpf_rodata {
    pub output_divisor: c_ulonglong,
    pub entries: c_uint,
    pub irq_threshold: c_ulonglong,
    pub thread_threshold: c_ulonglong,
    pub aa_only: bool,
    pub bucket_size: c_ulonglong,
}

#[repr(C)]
pub struct timerlat_bpf_maps {
    pub hist_irq: *mut bpf_map,
    pub hist_thread: *mut bpf_map,
    pub hist_user: *mut bpf_map,
    pub summary_irq: *mut bpf_map,
    pub summary_thread: *mut bpf_map,
    pub summary_user: *mut bpf_map,
    pub bpf_action: *mut bpf_map,
    pub signal_stop_tracing: *mut bpf_map,
    pub stop_tracing: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_object {
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
pub struct ring_buffer {
    _private: [u8; 0],
}

pub type size_t = usize;
pub type summary_field = c_uint;

const BPF_ANY: c_ulonglong = 0;

static mut bpf: *mut timerlat_bpf = ptr::null_mut();

/* BPF object and program for action program */
static mut obj: *mut bpf_object = ptr::null_mut();
static mut prog: *mut bpf_program = ptr::null_mut();

extern "C" {
    static nr_cpus: c_int;

    fn debug_msg(fmt: *const c_char, ...);
    fn err_msg(fmt: *const c_char, ...);

    fn timerlat_bpf__open() -> *mut timerlat_bpf;
    fn timerlat_bpf__load(skel: *mut timerlat_bpf) -> c_int;
    fn timerlat_bpf__attach(skel: *mut timerlat_bpf) -> c_int;
    fn timerlat_bpf__detach(skel: *mut timerlat_bpf);
    fn timerlat_bpf__destroy(skel: *mut timerlat_bpf);

    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: c_uint) -> c_int;
    fn bpf_map__set_autocreate(map: *mut bpf_map, autocreate: bool) -> c_int;
    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *const c_void,
        value_sz: size_t,
        flags: c_ulonglong,
    ) -> c_int;
    fn bpf_map__lookup_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *mut c_void,
        value_sz: size_t,
        flags: c_ulonglong,
    ) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_object__close(obj: *mut bpf_object);

    fn ring_buffer__new(
        map_fd: c_int,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, size_t) -> c_int>,
        ctx: *mut c_void,
        opts: *const c_void,
    ) -> *mut ring_buffer;
    fn ring_buffer__poll(rb: *mut ring_buffer, timeout_ms: c_int) -> c_int;
    fn ring_buffer__free(rb: *mut ring_buffer);
}

/*
 * timerlat_bpf_init - load and initialize BPF program to collect timerlat data
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_bpf_init(params: *mut timerlat_params) -> c_int {
    let mut err: c_int;

    debug_msg(c"Loading BPF program\n".as_ptr());

    bpf = timerlat_bpf__open();
    if bpf.is_null() {
        return 1;
    }

    /* Pass common options */
    (*(*bpf).rodata).output_divisor = (*params).common.output_divisor;
    (*(*bpf).rodata).entries = (*params).common.hist.entries;
    (*(*bpf).rodata).irq_threshold = (*params).common.stop_us;
    (*(*bpf).rodata).thread_threshold = (*params).common.stop_total_us;
    (*(*bpf).rodata).aa_only = (*params).common.aa_only;

    if (*params).common.hist.entries != 0 {
        /* Pass histogram options */
        (*(*bpf).rodata).bucket_size = (*params).common.hist.bucket_size;

        /* Set histogram array sizes */
        bpf_map__set_max_entries((*bpf).maps.hist_irq, (*params).common.hist.entries);
        bpf_map__set_max_entries((*bpf).maps.hist_thread, (*params).common.hist.entries);
        bpf_map__set_max_entries((*bpf).maps.hist_user, (*params).common.hist.entries);
    } else {
        /* No entries, disable histogram */
        bpf_map__set_autocreate((*bpf).maps.hist_irq, false);
        bpf_map__set_autocreate((*bpf).maps.hist_thread, false);
        bpf_map__set_autocreate((*bpf).maps.hist_user, false);
    }

    if (*params).common.aa_only {
        /* Auto-analysis only, disable summary */
        bpf_map__set_autocreate((*bpf).maps.summary_irq, false);
        bpf_map__set_autocreate((*bpf).maps.summary_thread, false);
        bpf_map__set_autocreate((*bpf).maps.summary_user, false);
    }

    /* Load and verify BPF program */
    err = timerlat_bpf__load(bpf);
    if err != 0 {
        timerlat_bpf__destroy(bpf);
        return err;
    }

    0
}

/*
 * timerlat_bpf_set_action - set action on threshold executed on BPF side
 */
unsafe extern "C" fn timerlat_bpf_set_action(prog: *mut bpf_program) -> c_int {
    let key: c_uint = 0;
    let value: c_uint = bpf_program__fd(prog) as c_uint;

    bpf_map__update_elem(
        (*bpf).maps.bpf_action,
        &key as *const c_uint as *const c_void,
        size_of::<c_uint>(),
        &value as *const c_uint as *const c_void,
        size_of::<c_uint>(),
        BPF_ANY,
    )
}

/*
 * timerlat_bpf_attach - attach BPF program to collect timerlat data
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_bpf_attach() -> c_int {
    debug_msg(c"Attaching BPF program\n".as_ptr());

    timerlat_bpf__attach(bpf)
}

/*
 * timerlat_bpf_detach - detach BPF program to collect timerlat data
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_bpf_detach() {
    timerlat_bpf__detach(bpf);
}

/*
 * timerlat_bpf_detach - destroy BPF program to collect timerlat data
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_bpf_destroy() {
    timerlat_bpf__destroy(bpf);
    bpf = ptr::null_mut();
    if !obj.is_null() {
        bpf_object__close(obj);
    }
    obj = ptr::null_mut();
    prog = ptr::null_mut();
}

unsafe extern "C" fn handle_rb_event(
    _ctx: *mut c_void,
    _data: *mut c_void,
    _data_sz: size_t,
) -> c_int {
    0
}

/*
 * timerlat_bpf_wait - wait until tracing is stopped or signal
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_bpf_wait(timeout: c_int) -> c_int {
    let rb: *mut ring_buffer;
    let retval: c_int;

    rb = ring_buffer__new(
        bpf_map__fd((*bpf).maps.signal_stop_tracing),
        Some(handle_rb_event),
        ptr::null_mut(),
        ptr::null(),
    );
    retval = ring_buffer__poll(rb, timeout * 1000);
    ring_buffer__free(rb);

    retval
}

/*
 * timerlat_bpf_restart_tracing - restart stopped tracing
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_bpf_restart_tracing() -> c_int {
    let key: c_uint = 0;
    let value: c_ulonglong = 0;

    bpf_map__update_elem(
        (*bpf).maps.stop_tracing,
        &key as *const c_uint as *const c_void,
        size_of::<c_uint>(),
        &value as *const c_ulonglong as *const c_void,
        size_of::<c_ulonglong>(),
        BPF_ANY,
    )
}

unsafe extern "C" fn get_value(
    map_irq: *mut bpf_map,
    map_thread: *mut bpf_map,
    map_user: *mut bpf_map,
    key: c_int,
    value_irq: *mut i64,
    value_thread: *mut i64,
    value_user: *mut i64,
) -> c_int {
    let mut err: c_int;

    err = bpf_map__lookup_elem(
        map_irq,
        &key as *const c_int as *const c_void,
        size_of::<c_uint>(),
        value_irq as *mut c_void,
        size_of::<i64>() * nr_cpus as usize,
        0,
    );
    if err != 0 {
        return err;
    }
    err = bpf_map__lookup_elem(
        map_thread,
        &key as *const c_int as *const c_void,
        size_of::<c_uint>(),
        value_thread as *mut c_void,
        size_of::<i64>() * nr_cpus as usize,
        0,
    );
    if err != 0 {
        return err;
    }
    err = bpf_map__lookup_elem(
        map_user,
        &key as *const c_int as *const c_void,
        size_of::<c_uint>(),
        value_user as *mut c_void,
        size_of::<i64>() * nr_cpus as usize,
        0,
    );
    if err != 0 {
        return err;
    }
    0
}

/*
 * timerlat_bpf_get_hist_value - get value from BPF hist map
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_bpf_get_hist_value(
    key: c_int,
    value_irq: *mut i64,
    value_thread: *mut i64,
    value_user: *mut i64,
) -> c_int {
    get_value(
        (*bpf).maps.hist_irq,
        (*bpf).maps.hist_thread,
        (*bpf).maps.hist_user,
        key,
        value_irq,
        value_thread,
        value_user,
    )
}

/*
 * timerlat_bpf_get_summary_value - get value from BPF summary map
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_bpf_get_summary_value(
    key: summary_field,
    value_irq: *mut i64,
    value_thread: *mut i64,
    value_user: *mut i64,
) -> c_int {
    get_value(
        (*bpf).maps.summary_irq,
        (*bpf).maps.summary_thread,
        (*bpf).maps.summary_user,
        key as c_int,
        value_irq,
        value_thread,
        value_user,
    )
}

/*
 * timerlat_load_bpf_action_program - load and register a BPF action program
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_load_bpf_action_program(program_path: *const c_char) -> c_int {
    let mut err: c_int;

    obj = bpf_object__open_file(program_path, ptr::null());
    if obj.is_null() {
        err_msg(
            c"Failed to open BPF action program: %s\n".as_ptr(),
            program_path,
        );
        return 1;
    }

    err = bpf_object__load(obj);
    if err != 0 {
        err_msg(
            c"Failed to load BPF action program: %s\n".as_ptr(),
            program_path,
        );
        bpf_object__close(obj);
        obj = ptr::null_mut();
        return 1;
    }

    prog = bpf_object__find_program_by_name(obj, c"action_handler".as_ptr());
    if prog.is_null() {
        err_msg(
            c"BPF action program must have 'action_handler' function: %s\n".as_ptr(),
            program_path,
        );
        bpf_object__close(obj);
        obj = ptr::null_mut();
        return 1;
    }

    err = timerlat_bpf_set_action(prog);
    if err != 0 {
        err_msg(
            c"Failed to register BPF action program: %s\n".as_ptr(),
            program_path,
        );
        prog = ptr::null_mut();
        bpf_object__close(obj);
        obj = ptr::null_mut();
        return 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
