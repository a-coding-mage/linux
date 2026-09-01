/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A demo sched_ext user space scheduler which provides vruntime semantics
 * using a simple ordered-list implementation.
 *
 * Each CPU in the system resides in a single, global domain. This precludes
 * the need to do any load balancing between domains. The scheduler could
 * easily be extended to support multiple domains, with load balancing
 * happening in user space.
 *
 * Any task which has any CPU affinity is scheduled entirely in BPF. This
 * program only schedules tasks which may run on any CPU.
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type __u32 = u32;
type __s32 = i32;
type __u64 = u64;
type size_t = usize;
type pid_t = c_int;
type pthread_t = c_ulong;
type va_list = *mut c_void;

const NULL: *mut c_void = ptr::null_mut();
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const MCL_CURRENT: c_int = 1;
const MCL_FUTURE: c_int = 2;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const __ATOMIC_RELAXED: c_int = 0;
const LIBBPF_DEBUG: libbpf_print_level = 0;
const __NR_sched_setscheduler: c_long = 144;

const HELP_FMT: &[u8] =
    b"A minimal userland sched_ext scheduler.\n\
\n\
See the top-level comment in .bpf.c for more details.\n\
\n\
Try to reduce `sysctl kernel.pid_max` if this program triggers OOMs.\n\
\n\
Usage: %s [-b BATCH] [-v]\n\
\n\
  -b BATCH      The number of tasks to batch when dispatching (default: 8)\n\
  -v            Print libbpf debug messages\n\
  -h            Display this help and exit\n\0";

/* Defined in UAPI */
const SCHED_EXT: c_int = 7;

/* Number of tasks to batch when dispatching to user space. */
static mut batch_size: __u32 = 8;

static mut verbose: bool = false;
static mut exit_req: c_int = 0;
static mut stats_stop: c_int = 0;
static mut enqueued_fd: c_int = 0;
static mut dispatched_fd: c_int = 0;

static mut stats_printer: pthread_t = 0;
static mut skel: *mut scx_userland = ptr::null_mut();
static mut ops_link: *mut bpf_link = ptr::null_mut();

/* Stats collected in user space. */
static mut nr_vruntime_enqueues: __u64 = 0;
static mut nr_vruntime_dispatches: __u64 = 0;
static mut nr_vruntime_failed: __u64 = 0;

/* Number of tasks currently enqueued. */
static mut nr_curr_enqueued: __u64 = 0;

/*
 * The data structure containing tasks that are enqueued in user space.
 */
#[repr(C)]
pub struct enqueued_task {
    pub entries: list_entry,
    pub sum_exec_runtime: __u64,
    pub vruntime: f64,
}

#[repr(C)]
pub struct list_entry {
    pub le_next: *mut enqueued_task,
    pub le_prev: *mut *mut enqueued_task,
}

/*
 * Use a vruntime-sorted list to store tasks. This could easily be extended to
 * a more optimal data structure, such as an rbtree as is done in CFS. We
 * currently elect to use a sorted list to simplify the example for
 * illustrative purposes.
 */
#[repr(C)]
pub struct listhead {
    pub lh_first: *mut enqueued_task,
}

/*
 * A vruntime-sorted list of tasks. The head of the list contains the task with
 * the lowest vruntime. That is, the task that has the "highest" claim to be
 * scheduled.
 */
static mut vruntime_head: listhead = listhead {
    lh_first: ptr::null_mut(),
};

/*
 * The main array of tasks. The array is allocated all at once during
 * initialization, based on /proc/sys/kernel/pid_max, to avoid having to
 * dynamically allocate memory on the enqueue path, which could cause a
 * deadlock. A more substantive user space scheduler could e.g. provide a hook
 * for newly enabled tasks that are passed to the scheduler from the
 * .prep_enable() callback to allows the scheduler to allocate on safe paths.
 */
static mut tasks: *mut enqueued_task = ptr::null_mut();
static mut pid_max: c_int = 0;

static mut min_vruntime: f64 = 0.0;

type libbpf_print_level = c_uint;

