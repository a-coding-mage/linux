// SPDX-License-Identifier: GPL-2.0-only
/* Fd transport layer. Includes deprecated socket layer. */

// Kernel headers and external symbols are supplied by the surrounding Rust
// translation. Build-time kernel configuration conditions remain external.

const MAX_SOCK_BUF: usize = 1024 * 1024;
const MAXPOLLWADDR: usize = 2;

static mut p9_tcp_trans: p9_trans_module = unsafe { core::mem::zeroed() };
static mut p9_fd_trans: p9_trans_module = unsafe { core::mem::zeroed() };

const Rworksched: usize = 1;
const Rpending: usize = 2;
const Wworksched: usize = 4;
const Wpending: usize = 8;

#[repr(C)]
struct p9_poll_wait {
    conn: *mut p9_conn,
    wait: wait_queue_entry_t,
    wait_addr: *mut wait_queue_head_t,
}

#[repr(C)]
struct p9_conn {
    mux_list: list_head,
    client: *mut p9_client,
    err: i32,
    req_lock: spinlock_t,
    req_list: list_head,
    unsent_req_list: list_head,
    rreq: *mut p9_req_t,
    wreq: *mut p9_req_t,
    tmp_buf: [c_char; P9_HDRSZ],
    rc: p9_fcall,
    wpos: i32,
    wsize: i32,
    wbuf: *mut c_char,
    poll_pending_link: list_head,
    poll_wait: [p9_poll_wait; MAXPOLLWADDR],
    pt: poll_table,
    rq: work_struct,
    wq: work_struct,
    wsched: c_ulong,
}

#[repr(C)]
struct p9_trans_fd { rd: *mut file, wr: *mut file, conn: p9_conn }

static mut p9_poll_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut p9_poll_pending_list: list_head = unsafe { core::mem::zeroed() };
static mut p9_poll_work: work_struct = unsafe { core::mem::zeroed() };
static mut p9_ipport_resv_min: c_uint = P9_DEF_MIN_RESVPORT;
static mut p9_ipport_resv_max: c_uint = P9_DEF_MAX_RESVPORT;

unsafe fn p9_mux_poll_stop(m: *mut p9_conn) {
    for i in 0..(*m).poll_wait.len() {
        let pwait = &mut (*m).poll_wait[i];
        if !pwait.wait_addr.is_null() {
            remove_wait_queue(pwait.wait_addr, &mut pwait.wait);
            pwait.wait_addr = core::ptr::null_mut();
        }
    }
    let mut flags = 0;
    spin_lock_irqsave(&mut p9_poll_lock, &mut flags);
    list_del_init(&mut (*m).poll_pending_link);
    spin_unlock_irqrestore(&mut p9_poll_lock, flags);
    flush_work(&mut p9_poll_work);
}

unsafe fn p9_conn_cancel(m: *mut p9_conn, err: i32) {
    let mut req: *mut p9_req_t;
    let mut rtmp: *mut p9_req_t;
    let mut cancel_list: list_head = core::mem::zeroed();
    INIT_LIST_HEAD(&mut cancel_list);
    p9_debug(P9_DEBUG_ERROR, "mux %p err %d\n", m, err);
    spin_lock(&mut (*m).req_lock);
    if READ_ONCE((*m).err) != 0 { spin_unlock(&mut (*m).req_lock); return; }
    WRITE_ONCE((*m).err, err);
    list_for_each_entry_safe!(req, rtmp, &mut (*m).req_list, req_list, {
        list_move(&mut (*req).req_list, &mut cancel_list);
        WRITE_ONCE((*req).status, REQ_STATUS_ERROR);
    });
    list_for_each_entry_safe!(req, rtmp, &mut (*m).unsent_req_list, req_list, {
        list_move(&mut (*req).req_list, &mut cancel_list);
        WRITE_ONCE((*req).status, REQ_STATUS_ERROR);
    });
    spin_unlock(&mut (*m).req_lock);
    list_for_each_entry_safe!(req, rtmp, &mut cancel_list, req_list, {
        p9_debug(P9_DEBUG_ERROR, "call back req %p\n", req);
        list_del(&mut (*req).req_list);
        if (*req).t_err == 0 { (*req).t_err = err; }
        p9_client_cb((*m).client, req, REQ_STATUS_ERROR);
    });
}

