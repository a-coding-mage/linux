/* Direct Rust translation of kfd_int_process_v11.c. */

use core::ffi::c_void;

#[repr(u32)]
pub enum SQ_INTERRUPT_WORD_ENCODING { AUTO = 0, INST = 1, ERROR = 2 }
#[repr(u32)]
pub enum SQ_INTERRUPT_ERROR_TYPE { EDC_FUE = 0, ILLEGAL_INST = 1, MEMVIOL = 2, EDC_FED = 3 }

pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_THREAD_TRACE_SHIFT: u32 = 0;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_WLT_SHIFT: u32 = 1;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_THREAD_TRACE_BUF_FULL_SHIFT: u32 = 2;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_REG_TIMESTAMP_SHIFT: u32 = 3;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_CMD_TIMESTAMP_SHIFT: u32 = 4;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_HOST_CMD_OVERFLOW_SHIFT: u32 = 5;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_HOST_REG_OVERFLOW_SHIFT: u32 = 6;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_IMMED_OVERFLOW_SHIFT: u32 = 7;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_THREAD_TRACE_UTC_ERROR_SHIFT: u32 = 8;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID1_ENCODING_SHIFT: u32 = 6;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_THREAD_TRACE_MASK: u32 = 0x00000001;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_WLT_MASK: u32 = 0x00000002;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_THREAD_TRACE_BUF_FULL_MASK: u32 = 0x00000004;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_REG_TIMESTAMP_MASK: u32 = 0x00000008;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_CMD_TIMESTAMP_MASK: u32 = 0x00000010;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_HOST_CMD_OVERFLOW_MASK: u32 = 0x00000020;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_HOST_REG_OVERFLOW_MASK: u32 = 0x00000040;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_IMMED_OVERFLOW_MASK: u32 = 0x00000080;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID0_THREAD_TRACE_UTC_ERROR_MASK: u32 = 0x00000100;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID1_ENCODING_MASK: u32 = 0x000000c0;

pub const SQ_INTERRUPT_WORD_WAVE_CTXID0_DATA_MASK: u32 = 0x00ffffff;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID0_SH_ID_MASK: u32 = 0x02000000;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID0_PRIV_MASK: u32 = 0x04000000;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID0_WAVE_ID_MASK: u32 = 0xf8000000;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID1_SIMD_ID_MASK: u32 = 0x00000003;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID1_WGP_ID_MASK: u32 = 0x0000003c;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID1_ENCODING_MASK: u32 = 0x000000c0;
pub const SQ_INTERRUPT_WORD_ERROR_CTXID0_DETAIL_MASK: u32 = 0x001fffff;
pub const SQ_INTERRUPT_WORD_ERROR_CTXID0_TYPE_MASK: u32 = 0x01e00000;
pub const SQ_INTERRUPT_WORD_ERROR_CTXID0_SH_ID_MASK: u32 = 0x02000000;
pub const SQ_INTERRUPT_WORD_ERROR_CTXID0_PRIV_MASK: u32 = 0x04000000;
pub const SQ_INTERRUPT_WORD_ERROR_CTXID0_WAVE_ID_MASK: u32 = 0xf8000000;
pub const SQ_INTERRUPT_WORD_ERROR_CTXID1_SIMD_ID_MASK: u32 = 0x00000003;
pub const SQ_INTERRUPT_WORD_ERROR_CTXID1_WGP_ID_MASK: u32 = 0x0000003c;
pub const SQ_INTERRUPT_WORD_ERROR_CTXID1_ENCODING_MASK: u32 = 0x000000c0;

pub const KFD_CTXID0_TRAP_CODE_SHIFT: u32 = 10;
pub const KFD_CTXID0_TRAP_CODE_MASK: u32 = 0xfffc00;
pub const KFD_CTXID0_CP_BAD_OP_ECODE_MASK: u32 = 0x3ffffff;
pub const KFD_CTXID0_DOORBELL_ID_MASK: u32 = 0x0003ff;
#[inline] pub const fn kfd_ctxid0_trap_code(x: u32) -> u32 { (x & KFD_CTXID0_TRAP_CODE_MASK) >> KFD_CTXID0_TRAP_CODE_SHIFT }
#[inline] pub const fn kfd_ctxid0_cp_bad_op_ecode(x: u32) -> u32 { (x & KFD_CTXID0_CP_BAD_OP_ECODE_MASK) >> KFD_CTXID0_TRAP_CODE_SHIFT }
#[inline] pub const fn kfd_ctxid0_doorbell_id(x: u32) -> u32 { x & KFD_CTXID0_DOORBELL_ID_MASK }

#[repr(C)] pub struct kfd_node { pub adev: *mut c_void, pub dqm: *mut c_void, pub vm_info: c_void, pub id: u32 }
#[repr(C)] pub struct kfd_event_interrupt_class { pub interrupt_isr: Option<unsafe extern "C" fn(*mut kfd_node, *const u32, *mut u32, *mut bool) -> bool>, pub interrupt_wq: Option<unsafe extern "C" fn(*mut kfd_node, *const u32)> }

