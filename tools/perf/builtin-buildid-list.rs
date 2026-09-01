/*
 * builtin-buildid-list.c
 *
 * Builtin buildid-list command: list buildids in perf.data, in the running
 * kernel and in ELF files.
 *
 * Copyright (C) 2009, Red Hat Inc.
 * Copyright (C) 2009, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const PERF_DATA_MODE_READ: c_int = 0;
const HEADER_AUXTRACE: c_int = 0;
const HEADER_BUILD_ID: c_int = 0;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zstd_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    pub header: perf_header,
    pub zstd_data: zstd_data,
}

#[repr(C)]
pub struct perf_data {
    pub path: *const c_char,
    pub mode: c_int,
    pub force: bool,
}

#[repr(C)]
pub struct perf_tool {
    pub sample: Option<unsafe extern "C" fn()>,
    pub mmap: Option<unsafe extern "C" fn()>,
    pub mmap2: Option<unsafe extern "C" fn()>,
    pub fork: Option<unsafe extern "C" fn()>,
    pub exit: Option<unsafe extern "C" fn()>,
    pub attr: Option<unsafe extern "C" fn()>,
    pub build_id: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut input_name: *const c_char;
    static mut verbose: c_int;
    static mut stdout: *mut FILE;

    static SBUILD_ID_SIZE: usize;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    fn map__dso(map: *mut map) -> *const dso;
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;

    fn dso__long_name(dso: *const dso) -> *const c_char;
    fn dso__short_name(dso: *const dso) -> *const c_char;
    fn dso__has_build_id(dso: *const dso) -> bool;
    fn dso__bid(dso: *const dso) -> *const c_void;
    fn dso__hit(dso: *mut dso) -> bool;

    fn build_id__snprintf(bid: *const c_void, buf: *mut c_char, size: usize) -> c_int;
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn machine__new_host(env: *mut perf_env) -> *mut machine;
    fn machine__for_each_kernel_map(
        machine: *mut machine,
        cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> c_int>,
        arg: *mut c_void,
    );
    fn machine__delete(machine: *mut machine);

    fn sysfs__snprintf_build_id(path: *const c_char, buf: *mut c_char, size: usize) -> c_int;
    fn filename__snprintf_build_id(name: *const c_char, buf: *mut c_char, size: usize) -> c_int;

    fn symbol__elf_init();
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool;
    fn perf_header__has_feat(header: *mut perf_header, feat: c_int) -> bool;
    fn zstd_init(data: *mut zstd_data, level: c_int) -> c_int;
    fn pr_warning(format: *const c_char, ...);
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_session__fprintf_dsos_buildid(
        session: *mut perf_session,
        fp: *mut FILE,
        skip: Option<unsafe extern "C" fn(*mut dso, c_int) -> bool>,
        parm: bool,
    ) -> c_int;
    fn perf_session__delete(session: *mut perf_session);

    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;

    fn build_id__mark_dso_hit();
    fn perf_event__process_mmap();
    fn perf_event__process_mmap2();
    fn perf_event__process_fork();
    fn perf_event__exit_del_thread();
    fn perf_event__process_attr();
    fn perf_event__process_build_id();

    fn OPT_BOOLEAN(short_name: c_char, long_name: *const c_char, value: *mut bool, help: *const c_char) -> option;
    fn OPT_STRING(
        short_name: c_char,
        long_name: *const c_char,
        value: *mut *const c_char,
        metavar: *const c_char,
        help: *const c_char,
    ) -> option;
    fn OPT_INCR(short_name: c_char, long_name: *const c_char, value: *mut c_int, help: *const c_char) -> option;
    fn OPT_END() -> option;
    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn setup_pager();
}

unsafe extern "C" fn buildid__map_cb(map: *mut map, arg: *mut c_void) -> c_int {
    let _ = arg;
    let dso = unsafe { map__dso(map) };
    let mut bid_buf: Vec<c_char> = vec![0; unsafe { SBUILD_ID_SIZE }];
    let dso_long_name = unsafe { dso__long_name(dso) };
    let dso_short_name = unsafe { dso__short_name(dso) };

    unsafe {
        memset(
            bid_buf.as_mut_ptr() as *mut c_void,
            0,
            bid_buf.len() as usize,
        );
    }
    if unsafe { dso__has_build_id(dso) } {
        unsafe {
            build_id__snprintf(dso__bid(dso), bid_buf.as_mut_ptr(), bid_buf.len());
        }
    }
    unsafe {
        printf(
            c"%s %16lx %16lx".as_ptr(),
            bid_buf.as_ptr(),
            map__start(map) as c_ulong,
            map__end(map) as c_ulong,
        );
    }
    if !dso_long_name.is_null() {
        unsafe {
            printf(c" %s".as_ptr(), dso_long_name);
        }
    } else if !dso_short_name.is_null() {
        unsafe {
            printf(c" %s".as_ptr(), dso_short_name);
        }
    }

    unsafe {
        printf(c"\n".as_ptr());
    }

    0
}

unsafe fn buildid__show_kernel_maps() {
    let mut host_env: perf_env = unsafe { core::mem::zeroed() };
    let machine: *mut machine;

    unsafe {
        perf_env__init(&mut host_env);
        machine = machine__new_host(&mut host_env);
        machine__for_each_kernel_map(machine, Some(buildid__map_cb), core::ptr::null_mut());
        machine__delete(machine);
        perf_env__exit(&mut host_env);
    }
}

unsafe fn sysfs__fprintf_build_id(fp: *mut FILE) -> c_int {
    let mut sbuild_id: Vec<c_char> = vec![0; unsafe { SBUILD_ID_SIZE }];
    let ret: c_int;

    unsafe {
        ret = sysfs__snprintf_build_id(c"/".as_ptr(), sbuild_id.as_mut_ptr(), sbuild_id.len());
    }
    if ret + 1 != sbuild_id.len() as c_int {
        return if ret < 0 { ret } else { -EINVAL };
    }

    unsafe { fprintf(fp, c"%s\n".as_ptr(), sbuild_id.as_ptr()) }
}

unsafe fn filename__fprintf_build_id(name: *const c_char, fp: *mut FILE) -> c_int {
    let mut sbuild_id: Vec<c_char> = vec![0; unsafe { SBUILD_ID_SIZE }];
    let ret: c_int;

    unsafe {
        ret = filename__snprintf_build_id(name, sbuild_id.as_mut_ptr(), sbuild_id.len());
    }
    if ret + 1 != sbuild_id.len() as c_int {
        return if ret < 0 { ret } else { -EINVAL };
    }

    unsafe { fprintf(fp, c"%s\n".as_ptr(), sbuild_id.as_ptr()) }
}

unsafe extern "C" fn dso__skip_buildid(dso: *mut dso, with_hits: c_int) -> bool {
    with_hits != 0 && unsafe { !dso__hit(dso) }
}

unsafe fn perf_session__list_build_ids(force: bool, mut with_hits: bool) -> c_int {
    let session: *mut perf_session;
    let mut data = perf_data {
        path: unsafe { input_name },
        mode: PERF_DATA_MODE_READ,
        force,
    };
    let mut build_id__mark_dso_hit_ops: perf_tool = unsafe { core::mem::zeroed() };

    unsafe {
        symbol__elf_init();
    }
    /*
     * See if this is an ELF file first:
     */
    if unsafe { filename__fprintf_build_id(input_name, stdout) } > 0 {
        return 0;
    }

    unsafe {
        perf_tool__init(&mut build_id__mark_dso_hit_ops, true);
    }
    build_id__mark_dso_hit_ops.sample = Some(unsafe { core::mem::transmute(build_id__mark_dso_hit as unsafe extern "C" fn()) });
    build_id__mark_dso_hit_ops.mmap = Some(unsafe { core::mem::transmute(perf_event__process_mmap as unsafe extern "C" fn()) });
    build_id__mark_dso_hit_ops.mmap2 = Some(unsafe { core::mem::transmute(perf_event__process_mmap2 as unsafe extern "C" fn()) });
    build_id__mark_dso_hit_ops.fork = Some(unsafe { core::mem::transmute(perf_event__process_fork as unsafe extern "C" fn()) });
    build_id__mark_dso_hit_ops.exit = Some(unsafe { core::mem::transmute(perf_event__exit_del_thread as unsafe extern "C" fn()) });
    build_id__mark_dso_hit_ops.attr = Some(unsafe { core::mem::transmute(perf_event__process_attr as unsafe extern "C" fn()) });
    build_id__mark_dso_hit_ops.build_id = Some(unsafe { core::mem::transmute(perf_event__process_build_id as unsafe extern "C" fn()) });

    session = unsafe { perf_session__new(&mut data, &mut build_id__mark_dso_hit_ops) };
    if unsafe { IS_ERR(session as *const c_void) } {
        return unsafe { PTR_ERR(session as *const c_void) as c_int };
    }

    /*
     * We take all buildids when the file contains AUX area tracing data
     * because we do not decode the trace because it would take too long.
     */
    if unsafe { !perf_data__is_pipe(&mut data) }
        && unsafe { perf_header__has_feat(&mut (*session).header, HEADER_AUXTRACE) }
    {
        with_hits = false;
    }

    if unsafe { !perf_header__has_feat(&mut (*session).header, HEADER_BUILD_ID) } {
        with_hits = true;
    }

    if unsafe { zstd_init(&mut (*session).zstd_data, 0) } < 0 {
        unsafe {
            pr_warning(c"Decompression initialization failed. Reported data may be incomplete.\n".as_ptr());
        }
    }

    /*
     * in pipe-mode, the only way to get the buildids is to parse
     * the record stream. Buildids are stored as RECORD_HEADER_BUILD_ID
     */
    if with_hits || unsafe { perf_data__is_pipe(&mut data) } {
        unsafe {
            perf_session__process_events(session);
        }
    }

    unsafe {
        perf_session__fprintf_dsos_buildid(
            session,
            stdout,
            Some(dso__skip_buildid),
            with_hits,
        );
        perf_session__delete(session);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_buildid_list(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut show_kernel = false;
    let mut show_kernel_maps = false;
    let mut with_hits = false;
    let mut force = false;
    let options = [
        unsafe { OPT_BOOLEAN(b'H' as c_char, c"with-hits".as_ptr(), &mut with_hits, c"Show only DSOs with hits".as_ptr()) },
        unsafe { OPT_STRING(b'i' as c_char, c"input".as_ptr(), &mut input_name, c"file".as_ptr(), c"input file name".as_ptr()) },
        unsafe { OPT_BOOLEAN(b'f' as c_char, c"force".as_ptr(), &mut force, c"don't complain, do it".as_ptr()) },
        unsafe { OPT_BOOLEAN(b'k' as c_char, c"kernel".as_ptr(), &mut show_kernel, c"Show current kernel build id".as_ptr()) },
        unsafe {
            OPT_BOOLEAN(
                b'm' as c_char,
                c"kernel-maps".as_ptr(),
                &mut show_kernel_maps,
                c"Show build id of current kernel + modules".as_ptr(),
            )
        },
        unsafe { OPT_INCR(b'v' as c_char, c"verbose".as_ptr(), &mut verbose, c"be more verbose".as_ptr()) },
        unsafe { OPT_END() },
    ];
    let buildid_list_usage = [c"perf buildid-list [<options>]".as_ptr(), core::ptr::null()];

    argc = unsafe { parse_options(argc, argv, options.as_ptr(), buildid_list_usage.as_ptr(), 0) };
    unsafe {
        setup_pager();
    }

    if show_kernel {
        return !(unsafe { sysfs__fprintf_build_id(stdout) } > 0) as c_int;
    } else if show_kernel_maps {
        unsafe {
            buildid__show_kernel_maps();
        }
        return 0;
    }

    unsafe { perf_session__list_build_ids(force, with_hits) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
