// SPDX-License-Identifier: GPL-2.0
/* System V messages: translated from ipc/msg.c.
 * All queue control flow and ABI conversions are owned here. Generated C
 * layouts and narrow helpers preserve the kernel's synchronization primitives.
 */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/ipc_msg_generated.rs"));
}
use bindings::*;
use kernel::ffi::{c_int, c_long, c_ulong, c_void};
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of_mut, null_mut};

const SEARCH_ANY: c_int = 1;
const SEARCH_EQUAL: c_int = 2;
const SEARCH_NOTEQUAL: c_int = 3;
const SEARCH_LESSEQUAL: c_int = 4;
const SEARCH_NUMBER: c_int = 5;

#[inline] fn err_ptr<T>(err: c_int) -> *mut T { err as isize as *mut T }
#[inline] fn is_err<T>(p: *mut T) -> bool { (p as usize) >= usize::MAX - 4094 }
#[inline] fn ptr_err<T>(p: *mut T) -> c_int { p as isize as c_int }
#[inline] unsafe fn queue(p: *mut kern_ipc_perm) -> *mut msg_queue {
    if is_err(p) { p.cast() } else { p.cast::<u8>().sub(offset_of!(msg_queue, q_perm)).cast() }
}
#[inline] unsafe fn perm(q: *mut msg_queue) -> *mut kern_ipc_perm { addr_of_mut!((*q).q_perm) }
#[inline] unsafe fn obtain(ns: *mut ipc_namespace, id: c_int) -> *mut msg_queue {
    queue(ipc_obtain_object_check(rust_msg_ids(ns), id))
}
unsafe extern "C" fn msg_rcu_free(head: *mut callback_head) {
    let p = head.cast::<u8>().sub(offset_of!(kern_ipc_perm, rcu)).cast::<kern_ipc_perm>();
    let q = queue(p);
    rust_msg_security_free(perm(q));
    kfree(q.cast());
}
unsafe extern "C" fn newque(ns: *mut ipc_namespace, params: *mut ipc_params) -> c_int {
    let q = rust_msg_alloc();
    if q.is_null() { return -(ENOMEM as c_int); }
    (*q).q_perm.mode = ((*params).flg & S_IRWXUGO as c_int) as _;
    (*q).q_perm.key = (*params).key;
    (*q).q_perm.security = null_mut();
    let ret = rust_msg_security_alloc(perm(q));
    if ret != 0 { kfree(q.cast()); return ret; }
    (*q).q_stime = 0;
    (*q).q_rtime = 0;
    (*q).q_ctime = ktime_get_real_seconds();
    (*q).q_cbytes = 0;
    (*q).q_qnum = 0;
    (*q).q_qbytes = *rust_msg_ctlmnb(ns) as c_ulong;
    (*q).q_lspid = null_mut();
    (*q).q_lrpid = null_mut();
    rust_msg_list_init(addr_of_mut!((*q).q_messages));
    rust_msg_list_init(addr_of_mut!((*q).q_receivers));
    rust_msg_list_init(addr_of_mut!((*q).q_senders));
    let ret = ipc_addid(rust_msg_ids(ns), perm(q), *rust_msg_ctlmni(ns));
    if ret < 0 { ipc_rcu_putref(perm(q), Some(msg_rcu_free)); return ret; }
    rust_msg_unlock(perm(q));
    rust_msg_rcu_read_unlock();
    (*q).q_perm.id
}
#[inline] unsafe fn msg_fits_inqueue(q: *mut msg_queue, size: usize) -> bool {
    (size as c_ulong).wrapping_add((*q).q_cbytes) <= (*q).q_qbytes &&
        (*q).q_qnum.wrapping_add(1) <= (*q).q_qbytes
}
unsafe fn ss_wakeup(q: *mut msg_queue, wake: *mut wake_q_head, kill: bool) {
    let head = addr_of_mut!((*q).q_senders);
    let mut node = (*head).next;
    let mut stop = null_mut();
    while node != head {
        let sender = node.cast::<msg_sender>();
        let next = (*node).next;
        if kill { (*sender).list.next = null_mut(); }
        else if stop == (*sender).tsk { break; }
        else if !msg_fits_inqueue(q, (*sender).msgsz) {
            if stop.is_null() { stop = (*sender).tsk; }
            rust_msg_list_move_tail(node, head);
            node = next;
            continue;
        }
        wake_q_add(wake, (*sender).tsk);
        node = next;
    }
}
unsafe fn expunge_all(q: *mut msg_queue, result: c_int, wake: *mut wake_q_head) {
    let head = addr_of_mut!((*q).q_receivers);
    let mut node = (*head).next;
    while node != head {
        let next = (*node).next;
        let recv = node.cast::<msg_receiver>();
        let task = rust_msg_get_task((*recv).r_tsk);
        rust_msg_store_release(addr_of_mut!((*recv).r_msg), err_ptr(result));
        wake_q_add_safe(wake, task);
        node = next;
    }
}
unsafe extern "C" fn freeque(ns: *mut ipc_namespace, p: *mut kern_ipc_perm) {
    let q = queue(p);
    let mut wake: wake_q_head = zeroed();
    rust_msg_wake_init(&raw mut wake);
    expunge_all(q, -(EIDRM as c_int), &raw mut wake);
    ss_wakeup(q, &raw mut wake, true);
    ipc_rmid(rust_msg_ids(ns), perm(q));
    rust_msg_unlock(perm(q));
    wake_up_q(&raw mut wake);
    rust_msg_rcu_read_unlock();
    let head = addr_of_mut!((*q).q_messages);
    let mut node = (*head).next;
    while node != head {
        let next = (*node).next;
        rust_msg_counter_add(rust_msg_hdrs(ns), -1);
        free_msg(node.cast());
        node = next;
    }
    rust_msg_counter_add(rust_msg_bytes(ns), ((*q).q_cbytes as i64).wrapping_neg());
    rust_msg_update_pid(addr_of_mut!((*q).q_lspid), null_mut());
    rust_msg_update_pid(addr_of_mut!((*q).q_lrpid), null_mut());
    ipc_rcu_putref(perm(q), Some(msg_rcu_free));
}
#[no_mangle]
pub unsafe extern "C" fn ksys_msgget(key: key_t, flags: c_int) -> c_long {
    let ns = rust_msg_current_ns();
    let ops = ipc_ops { getnew: Some(newque), associate: Some(rust_msg_security_associate), more_checks: None };
    let mut params: ipc_params = zeroed();
    params.key = key;
    params.flg = flags;
    ipcget(ns, rust_msg_ids(ns), &raw const ops, &raw mut params) as c_long
}
#[inline] unsafe fn testmsg(msg: *mut msg_msg, typ: c_long, mode: c_int) -> bool {
    match mode {
        SEARCH_ANY | SEARCH_NUMBER => true,
        SEARCH_LESSEQUAL => (*msg).m_type <= typ,
        SEARCH_EQUAL => (*msg).m_type == typ,
        SEARCH_NOTEQUAL => (*msg).m_type != typ,
        _ => false,
    }
}
unsafe fn pipelined_send(q: *mut msg_queue, msg: *mut msg_msg, wake: *mut wake_q_head) -> bool {
    let head = addr_of_mut!((*q).q_receivers);
    let mut node = (*head).next;
    while node != head {
        let next = (*node).next;
        let recv = node.cast::<msg_receiver>();
        if testmsg(msg, (*recv).r_msgtype, (*recv).r_mode) &&
            rust_msg_security_recv(perm(q), msg, (*recv).r_tsk, (*recv).r_msgtype, (*recv).r_mode) == 0 {
            rust_msg_list_del(node);
            if ((*recv).r_maxsize as usize) < (*msg).m_ts {
                wake_q_add(wake, (*recv).r_tsk);
                rust_msg_store_release(addr_of_mut!((*recv).r_msg), err_ptr(-(E2BIG as c_int)));
            } else {
                rust_msg_update_pid(addr_of_mut!((*q).q_lrpid), rust_msg_task_pid((*recv).r_tsk));
                (*q).q_rtime = ktime_get_real_seconds();
                wake_q_add(wake, (*recv).r_tsk);
                rust_msg_store_release(addr_of_mut!((*recv).r_msg), msg);
                return true;
            }
        }
        node = next;
    }
    false
}
unsafe fn do_msgsnd(id: c_int, typ: c_long, text: *mut c_void, size: usize, flags: c_int) -> c_long {
    let ns = rust_msg_current_ns();
    let mut wake: wake_q_head = zeroed();
    rust_msg_wake_init(&raw mut wake);
    if size > *rust_msg_ctlmax(ns) as usize || (size as c_long) < 0 || id < 0 || typ < 1 {
        return -(EINVAL as c_long);
    }
    let mut msg = load_msg(text, size);
    if is_err(msg) { return ptr_err(msg) as c_long; }
    (*msg).m_type = typ;
    (*msg).m_ts = size;
    rust_msg_rcu_read_lock();
    let q = obtain(ns, id);
    let ret;
    if is_err(q) { ret = ptr_err(q); }
    else {
        rust_msg_lock(perm(q));
        ret = 'send: loop {
            if ipcperms(ns, perm(q), S_IWUGO as _) != 0 { break -(EACCES as c_int); }
            if !rust_msg_valid(perm(q)) { break -(EIDRM as c_int); }
            let err = rust_msg_security_send(perm(q), msg, flags);
            if err != 0 { break err; }
            if msg_fits_inqueue(q, size) {
                rust_msg_update_pid(addr_of_mut!((*q).q_lspid), rust_msg_task_tgid(rust_msg_current()));
                (*q).q_stime = ktime_get_real_seconds();
                if !pipelined_send(q, msg, &raw mut wake) {
                    rust_msg_list_add_tail(addr_of_mut!((*msg).m_list), addr_of_mut!((*q).q_messages));
                    (*q).q_cbytes = (*q).q_cbytes.wrapping_add(size as c_ulong);
                    (*q).q_qnum = (*q).q_qnum.wrapping_add(1);
                    rust_msg_counter_add(rust_msg_bytes(ns), size as i64);
                    rust_msg_counter_add(rust_msg_hdrs(ns), 1);
                }
                msg = null_mut();
                break 0;
            }
            if flags & IPC_NOWAIT as c_int != 0 { break -(EAGAIN as c_int); }
            let mut sender: msg_sender = zeroed();
            sender.tsk = rust_msg_current();
            sender.msgsz = size;
            rust_msg_set_interruptible();
            rust_msg_list_add_tail(&raw mut sender.list, addr_of_mut!((*q).q_senders));
            if !ipc_rcu_getref(perm(q)) { break 'send -(EIDRM as c_int); }
            rust_msg_unlock(perm(q));
            rust_msg_rcu_read_unlock();
            schedule();
            rust_msg_rcu_read_lock();
            rust_msg_lock(perm(q));
            ipc_rcu_putref(perm(q), Some(msg_rcu_free));
            if !rust_msg_valid(perm(q)) { break -(EIDRM as c_int); }
            if !sender.list.next.is_null() { rust_msg_list_del(&raw mut sender.list); }
            if rust_msg_signal_pending() { break -(ERESTARTNOHAND as c_int); }
        };
        rust_msg_unlock(perm(q));
        wake_up_q(&raw mut wake);
    }
    rust_msg_rcu_read_unlock();
    if !msg.is_null() { free_msg(msg); }
    ret as c_long
}
#[no_mangle]
pub unsafe extern "C" fn ksys_msgsnd(id: c_int, msgp: *mut msgbuf, size: usize, flags: c_int) -> c_long {
    let mut typ = 0;
    if rust_msg_get_type(msgp.cast(), &raw mut typ) != 0 { return -(EFAULT as c_long); }
    do_msgsnd(id, typ, msgp.cast::<u8>().wrapping_add(size_of::<c_long>()).cast(), size, flags)
}
#[cfg(CONFIG_COMPAT)]
#[no_mangle]
pub unsafe extern "C" fn compat_ksys_msgsnd(id: c_int, msgp: compat_uptr_t, size: compat_ssize_t, flags: c_int) -> c_long {
    let up = rust_msg_compat_ptr(msgp);
    let mut typ: compat_long_t = 0;
    if rust_msg_get_compat_type(up, &raw mut typ) != 0 { return -(EFAULT as c_long); }
    do_msgsnd(id, typ as c_long, up.cast::<u8>().wrapping_add(size_of::<compat_long_t>()).cast(), size as isize as usize, flags)
}
fn convert_mode(typ: &mut c_long, flags: c_int) -> c_int {
    if flags & MSG_COPY as c_int != 0 { SEARCH_NUMBER }
    else if *typ == 0 { SEARCH_ANY }
    else if *typ < 0 {
        *typ = if *typ == c_long::MIN { c_long::MAX } else { typ.wrapping_neg() };
        SEARCH_LESSEQUAL
    } else if flags & MSG_EXCEPT as c_int != 0 { SEARCH_NOTEQUAL }
    else { SEARCH_EQUAL }
}
unsafe fn do_msg_fill(dest: *mut c_void, msg: *mut msg_msg, size: usize) -> c_long {
    if rust_msg_put_type(dest, (*msg).m_type) != 0 { return -(EFAULT as c_long); }
    let size = size.min((*msg).m_ts);
    if store_msg(dest.cast::<u8>().wrapping_add(size_of::<c_long>()).cast(), msg, size) != 0 { return -(EFAULT as c_long); }
    size as c_long
}
#[cfg(CONFIG_COMPAT)]
unsafe fn compat_do_msg_fill(dest: *mut c_void, msg: *mut msg_msg, size: usize) -> c_long {
    if rust_msg_put_compat_type(dest, (*msg).m_type) != 0 { return -(EFAULT as c_long); }
    let size = size.min((*msg).m_ts);
    if store_msg(dest.cast::<u8>().wrapping_add(size_of::<compat_long_t>()).cast(), msg, size) != 0 { return -(EFAULT as c_long); }
    size as c_long
}
unsafe fn prepare_copy(buf: *mut c_void, size: usize) -> *mut msg_msg {
    #[cfg(CONFIG_CHECKPOINT_RESTORE)] {
        let copy = load_msg(buf, size);
        if !is_err(copy) { (*copy).m_ts = size; }
        copy
    }
    #[cfg(not(CONFIG_CHECKPOINT_RESTORE))] { let _ = (buf, size); err_ptr(-(ENOSYS as c_int)) }
}
unsafe fn free_copy(copy: *mut msg_msg) {
    #[cfg(CONFIG_CHECKPOINT_RESTORE)] if !copy.is_null() { free_msg(copy); }
    #[cfg(not(CONFIG_CHECKPOINT_RESTORE))] let _ = copy;
}
unsafe fn find_msg(q: *mut msg_queue, typ: &mut c_long, mode: c_int) -> *mut msg_msg {
    let head = addr_of_mut!((*q).q_messages);
    let mut node = (*head).next;
    let mut found = err_ptr(-(EAGAIN as c_int));
    let mut count: c_long = 0;
    while node != head {
        let msg = node.cast::<msg_msg>();
        if testmsg(msg, *typ, mode) && rust_msg_security_recv(perm(q), msg, rust_msg_current(), *typ, mode) == 0 {
            if mode == SEARCH_LESSEQUAL && (*msg).m_type != 1 {
                *typ = (*msg).m_type.wrapping_sub(1);
                found = msg;
            } else if mode == SEARCH_NUMBER {
                if *typ == count { return msg; }
            } else { return msg; }
            count = count.wrapping_add(1);
        }
        node = (*node).next;
    }
    found
}
unsafe fn do_msgrcv(id: c_int, buf: *mut c_void, size: usize, mut typ: c_long, flags: c_int,
    handler: unsafe fn(*mut c_void, *mut msg_msg, usize) -> c_long) -> c_long {
    let ns = rust_msg_current_ns();
    let mut copy = null_mut();
    let mut wake: wake_q_head = zeroed();
    rust_msg_wake_init(&raw mut wake);
    if id < 0 || (size as c_long) < 0 { return -(EINVAL as c_long); }
    if flags & MSG_COPY as c_int != 0 {
        if flags & MSG_EXCEPT as c_int != 0 || flags & IPC_NOWAIT as c_int == 0 { return -(EINVAL as c_long); }
        copy = prepare_copy(buf, size.min(*rust_msg_ctlmax(ns) as usize));
        if is_err(copy) { return ptr_err(copy) as c_long; }
    }
    let mode = convert_mode(&mut typ, flags);
    rust_msg_rcu_read_lock();
    let q = obtain(ns, id);
    if is_err(q) {
        rust_msg_rcu_read_unlock();
        free_copy(copy);
        return ptr_err(q) as c_long;
    }
    let mut locked = false;
    let msg = loop {
        if ipcperms(ns, perm(q), S_IRUGO as _) != 0 { break err_ptr(-(EACCES as c_int)); }
        rust_msg_lock(perm(q));
        locked = true;
        if !rust_msg_valid(perm(q)) { break err_ptr(-(EIDRM as c_int)); }
        let found = find_msg(q, &mut typ, mode);
        if !is_err(found) {
            if size < (*found).m_ts && flags & MSG_NOERROR as c_int == 0 { break err_ptr(-(E2BIG as c_int)); }
            if flags & MSG_COPY as c_int != 0 { break copy_msg(found, copy); }
            rust_msg_list_del(addr_of_mut!((*found).m_list));
            (*q).q_qnum = (*q).q_qnum.wrapping_sub(1);
            (*q).q_rtime = ktime_get_real_seconds();
            rust_msg_update_pid(addr_of_mut!((*q).q_lrpid), rust_msg_task_tgid(rust_msg_current()));
            (*q).q_cbytes = (*q).q_cbytes.wrapping_sub((*found).m_ts as c_ulong);
            rust_msg_counter_add(rust_msg_bytes(ns), ((*found).m_ts as i64).wrapping_neg());
            rust_msg_counter_add(rust_msg_hdrs(ns), -1);
            ss_wakeup(q, &raw mut wake, false);
            break found;
        }
        if flags & IPC_NOWAIT as c_int != 0 { break err_ptr(-(ENOMSG as c_int)); }
        let mut recv: msg_receiver = zeroed();
        rust_msg_list_add_tail(&raw mut recv.r_list, addr_of_mut!((*q).q_receivers));
        recv.r_tsk = rust_msg_current();
        recv.r_msgtype = typ;
        recv.r_mode = mode;
        recv.r_maxsize = if flags & MSG_NOERROR as c_int != 0 { c_int::MAX as c_long } else { size as c_long };
        rust_msg_write_once(&raw mut recv.r_msg, err_ptr(-(EAGAIN as c_int)));
        rust_msg_set_interruptible();
        rust_msg_unlock(perm(q));
        locked = false;
        rust_msg_rcu_read_unlock();
        schedule();
        // MSG_BARRIER: an EAGAIN receiver guarantees queue existence under RCU.
        rust_msg_rcu_read_lock();
        let got = rust_msg_read_once(&raw mut recv.r_msg);
        if got != err_ptr(-(EAGAIN as c_int)) {
            rust_msg_acquire_after_ctrl_dep();
            break got;
        }
        rust_msg_lock(perm(q));
        locked = true;
        let got = rust_msg_read_once(&raw mut recv.r_msg);
        if got != err_ptr(-(EAGAIN as c_int)) { break got; }
        rust_msg_list_del(&raw mut recv.r_list);
        if rust_msg_signal_pending() { break err_ptr(-(ERESTARTNOHAND as c_int)); }
        rust_msg_unlock(perm(q));
        locked = false;
    };
    if locked {
        rust_msg_unlock(perm(q));
        wake_up_q(&raw mut wake);
    }
    rust_msg_rcu_read_unlock();
    if is_err(msg) { free_copy(copy); return ptr_err(msg) as c_long; }
    let result = handler(buf, msg, size);
    free_msg(msg);
    result
}
#[no_mangle]
pub unsafe extern "C" fn ksys_msgrcv(id: c_int, msgp: *mut msgbuf, size: usize, typ: c_long, flags: c_int) -> c_long {
    do_msgrcv(id, msgp.cast(), size, typ, flags, do_msg_fill)
}
#[cfg(CONFIG_COMPAT)]
#[no_mangle]
pub unsafe extern "C" fn compat_ksys_msgrcv(id: c_int, msgp: compat_uptr_t, size: compat_ssize_t, typ: compat_long_t, flags: c_int) -> c_long {
    do_msgrcv(id, rust_msg_compat_ptr(msgp), size as isize as usize, typ as c_long, flags, compat_do_msg_fill)
}

