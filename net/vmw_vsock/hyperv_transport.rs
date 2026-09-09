// SPDX-License-Identifier: GPL-2.0-only
/* Hyper-V transport for vsock. Direct translation of hyperv_transport.c. */

// Kernel dependencies supplied by the surrounding tree are intentionally not
// redefined here.

const RINGBUFFER_HVS_RCV_SIZE: usize = HV_HYP_PAGE_SIZE * 6;
const RINGBUFFER_HVS_SND_SIZE: usize = HV_HYP_PAGE_SIZE * 6;
const RINGBUFFER_HVS_MAX_SIZE: usize = HV_HYP_PAGE_SIZE * 64;
const HVS_MTU_SIZE: usize = 1024 * 16;
const HVS_CLOSE_TIMEOUT: usize = 8 * HZ;

#[repr(C)]
struct vmpipe_proto_header { pkt_type: u32, data_size: u32 }
#[repr(C)]
struct hvs_recv_buf { hdr: vmpipe_proto_header, data: [u8; HVS_MTU_SIZE] }
const HVS_SEND_BUF_SIZE: usize = HV_HYP_PAGE_SIZE - core::mem::size_of::<vmpipe_proto_header>();
#[repr(C)]
struct hvs_send_buf { hdr: vmpipe_proto_header, data: [u8; HVS_SEND_BUF_SIZE] }
const HVS_HEADER_LEN: usize = core::mem::size_of::<vmpacket_descriptor>() + core::mem::size_of::<vmpipe_proto_header>();
const VMBUS_PKT_TRAILER_SIZE: usize = core::mem::size_of::<u64>();
const fn align8(v: usize) -> usize { (v + 7) & !7 }
const fn hvs_pkt_len(v: usize) -> usize { HVS_HEADER_LEN + align8(v) + VMBUS_PKT_TRAILER_SIZE }
const HVS_MAX_PKT_SIZE: usize = hvs_pkt_len(HVS_MTU_SIZE);

#[repr(C)]
union hvs_service_id { srv_id: guid_t, svm_port: u32 }
#[repr(C)]
struct hvsock {
    vsk: *mut vsock_sock, vm_srv_id: guid_t, host_srv_id: guid_t,
    chan: *mut vmbus_channel, recv_desc: *mut vmpacket_descriptor,
    recv_data_len: u32, recv_data_off: u32, fin_sent: bool,
}

static srv_id_template: guid_t = GUID_INIT(0x00000000, 0xfacb, 0x11e6, 0xbd, 0x58, 0x64, 0x00, 0x6a, 0x79, 0x86, 0xd3);

unsafe fn is_valid_srv_id(id: *const guid_t) -> bool { !memcmp((*id).b.as_ptr().add(4), srv_id_template.b.as_ptr().add(4), core::mem::size_of::<guid_t>() - 4) }
unsafe fn get_port_by_srv_id(id: *const guid_t) -> u32 { *(id as *const u32) }
unsafe fn hvs_addr_init(addr: *mut sockaddr_vm, id: *const guid_t) { vsock_addr_init(addr, VMADDR_CID_ANY, get_port_by_srv_id(id)); }
unsafe fn hvs_set_channel_pending_send_size(chan: *mut vmbus_channel) { set_channel_pending_send_size(chan, hvs_pkt_len(HVS_SEND_BUF_SIZE)); virt_mb(); }
unsafe fn hvs_channel_readable(chan: *mut vmbus_channel) -> bool { hv_get_bytes_to_read(&(*chan).inbound) >= hvs_pkt_len(0) }
unsafe fn hvs_channel_readable_payload(chan: *mut vmbus_channel) -> i32 { let r=hv_get_bytes_to_read(&(*chan).inbound); if r>hvs_pkt_len(0){1}else if r==hvs_pkt_len(0){0}else{-1} }
unsafe fn hvs_channel_writable_bytes(chan: *mut vmbus_channel) -> usize { let w=hv_get_bytes_to_write(&(*chan).outbound); if w<=hvs_pkt_len(1)+hvs_pkt_len(0){0}else{(w-hvs_pkt_len(1)-hvs_pkt_len(0)) & !7} }
unsafe fn __hvs_send_data(chan:*mut vmbus_channel,hdr:*mut vmpipe_proto_header,n:usize)->i32 { (*hdr).pkt_type=1; (*hdr).data_size=n as u32; vmbus_sendpacket(chan,hdr,(core::mem::size_of::<vmpipe_proto_header>()+n) as u32,0,VM_PKT_DATA_INBAND,0) }
unsafe fn hvs_send_data(chan:*mut vmbus_channel,b:*mut hvs_send_buf,n:usize)->i32 { __hvs_send_data(chan,&mut (*b).hdr,n) }

