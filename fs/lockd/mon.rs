// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/lockd/mon.c
 *
 * The kernel statd client.
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

// C dependencies supplied by the surrounding kernel translation.

const NLMDBG_FACILITY: u32 = NLMDBG_MONITOR;
const NSM_PROGRAM: u32 = 100024;
const NSM_VERSION: u32 = 1;

#[repr(u32)]
enum NsmProc {
    Null,
    Stat,
    Mon,
    Unmon,
    UnmonAll,
    SimuCrash,
    Notify,
}

#[repr(C)]
struct nsm_args {
    priv_: *mut nsm_private,
    prog: u32,
    vers: u32,
    proc: u32,
    mon_name: *mut i8,
    nodename: *const i8,
}

#[repr(C)]
struct nsm_res {
    status: u32,
    state: u32,
}

static mut nsm_lock: spinlock_t = unsafe { core::mem::zeroed() };

/* Local NSM state */
static mut nsm_local_state: u32 = 0;
static mut nsm_use_hostnames: bool = false;

#[inline]
unsafe fn nsm_addr(nsm: *const nsm_handle) -> *mut sockaddr {
    &mut (*(nsm as *mut nsm_handle)).sm_addr as *mut _ as *mut sockaddr
}

unsafe fn nsm_create(net: *mut net, nodename: *const i8) -> *mut rpc_clnt {
    let mut sin: sockaddr_in = core::mem::zeroed();
    sin.sin_family = AF_INET as _;
    sin.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    let args = rpc_create_args {
        net,
        protocol: XPRT_TRANSPORT_TCP,
        address: &sin as *const _ as *mut sockaddr,
        addrsize: core::mem::size_of::<sockaddr_in>(),
        servername: b"rpc.statd\0".as_ptr() as *const i8,
        nodename,
        program: &nsm_program_def,
        version: NSM_VERSION,
        authflavor: RPC_AUTH_NULL,
        flags: RPC_CLNT_CREATE_NOPING,
        cred: current_cred(),
    };
    rpc_create(&args)
}

unsafe fn nsm_mon_unmon(nsm: *mut nsm_handle, proc: u32, res: *mut nsm_res,
                        host: *const nlm_host) -> i32 {
    let mut args = nsm_args {
        priv_: &mut (*nsm).sm_priv,
        prog: NLM_PROGRAM,
        vers: 3,
        proc: NLMPROC_NSM_NOTIFY,
        mon_name: (*nsm).sm_mon_name,
        nodename: (*host).nodename,
    };
    let mut msg: rpc_message = core::mem::zeroed();
    msg.rpc_argp = &mut args as *mut _ as *mut _;
    msg.rpc_resp = res as *mut _;
    core::ptr::write_bytes(res, 0, 1);
    let clnt = nsm_create((*host).net, (*host).nodename);
    if IS_ERR(clnt) {
        dprintk(b"lockd: failed to create NSM upcall transport\0".as_ptr() as *const i8,
                PTR_ERR(clnt), (*(*host).net).ns.inum);
        return PTR_ERR(clnt);
    }
    msg.rpc_proc = (*clnt).cl_procinfo.add(proc as usize);
    let mut status = rpc_call_sync(clnt, &mut msg, RPC_TASK_SOFTCONN);
    if status == -ECONNREFUSED {
        dprintk(b"lockd: NSM upcall RPC failed, forcing rebind\0".as_ptr() as *const i8, status);
        rpc_force_rebind(clnt);
        status = rpc_call_sync(clnt, &mut msg, RPC_TASK_SOFTCONN);
    }
    if status < 0 {
        dprintk(b"lockd: NSM upcall RPC failed\0".as_ptr() as *const i8, status);
    } else { status = 0; }
    rpc_shutdown_client(clnt);
    status
}

pub unsafe fn nsm_monitor(host: *const nlm_host) -> i32 {
    let nsm = (*host).h_nsmhandle;
    let mut res: nsm_res = core::mem::zeroed();
    if (*nsm).sm_monitored { return 0; }
    (*nsm).sm_mon_name = if nsm_use_hostnames { (*nsm).sm_name } else { (*nsm).sm_addrbuf.as_mut_ptr() };
    let mut status = nsm_mon_unmon(nsm, NsmProc::Mon as u32, &mut res, host);
    if res.status != 0 { status = -EIO; }
    if status < 0 { pr_notice_ratelimited(b"lockd: cannot monitor\0".as_ptr() as *const i8, (*nsm).sm_name); return status; }
    (*nsm).sm_monitored = 1;
    if nsm_local_state != res.state { nsm_local_state = res.state; }
    0
}

pub unsafe fn nsm_unmonitor(host: *const nlm_host) {
    let nsm = (*host).h_nsmhandle;
    if refcount_read(&(*nsm).sm_count) == 1 && (*nsm).sm_monitored && !(*nsm).sm_sticky {
        let mut res: nsm_res = core::mem::zeroed();
        let mut status = nsm_mon_unmon(nsm, NsmProc::Unmon as u32, &mut res, host);
        if res.status != 0 { status = -EIO; }
        if status >= 0 { (*nsm).sm_monitored = 0; }
    }
}

