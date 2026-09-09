// SPDX-License-Identifier: GPL-2.0+
/*
 *  linux/net/sunrpc/gss_rpc_upcall.c
 *
 *  Copyright (C) 2012 Simo Sorce <simo@redhat.com>
 */

// The following types, constants, macros, and functions are supplied by the
// Linux/RPC headers and by gss_rpc_upcall.h.

pub const GSSPROXY_SOCK_PATHNAME: &[u8] = b"/var/run/gssproxy.sock\0";
pub const GSSPROXY_PROGRAM: u32 = 400112u32;
pub const GSSPROXY_VERS_1: u32 = 1u32;

pub const GSSX_NULL: usize = 0;
pub const GSSX_INDICATE_MECHS: usize = 1;
pub const GSSX_GET_CALL_CONTEXT: usize = 2;
pub const GSSX_IMPORT_AND_CANON_NAME: usize = 3;
pub const GSSX_EXPORT_CRED: usize = 4;
pub const GSSX_IMPORT_CRED: usize = 5;
pub const GSSX_ACQUIRE_CRED: usize = 6;
pub const GSSX_STORE_CRED: usize = 7;
pub const GSSX_INIT_SEC_CONTEXT: usize = 8;
pub const GSSX_ACCEPT_SEC_CONTEXT: usize = 9;
pub const GSSX_RELEASE_HANDLE: usize = 10;
pub const GSSX_GET_MIC: usize = 11;
pub const GSSX_VERIFY: usize = 12;
pub const GSSX_WRAP: usize = 13;
pub const GSSX_UNWRAP: usize = 14;
pub const GSSX_WRAP_SIZE_LIMIT: usize = 15;

