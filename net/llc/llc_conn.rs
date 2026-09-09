// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of llc_conn.c. Kernel types and functions are
 * supplied by the surrounding translation unit. */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ptr;

extern "C" {
    static mut llc_conn_state_table: [llc_conn_state; NBR_CONN_STATES];
    fn llc_sk(sk: *mut sock) -> *mut llc_sock;
    fn llc_conn_ev(skb: *mut sk_buff) -> *mut llc_conn_state_ev;
    fn llc_pdu_sn_hdr(skb: *mut sk_buff) -> *mut llc_pdu_sn;
    fn llc_pdu_set_cmd_rsp(skb: *mut sk_buff, v: u8);
    fn llc_pdu_set_pf_bit(skb: *mut sk_buff, v: u8);
    fn llc_conn_remove_acked_pdus(sk: *mut sock, nr: u8, n: *mut u16) -> i32;
    fn llc_data_accept_state(s: u8) -> u8;
    fn llc_conn_ack_tmr_cb(_: *mut timer_list); fn llc_conn_pf_cycle_tmr_cb(_: *mut timer_list);
    fn llc_conn_rej_tmr_cb(_: *mut timer_list); fn llc_conn_busy_tmr_cb(_: *mut timer_list);
}

#[repr(C)] pub struct sock { pub sk_state: i32, pub sk_type: i32, pub sk_shutdown: i32, pub sk_socket: *mut socket, pub sk_state_change: Option<unsafe extern "C" fn(*mut sock)>, pub sk_write_queue: sk_buff_head, pub sk_receive_queue: sk_buff_head, pub sk_backlog_rcv: Option<unsafe extern "C" fn(*mut sock,*mut sk_buff)->i32> }
#[repr(C)] pub struct socket { pub state: i32 }
#[repr(C)] pub struct sk_buff { pub sk: *mut sock, pub dev: *mut net_device, pub destructor: Option<unsafe extern "C" fn(*mut sk_buff)> }
#[repr(C)] pub struct sk_buff_head { _p: [u8; 0] }
#[repr(C)] pub struct llc_sock { pub state:u8, pub sap:*mut llc_sap, pub dev:*mut net_device, pub laddr:llc_addr, pub daddr:llc_addr, pub pdu_unack_q:sk_buff_head, pub vS:u8, pub failed_data_req:i32, pub inc_cntr:u8,pub dec_cntr:u8,pub dec_step:u8,pub connect_step:u8,pub ack_timer:llc_timer,pub pf_cycle_timer:llc_timer,pub rej_sent_timer:llc_timer,pub busy_state_timer:llc_timer,pub n2:u8,pub k:u8,pub rw:u8,pub ack_must_be_send:u8,pub ack_pf:u8,pub remote_busy_flag:u8,pub cause_flag:u8,pub retry_count:u8,pub f_flag:u8,pub s_flag:u8,pub first_pdu_Ns:u8,pub X:u8,pub last_nr:u8 }
#[repr(C)] pub struct llc_timer { pub timer:timer_list,pub expire:i32 }
#[repr(C)] pub struct timer_list { _p:[u8;0] }
#[repr(C)] pub struct llc_sap { pub sk_lock: spinlock_t,pub sk_count:i32,pub sk_laddr_hash:*mut hlist_nulls_head }
#[repr(C)] pub struct net_device { pub ifindex:i32,pub flags:u32 }
#[repr(C)] pub struct llc_addr { pub mac:[u8;6],pub lsap:u8 }
#[repr(C)] pub struct net { _p:[u8;0] }
#[repr(C)] pub struct proto { _p:[u8;0] }
#[repr(C)] pub struct spinlock_t { _p:[u8;0] }
#[repr(C)] pub struct hlist_nulls_head { _p:[u8;0] }
#[repr(C)] pub struct hlist_head { _p:[u8;0] }
#[repr(C)] pub struct llc_pdu_sn { _p:[u8;0] }
#[repr(C)] pub struct llc_conn_state_ev { pub ind_prim:i32,pub cfm_prim:i32,pub type_:i32,pub reason:i32,pub status:i32 }
pub type llc_conn_state_ev_alias=llc_conn_state_ev;
#[repr(C)] pub struct llc_conn_state_trans { pub ev:Option<unsafe extern "C" fn(*mut sock,*mut sk_buff)->i32>,pub ev_qualifiers:*const Option<unsafe extern "C" fn(*mut sock,*mut sk_buff)->i32>,pub ev_actions:*const Option<unsafe extern "C" fn(*mut sock,*mut sk_buff)->i32>,pub next_state:i32 }
#[repr(C)] pub struct llc_conn_state { pub transitions:*const *const llc_conn_state_trans }

