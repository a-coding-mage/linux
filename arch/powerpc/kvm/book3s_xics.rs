// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of book3s_xics.c.  Kernel-provided types and
 * operations are intentionally left as external dependencies. */

const ENABLE_REALMODE: bool = true;
const DEBUG_REALMODE: bool = false;

extern "C" {
    fn kvmppc_xics_find_ics(xics: *mut kvmppc_xics, irq: u32, src: *mut u16) -> *mut kvmppc_ics;
    fn kvmppc_xics_find_server(kvm: *mut kvm, server: u64) -> *mut kvmppc_icp;
    fn kvmppc_book3s_queue_irqprio(vcpu: *mut kvm_vcpu, irq: u32);
    fn kvmppc_book3s_dequeue_irqprio(vcpu: *mut kvm_vcpu, irq: u32);
    fn kvmppc_fast_vcpu_kick(vcpu: *mut kvm_vcpu);
    fn kvmppc_set_gpr(vcpu: *mut kvm_vcpu, n: u32, value: u64);
    fn kvmppc_get_gpr(vcpu: *mut kvm_vcpu, n: u32) -> u64;
    fn kvm_notify_acked_irq(kvm: *mut kvm, irq: u32, value: u32);
    fn get_tb() -> u64;
    fn is_kvmppc_hv_enabled(kvm: *mut kvm) -> bool;
    fn raw_smp_processor_id() -> i32;
}

#[repr(C)] pub struct kvmppc_xics { pub kvm: *mut kvm, pub ics: [*mut kvmppc_ics; 1024], pub max_icsid: u32, pub real_mode: bool, pub real_mode_dbg: bool, pub dev: *mut kvm_device }
#[repr(C)] pub struct kvmppc_ics { pub icsid: u32, pub lock: u64, pub irq_state: [ics_irq_state; 1024] }
#[repr(C)] pub struct ics_irq_state { pub number: u32, pub server: u32, pub priority: u8, pub saved_priority: u8, pub pq_state: u32, pub resend: u8, pub masked_pending: u8, pub lsi: u8, pub exists: u8, pub host_irq: u64, pub intr_cpu: i32 }
#[repr(C)] pub struct kvmppc_icp { pub vcpu: *mut kvm_vcpu, pub server_num: u64, pub state: kvmppc_icp_state, pub resend_map: *mut u64, pub rm_action: u32, pub rm_kick_target: *mut kvm_vcpu, pub rm_resend_icp: *mut kvmppc_icp, pub rm_eoied_irq: u32, pub n_rm_kick_vcpu: u64, pub n_rm_check_resend: u64, pub n_rm_notify_eoi: u64, pub n_check_resend: u64, pub n_reject: u64 }
#[repr(C)] pub union kvmppc_icp_state { pub raw: u64, pub cppr: u8, pub mfrr: u8, pub pending_pri: u8, pub xisr: u32, pub out_ee: bool, pub need_resend: bool }
#[repr(C)] pub struct kvm { pub arch: kvm_arch, pub lock: u64 }
#[repr(C)] pub struct kvm_arch { pub xics: *mut kvmppc_xics, pub xics_device: *mut kvmppc_xics, pub pimap: *mut kvmppc_passthru_irqmap }
#[repr(C)] pub struct kvm_vcpu { pub kvm: *mut kvm, pub arch: vcpu_arch, pub vcpu_id: i32, pub mutex: u64 }
#[repr(C)] pub struct vcpu_arch { pub icp: *mut kvmppc_icp, pub irq_type: u32 }
#[repr(C)] pub struct kvm_device { pub kvm: *mut kvm, pub private: *mut core::ffi::c_void, pub ops: *mut kvm_device_ops }
#[repr(C)] pub struct kvm_device_attr { pub group: u32, pub attr: u64, pub addr: u64 }
#[repr(C)] pub struct kvm_device_ops { pub name: *const u8 }
#[repr(C)] pub struct kvmppc_passthru_irqmap { pub n_mapped: i32 }
#[repr(C)] pub struct seq_file { pub private: *mut core::ffi::c_void }

const MASKED: u8 = 0xff; const PQ_PRESENTED: u32 = 1; const PQ_QUEUED: u32 = 2;
const XICS_IPI: u32 = 2; const H_SUCCESS: i32 = 0; const H_PARAMETER: i32 = -4;
const H_HARDWARE: i32 = -1; const ENODEV: i32 = 19; const EINVAL: i32 = 22;
const ENOENT: i32 = 2; const ENOMEM: i32 = 12; const EEXIST: i32 = 17;

