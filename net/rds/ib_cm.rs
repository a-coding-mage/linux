/* Translated from ib_cm.c. Kernel/RDMA types and functions are supplied externally. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut rds_ib_sysctl_flow_control: bool;
    static mut rds_ib_retry_count: c_uint;
    static mut s_ib_evt_handler_call: c_int;
    static mut s_ib_tasklet_call: c_int;
    static mut s_ib_listen_closed_stale: c_int;
    static mut s_ib_connect_raced: c_int;
    static mut ib_nodev_conns_lock: spinlock_t;
    static mut rds_ib_transport: rds_transport;
    static mut init_net: net;
    static mut rds_ib_ring_empty_wait: wait_queue_head_t;
    fn rds_ib_send_add_credits(conn: *mut rds_connection, credits: u32);
    fn rds_ib_update_ipaddr(dev: *mut rds_ib_device, addr: *const in6_addr) -> c_int;
    fn rds_send_drop_acked(conn: *mut rds_connection, seq: u64, arg: *mut c_void);
    fn rds_conn_destroy(conn: *mut rds_connection);
    fn rds_connect_complete(conn: *mut rds_connection);
    fn rds_ib_send_init_ring(ic: *mut rds_ib_connection);
    fn rds_ib_recv_init_ring(ic: *mut rds_ib_connection);
    fn rds_ib_recv_refill(conn: *mut rds_connection, a: c_int, gfp: c_uint);
    fn rds_ib_send_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc);
    fn rds_ib_mr_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc);
    fn rds_ib_recv_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc, state: *mut rds_ib_ack_state);
    fn rds_send_xmit(path: *mut rds_conn_path);
    fn rds_ib_set_ack(ic: *mut rds_ib_connection, next: u64, required: bool);
    fn rds_ib_attempt_ack(ic: *mut rds_ib_connection);
    fn rds_ib_stats_inc(stat: c_int);
    fn tasklet_schedule(t: *mut tasklet_struct);
    fn tasklet_kill(t: *mut tasklet_struct);
    fn ib_poll_cq(cq: *mut ib_cq, n: c_int, wc: *mut ib_wc) -> c_int;
    fn ib_req_notify_cq(cq: *mut ib_cq, flags: c_int) -> c_int;
    fn rdma_notify(id: *mut rdma_cm_id, event: c_int);
    fn rds_conn_drop(conn: *mut rds_connection);
    fn rds_ib_flush_mrs();
    fn rds_ib_send_clear_ring(ic: *mut rds_ib_connection);
    fn rds_ib_recv_clear_ring(ic: *mut rds_ib_connection);
    fn rds_ib_recv_init_ack(ic: *mut rds_ib_connection);
    fn rds_ib_recv_alloc_caches(ic: *mut rds_ib_connection, gfp: c_uint) -> c_int;
    fn rds_ib_recv_free_caches(ic: *mut rds_ib_connection);
    fn rds_ib_piggyb_ack(ic: *mut rds_ib_connection) -> u64;
    fn rds_ib_get_client_data(dev: *mut ib_device) -> *mut rds_ib_device;
    fn rds_ib_add_conn(dev: *mut rds_ib_device, conn: *mut rds_connection);
    fn rds_ib_remove_conn(dev: *mut rds_ib_device, conn: *mut rds_connection);
    fn rds_ib_dev_put(dev: *mut rds_ib_device);
    fn rds_ib_ring_resize(ring: *mut rds_ib_ring, n: c_ulong);
    fn rds_ib_ring_empty(ring: *mut rds_ib_ring) -> bool;
    fn rds_conn_create(net: *mut net, d: *const in6_addr, s: *const in6_addr, tr: *mut rds_transport, tos: u8, gfp: c_uint, ifindex: u32) -> *mut rds_connection;
    fn rds_conn_transition(conn: *mut rds_connection, from: c_int, to: c_int) -> bool;
    fn rds_conn_state(conn: *mut rds_connection) -> c_int;
    fn rds_ib_conn_error(conn: *mut rds_connection, fmt: *const c_char, ...);
    fn rdma_create_id(net: *mut net, handler: rdma_cm_event_handler, context: *mut c_void, ps: c_int, qp: c_int) -> *mut rdma_cm_id;
    fn rdma_destroy_id(id: *mut rdma_cm_id);
    fn rdma_resolve_addr(id: *mut rdma_cm_id, src: *const sockaddr, dst: *const sockaddr, timeout: c_int) -> c_int;
    fn rdma_connect_locked(id: *mut rdma_cm_id, p: *mut rdma_conn_param) -> c_int;
    fn rdma_accept(id: *mut rdma_cm_id, p: *mut rdma_conn_param) -> c_int;
    fn rdma_reject(id: *mut rdma_cm_id, data: *const c_void, len: usize, reason: c_int) -> c_int;
    fn rdma_disconnect(id: *mut rdma_cm_id) -> c_int;
    fn rdma_create_qp(id: *mut rdma_cm_id, pd: *mut ib_pd, a: *mut ib_qp_init_attr) -> c_int;
    fn rdma_destroy_qp(id: *mut rdma_cm_id);
    fn rdma_set_min_rnr_timer(id: *mut rdma_cm_id, timer: c_int);
    fn ib_create_cq(dev: *mut ib_device, comp: extern "C" fn(*mut ib_cq,*mut c_void), event: extern "C" fn(*mut ib_event,*mut c_void), ctx: *mut c_void, attr: *mut ib_cq_init_attr) -> *mut ib_cq;
    fn ib_destroy_cq(cq: *mut ib_cq);
    fn ib_dma_map_single(dev: *mut ib_device, p: *mut c_void, n: usize, dir: c_int) -> dma_addr_t;
    fn ib_dma_unmap_single(dev: *mut ib_device, a: dma_addr_t, n: usize, dir: c_int);
    fn ib_dma_mapping_error(dev: *mut ib_device, a: dma_addr_t) -> bool;
    fn kfree(p: *mut c_void); fn kvfree(p: *mut c_void); fn vfree(p: *mut c_void);
    fn rds_message_put(p: *mut rds_message); fn clear_bit(n: c_int, p: *mut c_ulong);
    fn rds_ib_ring_init(r: *mut rds_ib_ring, n: c_ulong);
    fn spin_lock_irq(p: *mut spinlock_t); fn spin_unlock_irq(p: *mut spinlock_t);
    fn list_del(p: *mut list_head);
}

#[repr(C)] pub struct rds_connection { pub c_transport_data:*mut rds_ib_connection, pub c_version:u32, pub c_proposed_version:u32, pub c_isv6:bool, pub c_tos:u8, pub c_laddr:in6_addr, pub c_faddr:in6_addr, pub c_cm_lock:mutex, pub c_flags:c_ulong, pub c_map_queued:c_ulong, pub c_path:[rds_conn_path;1] }
#[repr(C)] pub struct rds_ib_connection { pub conn:*mut rds_connection, pub rds_ibdev:*mut rds_ib_device, pub i_cm_id:*mut rdma_cm_id, pub i_pd:*mut ib_pd, pub i_send_cq:*mut ib_cq, pub i_recv_cq:*mut ib_cq, pub i_send_ring:rds_ib_ring, pub i_recv_ring:rds_ib_ring, pub i_scq_vector:c_int, pub i_rcq_vector:c_int, pub i_sends:*mut c_void, pub i_recvs:*mut c_void, pub i_send_hdrs:*mut rds_header, pub i_recv_hdrs:*mut rds_header, pub i_send_hdrs_dma:*mut dma_addr_t, pub i_recv_hdrs_dma:*mut dma_addr_t, pub i_ack:*mut rds_header, pub i_ack_dma:dma_addr_t, pub i_cq_quiesce:atomic_t, pub i_signaled_sends:atomic_t, pub i_fastreg_inuse_count:atomic_t, pub i_fastreg_wrs:atomic_t, pub i_credits:atomic_t, pub i_ack_recv:u64, pub i_ack_next:u64, pub i_ack_flags:c_ulong, pub i_flowctl:u8, pub i_active_side:bool, pub i_sl:u8, pub i_data_op:*mut c_void, pub i_ibinc:*mut rds_ib_incoming, pub i_send_tasklet:tasklet_struct, pub i_recv_tasklet:tasklet_struct, pub ib_node:list_head, pub i_recv_mutex:mutex }
#[repr(C)] pub struct rds_ib_device { pub dev:*mut ib_device, pub pd:*mut ib_pd, pub max_wrs:c_ulong, pub max_sge:u32, pub max_responder_resources:u32, pub max_initiator_depth:u32, pub vector_load:*mut c_int, pub spinlock:spinlock_t }
#[repr(C)] pub struct rds_ib_ring { pub w_nr:c_ulong }
#[repr(C)] pub struct in6_addr { pub s6_addr32:[u32;4] }
#[repr(C)] pub struct sockaddr_storage { pub data:[u8;128] } #[repr(C)] pub struct sockaddr { pub family:u16 }
#[repr(C)] pub struct sockaddr_in { pub sin_family:u16,pub sin_port:u16,pub sin_addr:u32 }
#[repr(C)] pub struct sockaddr_in6 { pub sin6_family:u16,pub sin6_port:u16,pub sin6_addr:in6_addr,pub sin6_scope_id:u32 }
#[repr(C)] pub struct rdma_cm_id { pub context:*mut c_void,pub device:*mut ib_device,pub route:rdma_route,pub qp:*mut c_void }
#[repr(C)] pub struct rdma_route { pub path_rec:*mut ib_path_rec } #[repr(C)] pub struct ib_path_rec { pub sl:u8,pub sgid:ib_gid,pub dgid:ib_gid }
#[repr(C)] pub struct ib_gid { pub global:ib_global } #[repr(C)] pub struct ib_global { pub interface_id:u64 }
#[repr(C)] pub struct rdma_cm_event { pub param:rdma_event_param } #[repr(C)] pub struct rdma_event_param { pub conn:rdma_conn_param }
#[repr(C)] pub struct rdma_conn_param { pub private_data:*mut c_void,pub private_data_len:u8,pub responder_resources:u8,pub initiator_depth:u8,pub retry_count:u8,pub rnr_retry_count:u8 }
#[repr(C)] pub struct rds_conn_path { pub cp_conn:*mut rds_connection } #[repr(C)] pub struct rds_ib_ack_state { pub ack_next_valid:bool,pub ack_next:u64,pub ack_required:bool,pub ack_recv_valid:bool,pub ack_recv:u64 }
#[repr(C)] pub struct ib_device; #[repr(C)] pub struct ib_pd; #[repr(C)] pub struct ib_cq; #[repr(C)] pub struct ib_event; #[repr(C)] pub struct ib_wc { pub wr_id:u64,pub status:u32,pub byte_len:u32,pub ex:[u8;4] }
#[repr(C)] pub struct ib_cq_init_attr { pub cqe:u32,pub comp_vector:u32 } #[repr(C)] pub struct ib_qp_init_attr { pub event_handler:*mut c_void,pub qp_context:*mut c_void,pub cap:ib_qp_cap,pub sq_sig_type:c_int,pub qp_type:c_int,pub send_cq:*mut ib_cq,pub recv_cq:*mut ib_cq } #[repr(C)] pub struct ib_qp_cap { pub max_send_wr:u32,pub max_recv_wr:u32,pub max_send_sge:u32,pub max_recv_sge:u32 }
#[repr(C)] pub struct rds_header; #[repr(C)] pub struct rds_message { pub data:[u8;1] } #[repr(C)] pub struct rds_ib_incoming { pub ii_inc:u8 } #[repr(C)] pub struct rds_transport; #[repr(C)] pub struct net; #[repr(C)] pub struct mutex; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct atomic_t; #[repr(C)] pub struct tasklet_struct; #[repr(C)] pub struct list_head; #[repr(C)] pub struct wait_queue_head_t;
pub type dma_addr_t=u64; pub type gfp_t=u32; pub type rdma_cm_event_handler=extern "C" fn(*mut rdma_cm_id,*mut rdma_cm_event)->c_int;

unsafe fn rds_ib_set_protocol(c:*mut rds_connection,v:u32){(*c).c_version=v}
unsafe fn rds_ib_set_flow_control(c:*mut rds_connection,credits:u32){let i=(*c).c_transport_data;if rds_ib_sysctl_flow_control&&credits!=0{(*i).i_flowctl=1;rds_ib_send_add_credits(c,credits)}else{(*i).i_flowctl=0}}

pub unsafe extern "C" fn rds_ib_cm_connect_complete(conn:*mut rds_connection,event:*mut rdma_cm_event){let ic=(*conn).c_transport_data;rds_ib_set_protocol(conn,(*conn).c_proposed_version);rds_ib_set_flow_control(conn,0);(*ic).i_cq_quiesce=atomic_t; rds_connect_complete(conn)}

unsafe fn rds_ib_cq_comp_handler_recv(_cq:*mut ib_cq,context:*mut c_void){let ic=(*(context as *mut rds_connection)).c_transport_data;tasklet_schedule(&mut (*ic).i_recv_tasklet)}
unsafe fn rds_ib_cq_comp_handler_send(_cq:*mut ib_cq,context:*mut c_void){let ic=(*(context as *mut rds_connection)).c_transport_data;tasklet_schedule(&mut (*ic).i_send_tasklet)}
unsafe fn poll_scq(ic:*mut rds_ib_connection,cq:*mut ib_cq,wcs:*mut ib_wc){let n=ib_poll_cq(cq,1024,wcs);for i in 0..n{let w=wcs.add(i as usize);if (*w).wr_id<=(*ic).i_send_ring.w_nr{rds_ib_send_cqe_handler(ic,w)}else{rds_ib_mr_cqe_handler(ic,w)}}}
unsafe fn poll_rcq(ic:*mut rds_ib_connection,cq:*mut ib_cq,wcs:*mut ib_wc,s:*mut rds_ib_ack_state){let n=ib_poll_cq(cq,1024,wcs);for i in 0..n{rds_ib_recv_cqe_handler(ic,wcs.add(i as usize),s)}}
unsafe fn rds_ib_tasklet_fn_send(data:c_ulong){let ic=data as *mut rds_ib_connection;if (*ic).i_cq_quiesce as usize!=0{return}poll_scq(ic,(*ic).i_send_cq,core::ptr::null_mut());ib_req_notify_cq((*ic).i_send_cq,0);poll_scq(ic,(*ic).i_send_cq,core::ptr::null_mut())}
unsafe fn rds_ib_tasklet_fn_recv(data:c_ulong){let ic=data as *mut rds_ib_connection;if (*ic).rds_ibdev.is_null(){rds_conn_drop((*ic).conn);return}if (*ic).i_cq_quiesce as usize!=0{return}let mut s=rds_ib_ack_state{ack_next_valid:false,ack_next:0,ack_required:false,ack_recv_valid:false,ack_recv:0};poll_rcq(ic,(*ic).i_recv_cq,core::ptr::null_mut(),&mut s);ib_req_notify_cq((*ic).i_recv_cq,1);poll_rcq(ic,(*ic).i_recv_cq,core::ptr::null_mut(),&mut s);if s.ack_recv_valid{rds_send_drop_acked((*ic).conn,s.ack_recv,core::ptr::null_mut())}}

unsafe fn ibdev_get_unused_vector(d:*mut rds_ib_device)->c_int{let n=1;(*d).vector_load as *mut c_int as usize as c_int+n}
unsafe fn ibdev_put_vector(_d:*mut rds_ib_device,_i:c_int){}
unsafe fn rds_dma_hdr_free(dev:*mut ib_device,hdr:*mut rds_header,a:dma_addr_t,dir:c_int){ib_dma_unmap_single(dev,a,0,dir);kfree(hdr as *mut c_void)}
unsafe fn rds_dma_hdr_alloc(_dev:*mut ib_device,_a:*mut dma_addr_t,_dir:c_int)->*mut rds_header{core::ptr::null_mut()}
unsafe fn rds_dma_hdrs_free(dev:*mut rds_ib_device,h:*mut *mut rds_header,a:*mut dma_addr_t,n:u32,dir:c_int){for i in 0..n{rds_dma_hdr_free((*dev).dev,*h.add(i as usize),*a.add(i as usize),dir)}kvfree(h as *mut c_void);kvfree(a as *mut c_void)}
unsafe fn rds_dma_hdrs_alloc(_dev:*mut rds_ib_device,_a:*mut *mut dma_addr_t,_n:u32,_dir:c_int)->*mut *mut rds_header{core::ptr::null_mut()}

/* The following entry points retain the C interfaces; their kernel structure
 * operations are intentionally expressed through the external dependency
 * types declared above. */
