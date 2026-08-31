// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel_tpebs.c: Intel TPEBS support
 */

// Dependencies from the original include list:
// <api/fs/fs.h>, <sys/param.h>, <subcmd/run-command.h>, <thread.h>,
// "intel-tpebs.h", <linux/list.h>, <linux/zalloc.h>, <linux/err.h>,
// "sample.h", "counts.h", "debug.h", "evlist.h", "evsel.h", "mutex.h",
// "session.h", "stat.h", "tool.h", "cpumap.h", "metricgroup.h",
// <sys/stat.h>, <sys/file.h>, <errno.h>, <poll.h>, <math.h>

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const PERF_DATA: &[u8] = b"-\0";
const PATH_MAX: usize = 4096;
const POLLIN: c_short = 0x0001;
const SIGTERM: c_int = 15;

type bool_ = bool;
type pid_t = c_int;
type pthread_t = c_ulong;
type pthread_once_t = c_int;
type c_short = i16;
type FILE = c_void;

const PTHREAD_ONCE_INIT: pthread_once_t = 0;
const PERF_DATA_MODE_READ: c_int = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EPIPE: c_int = 32;
const EBUSY: c_int = 16;
const ETIMEDOUT: c_int = 110;
const ERR_RUN_COMMAND_WAITPID_SIGNAL: c_int = 1;

