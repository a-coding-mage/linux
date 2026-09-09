// SPDX-License-Identifier: GPL-2.0
// Rust translation of the SCMI VirtIO transport implementation.

const VIRTIO_MAX_RX_TIMEOUT_MS: u32 = 60000;
const VIRTIO_SCMI_MAX_MSG_SIZE: u32 = 128;
const DESCRIPTORS_PER_TX_MSG: usize = 2;

#[repr(C)]
pub struct ScmiVioChannel {
    pub vqueue: *mut virtqueue,
    pub cinfo: *mut scmi_chan_info,
    pub free_lock: spinlock_t,
    pub free_list: list_head,
    pub pending_lock: spinlock_t,
    pub pending_cmds_list: list_head,
    pub deferred_tx_work: work_struct,
    pub deferred_tx_wq: *mut workqueue_struct,
    pub is_rx: bool,
    pub max_msg: u32,
    pub lock: spinlock_t,
    pub shutdown_done: *mut completion,
    pub users: refcount_t,
}

#[repr(i32)]
pub enum PollStates { VioMsgNotPolled, VioMsgPollTimeout, VioMsgPolling, VioMsgPollDone }

#[repr(C)]
pub struct ScmiVioMsg {
    pub request: *mut scmi_msg_payld,
    pub input: *mut scmi_msg_payld,
    pub list: list_head,
    pub rx_len: u32,
    pub max_len: u32,
    pub poll_idx: u32,
    pub poll_status: PollStates,
    pub poll_lock: spinlock_t,
    pub users: refcount_t,
}

static mut CORE: *mut scmi_transport_core_operations = core::ptr::null_mut();
static mut SCMI_VDEV: *mut virtio_device = core::ptr::null_mut();

unsafe fn channel_ready(v: *mut ScmiVioChannel, c: *mut scmi_chan_info) {
    let mut f = 0; spin_lock_irqsave(&mut (*v).lock, &mut f);
    (*c).transport_info = v as *mut _; (*v).cinfo = c;
    spin_unlock_irqrestore(&mut (*v).lock, f); refcount_set(&mut (*v).users, 1);
}
unsafe fn channel_acquire(v: *mut ScmiVioChannel) -> bool { refcount_inc_not_zero(&mut (*v).users) }
unsafe fn channel_release(v: *mut ScmiVioChannel) {
    if refcount_dec_and_test(&mut (*v).users) { let mut f=0; spin_lock_irqsave(&mut (*v).lock,&mut f); if !(*v).shutdown_done { (*v).cinfo=core::ptr::null_mut(); complete((*v).shutdown_done); } spin_unlock_irqrestore(&mut (*v).lock,f); }
}
unsafe fn get_free_msg(v: *mut ScmiVioChannel) -> *mut ScmiVioMsg {
    let mut f=0; spin_lock_irqsave(&mut (*v).free_lock,&mut f); if list_empty(&(*v).free_list) { spin_unlock_irqrestore(&mut (*v).free_lock,f); return core::ptr::null_mut(); }
    let m=list_first_entry(&(*v).free_list); list_del_init(&mut (*m).list); spin_unlock_irqrestore(&mut (*v).free_lock,f); (*m).poll_status=PollStates::VioMsgNotPolled; refcount_set(&mut (*m).users,1); m
}
unsafe fn msg_acquire(m:*mut ScmiVioMsg)->bool { refcount_inc_not_zero(&mut (*m).users) }
unsafe fn msg_release(v:*mut ScmiVioChannel,m:*mut ScmiVioMsg)->bool { let r=refcount_dec_and_test(&mut (*m).users); if r { let mut f=0; spin_lock_irqsave(&mut (*v).free_lock,&mut f); list_add_tail(&mut (*m).list,&mut (*v).free_list); spin_unlock_irqrestore(&mut (*v).free_lock,f); } r }
unsafe fn have_rx(v:*mut virtio_device)->bool { virtio_has_feature(v,VIRTIO_SCMI_F_P2A_CHANNELS) }

unsafe fn feed_rx(v:*mut ScmiVioChannel,m:*mut ScmiVioMsg)->i32 { let mut sg=core::mem::zeroed(); sg_init_one(&mut sg,(*m).input,(*m).max_len as usize); let mut f=0; spin_lock_irqsave(&mut (*v).lock,&mut f); let r=virtqueue_add_inbuf((*v).vqueue,&mut sg,1,m as *mut _,GFP_ATOMIC); if r==0 { virtqueue_kick((*v).vqueue); } spin_unlock_irqrestore(&mut (*v).lock,f); r }
unsafe fn finalize(v:*mut ScmiVioChannel,m:*mut ScmiVioMsg) { if (*v).is_rx { feed_rx(v,m); } else { msg_release(v,m); } }

unsafe extern "C" fn complete_cb(q:*mut virtqueue) { if (*(*q).vdev).priv_.is_null(){return;} let v=&mut *((*(*q).vdev).priv_ as *mut ScmiVioChannel).add((*q).index as usize); loop { if !channel_acquire(v){return;} let mut f=0; spin_lock_irqsave(&mut v.lock,&mut f); virtqueue_disable_cb(q); let mut len=0; let m=virtqueue_get_buf(q,&mut len); if m.is_null() && virtqueue_enable_cb(q) { spin_unlock_irqrestore(&mut v.lock,f); channel_release(v); return; } spin_unlock_irqrestore(&mut v.lock,f); if !m.is_null() { (*m).rx_len=len; (*CORE).rx_callback(v.cinfo,(*(*CORE).msg).read_header((*m).input),m); finalize(v,m); } channel_release(v); } }

