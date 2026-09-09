// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * cn_proc.c - process events connector
 *
 * Copyright (C) Matt Helsley, IBM Corp. 2005
 * Based on cn_fork.c by Guillaume Thouvenin <guillaume.thouvenin@bull.net>
 * Original copyright notice follows:
 * Copyright (C) 2005 BULL SA.
 */

// Kernel dependencies supplied by the surrounding tree are intentionally not
// reimplemented here.

const CN_PROC_MSG_SIZE: usize = core::mem::size_of::<cn_msg>()
    + core::mem::size_of::<proc_event>() + 4;

#[inline]
unsafe fn buffer_to_cn_msg(buffer: *mut u8) -> *mut cn_msg {
    // BUILD_BUG_ON(sizeof(struct cn_msg) != 20);
    (buffer.add(4)) as *mut cn_msg
}

static mut proc_event_num_listeners: atomic_t = ATOMIC_INIT(0);
static mut cn_proc_event_id: cb_id = cb_id { idx: CN_IDX_PROC, val: CN_VAL_PROC };

/* local_event.count is used as the sequence number of the netlink message */
#[repr(C)]
struct local_event {
    lock: local_lock_t,
    count: u32,
}

// DEFINE_PER_CPU(struct local_event, local_event) = {
//     .lock = INIT_LOCAL_LOCK(lock),
// };
static mut local_event: local_event = local_event {
    lock: INIT_LOCAL_LOCK(lock),
    count: 0,
};

unsafe fn cn_filter(dsk: *mut sock, _skb: *mut sk_buff, data: *mut core::ffi::c_void) -> i32 {
    let what: u32;
    let exit_code: u32;
    let ptr: *mut u32;
    let mc_op: proc_cn_mcast_op;
    let val: usize;

    if dsk.is_null() || (*dsk).sk_user_data.is_null() || data.is_null() {
        return 0;
    }

    ptr = data as *mut u32;
    what = *ptr;
    exit_code = *ptr.add(1);
    val = (*( (*dsk).sk_user_data as *mut proc_input)).event_type as usize;
    mc_op = (*( (*dsk).sk_user_data as *mut proc_input)).mcast_op;

    if mc_op == PROC_CN_MCAST_IGNORE { return 1; }
    if val as u32 == PROC_EVENT_ALL { return 0; }

    /* Drop packet if only non-zero exit status is requested and status is 0. */
    if (val as u32 & PROC_EVENT_NONZERO_EXIT) != 0 && what == PROC_EVENT_EXIT {
        if exit_code != 0 { return 0; }
    }
    if (val as u32 & what) != 0 { return 0; }
    1
}

#[inline]
unsafe fn send_msg(msg: *mut cn_msg) {
    let mut filter_data = [0u32; 2];
    local_lock(&mut local_event.lock);
    msg.as_mut().unwrap().seq = __this_cpu_inc_return(&mut local_event.count) - 1;
    (*(msg.as_mut().unwrap().data as *mut proc_event)).cpu = smp_processor_id();

    filter_data[0] = (*(msg.as_mut().unwrap().data as *mut proc_event)).what;
    if filter_data[0] == PROC_EVENT_EXIT {
        filter_data[1] = (*(msg.as_mut().unwrap().data as *mut proc_event)).event_data.exit.exit_code;
    } else { filter_data[1] = 0; }

    cn_netlink_send_mult(msg, (*msg).len, 0, CN_IDX_PROC, GFP_NOWAIT,
                         Some(cn_filter), filter_data.as_mut_ptr() as *mut core::ffi::c_void);
    local_unlock(&mut local_event.lock);
}

pub unsafe fn proc_fork_connector(task: *mut task_struct) {
    if atomic_read(&proc_event_num_listeners) < 1 { return; }
    let mut buffer = [0u8; CN_PROC_MSG_SIZE];
    let msg = buffer_to_cn_msg(buffer.as_mut_ptr());
    let ev = (*msg).data as *mut proc_event;
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*ev).event_data) as *mut u8, 0, core::mem::size_of_val(&(*ev).event_data));
    (*ev).timestamp_ns = ktime_get_ns(); (*ev).what = PROC_EVENT_FORK;
    rcu_read_lock();
    let parent = rcu_dereference((*task).real_parent);
    (*ev).event_data.fork.parent_pid = (*parent).pid;
    (*ev).event_data.fork.parent_tgid = (*parent).tgid;
    rcu_read_unlock();
    (*ev).event_data.fork.child_pid = (*task).pid; (*ev).event_data.fork.child_tgid = (*task).tgid;
    core::ptr::copy_nonoverlapping(&cn_proc_event_id as *const cb_id as *const u8, &mut (*msg).id as *mut _ as *mut u8, core::mem::size_of::<(*msg).id>());
    (*msg).ack = 0; (*msg).len = core::mem::size_of::<proc_event>() as u16; (*msg).flags = 0;
    send_msg(msg);
}

