// SPDX-License-Identifier: GPL-2.0-only
/* Translation of linux/fs/lockd/clntproc.c. External kernel symbols are
 * intentionally left as dependencies supplied by the surrounding system. */

const NLMCLNT_GRACE_WAIT: u64 = 5 * HZ;
const NLMCLNT_POLL_TIMEOUT: u64 = 30 * HZ;
const NLMCLNT_MAX_RETRIES: u32 = 3;

static mut NLM_COOKIE: atomic_t = ATOMIC_INIT(0x1234);

unsafe fn nlmclnt_next_cookie(c: *mut lockd_cookie) {
    let cookie: u32 = atomic_inc_return(&mut NLM_COOKIE);
    memcpy((*c).data.as_mut_ptr() as *mut c_void, &cookie as *const _ as *const c_void, 4);
    (*c).len = 4;
}

unsafe fn nlmclnt_get_lockowner(p: *mut nlm_lockowner) -> *mut nlm_lockowner {
    refcount_inc(&mut (*p).count); p
}
unsafe fn nlmclnt_put_lockowner(p: *mut nlm_lockowner) {
    if !refcount_dec_and_lock(&mut (*p).count, &mut (*(*p).host).h_lock) { return; }
    list_del(&mut (*p).list); spin_unlock(&mut (*(*p).host).h_lock);
    nlmclnt_release_host((*p).host); kfree(p as *mut c_void);
}
unsafe fn nlm_pidbusy(host: *mut nlm_host, pid: u32) -> i32 {
    let mut p: *mut nlm_lockowner;
    list_for_each_entry!(p, &(*host).h_lockowners, list) { if (*p).pid == pid { return -EBUSY; } }
    0
}
unsafe fn __nlm_alloc_pid(host: *mut nlm_host) -> u32 {
    let mut r; loop { r = (*host).h_pidcount; (*host).h_pidcount = r.wrapping_add(1); if nlm_pidbusy(host,r) >= 0 { return r; } }
}
unsafe fn __nlmclnt_find_lockowner(host: *mut nlm_host, owner: fl_owner_t) -> *mut nlm_lockowner {
    let mut p: *mut nlm_lockowner;
    list_for_each_entry!(p, &(*host).h_lockowners, list) { if (*p).owner == owner { return nlmclnt_get_lockowner(p); } }
    core::ptr::null_mut()
}
unsafe fn nlmclnt_find_lockowner(host: *mut nlm_host, owner: fl_owner_t) -> *mut nlm_lockowner {
    spin_lock(&mut (*host).h_lock); let mut res = __nlmclnt_find_lockowner(host,owner);
    if res.is_null() { spin_unlock(&mut (*host).h_lock); let mut new = kmalloc_obj::<nlm_lockowner>(); spin_lock(&mut (*host).h_lock);
        res=__nlmclnt_find_lockowner(host,owner); if res.is_null() && !new.is_null() { res=new; refcount_set(&mut (*new).count,1); (*new).owner=owner; (*new).pid=__nlm_alloc_pid(host); (*new).host=nlm_get_host(host); list_add(&mut (*new).list,&mut (*host).h_lockowners); new=core::ptr::null_mut(); } kfree(new as *mut c_void); }
    spin_unlock(&mut (*host).h_lock); res
}
unsafe fn nlmclnt_setlockargs(req: *mut nlm_rqst, fl: *mut file_lock) {
    let argp=&mut (*req).a_args; let lock=&mut argp.lock; let nodename=(*(*req).a_host).h_rpcclnt.as_ref().unwrap().cl_nodename;
    nlmclnt_next_cookie(&mut argp.cookie); memcpy(&mut lock.fh as *mut _ as *mut c_void, NFS_FH(file_inode((*fl).c.flc_file)), core::mem::size_of::<nfs_fh>());
    lock.caller=nodename; lock.oh.data=(*req).a_owner.as_mut_ptr(); lock.oh.len=snprintf((*req).a_owner.as_mut_ptr(),(*req).a_owner.len(),c"%u@%s",(*fl).fl_u.nfs_fl.owner.as_ref().unwrap().pid,nodename); lock.svid=(*fl).fl_u.nfs_fl.owner.as_ref().unwrap().pid; lock.fl.fl_start=(*fl).fl_start; lock.fl.fl_end=(*fl).fl_end; lock.fl.c.flc_type=(*fl).c.flc_type;
}
unsafe fn nlmclnt_release_lockargs(req:*mut nlm_rqst) { WARN_ON_ONCE(!(*req).a_args.lock.fl.fl_ops.is_null()); }

