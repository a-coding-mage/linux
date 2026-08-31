// SPDX-License-Identifier: GPL-2.0
// Translated from C source: testing/selftests/net/tcp_ao/lib/ftrace.c
// Dependencies originally came from system headers, linux/kernel.h, and aolib.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type time_t = c_long;
type suseconds_t = c_long;
type uint64_t = u64;
type pthread_t = c_ulong;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_cond_t {
    _private: [u8; 48],
}

#[repr(C)]
pub struct pthread_mutex_t {
    _private: [u8; 40],
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ftracer_op {
    FTRACER_LINE_DISCARD,
    FTRACER_LINE_PRESERVE,
    FTRACER_EXIT,
}

#[repr(C)]
pub struct test_ftracer {
    pub tracer_thread: pthread_t,
    pub error: c_int,
    pub instance_path: *mut c_char,
    pub trace_pipe: *mut FILE,

    pub process_line: Option<unsafe extern "C" fn(line: *const c_char) -> ftracer_op>,
    pub destructor: Option<unsafe extern "C" fn(tracer: *mut test_ftracer)>,
    pub expecting_more: Option<unsafe extern "C" fn() -> bool>,

    pub saved_lines: *mut *mut c_char,
    pub saved_lines_size: size_t,
    pub next_line_ind: size_t,

    pub met_all_expected: pthread_cond_t,
    pub met_all_expected_lock: pthread_mutex_t,

    pub next: *mut test_ftracer,
}

#[repr(C)]
struct opts_list_t {
    opt_name: *mut c_char,
    next: *mut opts_list_t,
}

const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const SEEK_SET: c_int = 0;
const PTHREAD_CANCEL_ENABLE: c_int = 0;
const PTHREAD_CANCEL_DISABLE: c_int = 1;
const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;
const SOL_SOCKET: c_int = 1;
const SO_NETNS_COOKIE: c_int = 71;

extern "C" {
    static mut errno: c_int;
    static PTHREAD_CANCELED: *mut c_void;
    static KCONFIG_FTRACE: c_int;
    static TEST_TIMEOUT_SEC: c_uint;

    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn umount(target: *const c_char) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_cancel(thread: pthread_t) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_setcancelstate(state: c_int, oldstate: *mut c_int) -> c_int;
    fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const c_void) -> c_int;
    fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_timedwait(
        cond: *mut pthread_cond_t,
        mutex: *mut pthread_mutex_t,
        abstime: *const timespec,
    ) -> c_int;
    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const c_void) -> c_int;
    fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn getsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn test_error(fmt: *const c_char, ...);
    fn test_print(fmt: *const c_char, ...);
    fn test_fail(fmt: *const c_char, ...);
    fn test_sprintf(fmt: *const c_char, ...) -> *mut c_char;
    fn test_echo(path: *const c_char, flags: c_int, fmt: *const c_char, ...) -> c_int;
    fn test_add_destructor(destructor: unsafe extern "C" fn());
    fn setup_aolib_ftracer() -> c_int;
    fn switch_save_ns(nsfd: c_int) -> c_int;
    fn switch_close_ns(nsfd: c_int);
    fn kernel_config_has(config: c_int) -> bool;
}

static mut ftrace_path: [c_char; 20] = *b"ksft-ftrace-XXXXXX\0".as_ptr().cast::<[c_char; 20]>();
static mut ftrace_mounted: bool = false;
pub static mut ns_cookie1: uint64_t = 0;
pub static mut ns_cookie2: uint64_t = 0;

static mut ftracers: *mut test_ftracer = ptr::null_mut();
static mut ftracers_lock: pthread_mutex_t = unsafe { mem::zeroed() };

unsafe fn neg_errno() -> c_int {
    -errno
}

unsafe extern "C" fn mount_ftrace() -> c_int {
    if mkdtemp(ftrace_path.as_mut_ptr()).is_null() {
        test_error(c"Can't create temp dir".as_ptr());
    }

    if mount(
        c"tracefs".as_ptr(),
        ftrace_path.as_ptr(),
        c"tracefs".as_ptr(),
        0,
        c"rw".as_ptr() as *const c_void,
    ) != 0
    {
        return neg_errno();
    }

    ftrace_mounted = true;

    0
}