// Encoding/decoding procedure table.  The procedure descriptors and encoder
// symbols are defined by the RPC/GSSX interface declarations.
static GSSP_PROCEDURES: [rpc_procinfo; 15] = [
    rpc_procinfo { p_proc: GSSX_INDICATE_MECHS, p_encode: gssx_enc_indicate_mechs, p_decode: gssx_dec_indicate_mechs, p_arglen: GSSX_ARG_indicate_mechs_sz, p_replen: GSSX_RES_indicate_mechs_sz, p_statidx: GSSX_INDICATE_MECHS, p_name: "INDICATE_MECHS" },
    rpc_procinfo { p_proc: GSSX_GET_CALL_CONTEXT, p_encode: gssx_enc_get_call_context, p_decode: gssx_dec_get_call_context, p_arglen: GSSX_ARG_get_call_context_sz, p_replen: GSSX_RES_get_call_context_sz, p_statidx: GSSX_GET_CALL_CONTEXT, p_name: "GET_CALL_CONTEXT" },
    rpc_procinfo { p_proc: GSSX_IMPORT_AND_CANON_NAME, p_encode: gssx_enc_import_and_canon_name, p_decode: gssx_dec_import_and_canon_name, p_arglen: GSSX_ARG_import_and_canon_name_sz, p_replen: GSSX_RES_import_and_canon_name_sz, p_statidx: GSSX_IMPORT_AND_CANON_NAME, p_name: "IMPORT_AND_CANON_NAME" },
    rpc_procinfo { p_proc: GSSX_EXPORT_CRED, p_encode: gssx_enc_export_cred, p_decode: gssx_dec_export_cred, p_arglen: GSSX_ARG_export_cred_sz, p_replen: GSSX_RES_export_cred_sz, p_statidx: GSSX_EXPORT_CRED, p_name: "EXPORT_CRED" },
    rpc_procinfo { p_proc: GSSX_IMPORT_CRED, p_encode: gssx_enc_import_cred, p_decode: gssx_dec_import_cred, p_arglen: GSSX_ARG_import_cred_sz, p_replen: GSSX_RES_import_cred_sz, p_statidx: GSSX_IMPORT_CRED, p_name: "IMPORT_CRED" },
    rpc_procinfo { p_proc: GSSX_ACQUIRE_CRED, p_encode: gssx_enc_acquire_cred, p_decode: gssx_dec_acquire_cred, p_arglen: GSSX_ARG_acquire_cred_sz, p_replen: GSSX_RES_acquire_cred_sz, p_statidx: GSSX_ACQUIRE_CRED, p_name: "ACQUIRE_CRED" },
    rpc_procinfo { p_proc: GSSX_STORE_CRED, p_encode: gssx_enc_store_cred, p_decode: gssx_dec_store_cred, p_arglen: GSSX_ARG_store_cred_sz, p_replen: GSSX_RES_store_cred_sz, p_statidx: GSSX_STORE_CRED, p_name: "STORE_CRED" },
    rpc_procinfo { p_proc: GSSX_INIT_SEC_CONTEXT, p_encode: gssx_enc_init_sec_context, p_decode: gssx_dec_init_sec_context, p_arglen: GSSX_ARG_init_sec_context_sz, p_replen: GSSX_RES_init_sec_context_sz, p_statidx: GSSX_INIT_SEC_CONTEXT, p_name: "INIT_SEC_CONTEXT" },
    rpc_procinfo { p_proc: GSSX_ACCEPT_SEC_CONTEXT, p_encode: gssx_enc_accept_sec_context, p_decode: gssx_dec_accept_sec_context, p_arglen: GSSX_ARG_accept_sec_context_sz, p_replen: GSSX_RES_accept_sec_context_sz, p_statidx: GSSX_ACCEPT_SEC_CONTEXT, p_name: "ACCEPT_SEC_CONTEXT" },
    rpc_procinfo { p_proc: GSSX_RELEASE_HANDLE, p_encode: gssx_enc_release_handle, p_decode: gssx_dec_release_handle, p_arglen: GSSX_ARG_release_handle_sz, p_replen: GSSX_RES_release_handle_sz, p_statidx: GSSX_RELEASE_HANDLE, p_name: "RELEASE_HANDLE" },
    rpc_procinfo { p_proc: GSSX_GET_MIC, p_encode: gssx_enc_get_mic, p_decode: gssx_dec_get_mic, p_arglen: GSSX_ARG_get_mic_sz, p_replen: GSSX_RES_get_mic_sz, p_statidx: GSSX_GET_MIC, p_name: "GET_MIC" },
    rpc_procinfo { p_proc: GSSX_VERIFY, p_encode: gssx_enc_verify, p_decode: gssx_dec_verify, p_arglen: GSSX_ARG_verify_sz, p_replen: GSSX_RES_verify_sz, p_statidx: GSSX_VERIFY, p_name: "VERIFY" },
    rpc_procinfo { p_proc: GSSX_WRAP, p_encode: gssx_enc_wrap, p_decode: gssx_dec_wrap, p_arglen: GSSX_ARG_wrap_sz, p_replen: GSSX_RES_wrap_sz, p_statidx: GSSX_WRAP, p_name: "WRAP" },
    rpc_procinfo { p_proc: GSSX_UNWRAP, p_encode: gssx_enc_unwrap, p_decode: gssx_dec_unwrap, p_arglen: GSSX_ARG_unwrap_sz, p_replen: GSSX_RES_unwrap_sz, p_statidx: GSSX_UNWRAP, p_name: "UNWRAP" },
    rpc_procinfo { p_proc: GSSX_WRAP_SIZE_LIMIT, p_encode: gssx_enc_wrap_size_limit, p_decode: gssx_dec_wrap_size_limit, p_arglen: GSSX_ARG_wrap_size_limit_sz, p_replen: GSSX_RES_wrap_size_limit_sz, p_statidx: GSSX_WRAP_SIZE_LIMIT, p_name: "WRAP_SIZE_LIMIT" },
];

static mut GSSP_PROGRAM: rpc_program = rpc_program { name: "gssproxy", number: GSSPROXY_PROGRAM, nrvers: 0, version: core::ptr::null(), stats: core::ptr::null() };

pub unsafe fn set_gssp_clnt(net: *mut net) -> i32 {
    let sn = net_generic(net, sunrpc_net_id);
    let mut clnt: *mut rpc_clnt = core::ptr::null_mut();
    mutex_lock(&mut (*sn).gssp_lock);
    let ret = gssp_rpc_create(net, &mut clnt);
    if ret == 0 {
        if !(*sn).gssp_clnt.is_null() { rpc_shutdown_client((*sn).gssp_clnt); }
        (*sn).gssp_clnt = clnt;
    }
    mutex_unlock(&mut (*sn).gssp_lock);
    ret
}