// The list_for_each_entry traversal below is represented by the surrounding
// kernel list bindings; these functions preserve the C lookup operations.
unsafe fn nsm_lookup_hostname(handles: *const list_head, hostname: *const i8, len: usize) -> *mut nsm_handle {
    let mut nsm: *mut nsm_handle = core::ptr::null_mut();
    list_for_each_entry!(nsm, handles, sm_link, {
        if strlen((*nsm).sm_name) == len && memcmp((*nsm).sm_name as *const _, hostname as *const _, len) == 0 { return nsm; }
    });
    core::ptr::null_mut()
}
unsafe fn nsm_lookup_addr(handles: *const list_head, sap: *const sockaddr) -> *mut nsm_handle {
    let mut nsm: *mut nsm_handle = core::ptr::null_mut();
    list_for_each_entry!(nsm, handles, sm_link, { if rpc_cmp_addr(nsm_addr(nsm), sap) != 0 { return nsm; } });
    core::ptr::null_mut()
}
unsafe fn nsm_lookup_priv(handles: *const list_head, priv_: *const nsm_private) -> *mut nsm_handle {
    let mut nsm: *mut nsm_handle = core::ptr::null_mut();
    list_for_each_entry!(nsm, handles, sm_link, { if memcmp((*nsm).sm_priv.data.as_ptr() as *const _, (*priv_).data.as_ptr() as *const _, core::mem::size_of_val(&(*priv_).data)) == 0 { return nsm; } });
    core::ptr::null_mut()
}

unsafe fn nsm_init_private(nsm: *mut nsm_handle) {
    let p = (*nsm).sm_priv.data.as_mut_ptr() as *mut u64;
    put_unaligned(ktime_get_ns(), p);
    put_unaligned(nsm as usize as u64, p.add(1));
}

unsafe fn nsm_create_handle(sap: *const sockaddr, salen: usize, hostname: *const i8, hostname_len: usize) -> *mut nsm_handle {
    if hostname.is_null() { return core::ptr::null_mut(); }
    let new = kzalloc(core::mem::size_of::<nsm_handle>() + hostname_len + 1, GFP_KERNEL) as *mut nsm_handle;
    if new.is_null() { return core::ptr::null_mut(); }
    refcount_set(&mut (*new).sm_count, 1);
    (*new).sm_name = (new.add(1)) as *mut i8;
    core::ptr::copy_nonoverlapping(sap as *const u8, nsm_addr(new) as *mut u8, salen);
    (*new).sm_addrlen = salen;
    nsm_init_private(new);
    if rpc_ntop(nsm_addr(new), (*new).sm_addrbuf.as_mut_ptr(), (*new).sm_addrbuf.len()) == 0 { snprintf((*new).sm_addrbuf.as_mut_ptr(), (*new).sm_addrbuf.len(), b"unsupported address family\0".as_ptr() as *const i8); }
    core::ptr::copy_nonoverlapping(hostname, (*new).sm_name, hostname_len);
    *(*new).sm_name.add(hostname_len) = 0;
    new
}

pub unsafe fn nsm_get_handle(net: *const net, sap: *const sockaddr, salen: usize, hostname: *const i8, hostname_len: usize) -> *mut nsm_handle {
    let ln = net_generic(net, lockd_net_id);
    if !hostname.is_null() && !memchr(hostname as *const _, b'/' as i32, hostname_len).is_null() { return core::ptr::null_mut(); }
    let mut new: *mut nsm_handle = core::ptr::null_mut();
    loop {
        spin_lock(&mut nsm_lock);
        let cached = if nsm_use_hostnames && !hostname.is_null() { nsm_lookup_hostname(&(*ln).nsm_handles, hostname, hostname_len) } else { nsm_lookup_addr(&(*ln).nsm_handles, sap) };
        if !cached.is_null() { refcount_inc(&mut (*cached).sm_count); spin_unlock(&mut nsm_lock); kfree(new as *mut _); return cached; }
        if !new.is_null() { list_add(&mut (*new).sm_link, &mut (*ln).nsm_handles); spin_unlock(&mut nsm_lock); return new; }
        spin_unlock(&mut nsm_lock);
        new = nsm_create_handle(sap, salen, hostname, hostname_len);
        if new.is_null() { return core::ptr::null_mut(); }
    }
}

pub unsafe fn nsm_reboot_lookup(net: *const net, info: *const lockd_reboot) -> *mut nsm_handle {
    let ln = net_generic(net, lockd_net_id);
    spin_lock(&mut nsm_lock);
    let cached = nsm_lookup_priv(&(*ln).nsm_handles, &(*info).priv_);
    if !cached.is_null() { refcount_inc(&mut (*cached).sm_count); }
    spin_unlock(&mut nsm_lock);
    cached
}