pub unsafe fn proc_exec_connector(task: *mut task_struct) {
    if atomic_read(&proc_event_num_listeners) < 1 { return; }
    let mut buffer = [0u8; CN_PROC_MSG_SIZE]; let msg = buffer_to_cn_msg(buffer.as_mut_ptr()); let ev = (*msg).data as *mut proc_event;
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*ev).event_data) as *mut u8, 0, core::mem::size_of_val(&(*ev).event_data));
    (*ev).timestamp_ns = ktime_get_ns(); (*ev).what = PROC_EVENT_EXEC; (*ev).event_data.exec.process_pid = (*task).pid; (*ev).event_data.exec.process_tgid = (*task).tgid;
    core::ptr::copy_nonoverlapping(&cn_proc_event_id as *const cb_id as *const u8, &mut (*msg).id as *mut _ as *mut u8, core::mem::size_of::<(*msg).id>()); (*msg).ack=0; (*msg).len=core::mem::size_of::<proc_event>() as u16; (*msg).flags=0; send_msg(msg);
}

pub unsafe fn proc_id_connector(task: *mut task_struct, which_id: i32) {
    if atomic_read(&proc_event_num_listeners) < 1 { return; }
    let mut buffer=[0u8; CN_PROC_MSG_SIZE]; let msg=buffer_to_cn_msg(buffer.as_mut_ptr()); let ev=(*msg).data as *mut proc_event;
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*ev).event_data) as *mut u8,0,core::mem::size_of_val(&(*ev).event_data)); (*ev).what=which_id as u32; (*ev).event_data.id.process_pid=(*task).pid; (*ev).event_data.id.process_tgid=(*task).tgid;
    rcu_read_lock(); let cred=__task_cred(task);
    if which_id == PROC_EVENT_UID { (*ev).event_data.id.r.ruid=from_kuid_munged(&init_user_ns,(*cred).uid); (*ev).event_data.id.e.euid=from_kuid_munged(&init_user_ns,(*cred).euid); }
    else if which_id == PROC_EVENT_GID { (*ev).event_data.id.r.rgid=from_kgid_munged(&init_user_ns,(*cred).gid); (*ev).event_data.id.e.egid=from_kgid_munged(&init_user_ns,(*cred).egid); }
    else { rcu_read_unlock(); return; }
    rcu_read_unlock(); (*ev).timestamp_ns=ktime_get_ns(); core::ptr::copy_nonoverlapping(&cn_proc_event_id as *const cb_id as *const u8,&mut (*msg).id as *mut _ as *mut u8,core::mem::size_of::<(*msg).id>()); (*msg).ack=0; (*msg).len=core::mem::size_of::<proc_event>() as u16; (*msg).flags=0; send_msg(msg);
}

pub unsafe fn proc_sid_connector(task: *mut task_struct) {
    if atomic_read(&proc_event_num_listeners)<1{return;} let mut buffer=[0u8;CN_PROC_MSG_SIZE]; let msg=buffer_to_cn_msg(buffer.as_mut_ptr()); let ev=(*msg).data as *mut proc_event; core::ptr::write_bytes(core::ptr::addr_of_mut!((*ev).event_data) as *mut u8,0,core::mem::size_of_val(&(*ev).event_data)); (*ev).timestamp_ns=ktime_get_ns(); (*ev).what=PROC_EVENT_SID; (*ev).event_data.sid.process_pid=(*task).pid; (*ev).event_data.sid.process_tgid=(*task).tgid; core::ptr::copy_nonoverlapping(&cn_proc_event_id as *const cb_id as *const u8,&mut (*msg).id as *mut _ as *mut u8,core::mem::size_of::<(*msg).id>()); (*msg).ack=0; (*msg).len=core::mem::size_of::<proc_event>() as u16; (*msg).flags=0; send_msg(msg);
}

