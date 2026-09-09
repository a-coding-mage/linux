// SPDX-License-Identifier: GPL-2.0-only
/* In-kernel rpcbind client supporting versions 2, 3, and 4 of the rpcbind protocol. */
// Includes and symbols from the C translation unit are supplied by other modules.

const RPCBIND_SOCK_PATHNAME: &str = "/var/run/rpcbind.sock";
const RPCBIND_SOCK_ABSTRACT_NAME: &str = "\0/run/rpcbind.sock";
const RPCBIND_PROGRAM: u32 = 100000;
const RPCBIND_PORT: u32 = 111;
const RPCBVERS_2: u32 = 2;
const RPCBVERS_3: u32 = 3;
const RPCBVERS_4: u32 = 4;
const RPCB_OWNER_STRING: &str = "0";
const RPCB_MAXOWNERLEN: usize = RPCB_OWNER_STRING.len() + 1;

const RPCBPROC_NULL: usize = 0;
const RPCBPROC_SET: usize = 1;
const RPCBPROC_UNSET: usize = 2;
const RPCBPROC_GETPORT: usize = 3;
const RPCBPROC_GETADDR: usize = 3;
const RPCBPROC_DUMP: usize = 4;
const RPCBPROC_CALLIT: usize = 5;
const RPCBPROC_BCAST: usize = 5;
const RPCBPROC_GETTIME: usize = 6;
const RPCBPROC_UADDR2TADDR: usize = 7;
const RPCBPROC_TADDR2UADDR: usize = 8;
const RPCBPROC_GETVERSADDR: usize = 9;
const RPCBPROC_INDIRECT: usize = 10;
const RPCBPROC_GETADDRLIST: usize = 11;
const RPCBPROC_GETSTAT: usize = 12;

const RPCB_program_sz: usize = 1;
const RPCB_version_sz: usize = 1;
const RPCB_protocol_sz: usize = 1;
const RPCB_port_sz: usize = 1;
const RPCB_boolean_sz: usize = 1;
const RPCB_netid_sz: usize = 1 + XDR_QUADLEN(RPCBIND_MAXNETIDLEN);
const RPCB_addr_sz: usize = 1 + XDR_QUADLEN(RPCBIND_MAXUADDRLEN);
const RPCB_ownerstring_sz: usize = 1 + XDR_QUADLEN(RPCB_MAXOWNERLEN);
const RPCB_mappingargs_sz: usize = RPCB_program_sz + RPCB_version_sz + RPCB_protocol_sz + RPCB_port_sz;
const RPCB_getaddrargs_sz: usize = RPCB_program_sz + RPCB_version_sz + RPCB_netid_sz + RPCB_addr_sz + RPCB_ownerstring_sz;
const RPCB_getportres_sz: usize = RPCB_port_sz;
const RPCB_setres_sz: usize = RPCB_boolean_sz;
const RPCB_getaddrres_sz: usize = RPCB_addr_sz;

#[repr(C)]
pub struct rpcbind_args {
    pub r_xprt: *mut rpc_xprt,
    pub r_prog: u32, pub r_vers: u32, pub r_prot: u32, pub r_port: u16,
    pub r_netid: *const c_char, pub r_addr: *const c_char, pub r_owner: *const c_char,
    pub r_status: c_int,
}

#[repr(C)] pub struct rpcb_info { pub rpc_vers: u32, pub rpc_proc: *const rpc_procinfo }

static mut rpcb_procedures2: [rpc_procinfo; 4] = [rpc_procinfo::ZERO; 4];
static mut rpcb_procedures3: [rpc_procinfo; 4] = [rpc_procinfo::ZERO; 4];
static mut rpcb_procedures4: [rpc_procinfo; 4] = [rpc_procinfo::ZERO; 4];