unsafe extern "C" fn unmount_ftrace() {
    if ftrace_mounted && umount(ftrace_path.as_ptr()) != 0 {
        test_print(c"Failed on cleanup: can't unmount tracefs: %m".as_ptr());
    }

    if rmdir(ftrace_path.as_ptr()) != 0 {
        test_error(
            c"Failed on cleanup: can't remove ftrace dir %s".as_ptr(),
            ftrace_path.as_ptr(),
        );
    }
}

unsafe extern "C" fn disable_trace_options(ftrace_path_arg: *const c_char) -> c_int {
    let mut opts_list: *mut opts_list_t = ptr::null_mut();
    let mut line: *mut c_char = ptr::null_mut();
    let mut buf_len: size_t = 0;
    let mut line_len: ssize_t;
    let mut ret: c_int = 0;

    let fopts = test_sprintf(c"%s/%s".as_ptr(), ftrace_path_arg, c"trace_options".as_ptr());
    if fopts.is_null() {
        return -ENOMEM;
    }

    let opts = fopen(fopts, c"r+".as_ptr());
    if opts.is_null() {
        ret = neg_errno();
        free(fopts as *mut c_void);
        return ret;
    }

    loop {
        line_len = getline(&mut line, &mut buf_len, opts);
        if line_len == -1 {
            break;
        }

        if strncmp(line, c"no".as_ptr(), 2) == 0 {
            continue;
        }

        let tmp = malloc(mem::size_of::<opts_list_t>()) as *mut opts_list_t;
        if tmp.is_null() {
            ret = -ENOMEM;
            break;
        }
        (*tmp).next = opts_list;
        (*tmp).opt_name = test_sprintf(c"no%s".as_ptr(), line);
        if (*tmp).opt_name.is_null() {
            ret = -ENOMEM;
            free(tmp as *mut c_void);
            break;
        }
        opts_list = tmp;
    }

    if ret == 0 {
        while !opts_list.is_null() {
            let tmp = opts_list;

            fseek(opts, 0, SEEK_SET);
            fwrite(
                (*tmp).opt_name as *const c_void,
                1,
                strlen((*tmp).opt_name),
                opts,
            );

            opts_list = (*opts_list).next;
            free((*tmp).opt_name as *mut c_void);
            free(tmp as *mut c_void);
        }
    }

    while !opts_list.is_null() {
        let tmp = opts_list;

        opts_list = (*opts_list).next;
        free((*tmp).opt_name as *mut c_void);
        free(tmp as *mut c_void);
    }
    free(line as *mut c_void);
    fclose(opts);
    free(fopts as *mut c_void);
    ret
}

unsafe extern "C" fn setup_buffer_size(ftrace_path_arg: *const c_char, sz: size_t) -> c_int {
    let fbuf_size = test_sprintf(c"%s/buffer_size_kb".as_ptr(), ftrace_path_arg);
    let ret: c_int;

    if fbuf_size.is_null() {
        return -1;
    }

    ret = test_echo(fbuf_size, 0, c"%zu".as_ptr(), sz);
    free(fbuf_size as *mut c_void);
    ret
}

unsafe extern "C" fn setup_ftrace_instance(
    tracer: *mut test_ftracer,
    name: *const c_char,
) -> c_int {
    let tmp = test_sprintf(c"%s/instances/ksft-%s-XXXXXX".as_ptr(), ftrace_path.as_ptr(), name);
    if tmp.is_null() {
        return -ENOMEM;
    }

    (*tracer).instance_path = mkdtemp(tmp);
    if (*tracer).instance_path.is_null() {
        free(tmp as *mut c_void);
        return neg_errno();
    }

    0
}

unsafe extern "C" fn remove_ftrace_instance(tracer: *mut test_ftracer) {
    if rmdir((*tracer).instance_path) != 0 {
        test_print(
            c"Failed on cleanup: can't remove ftrace instance %s".as_ptr(),
            (*tracer).instance_path,
        );
    }
    free((*tracer).instance_path as *mut c_void);
}

unsafe extern "C" fn tracer_cleanup(arg: *mut c_void) {
    let tracer = arg as *mut test_ftracer;

    fclose((*tracer).trace_pipe);
}

unsafe extern "C" fn tracer_set_error(tracer: *mut test_ftracer, error: c_int) {
    if (*tracer).error == 0 {
        (*tracer).error = error;
    }
}

#[no_mangle]
pub unsafe extern "C" fn tracer_get_savedlines_nr(tracer: *mut test_ftracer) -> size_t {
    (*tracer).next_line_ind
}

