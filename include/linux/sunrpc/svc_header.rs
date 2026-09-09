/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/include/linux/sunrpc/svc.h. */

#[repr(C)]
pub struct svc_pool {
    pub sp_id: ::core::ffi::c_uint,
    pub sp_nrthreads: ::core::ffi::c_uint,
    pub sp_nrthrmin: ::core::ffi::c_uint,
    pub sp_nrthrmax: ::core::ffi::c_uint,
    pub sp_xprts: lwq,
    pub sp_all_threads: list_head,
    pub sp_idle_threads: llist_head,
    pub sp_messages_arrived: percpu_counter,
    pub sp_sockets_queued: percpu_counter,
    pub sp_threads_woken: percpu_counter,
    pub sp_flags: ::core::ffi::c_ulong,
}

pub const SP_TASK_PENDING: ::core::ffi::c_uint = 0;
pub const SP_NEED_VICTIM: ::core::ffi::c_uint = 1;
pub const SP_VICTIM_REMAINS: ::core::ffi::c_uint = 2;
pub const SP_TASK_STARTING: ::core::ffi::c_uint = 3;

#[repr(C)]
pub struct svc_serv {
    pub sv_programs: *mut svc_program,
    pub sv_stats: *mut svc_stat,
    pub sv_lock: spinlock_t,
    pub sv_nprogs: ::core::ffi::c_uint,
    pub sv_nrthreads: ::core::ffi::c_uint,
    pub sv_max_payload: ::core::ffi::c_uint,
    pub sv_max_mesg: ::core::ffi::c_uint,
    pub sv_xdrsize: ::core::ffi::c_uint,
    pub sv_permsocks: list_head,
    pub sv_tempsocks: list_head,
    pub sv_tmpcnt: ::core::ffi::c_int,
    pub sv_temptimer: timer_list,
    pub sv_name: *mut ::core::ffi::c_char,
    pub sv_is_pooled: bool,
    pub sv_pools: *mut svc_pool,
    pub sv_threadfn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct svc_info { pub serv: *mut svc_serv, pub mutex: *mut mutex }

pub const RPCSVC_MAXPAYLOAD: ::core::ffi::c_uint = 4 * 1024 * 1024;
pub const RPCSVC_MAXPAYLOAD_TCP: ::core::ffi::c_uint = RPCSVC_MAXPAYLOAD;
pub const RPCSVC_MAXPAYLOAD_UDP: ::core::ffi::c_uint = 32 * 1024;

extern "C" { pub fn svc_destroy(svcp: *mut *mut svc_serv); pub fn svc_max_payload(rqstp: *const svc_rqst) -> u32; }

#[inline]
pub unsafe fn svc_serv_maxpages(serv: *const svc_serv) -> ::core::ffi::c_ulong {
    ((*serv).sv_max_mesg as ::core::ffi::c_ulong + PAGE_SIZE as ::core::ffi::c_ulong - 1) / PAGE_SIZE as ::core::ffi::c_ulong + 3
}

#[repr(C)]
pub struct svc_rqst {
    pub rq_all: list_head, pub rq_idle: llist_node, pub rq_rcu_head: rcu_head, pub rq_xprt: *mut svc_xprt,
    pub rq_addr: sockaddr_storage, pub rq_addrlen: usize, pub rq_daddr: sockaddr_storage, pub rq_daddrlen: usize,
    pub rq_server: *mut svc_serv, pub rq_pool: *mut svc_pool, pub rq_procinfo: *const svc_procedure,
    pub rq_authop: *mut auth_ops, pub rq_cred: svc_cred, pub rq_xprt_ctxt: *mut ::core::ffi::c_void,
    pub rq_deferred: *mut svc_deferred_req, pub rq_arg: xdr_buf, pub rq_arg_stream: xdr_stream,
    pub rq_res_stream: xdr_stream, pub rq_scratch_folio: *mut folio, pub rq_res: xdr_buf,
    pub rq_maxpages: ::core::ffi::c_ulong, pub rq_pages_nfree: ::core::ffi::c_ulong,
    pub rq_pages: *mut *mut page, pub rq_respages: *mut *mut page, pub rq_next_page: *mut *mut page, pub rq_page_end: *mut *mut page,
    pub rq_fbatch: folio_batch, pub rq_bvec: *mut bio_vec, pub rq_xid: __be32, pub rq_prog: u32, pub rq_vers: u32,
    pub rq_proc: u32, pub rq_prot: u32, pub rq_flags: ::core::ffi::c_ulong, pub rq_qtime: ktime_t,
    pub rq_argp: *mut ::core::ffi::c_void, pub rq_resp: *mut ::core::ffi::c_void, pub rq_accept_statp: *mut __be32,
    pub rq_auth_data: *mut ::core::ffi::c_void, pub rq_auth_stat: __be32, pub rq_auth_slack: ::core::ffi::c_int,
    pub rq_reserved: ::core::ffi::c_int, pub rq_stime: ktime_t, pub rq_chandle: cache_req,
    pub rq_client: *mut auth_domain, pub rq_gssclient: *mut auth_domain, pub rq_task: *mut task_struct,
    pub rq_bc_net: *mut net, pub rq_err: ::core::ffi::c_int, pub bc_to_initval: ::core::ffi::c_ulong,
    pub bc_to_retries: ::core::ffi::c_uint, pub rq_status_counter: ::core::ffi::c_uint, pub rq_private: *mut ::core::ffi::c_void,
}

pub const RQ_SECURE: u32 = 0; pub const RQ_LOCAL: u32 = 1; pub const RQ_USEDEFERRAL: u32 = 2;
pub const RQ_DROPME: u32 = 3; pub const RQ_VICTIM: u32 = 4; pub const RQ_DATA: u32 = 5;
#[inline] pub unsafe fn SVC_NET(r: *mut svc_rqst) -> *mut net { if !(*r).rq_xprt.is_null() { (*(*r).rq_xprt).xpt_net } else { (*r).rq_bc_net } }

#[inline] pub unsafe fn svc_addr_in(r: *const svc_rqst) -> *mut sockaddr_in { &(*r).rq_addr as *const _ as *mut _ }
#[inline] pub unsafe fn svc_addr_in6(r: *const svc_rqst) -> *mut sockaddr_in6 { &(*r).rq_addr as *const _ as *mut _ }
#[inline] pub unsafe fn svc_addr(r: *const svc_rqst) -> *mut sockaddr { &(*r).rq_addr as *const _ as *mut _ }
#[inline] pub unsafe fn svc_daddr_in(r: *const svc_rqst) -> *mut sockaddr_in { &(*r).rq_daddr as *const _ as *mut _ }
#[inline] pub unsafe fn svc_daddr_in6(r: *const svc_rqst) -> *mut sockaddr_in6 { &(*r).rq_daddr as *const _ as *mut _ }
#[inline] pub unsafe fn svc_daddr(r: *const svc_rqst) -> *mut sockaddr { &(*r).rq_daddr as *const _ as *mut _ }

#[inline] pub unsafe fn svc_thread_should_stop(r: *mut svc_rqst) -> bool {
    if test_and_clear_bit(SP_NEED_VICTIM, &mut (*(*r).rq_pool).sp_flags) { set_bit(RQ_VICTIM, &mut (*r).rq_flags); }
    test_bit(RQ_VICTIM, &(*r).rq_flags)
}
#[inline] pub unsafe fn svc_thread_init_status(r: *mut svc_rqst, err: ::core::ffi::c_int) { store_release_wake_up(&mut (*r).rq_err, err); if err != 0 { kthread_exit(1); } }

