// SPDX-License-Identifier: GPL-2.0-only
/* shdlc Link Layer Control */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ShdlcState { Disconnected = 0, Connecting = 1, Negotiating = 2, HalfConnected = 3, Connected = 4 }

#[repr(C)]
struct LlcShdlc {
    hdev: *mut nfc_hci_dev, xmit_to_drv: xmit_to_drv_t, rcv_to_hci: rcv_to_hci_t,
    state_mutex: mutex, state: ShdlcState, hard_fault: i32,
    connect_wq: *mut wait_queue_head_t, connect_tries: i32, connect_result: i32,
    connect_timer: timer_list, w: u8, srej_support: bool,
    t1_timer: timer_list, t1_active: bool, t2_timer: timer_list, t2_active: bool,
    ns: i32, nr: i32, dnr: i32, rcv_q: sk_buff_head, send_q: sk_buff_head,
    rnr: bool, ack_pending_q: sk_buff_head, sm_work: work_struct,
    tx_headroom: i32, tx_tailroom: i32, llc_failure: llc_failure_t,
}

const SHDLC_LLC_HEAD_ROOM: i32 = 2;
const SHDLC_MAX_WINDOW: u8 = 4;
const SHDLC_SREJ_SUPPORT: bool = false;
const SHDLC_CONTROL_HEAD_MASK: u8 = 0xe0;
const SHDLC_CONTROL_HEAD_I: u8 = 0x80;
const SHDLC_CONTROL_HEAD_I2: u8 = 0xa0;
const SHDLC_CONTROL_HEAD_S: u8 = 0xc0;
const SHDLC_CONTROL_HEAD_U: u8 = 0xe0;
const SHDLC_CONTROL_NS_MASK: u8 = 0x38;
const SHDLC_CONTROL_NR_MASK: u8 = 0x07;
const SHDLC_CONTROL_TYPE_MASK: u8 = 0x18;
const SHDLC_CONTROL_M_MASK: u8 = 0x1f;
const SHDLC_CONNECT_VALUE_MS: u64 = 5;
const SHDLC_T2_VALUE_MS: u64 = 300;

#[repr(C)] enum SframeType { RR=0, REJ=1, RNR=2, SREJ=3 }
#[repr(C)] enum UframeModifier { UA=6, RSET=0x19 }

#[inline] fn shdlc_t1_value_ms(w: u8) -> u64 { (5 * w as u64) / 4 }

unsafe fn llc_shdlc_x_lt_y_lteq_z(x:i32,y:i32,z:i32)->bool { if x<z {(x<y)&&(y<=z)} else {(y>x)||(y<=z)} }
unsafe fn llc_shdlc_x_lteq_y_lt_z(x:i32,y:i32,z:i32)->bool { if x<=z {(x<=y)&&(y<z)} else {(y>=x)||(y<z)} }

unsafe fn llc_shdlc_alloc_skb(s:&LlcShdlc, len:i32)->*mut sk_buff {
    let skb=alloc_skb(s.tx_headroom+SHDLC_LLC_HEAD_ROOM+s.tx_tailroom+len,GFP_KERNEL);
    if !skb.is_null(){skb_reserve(skb,(s.tx_headroom+SHDLC_LLC_HEAD_ROOM) as u32);} skb
}
unsafe fn llc_shdlc_send_s_frame(s:&LlcShdlc,t:SframeType,nr:i32)->i32 { let skb=llc_shdlc_alloc_skb(s,0); if skb.is_null(){return -ENOMEM;} *(skb_push(skb,1) as *mut u8)=SHDLC_CONTROL_HEAD_S|((t as u8)<<3)|nr as u8; let r=(s.xmit_to_drv)(s.hdev,skb); kfree_skb(skb); r }
unsafe fn llc_shdlc_send_u_frame(s:&LlcShdlc,skb:*mut sk_buff,m:UframeModifier)->i32 { *(skb_push(skb,1) as *mut u8)=SHDLC_CONTROL_HEAD_U|m as u8; let r=(s.xmit_to_drv)(s.hdev,skb); kfree_skb(skb); r }