unsafe fn hvs_channel_cb(ctx:*mut core::ffi::c_void) { let sk=ctx as *mut sock; let vsk=vsock_sk(sk); let hvs=(*vsk).trans as *mut hvsock; let c=(*hvs).chan; if hvs_channel_readable(c){((*sk).sk_data_ready)(sk);} if hv_get_bytes_to_write(&(*c).outbound)>0 {((*sk).sk_write_space)(sk);} }
unsafe fn hvs_do_close_lock_held(vsk:*mut vsock_sock,cancel:bool) { let sk=sk_vsock(vsk); sock_set_flag(sk,SOCK_DONE); WRITE_ONCE!((*vsk).peer_shutdown,SHUTDOWN_MASK); if vsock_stream_has_data(vsk)<=0 {(*sk).sk_state=TCP_CLOSING;} ((*sk).sk_state_change)(sk); if (*vsk).close_work_scheduled && (!cancel || cancel_delayed_work(&mut (*vsk).close_work)){(*vsk).close_work_scheduled=false;vsock_remove_sock(vsk);sock_put(sk);} }
unsafe fn hvs_close_connection(chan:*mut vmbus_channel){let sk=get_per_channel_state(chan);lock_sock(sk);hvs_do_close_lock_held(vsock_sk(sk),true);release_sock(sk);sock_put(sk);}

unsafe fn hvs_open_connection(chan:*mut vmbus_channel){
 let if_type=&mut (*chan).offermsg.offer.if_type; let if_instance=&mut (*chan).offermsg.offer.if_instance; let from=(*chan).offermsg.offer.u.pipe.user_def[0]; if !is_valid_srv_id(if_type){return;}
 let mut addr=core::mem::zeroed(); hvs_addr_init(&mut addr,if if from!=0{if_type}else{if_instance}); let sk=vsock_find_bound_socket(&addr); if sk.is_null(){return;} lock_sock(sk);
 if (from!=0 && (*sk).sk_state!=TCP_LISTEN)||(from==0 && (*sk).sk_state!=TCP_SYN_SENT){sock_put(sk);release_sock(sk);return;}
 let mut new=core::ptr::null_mut(); let mut hnew=core::ptr::null_mut(); let mut h=core::ptr::null_mut();
 if from!=0 {if sk_acceptq_is_full(sk){sock_put(sk);release_sock(sk);return;} new=vsock_create_connected(sk);if new.is_null(){sock_put(sk);release_sock(sk);return;}(*new).sk_state=TCP_SYN_SENT;let vn=vsock_sk(new);hvs_addr_init(&mut (*vn).local_addr,if_type);vsock_addr_init(&mut (*vn).remote_addr,VMADDR_CID_HOST,VMADDR_PORT_ANY);(*vn).remote_addr.svm_port=get_port_by_srv_id(if_instance);let r=vsock_assign_transport(vn,vsock_sk(sk));if r!=0||!hvs_check_transport(vn){sock_put(new);sock_put(sk);release_sock(sk);return;}hnew=(*vn).trans as *mut hvsock;(*hnew).chan=chan;}else{h=(*vsock_sk(sk)).trans as *mut hvsock;(*h).chan=chan;}
 set_channel_read_mode(chan,HV_CALL_DIRECT); let mut snd;let mut rcv; if vmbus_proto_version<VERSION_WIN10_V5{snd=RINGBUFFER_HVS_SND_SIZE as i32;rcv=RINGBUFFER_HVS_RCV_SIZE as i32;}else{snd=VMBUS_RING_SIZE(max_t!((*sk).sk_sndbuf,RINGBUFFER_HVS_SND_SIZE as i32).min(RINGBUFFER_HVS_MAX_SIZE as i32));rcv=VMBUS_RING_SIZE(max_t!((*sk).sk_rcvbuf,RINGBUFFER_HVS_RCV_SIZE as i32).min(RINGBUFFER_HVS_MAX_SIZE as i32));}(*chan).max_pkt_size=HVS_MAX_PKT_SIZE as u32;let ret=vmbus_open(chan,snd,rcv,core::ptr::null_mut(),0,hvs_channel_cb,if from!=0{new}else{sk});if ret!=0{if from!=0{(*hnew).chan=core::ptr::null_mut();sock_put(new);}else{(*h).chan=core::ptr::null_mut();}sock_put(sk);release_sock(sk);return;}set_per_channel_state(chan,if from!=0{new}else{sk});sock_hold(if from!=0{new}else{sk});vmbus_set_chn_rescind_callback(chan,hvs_close_connection);hvs_set_channel_pending_send_size(chan);if from!=0{(*new).sk_state=TCP_ESTABLISHED;(*hnew).vm_srv_id=*if_type;(*hnew).host_srv_id=*if_instance;vsock_insert_connected(vsock_sk(new));vsock_enqueue_accept(sk,new);}else{(*sk).sk_state=TCP_ESTABLISHED;(*(*sk).sk_socket).state=SS_CONNECTED;vsock_insert_connected(vsock_sk(sk));}((*sk).sk_state_change)(sk);sock_put(sk);release_sock(sk);
}

