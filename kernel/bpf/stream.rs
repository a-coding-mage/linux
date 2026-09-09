// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// Translated from stream.c. Kernel declarations and helpers are supplied by
// the corresponding external dependencies.

unsafe fn bpf_stream_elem_init(elem: *mut bpf_stream_elem, len: i32) {
    init_llist_node(unsafe { &mut (*elem).node });
    unsafe {
        (*elem).total_len = len;
        (*elem).consumed_len = 0;
    }
}

unsafe fn bpf_stream_elem_alloc(len: i32) -> *mut bpf_stream_elem {
    let max_len: i32 = (core::mem::size_of::<bpf_bprintf_buffers>() -
        core::mem::offset_of!(bpf_bprintf_buffers, buf)) as i32;
    if len < 0 || len > max_len {
        return core::ptr::null_mut();
    }
    let alloc_size = core::mem::offset_of!(bpf_stream_elem, str) + len as usize;
    let elem = kmalloc_nolock(alloc_size, __GFP_ZERO, -1);
    if elem.is_null() {
        return core::ptr::null_mut();
    }
    bpf_stream_elem_init(elem, len);
    elem
}

unsafe fn __bpf_stream_push_str(log: *mut llist_head, str_: *const i8, len: i32) -> i32 {
    let elem = bpf_stream_elem_alloc(len);
    if elem.is_null() {
        return -ENOMEM;
    }
    core::ptr::copy_nonoverlapping(str_ as *const u8, (*elem).str.as_mut_ptr(), len as usize);
    llist_add(&mut (*elem).node, log);
    0
}

unsafe fn bpf_stream_consume_capacity(stream: *mut bpf_stream, len: i32) -> i32 {
    if atomic_read(&(*stream).capacity) >= BPF_STREAM_MAX_CAPACITY {
        return -ENOSPC;
    }
    if atomic_add_return(len, &mut (*stream).capacity) >= BPF_STREAM_MAX_CAPACITY {
        atomic_sub(len, &mut (*stream).capacity);
        return -ENOSPC;
    }
    0
}

unsafe fn bpf_stream_release_capacity(stream: *mut bpf_stream, elem: *mut bpf_stream_elem) {
    atomic_sub((*elem).total_len, &mut (*stream).capacity);
}

unsafe fn bpf_stream_push_str(stream: *mut bpf_stream, str_: *const i8, len: i32) -> i32 {
    let ret = bpf_stream_consume_capacity(stream, len);
    if ret != 0 { ret } else { __bpf_stream_push_str(&mut (*stream).log, str_, len) }
}

unsafe fn bpf_stream_get(stream_id: bpf_stream_id, aux: *mut bpf_prog_aux) -> *mut bpf_stream {
    if stream_id != BPF_STDOUT && stream_id != BPF_STDERR {
        return core::ptr::null_mut();
    }
    &mut (*aux).stream[(stream_id - 1) as usize]
}

unsafe fn bpf_stream_free_elem(elem: *mut bpf_stream_elem) { kfree_nolock(elem); }

unsafe fn bpf_stream_free_list(mut list: *mut llist_node) {
    while !list.is_null() {
        let elem = container_of_stream_elem(list);
        let tmp = (*list).next;
        bpf_stream_free_elem(elem);
        list = tmp;
    }
}

unsafe fn bpf_stream_backlog_peek(stream: *mut bpf_stream) -> *mut llist_node {
    (*stream).backlog_head
}

unsafe fn bpf_stream_backlog_pop(stream: *mut bpf_stream) -> *mut llist_node {
    let node = (*stream).backlog_head;
    if (*stream).backlog_head == (*stream).backlog_tail {
        (*stream).backlog_head = core::ptr::null_mut();
        (*stream).backlog_tail = core::ptr::null_mut();
    } else {
        (*stream).backlog_head = (*node).next;
    }
    node
}

unsafe fn bpf_stream_backlog_fill(stream: *mut bpf_stream) {
    if llist_empty(&(*stream).log) { return; }
    let tail = llist_del_all(&mut (*stream).log);
    if tail.is_null() { return; }
    let head = llist_reverse_order(tail);
    if (*stream).backlog_head.is_null() {
        (*stream).backlog_head = head;
        (*stream).backlog_tail = tail;
    } else {
        (*(*stream).backlog_tail).next = head;
        (*stream).backlog_tail = tail;
    }
}