pub unsafe fn clear_gssp_clnt(sn: *mut sunrpc_net) {
    mutex_lock(&mut (*sn).gssp_lock);
    if !(*sn).gssp_clnt.is_null() { rpc_shutdown_client((*sn).gssp_clnt); (*sn).gssp_clnt = core::ptr::null_mut(); }
    mutex_unlock(&mut (*sn).gssp_lock);
}

unsafe fn gssp_rpc_create(net: *mut net, clnt: *mut *mut rpc_clnt) -> i32 {
    let args = rpc_create_args { net, protocol: XPRT_TRANSPORT_LOCAL, address: core::ptr::null_mut(), addrsize: 0, servername: "localhost", program: &GSSP_PROGRAM, version: GSSPROXY_VERS_1, authflavor: RPC_AUTH_NULL, flags: RPC_CLNT_CREATE_NOPING | RPC_CLNT_CREATE_CONNECTED | RPC_CLNT_CREATE_NO_IDLE_TIMEOUT };
    let c = rpc_create(&args);
    if IS_ERR(c) { *clnt = core::ptr::null_mut(); return PTR_ERR(c) as i32; }
    *clnt = c; 0
}

unsafe fn get_gssp_clnt(sn: *mut sunrpc_net) -> *mut rpc_clnt {
    mutex_lock(&mut (*sn).gssp_lock);
    let c = (*sn).gssp_clnt;
    if !c.is_null() { refcount_inc(&mut (*c).cl_count); }
    mutex_unlock(&mut (*sn).gssp_lock); c
}

unsafe fn gssp_call(net: *mut net, msg: *mut rpc_message) -> i32 {
    let sn = net_generic(net, sunrpc_net_id); let c = get_gssp_clnt(sn);
    if c.is_null() { return -EIO; }
    let mut status = rpc_call_sync(c, msg, 0);
    if status < 0 { status = match status { -EPROTONOSUPPORT => -EINVAL, -ECONNREFUSED | -ETIMEDOUT | -ENOTCONN => -EAGAIN, -ERESTARTSYS if signalled() => -EINTR, _ => status }; }
    rpc_release_client(c); status
}

unsafe fn gssp_free_receive_pages(arg: *mut gssx_arg_accept_sec_context) {
    let mut i = 0; while i < (*arg).npages && !(*arg).pages.add(i).read().is_null() { __free_page((*arg).pages.add(i).read()); i += 1; }
    kfree((*arg).pages as *mut core::ffi::c_void);
}

unsafe fn gssp_alloc_receive_pages(arg: *mut gssx_arg_accept_sec_context) -> i32 {
    (*arg).npages = DIV_ROUND_UP(NGROUPS_MAX * 4, PAGE_SIZE);
    (*arg).pages = kzalloc_objs((*arg).npages);
    if (*arg).pages.is_null() { return -ENOMEM; }
    for i in 0..(*arg).npages { (*arg).pages.add(i).write(alloc_page(GFP_KERNEL)); if (*arg).pages.add(i).read().is_null() { gssp_free_receive_pages(arg); return -ENOMEM; } }
    0
}

unsafe fn gssp_stringify(netobj: *const xdr_netobj) -> *mut i8 { kmemdup_nul((*netobj).data, (*netobj).len, GFP_KERNEL) }

unsafe fn gssp_hostbased_service(principal: *mut *mut i8) {
    if (*principal).is_null() { return; }
    let mut c = strchr(*principal, b'@' as i32);
    if !c.is_null() { *c = 0; c = strchr(*principal, b'/' as i32); if !c.is_null() { *c = b'@' as i8; } }
    if c.is_null() { kfree(*principal as *mut core::ffi::c_void); *principal = core::ptr::null_mut(); }
}

pub const GSSX_MAX_OUT_HANDLE: usize = 128;
pub const GSSX_MAX_SRC_PRINC: usize = 256;
pub const GSSX_KMEMBUF: usize = GSSX_max_output_handle_sz + GSSX_max_oid_sz + GSSX_max_princ_sz + core::mem::size_of::<svc_cred>();