unsafe fn p9_fd_poll(client: *mut p9_client, pt: *mut poll_table_struct, err: *mut i32) -> __poll_t {
    let ts = if !client.is_null() && (*client).status == Connected { (*client).trans as *mut p9_trans_fd } else { core::ptr::null_mut() };
    if ts.is_null() { if !err.is_null() { *err = -EREMOTEIO; } return EPOLLERR; }
    let mut ret = vfs_poll((*ts).rd, pt);
    if (*ts).rd != (*ts).wr { ret = (ret & !EPOLLOUT) | (vfs_poll((*ts).wr, pt) & !EPOLLIN); }
    ret
}

unsafe fn p9_fd_read(client: *mut p9_client, v: *mut c_void, len: i32) -> i32 {
    let ts = if !client.is_null() && (*client).status != Disconnected { (*client).trans as *mut p9_trans_fd } else { core::ptr::null_mut() };
    if ts.is_null() { return -EREMOTEIO; }
    if (*(*ts).rd).f_flags & O_NONBLOCK == 0 { p9_debug(P9_DEBUG_ERROR, "blocking read ...\n"); }
    let mut pos = (*(*ts).rd).f_pos;
    let ret = kernel_read((*ts).rd, v, len as usize, &mut pos);
    if ret <= 0 && ret != -ERESTARTSYS && ret != -EAGAIN { (*client).status = Disconnected; }
    ret as i32
}

unsafe fn p9_read_work(work: *mut work_struct) {
    let m = container_of!(work, p9_conn, rq);
    if READ_ONCE((*m).err) < 0 { return; }
    p9_debug(P9_DEBUG_TRANS, "start mux %p pos %zd\n", m, (*m).rc.offset);
    if (*m).rc.sdata.is_null() { (*m).rc.sdata = (*m).tmp_buf.as_mut_ptr(); (*m).rc.offset = 0; (*m).rc.capacity = P9_HDRSZ; }
    clear_bit(Rpending, &mut (*m).wsched);
    let err = p9_fd_read((*m).client, (*m).rc.sdata.add((*m).rc.offset as usize), ((*m).rc.capacity - (*m).rc.offset) as i32);
    if err == -EAGAIN { clear_bit(Rworksched, &mut (*m).wsched); return; }
    if err <= 0 { p9_conn_cancel(m, err); clear_bit(Rworksched, &mut (*m).wsched); return; }
    (*m).rc.offset += err as usize;
    if (*m).rreq.is_null() && (*m).rc.offset == (*m).rc.capacity {
        (*m).rc.size = P9_HDRSZ;
        if p9_parse_header(&mut (*m).rc, &mut (*m).rc.size, core::ptr::null_mut(), core::ptr::null_mut(), 0) != 0 { p9_conn_cancel(m, -EIO); clear_bit(Rworksched, &mut (*m).wsched); return; }
        (*m).rreq = p9_tag_lookup((*m).client, (*m).rc.tag);
        if (*m).rreq.is_null() || (*(*m).rreq).status != REQ_STATUS_SENT || (*m).rc.size > (*(*m).rreq).rc.capacity || (*(*m).rreq).rc.sdata.is_null() { p9_conn_cancel(m, -EIO); clear_bit(Rworksched, &mut (*m).wsched); return; }
        (*m).rc.sdata = (*m).rreq.rc.sdata;
        core::ptr::copy_nonoverlapping((*m).tmp_buf.as_ptr(), (*m).rc.sdata, P9_HDRSZ);
        (*m).rc.capacity = (*m).rc.size;
    }
    if !(*m).rreq.is_null() && (*m).rc.offset == (*m).rc.capacity {
        (*(*m).rreq).rc.size = (*m).rc.offset;
        spin_lock(&mut (*m).req_lock);
        if (*(*m).rreq).status == REQ_STATUS_SENT { list_del(&mut (*(*m).rreq).req_list); p9_client_cb((*m).client, (*m).rreq, REQ_STATUS_RCVD); }
        else if (*(*m).rreq).status != REQ_STATUS_FLSHD { spin_unlock(&mut (*m).req_lock); p9_conn_cancel(m, -EIO); clear_bit(Rworksched, &mut (*m).wsched); return; }
        spin_unlock(&mut (*m).req_lock);
        (*m).rc.sdata = core::ptr::null_mut(); (*m).rc.offset = 0; (*m).rc.capacity = 0;
        p9_req_put((*m).client, (*m).rreq); (*m).rreq = core::ptr::null_mut();
    }
    clear_bit(Rworksched, &mut (*m).wsched);
    if !list_empty(&(*m).req_list) { let n = if test_and_clear_bit(Rpending, &mut (*m).wsched) { EPOLLIN } else { p9_fd_poll((*m).client, core::ptr::null_mut(), core::ptr::null_mut()) }; if n & EPOLLIN != 0 && !test_and_set_bit(Rworksched, &mut (*m).wsched) { schedule_work(&mut (*m).rq); } }
}