unsafe fn bpf_stream_consume_elem(elem: *mut bpf_stream_elem, len: *mut i32) -> bool {
    let rem = (*elem).total_len - (*elem).consumed_len;
    let used = core::cmp::min(rem, *len);
    (*elem).consumed_len += used;
    *len -= used;
    (*elem).consumed_len == (*elem).total_len
}

unsafe fn bpf_stream_read(stream: *mut bpf_stream, buf: *mut core::ffi::c_void, len: i32) -> i32 {
    let mut rem_len = len;
    let mut ret = 0;
    mutex_lock(&mut (*stream).lock);
    while rem_len != 0 {
        let pos = len - rem_len;
        let mut node = bpf_stream_backlog_peek(stream);
        if node.is_null() {
            bpf_stream_backlog_fill(stream);
            node = bpf_stream_backlog_peek(stream);
        }
        if node.is_null() { break; }
        let elem = container_of_stream_elem(node);
        let cons_len = (*elem).consumed_len;
        let cont = !bpf_stream_consume_elem(elem, &mut rem_len);
        ret = copy_to_user((buf as *mut u8).add(pos as usize), (*elem).str.as_ptr().add(cons_len as usize), ( (*elem).consumed_len - cons_len) as usize);
        if ret != 0 {
            ret = -EFAULT;
            (*elem).consumed_len = cons_len;
            break;
        }
        if cont { continue; }
        bpf_stream_backlog_pop(stream);
        bpf_stream_release_capacity(stream, elem);
        bpf_stream_free_elem(elem);
    }
    mutex_unlock(&mut (*stream).lock);
    if ret != 0 { ret } else { len - rem_len }
}

pub unsafe fn bpf_prog_stream_read(prog: *mut bpf_prog, stream_id: bpf_stream_id, buf: *mut core::ffi::c_void, len: i32) -> i32 {
    let stream = bpf_stream_get(stream_id, (*prog).aux);
    if stream.is_null() { return -ENOENT; }
    bpf_stream_read(stream, buf, len)
}

pub unsafe fn bpf_stream_vprintk(stream_id: i32, fmt__str: *const i8, args: *const core::ffi::c_void, len__sz: u32, aux: *mut bpf_prog_aux) -> i32 {
    let mut data = bpf_bprintf_data { get_bin_args: true, get_buf: true };
    let fmt_size = strlen(fmt__str) + 1;
    let stream = bpf_stream_get(stream_id, aux);
    if stream.is_null() { return -ENOENT; }
    if (len__sz & 7) != 0 || len__sz > MAX_BPRINTF_VARARGS * 8 || (len__sz != 0 && args.is_null()) { return -EINVAL; }
    let num_args = len__sz / 8;
    let mut ret = bpf_bprintf_prepare(fmt__str, fmt_size, args, num_args, &mut data);
    if ret < 0 { return ret; }
    ret = bstr_printf(data.buf, MAX_BPRINTF_BUF, fmt__str, data.bin_args);
    ret = bpf_stream_push_str(stream, data.buf, ret);
    bpf_bprintf_cleanup(&mut data);
    ret
}

pub unsafe fn bpf_stream_print_stack(stream_id: i32, aux: *mut bpf_prog_aux) -> i32 {
    if bpf_stream_get(stream_id, aux).is_null() { return -ENOENT; }
    let prog = (*(*aux).main_prog_aux).prog;
    let mut ss = bpf_stream_stage { log: core::mem::zeroed(), len: 0 };
    bpf_stream_stage_init(&mut ss);
    bpf_stream_dump_stack(&mut ss);
    0
}

pub unsafe fn bpf_prog_stream_init(prog: *mut bpf_prog) {
    for stream in (*(*prog).aux).stream.iter_mut() {
        atomic_set(&mut stream.capacity, 0);
        init_llist_head(&mut stream.log);
        mutex_init(&mut stream.lock);
        stream.backlog_head = core::ptr::null_mut();
        stream.backlog_tail = core::ptr::null_mut();
    }
}