unsafe fn llc_shdlc_reset_t2(s:&mut LlcShdlc,y:i32){let mut d=s.dnr;while d!=y{let p=skb_dequeue(&mut s.ack_pending_q);kfree_skb(p);d=(d+1)%8;}if skb_queue_empty(&s.ack_pending_q){if s.t2_active{timer_delete_sync(&mut s.t2_timer);s.t2_active=false;}}else{let p=skb_peek(&s.ack_pending_q);mod_timer(&mut s.t2_timer,*( (*p).cb.as_ptr() as *const u64)+msecs_to_jiffies(SHDLC_T2_VALUE_MS));s.t2_active=true;}}
unsafe fn llc_shdlc_rcv_i_frame(s:&mut LlcShdlc,mut skb:*mut sk_buff,ns:i32,nr:i32){if s.state!=ShdlcState::Connected{ kfree_skb(skb);return;}if ns!=s.nr{llc_shdlc_send_s_frame(s,SframeType::REJ,s.nr);kfree_skb(skb);return;}if !s.t1_active{s.t1_active=true;mod_timer(&mut s.t1_timer,jiffies+msecs_to_jiffies(shdlc_t1_value_ms(s.w)));}if (*skb).len!=0{(s.rcv_to_hci)(s.hdev,skb);skb=core::ptr::null_mut();}s.nr=(s.nr+1)%8;if llc_shdlc_x_lt_y_lteq_z(s.dnr,nr,s.ns){llc_shdlc_reset_t2(s,nr);s.dnr=nr;}kfree_skb(skb);}
unsafe fn llc_shdlc_rcv_ack(s:&mut LlcShdlc,nr:i32){if llc_shdlc_x_lt_y_lteq_z(s.dnr,nr,s.ns){llc_shdlc_reset_t2(s,nr);s.dnr=nr;}}
unsafe fn llc_shdlc_requeue_ack_pending(s:&mut LlcShdlc){while let Some(p)=skb_dequeue_tail(&mut s.ack_pending_q){skb_pull(p,1);skb_queue_head(&mut s.send_q,p);}s.ns=s.dnr;}
unsafe fn llc_shdlc_rcv_rej(s:&mut LlcShdlc,nr:i32){if llc_shdlc_x_lteq_y_lt_z(s.dnr,nr,s.ns){if s.t2_active{timer_delete_sync(&mut s.t2_timer);s.t2_active=false;}while s.dnr!=nr{s.dnr=(s.dnr+1)%8;kfree_skb(skb_dequeue(&mut s.ack_pending_q));}llc_shdlc_requeue_ack_pending(s);}}

unsafe fn llc_shdlc_rcv_s_frame(s:&mut LlcShdlc,t:SframeType,nr:i32){if s.state!=ShdlcState::Connected{return;}match t{SframeType::RR=>{llc_shdlc_rcv_ack(s,nr);if s.rnr{s.rnr=false;if s.send_q.qlen==0{let p=llc_shdlc_alloc_skb(s,0);if !p.is_null(){skb_queue_tail(&mut s.send_q,p);}}}},SframeType::REJ=>llc_shdlc_rcv_rej(s,nr),SframeType::RNR=>{llc_shdlc_rcv_ack(s,nr);s.rnr=true},_=>{}}}
unsafe fn llc_shdlc_connect_complete(s:&mut LlcShdlc,r:i32){timer_delete_sync(&mut s.connect_timer);if r==0{s.ns=0;s.nr=0;s.dnr=0;s.state=ShdlcState::HalfConnected}else{s.state=ShdlcState::Disconnected}s.connect_result=r;wake_up(s.connect_wq);}
unsafe fn llc_shdlc_connect_initiate(s:&LlcShdlc)->i32{let p=llc_shdlc_alloc_skb(s,2);if p.is_null(){return -ENOMEM;}skb_put_u8(p,SHDLC_MAX_WINDOW);skb_put_u8(p,if SHDLC_SREJ_SUPPORT{1}else{0});llc_shdlc_send_u_frame(s,p,UframeModifier::RSET)}
unsafe fn llc_shdlc_connect_send_ua(s:&LlcShdlc)->i32{let p=llc_shdlc_alloc_skb(s,0);if p.is_null(){return -ENOMEM;}llc_shdlc_send_u_frame(s,p,UframeModifier::UA)}

unsafe fn llc_shdlc_rcv_u_frame(s:&mut LlcShdlc,skb:*mut sk_buff,m:UframeModifier){match m{UframeModifier::RSET=>match s.state{ShdlcState::Negotiating|ShdlcState::Connecting=>{let w=if (*skb).len>0{(*skb).data[0]}else{SHDLC_MAX_WINDOW};let sr=if (*skb).len>1{(*skb).data[1]&1!=0}else{SHDLC_SREJ_SUPPORT};if w<=SHDLC_MAX_WINDOW&&(SHDLC_SREJ_SUPPORT||!sr){s.w=w;s.srej_support=sr;let r=llc_shdlc_connect_send_ua(s);llc_shdlc_connect_complete(s,r);}},ShdlcState::Connected=>s.hard_fault=-ECONNRESET,_=>{}},UframeModifier::UA=>{if (s.state==ShdlcState::Connecting&&s.connect_tries>0)||s.state==ShdlcState::Negotiating{llc_shdlc_connect_complete(s,0);s.state=ShdlcState::Connected}},_=>{}}kfree_skb(skb);}