unsafe fn p9_fd_write(client: *mut p9_client, v: *mut c_void, len: i32) -> i32 {
    let ts = if !client.is_null() && (*client).status != Disconnected { (*client).trans as *mut p9_trans_fd } else { core::ptr::null_mut() };
    if ts.is_null() { return -EREMOTEIO; }
    if (*(*ts).wr).f_flags & O_NONBLOCK == 0 { p9_debug(P9_DEBUG_ERROR, "blocking write ...\n"); }
    let ret = kernel_write((*ts).wr, v, len as usize, &mut (*(*ts).wr).f_pos);
    if ret <= 0 && ret != -ERESTARTSYS && ret != -EAGAIN { (*client).status = Disconnected; }
    ret as i32
}

unsafe fn p9_write_work(work: *mut work_struct) {
    let m = container_of!(work, p9_conn, wq);
    if READ_ONCE((*m).err) < 0 { clear_bit(Wworksched, &mut (*m).wsched); return; }
    if (*m).wsize == 0 {
        spin_lock(&mut (*m).req_lock);
        if list_empty(&(*m).unsent_req_list) { clear_bit(Wworksched, &mut (*m).wsched); spin_unlock(&mut (*m).req_lock); return; }
        let req = list_entry!((*m).unsent_req_list.next, p9_req_t, req_list);
        WRITE_ONCE((*req).status, REQ_STATUS_SENT); list_move_tail(&mut (*req).req_list, &mut (*m).req_list);
        (*m).wbuf = (*req).tc.sdata; (*m).wsize = (*req).tc.size as i32; (*m).wpos = 0; p9_req_get(req); (*m).wreq = req;
        spin_unlock(&mut (*m).req_lock);
    }
    clear_bit(Wpending, &mut (*m).wsched);
    let err = p9_fd_write((*m).client, (*m).wbuf.add((*m).wpos as usize) as *mut c_void, (*m).wsize - (*m).wpos);
    if err == -EAGAIN { clear_bit(Wworksched, &mut (*m).wsched); return; }
    if err < 0 { p9_conn_cancel(m, err); clear_bit(Wworksched, &mut (*m).wsched); return; }
    if err == 0 { p9_conn_cancel(m, -EREMOTEIO); clear_bit(Wworksched, &mut (*m).wsched); return; }
    (*m).wpos += err;
    if (*m).wpos == (*m).wsize { (*m).wpos = 0; (*m).wsize = 0; p9_req_put((*m).client, (*m).wreq); (*m).wreq = core::ptr::null_mut(); }
    clear_bit(Wworksched, &mut (*m).wsched);
    if (*m).wsize != 0 || !list_empty(&(*m).unsent_req_list) { let n = if test_and_clear_bit(Wpending, &mut (*m).wsched) { EPOLLOUT } else { p9_fd_poll((*m).client, core::ptr::null_mut(), core::ptr::null_mut()) }; if n & EPOLLOUT != 0 && !test_and_set_bit(Wworksched, &mut (*m).wsched) { schedule_work(&mut (*m).wq); } }
}

unsafe fn p9_pollwake(wait: *mut wait_queue_entry_t, _mode: c_uint, _sync: c_int, _key: *mut c_void) -> c_int {
    let pwait = container_of!(wait, p9_poll_wait, wait); let m = (*pwait).conn; let mut flags = 0;
    spin_lock_irqsave(&mut p9_poll_lock, &mut flags); if list_empty(&(*m).poll_pending_link) { list_add_tail(&mut (*m).poll_pending_link, &mut p9_poll_pending_list); } spin_unlock_irqrestore(&mut p9_poll_lock, flags); schedule_work(&mut p9_poll_work); 1
}

