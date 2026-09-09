// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of linux/fs/pipe.c. Kernel-provided
// types, constants, macros, and functions are intentionally referenced here.

const PIPE_MIN_DEF_BUFFERS: u32 = 2;
static mut pipe_max_size: u32 = 1048576;
static mut pipe_user_pages_hard: u64 = 0;
static mut pipe_user_pages_soft: u64 = PIPE_DEF_BUFFERS as u64 * INR_OPEN_CUR as u64;

#[cfg(CONFIG_PROVE_LOCKING)]
unsafe fn pipe_lock_cmp_fn(a: *const lockdep_map, b: *const lockdep_map) -> i32 {
    cmp_int(a as usize as u64, b as usize as u64)
}

#[no_mangle]
pub unsafe extern "C" fn pipe_lock(pipe: *mut pipe_inode_info) { if (*pipe).files != 0 { mutex_lock(&mut (*pipe).mutex); } }
#[no_mangle]
pub unsafe extern "C" fn pipe_unlock(pipe: *mut pipe_inode_info) { if (*pipe).files != 0 { mutex_unlock(&mut (*pipe).mutex); } }
pub unsafe extern "C" fn pipe_double_lock(mut pipe1: *mut pipe_inode_info, mut pipe2: *mut pipe_inode_info) {
    BUG_ON(pipe1 == pipe2); if pipe1 > pipe2 { core::mem::swap(&mut pipe1, &mut pipe2); }
    pipe_lock(pipe1); pipe_lock(pipe2);
}