pub const NBR_CONN_STATES:usize=32; pub const NBR_CONN_EV:usize=8; pub const NO_STATE_CHANGE:i32=0;
pub const LLC_DATA_PRIM:i32=1; pub const LLC_CONN_PRIM:i32=2; pub const LLC_DISC_PRIM:i32=3; pub const LLC_RESET_PRIM:i32=4;
pub const LLC_CONN_EV_TYPE_PRIM:i32=0; pub const LLC_CONN_EV_TYPE_SIMPLE:i32=1; pub const LLC_CONN_EV_TYPE_PDU:i32=2; pub const LLC_CONN_EV_TYPE_P_TMR:i32=3; pub const LLC_CONN_EV_TYPE_ACK_TMR:i32=4; pub const LLC_CONN_EV_TYPE_REJ_TMR:i32=5; pub const LLC_CONN_EV_TYPE_BUSY_TMR:i32=6;
pub const LLC_PDU_CMD:u8=0; pub const LLC_PDU_RSP:u8=1; pub const LLC_2_SEQ_NBR_MODULO:u8=8; pub const LLC_CONN_STATE_ADM:u8=1; pub const LLC_CONN_STATE_NORMAL:u8=2; pub const LLC_CONN_STATE_BUSY:u8=3; pub const LLC_CONN_STATE_REJ:u8=4; pub const LLC_CONN_OUT_OF_SVC:u8=0;
pub const SOCK_STREAM:i32=1; pub const TCP_ESTABLISHED:i32=1; pub const TCP_SYN_SENT:i32=2; pub const TCP_CLOSING:i32=3; pub const TCP_CLOSE:i32=0; pub const TCP_LISTEN:i32=4; pub const SS_CONNECTED:i32=1; pub const SS_UNCONNECTED:i32=0; pub const SHUTDOWN_MASK:i32=3; pub const IFF_LOOPBACK:u32=1;
pub const LLC_PACKET:u8=0; pub const LLC_EVENT:u8=1;

pub static mut llc_offset_table:[[i32;NBR_CONN_EV];NBR_CONN_STATES]=[[0;NBR_CONN_EV];NBR_CONN_STATES];
pub static mut sysctl_llc2_ack_timeout:i32=0; pub static mut sysctl_llc2_p_timeout:i32=0; pub static mut sysctl_llc2_rej_timeout:i32=0; pub static mut sysctl_llc2_busy_timeout:i32=0;

extern "C" { fn skb_get(*mut sk_buff); fn skb_queue_tail(*mut sk_buff_head,*mut sk_buff); fn skb_dequeue(*mut sk_buff_head)->*mut sk_buff; fn skb_peek(*mut sk_buff_head)->*mut sk_buff; fn skb_queue_len(*mut sk_buff_head)->i32; fn skb_queue_purge(*mut sk_buff_head); fn kfree_skb(*mut sk_buff); fn skb_clone(*mut sk_buff,u32)->*mut sk_buff; fn dev_queue_xmit(*mut sk_buff); fn llc_save_primitive(*mut sock,*mut sk_buff,i32); fn sock_queue_rcv_skb(*mut sock,*mut sk_buff)->i32; fn sock_hold(*mut sock); fn sock_put(*mut sock); fn sock_net(*mut sock)->*mut net; fn net_eq(*mut net,*const net)->bool; fn ether_addr_equal(*const u8,*const u8)->bool; fn llc_sk_laddr_hashfn(*mut llc_sap,*mut llc_addr)->i32; fn local_bh_disable(); fn local_bh_enable(); fn bh_lock_sock(*mut sock); fn bh_unlock_sock(*mut sock); fn sock_set_flag(*mut sock,i32); fn llc_sap_hold(*mut llc_sap); fn llc_sap_put(*mut llc_sap); }

pub unsafe extern "C" fn llc_data_accept_state_rust(state:u8)->u8 { (state!=LLC_CONN_STATE_NORMAL && state!=LLC_CONN_STATE_BUSY && state!=LLC_CONN_STATE_REJ) as u8 }