pub unsafe fn nsm_release(nsm: *mut nsm_handle) {
    if refcount_dec_and_lock(&mut (*nsm).sm_count, &mut nsm_lock) { list_del(&mut (*nsm).sm_link); spin_unlock(&mut nsm_lock); kfree(nsm as *mut _); }
}

unsafe fn encode_nsm_string(xdr: *mut xdr_stream, string: *const i8) { let len = strlen(string); let p = xdr_reserve_space(xdr, 4 + len); xdr_encode_opaque(p, string, len); }
unsafe fn encode_mon_name(xdr: *mut xdr_stream, argp: *const nsm_args) { encode_nsm_string(xdr, (*argp).mon_name); }
unsafe fn encode_my_id(xdr: *mut xdr_stream, argp: *const nsm_args) { encode_nsm_string(xdr, (*argp).nodename); let p = xdr_reserve_space(xdr, 12) as *mut u32; *p = cpu_to_be32((*argp).prog); *p.add(1) = cpu_to_be32((*argp).vers); *p.add(2) = cpu_to_be32((*argp).proc); }
unsafe fn encode_mon_id(xdr: *mut xdr_stream, argp: *const nsm_args) { encode_mon_name(xdr, argp); encode_my_id(xdr, argp); }
unsafe fn encode_priv(xdr: *mut xdr_stream, argp: *const nsm_args) { let p = xdr_reserve_space(xdr, SM_PRIV_SIZE); xdr_encode_opaque_fixed(p, (*argp).priv_.as_ref().unwrap().data.as_ptr(), SM_PRIV_SIZE); }
unsafe fn nsm_xdr_enc_mon(_req: *mut rpc_rqst, xdr: *mut xdr_stream, argp: *const core::ffi::c_void) { encode_mon_id(xdr, argp as *const _); encode_priv(xdr, argp as *const _); }
unsafe fn nsm_xdr_enc_unmon(_req: *mut rpc_rqst, xdr: *mut xdr_stream, argp: *const core::ffi::c_void) { encode_mon_id(xdr, argp as *const _); }
unsafe fn nsm_xdr_dec_stat_res(_rqstp: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut core::ffi::c_void) -> i32 { let p = xdr_inline_decode(xdr, 8); if p.is_null() { return -EIO; } (*((data) as *mut nsm_res)).status = be32_to_cpup(p); (*((data) as *mut nsm_res)).state = be32_to_cpup(p.add(1)); 0 }
unsafe fn nsm_xdr_dec_stat(_rqstp: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut core::ffi::c_void) -> i32 { let p = xdr_inline_decode(xdr, 4); if p.is_null() { return -EIO; } (*(data as *mut nsm_res)).state = be32_to_cpup(p); 0 }

const SM_my_name_sz: usize = 1 + XDR_QUADLEN(SM_MAXSTRLEN);
const SM_my_id_sz: usize = SM_my_name_sz + 3;
const SM_mon_name_sz: usize = 1 + XDR_QUADLEN(SM_MAXSTRLEN);
const SM_mon_id_sz: usize = SM_mon_name_sz + SM_my_id_sz;
const SM_priv_sz: usize = XDR_QUADLEN(SM_PRIV_SIZE);
const SM_mon_sz: usize = SM_mon_id_sz + SM_priv_sz;
const SM_monres_sz: usize = 2;
const SM_unmonres_sz: usize = 1;

static mut nsm_version1_counts: [u32; 2] = [0; 2];
static mut nsm_stats: rpc_stat = unsafe { core::mem::zeroed() };

static nsm_procedures: [rpc_procinfo; 2] = [
    rpc_procinfo { p_proc: NsmProc::Mon as u32, p_encode: nsm_xdr_enc_mon, p_decode: nsm_xdr_dec_stat_res, p_arglen: SM_mon_sz, p_replen: SM_monres_sz, p_statidx: NsmProc::Mon as u32, p_name: b"MONITOR\0".as_ptr() as *const i8 },
    rpc_procinfo { p_proc: NsmProc::Unmon as u32, p_encode: nsm_xdr_enc_unmon, p_decode: nsm_xdr_dec_stat, p_arglen: SM_mon_id_sz, p_replen: SM_unmonres_sz, p_statidx: NsmProc::Unmon as u32, p_name: b"UNMONITOR\0".as_ptr() as *const i8 },
];
static nsm_version1: rpc_version = rpc_version { number: 1, nrprocs: 2, procs: nsm_procedures.as_ptr(), counts: unsafe { nsm_version1_counts.as_ptr() } };
static nsm_version: [*const rpc_version; 2] = [core::ptr::null(), &nsm_version1];
static nsm_program_def: rpc_program = rpc_program { name: b"statd\0".as_ptr() as *const i8, number: NSM_PROGRAM, nrvers: 2, version: nsm_version.as_ptr(), stats: unsafe { &nsm_stats } };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