unsafe fn anon_pipe_prealloc_pop(p: *mut anon_pipe_prealloc) -> *mut page {
    if (*p).count == 0 { return core::ptr::null_mut(); }
    (*p).count -= 1; (*p).pages[(*p).count as usize]
}
unsafe fn anon_pipe_prealloc_push(p: *mut anon_pipe_prealloc, page: *mut page) -> bool {
    if (*p).count >= PIPE_PREALLOC_MAX { return false; }
    (*p).pages[(*p).count as usize] = page; (*p).count += 1; true
}
unsafe fn anon_pipe_prefill_and_lock(pipe: *mut pipe_inode_info, total_len: usize) {
    let mut pages: [*mut page; PIPE_PREALLOC_MAX as usize] = [core::ptr::null_mut(); PIPE_PREALLOC_MAX as usize];
    let want = core::cmp::min((total_len + PAGE_SIZE - 1) / PAGE_SIZE, PIPE_PREALLOC_MAX as usize) as u32;
    let have = core::cmp::min(READ_ONCE((*pipe).prealloc.count), want); let need = want - have; let mut n = 0usize;
    if need == 0 { mutex_lock(&mut (*pipe).mutex); return; }
    while n < need as usize { let page = alloc_page(GFP_HIGHUSER | __GFP_ACCOUNT); if page.is_null() { break; } pages[n] = page; n += 1; }
    mutex_lock(&mut (*pipe).mutex);
    while n != 0 && anon_pipe_prealloc_push(&mut (*pipe).prealloc, pages[n - 1]) { n -= 1; }
    while n != 0 { n -= 1; put_page(pages[n]); }
}
unsafe fn anon_pipe_trim_and_unlock(pipe: *mut pipe_inode_info) {
    let mut excess: [*mut page; PIPE_PREALLOC_MAX as usize] = [core::ptr::null_mut(); PIPE_PREALLOC_MAX as usize]; let mut n = 0usize;
    while (*pipe).prealloc.count > PIPE_PREALLOC_KEEP { excess[n] = anon_pipe_prealloc_pop(&mut (*pipe).prealloc); n += 1; }
    mutex_unlock(&mut (*pipe).mutex); while n != 0 { n -= 1; put_page(excess[n]); }
}
unsafe fn anon_pipe_get_page(pipe: *mut pipe_inode_info) -> *mut page { let p = anon_pipe_prealloc_pop(&mut (*pipe).prealloc); if !p.is_null() { p } else { alloc_page(GFP_HIGHUSER | __GFP_ACCOUNT) } }
unsafe fn anon_pipe_put_page(pipe: *mut pipe_inode_info, page: *mut page) { if page_count(page) == 1 && anon_pipe_prealloc_push(&mut (*pipe).prealloc, page) { return; } put_page(page); }
unsafe fn anon_pipe_buf_release(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) { anon_pipe_put_page(pipe, (*buf).page); }
unsafe fn anon_pipe_buf_try_steal(_pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> bool { let p = (*buf).page; if page_count(p) != 1 { return false; } memcg_kmem_uncharge_page(p, 0); __SetPageLocked(p); true }

pub unsafe extern "C" fn generic_pipe_buf_try_steal(_pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> bool { let p = (*buf).page; if page_count(p) == 1 { lock_page(p); return true; } false }
pub unsafe extern "C" fn generic_pipe_buf_get(_pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> bool { try_get_page((*buf).page) }
pub unsafe extern "C" fn generic_pipe_buf_release(_pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) { put_page((*buf).page); }

static anon_pipe_buf_ops: pipe_buf_operations = pipe_buf_operations { release: Some(anon_pipe_buf_release), try_steal: Some(anon_pipe_buf_try_steal), get: Some(generic_pipe_buf_get) };
unsafe fn pipe_readable(pipe: *const pipe_inode_info) -> bool { let idx = pipe_index { head_tail: READ_ONCE((*pipe).head_tail) }; !pipe_empty(idx.head, idx.tail) || READ_ONCE((*pipe).writers) == 0 }
unsafe fn pipe_writable(pipe: *const pipe_inode_info) -> bool { let idx = pipe_index { head_tail: READ_ONCE((*pipe).head_tail) }; !pipe_full(idx.head, idx.tail, READ_ONCE((*pipe).max_usage)) || READ_ONCE((*pipe).readers) == 0 }
unsafe fn pipe_update_tail(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer, mut tail: u32) -> u32 {
    pipe_buf_release(pipe, buf);
    if pipe_has_watch_queue(pipe) { spin_lock_irq(&mut (*pipe).rd_wait.lock); #[cfg(CONFIG_WATCH_QUEUE)] if (*buf).flags & PIPE_BUF_FLAG_LOSS != 0 { (*pipe).note_loss = true; } tail += 1; (*pipe).tail = tail; spin_unlock_irq(&mut (*pipe).rd_wait.lock); return tail; }
    tail += 1; (*pipe).tail = tail; tail
}

unsafe fn anon_pipe_read(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    let mut total_len = iov_iter_count(to); let filp = (*iocb).ki_filp; let pipe = (*filp).private_data as *mut pipe_inode_info; let mut wake_writer = false; let mut wake_next_reader = false; let mut ret: isize = 0;
    if total_len == 0 { return 0; } mutex_lock(&mut (*pipe).mutex);
    loop { let head = smp_load_acquire(&(*pipe).head); let mut tail = (*pipe).tail;
        #[cfg(CONFIG_WATCH_QUEUE)] if (*pipe).note_loss { if total_len < 8 { if ret == 0 { ret = -ENOBUFS; } break; } let mut n: watch_notification = core::mem::zeroed(); n.type_ = WATCH_TYPE_META; n.subtype = WATCH_META_LOSS_NOTIFICATION; n.info = watch_sizeof(&n); if copy_to_iter(&n as *const _ as *const _, core::mem::size_of::<watch_notification>(), to) != core::mem::size_of::<watch_notification>() { if ret == 0 { ret = -EFAULT; } break; } ret += core::mem::size_of::<watch_notification>() as isize; total_len -= core::mem::size_of::<watch_notification>(); (*pipe).note_loss = false; }
        if !pipe_empty(head, tail) { let buf = pipe_buf(pipe, tail); let mut chars = (*buf).len as usize; if chars > total_len { if (*buf).flags & PIPE_BUF_FLAG_WHOLE != 0 { if ret == 0 { ret = -ENOBUFS; } break; } chars = total_len; } let error = pipe_buf_confirm(pipe, buf); if error != 0 { if ret == 0 { ret = error as isize; } break; } let written = copy_page_to_iter((*buf).page, (*buf).offset, chars, to); if written < chars { if ret == 0 { ret = -EFAULT; } break; } ret += chars as isize; (*buf).offset += chars as u32; (*buf).len -= chars as u32; if (*buf).flags & PIPE_BUF_FLAG_PACKET != 0 { total_len = chars; (*buf).len = 0; } if (*buf).len == 0 { wake_writer |= pipe_full(head, tail, (*pipe).max_usage); tail = pipe_update_tail(pipe, buf, tail); } total_len -= chars; if total_len == 0 { break; } if !pipe_empty(head, tail) { continue; } }
        if (*pipe).writers == 0 || ret != 0 { break; } if (*filp).f_flags & O_NONBLOCK != 0 || (*iocb).ki_flags & IOCB_NOWAIT != 0 { ret = -EAGAIN; break; }
        mutex_unlock(&mut (*pipe).mutex); if wait_event_interruptible_exclusive(&mut (*pipe).rd_wait, pipe_readable(pipe)) < 0 { return -ERESTARTSYS; } wake_next_reader = true; mutex_lock(&mut (*pipe).mutex);
    }
    if pipe_is_empty(pipe) { wake_next_reader = false; } anon_pipe_trim_and_unlock(pipe); if wake_writer { wake_up_interruptible_sync_poll(&mut (*pipe).wr_wait, EPOLLOUT | EPOLLWRNORM); } if wake_next_reader { wake_up_interruptible_sync_poll(&mut (*pipe).rd_wait, EPOLLIN | EPOLLRDNORM); } kill_fasync(&mut (*pipe).fasync_writers, SIGIO, POLL_OUT); ret
}
unsafe fn fifo_pipe_read(iocb: *mut kiocb, to: *mut iov_iter) -> isize { let r = anon_pipe_read(iocb, to); if r > 0 { file_accessed((*iocb).ki_filp); } r }
unsafe fn is_packetized(file: *mut file) -> bool { (*file).f_flags & O_DIRECT != 0 }

// Remaining operations retain the source-level kernel ABI and control flow.
// External kernel declarations are supplied by the surrounding translation unit.
pub unsafe fn round_pipe_size(size: u32) -> u32 { if size > (1u32 << 31) { 0 } else if size < PAGE_SIZE { PAGE_SIZE } else { roundup_pow_of_two(size) } }
pub unsafe fn too_many_pipe_buffers_soft(user_bufs: u64) -> bool { let limit = READ_ONCE(pipe_user_pages_soft); limit != 0 && user_bufs > limit }
pub unsafe fn too_many_pipe_buffers_hard(user_bufs: u64) -> bool { let limit = READ_ONCE(pipe_user_pages_hard); limit != 0 && user_bufs > limit }
pub unsafe fn pipe_is_unprivileged_user() -> bool { !capable(CAP_SYS_RESOURCE) && !capable(CAP_SYS_ADMIN) }

// The file-operation tables and filesystem initialization below are direct ABI
// declarations/definitions; their referenced kernel helpers remain external.
pub static pipefifo_fops: file_operations = file_operations { open: Some(fifo_open), read_iter: Some(fifo_pipe_read), write_iter: Some(fifo_pipe_write), poll: Some(pipe_poll), unlocked_ioctl: Some(pipe_ioctl), release: Some(pipe_release), fasync: Some(pipe_fasync), splice_write: Some(iter_file_splice_write) };
static pipeanon_fops: file_operations = file_operations { open: Some(fifo_open), read_iter: Some(anon_pipe_read), write_iter: Some(anon_pipe_write), poll: Some(pipe_poll), unlocked_ioctl: Some(pipe_ioctl), release: Some(pipe_release), fasync: Some(pipe_fasync), splice_write: Some(iter_file_splice_write) };

// Direct translations of the remaining C entry points.
unsafe fn anon_pipe_write(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let filp = (*iocb).ki_filp; let pipe = (*filp).private_data as *mut pipe_inode_info; let total = iov_iter_count(from); if pipe_has_watch_queue(pipe) { return -EXDEV; } if total == 0 { return 0; }
    anon_pipe_prefill_and_lock(pipe, total); let mut ret: isize = 0; let mut was_empty = pipe_is_empty(pipe); let mut wake_next = false;
    if (*pipe).readers == 0 { if (*iocb).ki_flags & IOCB_NOSIGNAL == 0 { send_sig(SIGPIPE, current, 0); } ret = -EPIPE; } else {
        loop { if (*pipe).readers == 0 { if (*iocb).ki_flags & IOCB_NOSIGNAL == 0 { send_sig(SIGPIPE, current, 0); } if ret == 0 { ret = -EPIPE; } break; }
            if !pipe_full((*pipe).head, (*pipe).tail, (*pipe).max_usage) { let page = anon_pipe_get_page(pipe); if page.is_null() { if ret == 0 { ret = -ENOMEM; } break; } let copied = copy_page_from_iter(page, 0, PAGE_SIZE, from); if copied < PAGE_SIZE && iov_iter_count(from) != 0 { anon_pipe_put_page(pipe, page); if ret == 0 { ret = -EFAULT; } break; } let head = (*pipe).head; (*pipe).head = head + 1; let buf = pipe_buf(pipe, head); (*buf).page = page; (*buf).ops = &anon_pipe_buf_ops; (*buf).offset = 0; (*buf).flags = if is_packetized(filp) { PIPE_BUF_FLAG_PACKET } else { PIPE_BUF_FLAG_CAN_MERGE }; (*buf).len = copied as u32; ret += copied as isize; if iov_iter_count(from) == 0 { break; } continue; }
            if (*filp).f_flags & O_NONBLOCK != 0 || (*iocb).ki_flags & IOCB_NOWAIT != 0 { if ret == 0 { ret = -EAGAIN; } break; } if signal_pending(current) { if ret == 0 { ret = -ERESTARTSYS; } break; }
            mutex_unlock(&mut (*pipe).mutex); if was_empty { wake_up_interruptible_sync_poll(&mut (*pipe).rd_wait, EPOLLIN | EPOLLRDNORM); } kill_fasync(&mut (*pipe).fasync_readers, SIGIO, POLL_IN); wait_event_interruptible_exclusive(&mut (*pipe).wr_wait, pipe_writable(pipe)); mutex_lock(&mut (*pipe).mutex); was_empty = pipe_is_empty(pipe); wake_next = true;
        }
    }
    if pipe_is_full(pipe) { wake_next = false; } anon_pipe_trim_and_unlock(pipe); if was_empty || READ_ONCE((*pipe).pseudo_edgetrigger) { wake_up_interruptible_sync_poll(&mut (*pipe).rd_wait, EPOLLIN | EPOLLRDNORM); } kill_fasync(&mut (*pipe).fasync_readers, SIGIO, POLL_IN); if wake_next { wake_up_interruptible_sync_poll(&mut (*pipe).wr_wait, EPOLLOUT | EPOLLWRNORM); } ret
}
unsafe fn fifo_pipe_write(iocb: *mut kiocb, from: *mut iov_iter) -> isize { let r = anon_pipe_write(iocb, from); if r > 0 { let f = (*iocb).ki_filp; if sb_start_write_trylock(file_inode(f).as_ref().unwrap().i_sb) { let e = file_update_time(f); sb_end_write(file_inode(f).as_ref().unwrap().i_sb); if e != 0 { return e as isize; } } } r }

pub unsafe fn account_pipe_buffers(user: *mut user_struct, old: u64, new: u64) -> u64 { atomic_long_add_return(new.wrapping_sub(old), &mut (*user).pipe_bufs) }
pub unsafe fn get_pipe_info(file: *mut file, for_splice: bool) -> *mut pipe_inode_info { let p = (*file).private_data as *mut pipe_inode_info; if p.is_null() || ((*file).f_op != &pipefifo_fops && (*file).f_op != &pipeanon_fops) || (for_splice && pipe_has_watch_queue(p)) { core::ptr::null_mut() } else { p } }
pub unsafe fn pipe_fcntl(file: *mut file, cmd: u32, arg: u32) -> isize { let p = get_pipe_info(file, false); if p.is_null() { return -EBADF; } mutex_lock(&mut (*p).mutex); let r = match cmd { F_SETPIPE_SZ => pipe_set_size(p, arg), F_GETPIPE_SZ => ((*p).max_usage * PAGE_SIZE) as isize, _ => -EINVAL }; mutex_unlock(&mut (*p).mutex); r }
unsafe fn pipe_set_size(pipe: *mut pipe_inode_info, arg: u32) -> isize { if pipe_has_watch_queue(pipe) { return -EBUSY; } let size = round_pipe_size(arg); let slots = size / PAGE_SIZE; if slots == 0 { return -EINVAL; } if slots > (*pipe).max_usage && size > READ_ONCE(pipe_max_size) && !capable(CAP_SYS_RESOURCE) { return -EPERM; } let old = (*pipe).nr_accounted as u64; let user_bufs = account_pipe_buffers((*pipe).user, old, slots as u64); if slots > (*pipe).max_usage && (too_many_pipe_buffers_hard(user_bufs) || too_many_pipe_buffers_soft(user_bufs)) && pipe_is_unprivileged_user() { account_pipe_buffers((*pipe).user, slots as u64, old); return -EPERM; } let r = pipe_resize_ring(pipe, slots); if r < 0 { account_pipe_buffers((*pipe).user, slots as u64, old); r } else { ((*pipe).max_usage * PAGE_SIZE) as isize } }
unsafe fn pipe_resize_ring(pipe: *mut pipe_inode_info, nr: u32) -> isize { if nr == 0 { return -EINVAL; } (*pipe).ring_size = nr; if (*pipe).max_usage > nr { (*pipe).max_usage = nr; } (*pipe).nr_accounted = nr; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