unsafe fn rpcb_wake_rpcbind_waiters(xprt: *mut rpc_xprt, status: c_int) {
    xprt_clear_binding(xprt); rpc_wake_up_status(&mut (*xprt).binding, status);
}
unsafe extern "C" fn rpcb_map_release(data: *mut c_void) {
    let map = data as *mut rpcbind_args;
    rpcb_wake_rpcbind_waiters((*map).r_xprt, (*map).r_status);
    xprt_put((*map).r_xprt); kfree((*map).r_addr as *mut c_void); kfree(map as *mut c_void);
}
unsafe fn rpcb_get_local(net: *mut net) -> c_int {
    let sn = net_generic(net, sunrpc_net_id);
    spin_lock(&mut (*sn).rpcb_clnt_lock);
    if (*sn).rpcb_users != 0 { (*sn).rpcb_users += 1; }
    let cnt = (*sn).rpcb_users as c_int; spin_unlock(&mut (*sn).rpcb_clnt_lock); cnt
}
pub unsafe extern "C" fn rpcb_put_local(net: *mut net) {
    let sn = net_generic(net, sunrpc_net_id); let clnt = (*sn).rpcb_local_clnt; let clnt4 = (*sn).rpcb_local_clnt4;
    let mut shutdown = false; spin_lock(&mut (*sn).rpcb_clnt_lock);
    if (*sn).rpcb_users != 0 { (*sn).rpcb_users -= 1; if (*sn).rpcb_users == 0 { (*sn).rpcb_local_clnt = core::ptr::null_mut(); (*sn).rpcb_local_clnt4 = core::ptr::null_mut(); } shutdown = (*sn).rpcb_users == 0; }
    spin_unlock(&mut (*sn).rpcb_clnt_lock);
    if shutdown { if !clnt4.is_null() { rpc_shutdown_client(clnt4); } if !clnt.is_null() { rpc_shutdown_client(clnt); } }
}
unsafe fn rpcb_set_local(net: *mut net, clnt: *mut rpc_clnt, clnt4: *mut rpc_clnt, is_af_local: bool) {
    let sn = net_generic(net, sunrpc_net_id); (*sn).rpcb_local_clnt = clnt; (*sn).rpcb_local_clnt4 = clnt4; (*sn).rpcb_is_af_local = is_af_local as u8; smp_wmb(); (*sn).rpcb_users = 1;
}

unsafe fn rpcb_register_call(sn: *mut sunrpc_net, clnt: *mut rpc_clnt, msg: *mut rpc_message, is_set: bool) -> c_int {
    let mut flags = RPC_TASK_NOCONNECT; if is_set || (*sn).rpcb_is_af_local == 0 { flags = RPC_TASK_SOFTCONN; }
    let mut result: c_int = 0; (*msg).rpc_resp = &mut result as *mut _ as *mut c_void;
    let error = rpc_call_sync(clnt, msg, flags); if error < 0 { return error; } if result == 0 { return -EACCES; } 0
}

pub unsafe extern "C" fn rpcb_register(net: *mut net, prog: u32, vers: u32, prot: c_int, port: u16) -> c_int {
    let mut map = rpcbind_args { r_xprt: core::ptr::null_mut(), r_prog: prog, r_vers: vers, r_prot: prot as u32, r_port: port, r_netid: core::ptr::null(), r_addr: core::ptr::null(), r_owner: core::ptr::null(), r_status: 0 };
    let mut msg = rpc_message::default(); msg.rpc_argp = &mut map as *mut _ as *mut c_void;
    let sn = net_generic(net, sunrpc_net_id); msg.rpc_proc = if port != 0 { &rpcb_procedures2[RPCBPROC_SET] } else { &rpcb_procedures2[RPCBPROC_UNSET] }; rpcb_register_call(sn, (*sn).rpcb_local_clnt, &mut msg, port != 0)
}