#[no_mangle]
pub unsafe extern "C" fn tracer_get_savedlines(tracer: *mut test_ftracer) -> *mut *const c_char {
    (*tracer).saved_lines as *mut *const c_char
}

unsafe extern "C" fn tracer_thread_func(arg: *mut c_void) -> *mut c_void {
    let tracer = arg as *mut test_ftracer;

    // C used pthread_cleanup_push(tracer_cleanup, arg); translate the cleanup
    // action explicitly before returning from this routine.
    while (*tracer).next_line_ind < (*tracer).saved_lines_size {
        let lp = (*tracer).saved_lines.add((*tracer).next_line_ind);
        let mut buf_len: size_t = 0;
        let line_len: ssize_t;

        line_len = getline(lp, &mut buf_len, (*tracer).trace_pipe);
        if line_len == -1 {
            break;
        }

        pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, ptr::null_mut());
        let op = (*tracer).process_line.unwrap()(*lp);
        pthread_setcancelstate(PTHREAD_CANCEL_ENABLE, ptr::null_mut());

        if let Some(expecting_more) = (*tracer).expecting_more {
            pthread_mutex_lock(&mut (*tracer).met_all_expected_lock);
            if !expecting_more() {
                pthread_cond_signal(&mut (*tracer).met_all_expected);
            }
            pthread_mutex_unlock(&mut (*tracer).met_all_expected_lock);
        }

        if op == ftracer_op::FTRACER_LINE_DISCARD {
            continue;
        }
        if op == ftracer_op::FTRACER_EXIT {
            break;
        }
        if op != ftracer_op::FTRACER_LINE_PRESERVE {
            test_error(c"unexpected tracer command %d".as_ptr(), op as c_int);
        }

        (*tracer).next_line_ind += 1;
        buf_len = 0;
        let _ = buf_len;
    }
    test_print(
        c"too many lines in ftracer buffer %zu, exiting tracer".as_ptr(),
        (*tracer).next_line_ind,
    );

    tracer_cleanup(arg);
    ptr::null_mut()
}

unsafe extern "C" fn setup_trace_thread(tracer: *mut test_ftracer) -> c_int {
    let mut ret: c_int = 0;

    let path = test_sprintf(c"%s/trace_pipe".as_ptr(), (*tracer).instance_path);
    if path.is_null() {
        return -ENOMEM;
    }

    (*tracer).trace_pipe = fopen(path, c"r".as_ptr());
    if (*tracer).trace_pipe.is_null() {
        ret = neg_errno();
        free(path as *mut c_void);
        return ret;
    }

    if pthread_create(
        &mut (*tracer).tracer_thread,
        ptr::null(),
        tracer_thread_func,
        tracer as *mut c_void,
    ) != 0
    {
        ret = neg_errno();
        fclose((*tracer).trace_pipe);
    }

    free(path as *mut c_void);
    ret
}

unsafe extern "C" fn stop_trace_thread(tracer: *mut test_ftracer) {
    let mut res: *mut c_void = ptr::null_mut();

    if pthread_cancel((*tracer).tracer_thread) != 0 {
        test_print(c"Can't stop tracer pthread: %m".as_ptr());
        tracer_set_error(tracer, neg_errno());
    }
    if pthread_join((*tracer).tracer_thread, &mut res) != 0 {
        test_print(c"Can't join tracer pthread: %m".as_ptr());
        tracer_set_error(tracer, neg_errno());
    }
    if res != PTHREAD_CANCELED {
        test_print(c"Tracer thread wasn't canceled".as_ptr());
        tracer_set_error(tracer, neg_errno());
    }
    if (*tracer).error != 0 {
        test_fail(c"tracer errored by %s".as_ptr(), strerror((*tracer).error));
    }
}

unsafe extern "C" fn final_wait_for_events(tracer: *mut test_ftracer, timeout_sec: c_uint) {
    let mut timeout: timespec = mem::zeroed();
    let mut now: timeval = mem::zeroed();
    let mut ret: c_int = 0;

    let Some(expecting_more) = (*tracer).expecting_more else {
        return;
    };

    pthread_mutex_lock(&mut (*tracer).met_all_expected_lock);
    gettimeofday(&mut now, ptr::null_mut());
    timeout.tv_sec = now.tv_sec + timeout_sec as time_t;
    timeout.tv_nsec = now.tv_usec * 1000;

    while expecting_more() && ret != ETIMEDOUT {
        ret = pthread_cond_timedwait(
            &mut (*tracer).met_all_expected,
            &mut (*tracer).met_all_expected_lock,
            &timeout,
        );
    }
    pthread_mutex_unlock(&mut (*tracer).met_all_expected_lock);
}