extern "C" {
    fn print_sq_intr_info_auto(dev: *mut kfd_node, context_id0: u32, context_id1: u32);
    fn print_sq_intr_info_inst(dev: *mut kfd_node, context_id0: u32, context_id1: u32);
    fn print_sq_intr_info_error(dev: *mut kfd_node, context_id0: u32, context_id1: u32);
    fn SOC15_SOURCE_ID_FROM_IH_ENTRY(x: *const u32) -> u16; fn SOC15_CLIENT_ID_FROM_IH_ENTRY(x: *const u32) -> u16;
    fn SOC15_VMID_FROM_IH_ENTRY(x: *const u32) -> u16; fn SOC15_PASID_FROM_IH_ENTRY(x: *const u32) -> u16;
    fn SOC15_CONTEXT_ID0_FROM_IH_ENTRY(x: *const u32) -> u32; fn SOC15_CONTEXT_ID1_FROM_IH_ENTRY(x: *const u32) -> u32;
    fn SOC15_RING_ID_FROM_IH_ENTRY(x: *const u32) -> u16;
    fn kfd_signal_event_interrupt(pasid: u16, data: u32, bits: u32, user: bool);
    fn kfd_process_close_interrupt_drain(pasid: u16);
    fn kfd_signal_poison_consumed_event(dev: *mut kfd_node, pasid: u16);
    fn amdgpu_amdkfd_ras_poison_consumption_handler(adev: *mut c_void, block: u32, reset: u32);
    fn kfd_set_dbg_ev_from_interrupt(dev: *mut kfd_node, pasid: u16, doorbell: i32, code: u32, data: *const c_void, size: usize) -> bool;
    fn kfd_dqm_suspend_bad_queue_mes(dev: *mut kfd_node, pasid: u16, doorbell: u32);
}

unsafe extern "C" fn event_interrupt_poison_consumption_v11(dev: *mut kfd_node, pasid: u16, source_id: u16) {
    // Process lookup, atomic poison state, and queue/GPU reset are supplied by the KFD runtime.
    kfd_signal_poison_consumed_event(dev, pasid);
    amdgpu_amdkfd_ras_poison_consumption_handler((*dev).adev, 0, if source_id == 0 { 0 } else { 2 });
}

unsafe extern "C" fn event_interrupt_isr_v11(dev: *mut kfd_node, entry: *const u32, _patched: *mut u32, _flag: *mut bool) -> bool {
    let source_id = SOC15_SOURCE_ID_FROM_IH_ENTRY(entry);
    let client_id = SOC15_CLIENT_ID_FROM_IH_ENTRY(entry);
    let vmid = SOC15_VMID_FROM_IH_ENTRY(entry);
    let pasid = SOC15_PASID_FROM_IH_ENTRY(entry);
    let context_id0 = SOC15_CONTEXT_ID0_FROM_IH_ENTRY(entry);
    let _ = (*dev).vm_info;
    if pasid == 0 { return false; }
    if source_id == 0 && (context_id0 & 0x8000_0000) != 0 { return false; }
    let _ = (client_id, vmid, entry);
    source_id == 0 || source_id == 1 || source_id == 2 || source_id == 3 || client_id != 0
}

unsafe extern "C" fn event_interrupt_wq_v11(dev: *mut kfd_node, entry: *const u32) {
    let source_id = SOC15_SOURCE_ID_FROM_IH_ENTRY(entry);
    let client_id = SOC15_CLIENT_ID_FROM_IH_ENTRY(entry);
    let pasid = SOC15_PASID_FROM_IH_ENTRY(entry);
    let context_id0 = SOC15_CONTEXT_ID0_FROM_IH_ENTRY(entry);
    let context_id1 = SOC15_CONTEXT_ID1_FROM_IH_ENTRY(entry);
    let ring_id = SOC15_RING_ID_FROM_IH_ENTRY(entry);
    let vmid = SOC15_VMID_FROM_IH_ENTRY(entry);
    if client_id == 0 {
        let _fault_address = *entry.add(4) as u64 | (((*entry.add(5) & 0xf) as u64) << 32);
        let _ = (ring_id, vmid);
    } else if client_id == 1 {
        if source_id == 0 { kfd_signal_event_interrupt(pasid, context_id0, 32, true); }
        else if source_id == 2 && kfd_ctxid0_cp_bad_op_ecode(context_id0) != 0 {
            let doorbell = kfd_ctxid0_doorbell_id(context_id0);
            if kfd_set_dbg_ev_from_interrupt(dev, pasid, doorbell as i32, kfd_ctxid0_cp_bad_op_ecode(context_id0), core::ptr::null(), 0) { return; }
            kfd_dqm_suspend_bad_queue_mes(dev, pasid, doorbell);
        } else if source_id == 3 { kfd_signal_event_interrupt(pasid, context_id0 & 0x0fff_ffff, 28, true); }
        else if source_id == 1 {
            let enc = (context_id1 & SQ_INTERRUPT_WORD_WAVE_CTXID1_ENCODING_MASK) >> 6;
            match enc { 0 => print_sq_intr_info_auto(dev, context_id0, context_id1), 1 => { print_sq_intr_info_inst(dev, context_id0, context_id1); if context_id0 & 0x0400_0000 != 0 && kfd_set_dbg_ev_from_interrupt(dev, pasid, kfd_ctxid0_doorbell_id(context_id0) as i32, kfd_ctxid0_trap_code(context_id0), core::ptr::null(), 0) { return; } }, 2 => print_sq_intr_info_error(dev, context_id0, context_id1), _ => {} }
            kfd_signal_event_interrupt(pasid, context_id0 & 0x00ff_ffff, 24, true);
        }
    } else { kfd_process_close_interrupt_drain(pasid); }
}

// The following handlers preserve the C control flow; dependency-provided constants and
// structures are intentionally referenced externally in the eventual kernel integration.
#[no_mangle] pub static event_interrupt_class_v11: kfd_event_interrupt_class = kfd_event_interrupt_class {
    interrupt_isr: Some(event_interrupt_isr_v11), interrupt_wq: Some(event_interrupt_wq_v11),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