#[no_mangle]
pub unsafe extern "C" fn msg_init_ns(ns: *mut ipc_namespace) -> c_int {
    *rust_msg_ctlmax(ns) = MSGMAX as c_int;
    *rust_msg_ctlmnb(ns) = MSGMNB as c_int;
    *rust_msg_ctlmni(ns) = MSGMNI as c_int;
    let ret = rust_msg_counter_init(rust_msg_bytes(ns));
    if ret != 0 { return ret; }
    let ret = rust_msg_counter_init(rust_msg_hdrs(ns));
    if ret != 0 { rust_msg_counter_destroy(rust_msg_bytes(ns)); return ret; }
    ipc_init_ids(rust_msg_ids(ns));
    0
}
#[cfg(CONFIG_IPC_NS)]
#[no_mangle]
pub unsafe extern "C" fn msg_exit_ns(ns: *mut ipc_namespace) {
    free_ipcs(ns, rust_msg_ids(ns), Some(freeque));
    rust_msg_destroy_idr(rust_msg_ids(ns));
    rust_msg_destroy_keys(rust_msg_ids(ns));
    rust_msg_counter_destroy(rust_msg_bytes(ns));
    rust_msg_counter_destroy(rust_msg_hdrs(ns));
}
#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn sysvipc_msg_proc_show(s: *mut seq_file, it: *mut c_void) -> c_int {
    let pid_ns = ipc_seq_pid_ns(s);
    let q = queue(it.cast());
    seq_printf(s, c"%10d %10d  %4o  %10lu %10lu %5u %5u %5u %5u %5u %5u %10llu %10llu %10llu\n".as_ptr().cast(),
        (*q).q_perm.key, (*q).q_perm.id, (*q).q_perm.mode as c_int, (*q).q_cbytes, (*q).q_qnum,
        pid_nr_ns((*q).q_lspid, pid_ns), pid_nr_ns((*q).q_lrpid, pid_ns),
        rust_msg_uid(s, (*q).q_perm.uid), rust_msg_gid(s, (*q).q_perm.gid),
        rust_msg_uid(s, (*q).q_perm.cuid), rust_msg_gid(s, (*q).q_perm.cgid),
        (*q).q_stime as u64, (*q).q_rtime as u64, (*q).q_ctime as u64);
    0
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn msg_init() {
    msg_init_ns(rust_msg_init_ns_ptr());
    #[cfg(CONFIG_PROC_FS)] rust_msg_proc_init(Some(sysvipc_msg_proc_show));
    #[cfg(not(CONFIG_PROC_FS))] rust_msg_proc_init(None);
}