pub unsafe fn bpf_prog_stream_free(prog: *mut bpf_prog) {
    for stream in (*(*prog).aux).stream.iter_mut() {
        let list = llist_del_all(&mut stream.log);
        bpf_stream_free_list(list);
        bpf_stream_free_list(stream.backlog_head);
    }
}

pub unsafe fn bpf_stream_stage_init(ss: *mut bpf_stream_stage) {
    init_llist_head(&mut (*ss).log);
    (*ss).len = 0;
}

pub unsafe fn bpf_stream_stage_free(ss: *mut bpf_stream_stage) {
    bpf_stream_free_list(llist_del_all(&mut (*ss).log));
}

pub unsafe fn bpf_stream_stage_printk(ss: *mut bpf_stream_stage, fmt: *const i8, mut args: ...) -> i32 {
    let mut buf: *mut bpf_bprintf_buffers = core::ptr::null_mut();
    if bpf_try_get_buffers(&mut buf) != 0 { return -EBUSY; }
    let ret = vsnprintf((*buf).buf, (*buf).buf.len(), fmt, args);
    (*ss).len += ret;
    let ret = __bpf_stream_push_str(&mut (*ss).log, (*buf).buf, ret);
    bpf_put_buffers();
    ret
}

pub unsafe fn bpf_stream_stage_commit(ss: *mut bpf_stream_stage, prog: *mut bpf_prog, stream_id: bpf_stream_id) -> i32 {
    let stream = bpf_stream_get(stream_id, (*prog).aux);
    if stream.is_null() { return -EINVAL; }
    let ret = bpf_stream_consume_capacity(stream, (*ss).len);
    if ret != 0 { return ret; }
    let list = llist_del_all(&mut (*ss).log);
    if list.is_null() { return 0; }
    let head = list;
    let mut tail = list;
    while !(*tail).next.is_null() { tail = (*tail).next; }
    llist_add_batch(head, tail, &mut (*stream).log);
    0
}

pub struct dump_stack_ctx { pub ss: *mut bpf_stream_stage, pub err: i32 }

pub unsafe extern "C" fn dump_stack_cb(cookie: *mut core::ffi::c_void, ip: u64, _sp: u64, _bp: u64) -> bool {
    let ctxp = cookie as *mut dump_stack_ctx;
    let mut file: *const i8 = b"\0".as_ptr() as *const i8;
    let mut line: *const i8 = b"\0".as_ptr() as *const i8;
    rcu_read_lock();
    let prog = bpf_prog_ksym_find(ip);
    rcu_read_unlock();
    if !prog.is_null() {
        let mut num = 0;
        let ret = bpf_prog_get_file_line(prog, ip, &mut file, &mut line, &mut num);
        if ret >= 0 {
            (*ctxp).err = bpf_stream_stage_printk((*ctxp).ss, c"%pS\n  %s @ %s:%d\n".as_ptr(), ip as i64 as *const core::ffi::c_void, line, file, num);
            return (*ctxp).err == 0;
        }
    }
    (*ctxp).err = bpf_stream_stage_printk((*ctxp).ss, c"%pS\n".as_ptr(), ip as i64 as *const core::ffi::c_void);
    (*ctxp).err == 0
}

pub unsafe fn bpf_stream_stage_dump_stack(ss: *mut bpf_stream_stage) -> i32 {
    let mut ctx = dump_stack_ctx { ss, err: 0 };
    let mut ret = bpf_stream_stage_printk(ss, c"CPU: %d UID: %d PID: %d Comm: %s\n".as_ptr(), raw_smp_processor_id(), __kuid_val(current_real_cred()->euid), current->pid, current->comm);
    if ret != 0 { return ret; }
    ret = bpf_stream_stage_printk(ss, c"Call trace:\n".as_ptr());
    if ret != 0 { return ret; }
    arch_bpf_stack_walk(dump_stack_cb, &mut ctx as *mut _ as *mut core::ffi::c_void);
    if ctx.err != 0 { return ctx.err; }
    bpf_stream_stage_printk(ss, c"\n".as_ptr())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