unsafe fn nlmclnt_proc(host:*mut nlm_host, cmd:i32, fl:*mut file_lock, data:*mut c_void)->i32 {
    let call=nlm_alloc_call(host); if call.is_null(){return -ENOMEM;} let ops=(*host).h_nlmclnt_ops;
    if !ops.is_null() && (*ops).nlmclnt_alloc_call.is_some(){((*ops).nlmclnt_alloc_call.unwrap())(data);} nlmclnt_locks_init_private(fl,host);
    if (*fl).fl_u.nfs_fl.owner.is_null(){nlmclnt_release_call(call);return -ENOMEM;} nlmclnt_setlockargs(call,fl);(*call).a_callback_data=data;
    let status=if IS_SETLK(cmd)||IS_SETLKW(cmd){if (*fl).c.flc_type!=F_UNLCK{(*call).a_args.block=if IS_SETLKW(cmd){1}else{0};nlmclnt_lock(call,fl)}else{nlmclnt_unlock(call,fl)}}else if IS_GETLK(cmd){nlmclnt_test(call,fl)}else{-EINVAL};
    ((*fl).fl_ops.unwrap()).fl_release_private.unwrap()(fl);(*fl).fl_ops=None; dprintk!("lockd: clnt proc returns %d\n",status);status
}

unsafe fn nlm_alloc_call(host:*mut nlm_host)->*mut nlm_rqst { loop { let call=kzalloc_obj::<nlm_rqst>(); if !call.is_null(){refcount_set(&mut (*call).a_count,1);locks_init_lock(&mut (*call).a_args.lock.fl);locks_init_lock(&mut (*call).a_res.lock.fl);(*call).a_host=nlm_get_host(host);return call;} if signalled(){break;} printk!("nlm_alloc_call: failed, waiting for memory\n"); schedule_timeout_interruptible(5*HZ); } core::ptr::null_mut() }
unsafe fn nlmclnt_release_call(call:*mut nlm_rqst){let ops=(*(*call).a_host).h_nlmclnt_ops;if !refcount_dec_and_test(&mut (*call).a_count){return;}if !ops.is_null()&&(*ops).nlmclnt_release_call.is_some(){((*ops).nlmclnt_release_call.unwrap())((*call).a_callback_data);}nlmclnt_release_host((*call).a_host);nlmclnt_release_lockargs(call);kfree(call as *mut c_void);}
unsafe fn nlmclnt_rpc_release(data:*mut c_void){nlmclnt_release_call(data as *mut nlm_rqst);}

unsafe fn nlm_wait_on_grace(queue:*mut wait_queue_head_t)->i32{let mut wait=DEFINE_WAIT!();let mut status=-EINTR;prepare_to_wait(queue,&mut wait,TASK_INTERRUPTIBLE);if !signalled(){schedule_timeout(NLMCLNT_GRACE_WAIT);try_to_freeze();if !signalled(){status=0;}}finish_wait(queue,&mut wait);status}

/* Generic synchronous and asynchronous RPC paths. */
unsafe fn nlmclnt_call(cred:*const cred, req:*mut nlm_rqst, proc:u32)->i32{let host=(*req).a_host;let mut msg=rpc_message{rpc_argp:&mut (*req).a_args as *mut _,rpc_resp:&mut (*req).a_res as *mut _,rpc_cred:cred,rpc_proc:core::ptr::null()};loop{if (*host).h_reclaiming&&!(*req).a_args.reclaim{goto!(in_grace_period);}let clnt=nlm_bind_host(host);if clnt.is_null(){return -ENOLCK;}msg.rpc_proc=&(*clnt).cl_procinfo[proc as usize];let mut status=rpc_call_sync(clnt,&mut msg,0);if status<0{match status{-EPROTONOSUPPORT=>status=-EINVAL,-ECONNREFUSED|-ETIMEDOUT|-ENOTCONN=>{nlm_rebind_host(host);status=-EAGAIN;},-ERESTARTSYS=>return if signalled(){-EINTR}else{status},_=>{}}break;}if (*req).a_res.status==nlm_lck_denied_grace_period{if (*req).a_args.reclaim{return -ENOLCK;}}else{if !(*req).a_args.reclaim{wake_up_all(&mut (*host).h_gracewait);}return 0;}in_grace_period:status=nlm_wait_on_grace(&mut (*host).h_gracewait);if status!=0{return status;}}}