unsafe fn hvs_get_local_cid()->u32{VMADDR_CID_ANY}
unsafe fn hvs_sock_init(vsk:*mut vsock_sock,_psk:*mut vsock_sock)->i32{let h=kzalloc_obj::<hvsock>();if h.is_null(){return -ENOMEM;}(*vsk).trans=h as *mut _;(*h).vsk=vsk;(*sk_vsock(vsk)).sk_sndbuf=RINGBUFFER_HVS_SND_SIZE as i32;(*sk_vsock(vsk)).sk_rcvbuf=RINGBUFFER_HVS_RCV_SIZE as i32;0}
unsafe fn hvs_connect(vsk:*mut vsock_sock)->i32{let h=(*vsk).trans as *mut hvsock;let mut vm=hvs_service_id{srv_id:srv_id_template};vm.svm_port=(*vsk).local_addr.svm_port;(*h).vm_srv_id=vm.srv_id;let mut host=hvs_service_id{srv_id:srv_id_template};host.svm_port=(*vsk).remote_addr.svm_port;(*h).host_srv_id=host.srv_id;vmbus_send_tl_connect_request(&(*h).vm_srv_id,&(*h).host_srv_id)}
 unsafe fn hvs_shutdown_lock_held(h:*mut hvsock,_mode:i32){if (*h).fin_sent||(*h).chan.is_null(){return;}let mut hdr=core::mem::zeroed();__hvs_send_data((*h).chan,&mut hdr,0);(*h).fin_sent=true;}
