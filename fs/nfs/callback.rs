// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/nfs/callback.c
 *
 * Copyright (C) 2004 Trond Myklebust
 *
 * NFSv4 callback handling
 */

// Kernel includes and local headers are supplied by the surrounding translation unit.

#[repr(C)]
struct nfs_callback_data {
    users: u32,
    serv: *mut svc_serv,
}

static mut nfs_callback_info: [nfs_callback_data; (NFS4_MAX_MINOR_VERSION + 1) as usize] =
    [nfs_callback_data { users: 0, serv: core::ptr::null_mut() }; (NFS4_MAX_MINOR_VERSION + 1) as usize];
static mut nfs_callback_mutex: mutex = DEFINE_MUTEX!();
static mut nfs4_callback_program: svc_program = svc_program {
    pg_prog: NFS4_CALLBACK,
    pg_nvers: ARRAY_SIZE!(nfs4_callback_version),
    pg_vers: nfs4_callback_version.as_ptr(),
    pg_name: c"NFSv4 callback".as_ptr(),
    pg_class: c"nfs".as_ptr(),
    pg_authenticate: Some(nfs_callback_authenticate),
    pg_init_request: Some(svc_generic_init_request),
    pg_rpcbind_set: Some(svc_generic_rpcbind_set),
};

static unsafe fn nfs4_callback_up_net(serv: *mut svc_serv, net: *mut net) -> i32 {
    let cred = current_cred();
    let mut ret: i32;
    let nn = net_generic(net, nfs_net_id);

    ret = svc_xprt_create(serv, c"tcp".as_ptr(), net, PF_INET,
                          nfs_callback_set_tcpport, SVC_SOCK_ANONYMOUS, cred);
    if ret <= 0 {
        return if ret != 0 { ret } else { -ENOMEM };
    }
    (*nn).nfs_callback_tcpport = ret;
    dprintk!("NFS: Callback listener port = %u (af %u, net %x)\n",
             (*nn).nfs_callback_tcpport, PF_INET, (*net).ns.inum);

    ret = svc_xprt_create(serv, c"tcp".as_ptr(), net, PF_INET6,
                          nfs_callback_set_tcpport, SVC_SOCK_ANONYMOUS, cred);
    if ret > 0 {
        (*nn).nfs_callback_tcpport6 = ret;
        dprintk!("NFS: Callback listener port = %u (af %u, net %x)\n",
                 (*nn).nfs_callback_tcpport6, PF_INET6, (*net).ns.inum);
    } else if ret != -EAFNOSUPPORT {
        return if ret != 0 { ret } else { -ENOMEM };
    }
    0
}

/*
 * This is the NFSv4 callback kernel thread.
 */
unsafe fn nfs4_callback_svc(vrqstp: *mut c_void) -> i32 {
    let rqstp = vrqstp as *mut svc_rqst;

    svc_thread_init_status(rqstp, 0);
    set_freezable();

    while !svc_thread_should_stop(rqstp) {
        svc_recv(rqstp, 0);
    }

    svc_exit_thread(rqstp);
    0
}

#[inline]
unsafe fn nfs_callback_bc_serv(minorversion: u32, xprt: *mut rpc_xprt,
                               serv: *mut svc_serv) {
    if minorversion != 0 {
        /*
         * Save the svc_serv in the transport so that it can
         * be referenced when the session backchannel is initialized
         */
        (*xprt).bc_serv = serv;
    }
}

unsafe fn nfs_callback_start_svc(minorversion: i32, xprt: *mut rpc_xprt,
                                 serv: *mut svc_serv) -> i32 {
    let mut nrservs = nfs_callback_nr_threads;
    nfs_callback_bc_serv(minorversion as u32, xprt, serv);

    if nrservs < NFS4_MIN_NR_CALLBACK_THREADS {
        nrservs = NFS4_MIN_NR_CALLBACK_THREADS;
    }
    if (*serv).sv_nrthreads == nrservs {
        return 0;
    }
    let ret = svc_set_num_threads(serv, 0, nrservs);
    if ret != 0 {
        svc_set_num_threads(serv, 0, 0);
        return ret;
    }
    dprintk!("nfs_callback_up: service started\n");
    0
}

unsafe fn nfs_callback_down_net(minorversion: u32, serv: *mut svc_serv, net: *mut net) {
    let nn = net_generic(net, nfs_net_id);
    (*nn).cb_users[minorversion as usize] -= 1;
    if (*nn).cb_users[minorversion as usize] != 0 {
        return;
    }
    dprintk!("NFS: destroy per-net callback data; net=%x\n", (*net).ns.inum);
    svc_xprt_destroy_all(serv, net, false);
}

unsafe fn nfs_callback_up_net(minorversion: i32, serv: *mut svc_serv,
                              net: *mut net, xprt: *mut rpc_xprt) -> i32 {
    let nn = net_generic(net, nfs_net_id);
    if (*nn).cb_users[minorversion as usize] != 0 {
        (*nn).cb_users[minorversion as usize] += 1;
        return 0;
    }
    (*nn).cb_users[minorversion as usize] += 1;
    dprintk!("NFS: create per-net callback data; net=%x\n", (*net).ns.inum);

    let mut ret = svc_bind(serv, net);
    if ret < 0 {
        printk!(KERN_WARNING, "NFS: bind callback service failed\n");
        (*nn).cb_users[minorversion as usize] -= 1;
        dprintk!("NFS: Couldn't create callback socket: err = %d; net = %x\n", ret, (*net).ns.inum);
        return ret;
    }
    ret = 0;
    if minorversion == 0 {
        ret = nfs4_callback_up_net(serv, net);
    } else if (*(*xprt).ops).bc_setup.is_some() {
        set_bc_enabled(serv);
    } else {
        ret = -EPROTONOSUPPORT;
    }
    if ret < 0 {
        printk!(KERN_ERR, "NFS: callback service start failed\n");
        (*nn).cb_users[minorversion as usize] -= 1;
        dprintk!("NFS: Couldn't create callback socket: err = %d; net = %x\n", ret, (*net).ns.inum);
        return ret;
    }
    0
}