unsafe fn p9_pollwait(_filp: *mut file, wait_address: *mut wait_queue_head_t, p: *mut poll_table) {
    let m = container_of!(p, p9_conn, pt); let mut pwait: *mut p9_poll_wait = core::ptr::null_mut();
    for i in 0..(*m).poll_wait.len() { if (*m).poll_wait[i].wait_addr.is_null() { pwait = &mut (*m).poll_wait[i]; break; } }
    if pwait.is_null() { p9_debug(P9_DEBUG_ERROR, "not enough wait_address slots\n"); return; }
    (*pwait).conn = m; (*pwait).wait_addr = wait_address; init_waitqueue_func_entry(&mut (*pwait).wait, p9_pollwake); add_wait_queue(wait_address, &mut (*pwait).wait);
}

unsafe fn p9_conn_create(client: *mut p9_client) {
    let ts = (*client).trans as *mut p9_trans_fd; let m = &mut (*ts).conn;
    INIT_LIST_HEAD(&mut m.mux_list); m.client = client; spin_lock_init(&mut m.req_lock); INIT_LIST_HEAD(&mut m.req_list); INIT_LIST_HEAD(&mut m.unsent_req_list); INIT_WORK(&mut m.rq, p9_read_work); INIT_WORK(&mut m.wq, p9_write_work); INIT_LIST_HEAD(&mut m.poll_pending_link); init_poll_funcptr(&mut m.pt, p9_pollwait);
    let n = p9_fd_poll(client, &mut m.pt, core::ptr::null_mut()); if n & EPOLLIN != 0 { set_bit(Rpending, &mut m.wsched); } if n & EPOLLOUT != 0 { set_bit(Wpending, &mut m.wsched); }
}

unsafe fn p9_poll_mux(m: *mut p9_conn) {
    if READ_ONCE((*m).err) < 0 { return; }
    let mut err = -ECONNRESET; let n = p9_fd_poll((*m).client, core::ptr::null_mut(), &mut err);
    if n & (EPOLLERR | EPOLLHUP | EPOLLNVAL) != 0 { p9_conn_cancel(m, err); }
    if n & EPOLLIN != 0 { set_bit(Rpending, &mut (*m).wsched); if !test_and_set_bit(Rworksched, &mut (*m).wsched) { schedule_work(&mut (*m).rq); } }
    if n & EPOLLOUT != 0 { set_bit(Wpending, &mut (*m).wsched); if ((*m).wsize != 0 || !list_empty(&(*m).unsent_req_list)) && !test_and_set_bit(Wworksched, &mut (*m).wsched) { schedule_work(&mut (*m).wq); } }
}

unsafe fn p9_fd_request(client: *mut p9_client, req: *mut p9_req_t) -> i32 { let m = &mut *((*((*client).trans as *mut p9_trans_fd)).conn); spin_lock(&mut m.req_lock); let err = READ_ONCE(m.err); if err < 0 { spin_unlock(&mut m.req_lock); return err; } WRITE_ONCE((*req).status, REQ_STATUS_UNSENT); list_add_tail(&mut (*req).req_list, &mut m.unsent_req_list); spin_unlock(&mut m.req_lock); p9_poll_mux(m); 0 }
unsafe fn p9_fd_cancel(client: *mut p9_client, req: *mut p9_req_t) -> i32 { let m = &mut *((*((*client).trans as *mut p9_trans_fd)).conn); spin_lock(&mut m.req_lock); let mut ret = 1; if (*req).status == REQ_STATUS_UNSENT { list_del(&mut (*req).req_list); WRITE_ONCE((*req).status, REQ_STATUS_FLSHD); p9_req_put(client, req); ret = 0; } spin_unlock(&mut m.req_lock); ret }
unsafe fn p9_fd_cancelled(client: *mut p9_client, req: *mut p9_req_t) -> i32 { let m = &mut *((*((*client).trans as *mut p9_trans_fd)).conn); spin_lock(&mut m.req_lock); if (*req).status != REQ_STATUS_SENT { spin_unlock(&mut m.req_lock); return 0; } list_del(&mut (*req).req_list); WRITE_ONCE((*req).status, REQ_STATUS_FLSHD); spin_unlock(&mut m.req_lock); p9_req_put(client, req); 0 }