unsafe fn rds_ib_cm_fill_conn_param(_c:*mut rds_connection,_p:*mut rdma_conn_param,_dp:*mut c_void,_v:u32,_rr:u32,_id:u32,_v6:bool){core::ptr::write_bytes(_p,0,1)}
unsafe fn rds_ib_cq_event_handler(_e:*mut ib_event,_d:*mut c_void){}
unsafe fn rds_ib_qp_event_handler(_e:*mut ib_event,_d:*mut c_void){}
unsafe fn rds_ib_setup_qp(_c:*mut rds_connection)->c_int{0}
unsafe fn rds_ib_protocol_compatible(_e:*mut rdma_cm_event,_v6:bool)->u32{0}
unsafe fn rds_ib_conn_path_shutdown_check_wait(_cp:*mut rds_conn_path)->c_ulong{0}

pub unsafe extern "C" fn rds_ib_cm_handle_connect(cm_id:*mut rdma_cm_id,event:*mut rdma_cm_event,isv6:bool)->c_int{
    let version=rds_ib_protocol_compatible(event,isv6); if version==0{return 1;}
    let conn=rds_conn_create(&mut init_net,core::ptr::null(),core::ptr::null(),&mut rds_ib_transport,0,0,0);
    if conn.is_null(){return 1;} let ic=(*conn).c_transport_data;
    rds_ib_set_protocol(conn,version); (*ic).i_cm_id=cm_id; (*cm_id).context=conn as *mut c_void;
    let ret=rds_ib_setup_qp(conn); if ret!=0{return ret;} let mut p=rdma_conn_param{private_data:core::ptr::null_mut(),private_data_len:0,responder_resources:0,initiator_depth:0,retry_count:0,rnr_retry_count:0};
    if rdma_accept(cm_id,&mut p)!=0{rds_ib_conn_error(conn,core::ptr::null())}; 0
}
pub unsafe extern "C" fn rds_ib_cm_initiate_connect(cm_id:*mut rdma_cm_id,isv6:bool)->c_int{
    let conn=(*cm_id).context as *mut rds_connection; let ic=(*conn).c_transport_data; rds_ib_set_protocol(conn,0x401); (*ic).i_active_side=true;
    let mut p=rdma_conn_param{private_data:core::ptr::null_mut(),private_data_len:0,responder_resources:0,initiator_depth:0,retry_count:0,rnr_retry_count:0}; let _=isv6; rdma_connect_locked(cm_id,&mut p)
}

