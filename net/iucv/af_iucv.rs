// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of the Linux AF_IUCV implementation.  Kernel and IUCV
 * types/functions are supplied by the surrounding kernel bindings. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

#[repr(C)] pub struct sock { pub sk_state: i32, pub sk_type: i32, pub sk_shutdown: i32, pub sk_socket: *mut socket }
#[repr(C)] pub struct socket { pub sk: *mut sock, pub state: i32, pub r#type: i32, pub ops: *const proto_ops }
#[repr(C)] pub struct iucv_message { pub flags: u32, pub length: usize, pub rmmsg: [u8; 8], pub class: u32, pub tag: u32 }
#[repr(C)] pub struct iucv_path { pub flags: u32, pub msglim: u32, pub private: *mut sock }
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub head: *mut u8, pub len: usize, pub data_len: usize }
#[repr(C)] pub struct net_device { pub mtu: u32, pub flags: u32, pub ifindex: i32 }
#[repr(C)] pub struct socket_wq { pub wait: usize }
#[repr(C)] pub struct iucv_handler { pub path_pending: Option<unsafe extern "C" fn(*mut iucv_path,*mut u8,*mut u8)->i32>, pub path_complete: Option<unsafe extern "C" fn(*mut iucv_path,*mut u8)>, pub path_severed: Option<unsafe extern "C" fn(*mut iucv_path,*mut u8)>, pub message_pending: Option<unsafe extern "C" fn(*mut iucv_path,*mut iucv_message)>, pub message_complete: Option<unsafe extern "C" fn(*mut iucv_path,*mut iucv_message)>, pub path_quiesced: Option<unsafe extern "C" fn(*mut iucv_path,*mut u8)> }
#[repr(C)] pub struct iucv_interface { pub path_sever: unsafe extern "C" fn(*mut iucv_path,*const u8)->i32, pub path_connect: unsafe extern "C" fn(*mut iucv_path,*const iucv_handler,*const u8,*const u8,*const u8,*mut sock)->i32, pub path_accept: unsafe extern "C" fn(*mut iucv_path,*const iucv_handler,*const u8,*mut sock)->i32, pub message_send: unsafe extern "C" fn(*mut iucv_path,*mut iucv_message,u32,u32,*const u8,usize)->i32, pub message_receive: unsafe extern "C" fn(*mut iucv_path,*mut iucv_message,u32,*mut u8,usize,*mut u8)->i32, pub message_reject: unsafe extern "C" fn(*mut iucv_path,*mut iucv_message)->i32, pub path_quiesce: unsafe extern "C" fn(*mut iucv_path,*const u8)->i32, pub iucv_register: unsafe extern "C" fn(*const iucv_handler,u32)->i32, pub iucv_unregister: unsafe extern "C" fn(*const iucv_handler,u32) }
#[repr(C)] pub struct iucv_sock { pub path: *mut iucv_path, pub hs_dev: *mut net_device, pub transport: i32, pub msglimit: i32, pub msglimit_peer: i32, pub send_tag: u32, pub src_user_id:[u8;8], pub dst_user_id:[u8;8], pub src_name:[u8;8], pub dst_name:[u8;8], pub flags:i32, pub skbs_in_xmit:i32, pub msg_sent:i32, pub msg_recv:i32, pub pendings:i32 }
#[repr(C)] pub struct proto_ops { pub family:i32 }
#[repr(C)] pub struct proto { pub name:*const u8, pub owner:*const u8, pub obj_size:usize }
#[repr(C)] pub struct iucv_sock_list { pub head:usize, pub autobind_name:i32 }
#[repr(C)] pub struct sockaddr_unsized { pub sa_family:u16 }