pub unsafe fn gssp_accept_sec_context_upcall(net: *mut net, data: *mut gssp_upcall_data) -> i32 {
    let mut ctxh = gssx_ctx { state: (*data).in_handle };
    let mut rctxh = gssx_ctx { exported_context_token: gssx_buffer { len: GSSX_max_output_handle_sz, data: core::ptr::null_mut() }, mech: gssx_buffer { len: GSS_OID_MAX_LEN, data: core::ptr::null_mut() }, targ_name: Default::default(), src_name: Default::default() };
    rctxh.targ_name.display_name.len = GSSX_max_princ_sz;
    rctxh.src_name.display_name.len = GSSX_max_princ_sz;
    let mut arg = gssx_arg_accept_sec_context { context_handle: core::ptr::null_mut(), input_token: (*data).in_token, npages: 0, pages: core::ptr::null_mut() };
    let mut res = gssx_res_accept_sec_context { context_handle: &mut rctxh, output_token: &mut (*data).out_token, status: Default::default(), options: Default::default() };
    let mut msg = rpc_message { rpc_proc: &GSSP_PROCEDURES[GSSX_ACCEPT_SEC_CONTEXT], rpc_argp: &mut arg, rpc_resp: &mut res, rpc_cred: core::ptr::null_mut() };
    if (*data).in_handle.len != 0 { arg.context_handle = &mut ctxh; }
    (*res.output_token).len = GSSX_max_output_token_sz;
    let ret = gssp_alloc_receive_pages(&mut arg); if ret != 0 { return ret; }
    let ret = gssp_call(net, &mut msg); gssp_free_receive_pages(&mut arg);
    (*data).major_status = res.status.major_status; (*data).minor_status = res.status.minor_status;
    let mut client_name = xdr_netobj { len: 0, data: core::ptr::null_mut() }; let mut target_name = xdr_netobj { len: 0, data: core::ptr::null_mut() };
    if !res.context_handle.is_null() {
        (*data).out_handle = rctxh.exported_context_token; (*data).mech_oid.len = rctxh.mech.len;
        if !rctxh.mech.data.is_null() { memcpy((*data).mech_oid.data, rctxh.mech.data, (*data).mech_oid.len); kfree(rctxh.mech.data as *mut core::ffi::c_void); }
        client_name = rctxh.src_name.display_name; target_name = rctxh.targ_name.display_name;
    }
    if res.options.count == 1 { let value = &mut (*res.options.data).value; if value.len == 1 { (*data).creds = *(value.data as *const svc_cred); (*data).found_creds = 1; } kfree(value.data as *mut core::ffi::c_void); }
    if res.options.count != 0 { kfree(res.options.data as *mut core::ffi::c_void); }
    if (*data).found_creds != 0 { if !client_name.data.is_null() { (*data).creds.cr_raw_principal = gssp_stringify(&client_name); (*data).creds.cr_principal = gssp_stringify(&client_name); gssp_hostbased_service(&mut (*data).creds.cr_principal); } if !target_name.data.is_null() { (*data).creds.cr_targ_princ = gssp_stringify(&target_name); gssp_hostbased_service(&mut (*data).creds.cr_targ_princ); } }
    kfree(client_name.data as *mut core::ffi::c_void); kfree(target_name.data as *mut core::ffi::c_void); ret
}

pub unsafe fn gssp_free_upcall_data(data: *mut gssp_upcall_data) {
    kfree((*data).in_handle.data as *mut core::ffi::c_void); kfree((*data).out_handle.data as *mut core::ffi::c_void); kfree((*data).out_token.data as *mut core::ffi::c_void); free_svc_cred(&mut (*data).creds);
}

static mut GSSP_VERSION1_COUNTS: [u32; 15] = [0; 15];
static GSSP_VERSION1: rpc_version = rpc_version { number: GSSPROXY_VERS_1, nrprocs: 15, procs: &GSSP_PROCEDURES, counts: unsafe { &GSSP_VERSION1_COUNTS } };
static GSSP_VERSION: [*const rpc_version; 2] = [core::ptr::null(), &GSSP_VERSION1];
static mut GSSP_STATS: rpc_stat = rpc_stat {};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
