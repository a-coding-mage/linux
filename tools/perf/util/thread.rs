// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/thread.c.
// C includes were intentionally not translated; referenced symbols are external
// dependencies supplied by the surrounding perf sources.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type pid_t = c_int;
type size_t = usize;
type u8 = u8;
type u64 = u64;
type uint16_t = u16;
type uint32_t = u32;

const EM_NONE: uint16_t = 0;
const EM_HOST: uint16_t = 0xffff;
const EF_HOST: uint32_t = 0;
const O_RDONLY: c_int = 0;
const ENOMEM: c_int = 12;
const PERF_RECORD_MISC_USER: u8 = 0;
const PERF_RECORD_MISC_KERNEL: u8 = 1;
const PERF_RECORD_MISC_GUEST_USER: u8 = 2;
const PERF_RECORD_MISC_GUEST_KERNEL: u8 = 3;
const DSO_DATA_STATUS_ERROR: c_int = -1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    pub env: *mut perf_env,
    pub machines: *mut machines,
}

#[repr(C)]
pub struct machines {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    pub machines: machines,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso_data {
    pub status: c_int,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct namespaces {
    pub list: list_head,
    pub end_time: u64,
}

#[repr(C)]
pub struct comm {
    pub list: list_head,
    pub exec: bool,
    pub start: u64,
}

#[repr(C)]
pub struct perf_record_namespaces {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_location {
    pub map: *mut map,
}

#[repr(C)]
pub struct map_symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct callchain_cursor_node {
    pub ms: map_symbol,
}

#[repr(C)]
pub struct stitch_cursor {
    pub ms: map_symbol,
}

#[repr(C)]
pub struct stitch_list {
    pub node: list_head,
    pub cursor: callchain_cursor_node,
}

#[repr(C)]
pub struct lbr_stitch {
    pub lists: list_head,
    pub free_lists: list_head,
    pub prev_lbr_cursor_size: c_uint,
    pub prev_lbr_cursor: *mut stitch_cursor,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

type ThreadPrivDestructor = Option<unsafe extern "C" fn(*mut c_void)>;

static mut thread__priv_destructor: ThreadPrivDestructor = None;

unsafe extern "C" {
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zfree(ptr: *mut *mut c_void);
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn maps__new(machine: *mut machine) -> *mut maps;
    fn maps__get(maps: *mut maps) -> *mut maps;
    fn maps__put(maps: *mut maps);
    fn maps__fprintf(maps: *mut maps, fp: *mut FILE) -> size_t;
    fn maps__fixup_overlap_and_insert(maps: *mut maps, map: *mut map) -> c_int;
    fn maps__equal(a: *mut maps, b: *mut maps) -> bool;
    fn maps__copy_from(dst: *mut maps, src: *mut maps) -> c_int;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn maps__for_each_map(
        maps: *mut maps,
        cb: unsafe extern "C" fn(*mut map, *mut c_void) -> c_int,
        args: *mut c_void,
    ) -> c_int;

    fn machine__findnew_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__find_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__kernel_ip(machine: *mut machine, ip: u64) -> bool;

    fn thread__pid(thread: *mut thread) -> pid_t;
    fn thread__tid(thread: *mut thread) -> pid_t;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread__set_maps(thread: *mut thread, maps: *mut maps);
    fn thread__set_pid(thread: *mut thread, pid: pid_t);
    fn thread__set_tid(thread: *mut thread, tid: pid_t);
    fn thread__set_ppid(thread: *mut thread, ppid: pid_t);
    fn thread__set_cpu(thread: *mut thread, cpu: c_int);
    fn thread__set_guest_cpu(thread: *mut thread, cpu: c_int);
    fn thread__set_e_machine(thread: *mut thread, e_machine: uint16_t);
    fn thread__e_machine(thread: *mut thread, machine: *mut machine, e_flags: *mut uint32_t) -> uint16_t;
    fn thread__set_e_flags(thread: *mut thread, e_flags: uint32_t);
    fn thread__e_flags(thread: *mut thread) -> uint32_t;
    fn thread__set_e_is_big_endian(thread: *mut thread, is_big_endian: bool);
    fn thread__e_is_big_endian(thread: *mut thread) -> bool;
    fn thread__set_lbr_stitch_enable(thread: *mut thread, enable: bool);
    fn thread__namespaces_list(thread: *mut thread) -> *mut list_head;
    fn thread__comm_list(thread: *mut thread) -> *mut list_head;
    fn thread__namespaces_lock(thread: *mut thread) -> *mut rw_semaphore;
    fn thread__comm_lock(thread: *mut thread) -> *mut rw_semaphore;
    fn thread__refcnt(thread: *mut thread) -> *mut refcount_t;
    fn thread__srccode_state(thread: *mut thread) -> *mut c_void;
    fn thread__priv(thread: *mut thread) -> *mut c_void;
    fn thread__comm_set(thread: *mut thread) -> bool;
    fn thread__set_comm_set(thread: *mut thread, set: bool);
    fn thread__set_comm(thread: *mut thread, str_: *const c_char, timestamp: u64) -> c_int;
    fn thread__set_comm_len(thread: *mut thread, len: size_t);
    fn thread__var_comm_len(thread: *mut thread) -> c_int;
    fn thread__find_symbol(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> *mut c_void;
    fn thread__find_map(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> *mut map;
    fn thread__lbr_stitch(thread: *mut thread) -> *mut lbr_stitch;
    fn thread__set_lbr_stitch(thread: *mut thread, stitch: *mut lbr_stitch);
    fn thread_stack__free(thread: *mut thread);

    fn comm__new(str_: *const c_char, timestamp: u64, exec: bool) -> *mut comm;
    fn comm__free(comm: *mut comm);
    fn comm__override(comm: *mut comm, str_: *const c_char, timestamp: u64, exec: bool) -> c_int;
    fn comm__str(comm: *const comm) -> *const c_char;

    fn namespaces__new(event: *mut perf_record_namespaces) -> *mut namespaces;
    fn namespaces__free(namespaces: *mut namespaces);
    fn nsinfo__new(pid: pid_t) -> *mut c_void;
    fn nsinfo__zput(nsinfo: *mut c_void);

    fn srccode_state_init(state: *mut c_void);
    fn srccode_state_free(state: *mut c_void);
    fn unwind__flush_access(maps: *mut maps);
    fn unwind__prepare_access(maps: *mut maps, e_machine: uint16_t) -> c_int;

    fn map__dso(map: *mut map) -> *mut dso;
    fn map__load(map: *mut map) -> c_int;
    fn map__map_ip(map: *mut map, ip: u64) -> c_long;
    fn map_symbol__exit(ms: *mut map_symbol);

    fn dso__read_e_machine_endian(
        optional_dso: *mut dso,
        fd: c_int,
        e_flags: *mut uint32_t,
        is_big_endian: *mut bool,
    ) -> uint16_t;
    fn dso__e_machine_endian(
        dso: *mut dso,
        machine: *mut machine,
        e_flags: *mut uint32_t,
        is_big_endian: *mut bool,
    ) -> uint16_t;
    fn dso__data(dso: *mut dso) -> *mut dso_data;
    fn dso__is_64_bit(dso: *mut dso) -> bool;
    fn dso__data_read_offset(
        dso: *mut dso,
        machine: *mut machine,
        offset: c_long,
        buf: *mut c_void,
        len: c_int,
    ) -> c_int;

    fn perf_env__arch(env: *mut perf_env) -> *const c_char;
    fn perf_arch_is_big_endian(arch: *const c_char) -> bool;
    fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut uint32_t) -> uint16_t;
    fn procfs__read_str(path: *const c_char, buf: *mut *mut c_char, sz: *mut size_t) -> c_int;

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn pr_debug(format: *const c_char, ...);

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn init_rwsem(sem: *mut rw_semaphore);
    fn exit_rwsem(sem: *mut rw_semaphore);
    fn refcount_set(r: *mut refcount_t, n: c_int);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;

    fn RC_CHK_ALLOC_THREAD(mem: *mut c_void) -> *mut thread;
    fn RC_CHK_ACCESS_THREAD(thread: *mut thread) -> *mut thread_rc_access;
    fn RC_CHK_FREE_THREAD(thread: *mut thread);
    fn RC_CHK_GET_THREAD(result: *mut *mut thread, thread: *mut thread) -> bool;
    fn RC_CHK_PUT_THREAD(thread: *mut thread);
}

#[repr(C)]
pub struct thread_rc_access {
    pub nsinfo: *mut c_void,
    pub e_machine: uint16_t,
}

unsafe fn container_of_perf_session(ptr: *mut machines) -> *mut perf_session {
    (ptr as *mut u8).sub(core::mem::offset_of!(perf_session, machines)) as *mut perf_session
}

unsafe fn list_first_entry_namespaces(head: *mut list_head) -> *mut namespaces {
    (*head).next as *mut namespaces
}

unsafe fn list_first_entry_comm(head: *mut list_head) -> *mut comm {
    (*head).next as *mut comm
}

unsafe fn list_next_entry_namespaces(pos: *mut namespaces) -> *mut namespaces {
    (*(*pos).list.next).next as *mut namespaces
}

unsafe fn list_entry_stitch_list(node: *mut list_head) -> *mut stitch_list {
    (node as *mut u8).sub(core::mem::offset_of!(stitch_list, node)) as *mut stitch_list
}

#[no_mangle]
pub unsafe extern "C" fn thread__init_maps(thread: *mut thread, machine: *mut machine) -> c_int {
    let pid = thread__pid(thread);

    if pid == thread__tid(thread) || pid == -1 {
        thread__set_maps(thread, maps__new(machine));
    } else {
        let leader = machine__findnew_thread(machine, pid, pid);

        if !leader.is_null() {
            thread__set_maps(thread, maps__get(thread__maps(leader)));
            thread__put(leader);
        }
    }

    if !thread__maps(thread).is_null() { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn thread__new(pid: pid_t, tid: pid_t) -> *mut thread {
    /* Allocation/creation is inherently single threaded. */
    let _thread = zalloc(0);
    let thread = RC_CHK_ALLOC_THREAD(_thread);

    if !thread.is_null() {
        let mut comm: *mut comm;
        let mut comm_str = [0 as c_char; 32];

        thread__set_pid(thread, pid);
        thread__set_tid(thread, tid);
        thread__set_ppid(thread, -1);
        thread__set_cpu(thread, -1);
        thread__set_guest_cpu(thread, -1);
        thread__set_e_machine(thread, EM_NONE);
        thread__set_e_is_big_endian(thread, false);
        thread__set_lbr_stitch_enable(thread, false);
        INIT_LIST_HEAD(thread__namespaces_list(thread));
        INIT_LIST_HEAD(thread__comm_list(thread));
        init_rwsem(thread__namespaces_lock(thread));
        init_rwsem(thread__comm_lock(thread));

        snprintf(
            comm_str.as_mut_ptr(),
            comm_str.len(),
            b":%d\0".as_ptr() as *const c_char,
            tid,
        );
        comm = comm__new(comm_str.as_ptr(), 0, false);
        if comm.is_null() {
            thread__delete(thread);
            return core::ptr::null_mut();
        }

        list_add(&mut (*comm).list, thread__comm_list(thread));
        refcount_set(thread__refcnt(thread), 1);
        /* Thread holds first ref to nsdata. */
        (*RC_CHK_ACCESS_THREAD(thread)).nsinfo = nsinfo__new(pid);
        srccode_state_init(thread__srccode_state(thread));
    }

    thread
}

#[no_mangle]
pub unsafe extern "C" fn thread__set_priv_destructor(destructor: ThreadPrivDestructor) {
    assert!(thread__priv_destructor.is_none());
    thread__priv_destructor = destructor;
}

#[no_mangle]
pub unsafe extern "C" fn thread__delete(thread: *mut thread) {
    thread_stack__free(thread);

    if !thread__maps(thread).is_null() {
        maps__put(thread__maps(thread));
        thread__set_maps(thread, core::ptr::null_mut());
    }

    down_write(thread__namespaces_lock(thread));
    {
        let head = thread__namespaces_list(thread);
        let mut pos = (*head).next;
        while pos != head {
            let next = (*pos).next;
            let namespaces = pos as *mut namespaces;
            list_del_init(&mut (*namespaces).list);
            namespaces__free(namespaces);
            pos = next;
        }
    }
    up_write(thread__namespaces_lock(thread));

    down_write(thread__comm_lock(thread));
    {
        let head = thread__comm_list(thread);
        let mut pos = (*head).next;
        while pos != head {
            let next = (*pos).next;
            let comm = pos as *mut comm;
            list_del_init(&mut (*comm).list);
            comm__free(comm);
            pos = next;
        }
    }
    up_write(thread__comm_lock(thread));

    nsinfo__zput((*RC_CHK_ACCESS_THREAD(thread)).nsinfo);
    srccode_state_free(thread__srccode_state(thread));

    exit_rwsem(thread__namespaces_lock(thread));
    exit_rwsem(thread__comm_lock(thread));
    thread__free_stitch_list(thread);

    if let Some(destructor) = thread__priv_destructor {
        destructor(thread__priv(thread));
    }

    RC_CHK_FREE_THREAD(thread);
}

#[no_mangle]
pub unsafe extern "C" fn thread__get(thread: *mut thread) -> *mut thread {
    let mut result: *mut thread = core::ptr::null_mut();

    if RC_CHK_GET_THREAD(&mut result, thread) {
        refcount_inc(thread__refcnt(thread));
    }

    result
}

#[no_mangle]
pub unsafe extern "C" fn thread__put(thread: *mut thread) {
    if !thread.is_null() && refcount_dec_and_test(thread__refcnt(thread)) {
        thread__delete(thread);
    } else {
        RC_CHK_PUT_THREAD(thread);
    }
}

unsafe extern "C" fn __thread__namespaces(thread: *mut thread) -> *mut namespaces {
    if list_empty(thread__namespaces_list(thread)) {
        return core::ptr::null_mut();
    }

    list_first_entry_namespaces(thread__namespaces_list(thread))
}

#[no_mangle]
pub unsafe extern "C" fn thread__namespaces(thread: *mut thread) -> *mut namespaces {
    let ns: *mut namespaces;

    down_read(thread__namespaces_lock(thread));
    ns = __thread__namespaces(thread);
    up_read(thread__namespaces_lock(thread));

    ns
}

unsafe extern "C" fn __thread__set_namespaces(
    thread: *mut thread,
    timestamp: u64,
    event: *mut perf_record_namespaces,
) -> c_int {
    let new = namespaces__new(event);
    let mut curr = __thread__namespaces(thread);

    if new.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*new).list, thread__namespaces_list(thread));

    if timestamp != 0 && !curr.is_null() {
        /*
         * setns syscall must have changed few or all the namespaces
         * of this thread. Update end time for the namespaces
         * previously used.
         */
        curr = list_next_entry_namespaces(new);
        (*curr).end_time = timestamp;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn thread__set_namespaces(
    thread: *mut thread,
    timestamp: u64,
    event: *mut perf_record_namespaces,
) -> c_int {
    let ret: c_int;

    down_write(thread__namespaces_lock(thread));
    ret = __thread__set_namespaces(thread, timestamp, event);
    up_write(thread__namespaces_lock(thread));
    ret
}

unsafe extern "C" fn __thread__comm(thread: *mut thread) -> *mut comm {
    if list_empty(thread__comm_list(thread)) {
        return core::ptr::null_mut();
    }

    list_first_entry_comm(thread__comm_list(thread))
}

#[no_mangle]
pub unsafe extern "C" fn thread__comm(thread: *mut thread) -> *mut comm {
    let mut res: *mut comm = core::ptr::null_mut();

    down_read(thread__comm_lock(thread));
    res = __thread__comm(thread);
    up_read(thread__comm_lock(thread));
    res
}

#[no_mangle]
pub unsafe extern "C" fn thread__exec_comm(thread: *mut thread) -> *mut comm {
    let mut last: *mut comm = core::ptr::null_mut();
    let mut second_last: *mut comm = core::ptr::null_mut();

    down_read(thread__comm_lock(thread));
    {
        let head = thread__comm_list(thread);
        let mut pos = (*head).next;
        while pos != head {
            let comm = pos as *mut comm;
            if (*comm).exec {
                up_read(thread__comm_lock(thread));
                return comm;
            }
            second_last = last;
            last = comm;
            pos = (*pos).next;
        }
    }
    up_read(thread__comm_lock(thread));

    /*
     * 'last' with no start time might be the parent's comm of a synthesized
     * thread (created by processing a synthesized fork event). For a main
     * thread, that is very probably wrong. Prefer a later comm to avoid
     * that case.
     */
    if !second_last.is_null() && (*last).start == 0 && thread__pid(thread) == thread__tid(thread) {
        return second_last;
    }

    last
}

unsafe extern "C" fn ____thread__set_comm(
    thread: *mut thread,
    str_: *const c_char,
    timestamp: u64,
    exec: bool,
) -> c_int {
    let mut new: *mut comm;
    let curr = __thread__comm(thread);

    /* Override the default :tid entry */
    if !thread__comm_set(thread) {
        let err = comm__override(curr, str_, timestamp, exec);
        if err != 0 {
            return err;
        }
    } else {
        new = comm__new(str_, timestamp, exec);
        if new.is_null() {
            return -ENOMEM;
        }
        list_add(&mut (*new).list, thread__comm_list(thread));

        if exec {
            unwind__flush_access(thread__maps(thread));
        }
    }

    thread__set_comm_set(thread, true);

    0
}

#[no_mangle]
pub unsafe extern "C" fn __thread__set_comm(
    thread: *mut thread,
    str_: *const c_char,
    timestamp: u64,
    exec: bool,
) -> c_int {
    let ret: c_int;

    down_write(thread__comm_lock(thread));
    ret = ____thread__set_comm(thread, str_, timestamp, exec);
    up_write(thread__comm_lock(thread));
    ret
}

#[no_mangle]
pub unsafe extern "C" fn thread__set_comm_from_proc(thread: *mut thread) -> c_int {
    let mut path = [0 as c_char; 64];
    let mut comm: *mut c_char = core::ptr::null_mut();
    let mut sz: size_t = 0;
    let mut err: c_int = -1;

    if !(snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%d/task/%d/comm\0".as_ptr() as *const c_char,
        thread__pid(thread),
        thread__tid(thread),
    ) >= path.len() as c_int)
        && procfs__read_str(path.as_ptr(), &mut comm, &mut sz) == 0
    {
        /* sz==0: read got nothing, e.g. race during exit teardown */
        if sz == 0 {
            free(comm as *mut c_void);
            return -1;
        }
        *comm.add(sz - 1) = 0;
        err = thread__set_comm(thread, comm, 0);
    }

    err
}

unsafe extern "C" fn __thread__comm_str(thread: *mut thread) -> *const c_char {
    let comm = __thread__comm(thread);

    if comm.is_null() {
        return core::ptr::null();
    }

    comm__str(comm)
}

#[no_mangle]
pub unsafe extern "C" fn thread__comm_str(thread: *mut thread) -> *const c_char {
    let str_: *const c_char;

    down_read(thread__comm_lock(thread));
    str_ = __thread__comm_str(thread);
    up_read(thread__comm_lock(thread));

    str_
}

unsafe extern "C" fn __thread__comm_len(thread: *mut thread, comm: *const c_char) -> c_int {
    if comm.is_null() {
        return 0;
    }
    thread__set_comm_len(thread, strlen(comm));

    thread__var_comm_len(thread)
}

/* CHECKME: it should probably better return the max comm len from its comm list */
#[no_mangle]
pub unsafe extern "C" fn thread__comm_len(thread: *mut thread) -> c_int {
    let mut comm_len = thread__var_comm_len(thread);

    if comm_len == 0 {
        let comm: *const c_char;

        down_read(thread__comm_lock(thread));
        comm = __thread__comm_str(thread);
        comm_len = __thread__comm_len(thread, comm);
        up_read(thread__comm_lock(thread));
    }

    comm_len
}

#[no_mangle]
pub unsafe extern "C" fn thread__fprintf(thread: *mut thread, fp: *mut FILE) -> size_t {
    fprintf(
        fp,
        b"Thread %d %s\n\0".as_ptr() as *const c_char,
        thread__tid(thread),
        thread__comm_str(thread),
    ) as size_t
        + maps__fprintf(thread__maps(thread), fp)
}

#[no_mangle]
pub unsafe extern "C" fn thread__insert_map(thread: *mut thread, map: *mut map) -> c_int {
    let ret: c_int;
    let e_machine: uint16_t;

    ret = maps__fixup_overlap_and_insert(thread__maps(thread), map);
    if ret != 0 {
        return ret;
    }

    e_machine = thread__e_machine(thread, core::ptr::null_mut(), core::ptr::null_mut());
    unwind__prepare_access(thread__maps(thread), e_machine)
}

unsafe extern "C" fn thread__prepare_access(thread: *mut thread) -> c_int {
    let e_machine = thread__e_machine(thread, core::ptr::null_mut(), core::ptr::null_mut());

    unwind__prepare_access(thread__maps(thread), e_machine)
}

unsafe extern "C" fn thread__clone_maps(
    thread: *mut thread,
    parent: *mut thread,
    do_maps_clone: bool,
) -> c_int {
    /* This is new thread, we share map groups for process. */
    if thread__pid(thread) == thread__pid(parent) {
        return thread__prepare_access(thread);
    }

    if maps__equal(thread__maps(thread), thread__maps(parent)) {
        pr_debug(
            b"broken map groups on thread %d/%d parent %d/%d\n\0".as_ptr() as *const c_char,
            thread__pid(thread),
            thread__tid(thread),
            thread__pid(parent),
            thread__tid(parent),
        );
        return 0;
    }
    /* But this one is new process, copy maps. */
    if do_maps_clone {
        maps__copy_from(thread__maps(thread), thread__maps(parent))
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn thread__fork(
    thread: *mut thread,
    parent: *mut thread,
    timestamp: u64,
    do_maps_clone: bool,
) -> c_int {
    if thread__comm_set(parent) {
        let comm = thread__comm_str(parent);
        let err: c_int;
        if comm.is_null() {
            return -ENOMEM;
        }
        err = thread__set_comm(thread, comm, timestamp);
        if err != 0 {
            return err;
        }
    }

    thread__set_ppid(thread, thread__tid(parent));
    thread__clone_maps(thread, parent, do_maps_clone)
}

#[no_mangle]
pub unsafe extern "C" fn thread__find_cpumode_addr_location(
    thread: *mut thread,
    addr: u64,
    symbols: bool,
    al: *mut addr_location,
) {
    let cpumodes: [u8; 4] = [
        PERF_RECORD_MISC_USER,
        PERF_RECORD_MISC_KERNEL,
        PERF_RECORD_MISC_GUEST_USER,
        PERF_RECORD_MISC_GUEST_KERNEL,
    ];

    for i in 0..cpumodes.len() {
        if symbols {
            thread__find_symbol(thread, cpumodes[i], addr, al);
        } else {
            thread__find_map(thread, cpumodes[i], addr, al);
        }

        if !(*al).map.is_null() {
            break;
        }
    }
}

unsafe extern "C" fn read_proc_e_machine_for_pid(
    pid: pid_t,
    e_flags: *mut uint32_t,
    is_big_endian: *mut bool,
) -> uint16_t {
    let mut path = [0 as c_char; 6 + 11 + 5];
    let fd: c_int;
    let mut e_machine: uint16_t = EM_NONE;

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"/proc/%d/exe\0".as_ptr() as *const c_char,
        pid,
    );
    fd = open(path.as_ptr(), O_RDONLY);
    if fd >= 0 {
        e_machine = dso__read_e_machine_endian(core::ptr::null_mut(), fd, e_flags, is_big_endian);
        close(fd);
    }
    e_machine
}

#[repr(C)]
struct thread__e_machine_callback_args {
    machine: *mut machine,
    e_flags: uint32_t,
    e_machine: uint16_t,
    is_big_endian: bool,
}

unsafe extern "C" fn thread__e_machine_callback(map: *mut map, _args: *mut c_void) -> c_int {
    let args = _args as *mut thread__e_machine_callback_args;
    let dso = map__dso(map);

    if dso.is_null() {
        return 0; // No dso, continue search.
    }

    (*args).e_machine = dso__e_machine_endian(
        dso,
        (*args).machine,
        &mut (*args).e_flags,
        &mut (*args).is_big_endian,
    );
    if (*args).e_machine != EM_NONE { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn thread__e_machine_endian(
    thread: *mut thread,
    mut machine: *mut machine,
    e_flags: *mut uint32_t,
    is_big_endian: *mut bool,
) -> uint16_t {
    let tid: pid_t;
    let pid: pid_t;
    let mut e_machine: uint16_t;
    let mut local_e_flags: uint32_t = 0;
    let mut args = thread__e_machine_callback_args {
        machine,
        e_flags: 0,
        e_machine: EM_NONE,
        is_big_endian: false,
    };

    if thread.is_null() {
        if !is_big_endian.is_null() {
            *is_big_endian = perf_arch_is_big_endian(if !machine.is_null() && !(*machine).env.is_null() {
                perf_env__arch((*machine).env)
            } else {
                core::ptr::null()
            });
        }
        return perf_env__e_machine(if !machine.is_null() { (*machine).env } else { core::ptr::null_mut() }, e_flags);
    }

    e_machine = (*RC_CHK_ACCESS_THREAD(thread)).e_machine;
    args.machine = machine;
    args.e_flags = 0;
    args.e_machine = EM_NONE;
    args.is_big_endian = false;

    if e_machine != EM_NONE {
        if !e_flags.is_null() {
            *e_flags = thread__e_flags(thread);
        }
        if !is_big_endian.is_null() {
            *is_big_endian = thread__e_is_big_endian(thread);
        }
        return e_machine;
    }

    if machine.is_null() {
        let maps = thread__maps(thread);

        machine = maps__machine(maps);
        args.machine = machine;
    }
    tid = thread__tid(thread);
    pid = thread__pid(thread);
    if pid != tid {
        let parent = machine__findnew_thread(machine, pid, pid);

        if !parent.is_null() {
            e_machine = thread__e_machine_endian(
                parent,
                machine,
                &mut local_e_flags,
                &mut args.is_big_endian,
            );
            thread__put(parent);
            e_machine = goto_out(
                thread,
                e_machine,
                &mut local_e_flags,
                &mut args,
                e_flags,
                is_big_endian,
            );
            return e_machine;
        }
        /* Something went wrong, fallback. */
    }
    /* Reading on the PID thread. First try to find from the maps. */
    maps__for_each_map(
        thread__maps(thread),
        thread__e_machine_callback,
        &mut args as *mut _ as *mut c_void,
    );

    if args.e_machine != EM_NONE {
        e_machine = args.e_machine;
        local_e_flags = args.e_flags;
    } else {
        /* Maps failed, perhaps we're live with map events disabled. */
        let mut is_live = (*machine).machines.is_null();

        if !is_live {
            /* Check if the session has a data file. */
            let session = container_of_perf_session((*machine).machines);

            is_live = !(*session).data.is_null();
        }
        /* Read from /proc/pid/exe if live. */
        if is_live {
            e_machine = read_proc_e_machine_for_pid(pid, &mut local_e_flags, &mut args.is_big_endian);
        } else if !machine.is_null() && !(*machine).env.is_null() {
            /* Offline analysis: fallback to environment metadata. */
            e_machine = perf_env__e_machine((*machine).env, &mut local_e_flags);
            args.is_big_endian = perf_arch_is_big_endian(perf_env__arch((*machine).env));
        }
    }

    e_machine = goto_out(
        thread,
        e_machine,
        &mut local_e_flags,
        &mut args,
        e_flags,
        is_big_endian,
    );
    e_machine
}

unsafe fn goto_out(
    thread: *mut thread,
    mut e_machine: uint16_t,
    local_e_flags: *mut uint32_t,
    args: *mut thread__e_machine_callback_args,
    e_flags: *mut uint32_t,
    is_big_endian: *mut bool,
) -> uint16_t {
    if e_machine != EM_NONE {
        thread__set_e_flags(thread, *local_e_flags);
        thread__set_e_is_big_endian(thread, (*args).is_big_endian);
        thread__set_e_machine(thread, e_machine);
        if !is_big_endian.is_null() {
            *is_big_endian = (*args).is_big_endian;
        }
    } else {
        e_machine = EM_HOST;
        *local_e_flags = EF_HOST;
        if !is_big_endian.is_null() {
            *is_big_endian = cfg!(target_endian = "big");
        }
    }
    if !e_flags.is_null() {
        *e_flags = *local_e_flags;
    }
    e_machine
}

#[no_mangle]
pub unsafe extern "C" fn thread__main_thread(
    machine: *mut machine,
    thread: *mut thread,
) -> *mut thread {
    if thread__pid(thread) == thread__tid(thread) {
        return thread__get(thread);
    }

    if thread__pid(thread) == -1 {
        return core::ptr::null_mut();
    }

    machine__find_thread(machine, thread__pid(thread), thread__pid(thread))
}

#[no_mangle]
pub unsafe extern "C" fn thread__memcpy(
    thread: *mut thread,
    machine: *mut machine,
    buf: *mut c_void,
    ip: u64,
    len: c_int,
    is64bit: *mut bool,
) -> c_int {
    let mut cpumode = PERF_RECORD_MISC_USER;
    let mut al: addr_location = core::mem::zeroed();
    let dso: *mut dso;
    let offset: c_long;

    if machine__kernel_ip(machine, ip) {
        cpumode = PERF_RECORD_MISC_KERNEL;
    }

    addr_location__init(&mut al);
    if thread__find_map(thread, cpumode, ip, &mut al).is_null() {
        addr_location__exit(&mut al);
        return -1;
    }

    dso = map__dso(al.map);

    if dso.is_null() || (*dso__data(dso)).status == DSO_DATA_STATUS_ERROR || map__load(al.map) < 0 {
        addr_location__exit(&mut al);
        return -1;
    }

    offset = map__map_ip(al.map, ip);
    if !is64bit.is_null() {
        *is64bit = dso__is_64_bit(dso);
    }

    addr_location__exit(&mut al);

    dso__data_read_offset(dso, machine, offset, buf, len)
}

#[no_mangle]
pub unsafe extern "C" fn thread__free_stitch_list(thread: *mut thread) {
    let lbr_stitch = thread__lbr_stitch(thread);

    if lbr_stitch.is_null() {
        return;
    }

    {
        let head = &mut (*lbr_stitch).lists as *mut list_head;
        let mut node = (*head).next;
        while node != head {
            let next = (*node).next;
            let pos = list_entry_stitch_list(node);
            map_symbol__exit(&mut (*pos).cursor.ms);
            list_del_init(&mut (*pos).node);
            free(pos as *mut c_void);
            node = next;
        }
    }

    {
        let head = &mut (*lbr_stitch).free_lists as *mut list_head;
        let mut node = (*head).next;
        while node != head {
            let next = (*node).next;
            let pos = list_entry_stitch_list(node);
            list_del_init(&mut (*pos).node);
            free(pos as *mut c_void);
            node = next;
        }
    }

    for i in 0..(*lbr_stitch).prev_lbr_cursor_size {
        map_symbol__exit(&mut (*(*lbr_stitch).prev_lbr_cursor.add(i as usize)).ms);
    }

    zfree(&mut (*lbr_stitch).prev_lbr_cursor as *mut *mut stitch_cursor as *mut *mut c_void);
    free(thread__lbr_stitch(thread) as *mut c_void);
    thread__set_lbr_stitch(thread, core::ptr::null_mut());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