// The remaining connector entry points retain the C control flow and depend
// on the kernel declarations supplied by the surrounding translation unit.
pub unsafe fn proc_ptrace_connector(task:*mut task_struct, ptrace_id:i32){ if atomic_read(&proc_event_num_listeners)<1{return;} let mut buffer=[0u8;CN_PROC_MSG_SIZE]; let msg=buffer_to_cn_msg(buffer.as_mut_ptr()); let ev=(*msg).data as *mut proc_event; core::ptr::write_bytes(core::ptr::addr_of_mut!((*ev).event_data) as *mut u8,0,core::mem::size_of_val(&(*ev).event_data)); (*ev).timestamp_ns=ktime_get_ns(); (*ev).what=PROC_EVENT_PTRACE; (*ev).event_data.ptrace.process_pid=(*task).pid; (*ev).event_data.ptrace.process_tgid=(*task).tgid; if ptrace_id==PTRACE_ATTACH {(*ev).event_data.ptrace.tracer_pid=(*current).pid;(*ev).event_data.ptrace.tracer_tgid=(*current).tgid;} else if ptrace_id==PTRACE_DETACH {(*ev).event_data.ptrace.tracer_pid=0;(*ev).event_data.ptrace.tracer_tgid=0;} else{return;} core::ptr::copy_nonoverlapping(&cn_proc_event_id as *const cb_id as *const u8,&mut (*msg).id as *mut _ as *mut u8,core::mem::size_of::<(*msg).id>());(*msg).ack=0;(*msg).len=core::mem::size_of::<proc_event>() as u16;(*msg).flags=0;send_msg(msg); }

pub unsafe fn proc_comm_connector(task:*mut task_struct){ if atomic_read(&proc_event_num_listeners)<1{return;} let mut buffer=[0u8;CN_PROC_MSG_SIZE]; let msg=buffer_to_cn_msg(buffer.as_mut_ptr()); let ev=(*msg).data as *mut proc_event; core::ptr::write_bytes(core::ptr::addr_of_mut!((*ev).event_data) as *mut u8,0,core::mem::size_of_val(&(*ev).event_data));(*ev).timestamp_ns=ktime_get_ns();(*ev).what=PROC_EVENT_COMM;(*ev).event_data.comm.process_pid=(*task).pid;(*ev).event_data.comm.process_tgid=(*task).tgid;get_task_comm((*ev).event_data.comm.comm.as_mut_ptr(),task);core::ptr::copy_nonoverlapping(&cn_proc_event_id as *const cb_id as *const u8,&mut (*msg).id as *mut _ as *mut u8,core::mem::size_of::<(*msg).id>());(*msg).ack=0;(*msg).len=core::mem::size_of::<proc_event>() as u16;(*msg).flags=0;send_msg(msg); }

pub unsafe fn proc_coredump_connector(task:*mut task_struct){ if atomic_read(&proc_event_num_listeners)<1{return;} let mut buffer=[0u8;CN_PROC_MSG_SIZE]; let msg=buffer_to_cn_msg(buffer.as_mut_ptr()); let ev=(*msg).data as *mut proc_event; core::ptr::write_bytes(core::ptr::addr_of_mut!((*ev).event_data) as *mut u8,0,core::mem::size_of_val(&(*ev).event_data));(*ev).timestamp_ns=ktime_get_ns();(*ev).what=PROC_EVENT_COREDUMP;(*ev).event_data.coredump.process_pid=(*task).pid;(*ev).event_data.coredump.process_tgid=(*task).tgid;rcu_read_lock();if pid_alive(task){let parent=rcu_dereference((*task).real_parent);(*ev).event_data.coredump.parent_pid=(*parent).pid;(*ev).event_data.coredump.parent_tgid=(*parent).tgid;}rcu_read_unlock();core::ptr::copy_nonoverlapping(&cn_proc_event_id as *const cb_id as *const u8,&mut (*msg).id as *mut _ as *mut u8,core::mem::size_of::<(*msg).id>());(*msg).ack=0;(*msg).len=core::mem::size_of::<proc_event>() as u16;(*msg).flags=0;send_msg(msg); }