pub unsafe extern "C" fn llc_conn_send_pdu(sk:*mut sock,skb:*mut sk_buff){ skb_queue_tail(&mut (*sk).sk_write_queue,skb); llc_conn_send_pdus(sk); }
pub unsafe extern "C" fn llc_conn_rtn_pdu(_: *mut sock,skb:*mut sk_buff){ (*llc_conn_ev(skb)).ind_prim=LLC_DATA_PRIM; }
pub unsafe extern "C" fn llc_conn_resend_i_pdu_as_cmd(sk:*mut sock,nr:u8,mut p:u8){ let mut n=0u16; llc_conn_remove_acked_pdus(sk,nr,&mut n); if n==0{return} let l=llc_sk(sk); loop{let skb=skb_dequeue(&mut (*l).pdu_unack_q);if skb.is_null(){break} llc_pdu_set_cmd_rsp(skb,LLC_PDU_CMD);llc_pdu_set_pf_bit(skb,p);skb_queue_tail(&mut (*sk).sk_write_queue,skb);p=0;(*l).vS=(*l).vS.wrapping_add(1)%LLC_2_SEQ_NBR_MODULO;}llc_conn_send_pdus(sk); }
pub unsafe extern "C" fn llc_conn_resend_i_pdu_as_rsp(sk:*mut sock,nr:u8,mut p:u8){ let mut n=0u16;llc_conn_remove_acked_pdus(sk,nr,&mut n);if n==0{return}let l=llc_sk(sk);loop{let skb=skb_dequeue(&mut (*l).pdu_unack_q);if skb.is_null(){break}llc_pdu_set_cmd_rsp(skb,LLC_PDU_RSP);llc_pdu_set_pf_bit(skb,p);skb_queue_tail(&mut (*sk).sk_write_queue,skb);p=0;}llc_conn_send_pdus(sk); }
pub unsafe extern "C" fn llc_conn_remove_acked_pdus_rust(sk:*mut sock,nr:u8,n:*mut u16)->i32{let l=llc_sk(sk);let q=skb_queue_len(&mut (*l).pdu_unack_q);if q==0{*n=0;return 0}let p=llc_pdu_sn_hdr(skb_peek(&mut (*l).pdu_unack_q));let pos=((LLC_2_SEQ_NBR_MODULO as i32+nr as i32-(*((p as *mut u8).add(0)) as i32))%LLC_2_SEQ_NBR_MODULO as i32) as i32;let mut a=0;for _ in 0..pos.min(q){kfree_skb(skb_dequeue(&mut (*l).pdu_unack_q));a+=1}*n=skb_queue_len(&mut (*l).pdu_unack_q) as u16;a}
pub unsafe fn llc_conn_send_pdus(sk:*mut sock){loop{let skb=skb_dequeue(&mut (*sk).sk_write_queue);if skb.is_null(){break}dev_queue_xmit(skb);}}

pub unsafe extern "C" fn llc_conn_state_process(sk:*mut sock,skb:*mut sk_buff)->i32 {
    let ev=llc_conn_ev(skb); (*ev).ind_prim=0; (*ev).cfm_prim=0;
    let rc=llc_conn_service(sk,skb); kfree_skb(skb); rc
}
pub unsafe extern "C" fn llc_conn_service(sk:*mut sock,skb:*mut sk_buff)->i32 {
    let l=llc_sk(sk); if (*l).state as usize>NBR_CONN_STATES{return 1} 0
}
pub unsafe extern "C" fn llc_build_offset_table(){
    for s in 0..NBR_CONN_STATES { for e in 0..NBR_CONN_EV { llc_offset_table[s][e]=0; } }
}
pub unsafe extern "C" fn llc_find_offset(state:i32,ev:i32)->i32 { if state<0||state as usize>=NBR_CONN_STATES{return 0} match ev { LLC_CONN_EV_TYPE_PRIM=>llc_offset_table[state as usize][0], LLC_CONN_EV_TYPE_SIMPLE=>llc_offset_table[state as usize][1], LLC_CONN_EV_TYPE_PDU=>llc_offset_table[state as usize][4], _=>llc_offset_table[state as usize][3] } }
pub unsafe extern "C" fn llc_sk_stop_all_timers(sk:*mut sock,_sync:bool){let l=llc_sk(sk);(*l).ack_must_be_send=0;(*l).ack_pf=0;}
pub unsafe extern "C" fn llc_sk_reset(sk:*mut sock){let l=llc_sk(sk);skb_queue_purge(&mut (*sk).sk_write_queue);skb_queue_purge(&mut (*l).pdu_unack_q);(*l).remote_busy_flag=0;(*l).cause_flag=0;(*l).retry_count=0;(*l).f_flag=0;(*l).s_flag=0;(*l).ack_pf=0;(*l).first_pdu_Ns=0;(*l).ack_must_be_send=0;(*l).dec_step=1;(*l).inc_cntr=2;(*l).dec_cntr=2;(*l).X=0;(*l).failed_data_req=0;(*l).last_nr=0;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
