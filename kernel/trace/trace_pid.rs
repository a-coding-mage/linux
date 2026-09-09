// SPDX-License-Identifier: GPL-2.0

// Dependency declarations from trace.h are supplied by the surrounding translation unit.

/**
 * trace_find_filtered_pid - check if a pid exists in a filtered_pid list
 * @filtered_pids: The list of pids to check
 * @search_pid: The PID to find in @filtered_pids
 *
 * Returns true if @search_pid is found in @filtered_pids, and false otherwise.
 */
pub unsafe fn trace_find_filtered_pid(
    filtered_pids: *mut trace_pid_list,
    search_pid: pid_t,
) -> bool {
    trace_pid_list_is_set(filtered_pids, search_pid)
}

/**
 * trace_ignore_this_task - should a task be ignored for tracing
 * @filtered_pids: The list of pids to check
 * @filtered_no_pids: The list of pids not to be traced
 * @task: The task that should be ignored if not filtered
 *
 * Checks if @task should be traced or not from @filtered_pids.
 * Returns true if @task should *NOT* be traced.
 * Returns false if @task should be traced.
 */
pub unsafe fn trace_ignore_this_task(
    filtered_pids: *mut trace_pid_list,
    filtered_no_pids: *mut trace_pid_list,
    task: *mut task_struct,
) -> bool {
    ( !filtered_pids.is_null()
        && !trace_find_filtered_pid(filtered_pids, (*task).pid))
        || (!filtered_no_pids.is_null()
            && trace_find_filtered_pid(filtered_no_pids, (*task).pid))
}

/**
 * trace_filter_add_remove_task - Add or remove a task from a pid_list
 * @pid_list: The list to modify
 * @self: The current task for fork or NULL for exit
 * @task: The task to add or remove
 */
pub unsafe fn trace_filter_add_remove_task(
    pid_list: *mut trace_pid_list,
    self_: *mut task_struct,
    task: *mut task_struct,
) {
    if pid_list.is_null() {
        return;
    }

    /* For forks, we only add if the forking task is listed */
    if !self_.is_null() && !trace_find_filtered_pid(pid_list, (*self_).pid) {
        return;
    }

    /* "self" is set for forks, and NULL for exits */
    if !self_.is_null() {
        trace_pid_list_set(pid_list, (*task).pid);
    } else {
        trace_pid_list_clear(pid_list, (*task).pid);
    }
}

/**
 * trace_pid_next - Used for seq_file to get to the next pid of a pid_list
 */
pub unsafe fn trace_pid_next(
    pid_list: *mut trace_pid_list,
    v: *mut core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    let mut pid = v as usize as c_long;
    let mut next: c_uint = 0;

    *pos += 1;

    /* pid already is +1 of the actual previous bit */
    if trace_pid_list_next(pid_list, pid, &mut next) < 0 {
        return core::ptr::null_mut();
    }

    pid = next as c_long;

    /* Return pid + 1 to allow zero to be represented */
    (pid + 1) as usize as *mut core::ffi::c_void
}

/**
 * trace_pid_start - Used for seq_file to start reading pid lists
 */
pub unsafe fn trace_pid_start(
    pid_list: *mut trace_pid_list,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    let mut pid: usize;
    let mut first: c_uint = 0;
    let mut l: loff_t = 0;

    if trace_pid_list_first(pid_list, &mut first) < 0 {
        return core::ptr::null_mut();
    }

    pid = first as usize;

    /* Return pid + 1 so that zero can be the exit value */
    pid += 1;
    while pid != 0 && l < *pos {
        pid = trace_pid_next(pid_list, pid as *mut core::ffi::c_void, &mut l)
            as usize;
    }
    pid as *mut core::ffi::c_void
}

/**
 * trace_pid_show - show the current pid in seq_file processing
 */
pub unsafe fn trace_pid_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> c_int {
    let pid = v as usize - 1;

    seq_printf(m, "%lu\n", pid);
    0
}

/* 128 should be much more than enough */
pub const PID_BUF_SIZE: usize = 127;

pub unsafe fn trace_pid_write(
    filtered_pids: *mut trace_pid_list,
    new_pid_list: *mut *mut trace_pid_list,
    mut ubuf: *const c_char,
    mut cnt: usize,
) -> isize {
    let mut pid_list: *mut trace_pid_list;
    let mut parser: trace_parser;
    let mut val: c_ulong = 0;
    let mut nr_pids: c_int = 0;
    let mut read: isize = 0;
    let mut ret: isize;
    let mut pos: loff_t;
    let mut pid: pid_t;

    if trace_parser_get_init(&mut parser, PID_BUF_SIZE + 1) != 0 {
        return -ENOMEM as isize;
    }

    /* Always recreate a new array. The write is an all or nothing operation. */
    pid_list = trace_pid_list_alloc();
    if pid_list.is_null() {
        trace_parser_put(&mut parser);
        return -ENOMEM as isize;
    }

    if !filtered_pids.is_null() {
        ret = trace_pid_list_first(filtered_pids, &mut pid);
        while ret == 0 {
            ret = trace_pid_list_set(pid_list, pid);
            if ret < 0 {
                break;
            }
            ret = trace_pid_list_next(filtered_pids, pid + 1, &mut pid);
            nr_pids += 1;
        }
        if ret < 0 {
            trace_parser_put(&mut parser);
            trace_pid_list_free(pid_list);
            return ret;
        }
    }

    ret = 0;
    while cnt > 0 {
        pos = 0;
        ret = trace_get_user(&mut parser, ubuf, cnt, &mut pos);
        if ret < 0 { break; }
        read += ret;
        ubuf = ubuf.add(ret as usize);
        cnt -= ret as usize;
        if !trace_parser_loaded(&parser) { break; }
        ret = -EINVAL as isize;
        if kstrtoul(parser.buffer, 0, &mut val) != 0 { break; }
        pid = val as pid_t;
        if trace_pid_list_set(pid_list, pid) < 0 { ret = -1; break; }
        nr_pids += 1;
        trace_parser_clear(&mut parser);
        ret = 0;
    }

    trace_parser_put(&mut parser);
    if ret < 0 {
        trace_pid_list_free(pid_list);
        return ret;
    }
    if nr_pids == 0 {
        /* Cleared the list of pids */
        trace_pid_list_free(pid_list);
        pid_list = core::ptr::null_mut();
    }
    *new_pid_list = pid_list;
    read
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