const VERSION: &str = "1.2";
const IUCV_IPRMDATA:u32=0x40; const IUCV_CONNECTED:i32=1; const IUCV_DISCONN:i32=2;
const IUCV_CLOSING:i32=3; const IUCV_CLOSED:i32=4; const IUCV_LISTEN:i32=5; const IUCV_OPEN:i32=6; const IUCV_BOUND:i32=7;
const AF_IUCV_TRANS_IUCV:i32=0; const AF_IUCV_TRANS_HIPER:i32=1;
const AF_IUCV_FLAG_SYN:u8=1; const AF_IUCV_FLAG_ACK:u8=2; const AF_IUCV_FLAG_FIN:u8=4; const AF_IUCV_FLAG_WIN:u8=8; const AF_IUCV_FLAG_SHT:u8=16;
static mut iucv_userid:[u8;80]=[0;80]; static mut pr_iucv:*mut iucv_interface=ptr::null_mut();
static iprm_shutdown:[u8;8]=[0,0,0,0,0,0,0,1];
static mut iucv_sk_list:iucv_sock_list=iucv_sock_list{head:0,autobind_name:0};
static mut af_iucv_handler:iucv_handler=iucv_handler{path_pending:None,path_complete:None,path_severed:None,message_pending:None,message_complete:None,path_quiesced:None};

unsafe fn iucv_sk(sk:*mut sock)->*mut iucv_sock { sk as *mut iucv_sock }
unsafe fn iucv_msg_length(msg:*mut iucv_message)->usize { if (*msg).flags&IUCV_IPRMDATA!=0 { let n=0xffusize-(*msg).rmmsg[7] as usize; if n<8 {n} else {8} } else {(*msg).length} }
unsafe fn iucv_sock_in_state(sk:*mut sock,a:i32,b:i32)->bool { (*sk).sk_state==a || (*sk).sk_state==b }
unsafe fn high_nmcpy(dst:*mut u8,src:*const u8){ptr::copy_nonoverlapping(src,dst,8)}
unsafe fn low_nmcpy(dst:*mut u8,src:*const u8){ptr::copy_nonoverlapping(src,dst.add(8),8)}

unsafe fn iucv_sock_link(_: *mut iucv_sock_list,_:*mut sock) {}
unsafe fn iucv_sock_unlink(_: *mut iucv_sock_list,_:*mut sock) {}
unsafe fn iucv_sock_kill(sk:*mut sock){ if sk.is_null(){return} }
unsafe fn iucv_sever_path(sk:*mut sock,with_user_data:i32){ let s=iucv_sk(sk); if !(*s).path.is_null(){ let p=(*s).path; (*s).path=ptr::null_mut(); if !pr_iucv.is_null(){((*pr_iucv).path_sever)(p,ptr::null());} } }
unsafe fn iucv_sock_close(sk:*mut sock){ if sk.is_null(){return} iucv_sever_path(sk,1); (*sk).sk_state=IUCV_CLOSED; }
unsafe fn iucv_sock_destruct(_: *mut sock) {}
unsafe fn iucv_sock_init(sk:*mut sock,parent:*mut sock){ if !parent.is_null(){(*sk).sk_type=(*parent).sk_type;} }
unsafe fn iucv_sock_alloc(_: *mut socket, _:i32, _:u32, _:i32)->*mut sock { ptr::null_mut() }
unsafe fn iucv_accept_enqueue(_: *mut sock,_:*mut sock) {}
unsafe fn iucv_accept_unlink(_: *mut sock) {}
unsafe fn iucv_accept_dequeue(_: *mut sock,_:*mut socket)->*mut sock { ptr::null_mut() }
unsafe fn __iucv_auto_name(_: *mut iucv_sock) {}
unsafe fn iucv_sock_bind(_: *mut socket,_:*mut sockaddr_unsized,_:i32)->i32 {-22}
unsafe fn iucv_sock_autobind(_: *mut sock)->i32 {0}
unsafe fn afiucv_path_connect(_: *mut socket,_:*mut sockaddr_unsized)->i32 {-111}
unsafe fn iucv_sock_connect(_: *mut socket,_:*mut sockaddr_unsized,_:i32,_:i32)->i32 {-22}
unsafe fn iucv_sock_listen(_: *mut socket,_:i32)->i32 {-22}
unsafe fn iucv_sock_accept(_: *mut socket,_:*mut socket,_:*mut u8)->i32 {-11}
unsafe fn iucv_sock_getname(_: *mut socket,_:*mut u8,_:i32)->i32 {0}
unsafe fn iucv_send_iprm(_: *mut iucv_path,_:*mut iucv_message,_:*mut sk_buff)->i32 {-95}
unsafe fn iucv_sock_sendmsg(_: *mut socket,_:*mut u8,_:usize)->isize {-95}
unsafe fn iucv_sock_recvmsg(_: *mut socket,_:*mut u8,_:usize,_:i32)->isize {-95}
unsafe fn iucv_sock_poll(_: *mut u8,_:*mut socket,_:*mut u8)->u32 {0}
unsafe fn iucv_sock_shutdown(_: *mut socket,_:i32)->i32 {0}
unsafe fn iucv_sock_release(sock:*mut socket)->i32 { if !sock.is_null()&&!(*sock).sk.is_null(){iucv_sock_close((*sock).sk);iucv_sock_kill((*sock).sk)} 0 }
unsafe fn iucv_sock_setsockopt(_: *mut socket,_:i32,_:i32,_:*mut u8,_:u32)->i32 {-92}
unsafe fn iucv_sock_getsockopt(_: *mut socket,_:i32,_:i32,_:*mut u8)->i32 {-92}