#[no_mangle]
pub unsafe extern "C" fn setup_trace_event(
    tracer: *mut test_ftracer,
    event: *const c_char,
    filter: *const c_char,
) -> c_int {
    let instance = (*tracer).instance_path;
    let mut ret: c_int;

    let enable_path = test_sprintf(c"%s/events/%s/enable".as_ptr(), instance, event);
    if enable_path.is_null() {
        return -ENOMEM;
    }

    let filter_path = test_sprintf(c"%s/events/%s/filter".as_ptr(), instance, event);
    if filter_path.is_null() {
        ret = -ENOMEM;
        free(enable_path as *mut c_void);
        return ret;
    }

    ret = test_echo(filter_path, 0, c"%s".as_ptr(), filter);
    if ret == 0 {
        ret = test_echo(enable_path, 0, c"1".as_ptr());
    }

    free(filter_path as *mut c_void);
    free(enable_path as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn create_ftracer(
    name: *const c_char,
    process_line: Option<unsafe extern "C" fn(line: *const c_char) -> ftracer_op>,
    destructor: Option<unsafe extern "C" fn(tracer: *mut test_ftracer)>,
    expecting_more: Option<unsafe extern "C" fn() -> bool>,
    lines_buf_sz: size_t,
    buffer_size_kb: size_t,
) -> *mut test_ftracer {
    let tracer: *mut test_ftracer;
    let mut err: c_int;

    /* XXX: separate __create_ftracer() helper and do here
     * if (!kernel_config_has(KCONFIG_FTRACE))
     *	return NULL;
     */

    tracer = malloc(mem::size_of::<test_ftracer>()) as *mut test_ftracer;
    if tracer.is_null() {
        test_print(c"malloc()".as_ptr());
        return ptr::null_mut();
    }

    memset(tracer as *mut c_void, 0, mem::size_of::<test_ftracer>());

    err = setup_ftrace_instance(tracer, name);
    if err != 0 {
        test_print(c"setup_ftrace_instance(): %d".as_ptr(), err);
        free(tracer as *mut c_void);
        return ptr::null_mut();
    }

    err = disable_trace_options((*tracer).instance_path);
    if err != 0 {
        test_print(c"disable_trace_options(): %d".as_ptr(), err);
        remove_ftrace_instance(tracer);
        free(tracer as *mut c_void);
        return ptr::null_mut();
    }

    err = setup_buffer_size((*tracer).instance_path, buffer_size_kb);
    if err != 0 {
        test_print(c"disable_trace_options(): %d".as_ptr(), err);
        remove_ftrace_instance(tracer);
        free(tracer as *mut c_void);
        return ptr::null_mut();
    }

    (*tracer).saved_lines = calloc(lines_buf_sz, mem::size_of::<*mut c_char>()) as *mut *mut c_char;
    if (*tracer).saved_lines.is_null() {
        test_print(c"calloc()".as_ptr());
        remove_ftrace_instance(tracer);
        free(tracer as *mut c_void);
        return ptr::null_mut();
    }
    (*tracer).saved_lines_size = lines_buf_sz;

    (*tracer).process_line = process_line;
    (*tracer).destructor = destructor;
    (*tracer).expecting_more = expecting_more;

    err = pthread_cond_init(&mut (*tracer).met_all_expected, ptr::null());
    if err != 0 {
        test_print(c"pthread_cond_init(): %d".as_ptr(), err);
        free((*tracer).saved_lines as *mut c_void);
        remove_ftrace_instance(tracer);
        free(tracer as *mut c_void);
        return ptr::null_mut();
    }

    err = pthread_mutex_init(&mut (*tracer).met_all_expected_lock, ptr::null());
    if err != 0 {
        test_print(c"pthread_mutex_init(): %d".as_ptr(), err);
        pthread_cond_destroy(&mut (*tracer).met_all_expected);
        free((*tracer).saved_lines as *mut c_void);
        remove_ftrace_instance(tracer);
        free(tracer as *mut c_void);
        return ptr::null_mut();
    }

    err = setup_trace_thread(tracer);
    if err != 0 {
        test_print(c"setup_trace_thread(): %d".as_ptr(), err);
        pthread_mutex_destroy(&mut (*tracer).met_all_expected_lock);
        pthread_cond_destroy(&mut (*tracer).met_all_expected);
        free((*tracer).saved_lines as *mut c_void);
        remove_ftrace_instance(tracer);
        free(tracer as *mut c_void);
        return ptr::null_mut();
    }

    pthread_mutex_lock(&mut ftracers_lock);
    (*tracer).next = ftracers;
    ftracers = tracer;
    pthread_mutex_unlock(&mut ftracers_lock);

    tracer
}

unsafe extern "C" fn __destroy_ftracer(tracer: *mut test_ftracer) {
    let mut i: size_t;

    final_wait_for_events(tracer, TEST_TIMEOUT_SEC);
    stop_trace_thread(tracer);
    remove_ftrace_instance(tracer);
    if let Some(destructor) = (*tracer).destructor {
        destructor(tracer);
    }
    i = 0;
    while i < (*tracer).saved_lines_size {
        free(*(*tracer).saved_lines.add(i) as *mut c_void);
        i += 1;
    }
    pthread_cond_destroy(&mut (*tracer).met_all_expected);
    pthread_mutex_destroy(&mut (*tracer).met_all_expected_lock);
    free(tracer as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn destroy_ftracer(tracer: *mut test_ftracer) {
    pthread_mutex_lock(&mut ftracers_lock);
    if tracer == ftracers {
        ftracers = (*tracer).next;
    } else {
        let mut f = ftracers;

        while (*f).next != tracer {
            if (*f).next.is_null() {
                test_error(
                    c"tracers list corruption or double free %p".as_ptr(),
                    tracer as *mut c_void,
                );
            }
            f = (*f).next;
        }
        (*f).next = (*tracer).next;
    }
    (*tracer).next = ptr::null_mut();
    pthread_mutex_unlock(&mut ftracers_lock);
    __destroy_ftracer(tracer);
}

unsafe extern "C" fn destroy_all_ftracers() {
    let mut f: *mut test_ftracer;

    pthread_mutex_lock(&mut ftracers_lock);
    f = ftracers;
    ftracers = ptr::null_mut();
    pthread_mutex_unlock(&mut ftracers_lock);

    while !f.is_null() {
        let n = (*f).next;

        (*f).next = ptr::null_mut();
        __destroy_ftracer(f);
        f = n;
    }
}

unsafe extern "C" fn test_unset_tracing() {
    destroy_all_ftracers();
    unmount_ftrace();
}

#[no_mangle]
pub unsafe extern "C" fn test_setup_tracing() -> c_int {
    /*
     * Just a basic protection - this should be called only once from
     * lib/kconfig. Not thread safe, which is fine as it's early, before
     * threads are created.
     */
    static mut already_set: c_int = 0;
    let err: c_int;

    if already_set != 0 {
        return -1;
    }

    /* Needs net-namespace cookies for filters */
    if ns_cookie1 == ns_cookie2 {
        test_print(
            c"net-namespace cookies: %lu == %lu, can't set up tracing".as_ptr(),
            ns_cookie1,
            ns_cookie2,
        );
        return -1;
    }

    already_set = 1;

    test_add_destructor(test_unset_tracing);

    err = mount_ftrace();
    if err != 0 {
        test_print(c"failed to mount_ftrace(): %d".as_ptr(), err);
        return err;
    }

    setup_aolib_ftracer()
}

unsafe extern "C" fn get_ns_cookie(nsfd: c_int, out: *mut uint64_t) -> c_int {
    let old_ns = switch_save_ns(nsfd);
    let mut size: socklen_t = mem::size_of_val(&*out) as socklen_t;
    let sk: c_int;

    sk = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_print(c"socket(): %m".as_ptr());
        return neg_errno();
    }

    if getsockopt(
        sk,
        SOL_SOCKET,
        SO_NETNS_COOKIE,
        out as *mut c_void,
        &mut size,
    ) != 0
    {
        test_print(c"getsockopt(SO_NETNS_COOKIE): %m".as_ptr());
        close(sk);
        return neg_errno();
    }

    close(sk);
    switch_close_ns(old_ns);
    0
}

#[no_mangle]
pub unsafe extern "C" fn test_init_ftrace(nsfd1: c_int, nsfd2: c_int) {
    get_ns_cookie(nsfd1, &mut ns_cookie1);
    get_ns_cookie(nsfd2, &mut ns_cookie2);
    /* Populate kernel config state */
    kernel_config_has(KCONFIG_FTRACE);
}