unsafe fn __nlm_async_call(req:*mut nlm_rqst,proc:u32,msg:*mut rpc_message,ops:*const rpc_call_ops)->*mut rpc_task{let clnt=nlm_bind_host((*req).a_host);if clnt.is_null(){((*ops).rpc_release.unwrap())(req as *mut c_void);return ERR_PTR(-ENOLCK);}(*msg).rpc_proc=&(*clnt).cl_procinfo[proc as usize];let setup=rpc_task_setup{rpc_message:msg,callback_ops:ops,callback_data:req as *mut c_void,flags:RPC_TASK_ASYNC,rpc_client:clnt};rpc_run_task(&setup)}
unsafe fn nlm_do_async_call(req:*mut nlm_rqst,proc:u32,msg:*mut rpc_message,ops:*const rpc_call_ops)->i32{let task=__nlm_async_call(req,proc,msg,ops);if IS_ERR(task){return PTR_ERR(task);}rpc_put_task(task);0}
unsafe fn nlm_async_call(req:*mut nlm_rqst,proc:u32,ops:*const rpc_call_ops)->i32{let mut msg=rpc_message{rpc_argp:&mut (*req).a_args as *mut _,rpc_resp:&mut (*req).a_res as *mut _,..Default::default()};nlm_do_async_call(req,proc,&mut msg,ops)}
unsafe fn nlm_async_reply(req:*mut nlm_rqst,proc:u32,ops:*const rpc_call_ops)->i32{let mut msg=rpc_message{rpc_argp:&mut (*req).a_res as *mut _,..Default::default()};nlm_do_async_call(req,proc,&mut msg,ops)}
unsafe fn nlmclnt_async_call(cred:*const cred,req:*mut nlm_rqst,proc:u32,ops:*const rpc_call_ops)->i32{let mut msg=rpc_message{rpc_argp:&mut (*req).a_args as *mut _,rpc_resp:&mut (*req).a_res as *mut _,rpc_cred:cred,..Default::default()};let task=__nlm_async_call(req,proc,&mut msg,ops);if IS_ERR(task){return PTR_ERR(task);}let e=rpc_wait_for_completion_task(task);rpc_put_task(task);e}

/* File-lock private data and the lock/test/unlock operations. */
unsafe fn nlmclnt_locks_copy_lock(new:*mut file_lock,fl:*mut file_lock){spin_lock(&mut (*(*fl).fl_u.nfs_fl.owner).host.as_ref().unwrap().h_lock);(*new).fl_u.nfs_fl.state=(*fl).fl_u.nfs_fl.state;(*new).fl_u.nfs_fl.owner=nlmclnt_get_lockowner((*fl).fl_u.nfs_fl.owner);list_add_tail(&mut (*new).fl_u.nfs_fl.list,&mut (*(*fl).fl_u.nfs_fl.owner).host.as_ref().unwrap().h_granted);spin_unlock(&mut (*(*fl).fl_u.nfs_fl.owner).host.as_ref().unwrap().h_lock);}
unsafe fn nlmclnt_locks_release_private(fl:*mut file_lock){spin_lock(&mut (*(*fl).fl_u.nfs_fl.owner).host.as_ref().unwrap().h_lock);list_del(&mut (*fl).fl_u.nfs_fl.list);spin_unlock(&mut (*(*fl).fl_u.nfs_fl.owner).host.as_ref().unwrap().h_lock);nlmclnt_put_lockowner((*fl).fl_u.nfs_fl.owner);}
static NLMCLNT_LOCK_OPS:file_lock_operations=file_lock_operations{fl_copy_lock:Some(nlmclnt_locks_copy_lock),fl_release_private:Some(nlmclnt_locks_release_private)};
unsafe fn nlmclnt_locks_init_private(fl:*mut file_lock,host:*mut nlm_host){(*fl).fl_u.nfs_fl.state=0;(*fl).fl_ops=None;(*fl).fl_u.nfs_fl.owner=nlmclnt_find_lockowner(host,(*fl).c.flc_owner);INIT_LIST_HEAD!(&mut (*fl).fl_u.nfs_fl.list);if !(*fl).fl_u.nfs_fl.owner.is_null(){(*fl).fl_ops=Some(&NLMCLNT_LOCK_OPS);}}
unsafe fn do_vfs_lock(fl:*mut file_lock)->i32{locks_lock_file_wait((*fl).c.flc_file,fl)}