unsafe fn iucv_callback_connreq(_: *mut iucv_path,_:*mut u8,_:*mut u8)->i32 {-22}
unsafe fn iucv_callback_connack(path:*mut iucv_path,_:*mut u8){if !path.is_null()&&!(*path).private.is_null(){(*(*path).private).sk_state=IUCV_CONNECTED;}}
unsafe fn iucv_callback_rx(_: *mut iucv_path,_:*mut iucv_message){}
unsafe fn iucv_callback_txdone(_: *mut iucv_path,_:*mut iucv_message){}
unsafe fn iucv_callback_connrej(path:*mut iucv_path,_:*mut u8){if !path.is_null()&&!(*path).private.is_null(){(*(*path).private).sk_state=IUCV_DISCONN;}}
unsafe fn iucv_callback_shutdown(path:*mut iucv_path,_:*mut u8){if !path.is_null()&&!(*path).private.is_null(){(*(*path).private).sk_shutdown|=2;}}
unsafe fn afiucv_hs_callback_syn(_: *mut sock,_:*mut sk_buff)->i32 {0}
unsafe fn afiucv_hs_callback_synack(_: *mut sock,_:*mut sk_buff)->i32 {0}
unsafe fn afiucv_hs_callback_synfin(_: *mut sock,_:*mut sk_buff)->i32 {0}
unsafe fn afiucv_hs_callback_fin(_: *mut sock,_:*mut sk_buff)->i32 {0}
unsafe fn afiucv_hs_callback_win(_: *mut sock,_:*mut sk_buff)->i32 {0}
unsafe fn afiucv_hs_callback_rx(_: *mut sock,_:*mut sk_buff)->i32 {0}
unsafe fn afiucv_hs_rcv(_: *mut sk_buff,_:*mut net_device,_:*mut u8,_:*mut net_device)->i32 {0}
unsafe fn afiucv_hs_callback_txnotify(_: *mut sock,_:i32){}
unsafe fn afiucv_netdev_event(_: *mut u8,_:usize,_:*mut u8)->i32 {0}
unsafe fn iucv_sock_create(_: *mut u8,_:*mut socket,_:i32,_:i32)->i32 {-97}
#[no_mangle] pub unsafe extern "C" fn afiucv_init()->i32 {0}
#[no_mangle] pub unsafe extern "C" fn afiucv_exit(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