pub unsafe fn proc_exit_connector(task:*mut task_struct){ if atomic_read(&proc_event_num_listeners)<1{return;} let mut buffer=[0u8;CN_PROC_MSG_SIZE]; let msg=buffer_to_cn_msg(buffer.as_mut_ptr()); let ev=(*msg).data as *mut proc_event; core::ptr::write_bytes(core::ptr::addr_of_mut!((*ev).event_data) as *mut u8,0,core::mem::size_of_val(&(*ev).event_data));(*ev).timestamp_ns=ktime_get_ns();(*ev).what=PROC_EVENT_EXIT;(*ev).event_data.exit.process_pid=(*task).pid;(*ev).event_data.exit.process_tgid=(*task).tgid;(*ev).event_data.exit.exit_code=(*task).exit_code;(*ev).event_data.exit.exit_signal=(*task).exit_signal;rcu_read_lock();if pid_alive(task){let parent=rcu_dereference((*task).real_parent);(*ev).event_data.exit.parent_pid=(*parent).pid;(*ev).event_data.exit.parent_tgid=(*parent).tgid;}rcu_read_unlock();core::ptr::copy_nonoverlapping(&cn_proc_event_id as *const cb_id as *const u8,&mut (*msg).id as *mut _ as *mut u8,core::mem::size_of::<(*msg).id>());(*msg).ack=0;(*msg).len=core::mem::size_of::<proc_event>() as u16;(*msg).flags=0;send_msg(msg); }

/* Send an acknowledgement message to userspace. */
unsafe fn cn_proc_ack(err:i32,rcvd_seq:i32,rcvd_ack:i32){if atomic_read(&proc_event_num_listeners)<1{return;}let mut buffer=[0u8;CN_PROC_MSG_SIZE];let msg=buffer_to_cn_msg(buffer.as_mut_ptr());let ev=(*msg).data as *mut proc_event;core::ptr::write_bytes(core::ptr::addr_of_mut!((*ev).event_data) as *mut u8,0,core::mem::size_of_val(&(*ev).event_data));(*msg).seq=rcvd_seq;(*ev).timestamp_ns=ktime_get_ns();(*ev).cpu=-1;(*ev).what=PROC_EVENT_NONE;(*ev).event_data.ack.err=err;core::ptr::copy_nonoverlapping(&cn_proc_event_id as *const cb_id as *const u8,&mut (*msg).id as *mut _ as *mut u8,core::mem::size_of::<(*msg).id>());(*msg).ack=rcvd_ack+1;(*msg).len=core::mem::size_of::<proc_event>() as u16;(*msg).flags=0;send_msg(msg);}

unsafe fn cn_proc_mcast_ctl(msg:*mut cn_msg,nsp:*mut netlink_skb_parms){
    let mut mc_op=0; let mut prev_mc_op=0; let mut pinput: *mut proc_input=core::ptr::null_mut(); let mut ev_type=0; let mut err=0; let mut initial=0; let mut sk: *mut sock=core::ptr::null_mut();
    if current_user_ns()!=&init_user_ns || !task_is_in_init_pid_ns(current){return;}
    if (*msg).len as usize==core::mem::size_of::<proc_input>() {pinput=(*msg).data as *mut proc_input;mc_op=(*pinput).mcast_op;ev_type=(*pinput).event_type;} else if (*msg).len as usize==core::mem::size_of::<proc_cn_mcast_op>() {mc_op=*((*msg).data as *mut proc_cn_mcast_op);ev_type=PROC_EVENT_ALL;} else{return;}
    ev_type=valid_event(ev_type);if ev_type==PROC_EVENT_NONE{ev_type=PROC_EVENT_ALL;}
    if !(*nsp).sk.is_null(){sk=(*nsp).sk;if (*sk).sk_user_data.is_null(){(*sk).sk_user_data=kzalloc_obj::<proc_input>();if (*sk).sk_user_data.is_null(){err=ENOMEM;cn_proc_ack(err,(*msg).seq,(*msg).ack);return;}initial=1;}else{prev_mc_op=(*( (*sk).sk_user_data as *mut proc_input)).mcast_op;}(*( (*sk).sk_user_data as *mut proc_input)).event_type=ev_type;(*( (*sk).sk_user_data as *mut proc_input)).mcast_op=mc_op;}
    match mc_op { PROC_CN_MCAST_LISTEN=>{if initial||prev_mc_op!=PROC_CN_MCAST_LISTEN{atomic_inc(&proc_event_num_listeners);}}, PROC_CN_MCAST_IGNORE=>{if !initial&&prev_mc_op!=PROC_CN_MCAST_IGNORE{atomic_dec(&proc_event_num_listeners);}(*( (*sk).sk_user_data as *mut proc_input)).event_type=PROC_EVENT_NONE;}, _=>{err=EINVAL;} }
    cn_proc_ack(err,(*msg).seq,(*msg).ack);
}

unsafe fn cn_proc_init()->i32{let err=cn_add_callback(&mut cn_proc_event_id,"cn_proc",cn_proc_mcast_ctl);if err!=0{pr_warn!("cn_proc failed to register\n");return err;}0}
// device_initcall(cn_proc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