unsafe fn copy_msqid_to_user(buf: *mut c_void, input: *mut msqid64_ds, version: c_int) -> bool {
    if version == IPC_64 as c_int {
        return rust_msg_copy_to_user(buf, input.cast(), size_of::<msqid64_ds>()) != 0;
    }
    if version != IPC_OLD as c_int { return true; }
    let mut out: msqid_ds = zeroed();
    ipc64_perm_to_ipc_perm(addr_of_mut!((*input).msg_perm), &raw mut out.msg_perm);
    out.msg_stime = (*input).msg_stime as _;
    out.msg_rtime = (*input).msg_rtime as _;
    out.msg_ctime = (*input).msg_ctime as _;
    out.msg_cbytes = (*input).msg_cbytes.min(u16::MAX as _) as _;
    out.msg_lcbytes = (*input).msg_cbytes;
    out.msg_qnum = (*input).msg_qnum.min(u16::MAX as _) as _;
    out.msg_qbytes = (*input).msg_qbytes.min(u16::MAX as _) as _;
    out.msg_lqbytes = (*input).msg_qbytes;
    out.msg_lspid = (*input).msg_lspid as _;
    out.msg_lrpid = (*input).msg_lrpid as _;
    rust_msg_copy_to_user(buf, (&raw const out).cast(), size_of::<msqid_ds>()) != 0
}
unsafe fn copy_msqid_from_user(out: *mut msqid64_ds, buf: *mut c_void, version: c_int) -> bool {
    if version == IPC_64 as c_int {
        return rust_msg_copy_from_user(out.cast(), buf, size_of::<msqid64_ds>()) != 0;
    }
    if version != IPC_OLD as c_int { return true; }
    let mut old: msqid_ds = zeroed();
    if rust_msg_copy_from_user((&raw mut old).cast(), buf, size_of::<msqid_ds>()) != 0 { return true; }
    (*out).msg_perm.uid = old.msg_perm.uid as _;
    (*out).msg_perm.gid = old.msg_perm.gid as _;
    (*out).msg_perm.mode = old.msg_perm.mode as _;
    (*out).msg_qbytes = if old.msg_qbytes == 0 { old.msg_lqbytes } else { old.msg_qbytes as _ };
    false
}
unsafe fn msgctl_down(ns: *mut ipc_namespace, id: c_int, cmd: c_int, input: *mut ipc64_perm, qbytes: c_int) -> c_int {
    let sem = rust_msg_rwsem(rust_msg_ids(ns));
    down_write(sem);
    rust_msg_rcu_read_lock();
    let p = ipcctl_obtain_check(ns, rust_msg_ids(ns), id, cmd, input, qbytes);
    let result = if is_err(p) { ptr_err(p) } else {
        let q = queue(p);
        let security = rust_msg_security_ctl(perm(q), cmd);
        if security != 0 { security }
        else if cmd == IPC_RMID as c_int {
            rust_msg_lock(perm(q));
            freeque(ns, p); // releases both the object lock and RCU
            up_write(sem);
            return 0;
        } else if cmd == IPC_SET as c_int {
            if qbytes > *rust_msg_ctlmnb(ns) && !capable(CAP_SYS_RESOURCE as c_int) { -(EPERM as c_int) }
            else {
                let mut wake: wake_q_head = zeroed();
                rust_msg_wake_init(&raw mut wake);
                rust_msg_lock(perm(q));
                let result = ipc_update_perm(input, p);
                if result == 0 {
                    (*q).q_qbytes = qbytes as c_ulong;
                    (*q).q_ctime = ktime_get_real_seconds();
                    expunge_all(q, -(EAGAIN as c_int), &raw mut wake);
                    ss_wakeup(q, &raw mut wake, false);
                }
                rust_msg_unlock(perm(q));
                if result == 0 { wake_up_q(&raw mut wake); }
                result
            }
        } else { -(EINVAL as c_int) }
    };
    rust_msg_rcu_read_unlock();
    up_write(sem);
    result
}
unsafe fn msgctl_info(ns: *mut ipc_namespace, cmd: c_int, info: *mut msginfo) -> c_int {
    let result = rust_msg_security_ctl(null_mut(), cmd);
    if result != 0 { return result; }
    // Clear padding as well as fields before exposing this structure to userspace.
    core::ptr::write_bytes(info, 0, 1);
    (*info).msgmni = *rust_msg_ctlmni(ns);
    (*info).msgmax = *rust_msg_ctlmax(ns);
    (*info).msgmnb = *rust_msg_ctlmnb(ns);
    (*info).msgssz = MSGSSZ as c_int;
    (*info).msgseg = RUST_MSGSEG as _;
    let ids = rust_msg_ids(ns);
    down_read(rust_msg_rwsem(ids));
    if cmd == MSG_INFO as c_int { (*info).msgpool = rust_msg_in_use(ids); }
    let maxidx = rust_msg_maxidx(ids);
    up_read(rust_msg_rwsem(ids));
    if cmd == MSG_INFO as c_int {
        (*info).msgmap = rust_msg_counter_sum(rust_msg_hdrs(ns)) as c_int;
        (*info).msgtql = rust_msg_counter_sum(rust_msg_bytes(ns)) as c_int;
    } else {
        (*info).msgmap = MSGMAP as c_int;
        (*info).msgpool = MSGPOOL as c_int;
        (*info).msgtql = MSGTQL as c_int;
    }
    maxidx.max(0)
}
unsafe fn msgctl_stat(ns: *mut ipc_namespace, id: c_int, cmd: c_int, out: *mut msqid64_ds) -> c_int {
    core::ptr::write_bytes(out, 0, 1);
    rust_msg_rcu_read_lock();
    let q = if cmd == MSG_STAT as c_int || cmd == MSG_STAT_ANY as c_int {
        queue(ipc_obtain_object_idr(rust_msg_ids(ns), id))
    } else { obtain(ns, id) };
    let result = 'out: {
        if is_err(q) { break 'out ptr_err(q); }
        if cmd == MSG_STAT_ANY as c_int { rust_msg_audit(perm(q)); }
        else if ipcperms(ns, perm(q), S_IRUGO as _) != 0 { break 'out -(EACCES as c_int); }
        let security = rust_msg_security_ctl(perm(q), cmd);
        if security != 0 { break 'out security; }
        rust_msg_lock(perm(q));
        if !rust_msg_valid(perm(q)) {
            rust_msg_unlock(perm(q));
            break 'out -(EIDRM as c_int);
        }
        kernel_to_ipc64_perm(perm(q), addr_of_mut!((*out).msg_perm));
        (*out).msg_stime = (*q).q_stime as _;
        (*out).msg_rtime = (*q).q_rtime as _;
        (*out).msg_ctime = (*q).q_ctime as _;
        #[cfg(not(CONFIG_64BIT))] {
            (*out).msg_stime_high = ((*q).q_stime >> 32) as _;
            (*out).msg_rtime_high = ((*q).q_rtime >> 32) as _;
            (*out).msg_ctime_high = ((*q).q_ctime >> 32) as _;
        }
        (*out).msg_cbytes = (*q).q_cbytes;
        (*out).msg_qnum = (*q).q_qnum;
        (*out).msg_qbytes = (*q).q_qbytes;
        (*out).msg_lspid = rust_msg_pid_vnr((*q).q_lspid);
        (*out).msg_lrpid = rust_msg_pid_vnr((*q).q_lrpid);
        let result = if cmd == IPC_STAT as c_int { 0 } else { (*q).q_perm.id };
        rust_msg_unlock(perm(q));
        result
    };
    rust_msg_rcu_read_unlock();
    result
}
#[no_mangle]
pub unsafe extern "C" fn rust_ksys_msgctl(id: c_int, cmd: c_int, buf: *mut c_void, version: c_int) -> c_long {
    if id < 0 || cmd < 0 { return -(EINVAL as c_long); }
    let ns = rust_msg_current_ns();
    let mut out: msqid64_ds = zeroed();
    if cmd == IPC_INFO as c_int || cmd == MSG_INFO as c_int {
        let mut info: msginfo = zeroed();
        let result = msgctl_info(ns, cmd, &raw mut info);
        if result < 0 { return result as c_long; }
        if rust_msg_copy_to_user(buf, (&raw const info).cast(), size_of::<msginfo>()) != 0 { return -(EFAULT as c_long); }
        result as c_long
    } else if cmd == MSG_STAT as c_int || cmd == MSG_STAT_ANY as c_int || cmd == IPC_STAT as c_int {
        let result = msgctl_stat(ns, id, cmd, &raw mut out);
        if result < 0 { return result as c_long; }
        if copy_msqid_to_user(buf, &raw mut out, version) { return -(EFAULT as c_long); }
        result as c_long
    } else if cmd == IPC_SET as c_int {
        if copy_msqid_from_user(&raw mut out, buf, version) { return -(EFAULT as c_long); }
        msgctl_down(ns, id, cmd, &raw mut out.msg_perm, out.msg_qbytes as c_int) as c_long
    } else if cmd == IPC_RMID as c_int { msgctl_down(ns, id, cmd, null_mut(), 0) as c_long }
    else { -(EINVAL as c_long) }
}
#[cfg(CONFIG_ARCH_WANT_IPC_PARSE_VERSION)]
#[no_mangle]
pub unsafe extern "C" fn ksys_old_msgctl(id: c_int, mut cmd: c_int, buf: *mut msqid_ds) -> c_long {
    let version = ipc_parse_version(&raw mut cmd);
    rust_ksys_msgctl(id, cmd, buf.cast(), version)
}
#[cfg(CONFIG_COMPAT)]
unsafe fn copy_compat_msqid_from_user(out: *mut msqid64_ds, buf: *mut c_void, version: c_int) -> bool {
    core::ptr::write_bytes(out, 0, 1);
    if version == IPC_64 as c_int {
        let p = buf.cast::<compat_msqid64_ds>();
        if get_compat_ipc64_perm(addr_of_mut!((*out).msg_perm), addr_of_mut!((*p).msg_perm)) != 0 { return true; }
        let mut qbytes: u32 = 0;
        if rust_msg_get_u32(addr_of_mut!((*p).msg_qbytes).cast(), &raw mut qbytes) != 0 { return true; }
        (*out).msg_qbytes = qbytes as _;
    } else {
        let p = buf.cast::<compat_msqid_ds>();
        if get_compat_ipc_perm(addr_of_mut!((*out).msg_perm), addr_of_mut!((*p).msg_perm)) != 0 { return true; }
        let mut qbytes: u16 = 0;
        if rust_msg_get_u16(addr_of_mut!((*p).msg_qbytes), &raw mut qbytes) != 0 { return true; }
        (*out).msg_qbytes = qbytes as _;
    }
    false
}
#[cfg(CONFIG_COMPAT)]
unsafe fn copy_compat_msqid_to_user(buf: *mut c_void, input: *mut msqid64_ds, version: c_int) -> bool {
    if version == IPC_64 as c_int {
        let mut out: compat_msqid64_ds = zeroed();
        to_compat_ipc64_perm(&raw mut out.msg_perm, addr_of_mut!((*input).msg_perm));
        out.msg_stime = (*input).msg_stime as _;
        out.msg_stime_high = (((*input).msg_stime as u64) >> 32) as _;
        out.msg_rtime = (*input).msg_rtime as _;
        out.msg_rtime_high = (((*input).msg_rtime as u64) >> 32) as _;
        out.msg_ctime = (*input).msg_ctime as _;
        out.msg_ctime_high = (((*input).msg_ctime as u64) >> 32) as _;
        out.msg_cbytes = (*input).msg_cbytes as _;
        out.msg_qnum = (*input).msg_qnum as _;
        out.msg_qbytes = (*input).msg_qbytes as _;
        out.msg_lspid = (*input).msg_lspid;
        out.msg_lrpid = (*input).msg_lrpid;
        rust_msg_copy_to_user(buf, (&raw const out).cast(), size_of::<compat_msqid64_ds>()) != 0
    } else {
        let mut out: compat_msqid_ds = zeroed();
        to_compat_ipc_perm(&raw mut out.msg_perm, addr_of_mut!((*input).msg_perm));
        out.msg_stime = (*input).msg_stime as _;
        out.msg_rtime = (*input).msg_rtime as _;
        out.msg_ctime = (*input).msg_ctime as _;
        out.msg_cbytes = (*input).msg_cbytes as _;
        out.msg_qnum = (*input).msg_qnum as _;
        out.msg_qbytes = (*input).msg_qbytes as _;
        out.msg_lspid = (*input).msg_lspid as _;
        out.msg_lrpid = (*input).msg_lrpid as _;
        rust_msg_copy_to_user(buf, (&raw const out).cast(), size_of::<compat_msqid_ds>()) != 0
    }
}
#[cfg(CONFIG_COMPAT)]
#[no_mangle]
pub unsafe extern "C" fn rust_compat_ksys_msgctl(id: c_int, cmd: c_int, buf: *mut c_void, version: c_int) -> c_long {
    let ns = rust_msg_current_ns();
    if id < 0 || cmd < 0 { return -(EINVAL as c_long); }
    let mut out: msqid64_ds = zeroed();
    let operation = cmd & !(IPC_64 as c_int);
    // Preserve the original cmd passed to LSM hooks and helpers.
    if operation == IPC_INFO as c_int || operation == MSG_INFO as c_int {
        let mut info: msginfo = zeroed();
        let result = msgctl_info(ns, cmd, &raw mut info);
        if result < 0 { return result as c_long; }
        if rust_msg_copy_to_user(buf, (&raw const info).cast(), size_of::<msginfo>()) != 0 { return -(EFAULT as c_long); }
        result as c_long
    } else if operation == MSG_STAT as c_int || operation == MSG_STAT_ANY as c_int || operation == IPC_STAT as c_int {
        let result = msgctl_stat(ns, id, cmd, &raw mut out);
        if result < 0 { return result as c_long; }
        if copy_compat_msqid_to_user(buf, &raw mut out, version) { return -(EFAULT as c_long); }
        result as c_long
    } else if operation == IPC_SET as c_int {
        if copy_compat_msqid_from_user(&raw mut out, buf, version) { return -(EFAULT as c_long); }
        msgctl_down(ns, id, cmd, &raw mut out.msg_perm, out.msg_qbytes as c_int) as c_long
    } else if operation == IPC_RMID as c_int { msgctl_down(ns, id, cmd, null_mut(), 0) as c_long }
    else { -(EINVAL as c_long) }
}
#[cfg(all(CONFIG_COMPAT, CONFIG_ARCH_WANT_COMPAT_IPC_PARSE_VERSION))]
#[no_mangle]
pub unsafe extern "C" fn compat_ksys_old_msgctl(id: c_int, mut cmd: c_int, buf: *mut c_void) -> c_long {
    let version = rust_msg_compat_parse_version(&raw mut cmd);
    rust_compat_ksys_msgctl(id, cmd, buf, version)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