/* The remaining lock/reclaim/unlock/cancel routines retain the C state machine. */
unsafe fn nlmclnt_test(req:*mut nlm_rqst,fl:*mut file_lock)->i32{let mut s=nlmclnt_call(nfs_file_cred((*fl).c.flc_file),req,NLMPROC_TEST);if s>=0{match (*req).a_res.status{nlm_granted=>(*fl).c.flc_type=F_UNLCK,nlm_lck_denied=>{(*fl).fl_start=(*req).a_res.lock.fl.fl_start;(*fl).fl_end=(*req).a_res.lock.fl.fl_end;(*fl).c.flc_type=(*req).a_res.lock.fl.c.flc_type;(*fl).c.flc_pid=-(*req).a_res.lock.fl.c.flc_pid;},_=>s=nlm_stat_to_errno((*req).a_res.status)}}nlmclnt_release_call(req);s}
unsafe fn nlmclnt_lock(req:*mut nlm_rqst,fl:*mut file_lock)->i32{let cred=nfs_file_cred((*fl).c.flc_file);let host=(*req).a_host;if nsm_monitor(host)<0{return -ENOLCK;}(*req).a_args.state=nsm_local_state;let flags=(*fl).c.flc_flags;(*fl).c.flc_flags|=FL_ACCESS;let mut status=do_vfs_lock(fl);(*fl).c.flc_flags=flags;if status<0{return status;}let mut block=nlm_wait_init(host,fl);nlmclnt_queue_block(&mut block);loop{(*fl).fl_u.nfs_fl.state=(*host).h_state;status=nlmclnt_call(cred,req,NLMPROC_LOCK);if status<0{break;}if (*req).a_res.status==nlm_lck_denied_grace_period{continue;}if (*req).a_res.status!=nlm_lck_blocked{break;}status=nlmclnt_wait(&mut block,req,NLMCLNT_POLL_TIMEOUT);if status<0||block.b_status!=nlm_lck_blocked{break;}}let bs=nlmclnt_dequeue_block(&mut block);if (*req).a_res.status==nlm_lck_blocked{(*req).a_res.status=bs;}if (*req).a_res.status==nlm_lck_blocked{if !(*req).a_args.block{return nlmclnt_lock_cleanup(req,fl,status,flags);}if nlmclnt_cancel(host,(*req).a_args.block,fl)==0{return status;}}if (*req).a_res.status==nlm_granted{down_read(&mut (*host).h_rwsem);if (*fl).fl_u.nfs_fl.state!=(*host).h_state{up_read(&mut (*host).h_rwsem);return nlmclnt_lock(req,fl);}(*fl).c.flc_flags|=FL_SLEEP;do_vfs_lock(fl);up_read(&mut (*host).h_rwsem);(*fl).c.flc_flags=flags;status=0;}if status<0{return nlmclnt_lock_cleanup(req,fl,status,flags);}if (*req).a_res.status==nlm_lck_denied&&(flags&FL_SLEEP)!=0{-ENOLCK}else{nlm_stat_to_errno((*req).a_res.status)}}
unsafe fn nlmclnt_lock_cleanup(req:*mut nlm_rqst,fl:*mut file_lock,status:i32,flags:u8)->i32{let typ=(*fl).c.flc_type;(*fl).c.flc_type=F_UNLCK;down_read(&mut (*(*req).a_host).h_rwsem);do_vfs_lock(fl);up_read(&mut (*(*req).a_host).h_rwsem);(*fl).c.flc_type=typ;(*fl).c.flc_flags=flags;nlmclnt_async_call(nfs_file_cred((*fl).c.flc_file),req,NLMPROC_UNLOCK,&NLMCLNT_UNLOCK_OPS);status}