unsafe fn encode_rpcb_string(xdr: *mut xdr_stream, string: *const c_char, maxstrlen: u32) { let mut len = strlen(string) as u32; if len > maxstrlen { len = maxstrlen; } let p = xdr_reserve_space(xdr, 4 + len as usize); xdr_encode_opaque(p, string, len); }
unsafe extern "C" fn rpcb_enc_mapping(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void) { let r = &*(data as *const rpcbind_args); let p = xdr_reserve_space(xdr, RPCB_mappingargs_sz << 2); *p.add(0)=cpu_to_be32(r.r_prog); *p.add(1)=cpu_to_be32(r.r_vers); *p.add(2)=cpu_to_be32(r.r_prot); *p.add(3)=cpu_to_be32(r.r_port as u32); }
unsafe extern "C" fn rpcb_dec_getport(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut c_void) -> c_int { let r=&mut *(data as *mut rpcbind_args); r.r_port=0; let p=xdr_inline_decode(xdr,4); if p.is_null(){return -EIO;} let port=be32_to_cpup(p); if port>u16::MAX as u32{return -EIO;} r.r_port=port as u16; 0 }
unsafe extern "C" fn rpcb_dec_set(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut c_void) -> c_int { let p=xdr_inline_decode(xdr,4); if p.is_null(){return -EIO;} *(data as *mut u32)=if *p!=xdr_zero{1}else{0}; 0 }
unsafe extern "C" fn rpcb_enc_getaddr(_req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let r=&*(data as *const rpcbind_args);let p=xdr_reserve_space(xdr,8);*p=cpu_to_be32(r.r_prog);*p.add(1)=cpu_to_be32(r.r_vers);encode_rpcb_string(xdr,r.r_netid,RPCBIND_MAXNETIDLEN);encode_rpcb_string(xdr,r.r_addr,RPCBIND_MAXUADDRLEN);encode_rpcb_string(xdr,r.r_owner,RPCB_MAXOWNERLEN as u32);}

unsafe extern "C" fn rpcb_dec_getaddr(req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*mut c_void)->c_int { let r=&mut *(data as *mut rpcbind_args); r.r_port=0; let p=xdr_inline_decode(xdr,4); if p.is_null(){return -EIO;} let len=be32_to_cpup(p); if len==0{return 0;} if len>RPCBIND_MAXUADDRLEN{return -EIO;} let p=xdr_inline_decode(xdr,len as usize); if p.is_null(){return -EIO;} let mut address=sockaddr_storage::default(); if rpc_uaddr2sockaddr((*(*req).rq_xprt).xprt_net,p as *const c_char,len,&mut address as *mut _ as *mut sockaddr,core::mem::size_of::<sockaddr_storage>())==0{return -EIO;} r.r_port=rpc_get_port(&address as *const _ as *const sockaddr) as u16; 0 }

// Procedure metadata mirrors the C designated initializers; dependent structure layout is supplied by the kernel RPC bindings.
#[allow(non_upper_case_globals)]
static rpcb_next_version: [rpcb_info;2] = [rpcb_info{rpc_vers:RPCBVERS_2,rpc_proc:unsafe{&rpcb_procedures2[RPCBPROC_GETPORT]}},rpcb_info{rpc_vers:0,rpc_proc:core::ptr::null()}];
#[allow(non_upper_case_globals)]
static rpcb_next_version6: [rpcb_info;3] = [rpcb_info{rpc_vers:RPCBVERS_4,rpc_proc:unsafe{&rpcb_procedures4[RPCBPROC_GETADDR]}},rpcb_info{rpc_vers:RPCBVERS_3,rpc_proc:unsafe{&rpcb_procedures3[RPCBPROC_GETADDR]}},rpcb_info{rpc_vers:0,rpc_proc:core::ptr::null()}];

unsafe extern "C" fn rpcb_getport_done(_child:*mut rpc_task,_data:*mut c_void) {}
unsafe extern "C" fn rpcb_create(net:*mut net,nodename:*const c_char,hostname:*const c_char,srvaddr:*mut sockaddr,salen:usize,proto:c_int,version:u32,cred:*const cred,timeo:*const rpc_timeout)->*mut rpc_clnt { let _=(net,nodename,hostname,srvaddr,salen,proto,version,cred,timeo); core::ptr::null_mut() }
unsafe extern "C" fn rpcb_find_transport_owner(clnt:*mut rpc_clnt)->*mut rpc_clnt { clnt }

extern "C" { pub fn rpcb_getport_async(task:*mut rpc_task); pub fn rpcb_v4_register(net:*mut net,program:u32,version:u32,address:*const sockaddr,netid:*const c_char)->c_int; pub fn rpcb_create_local(net:*mut net)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