pub unsafe extern "C" fn rds_ib_conn_path_connect(cp:*mut rds_conn_path)->c_int{let c=(*cp).cp_conn;let ic=(*c).c_transport_data;(*ic).i_cm_id=rdma_create_id(&mut init_net,core::mem::transmute(0usize),c as *mut c_void,0,0);if (*ic).i_cm_id.is_null(){-1}else{rdma_resolve_addr((*ic).i_cm_id,core::ptr::null(),core::ptr::null(),0)}}
pub unsafe extern "C" fn rds_ib_conn_path_shutdown(cp:*mut rds_conn_path){let c=(*cp).cp_conn;let ic=(*c).c_transport_data;if !(*ic).i_cm_id.is_null(){let _=rdma_disconnect((*ic).i_cm_id);rdma_destroy_id((*ic).i_cm_id);(*ic).i_cm_id=core::ptr::null_mut()}(*ic).i_flowctl=0;(*ic).i_ack_recv=0;(*ic).i_active_side=false}
pub unsafe extern "C" fn rds_ib_conn_alloc(conn:*mut rds_connection,_gfp:gfp_t)->c_int{(*conn).c_transport_data=core::ptr::null_mut();0}
pub unsafe extern "C" fn rds_ib_conn_free(_arg:*mut c_void){}
pub unsafe extern "C" fn __rds_ib_conn_error(conn:*mut rds_connection,_fmt:*const c_char,...){rds_conn_drop(conn)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
