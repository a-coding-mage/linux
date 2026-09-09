// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Rust translation of kfd_int_process_v12_1.c. */

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
#[repr(u32)]
enum SQ_INTERRUPT_WORD_ENCODING {
    SQ_INTERRUPT_WORD_ENCODING_AUTO = 0x0,
    SQ_INTERRUPT_WORD_ENCODING_INST,
    SQ_INTERRUPT_WORD_ENCODING_ERROR,
}

#[repr(u32)]
enum SQ_INTERRUPT_ERROR_TYPE {
    SQ_INTERRUPT_ERROR_TYPE_EDC_FUE = 0x0,
    SQ_INTERRUPT_ERROR_TYPE_ILLEGAL_INST,
    SQ_INTERRUPT_ERROR_TYPE_MEMVIOL,
    SQ_INTERRUPT_ERROR_TYPE_EDC_FED,
}

const KFD_CTXID0_TRAP_CODE_SHIFT: u32 = 10;
const KFD_CTXID0_TRAP_CODE_MASK: u32 = 0xfffc00;
const KFD_CTXID0_CP_BAD_OP_ECODE_MASK: u32 = 0x3ffffff;
const KFD_CTXID0_DOORBELL_ID_MASK: u32 = 0x0003ff;

const fn kfd_ctxid0_trap_code(x: u32) -> u32 { (x & KFD_CTXID0_TRAP_CODE_MASK) >> KFD_CTXID0_TRAP_CODE_SHIFT }
const fn kfd_ctxid0_cp_bad_op_ecode(x: u32) -> u32 { (x & KFD_CTXID0_CP_BAD_OP_ECODE_MASK) >> KFD_CTXID0_TRAP_CODE_SHIFT }
const fn kfd_ctxid0_doorbell_id(x: u32) -> u32 { x & KFD_CTXID0_DOORBELL_ID_MASK }

const AUTO_THREAD_TRACE_SHIFT: u32 = 0;
const AUTO_WLT_SHIFT: u32 = 1;
const AUTO_BUF0_SHIFT: u32 = 2;
const AUTO_BUF1_SHIFT: u32 = 3;
const AUTO_UTC_SHIFT: u32 = 8;
const AUTO_ENCODING_SHIFT: u32 = 6;
const AUTO_THREAD_TRACE_MASK: u32 = 0x00000001;
const AUTO_WLT_MASK: u32 = 0x00000002;
const AUTO_BUF0_MASK: u32 = 0x00000004;
const AUTO_BUF1_MASK: u32 = 0x00000008;
const AUTO_UTC_MASK: u32 = 0x00000100;

const WAVE_DATA_MASK: u32 = 0x00ffffff;
const WAVE_PRIV_SHIFT: u32 = 26;
const WAVE_ENCODING_SHIFT: u32 = 6;
const WAVE_ENCODING_MASK: u32 = 0x000000c0;
const ERROR_DETAIL_MASK: u32 = 0x0007ffff;
const ERROR_TYPE_SHIFT: u32 = 21;
const ERROR_TYPE_MASK: u32 = 0x01e00000;

#[repr(C)] pub struct kfd_node { pub adev: *mut amdgpu_device, pub dqm: *mut kfd_device_queue_manager, pub vm_info: kfd_vm_info, pub id: u32 }
#[repr(C)] pub struct amdgpu_device { pub dev: *mut core::ffi::c_void }
#[repr(C)] pub struct kfd_device_queue_manager { pub ops: kfd_dqm_ops }
#[repr(C)] pub struct kfd_dqm_ops { pub reset_queues: Option<unsafe extern "C" fn(*mut kfd_device_queue_manager, u16) -> i32> }
#[repr(C)] pub struct kfd_vm_info { pub first_vmid_kfd: u16, pub last_vmid_kfd: u16 }
#[repr(C)] pub struct kfd_process { pub poison: atomic_t }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct kfd_vm_fault_info { pub vmid: u16, pub mc_id: u16, pub page_addr: u64, pub prot_valid: u16, pub prot_read: u16, pub prot_write: u16, pub prot_exec: u16 }
#[repr(C)] pub struct kfd_hsa_memory_exception_data { pub gpu_id: u32, pub va: u64, pub failure: kfd_hsa_memory_exception_failure }
#[repr(C)] pub struct kfd_hsa_memory_exception_failure { pub NotPresent: u32, pub NoExecute: u32, pub ReadOnly: u32, pub imprecise: u32 }