unsafe fn llc_shdlc_w_used(ns:i32,dnr:i32)->i32{if dnr<=ns{ns-dnr}else{8-dnr+ns}}

unsafe fn llc_shdlc_handle_rcv_queue(s:&mut LlcShdlc){while let Some(p)=skb_dequeue(&mut s.rcv_q){let c=(*p).data[0];skb_pull(p,1);match c&SHDLC_CONTROL_HEAD_MASK{SHDLC_CONTROL_HEAD_I|SHDLC_CONTROL_HEAD_I2=>{if s.state==ShdlcState::HalfConnected{s.state=ShdlcState::Connected}llc_shdlc_rcv_i_frame(s,p,((c&SHDLC_CONTROL_NS_MASK)>>3) as i32,(c&SHDLC_CONTROL_NR_MASK) as i32)},SHDLC_CONTROL_HEAD_S=>{if s.state==ShdlcState::HalfConnected{s.state=ShdlcState::Connected}llc_shdlc_rcv_s_frame(s,match (c&SHDLC_CONTROL_TYPE_MASK)>>3{0=>SframeType::RR,1=>SframeType::REJ,2=>SframeType::RNR,_=>SframeType::SREJ},(c&7) as i32);kfree_skb(p)},SHDLC_CONTROL_HEAD_U=>llc_shdlc_rcv_u_frame(s,p,match c&SHDLC_CONTROL_M_MASK{6=>UframeModifier::UA,0x19=>UframeModifier::RSET,_=>{kfree_skb(p);continue}}),_=>kfree_skb(p)}}}

unsafe fn llc_shdlc_handle_send_queue(s:&mut LlcShdlc){while s.send_q.qlen!=0&&s.ack_pending_q.qlen<s.w as u32&&!s.rnr{if s.t1_active{timer_delete_sync(&mut s.t1_timer);s.t1_active=false;}let p=skb_dequeue(&mut s.send_q);*(skb_push(p,1) as *mut u8)=SHDLC_CONTROL_HEAD_I|((s.ns as u8)<<3)|s.nr as u8;let r=(s.xmit_to_drv)(s.hdev,p);if r<0{s.hard_fault=r;break}s.ns=(s.ns+1)%8;*( (*p).cb.as_mut_ptr() as *mut u64)=jiffies;skb_queue_tail(&mut s.ack_pending_q,p);if !s.t2_active{s.t2_active=true;mod_timer(&mut s.t2_timer,jiffies+msecs_to_jiffies(SHDLC_T2_VALUE_MS));}}}

unsafe fn llc_shdlc_connect_timeout(t:*mut timer_list){schedule_work(&mut (*(timer_container_of(t,0) as *mut LlcShdlc)).sm_work)}
unsafe fn llc_shdlc_t1_timeout(t:*mut timer_list){schedule_work(&mut (*(timer_container_of(t,0) as *mut LlcShdlc)).sm_work)}
unsafe fn llc_shdlc_t2_timeout(t:*mut timer_list){schedule_work(&mut (*(timer_container_of(t,0) as *mut LlcShdlc)).sm_work)}

unsafe fn llc_shdlc_recv_frame(s:&mut LlcShdlc,p:*mut sk_buff){if p.is_null(){s.hard_fault=-EREMOTEIO}else{skb_queue_tail(&mut s.rcv_q,p)}schedule_work(&mut s.sm_work)}
unsafe fn llc_shdlc_connect(s:&mut LlcShdlc)->i32{let mut w=core::mem::zeroed::<wait_queue_head_t>();mutex_lock(&mut s.state_mutex);s.state=ShdlcState::Connecting;s.connect_wq=&mut w;s.connect_tries=0;s.connect_result=1;mutex_unlock(&mut s.state_mutex);schedule_work(&mut s.sm_work);wait_event(&mut w,s.connect_result!=1);s.connect_result}
unsafe fn llc_shdlc_disconnect(s:&mut LlcShdlc){mutex_lock(&mut s.state_mutex);s.state=ShdlcState::Disconnected;mutex_unlock(&mut s.state_mutex);schedule_work(&mut s.sm_work)}