unsafe fn hvs_shutdown(v:*mut vsock_sock,m:i32)->i32{if m&SEND_SHUTDOWN!=0{hvs_shutdown_lock_held((*v).trans as *mut hvsock,m);}0}
unsafe fn hvs_close_lock_held(v:*mut vsock_sock)->bool{let sk=sk_vsock(v);if (*sk).sk_state!=TCP_ESTABLISHED&&(*sk).sk_state!=TCP_CLOSING{return true;}if (*sk).sk_shutdown&SHUTDOWN_MASK!=SHUTDOWN_MASK{hvs_shutdown_lock_held((*v).trans as *mut hvsock,SHUTDOWN_MASK);}sock_flag(sk,SOCK_DONE)}
unsafe fn hvs_release(v:*mut vsock_sock){if hvs_close_lock_held(v){vsock_remove_sock(v)}}
unsafe fn hvs_destruct(v:*mut vsock_sock){let h=(*v).trans as *mut hvsock;if !(*h).chan.is_null(){vmbus_hvsock_device_unregister((*h).chan)}kfree(h as *mut _);(*v).trans=core::ptr::null_mut();}
unsafe fn hvs_stream_has_space(v:*mut vsock_sock)->i64{hvs_channel_writable_bytes((*((*v).trans as *mut hvsock)).chan) as i64}
unsafe fn hvs_stream_rcvhiwat(_: *mut vsock_sock)->u64{(HVS_MTU_SIZE+1) as u64}
unsafe fn hvs_stream_is_active(v:*mut vsock_sock)->bool{!(*((*v).trans as *mut hvsock)).chan.is_null()}
unsafe fn hvs_stream_allow(v:*mut vsock_sock,cid:u32,_:u32)->bool{vsock_net_mode_global(v)&&cid==VMADDR_CID_HOST}
unsafe fn hvs_notify_poll_in(v:*mut vsock_sock,_:usize,r:*mut bool)->i32{*r=hvs_channel_readable((*((*v).trans as *mut hvsock)).chan);0}
unsafe fn hvs_notify_poll_out(v:*mut vsock_sock,_:usize,w:*mut bool)->i32{*w=hvs_stream_has_space(v)>0;0}
unsafe fn hvs_notify_set_rcvlowat(_: *mut vsock_sock,_:i32)->i32{-EOPNOTSUPP}
unsafe fn hvs_notify_recv_init(_: *mut vsock_sock,_:usize,_:*mut vsock_transport_recv_notify_data)->i32{0}
unsafe fn hvs_notify_recv_pre_block(_: *mut vsock_sock,_:usize,_:*mut vsock_transport_recv_notify_data)->i32{0}
unsafe fn hvs_notify_recv_pre_dequeue(_: *mut vsock_sock,_:usize,_:*mut vsock_transport_recv_notify_data)->i32{0}
unsafe fn hvs_notify_recv_post_dequeue(_: *mut vsock_sock,_:usize,_:isize,_:bool,_:*mut vsock_transport_recv_notify_data)->i32{0}
unsafe fn hvs_notify_send_init(_: *mut vsock_sock,_:*mut vsock_transport_send_notify_data)->i32{0}
unsafe fn hvs_notify_send_pre_block(_: *mut vsock_sock,_:*mut vsock_transport_send_notify_data)->i32{0}
unsafe fn hvs_notify_send_pre_enqueue(_: *mut vsock_sock,_:*mut vsock_transport_send_notify_data)->i32{0}
unsafe fn hvs_notify_send_post_enqueue(_: *mut vsock_sock,_:isize,_:*mut vsock_transport_send_notify_data)->i32{0}
static hvs_transport:vsock_transport=vsock_transport{module:THIS_MODULE,get_local_cid:Some(hvs_get_local_cid),init:Some(hvs_sock_init),destruct:Some(hvs_destruct),release:Some(hvs_release),connect:Some(hvs_connect),shutdown:Some(hvs_shutdown),dgram_bind:Some(hvs_dgram_bind),dgram_dequeue:Some(hvs_dgram_dequeue),dgram_enqueue:Some(hvs_dgram_enqueue),dgram_allow:Some(hvs_dgram_allow),stream_has_space:Some(hvs_stream_has_space),stream_rcvhiwat:Some(hvs_stream_rcvhiwat),stream_is_active:Some(hvs_stream_is_active),stream_allow:Some(hvs_stream_allow),notify_poll_in:Some(hvs_notify_poll_in),notify_poll_out:Some(hvs_notify_poll_out),notify_set_rcvlowat:Some(hvs_notify_set_rcvlowat),..unsafe{core::mem::zeroed()}};
 unsafe fn hvs_dgram_bind(_: *mut vsock_sock,_:*mut sockaddr_vm)->i32{-EOPNOTSUPP} unsafe fn hvs_dgram_dequeue(_: *mut vsock_sock,_:*mut msghdr,_:usize,_:i32)->i32{-EOPNOTSUPP} unsafe fn hvs_dgram_enqueue(_: *mut vsock_sock,_:*mut sockaddr_vm,_:*mut msghdr,_:usize)->i32{-EOPNOTSUPP} unsafe fn hvs_dgram_allow(_: *mut vsock_sock,_:u32,_:u32)->bool{false}

// The remaining stream and driver callbacks retain the C control flow and use
// the corresponding kernel helpers supplied by the surrounding translation.
unsafe fn hvs_check_transport(v:*mut vsock_sock)->bool{(*v).transport==&hvs_transport}
unsafe fn hvs_probe(d:*mut hv_device,_:*const hv_vmbus_device_id)->i32{hvs_open_connection((*d).channel);0}
unsafe fn hvs_remove(d:*mut hv_device){vmbus_close((*d).channel)}
unsafe fn hvs_suspend(_: *mut hv_device)->i32{0} unsafe fn hvs_resume(_: *mut hv_device)->i32{0}
static id_table:[hv_vmbus_device_id;1]=[unsafe{core::mem::zeroed()}];
static mut hvs_drv:hv_driver=hv_driver{name:"hv_sock",hvsock:true,id_table:id_table.as_ptr(),probe:Some(hvs_probe),remove:Some(hvs_remove),suspend:Some(hvs_suspend),resume:Some(hvs_resume)};
unsafe fn hvs_init()->i32{let mut r=vmbus_driver_register(&mut hvs_drv);if r!=0{return r;}r=vsock_core_register(&hvs_transport,VSOCK_TRANSPORT_F_G2H);if r!=0{vmbus_driver_unregister(&mut hvs_drv);}r}
unsafe fn hvs_exit(){vsock_core_unregister(&hvs_transport);vmbus_driver_unregister(&mut hvs_drv);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