extern "C" {
    fn dev_dbg_ratelimited(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn dev_warn_ratelimited(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn kfd_lookup_process_by_pasid(pasid: u16, x: *mut core::ffi::c_void) -> *mut kfd_process;
    fn kfd_unref_process(p: *mut kfd_process);
    fn atomic_read(p: *mut atomic_t) -> i32;
    fn atomic_set(p: *mut atomic_t, v: i32);
    fn kfd_signal_poison_consumed_event(node: *mut kfd_node, pasid: u16);
    fn kfd_signal_event_interrupt(pasid: u16, data: u32, bits: u32, is_sdma: bool);
    fn kfd_set_dbg_ev_from_interrupt(node: *mut kfd_node, pasid: u16, doorbell: i32, event: u32, data: *const core::ffi::c_void, size: usize) -> bool;
    fn kfd_dqm_suspend_bad_queue_mes(node: *mut kfd_node, pasid: u16, doorbell: u32);
    fn kfd_process_close_interrupt_drain(pasid: u16);
    fn kfd_irq_is_from_node(node: *mut kfd_node, node_id: u16, vmid: u16) -> bool;
    fn amdgpu_no_queue_eviction_on_vm_fault() -> bool;
    fn KFD_DBG_EC_TYPE_IS_PACKET(x: u32) -> bool;
    fn amdgpu_uniras_enabled(adev: *mut amdgpu_device) -> bool;
    fn amdgpu_ras_mgr_gen_ras_event_seqno(adev: *mut amdgpu_device, typ: u32) -> u64;
    fn amdgpu_amdkfd_ras_pasid_poison_consumption_handler(adev: *mut amdgpu_device, block: u32, pasid: u16, a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, reset: u32);
}

unsafe fn field(v: u32, mask: u32, shift: u32) -> u32 { (v & mask) >> shift }

unsafe fn print_sq_intr_info_auto(dev: *mut kfd_node, c0: u32, _c1: u32) {
    dev_dbg_ratelimited((*(*dev).adev).dev, b"sq_intr: auto, ttrace %d, wlt %d, ttrace_buf0_full %d, ttrace_buf1_full %d ttrace_utc_err %d\0".as_ptr(), field(c0,AUTO_THREAD_TRACE_MASK,AUTO_THREAD_TRACE_SHIFT), field(c0,AUTO_WLT_MASK,AUTO_WLT_SHIFT), field(c0,AUTO_BUF0_MASK,AUTO_BUF0_SHIFT), field(c0,AUTO_BUF1_MASK,AUTO_BUF1_SHIFT), field(c0,AUTO_UTC_MASK,AUTO_UTC_SHIFT));
}
unsafe fn print_sq_intr_info_inst(dev: *mut kfd_node, c0: u32, c1: u32) {
    dev_dbg_ratelimited((*(*dev).adev).dev, b"sq_intr: inst, data 0x%08x, sh %d, priv %d, wave_id %d, simd_id %d, wgp_id %d\0".as_ptr(), c0 & WAVE_DATA_MASK, field(c0,0x02000000,25), field(c0,0x04000000,26), field(c0,0xf8000000,27), c1 & 3, field(c1,0x3c,2));
}
unsafe fn print_sq_intr_info_error(dev: *mut kfd_node, c0: u32, c1: u32) {
    dev_warn_ratelimited((*(*dev).adev).dev, b"sq_intr: error, detail 0x%08x, type %d, sh %d, priv %d, wave_id %d, simd_id %d, wgp_id %d\0".as_ptr(), c0 & ERROR_DETAIL_MASK, field(c0,ERROR_TYPE_MASK,ERROR_TYPE_SHIFT), field(c0,0x02000000,25), field(c0,0x04000000,26), field(c0,0xf8000000,27), c1 & 3, field(c1,0x3c,2));
}

unsafe fn event_interrupt_poison_consumption_v12_1(node: *mut kfd_node, pasid: u16, source_id: u16) {
    let mut block = 0u32;
    let mut ret = -22i32;
    let mut reset = 0u32;
    let p = kfd_lookup_process_by_pasid(pasid, core::ptr::null_mut());
    if p.is_null() { return; }
    if atomic_read(&mut (*p).poison) != 0 { kfd_unref_process(p); return; }
    atomic_set(&mut (*p).poison, 1); kfd_unref_process(p);
    if source_id == 0x2b { // SOC15_INTSRC_SQ_INTERRUPT_MSG
        if let Some(f) = (*(*node).dqm).ops.reset_queues { ret = f((*node).dqm, pasid); }
        block = 1;
        if ret != 0 { reset = 2; }
    } else { block = 1; reset = 2; }
    kfd_signal_poison_consumed_event(node, pasid);
    let _event_id = if amdgpu_uniras_enabled((*node).adev) { amdgpu_ras_mgr_gen_ras_event_seqno((*node).adev, 0) } else { u64::MAX };
    amdgpu_amdkfd_ras_pasid_poison_consumption_handler((*node).adev, block, pasid, core::ptr::null_mut(), core::ptr::null_mut(), reset);
}

unsafe fn event_interrupt_isr_v12_1(node: *mut kfd_node, e: *const u32, _patched: *mut u32, _flag: *mut bool) -> bool {
    let client_id = ((*e.add(0) >> 8) & 0xff) as u16;
    let source_id = ((*e.add(0) >> 16) & 0xff) as u16;
    let vmid = ((*e.add(0) >> 24) & 0xff) as u16;
    let pasid = ((*e.add(1)) & 0xffff) as u16;
    let c0 = *e.add(2);
    let node_id = ((*e.add(0) >> 0) & 0xff) as u16;
    if !kfd_irq_is_from_node(node, node_id, vmid) { return false; }
    if !(client_id == 0xff || (vmid >= (*node).vm_info.first_vmid_kfd && vmid <= (*node).vm_info.last_vmid_kfd)) { return false; }
    if source_id == 0x04 && (c0 & 0x80000000) != 0 { return false; }
    if pasid == 0 { return false; }
    source_id == 0x04 || source_id == 0x2b || source_id == 0x07 || source_id == 0x2c || client_id == 0xff ||
        ((client_id == 0x0 || client_id == 0x1) && !amdgpu_no_queue_eviction_on_vm_fault())
}

unsafe fn event_interrupt_wq_v12_1(node: *mut kfd_node, e: *const u32) {
    let source_id = ((*e.add(0) >> 16) & 0xff) as u16;
    let client_id = ((*e.add(0) >> 8) & 0xff) as u16;
    let ring_id = ((*e.add(0) >> 0) & 0xff) as u16;
    let pasid = ((*e.add(1)) & 0xffff) as u16;
    let vmid = ((*e.add(0) >> 24) & 0xff) as u16;
    let c0 = *e.add(2); let c1 = *e.add(3);
    if client_id == 0 || client_id == 1 {
        let mut ex: kfd_hsa_memory_exception_data = core::mem::zeroed();
        ex.gpu_id = (*node).id; ex.va = ((*e.add(4) as u64) | (((*e.add(5) & 0xf) as u64) << 32)) << 12;
        ex.failure.NotPresent = if ring_id & 8 != 0 { 1 } else { 0 };
        kfd_set_dbg_ev_from_interrupt(node,pasid,-1,1,&ex as *const _ as *const _,core::mem::size_of_val(&ex));
    } else if client_id == 2 || client_id == 3 {
        if source_id == 4 { kfd_signal_event_interrupt(pasid,c0,32,false); }
        else if source_id == 7 && KFD_DBG_EC_TYPE_IS_PACKET(kfd_ctxid0_cp_bad_op_ecode(c0)) { kfd_set_dbg_ev_from_interrupt(node,pasid,kfd_ctxid0_doorbell_id(c0) as i32,kfd_ctxid0_cp_bad_op_ecode(c0),core::ptr::null(),0); kfd_dqm_suspend_bad_queue_mes(node,pasid,kfd_ctxid0_doorbell_id(c0)); }
        else if source_id == 0x2c { kfd_signal_event_interrupt(pasid,c0 & 0x0fffffff,28,true); }
        else if source_id == 0x2d { event_interrupt_poison_consumption_v12_1(node,pasid,source_id); return; }
        else if source_id == 0x2b {
            let enc = field(c1, WAVE_ENCODING_MASK, WAVE_ENCODING_SHIFT);
            if enc == 0 { print_sq_intr_info_auto(node,c0,c1); }
            else if enc == 1 { print_sq_intr_info_inst(node,c0,c1); if field(c0,0x04000000,WAVE_PRIV_SHIFT) != 0 && kfd_set_dbg_ev_from_interrupt(node,pasid,kfd_ctxid0_doorbell_id(c0) as i32,kfd_ctxid0_trap_code(c0),core::ptr::null(),0) { return; } }
            else if enc == 2 { print_sq_intr_info_error(node,c0,c1); let typ=field(c0,ERROR_TYPE_MASK,ERROR_TYPE_SHIFT); if typ != 1 && typ != 2 { event_interrupt_poison_consumption_v12_1(node,pasid,source_id); return; } }
            kfd_signal_event_interrupt(pasid,c0 & 0xffffff,24,true);
        }
    } else if client_id == 0xff { kfd_process_close_interrupt_drain(pasid); }
    let _ = vmid;
}

#[repr(C)] pub struct kfd_event_interrupt_class { pub interrupt_isr: unsafe fn(*mut kfd_node,*const u32,*mut u32,*mut bool)->bool, pub interrupt_wq: unsafe fn(*mut kfd_node,*const u32) }
pub static event_interrupt_class_v12_1: kfd_event_interrupt_class = kfd_event_interrupt_class { interrupt_isr: event_interrupt_isr_v12_1, interrupt_wq: event_interrupt_wq_v12_1 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