unsafe fn nlmclnt_reclaim(host:*mut nlm_host,fl:*mut file_lock,req:*mut nlm_rqst)->i32{memset(req,0,core::mem::size_of::<nlm_rqst>());locks_init_lock(&mut (*req).a_args.lock.fl);locks_init_lock(&mut (*req).a_res.lock.fl);(*req).a_host=host;nlmclnt_setlockargs(req,fl);(*req).a_args.reclaim=1;if nlmclnt_call(nfs_file_cred((*fl).c.flc_file),req,NLMPROC_LOCK)>=0&&(*req).a_res.status==nlm_granted{0}else{-ENOLCK}}
unsafe fn nlmclnt_unlock(req:*mut nlm_rqst,fl:*mut file_lock)->i32{let host=(*req).a_host;let flags=(*fl).c.flc_flags;(*fl).c.flc_flags|=FL_EXISTS;down_read(&mut (*host).h_rwsem);let mut s=do_vfs_lock(fl);up_read(&mut (*host).h_rwsem);(*fl).c.flc_flags=flags;if s==-ENOENT{s=0;nlmclnt_release_call(req);return s;}refcount_inc(&mut (*req).a_count);s=nlmclnt_async_call(nfs_file_cred((*fl).c.flc_file),req,NLMPROC_UNLOCK,&NLMCLNT_UNLOCK_OPS);if s>=0&&(*req).a_res.status!=nlm_granted{s=-ENOLCK;}nlmclnt_release_call(req);s}

static NLMCLNT_UNLOCK_OPS:rpc_call_ops=rpc_call_ops{rpc_call_prepare:Some(nlmclnt_unlock_prepare),rpc_call_done:Some(nlmclnt_unlock_callback),rpc_release:Some(nlmclnt_rpc_release)};
unsafe fn nlmclnt_unlock_prepare(task:*mut rpc_task,data:*mut c_void){let req=data as *mut nlm_rqst;let ops=(*(*req).a_host).h_nlmclnt_ops;let defer=!ops.is_null()&&(*ops).nlmclnt_unlock_prepare.is_some()&&((*ops).nlmclnt_unlock_prepare.unwrap())(task,(*req).a_callback_data);if !defer{rpc_call_start(task);}}
unsafe fn nlmclnt_unlock_callback(task:*mut rpc_task,_data:*mut c_void){if RPC_SIGNALLED(task){return;}if (*task).tk_status<0{if (*task).tk_status==-EACCES||(*task).tk_status==-EIO{return;}nlm_rebind_host((*task).tk_client.as_ref().unwrap().cl_private as *mut nlm_host);rpc_restart_call(task);return;}if ntohl((*task).tk_msg.as_ref().unwrap().rpc_resp.cast::<lockd_res>().as_ref().unwrap().status)==NLM_LCK_DENIED_GRACE_PERIOD{rpc_delay(task,NLMCLNT_GRACE_WAIT);rpc_restart_call(task);}}
unsafe fn nlmclnt_cancel(host:*mut nlm_host,block:i32,fl:*mut file_lock)->i32{let req=nlm_alloc_call(host);if req.is_null(){return -ENOMEM;}(*req).a_flags=RPC_TASK_ASYNC;nlmclnt_setlockargs(req,fl);(*req).a_args.block=block;refcount_inc(&mut (*req).a_count);let mut s=nlmclnt_async_call(nfs_file_cred((*fl).c.flc_file),req,NLMPROC_CANCEL,&NLMCLNT_CANCEL_OPS);if s==0&&(*req).a_res.status==nlm_lck_denied{s=-ENOLCK;}nlmclnt_release_call(req);s}
static NLMCLNT_CANCEL_OPS:rpc_call_ops=rpc_call_ops{rpc_call_done:Some(nlmclnt_cancel_callback),rpc_release:Some(nlmclnt_rpc_release)};
unsafe fn nlmclnt_cancel_callback(task:*mut rpc_task,data:*mut c_void){let req=data as *mut nlm_rqst;if RPC_SIGNALLED(task){return;}if (*task).tk_status<0||(*req).a_retries>=NLMCLNT_MAX_RETRIES{return;}(*req).a_retries+=1;nlm_rebind_host((*req).a_host);rpc_restart_call(task);rpc_delay(task,30*HZ);}
unsafe fn nlm_stat_to_errno(status:__be32)->i32{match ntohl(status){NLM_LCK_GRANTED=>0,NLM_LCK_DENIED=>-EAGAIN,NLM_LCK_DENIED_NOLOCKS|NLM_LCK_DENIED_GRACE_PERIOD|NLM_LCK_BLOCKED=>-ENOLCK,_=>-ENOLCK}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
