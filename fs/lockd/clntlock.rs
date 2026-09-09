// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/fs/lockd/clntlock.c
 *
 * Lock handling for the client side NLM implementation
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

// Kernel headers and local headers provide the declarations used below.
// #define NLMDBG_FACILITY NLMDBG_CLIENT

extern "C" {
    fn reclaimer(ptr: *mut core::ffi::c_void) -> i32;
}

static mut NLM_BLOCKED: ListHead = ListHead::new();
static mut NLM_BLOCKED_LOCK: SpinLock = SpinLock::new();

/// nlmclnt_init - Set up per-NFS mount point lockd data structures
/// @nlm_init: pointer to arguments structure
///
/// Returns pointer to an appropriate nlm_host struct,
/// or an ERR_PTR value.
pub unsafe extern "C" fn nlmclnt_init(nlm_init: *const NlmclntInitdata) -> *mut NlmHost {
    let nlm_version: u32 = if (*nlm_init).nfs_version == 2 { 1 } else { 4 };
    let status = lockd_up((*nlm_init).net, (*nlm_init).cred);
    if status < 0 {
        return err_ptr(status);
    }

    let host = nlmclnt_lookup_host(
        (*nlm_init).address,
        (*nlm_init).addrlen,
        (*nlm_init).protocol,
        nlm_version,
        (*nlm_init).hostname,
        (*nlm_init).noresvport,
        (*nlm_init).net,
        (*nlm_init).cred,
    );
    if host.is_null() {
        lockd_down((*nlm_init).net);
        return err_ptr(-ENOLCK);
    }
    if (*host).h_rpcclnt.is_null() && nlm_bind_host(host).is_null() {
        nlmclnt_release_host(host);
        lockd_down((*nlm_init).net);
        return err_ptr(-ENOLCK);
    }

    (*host).h_nlmclnt_ops = (*nlm_init).nlmclnt_ops;
    host
}

pub unsafe extern "C" fn nlmclnt_done(host: *mut NlmHost) {
    let net = (*host).net;
    nlmclnt_release_host(host);
    lockd_down(net);
}

pub unsafe extern "C" fn nlmclnt_prepare_block(
    block: *mut NlmWait,
    host: *mut NlmHost,
    fl: *mut FileLock,
) {
    (*block).b_host = host;
    (*block).b_lock = fl;
    init_waitqueue_head(&mut (*block).b_wait);
    (*block).b_status = nlm_lck_blocked;
}

pub unsafe extern "C" fn nlmclnt_rpc_clnt(host: *mut NlmHost) -> *mut RpcClnt {
    (*host).h_rpcclnt
}

pub unsafe extern "C" fn nlmclnt_queue_block(block: *mut NlmWait) {
    spin_lock(&mut NLM_BLOCKED_LOCK);
    list_add(&mut (*block).b_list, &mut NLM_BLOCKED);
    spin_unlock(&mut NLM_BLOCKED_LOCK);
}

pub unsafe extern "C" fn nlmclnt_dequeue_block(block: *mut NlmWait) -> Be32 {
    spin_lock(&mut NLM_BLOCKED_LOCK);
    list_del(&mut (*block).b_list);
    let status = (*block).b_status;
    spin_unlock(&mut NLM_BLOCKED_LOCK);
    status
}

pub unsafe extern "C" fn nlmclnt_wait(
    block: *mut NlmWait,
    _req: *mut NlmRqst,
    timeout: i64,
) -> i32 {
    if block.is_null() {
        return -EAGAIN;
    }
    let ret = wait_event_interruptible_timeout(
        &mut (*block).b_wait,
        (*block).b_status != nlm_lck_blocked,
        timeout,
    );
    if ret < 0 {
        return -ERESTARTSYS;
    }
    if (*block).b_status == nlm_lck_denied_grace_period {
        (*block).b_status = nlm_lck_blocked;
    }
    0
}

pub unsafe extern "C" fn nlmclnt_grant(
    addr: *const SockAddr,
    lock: *const LockdLock,
) -> Be32 {
    let fl = &(*lock).fl;
    let fh = &(*lock).fh;
    let mut res = nlm_lck_denied;

    spin_lock(&mut NLM_BLOCKED_LOCK);
    let mut block = list_first_entry(&NLM_BLOCKED);
    while !block.is_null() {
        let fl_blocked = (*block).b_lock;
        if (*fl_blocked).fl_start == fl.fl_start
            && (*fl_blocked).fl_end == fl.fl_end
            && (*(*fl_blocked).fl_u.nfs_fl.owner).pid == (*lock).svid
            && rpc_cmp_addr(nlm_addr((*block).b_host), addr)
            && nfs_compare_fh(NFS_FH(file_inode((*fl_blocked).c.flc_file)), fh) == 0
        {
            (*block).b_status = nlm_granted;
            wake_up(&mut (*block).b_wait);
            res = nlm_granted;
        }
        block = list_next_entry(block);
    }
    spin_unlock(&mut NLM_BLOCKED_LOCK);
    trace_nlmclnt_grant(lock, addr, svc_addr_len(addr), res);
    res
}

pub unsafe extern "C" fn nlmclnt_recovery(host: *mut NlmHost) {
    if (*host).h_reclaiming == 0 {
        (*host).h_reclaiming += 1;
        nlm_get_host(host);
        let task = kthread_run(reclaimer, host.cast(), (*host).h_name);
        if is_err(task) {
            printk(KERN_ERR, (*host).h_name, ptr_err(task));
        }
    }
}

#[allow(unused_variables)]
pub unsafe extern "C" fn reclaimer(ptr: *mut core::ffi::c_void) -> i32 {
    let host = ptr as *mut NlmHost;
    let mut block: *mut NlmWait;
    let mut req = kmalloc_obj::<NlmRqst>();
    let mut fl: *mut FileLock;
    let mut next: *mut FileLock;
    let net = (*host).net;

    if req.is_null() {
        return 0;
    }
    allow_signal(SIGKILL);
    down_write(&mut (*host).h_rwsem);
    lockd_up(net, core::ptr::null_mut());
    dprintk!("lockd: reclaiming locks for host %s\n", (*host).h_name);

    'restart: loop {
        let nsmstate = (*host).h_nsmstate;
        (*host).h_nextrebind = jiffies;
        nlm_rebind_host(host);
        list_splice_init(&mut (*host).h_granted, &mut (*host).h_reclaim);
        fl = list_first_file_lock(&(*host).h_reclaim);
        while !fl.is_null() {
            next = list_next_file_lock(fl);
            list_del_init(&mut (*fl).fl_u.nfs_fl.list);
            if !signalled() && nlmclnt_reclaim(host, fl, req) == 0 {
                list_add_tail(&mut (*fl).fl_u.nfs_fl.list, &mut (*host).h_granted);
                if (*host).h_nsmstate != nsmstate {
                    continue 'restart;
                }
            }
            fl = next;
        }
        break;
    }

    (*host).h_reclaiming = 0;
    up_write(&mut (*host).h_rwsem);
    dprintk!("NLM: done reclaiming locks for host %s\n", (*host).h_name);
    spin_lock(&mut NLM_BLOCKED_LOCK);
    block = list_first_entry(&NLM_BLOCKED);
    while !block.is_null() {
        if (*block).b_host == host {
            (*block).b_status = nlm_lck_denied_grace_period;
            wake_up(&mut (*block).b_wait);
        }
        block = list_next_entry(block);
    }
    spin_unlock(&mut NLM_BLOCKED_LOCK);
    nlmclnt_release_host(host);
    lockd_down(net);
    kfree(req);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