#[repr(C)] pub struct svc_deferred_req { pub prot: u32, pub xprt: *mut svc_xprt, pub addr: sockaddr_storage, pub addrlen: usize, pub daddr: sockaddr_storage, pub daddrlen: usize, pub xprt_ctxt: *mut ::core::ffi::c_void, pub handle: cache_deferred_req, pub argslen: ::core::ffi::c_int, pub args: [__be32; 0] }
#[repr(C)] pub union svc_process_info { pub dispatch: Option<unsafe extern "C" fn(*mut svc_rqst) -> ::core::ffi::c_int>, pub mismatch: svc_process_mismatch }
#[repr(C)] pub struct svc_process_mismatch { pub lovers: ::core::ffi::c_uint, pub hivers: ::core::ffi::c_uint }
#[repr(C)] pub struct svc_program { pub pg_prog: u32, pub pg_lovers: u32, pub pg_hivers: u32, pub pg_nvers: u32, pub pg_vers: *const *const svc_version, pub pg_name: *mut ::core::ffi::c_char, pub pg_class: *mut ::core::ffi::c_char, pub pg_authenticate: Option<unsafe extern "C" fn(*mut svc_rqst) -> svc_auth_status>, pub pg_init_request: Option<unsafe extern "C" fn(*mut svc_rqst,*const svc_program,*mut svc_process_info)->__be32>, pub pg_rpcbind_set: Option<unsafe extern "C" fn(*mut net,*const svc_program,u32,::core::ffi::c_int,u16,u16)->::core::ffi::c_int> }
#[repr(C)] pub struct svc_version { pub vs_vers: u32, pub vs_nproc: u32, pub vs_proc: *const svc_procedure, pub vs_xdrsize: u32, pub vs_hidden: bool, pub vs_rpcb_optnl: bool, pub vs_need_cong_ctrl: bool, pub vs_dispatch: Option<unsafe extern "C" fn(*mut svc_rqst)->::core::ffi::c_int> }
#[repr(C)] pub struct svc_procedure { pub pc_func: Option<unsafe extern "C" fn(*mut svc_rqst)->__be32>, pub pc_decode: Option<unsafe extern "C" fn(*mut svc_rqst,*mut xdr_stream)->bool>, pub pc_encode: Option<unsafe extern "C" fn(*mut svc_rqst,*mut xdr_stream)->bool>, pub pc_release: Option<unsafe extern "C" fn(*mut svc_rqst)>, pub pc_argsize:u32, pub pc_argzero:u32, pub pc_ressize:u32, pub pc_cachetype:u32, pub pc_xdrressize:u32, pub pc_name:*const ::core::ffi::c_char }