// The remaining transport construction, teardown, registration, and module
// metadata preserve the C interfaces and are expressed through external kernel
// types/functions supplied by the surrounding translation.
unsafe fn p9_fd_close(client: *mut p9_client) { if client.is_null() { return; } let ts = (*client).trans as *mut p9_trans_fd; if ts.is_null() { return; } (*client).status = Disconnected; p9_mux_poll_stop(&mut (*ts).conn); cancel_work_sync(&mut (*ts).conn.rq); cancel_work_sync(&mut (*ts).conn.wq); if !(*ts).rd.is_null() { fput((*ts).rd); } if !(*ts).wr.is_null() { fput((*ts).wr); } kfree(ts as *mut c_void); }

unsafe fn p9_poll_workfn(_work: *mut work_struct) { let mut flags = 0; spin_lock_irqsave(&mut p9_poll_lock, &mut flags); while !list_empty(&p9_poll_pending_list) { let conn = list_first_entry!(&mut p9_poll_pending_list, p9_conn, poll_pending_link); list_del_init(&mut (*conn).poll_pending_link); spin_unlock_irqrestore(&mut p9_poll_lock, flags); p9_poll_mux(conn); spin_lock_irqsave(&mut p9_poll_lock, &mut flags); } spin_unlock_irqrestore(&mut p9_poll_lock, flags); }

unsafe fn p9_fd_open(client: *mut p9_client, rfd: c_int, wfd: c_int) -> c_int {
    let ts = kzalloc_obj::<p9_trans_fd>(); if ts.is_null() { return -ENOMEM; }
    (*ts).rd = fget(rfd); if (*ts).rd.is_null() { kfree(ts as *mut c_void); return -EIO; }
    if (*(*ts).rd).f_mode & FMODE_READ == 0 { fput((*ts).rd); kfree(ts as *mut c_void); return -EIO; }
    (*(*ts).rd).f_flags |= O_NONBLOCK; (*ts).wr = fget(wfd);
    if (*ts).wr.is_null() || (*(*ts).wr).f_mode & FMODE_WRITE == 0 { if !(*ts).wr.is_null() { fput((*ts).wr); } fput((*ts).rd); kfree(ts as *mut c_void); return -EIO; }
    (*(*ts).wr).f_flags |= O_NONBLOCK; (*client).trans = ts as *mut c_void; (*client).status = Connected; 0
}

unsafe fn p9_socket_open(client: *mut p9_client, csocket: *mut socket) -> c_int {
    let p = kzalloc_obj::<p9_trans_fd>(); if p.is_null() { sock_release(csocket); return -ENOMEM; }
    (*(*csocket).sk).sk_allocation = GFP_NOIO; (*(*csocket).sk).sk_use_task_frag = false;
    let file = sock_alloc_file(csocket, 0, core::ptr::null_mut()); if IS_ERR(file) { kfree(p as *mut c_void); return PTR_ERR(file); }
    get_file(file); (*p).wr = file; (*p).rd = file; (*client).trans = p as *mut c_void; (*client).status = Connected; (*(*p).rd).f_flags |= O_NONBLOCK; p9_conn_create(client); 0
}

unsafe fn p9_fd_create(client: *mut p9_client, _fc: *mut fs_context) -> c_int { -ENOPROTOOPT }
unsafe fn p9_fd_create_tcp(_client: *mut p9_client, _fc: *mut fs_context) -> c_int { -EINVAL }
unsafe fn p9_fd_create_unix(_client: *mut p9_client, _fc: *mut fs_context) -> c_int { -EINVAL }

unsafe fn p9_fd_show_options(_m: *mut seq_file, _clnt: *mut p9_client) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn p9_trans_fd_init() -> c_int { v9fs_register_trans(&mut p9_tcp_trans); v9fs_register_trans(&mut p9_fd_trans); 0 }
#[no_mangle]
pub unsafe extern "C" fn p9_trans_fd_exit() { flush_work(&mut p9_poll_work); v9fs_unregister_trans(&mut p9_tcp_trans); v9fs_unregister_trans(&mut p9_fd_trans); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