unsafe fn llc_shdlc_sm_work(work:*mut work_struct){let s=&mut *(container_of(work,0) as *mut LlcShdlc);mutex_lock(&mut s.state_mutex);match s.state{ShdlcState::Disconnected=>{skb_queue_purge(&mut s.rcv_q);skb_queue_purge(&mut s.send_q);skb_queue_purge(&mut s.ack_pending_q)},ShdlcState::Connecting=>{let r=if s.hard_fault!=0{s.hard_fault}else if s.connect_tries<5{s.connect_tries+=1;llc_shdlc_connect_initiate(s)}else{-ETIME};if r<0{llc_shdlc_connect_complete(s,r)}else{mod_timer(&mut s.connect_timer,jiffies+msecs_to_jiffies(SHDLC_CONNECT_VALUE_MS));s.state=ShdlcState::Negotiating}},ShdlcState::Negotiating=>{if timer_pending(&s.connect_timer)==0{s.state=ShdlcState::Connecting;schedule_work(&mut s.sm_work)}llc_shdlc_handle_rcv_queue(s);if s.hard_fault!=0{llc_shdlc_connect_complete(s,s.hard_fault)}},ShdlcState::HalfConnected|ShdlcState::Connected=>{llc_shdlc_handle_rcv_queue(s);llc_shdlc_handle_send_queue(s);if s.t1_active&&timer_pending(&s.t1_timer)==0{s.t1_active=false;let r=llc_shdlc_send_s_frame(s,SframeType::RR,s.nr);if r<0{s.hard_fault=r}}if s.t2_active&&timer_pending(&s.t2_timer)==0{s.t2_active=false;llc_shdlc_requeue_ack_pending(s);llc_shdlc_handle_send_queue(s)}if s.hard_fault!=0{(s.llc_failure)(s.hdev,s.hard_fault)}},_=>{}}mutex_unlock(&mut s.state_mutex)}

unsafe fn llc_shdlc_rcv_from_drv(llc:*mut nfc_llc,skb:*mut sk_buff){llc_shdlc_recv_frame(&mut *(nfc_llc_get_data(llc) as *mut LlcShdlc),skb)}
unsafe fn llc_shdlc_xmit_from_hci(llc:*mut nfc_llc,skb:*mut sk_buff)->i32{let s=&mut *(nfc_llc_get_data(llc) as *mut LlcShdlc);skb_queue_tail(&mut s.send_q,skb);schedule_work(&mut s.sm_work);0}
unsafe fn llc_shdlc_start(llc:*mut nfc_llc)->i32{llc_shdlc_connect(&mut *(nfc_llc_get_data(llc) as *mut LlcShdlc))}
unsafe fn llc_shdlc_stop(llc:*mut nfc_llc)->i32{llc_shdlc_disconnect(&mut *(nfc_llc_get_data(llc) as *mut LlcShdlc));0}
unsafe fn llc_shdlc_deinit(llc:*mut nfc_llc){let s=nfc_llc_get_data(llc) as *mut LlcShdlc;timer_shutdown_sync(&mut (*s).connect_timer);timer_shutdown_sync(&mut (*s).t1_timer);timer_shutdown_sync(&mut (*s).t2_timer);cancel_work_sync(&mut (*s).sm_work);skb_queue_purge(&mut (*s).rcv_q);skb_queue_purge(&mut (*s).send_q);skb_queue_purge(&mut (*s).ack_pending_q);kfree(s as *mut _)}

unsafe fn llc_shdlc_init(hdev:*mut nfc_hci_dev,x:xmit_to_drv_t,r:rcv_to_hci_t,head:i32,tail:i32,rh:*mut i32,rt:*mut i32,f:llc_failure_t)->*mut core::ffi::c_void{*rh=SHDLC_LLC_HEAD_ROOM;*rt=0;let s=kzalloc_obj::<LlcShdlc>();if s.is_null(){return core::ptr::null_mut()}(*s).hdev=hdev;(*s).xmit_to_drv=x;(*s).rcv_to_hci=r;(*s).tx_headroom=head;(*s).tx_tailroom=tail;(*s).llc_failure=f;(*s).state=ShdlcState::Disconnected;(*s).w=SHDLC_MAX_WINDOW;skb_queue_head_init(&mut (*s).rcv_q);skb_queue_head_init(&mut (*s).send_q);skb_queue_head_init(&mut (*s).ack_pending_q);s as *mut _ as *mut core::ffi::c_void}

unsafe fn nfc_llc_shdlc_register()->i32{nfc_llc_register(LLC_SHDLC_NAME,&llc_shdlc_ops)}
static llc_shdlc_ops:nfc_llc_ops=nfc_llc_ops{init:llc_shdlc_init,deinit:llc_shdlc_deinit,start:llc_shdlc_start,stop:llc_shdlc_stop,rcv_from_drv:llc_shdlc_rcv_from_drv,xmit_from_hci:llc_shdlc_xmit_from_hci};

// External kernel and LLC declarations are supplied by the surrounding translation unit.
extern "C" { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