extern "C" {
    pub fn sunrpc_set_pool_mode(val:*const ::core::ffi::c_char)->::core::ffi::c_int; pub fn sunrpc_get_pool_mode(val:*mut ::core::ffi::c_char,size:usize)->::core::ffi::c_int;
    pub fn svc_rpcb_cleanup(serv:*mut svc_serv,net:*mut net); pub fn svc_bind(serv:*mut svc_serv,net:*mut net)->::core::ffi::c_int;
    pub fn svc_create(p:*mut svc_program,n:u32,f:Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)->::core::ffi::c_int>)->*mut svc_serv;
    pub fn svc_rqst_replace_page(r:*mut svc_rqst,p:*mut page)->bool; pub fn svc_rqst_release_pages(r:*mut svc_rqst); pub fn svc_new_thread(s:*mut svc_serv,p:*mut svc_pool)->::core::ffi::c_int; pub fn svc_exit_thread(r:*mut svc_rqst);
    pub fn svc_create_pooled(p:*mut svc_program,n:u32,st:*mut svc_stat,b:u32,f:Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)->::core::ffi::c_int>)->*mut svc_serv;
    pub fn svc_set_pool_threads(s:*mut svc_serv,p:*mut svc_pool,min:u32,max:u32)->::core::ffi::c_int; pub fn svc_set_num_threads(s:*mut svc_serv,min:u32,n:u32)->::core::ffi::c_int; pub fn svc_serv_maxthreads(s:*const svc_serv)->u32; pub fn svc_pool_stats_open(s:*mut svc_info,f:*mut file)->::core::ffi::c_int; pub fn svc_process(r:*mut svc_rqst); pub fn svc_process_bc(req:*mut rpc_rqst,r:*mut svc_rqst); pub fn svc_register(s:*const svc_serv,n:*mut net,f:::core::ffi::c_int,pr:u16,po:u16)->::core::ffi::c_int;
    pub fn svc_wake_up(s:*mut svc_serv); pub fn svc_reserve(r:*mut svc_rqst,space:::core::ffi::c_int); pub fn svc_pool_wake_idle_thread(p:*mut svc_pool); pub fn svc_pool_for_cpu(s:*mut svc_serv)->*mut svc_pool; pub fn svc_serv_nrpools(s:*const svc_serv)->u32; pub fn svc_print_addr(r:*mut svc_rqst,b:*mut ::core::ffi::c_char,n:usize)->*mut ::core::ffi::c_char; pub fn svc_proc_name(r:*const svc_rqst)->*const ::core::ffi::c_char; pub fn svc_encode_result_payload(r:*mut svc_rqst,o:u32,l:u32)->::core::ffi::c_int; pub fn svc_fill_symlink_pathname(r:*mut svc_rqst,f:*mut kvec,p:*mut ::core::ffi::c_void,t:usize)->*mut ::core::ffi::c_char; pub fn svc_generic_init_request(r:*mut svc_rqst,p:*const svc_program,i:*mut svc_process_info)->__be32; pub fn svc_generic_rpcbind_set(n:*mut net,p:*const svc_program,v:u32,f: ::core::ffi::c_int,pr:u16,po:u16)->::core::ffi::c_int;
}
pub const RPC_MAX_ADDRBUFLEN:u32=63;