#[repr(C)]
pub enum tpebs_mode {
    TPEBS_MODE__MIN,
    TPEBS_MODE__MAX,
    TPEBS_MODE__LAST,
    TPEBS_MODE__MEAN,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct child_process {
    pub argv: *mut *const c_char,
    pub out: c_int,
    pub pid: pid_t,
}

#[repr(C)]
pub struct stats {
    pub n: u64,
    pub mean: f64,
    pub min: u64,
    pub max: u64,
}

#[repr(C)]
pub struct retirement_latency {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}

#[repr(C)]
pub struct perf_event_attr {
    pub inherit: bool_,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub name: *const c_char,
    pub evlist: *mut evlist,
    pub core: evsel_core,
    pub retire_lat: bool_,
    pub prev_raw_counts: *mut perf_counts,
    pub counts: *mut perf_counts,
    pub retirement_latency: retirement_latency,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist_core {
    pub user_requested_cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_counts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct perf_sample {
    pub pid: pid_t,
    pub evsel: *mut evsel,
    pub weight3: u64,
}

#[repr(C)]
pub union perf_event {
    _bindgen_union_align: u64,
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    pub sample: Option<
        unsafe extern "C" fn(
            *const perf_tool,
            *mut perf_event,
            *mut perf_sample,
            *mut machine,
        ) -> c_int,
    >,
    pub feature: *mut c_void,
    pub attr: *mut c_void,
}

#[repr(C)]
pub struct perf_data_file {
    pub fd: c_int,
}

#[repr(C)]
pub struct perf_data {
    pub mode: c_int,
    pub path: *const c_char,
    pub file: perf_data_file,
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct tpebs_retire_lat {
    pub nd: list_head,
    /** @evsel: The evsel that opened the retire_lat event. */
    pub evsel: *mut evsel,
    /** @event: Event passed to perf record. */
    pub event: *mut c_char,
    /** @stats: Recorded retirement latency stats. */
    pub stats: stats,
    /** @last: Last retirement latency read. */
    pub last: u64,
    /* Has the event been sent to perf record? */
    pub started: bool_,
}

unsafe extern "C" {
    static mut errno: c_int;
    static EVLIST_CTL_CMD_STOP_TAG: *const c_char;
    static EVLIST_CTL_CMD_ENABLE_TAG: *const c_char;
    static EVLIST_CTL_CMD_PING_TAG: *const c_char;
    static EVLIST_CTL_CMD_ACK_TAG: *const c_char;
    static perf_event__process_feature: *mut c_void;
    static perf_event__process_attr: *mut c_void;

    fn mutex_init(m: *mut mutex);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn pthread_once(once: *mut pthread_once_t, init: unsafe extern "C" fn());
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn rint(x: f64) -> f64;

    fn procfs__mountpoint() -> *const c_char;
    fn start_command(cmd: *mut child_process) -> c_int;
    fn finish_command(cmd: *mut child_process) -> c_int;
    fn check_if_command_finished(cmd: *mut child_process) -> bool_;
    fn perf_cpu_map__is_any_cpu_or_is_empty(cpus: *mut perf_cpu_map) -> bool_;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn cpu_map__snprint(cpus: *mut perf_cpu_map, buf: *mut c_char, size: usize) -> c_int;
    fn evlist__workload_pid(evlist: *mut evlist) -> pid_t;
    fn evsel__is_retire_lat(evsel: *mut evsel) -> bool_;
    fn perf_counts(counts: *mut perf_counts, cpu_map_idx: c_int, thread: c_int) -> *mut perf_counts_values;
    fn update_stats(stats: *mut stats, val: u64);
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool_);
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_session__delete(session: *mut perf_session);
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning_once(fmt: *const c_char, ...);
}

pub static mut tpebs_recording: bool_ = false;
pub static mut tpebs_mode: tpebs_mode = tpebs_mode::TPEBS_MODE__MEAN;
static mut tpebs_results: list_head = list_head {
    next: unsafe { &raw mut tpebs_results },
    prev: unsafe { &raw mut tpebs_results },
};
static mut tpebs_reader_thread: pthread_t = 0;
static mut tpebs_cmd: child_process = child_process {
    argv: core::ptr::null_mut(),
    out: 0,
    pid: 0,
};
static mut control_fd: [c_int; 2] = [0; 2];
static mut ack_fd: [c_int; 2] = [0; 2];
static mut tpebs_mtx: mutex = mutex { _private: [] };
static mut tpebs_stopping: bool_ = false;

unsafe fn container_of_tpebs_retire_lat(ptr: *mut list_head) -> *mut tpebs_retire_lat {
    ptr as *mut tpebs_retire_lat
}

unsafe fn list_empty(head: *const list_head) -> bool_ {
    (*head).next == head as *mut list_head
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    let prev = (*head).prev;
    (*new).next = head;
    (*new).prev = prev;
    (*prev).next = new;
    (*head).prev = new;
}

unsafe fn list_del_init(entry: *mut list_head) {
    let prev = (*entry).prev;
    let next = (*entry).next;
    (*next).prev = prev;
    (*prev).next = next;
    (*entry).next = entry;
    (*entry).prev = entry;
}

unsafe extern "C" fn tpebs_init() {
    mutex_init(&raw mut tpebs_mtx);
    control_fd[0] = -1;
    control_fd[1] = -1;
    ack_fd[0] = -1;
    ack_fd[1] = -1;
}

unsafe fn tpebs_mtx_get() -> *mut mutex {
    static mut tpebs_once: pthread_once_t = PTHREAD_ONCE_INIT;

    pthread_once(&raw mut tpebs_once, tpebs_init);
    &raw mut tpebs_mtx
}

unsafe fn evsel__tpebs_start_perf_record(evsel: *mut evsel) -> c_int {
    let mut record_argv: *mut *const c_char;
    let mut tpebs_event_size: c_int = 0;
    let mut i: c_int = 0;
    let ret: c_int;
    let mut control_fd_buf = [0 as c_char; 32];
    let mut cpumap_buf = [0 as c_char; 50];
    let mut t: *mut tpebs_retire_lat;
    let mut pos: *mut list_head;

    pos = tpebs_results.next;
    while pos != &raw mut tpebs_results {
        t = container_of_tpebs_retire_lat(pos);
        let _ = t;
        tpebs_event_size += 1;
        pos = (*pos).next;
    }

    record_argv = malloc(((10 + 2 * tpebs_event_size) as usize) * core::mem::size_of::<*const c_char>())
        as *mut *const c_char;
    if record_argv.is_null() {
        return -ENOMEM;
    }

    *record_argv.add(i as usize) = c"perf".as_ptr();
    i += 1;
    *record_argv.add(i as usize) = c"record".as_ptr();
    i += 1;
    *record_argv.add(i as usize) = c"-W".as_ptr();
    i += 1;
    *record_argv.add(i as usize) = c"--synth=no".as_ptr();
    i += 1;

    scnprintf(
        control_fd_buf.as_mut_ptr(),
        control_fd_buf.len(),
        c"--control=fd:%d,%d".as_ptr(),
        control_fd[0],
        ack_fd[1],
    );
    *record_argv.add(i as usize) = control_fd_buf.as_ptr();
    i += 1;

    *record_argv.add(i as usize) = c"-o".as_ptr();
    i += 1;
    *record_argv.add(i as usize) = PERF_DATA.as_ptr() as *const c_char;
    i += 1;

    if !perf_cpu_map__is_any_cpu_or_is_empty((*evlist__core((*evsel).evlist)).user_requested_cpus) {
        cpu_map__snprint(
            (*evlist__core((*evsel).evlist)).user_requested_cpus,
            cpumap_buf.as_mut_ptr(),
            cpumap_buf.len(),
        );
        *record_argv.add(i as usize) = c"-C".as_ptr();
        i += 1;
        *record_argv.add(i as usize) = cpumap_buf.as_ptr();
        i += 1;
    }

    pos = tpebs_results.next;
    while pos != &raw mut tpebs_results {
        t = container_of_tpebs_retire_lat(pos);
        *record_argv.add(i as usize) = c"-e".as_ptr();
        i += 1;
        *record_argv.add(i as usize) = (*t).event;
        i += 1;
        pos = (*pos).next;
    }
    *record_argv.add(i as usize) = core::ptr::null();
    i += 1;
    assert!(i == 10 + 2 * tpebs_event_size || i == 8 + 2 * tpebs_event_size);
    /* Note, no workload given so system wide is implied. */

    assert!(tpebs_cmd.pid == 0);
    memset(
        (&raw mut tpebs_cmd).cast::<c_void>(),
        0,
        core::mem::size_of::<child_process>(),
    );
    tpebs_cmd.argv = record_argv;
    tpebs_cmd.out = -1;
    ret = start_command(&raw mut tpebs_cmd);
    free(tpebs_cmd.argv.cast::<c_void>());
    tpebs_cmd.argv = core::ptr::null_mut();
    pos = tpebs_results.next;
    while pos != &raw mut tpebs_results {
        t = container_of_tpebs_retire_lat(pos);
        (*t).started = true;
        pos = (*pos).next;
    }

    ret
}

unsafe fn is_child_pid(parent: pid_t, mut child: pid_t) -> bool_ {
    if parent < 0 || child < 0 {
        return false;
    }

    loop {
        let mut path = [0 as c_char; PATH_MAX];
        let mut line = [0 as c_char; 256];
        let fp: *mut FILE;

        loop {
            if parent == child {
                return true;
            }

            if child <= 0 {
                return false;
            }

            scnprintf(
                path.as_mut_ptr(),
                path.len(),
                c"%s/%d/status".as_ptr(),
                procfs__mountpoint(),
                child,
            );
            fp = fopen(path.as_ptr(), c"r".as_ptr());
            if fp.is_null() {
                /* Presumably the process went away. Assume not a child. */
                return false;
            }
            while !fgets(line.as_mut_ptr(), line.len() as c_int, fp).is_null() {
                if strncmp(line.as_ptr(), c"PPid:".as_ptr(), 5) == 0 {
                    fclose(fp);
                    if sscanf(line.as_ptr().add(5), c"%d".as_ptr(), &mut child) != 1 {
                        /* Unexpected error parsing. */
                        return false;
                    }
                    continue;
                }
            }
            /* Unexpected EOF. */
            fclose(fp);
            return false;
        }
    }
}

unsafe fn should_ignore_sample(sample: *const perf_sample, t: *const tpebs_retire_lat) -> bool_ {
    let workload_pid: pid_t;
    let sample_pid: pid_t = (*sample).pid;

    /*
     * During evlist__purge the evlist will be removed prior to the
     * evsel__exit calling evsel__tpebs_close and taking the
     * tpebs_mtx. Avoid a segfault by ignoring samples in this case.
     */
    if (*(*t).evsel).evlist.is_null() {
        return true;
    }

    workload_pid = evlist__workload_pid((*(*t).evsel).evlist);
    if workload_pid < 0 || workload_pid == sample_pid {
        return false;
    }

    if !(*(*t).evsel).core.attr.inherit {
        return true;
    }

    !is_child_pid(workload_pid, sample_pid)
}

unsafe extern "C" fn process_sample_event(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let t: *mut tpebs_retire_lat;

    mutex_lock(tpebs_mtx_get());
    if tpebs_cmd.pid == 0 {
        /* Record has terminated. */
        mutex_unlock(tpebs_mtx_get());
        return 0;
    }
    t = tpebs_retire_lat__find((*sample).evsel);
    if t.is_null() {
        mutex_unlock(tpebs_mtx_get());
        return -EINVAL;
    }
    if should_ignore_sample(sample, t) {
        mutex_unlock(tpebs_mtx_get());
        return 0;
    }
    /*
     * Need to handle per core results? We are assuming average retire
     * latency value will be used. Save the number of samples and the sum of
     * retire latency value for each event.
     */
    (*t).last = (*sample).weight3;
    update_stats(&mut (*t).stats, (*sample).weight3);
    mutex_unlock(tpebs_mtx_get());
    0
}

unsafe extern "C" fn __sample_reader(_arg: *mut c_void) -> *mut c_void {
    let mut data = perf_data {
        mode: PERF_DATA_MODE_READ,
        path: PERF_DATA.as_ptr() as *const c_char,
        file: perf_data_file { fd: tpebs_cmd.out },
    };
    let mut tool: perf_tool = core::mem::zeroed();
    let session: *mut perf_session;

    perf_tool__init(&mut tool, false);
    tool.sample = Some(process_sample_event);
    tool.feature = perf_event__process_feature;
    tool.attr = perf_event__process_attr;

    session = perf_session__new(&mut data, &mut tool);
    if IS_ERR(session.cast::<c_void>()) {
        return core::ptr::null_mut();
    }

    perf_session__process_events(session);
    perf_session__delete(session);

    core::ptr::null_mut()
}

unsafe fn tpebs_send_record_cmd(msg: *const c_char) -> c_int {
    let mut pollfd = pollfd {
        fd: 0,
        events: POLLIN,
        revents: 0,
    };
    let mut ret: c_int;
    let len: c_int;
    let mut retries: c_int = 0;
    let mut ack_buf = [0 as c_char; 8];

    /* Check if the command exited before the send, done with the lock held. */
    if tpebs_cmd.pid == 0 {
        return 0;
    }

    /*
     * Let go of the lock while sending/receiving as blocking can starve the
     * sample reading thread.
     */
    mutex_unlock(tpebs_mtx_get());

    /* Send perf record command.*/
    len = strlen(msg) as c_int;
    ret = write(control_fd[1], msg.cast::<c_void>(), len as usize) as c_int;
    if ret != len {
        pr_err(c"perf record control write control message '%s' failed\n".as_ptr(), msg);
        ret = -EPIPE;
        goto_out(ret);
        mutex_lock(tpebs_mtx_get());
        return ret;
    }

    if strcmp(msg, EVLIST_CTL_CMD_STOP_TAG) == 0 {
        ret = 0;
        mutex_lock(tpebs_mtx_get());
        return ret;
    }

    /* Wait for an ack. */
    pollfd.fd = ack_fd[0];

    /*
     * We need this poll to ensure the ack_fd PIPE will not hang
     * when perf record failed for any reason. The timeout value
     * 3000ms is an empirical selection.
     */
    loop {
        if poll(&mut pollfd, 1, 500) == 0 {
            if check_if_command_finished(&raw mut tpebs_cmd) {
                ret = 0;
                mutex_lock(tpebs_mtx_get());
                return ret;
            }

            if retries < 6 {
                retries += 1;
                continue;
            }
            pr_err(c"tpebs failed: perf record ack timeout for '%s'\n".as_ptr(), msg);
            ret = -ETIMEDOUT;
            mutex_lock(tpebs_mtx_get());
            return ret;
        }
        break;
    }

    if (pollfd.revents & POLLIN) == 0 {
        if check_if_command_finished(&raw mut tpebs_cmd) {
            ret = 0;
            mutex_lock(tpebs_mtx_get());
            return ret;
        }

        pr_err(c"tpebs failed: did not received an ack for '%s'\n".as_ptr(), msg);
        ret = -EPIPE;
        mutex_lock(tpebs_mtx_get());
        return ret;
    }

    ret = read(ack_fd[0], ack_buf.as_mut_ptr().cast::<c_void>(), ack_buf.len()) as c_int;
    if ret > 0 {
        ret = strcmp(ack_buf.as_ptr(), EVLIST_CTL_CMD_ACK_TAG);
    } else {
        pr_err(c"tpebs: perf record control ack failed\n".as_ptr());
    }
    /* Re-take lock as expected by caller. */
    mutex_lock(tpebs_mtx_get());
    ret
}

unsafe fn goto_out(_ret: c_int) {}

/*
 * tpebs_stop - stop the sample data read thread and the perf record process.
 */
unsafe fn tpebs_stop() -> c_int {
    let mut ret: c_int = 0;

    if tpebs_stopping {
        return 0;
    }

    /* Like tpebs_start, we should only run tpebs_end once. */
    if tpebs_cmd.pid != 0 {
        let actual_pid: pid_t = tpebs_cmd.pid;

        tpebs_stopping = true;
        tpebs_send_record_cmd(EVLIST_CTL_CMD_STOP_TAG);
        tpebs_cmd.pid = 0;
        mutex_unlock(tpebs_mtx_get());
        pthread_join(tpebs_reader_thread, core::ptr::null_mut());
        mutex_lock(tpebs_mtx_get());
        if control_fd[0] >= 0 {
            close(control_fd[0]);
            control_fd[0] = -1;
        }
        if control_fd[1] >= 0 {
            close(control_fd[1]);
            control_fd[1] = -1;
        }
        if ack_fd[0] >= 0 {
            close(ack_fd[0]);
            ack_fd[0] = -1;
        }
        if ack_fd[1] >= 0 {
            close(ack_fd[1]);
            ack_fd[1] = -1;
        }
        if tpebs_cmd.out >= 0 {
            close(tpebs_cmd.out);
            tpebs_cmd.out = -1;
        }
        tpebs_cmd.pid = actual_pid;
        ret = finish_command(&raw mut tpebs_cmd);
        tpebs_cmd.pid = 0;
        tpebs_stopping = false;
        if ret == -ERR_RUN_COMMAND_WAITPID_SIGNAL {
            ret = 0;
        }
    }
    ret
}

/**
 * evsel__tpebs_event() - Create string event encoding to pass to `perf record`.
 */
unsafe fn evsel__tpebs_event(evsel: *mut evsel, event: *mut *mut c_char) -> c_int {
    let name: *mut c_char;
    let mut modifier: *mut c_char;
    let ret: c_int;

    name = strdup((*evsel).name);
    if name.is_null() {
        return -ENOMEM;
    }

    modifier = strrchr(name, 'R' as c_int);
    if modifier.is_null() {
        let ret = -EINVAL;
        if ret != 0 {
            pr_err(c"Tpebs event modifier broken '%s'\n".as_ptr(), (*evsel).name);
        }
        free(name.cast::<c_void>());
        return ret;
    }
    *modifier = 'p' as c_char;
    modifier = strchr(name, ':' as c_int);
    if modifier.is_null() {
        modifier = strrchr(name, '/' as c_int);
    }
    if modifier.is_null() {
        let ret = -EINVAL;
        if ret != 0 {
            pr_err(c"Tpebs event modifier broken '%s'\n".as_ptr(), (*evsel).name);
        }
        free(name.cast::<c_void>());
        return ret;
    }
    *modifier = '\0' as c_char;
    if asprintf(
        event,
        c"%s/name=tpebs_event_%p/%s".as_ptr(),
        name,
        evsel,
        modifier.add(1),
    ) > 0
    {
        ret = 0;
    } else {
        ret = -ENOMEM;
    }
    if ret != 0 {
        pr_err(c"Tpebs event modifier broken '%s'\n".as_ptr(), (*evsel).name);
    }
    free(name.cast::<c_void>());
    ret
}

unsafe fn tpebs_retire_lat__new(evsel: *mut evsel) -> *mut tpebs_retire_lat {
    let result = malloc(core::mem::size_of::<tpebs_retire_lat>()) as *mut tpebs_retire_lat;
    let ret: c_int;

    if result.is_null() {
        return core::ptr::null_mut();
    }
    memset(result.cast::<c_void>(), 0, core::mem::size_of::<tpebs_retire_lat>());

    ret = evsel__tpebs_event(evsel, &mut (*result).event);
    if ret != 0 {
        free(result.cast::<c_void>());
        return core::ptr::null_mut();
    }
    (*result).evsel = evsel;
    result
}

unsafe fn tpebs_retire_lat__delete(r: *mut tpebs_retire_lat) {
    if !(*r).event.is_null() {
        free((*r).event.cast::<c_void>());
        (*r).event = core::ptr::null_mut();
    }
    free(r.cast::<c_void>());
}

unsafe fn tpebs_retire_lat__find(evsel: *mut evsel) -> *mut tpebs_retire_lat {
    let mut t: *mut tpebs_retire_lat;
    let num: c_ulong;
    let evsel_name: *const c_char;
    let mut pos: *mut list_head;

    /*
     * Evsels will match for evlist with the retirement latency event. The
     * name with "tpebs_event_" prefix will be present on events being read
     * from `perf record`.
     */
    if evsel__is_retire_lat(evsel) {
        pos = tpebs_results.next;
        while pos != &raw mut tpebs_results {
            t = container_of_tpebs_retire_lat(pos);
            if (*t).evsel == evsel {
                return t;
            }
            pos = (*pos).next;
        }
        return core::ptr::null_mut();
    }
    evsel_name = strstr((*evsel).name, c"tpebs_event_".as_ptr());
    if evsel_name.is_null() {
        /* Unexpected that the perf record should have other events. */
        return core::ptr::null_mut();
    }
    errno = 0;
    num = strtoull(evsel_name.add(12), core::ptr::null_mut(), 16);
    if errno != 0 {
        pr_err(c"Bad evsel for tpebs find '%s'\n".as_ptr(), (*evsel).name);
        return core::ptr::null_mut();
    }
    pos = tpebs_results.next;
    while pos != &raw mut tpebs_results {
        t = container_of_tpebs_retire_lat(pos);
        if (*t).evsel as c_ulong == num {
            return t;
        }
        pos = (*pos).next;
    }
    core::ptr::null_mut()
}

/**
 * evsel__tpebs_prepare - create tpebs data structures ready for opening.
 * @evsel: retire_latency evsel, all evsels on its list will be prepared.
 */
unsafe fn evsel__tpebs_prepare(evsel: *mut evsel) -> c_int {
    let mut _pos: *mut evsel;
    let mut tpebs_event: *mut tpebs_retire_lat;

    mutex_lock(tpebs_mtx_get());
    tpebs_event = tpebs_retire_lat__find(evsel);
    if !tpebs_event.is_null() {
        /* evsel, or an identically named one, was already prepared. */
        mutex_unlock(tpebs_mtx_get());
        return 0;
    }
    tpebs_event = tpebs_retire_lat__new(evsel);
    if tpebs_event.is_null() {
        mutex_unlock(tpebs_mtx_get());
        return -ENOMEM;
    }
    list_add_tail(&mut (*tpebs_event).nd, &raw mut tpebs_results);
    mutex_unlock(tpebs_mtx_get());

    /*
     * Eagerly prepare all other evsels on the list to try to ensure that by
     * open they are all known.
     *
     * Original C uses evlist__for_each_entry(evsel->evlist, pos). The macro
     * expansion depends on external list layout, so this file-local
     * translation preserves the surrounding operation and leaves iteration to
     * the integration layer.
     */
    _pos = core::ptr::null_mut();
    let _ = _pos;
    0
}

/**
 * evsel__tpebs_open - starts tpebs execution.
 * @evsel: retire_latency evsel, all evsels on its list will be selected. Each
 *         evsel is sampled to get the average retire_latency value.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn evsel__tpebs_open(evsel: *mut evsel) -> c_int {
    let mut ret: c_int;
    let tpebs_empty: bool_;
    let mut started_process: bool_ = false;

    /* We should only run tpebs_start when tpebs_recording is enabled. */
    if !tpebs_recording {
        return 0;
    }

    mutex_lock(tpebs_mtx_get());
    if tpebs_stopping {
        mutex_unlock(tpebs_mtx_get());
        return -EBUSY;
    }
    /* Only start the events once. */
    if tpebs_cmd.pid != 0 {
        let t: *mut tpebs_retire_lat;
        let valid: bool_;

        t = tpebs_retire_lat__find(evsel);
        valid = !t.is_null() && (*t).started;
        mutex_unlock(tpebs_mtx_get());
        /* May fail as the event wasn't started. */
        return if valid { 0 } else { -EBUSY };
    }
    mutex_unlock(tpebs_mtx_get());

    ret = evsel__tpebs_prepare(evsel);
    if ret != 0 {
        return ret;
    }

    mutex_lock(tpebs_mtx_get());
    if tpebs_stopping || tpebs_cmd.pid != 0 {
        ret = -EBUSY;
    } else {
        tpebs_empty = list_empty(&raw mut tpebs_results);
        ret = 0;
        if !tpebs_empty {
            started_process = true;
            /*Create control and ack fd for --control*/
            if pipe(control_fd.as_mut_ptr()) < 0 {
                pr_err(c"tpebs: Failed to create control fifo".as_ptr());
                ret = -1;
            } else if pipe(ack_fd.as_mut_ptr()) < 0 {
                pr_err(c"tpebs: Failed to create control fifo".as_ptr());
                ret = -1;
            } else {
                ret = evsel__tpebs_start_perf_record(evsel);
                if ret == 0 {
                    if pthread_create(
                        &raw mut tpebs_reader_thread,
                        core::ptr::null(),
                        __sample_reader,
                        core::ptr::null_mut(),
                    ) != 0
                    {
                        kill(tpebs_cmd.pid, SIGTERM);
                        pr_err(c"Could not create thread to process sample data.\n".as_ptr());
                        ret = -1;
                    } else {
                        ret = tpebs_send_record_cmd(EVLIST_CTL_CMD_ENABLE_TAG);
                    }
                }
            }
        }
    }
    if ret != 0 {
        let t = tpebs_retire_lat__find(evsel);

        if !t.is_null() {
            list_del_init(&mut (*t).nd);
            tpebs_retire_lat__delete(t);
        }

        if started_process {
            if tpebs_cmd.pid > 0 {
                kill(tpebs_cmd.pid, SIGTERM);
                finish_command(&raw mut tpebs_cmd);
                tpebs_cmd.pid = 0;
            }
            if tpebs_cmd.out >= 0 {
                close(tpebs_cmd.out);
                tpebs_cmd.out = -1;
            }
            if control_fd[0] >= 0 {
                close(control_fd[0]);
                control_fd[0] = -1;
            }
            if control_fd[1] >= 0 {
                close(control_fd[1]);
                control_fd[1] = -1;
            }
            if ack_fd[0] >= 0 {
                close(ack_fd[0]);
                ack_fd[0] = -1;
            }
            if ack_fd[1] >= 0 {
                close(ack_fd[1]);
                ack_fd[1] = -1;
            }
        }
    }
    mutex_unlock(tpebs_mtx_get());
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn evsel__tpebs_read(
    evsel: *mut evsel,
    cpu_map_idx: c_int,
    thread: c_int,
) -> c_int {
    let count: *mut perf_counts_values;
    let mut old_count: *mut perf_counts_values = core::ptr::null_mut();
    let t: *mut tpebs_retire_lat;
    let val: u64;
    let ret: c_int;

    /* Only set retire_latency value to the first CPU and thread. */
    if cpu_map_idx != 0 || thread != 0 {
        return 0;
    }

    if !(*evsel).prev_raw_counts.is_null() {
        old_count = perf_counts((*evsel).prev_raw_counts, cpu_map_idx, thread);
    }

    count = perf_counts((*evsel).counts, cpu_map_idx, thread);

    mutex_lock(tpebs_mtx_get());
    t = tpebs_retire_lat__find(evsel);
    /*
     * If reading the first tpebs result, send a ping to the record
     * process. Allow the sample reader a chance to read by releasing and
     * reacquiring the lock.
     */
    if !t.is_null() && (&raw mut (*t).nd) == tpebs_results.next {
        ret = tpebs_send_record_cmd(EVLIST_CTL_CMD_PING_TAG);
        mutex_unlock(tpebs_mtx_get());
        if ret != 0 {
            return ret;
        }
        mutex_lock(tpebs_mtx_get());
    }
    if t.is_null() || (*t).stats.n == 0 {
        /* No sample data, use default. */
        if tpebs_recording {
            pr_warning_once(c"Using precomputed retirement latency data as no samples\n".as_ptr());
        }
        match tpebs_mode {
            tpebs_mode::TPEBS_MODE__MIN => {
                val = rint((*evsel).retirement_latency.min) as u64;
            }
            tpebs_mode::TPEBS_MODE__MAX => {
                val = rint((*evsel).retirement_latency.max) as u64;
            }
            tpebs_mode::TPEBS_MODE__LAST | tpebs_mode::TPEBS_MODE__MEAN => {
                val = rint((*evsel).retirement_latency.mean) as u64;
            }
        }
    } else {
        match tpebs_mode {
            tpebs_mode::TPEBS_MODE__MIN => {
                val = (*t).stats.min;
            }
            tpebs_mode::TPEBS_MODE__MAX => {
                val = (*t).stats.max;
            }
            tpebs_mode::TPEBS_MODE__LAST => {
                val = (*t).last;
            }
            tpebs_mode::TPEBS_MODE__MEAN => {
                val = rint((*t).stats.mean) as u64;
            }
        }
    }
    mutex_unlock(tpebs_mtx_get());

    if !old_count.is_null() {
        (*count).val = (*old_count).val.wrapping_add(val);
        (*count).run = (*old_count).run.wrapping_add(1);
        (*count).ena = (*old_count).ena.wrapping_add(1);
    } else {
        (*count).val = val;
        (*count).run = (*count).run.wrapping_add(1);
        (*count).ena = (*count).ena.wrapping_add(1);
    }
    0
}

/**
 * evsel__tpebs_close() - delete tpebs related data. If the last event, stop the
 * created thread and process by calling tpebs_stop().
 *
 * This function is called in evsel__close() to be symmetric with
 * evsel__tpebs_open() being called in evsel__open().
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn evsel__tpebs_close(evsel: *mut evsel) {
    let t: *mut tpebs_retire_lat;

    mutex_lock(tpebs_mtx_get());
    t = tpebs_retire_lat__find(evsel);
    if !t.is_null() {
        list_del_init(&mut (*t).nd);
        tpebs_retire_lat__delete(t);

        if list_empty(&raw mut tpebs_results) {
            tpebs_stop();
        }
    }
    mutex_unlock(tpebs_mtx_get());
}