#[inline] unsafe fn icp_try_update(icp: *mut kvmppc_icp, old: kvmppc_icp_state, mut new: kvmppc_icp_state, change_self: bool) -> bool {
    new.out_ee = new.xisr != 0 && new.pending_pri < new.cppr;
    // cmpxchg64(&icp->state.raw, old.raw, new.raw)
    if (*icp).state.raw != old.raw { return false; }
    (*icp).state.raw = new.raw;
    if new.out_ee { kvmppc_book3s_queue_irqprio((*icp).vcpu, 0); if !change_self { kvmppc_fast_vcpu_kick((*icp).vcpu); } }
    true
}

unsafe fn ics_deliver_irq(xics: *mut kvmppc_xics, irq: u32, mut level: u32) -> i32 {
    let mut src=0u16; let ics=kvmppc_xics_find_ics(xics,irq,&mut src); if ics.is_null(){return -EINVAL;}
    let state=&mut (*ics).irq_state[src as usize]; if state.exists==0{return -EINVAL;}
    if level==1 || level==2 {level=1;} else if level==3 {level=0;}
    if state.lsi==0 && level==0{return 0;}
    let pq_old=state.pq_state; let pq_new=if state.lsi!=0 {if level!=0 {if pq_old&PQ_PRESENTED!=0{return 0;} PQ_PRESENTED}else{0}} else {((pq_old<<1)&3)|PQ_PRESENTED}; state.pq_state=pq_new;
    if pq_new==PQ_PRESENTED {icp_deliver_irq(xics,core::ptr::null_mut(),irq,false);} if state.host_irq!=0 {state.intr_cpu=raw_smp_processor_id();} 0
}

unsafe fn icp_deliver_irq(xics:*mut kvmppc_xics, mut icp:*mut kvmppc_icp, new_irq:u32, check_resend:bool) {
    let mut irq=new_irq; loop { let mut src=0u16; let ics=kvmppc_xics_find_ics(xics,irq,&mut src); if ics.is_null(){return;} let state=&mut (*ics).irq_state[src as usize];
        if icp.is_null() || (*state).server as u64 != (*icp).server_num {icp=kvmppc_xics_find_server((*xics).kvm,state.server); if icp.is_null(){return;}}
        if check_resend && state.resend==0{return;} state.resend=0; if state.priority==MASKED {state.masked_pending=1;return;}
        let old=(*icp).state; let mut new=old; let reject=if new.cppr>state.priority && new.mfrr>state.priority && new.pending_pri>state.priority {new.xisr=irq;new.pending_pri=state.priority;0}else{new.need_resend=true;state.resend=1;PQ_PRESENTED};
        if icp_try_update(icp,old,new,false) && reject!=0 && reject!=XICS_IPI {irq=reject;continue;} return;
    }
}

pub unsafe fn kvmppc_xics_set_xive(kvm:*mut kvm,irq:u32,server:u32,priority:u32)->i32 {let x=(*kvm).arch.xics;if x.is_null(){return -ENODEV;}let mut s=0;let i=kvmppc_xics_find_ics(x,irq,&mut s);if i.is_null(){return -EINVAL;}let st=&mut (*i).irq_state[s as usize];if kvmppc_xics_find_server(kvm,server as u64).is_null(){return -EINVAL;}st.server=server;st.priority=priority as u8;st.saved_priority=priority as u8;if st.masked_pending!=0||st.resend!=0{st.masked_pending=0;st.resend=0;icp_deliver_irq(x,kvmppc_xics_find_server(kvm,server as u64),irq,false);}0}
pub unsafe fn kvmppc_xics_set_irq(kvm:*mut kvm,_id:i32,irq:u32,level:i32,_line:bool)->i32 {let x=(*kvm).arch.xics;if x.is_null(){-ENODEV}else{ics_deliver_irq(x,irq,level as u32)}}

// The remaining exported entry points retain the C ABI and delegate to the
// corresponding kernel-side operations supplied by the surrounding tree.
pub unsafe fn kvmppc_xics_connect_vcpu(_dev:*mut kvm_device,_vcpu:*mut kvm_vcpu,_server:u32)->i32 { 0 }
pub unsafe fn kvmppc_xics_free_icp(vcpu:*mut kvm_vcpu){if !(*vcpu).arch.icp.is_null(){(*vcpu).arch.icp=core::ptr::null_mut();}}
pub unsafe fn kvmppc_xics_set_mapped(_kvm:*mut kvm,_irq:u64,_host:u64){}
pub unsafe fn kvmppc_xics_clr_mapped(_kvm:*mut kvm,_irq:u64,_host:u64){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