unsafe fn nfs_callback_create_svc(minorversion: i32) -> *mut svc_serv {
    let cb_info = &mut nfs_callback_info[minorversion as usize];
    if !cb_info.serv.is_null() {
        return cb_info.serv;
    }
    if cb_info.users != 0 {
        printk!(KERN_WARNING, "nfs_callback_create_svc: no kthread, %d users??\n", cb_info.users);
    }
    let serv = svc_create(&mut nfs4_callback_program, NFS4_CALLBACK_BUFSIZE, nfs4_callback_svc);
    if serv.is_null() {
        printk!(KERN_ERR, "nfs_callback_create_svc: create service failed\n");
        return ERR_PTR(-ENOMEM);
    }
    cb_info.serv = serv;
    dprintk!("nfs_callback_create_svc: service created\n");
    serv
}

/*
 * Bring up the callback thread if it is not already up.
 */
pub unsafe fn nfs_callback_up(minorversion: u32, xprt: *mut rpc_xprt) -> i32 {
    mutex_lock(&mut nfs_callback_mutex);
    let cb_info = &mut nfs_callback_info[minorversion as usize];
    let serv = nfs_callback_create_svc(minorversion as i32);
    let mut ret: i32;
    if IS_ERR(serv) {
        ret = PTR_ERR(serv);
        mutex_unlock(&mut nfs_callback_mutex);
        return ret;
    }
    ret = nfs_callback_up_net(minorversion as i32, serv, (*xprt).xprt_net, xprt);
    if ret < 0 {
        if cb_info.users == 0 {
            xprt_svc_shutdown_bc(xprt);
            svc_set_num_threads(cb_info.serv, 0, 0);
            xprt_svc_destroy_nullify_bc(xprt, &mut cb_info.serv);
        }
        mutex_unlock(&mut nfs_callback_mutex);
        return ret;
    }
    ret = nfs_callback_start_svc(minorversion as i32, xprt, serv);
    if ret < 0 {
        nfs_callback_down_net(minorversion, serv, (*xprt).xprt_net);
        dprintk!("NFS: Couldn't create server thread; err = %d\n", ret);
        if cb_info.users == 0 {
            xprt_svc_shutdown_bc(xprt);
            svc_set_num_threads(cb_info.serv, 0, 0);
            xprt_svc_destroy_nullify_bc(xprt, &mut cb_info.serv);
        }
        mutex_unlock(&mut nfs_callback_mutex);
        return ret;
    }
    cb_info.users += 1;
    mutex_unlock(&mut nfs_callback_mutex);
    ret
}

/*
 * Kill the callback thread if it's no longer being used.
 */
pub unsafe fn nfs_callback_down(minorversion: i32, net: *mut net, xprt: *mut rpc_xprt) {
    mutex_lock(&mut nfs_callback_mutex);
    let cb_info = &mut nfs_callback_info[minorversion as usize];
    let serv = cb_info.serv;
    xprt_svc_shutdown_bc(xprt);
    nfs_callback_down_net(minorversion as u32, serv, net);
    cb_info.users -= 1;
    if cb_info.users == 0 {
        svc_set_num_threads(serv, 0, 0);
        dprintk!("nfs_callback_down: service destroyed\n");
        xprt_svc_destroy_nullify_bc(xprt, &mut cb_info.serv);
    }
    mutex_unlock(&mut nfs_callback_mutex);
}

/* Boolean check of RPC_AUTH_GSS principal */
pub unsafe fn check_gss_callback_principal(clp: *mut nfs_client, rqstp: *mut svc_rqst) -> i32 {
    let mut p = (*rqstp).rq_cred.cr_principal;
    if (*(*rqstp).rq_authop).flavour != RPC_AUTH_GSS { return 1; }
    /* No RPC_AUTH_GSS on NFSv4.1 back channel yet */
    if (*clp).cl_minorversion != 0 || p.is_null() { return 0; }
    if !(*clp).cl_acceptor.is_null() { return (!strcmp(p, (*clp).cl_acceptor)) as i32; }
    /* Expect a GSS_C_NT_HOSTBASED_NAME like "nfs@serverhostname" */
    if memcmp(p as *const c_void, c"nfs@".as_ptr() as *const c_void, 4) != 0 { return 0; }
    p = p.add(4);
    if strcmp(p, (*clp).cl_hostname) != 0 { return 0; }
    1
}

/*
 * pg_authenticate method for nfsv4 callback threads.
 */
unsafe fn nfs_callback_authenticate(rqstp: *mut svc_rqst) -> svc_auth_status {
    (*rqstp).rq_auth_stat = rpc_autherr_badcred;
    match (*(*rqstp).rq_authop).flavour {
        RPC_AUTH_NULL => {
            if (*rqstp).rq_proc != CB_NULL { return SVC_DENIED; }
        }
        RPC_AUTH_GSS => {
            /* No RPC_AUTH_GSS support yet in NFSv4.1 */
            if svc_is_backchannel(rqstp) { return SVC_DENIED; }
        }
        _ => {}
    }
    (*rqstp).rq_auth_stat = rpc_auth_ok;
    SVC_OK
}

/* Define NFS4 callback program */
static nfs4_callback_version: [*const svc_version; 5] = [
    core::ptr::null(),
    &nfs4_callback_version1,
    core::ptr::null(),
    core::ptr::null(),
    &nfs4_callback_version4,
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