unsafe fn chan_available(_n:*mut device_node,idx:i32)->bool { if SCMI_VDEV.is_null(){return false;} let c=(*SCMI_VDEV).priv_ as *mut ScmiVioChannel; let v=match idx { VIRTIO_SCMI_VQ_TX=>c, VIRTIO_SCMI_VQ_RX=>if have_rx(SCMI_VDEV){c.add(1)}else{core::ptr::null_mut()}, _=>core::ptr::null_mut() }; !v.is_null()&&(*v).cinfo.is_null() }
unsafe fn get_max(c:*mut scmi_chan_info)->u32 { (*( (*c).transport_info as *mut ScmiVioChannel)).max_msg }
unsafe fn chan_free(_id:i32,p:*mut core::ffi::c_void,_d:*mut core::ffi::c_void)->i32 { let c=p as *mut scmi_chan_info; let v=(*c).transport_info as *mut ScmiVioChannel; virtio_break_device((*v).vqueue); channel_release(v); 0 }

unsafe fn send_message(c:*mut scmi_chan_info,x:*mut scmi_xfer)->i32 {
    let v=(*c).transport_info as *mut ScmiVioChannel; if !channel_acquire(v){return -EINVAL;}
    let m=get_free_msg(v); if m.is_null(){channel_release(v);return -EBUSY;}
    (*CORE).msg.tx_prepare((*m).request,x); let mut out=core::mem::zeroed(); let mut input=core::mem::zeroed();
    sg_init_one(&mut out,(*m).request,(*CORE).msg.command_size(x) as usize); sg_init_one(&mut input,(*m).input,(*CORE).msg.response_size(x) as usize);
    let mut f=0; spin_lock_irqsave(&mut (*v).lock,&mut f); let r=virtqueue_add_sgs((*v).vqueue,&mut out,&mut input,m as *mut _,GFP_ATOMIC); if r==0{virtqueue_kick((*v).vqueue);} spin_unlock_irqrestore(&mut (*v).lock,f); if r!=0{msg_release(v,m);} channel_release(v); r
}
unsafe fn fetch_response(_c:*mut scmi_chan_info,x:*mut scmi_xfer){let m=(*x).priv_ as *mut ScmiVioMsg;if !m.is_null(){(*CORE).msg.fetch_response((*m).input,(*m).rx_len,x);}}
unsafe fn fetch_notification(_c:*mut scmi_chan_info,max:usize,x:*mut scmi_xfer){let m=(*x).priv_ as *mut ScmiVioMsg;if !m.is_null(){(*CORE).msg.fetch_notification((*m).input,(*m).rx_len,max,x);}}
unsafe fn mark_txdone(c:*mut scmi_chan_info,ret:i32,x:*mut scmi_xfer){let v=(*c).transport_info as *mut ScmiVioChannel;let m=(*x).priv_ as *mut ScmiVioMsg;if !m.is_null()&&channel_acquire(v){(*x).priv_=core::ptr::null_mut();if ret!=-ETIMEDOUT||msg_release(v,m){ } channel_release(v);}}
unsafe fn poll_done(c:*mut scmi_chan_info,x:*mut scmi_xfer)->bool{let v=(*c).transport_info as *mut ScmiVioChannel;let m=(*x).priv_ as *mut ScmiVioMsg;if m.is_null(){return true;}if (*m).poll_status==PollStates::VioMsgPollDone{return true;}if !channel_acquire(v){return true;}let r=virtqueue_poll((*v).vqueue,(*m).poll_idx);channel_release(v);r}

#[repr(C)] pub struct VirtioVioOps { pub chan_available: unsafe fn(*mut device_node,i32)->bool, pub chan_free: unsafe fn(i32,*mut core::ffi::c_void,*mut core::ffi::c_void)->i32, pub send_message: unsafe fn(*mut scmi_chan_info,*mut scmi_xfer)->i32, pub fetch_response: unsafe fn(*mut scmi_chan_info,*mut scmi_xfer), pub fetch_notification: unsafe fn(*mut scmi_chan_info,usize,*mut scmi_xfer), pub mark_txdone: unsafe fn(*mut scmi_chan_info,i32,*mut scmi_xfer), pub poll_done: unsafe fn(*mut scmi_chan_info,*mut scmi_xfer)->bool }
static SCMI_VIRTIO_OPS: VirtioVioOps=VirtioVioOps{chan_available,chan_free,send_message,fetch_response,fetch_notification,mark_txdone,poll_done};

// External kernel and SCMI declarations, supplied by the surrounding translation unit.
extern "C" {
    fn virtio_has_feature(*mut virtio_device,u64)->bool; fn virtqueue_kick(*mut virtqueue); fn virtqueue_disable_cb(*mut virtqueue); fn virtqueue_enable_cb(*mut virtqueue)->bool; fn virtqueue_add_inbuf(*mut virtqueue,*mut scatterlist,u32,*mut core::ffi::c_void,u32)->i32; fn virtqueue_get_buf(*mut virtqueue,*mut u32)->*mut ScmiVioMsg; fn virtio_break_device(*mut virtio_device); fn spin_lock_irqsave(*mut spinlock_t,*mut u64); fn spin_unlock_irqrestore(*mut spinlock_t,u64); fn list_empty(*const list_head)->bool; fn list_del_init(*mut list_head); fn list_add_tail(*mut list_head,*mut list_head); fn list_first_entry(*const list_head)->*mut ScmiVioMsg; fn refcount_set(*mut refcount_t,u32); fn refcount_inc_not_zero(*mut refcount_t)->bool; fn refcount_dec_and_test(*mut refcount_t)->bool; fn complete(*mut completion); fn sg_init_one(*mut scatterlist,*mut core::ffi::c_void,usize); fn smp_store_mb<T>(_:*mut T,_:T);
}

// Driver registration, probe/remove, feature validation, and module init/exit are supplied
// by the surrounding kernel binding in the final translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
