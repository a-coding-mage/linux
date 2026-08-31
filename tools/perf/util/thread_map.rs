// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source: perf/util/thread_map.c

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

pub type pid_t = c_int;
pub type size_t = usize;

const INT_MIN: c_int = c_int::MIN;
const INT_MAX: c_int = c_int::MAX;
const NAME_MAX: usize = 255;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map_entry {
    pub pid: pid_t,
    pub comm: *mut c_char,
}

#[repr(C)]
pub struct perf_thread_map {
    pub refcnt: refcount_t,
    pub nr: c_int,
    pub map: [perf_thread_map_entry; 0],
}

#[repr(C)]
pub struct perf_record_thread_map_entry {
    pub pid: u64,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct perf_record_thread_map {
    pub nr: c_uint,
    pub entries: [perf_record_thread_map_entry; 0],
}

#[repr(C)]
pub struct str_node {
    pub rb_node: [usize; 3],
    pub s: *const c_char,
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn atoi(nptr: *const c_char) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn filename__read_str(filename: *const c_char, buf: *mut *mut c_char, size: *mut size_t) -> c_int;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn perf_thread_map__new_dummy() -> *mut perf_thread_map;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, idx: c_int) -> pid_t;
    fn perf_thread_map__realloc(threads: *mut perf_thread_map, nr: c_int) -> *mut perf_thread_map;
    fn perf_thread_map__set_pid(threads: *mut perf_thread_map, idx: c_int, pid: pid_t);
    fn procfs__mountpoint() -> *const c_char;
    fn pr_debug(format: *const c_char, ...);
    fn pr_warning(format: *const c_char, ...);
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn refcount_set(r: *mut refcount_t, n: c_int);
    fn scandir(
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int>,
    ) -> c_int;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn sprintf(str_: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strim(s: *mut c_char) -> *mut c_char;
    fn strlist__delete(slist: *mut strlist);
    fn strlist__new(s: *const c_char, dupstr: *const c_void) -> *mut strlist;
    fn strlist__next(slist: *mut strlist, pos: *mut str_node) -> *mut str_node;
    fn strlist__first(slist: *mut strlist) -> *mut str_node;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
}

unsafe fn zfree<T>(ptr: *mut *mut T) {
    if !(*ptr).is_null() {
        free(*ptr as *mut c_void);
        *ptr = core::ptr::null_mut();
    }
}

unsafe fn map_entry(map: *mut perf_thread_map, idx: c_int) -> *mut perf_thread_map_entry {
    (*map).map.as_mut_ptr().add(idx as usize)
}

/* Skip "." and ".." directories */
unsafe extern "C" fn filter(dir: *const dirent) -> c_int {
    if (*dir).d_name[0] == b'.' as c_char {
        0
    } else {
        1
    }
}

unsafe fn thread_map__alloc(__nr: c_int) -> *mut perf_thread_map {
    perf_thread_map__realloc(core::ptr::null_mut(), __nr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__new_by_pid(pid: pid_t) -> *mut perf_thread_map {
    let mut threads: *mut perf_thread_map;
    let mut name = [0 as c_char; 256];
    let items: c_int;
    let mut namelist: *mut *mut dirent = core::ptr::null_mut();
    let mut i: c_int;

    sprintf(name.as_mut_ptr(), c"/proc/%d/task".as_ptr(), pid);
    items = scandir(name.as_ptr(), &mut namelist, Some(filter), None);
    if items <= 0 {
        return core::ptr::null_mut();
    }

    threads = thread_map__alloc(items);
    if !threads.is_null() {
        i = 0;
        while i < items {
            perf_thread_map__set_pid(threads, i, atoi((**namelist.add(i as usize)).d_name.as_ptr()));
            i += 1;
        }
        (*threads).nr = items;
        refcount_set(&mut (*threads).refcnt, 1);
    }

    i = 0;
    while i < items {
        zfree(namelist.add(i as usize));
        i += 1;
    }
    free(namelist as *mut c_void);

    threads
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__new_by_tid(tid: pid_t) -> *mut perf_thread_map {
    let threads = thread_map__alloc(1);

    if !threads.is_null() {
        perf_thread_map__set_pid(threads, 0, tid);
        (*threads).nr = 1;
        refcount_set(&mut (*threads).refcnt, 1);
    }

    threads
}

unsafe fn thread_map__new_all_cpus() -> *mut perf_thread_map {
    let mut proc_: *mut DIR;
    let mut max_threads: c_int = 32;
    let mut items: c_int;
    let mut i: c_int;
    let mut path = [0 as c_char; NAME_MAX + 1 + 6];
    let mut dirent_: *mut dirent;
    let mut namelist: *mut *mut dirent = core::ptr::null_mut();
    let mut threads = thread_map__alloc(max_threads);

    if threads.is_null() {
        return threads;
    }

    proc_ = opendir(c"/proc".as_ptr());
    if proc_.is_null() {
        free(threads as *mut c_void);
        return core::ptr::null_mut();
    }

    (*threads).nr = 0;
    refcount_set(&mut (*threads).refcnt, 1);

    loop {
        dirent_ = readdir(proc_);
        if dirent_.is_null() {
            break;
        }

        let mut end: *mut c_char = core::ptr::null_mut();
        let mut grow = false;
        let pid = strtol((*dirent_).d_name.as_ptr(), &mut end, 10) as pid_t;

        if *end != 0 {
            continue;
        }

        snprintf(path.as_mut_ptr(), path.len(), c"/proc/%d/task".as_ptr(), pid);
        items = scandir(path.as_ptr(), &mut namelist, Some(filter), None);
        if items <= 0 {
            pr_debug(c"scandir for %d returned empty, skipping\n".as_ptr(), pid);
            continue;
        }
        while (*threads).nr + items >= max_threads {
            max_threads *= 2;
            grow = true;
        }

        if grow {
            let tmp = perf_thread_map__realloc(threads, max_threads);
            if tmp.is_null() {
                i = 0;
                while i < items {
                    zfree(namelist.add(i as usize));
                    i += 1;
                }
                free(namelist as *mut c_void);
                zfree(&mut threads);
                closedir(proc_);
                return threads;
            }

            threads = tmp;
        }

        i = 0;
        while i < items {
            perf_thread_map__set_pid(
                threads,
                (*threads).nr + i,
                atoi((**namelist.add(i as usize)).d_name.as_ptr()),
            );
            i += 1;
        }

        i = 0;
        while i < items {
            zfree(namelist.add(i as usize));
            i += 1;
        }
        free(namelist as *mut c_void);

        (*threads).nr += items;
    }

    closedir(proc_);
    threads
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__new(pid: pid_t, tid: pid_t) -> *mut perf_thread_map {
    if pid != -1 {
        return thread_map__new_by_pid(pid);
    }

    thread_map__new_by_tid(tid)
}

unsafe fn thread_map__new_by_pid_str(pid_str: *const c_char) -> *mut perf_thread_map {
    let mut threads: *mut perf_thread_map = core::ptr::null_mut();
    let mut nt: *mut perf_thread_map;
    let mut name = [0 as c_char; 256];
    let mut items: c_int = 0;
    let mut total_tasks: c_int = 0;
    let mut namelist: *mut *mut dirent = core::ptr::null_mut();
    let mut i: c_int;
    let mut j: c_int = 0;
    let mut pid: pid_t;
    let mut prev_pid: pid_t = INT_MAX;
    let slist = strlist__new(pid_str, core::ptr::null());

    if slist.is_null() {
        return core::ptr::null_mut();
    }

    let mut pos = strlist__first(slist);
    while !pos.is_null() {
        pid = strtol((*pos).s, core::ptr::null_mut(), 10) as pid_t;

        if pid == INT_MIN || pid == INT_MAX {
            zfree(&mut threads);
            break;
        }

        if pid == prev_pid {
            pos = strlist__next(slist, pos);
            continue;
        }

        sprintf(name.as_mut_ptr(), c"/proc/%d/task".as_ptr(), pid);
        items = scandir(name.as_ptr(), &mut namelist, Some(filter), None);
        if items <= 0 {
            zfree(&mut threads);
            break;
        }

        total_tasks += items;
        nt = perf_thread_map__realloc(threads, total_tasks);
        if nt.is_null() {
            i = 0;
            while i < items {
                zfree(namelist.add(i as usize));
                i += 1;
            }
            free(namelist as *mut c_void);
            zfree(&mut threads);
            break;
        }

        threads = nt;

        i = 0;
        while i < items {
            perf_thread_map__set_pid(threads, j, atoi((**namelist.add(i as usize)).d_name.as_ptr()));
            j += 1;
            zfree(namelist.add(i as usize));
            i += 1;
        }
        (*threads).nr = total_tasks;
        free(namelist as *mut c_void);

        pos = strlist__next(slist, pos);
    }

    strlist__delete(slist);
    if !threads.is_null() {
        refcount_set(&mut (*threads).refcnt, 1);
    }
    threads
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__new_by_tid_str(tid_str: *const c_char) -> *mut perf_thread_map {
    let mut threads: *mut perf_thread_map = core::ptr::null_mut();
    let mut nt: *mut perf_thread_map;
    let mut ntasks: c_int = 0;
    let mut tid: pid_t;
    let mut prev_tid: pid_t = INT_MAX;

    /* perf-stat expects threads to be generated even if tid not given */
    if tid_str.is_null() {
        return perf_thread_map__new_dummy();
    }

    let slist = strlist__new(tid_str, core::ptr::null());
    if slist.is_null() {
        return core::ptr::null_mut();
    }

    let mut pos = strlist__first(slist);
    while !pos.is_null() {
        tid = strtol((*pos).s, core::ptr::null_mut(), 10) as pid_t;

        if tid == INT_MIN || tid == INT_MAX {
            zfree(&mut threads);
            break;
        }

        if tid == prev_tid {
            pos = strlist__next(slist, pos);
            continue;
        }

        ntasks += 1;
        nt = perf_thread_map__realloc(threads, ntasks);

        if nt.is_null() {
            zfree(&mut threads);
            break;
        }

        threads = nt;
        perf_thread_map__set_pid(threads, ntasks - 1, tid);
        (*threads).nr = ntasks;

        pos = strlist__next(slist, pos);
    }

    strlist__delete(slist);
    if !threads.is_null() {
        refcount_set(&mut (*threads).refcnt, 1);
    }
    threads
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__new_str(
    pid: *const c_char,
    tid: *const c_char,
    all_threads: bool,
) -> *mut perf_thread_map {
    if !pid.is_null() {
        return thread_map__new_by_pid_str(pid);
    }

    if all_threads {
        return thread_map__new_all_cpus();
    }

    thread_map__new_by_tid_str(tid)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__fprintf(threads: *mut perf_thread_map, fp: *mut FILE) -> size_t {
    let mut i: c_int;
    let mut printed = fprintf(
        fp,
        c"%d thread%s: ".as_ptr(),
        (*threads).nr,
        if (*threads).nr > 1 { c"s".as_ptr() } else { c"".as_ptr() },
    ) as size_t;
    i = 0;
    while i < (*threads).nr {
        printed += fprintf(
            fp,
            c"%s%d".as_ptr(),
            if i != 0 { c", ".as_ptr() } else { c"".as_ptr() },
            perf_thread_map__pid(threads, i),
        ) as size_t;
        i += 1;
    }

    printed + fprintf(fp, c"\n".as_ptr()) as size_t
}

unsafe fn get_comm(comm: *mut *mut c_char, pid: pid_t) -> c_int {
    let mut path: *mut c_char = core::ptr::null_mut();
    let mut size: size_t = 0;
    let err: c_int;

    if asprintf(&mut path, c"%s/%d/comm".as_ptr(), procfs__mountpoint(), pid) == -1 {
        return -ENOMEM;
    }

    err = filename__read_str(path, comm, &mut size);
    if err == 0 {
        /*
         * We're reading 16 bytes, while filename__read_str
         * allocates data per BUFSIZ bytes, so we can safely
         * mark the end of the string.
         */
        *(*comm).add(size) = 0;
        strim(*comm);
    }

    free(path as *mut c_void);
    err
}

unsafe fn comm_init(map: *mut perf_thread_map, i: c_int) {
    let pid = perf_thread_map__pid(map, i);
    let mut comm: *mut c_char = core::ptr::null_mut();

    /* dummy pid comm initialization */
    if pid == -1 {
        (*map_entry(map, i)).comm = strdup(c"dummy".as_ptr());
        return;
    }

    /*
     * The comm name is like extra bonus ;-),
     * so just warn if we fail for any reason.
     */
    if get_comm(&mut comm, pid) != 0 {
        pr_warning(c"Couldn't resolve comm name for pid %d\n".as_ptr(), pid);
    }

    (*map_entry(map, i)).comm = comm;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__read_comms(threads: *mut perf_thread_map) {
    let mut i: c_int = 0;

    while i < (*threads).nr {
        comm_init(threads, i);
        i += 1;
    }
}

unsafe fn thread_map__copy_event(
    threads: *mut perf_thread_map,
    event: *mut perf_record_thread_map,
) {
    let mut i: c_uint;

    (*threads).nr = (*event).nr as c_int;

    i = 0;
    while i < (*event).nr {
        let entry = (*event).entries.as_ptr().add(i as usize);
        perf_thread_map__set_pid(threads, i as c_int, (*entry).pid as pid_t);
        (*map_entry(threads, i as c_int)).comm = strndup((*entry).comm.as_ptr(), 16);
        i += 1;
    }

    refcount_set(&mut (*threads).refcnt, 1);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__new_event(
    event: *mut perf_record_thread_map,
) -> *mut perf_thread_map {
    let threads: *mut perf_thread_map;

    threads = thread_map__alloc((*event).nr as c_int);
    if !threads.is_null() {
        thread_map__copy_event(threads, event);
    }

    threads
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__has(threads: *mut perf_thread_map, pid: pid_t) -> bool {
    let mut i: c_int = 0;

    while i < (*threads).nr {
        if (*map_entry(threads, i)).pid == pid {
            return true;
        }
        i += 1;
    }

    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_map__remove(threads: *mut perf_thread_map, idx: c_int) -> c_int {
    let mut i: c_int;

    if (*threads).nr < 1 {
        return -EINVAL;
    }

    if idx >= (*threads).nr {
        return -EINVAL;
    }

    /*
     * Free the 'idx' item and shift the rest up.
     */
    zfree(&mut (*map_entry(threads, idx)).comm);

    i = idx;
    while i < (*threads).nr - 1 {
        *map_entry(threads, i) = core::ptr::read(map_entry(threads, i + 1));
        i += 1;
    }

    (*threads).nr -= 1;
    0
}