#[inline] pub unsafe fn svc_rqst_page_release(r:*mut svc_rqst,p:*mut page){if !folio_batch_add(&mut (*r).rq_fbatch,page_folio(p)){__folio_batch_release(&mut (*r).rq_fbatch)}}
#[inline] pub unsafe fn svc_reserve_auth(r:*mut svc_rqst,space:::core::ffi::c_int){svc_reserve(r,space+(*r).rq_auth_slack)}
#[inline] pub unsafe fn svcxdr_init_decode(r:*mut svc_rqst){let x=&mut (*r).rq_arg_stream;let b=&mut (*r).rq_arg;let a=b.head; WARN_ON(b.len != (*a).iov_len+b.page_len+(*b.tail).iov_len);b.len=(*a).iov_len+b.page_len+(*b.tail).iov_len;xdr_init_decode(x,b,(*a).iov_base,core::ptr::null_mut());xdr_set_scratch_folio(x,(*r).rq_scratch_folio)}
#[inline] pub unsafe fn svcxdr_init_encode(r:*mut svc_rqst){let x=&mut (*r).rq_res_stream;let b=&mut (*r).rq_res;let v=b.head;xdr_reset_scratch_buffer(x);x.buf=b;x.iov=v;x.p=v.iov_base.add(v.iov_len);x.end=v.iov_base.add(PAGE_SIZE);b.len=v.iov_len;x.page_ptr=b.pages.offset(-1);b.buflen=PAGE_SIZE*((*r).rq_page_end.offset_from(b.pages) as usize);x.rqst=core::ptr::null_mut()}
#[inline] pub unsafe fn svcxdr_encode_opaque_pages(r:*mut svc_rqst,x:*mut xdr_stream,p:*mut *mut page,base:u32,len:u32){xdr_write_pages(x,p,base,len);(*x).page_ptr=(*r).rq_next_page.offset(-1)}
#[inline] pub unsafe fn svcxdr_set_auth_slack(r:*mut svc_rqst,slack:::core::ffi::c_int){let x=&mut (*r).rq_res_stream;let b=&mut (*r).rq_res;let v=b.head;(*r).rq_auth_slack=slack;x.end=x.end.sub(XDR_QUADLEN(slack) as usize);b.buflen-=(*r).rq_auth_slack as usize;WARN_ON(x.iov!=v);WARN_ON(x.p>x.end)}
#[inline] pub unsafe fn svcxdr_set_accept_stat(r:*mut svc_rqst)->bool{let x=&mut (*r).rq_res_stream;(*r).rq_accept_statp=xdr_reserve_space(x,XDR_UNIT);if (*r).rq_accept_statp.is_null(){return false}*(*r).rq_accept_statp=rpc_success;true}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