#[repr(C)]
pub struct FILE {
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
pub struct sched_param {
    pub sched_priority: c_int,
}

#[repr(C)]
pub struct scx_userland_enqueued_task {
    pub pid: __s32,
    pub weight: __u64,
    pub sum_exec_runtime: __u64,
}

#[repr(C)]
pub struct scx_userland_bss {
    pub nr_queued: __u64,
    pub nr_scheduled: __u64,
    pub nr_failed_enqueues: __u64,
    pub nr_kernel_enqueues: __u64,
    pub nr_user_enqueues: __u64,
}

#[repr(C)]
pub struct scx_userland_rodata {
    pub num_possible_cpus: c_int,
    pub usersched_pid: pid_t,
}

#[repr(C)]
pub struct scx_userland_maps {
    pub enqueued: *mut bpf_map,
    pub dispatched: *mut bpf_map,
}

#[repr(C)]
pub struct scx_userland {
    pub bss: *mut scx_userland_bss,
    pub rodata: *mut scx_userland_rodata,
    pub maps: scx_userland_maps,
}

extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut optarg: *mut c_char;

    fn vfprintf(stream: *mut FILE, format: *const c_char, arg: va_list) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn exit(status: c_int) -> !;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn getpid() -> pid_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn sched_yield() -> c_int;
    fn sched_get_priority_max(policy: c_int) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn mlockall(flags: c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn libbpf_set_print(
        fn_: extern "C" fn(libbpf_print_level, *const c_char, va_list) -> c_int,
    );
    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map_lookup_and_delete_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn scx_userland__open() -> *mut scx_userland;
    fn scx_userland__load(skel: *mut scx_userland) -> c_int;
    fn scx_userland__attach(skel: *mut scx_userland) -> *mut bpf_link;
    fn scx_userland__destroy(skel: *mut scx_userland);
    fn uei_exited(skel: *mut scx_userland) -> bool;
    fn uei_report(skel: *mut scx_userland) -> __u64;
    fn uei_ecode_restart(ecode: __u64) -> bool;
}

unsafe fn atomic_add_fetch_u64(ptr_: *mut __u64, val: __u64) -> __u64 {
    *ptr_ = (*ptr_).wrapping_add(val);
    *ptr_
}

unsafe fn atomic_sub_fetch_u64(ptr_: *mut __u64, val: __u64) -> __u64 {
    *ptr_ = (*ptr_).wrapping_sub(val);
    *ptr_
}

unsafe fn atomic_load_u64(ptr_: *const __u64) -> __u64 {
    *ptr_
}

unsafe fn atomic_store_u64(ptr_: *mut __u64, val: __u64) {
    *ptr_ = val;
}

unsafe fn list_empty(head: *mut listhead) -> bool {
    (*head).lh_first.is_null()
}

unsafe fn list_first(head: *mut listhead) -> *mut enqueued_task {
    (*head).lh_first
}

unsafe fn list_init(head: *mut listhead) {
    (*head).lh_first = ptr::null_mut();
}

unsafe fn list_insert_head(head: *mut listhead, elm: *mut enqueued_task) {
    let first = (*head).lh_first;
    (*elm).entries.le_next = first;
    if !first.is_null() {
        (*first).entries.le_prev = &mut (*elm).entries.le_next;
    }
    (*head).lh_first = elm;
    (*elm).entries.le_prev = &mut (*head).lh_first;
}

unsafe fn list_insert_before(listelm: *mut enqueued_task, elm: *mut enqueued_task) {
    (*elm).entries.le_prev = (*listelm).entries.le_prev;
    (*elm).entries.le_next = listelm;
    *(*listelm).entries.le_prev = elm;
    (*listelm).entries.le_prev = &mut (*elm).entries.le_next;
}

unsafe fn list_insert_after(listelm: *mut enqueued_task, elm: *mut enqueued_task) {
    let next = (*listelm).entries.le_next;
    (*elm).entries.le_next = next;
    if !next.is_null() {
        (*next).entries.le_prev = &mut (*elm).entries.le_next;
    }
    (*listelm).entries.le_next = elm;
    (*elm).entries.le_prev = &mut (*listelm).entries.le_next;
}

unsafe fn list_remove(elm: *mut enqueued_task) {
    let next = (*elm).entries.le_next;
    if !next.is_null() {
        (*next).entries.le_prev = (*elm).entries.le_prev;
    }
    *(*elm).entries.le_prev = next;
}

unsafe fn scx_bug_on(err: c_int, msg: *const c_char) {
    if err != 0 {
        fprintf(stderr, b"%s\n\0".as_ptr() as *const c_char, msg);
        exit(1);
    }
}

extern "C" fn libbpf_print_fn(level: libbpf_print_level, format: *const c_char, args: va_list) -> c_int {
    unsafe {
        if level == LIBBPF_DEBUG && !verbose {
            return 0;
        }
        vfprintf(stderr, format, args)
    }
}

extern "C" fn sigint_handler(_userland: c_int) {
    unsafe {
        exit_req = 1;
    }
}

unsafe fn get_pid_max() -> c_int {
    let fp: *mut FILE;
    let mut pid_max_local: c_int = 0;

    fp = fopen(
        b"/proc/sys/kernel/pid_max\0".as_ptr() as *const c_char,
        b"r\0".as_ptr() as *const c_char,
    );
    if fp.is_null() {
        fprintf(
            stderr,
            b"Error opening /proc/sys/kernel/pid_max\n\0".as_ptr() as *const c_char,
        );
        return -1;
    }
    if fscanf(
        fp,
        b"%d\0".as_ptr() as *const c_char,
        &mut pid_max_local as *mut c_int,
    ) != 1
    {
        fprintf(
            stderr,
            b"Error reading from /proc/sys/kernel/pid_max\n\0".as_ptr() as *const c_char,
        );
        fclose(fp);
        return -1;
    }
    fclose(fp);

    pid_max_local
}

unsafe fn init_tasks() -> c_int {
    pid_max = get_pid_max();
    if pid_max < 0 {
        return pid_max;
    }

    tasks = calloc(pid_max as size_t, mem::size_of::<enqueued_task>()) as *mut enqueued_task;
    if tasks.is_null() {
        fprintf(stderr, b"Error allocating tasks array\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    0
}

unsafe fn task_pid(task: *const enqueued_task) -> __u32 {
    ((task as usize).wrapping_sub(tasks as usize) / mem::size_of::<enqueued_task>()) as __u32
}

unsafe fn dispatch_task(pid: __s32) -> c_int {
    let err: c_int;

    err = bpf_map_update_elem(
        dispatched_fd,
        ptr::null(),
        &pid as *const __s32 as *const c_void,
        0,
    );
    if err != 0 {
        atomic_add_fetch_u64(&mut nr_vruntime_failed, 1);
    } else {
        atomic_add_fetch_u64(&mut nr_vruntime_dispatches, 1);
    }

    err
}

unsafe fn get_enqueued_task(pid: __s32) -> *mut enqueued_task {
    if pid >= pid_max {
        return ptr::null_mut();
    }

    tasks.add(pid as usize)
}

unsafe fn calc_vruntime_delta(weight: __u64, delta: __u64) -> f64 {
    let weight_f: f64 = weight as f64 / 100.0;
    let delta_f: f64 = delta as f64;

    delta_f / weight_f
}

unsafe fn update_enqueued(enqueued: *mut enqueued_task, bpf_task: *const scx_userland_enqueued_task) {
    let delta: __u64;

    delta = (*bpf_task)
        .sum_exec_runtime
        .wrapping_sub((*enqueued).sum_exec_runtime);

    (*enqueued).vruntime += calc_vruntime_delta((*bpf_task).weight, delta);
    if min_vruntime > (*enqueued).vruntime {
        (*enqueued).vruntime = min_vruntime;
    }
    (*enqueued).sum_exec_runtime = (*bpf_task).sum_exec_runtime;
}

unsafe fn vruntime_enqueue(bpf_task: *const scx_userland_enqueued_task) -> c_int {
    let mut enqueued: *mut enqueued_task;
    let curr: *mut enqueued_task;
    let mut prev: *mut enqueued_task = ptr::null_mut();

    curr = get_enqueued_task((*bpf_task).pid);
    if curr.is_null() {
        return ENOENT;
    }

    update_enqueued(curr, bpf_task);
    atomic_add_fetch_u64(&mut nr_vruntime_enqueues, 1);
    atomic_add_fetch_u64(&mut nr_curr_enqueued, 1);

    /*
     * Enqueue the task in a vruntime-sorted list. A more optimal data
     * structure such as an rbtree could easily be used as well. We elect
     * to use a list here simply because it's less code, and thus the
     * example is less convoluted and better serves to illustrate what a
     * user space scheduler could look like.
     */

    if list_empty(&mut vruntime_head) {
        list_insert_head(&mut vruntime_head, curr);
        return 0;
    }

    enqueued = vruntime_head.lh_first;
    while !enqueued.is_null() {
        if (*curr).vruntime <= (*enqueued).vruntime {
            list_insert_before(enqueued, curr);
            return 0;
        }
        prev = enqueued;
        enqueued = (*enqueued).entries.le_next;
    }

    list_insert_after(prev, curr);

    0
}

unsafe fn drain_enqueued_map() {
    loop {
        let mut task: scx_userland_enqueued_task = mem::zeroed();
        let err: c_int;

        if bpf_map_lookup_and_delete_elem(
            enqueued_fd,
            ptr::null(),
            &mut task as *mut scx_userland_enqueued_task as *mut c_void,
        ) != 0
        {
            (*(*skel).bss).nr_queued = 0;
            (*(*skel).bss).nr_scheduled = nr_curr_enqueued;
            return;
        }

        err = vruntime_enqueue(&task);
        if err != 0 {
            fprintf(
                stderr,
                b"Failed to enqueue task %d: %s\n\0".as_ptr() as *const c_char,
                task.pid,
                strerror(err),
            );
            exit_req = 1;
            return;
        }
    }
}

unsafe fn dispatch_batch() {
    let mut i: __u32;

    i = 0;
    while i < batch_size {
        let task: *mut enqueued_task;
        let err: c_int;
        let pid: __s32;

        task = list_first(&mut vruntime_head);
        if task.is_null() {
            break;
        }

        min_vruntime = (*task).vruntime;
        pid = task_pid(task) as __s32;
        list_remove(task);
        err = dispatch_task(pid);
        if err != 0 {
            /*
             * If we fail to dispatch, put the task back to the
             * vruntime_head list and stop dispatching additional
             * tasks in this batch.
             */
            list_insert_head(&mut vruntime_head, task);
            break;
        }
        atomic_sub_fetch_u64(&mut nr_curr_enqueued, 1);
        i = i.wrapping_add(1);
    }
    (*(*skel).bss).nr_scheduled = atomic_load_u64(&nr_curr_enqueued);
}

extern "C" fn run_stats_printer(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        while stats_stop == 0 {
            let nr_failed_enqueues: __u64;
            let nr_kernel_enqueues: __u64;
            let nr_user_enqueues: __u64;
            let total: __u64;

            nr_failed_enqueues = (*(*skel).bss).nr_failed_enqueues;
            nr_kernel_enqueues = (*(*skel).bss).nr_kernel_enqueues;
            nr_user_enqueues = (*(*skel).bss).nr_user_enqueues;
            total = nr_failed_enqueues
                .wrapping_add(nr_kernel_enqueues)
                .wrapping_add(nr_user_enqueues);

            printf(b"o-----------------------o\n\0".as_ptr() as *const c_char);
            printf(b"| BPF ENQUEUES          |\n\0".as_ptr() as *const c_char);
            printf(b"|-----------------------|\n\0".as_ptr() as *const c_char);
            printf(
                b"|  kern:     %10llu |\n\0".as_ptr() as *const c_char,
                nr_kernel_enqueues,
            );
            printf(
                b"|  user:     %10llu |\n\0".as_ptr() as *const c_char,
                nr_user_enqueues,
            );
            printf(
                b"|  failed:   %10llu |\n\0".as_ptr() as *const c_char,
                nr_failed_enqueues,
            );
            printf(b"|  -------------------- |\n\0".as_ptr() as *const c_char);
            printf(b"|  total:    %10llu |\n\0".as_ptr() as *const c_char, total);
            printf(b"|                       |\n\0".as_ptr() as *const c_char);
            printf(b"|-----------------------|\n\0".as_ptr() as *const c_char);
            printf(b"| VRUNTIME / USER       |\n\0".as_ptr() as *const c_char);
            printf(b"|-----------------------|\n\0".as_ptr() as *const c_char);
            printf(
                b"|  enq:      %10llu |\n\0".as_ptr() as *const c_char,
                atomic_load_u64(&nr_vruntime_enqueues),
            );
            printf(
                b"|  disp:     %10llu |\n\0".as_ptr() as *const c_char,
                atomic_load_u64(&nr_vruntime_dispatches),
            );
            printf(
                b"|  failed:   %10llu |\n\0".as_ptr() as *const c_char,
                atomic_load_u64(&nr_vruntime_failed),
            );
            printf(b"o-----------------------o\n\0".as_ptr() as *const c_char);
            printf(b"\n\n\0".as_ptr() as *const c_char);
            fflush(stdout);
            sleep(1);
        }

        ptr::null_mut()
    }
}

unsafe fn spawn_stats_thread() -> c_int {
    pthread_create(
        &mut stats_printer,
        ptr::null(),
        run_stats_printer,
        ptr::null_mut(),
    )
}

unsafe fn pre_bootstrap(argc: c_int, argv: *mut *mut c_char) {
    let mut err: c_int;
    let mut opt: __s32;
    let sched_param = sched_param {
        sched_priority: sched_get_priority_max(SCHED_EXT),
    };

    err = init_tasks();
    if err != 0 {
        exit(err);
    }

    libbpf_set_print(libbpf_print_fn);
    signal(SIGINT, sigint_handler);
    signal(SIGTERM, sigint_handler);

    /*
     * Enforce that the user scheduler task is managed by sched_ext. The
     * task eagerly drains the list of enqueued tasks in its main work
     * loop, and then yields the CPU. The BPF scheduler only schedules the
     * user space scheduler task when at least one other task in the system
     * needs to be scheduled.
     */
    err = syscall(
        __NR_sched_setscheduler,
        getpid(),
        SCHED_EXT,
        &sched_param as *const sched_param,
    ) as c_int;
    scx_bug_on(
        err,
        b"Failed to set scheduler to SCHED_EXT\0".as_ptr() as *const c_char,
    );

    loop {
        opt = getopt(argc, argv, b"b:vh\0".as_ptr() as *const c_char);
        if opt == -1 {
            break;
        }
        match opt {
            x if x == b'b' as __s32 => {
                batch_size = strtoul(optarg, ptr::null_mut(), 0) as __u32;
            }
            x if x == b'v' as __s32 => {
                verbose = true;
            }
            _ => {
                fprintf(
                    stderr,
                    HELP_FMT.as_ptr() as *const c_char,
                    basename(*argv.add(0)),
                );
                exit((opt != b'h' as __s32) as c_int);
            }
        }
    }

    /*
     * It's not always safe to allocate in a user space scheduler, as an
     * enqueued task could hold a lock that we require in order to be able
     * to allocate.
     */
    err = mlockall(MCL_CURRENT | MCL_FUTURE);
    scx_bug_on(
        err,
        b"Failed to prefault and lock address space\0".as_ptr() as *const c_char,
    );
}

unsafe fn bootstrap(_comm: *mut c_char) {
    stats_stop = 0;
    min_vruntime = 0.0;
    atomic_store_u64(&mut nr_vruntime_enqueues, 0);
    atomic_store_u64(&mut nr_vruntime_dispatches, 0);
    atomic_store_u64(&mut nr_vruntime_failed, 0);
    atomic_store_u64(&mut nr_curr_enqueued, 0);
    memset(
        tasks as *mut c_void,
        0,
        pid_max as size_t * mem::size_of::<enqueued_task>(),
    );
    list_init(&mut vruntime_head);

    skel = scx_userland__open();

    (*(*skel).rodata).num_possible_cpus = libbpf_num_possible_cpus();
    assert!((*(*skel).rodata).num_possible_cpus > 0);
    (*(*skel).rodata).usersched_pid = getpid();
    assert!((*(*skel).rodata).usersched_pid > 0);

    scx_bug_on(scx_userland__load(skel), b"SCX_OPS_LOAD failed\0".as_ptr() as *const c_char);

    enqueued_fd = bpf_map__fd((*skel).maps.enqueued);
    dispatched_fd = bpf_map__fd((*skel).maps.dispatched);
    assert!(enqueued_fd > 0);
    assert!(dispatched_fd > 0);

    scx_bug_on(
        spawn_stats_thread(),
        b"Failed to spawn stats thread\0".as_ptr() as *const c_char,
    );

    ops_link = scx_userland__attach(skel);
}

unsafe fn sched_main_loop() {
    while exit_req == 0 && !uei_exited(skel) {
        /*
         * Perform the following work in the main user space scheduler
         * loop:
         *
         * 1. Drain all tasks from the enqueued map, and enqueue them
         *    to the vruntime sorted list.
         *
         * 2. Dispatch a batch of tasks from the vruntime sorted list
         *    down to the kernel.
         *
         * 3. Yield the CPU back to the system. The BPF scheduler will
         *    reschedule the user space scheduler once another task has
         *    been enqueued to user space.
         */
        drain_enqueued_map();
        dispatch_batch();
        sched_yield();
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ecode: __u64;

    pre_bootstrap(argc, argv);
    loop {
        bootstrap(*argv.add(0));
        sched_main_loop();

        stats_stop = 1;
        bpf_link__destroy(ops_link);
        pthread_join(stats_printer, ptr::null_mut());
        ecode = uei_report(skel);
        scx_userland__destroy(skel);

        if !(exit_req == 0 && uei_ecode_restart(ecode)) {
            break;
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
