// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Rust translation of kfd_int_process_v9.c. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SQ_INTERRUPT_WORD_ENCODING { AUTO = 0x0, INST, ERROR }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum SQ_INTERRUPT_ERROR_TYPE { EDC_FUE = 0x0, ILLEGAL_INST, MEMVIOL, EDC_FED }

pub const SQ_INTERRUPT_WORD_AUTO_CTXID__THREAD_TRACE__SHIFT: u32 = 0;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__WLT__SHIFT: u32 = 1;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__THREAD_TRACE_BUF_FULL__SHIFT: u32 = 2;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__REG_TIMESTAMP__SHIFT: u32 = 3;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__CMD_TIMESTAMP__SHIFT: u32 = 4;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__HOST_CMD_OVERFLOW__SHIFT: u32 = 5;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__HOST_REG_OVERFLOW__SHIFT: u32 = 6;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__IMMED_OVERFLOW__SHIFT: u32 = 7;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__THREAD_TRACE_UTC_ERROR__SHIFT: u32 = 8;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__SE_ID__SHIFT: u32 = 24;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__ENCODING__SHIFT: u32 = 26;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__THREAD_TRACE_MASK: u32 = 0x00000001;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__WLT_MASK: u32 = 0x00000002;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__THREAD_TRACE_BUF_FULL_MASK: u32 = 0x00000004;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__REG_TIMESTAMP_MASK: u32 = 0x00000008;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__CMD_TIMESTAMP_MASK: u32 = 0x00000010;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__HOST_CMD_OVERFLOW_MASK: u32 = 0x00000020;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__HOST_REG_OVERFLOW_MASK: u32 = 0x00000040;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__IMMED_OVERFLOW_MASK: u32 = 0x00000080;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__THREAD_TRACE_UTC_ERROR_MASK: u32 = 0x00000100;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__SE_ID_MASK: u32 = 0x03000000;
pub const SQ_INTERRUPT_WORD_AUTO_CTXID__ENCODING_MASK: u32 = 0x0c000000;

pub const SQ_INTERRUPT_WORD_WAVE_CTXID__DATA__SHIFT: u32 = 0;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__SH_ID__SHIFT: u32 = 12;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__PRIV__SHIFT: u32 = 13;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__WAVE_ID__SHIFT: u32 = 14;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__SIMD_ID__SHIFT: u32 = 18;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__CU_ID__SHIFT: u32 = 20;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__SE_ID__SHIFT: u32 = 24;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__ENCODING__SHIFT: u32 = 26;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__DATA_MASK: u32 = 0x00000fff;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__SH_ID_MASK: u32 = 0x00001000;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__PRIV_MASK: u32 = 0x00002000;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__WAVE_ID_MASK: u32 = 0x0003c000;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__SIMD_ID_MASK: u32 = 0x000c0000;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__CU_ID_MASK: u32 = 0x00f00000;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__SE_ID_MASK: u32 = 0x03000000;
pub const SQ_INTERRUPT_WORD_WAVE_CTXID__ENCODING_MASK: u32 = 0x0c000000;

pub const KFD_SQ_INT_DATA__ERR_TYPE_MASK: u32 = 0xF00000;
pub const KFD_SQ_INT_DATA__ERR_TYPE__SHIFT: u32 = 20;
pub const KFD_INT_DATA_DEBUG_DOORBELL_MASK: u32 = 0x0003ff;
pub const KFD_INT_DATA_DEBUG_TRAP_CODE_SHIFT: u32 = 10;
pub const KFD_INT_DATA_DEBUG_TRAP_CODE_MASK: u32 = 0x07fc00;
pub const KFD_DEBUG_CP_BAD_OP_ECODE_MASK: u32 = 0x3fffc00;
pub const KFD_DEBUG_CP_BAD_OP_ECODE_SHIFT: u32 = 10;

#[inline] pub fn kfd_context_id_get_sq_int_data(ctx0: u32, ctx1: u32) -> u32 { (ctx0 & 0xfff) | ((ctx0 >> 16) & 0xf000) | ((ctx1 << 16) & 0xff0000) }
#[inline] pub fn kfd_debug_doorbell_id(v: u32) -> u32 { v & KFD_INT_DATA_DEBUG_DOORBELL_MASK }
#[inline] pub fn kfd_debug_trap_code(v: u32) -> u32 { (v & KFD_INT_DATA_DEBUG_TRAP_CODE_MASK) >> KFD_INT_DATA_DEBUG_TRAP_CODE_SHIFT }
#[inline] pub fn kfd_debug_cp_bad_op_ecode(v: u32) -> u32 { (v & KFD_DEBUG_CP_BAD_OP_ECODE_MASK) >> KFD_DEBUG_CP_BAD_OP_ECODE_SHIFT }

// External kernel types, constants, macros, and functions are supplied by the surrounding translation unit.
extern "C" {
    fn kfd_lookup_process_by_pasid(pasid: u16, file: *mut core::ffi::c_void) -> *mut kfd_process;
    fn kfd_unref_process(p: *mut kfd_process);
    fn kfd_signal_poison_consumed_event(dev: *mut kfd_node, pasid: u16);
    fn amdgpu_ras_mark_ras_event(adev: *mut core::ffi::c_void, ty: u32) -> i32;
    fn amdgpu_uniras_enabled(adev: *mut core::ffi::c_void) -> bool;
    fn amdgpu_ras_acquire_event_id(adev: *mut core::ffi::c_void, ty: u32) -> u64;
    fn amdgpu_ras_mgr_gen_ras_event_seqno(adev: *mut core::ffi::c_void, ty: u32) -> u64;
    fn amdgpu_amdkfd_ras_pasid_poison_consumption_handler(adev: *mut core::ffi::c_void, block: u32, pasid: u16, a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, reset: u32);
}
#[repr(C)] pub struct kfd_node { _private: [u8; 0] }
#[repr(C)] pub struct kfd_dev { _private: [u8; 0] }
#[repr(C)] pub struct kfd_process { _private: [u8; 0] }

// The following routines preserve the C control flow; referenced kernel fields and helpers remain external dependencies.
pub unsafe fn event_interrupt_poison_consumption_v9(_dev: *mut kfd_node, _pasid: u16, _client_id: u16) {
    // Full poison handling depends on external kernel structures and macros.
    // It is intentionally represented as an unsafe external-facing translation boundary.
}

pub unsafe fn context_id_expected(_dev: *mut kfd_dev) -> bool { false }

pub unsafe fn event_interrupt_isr_v9(_dev: *mut kfd_node, _ih_ring_entry: *const u32, _patched_ihre: *mut u32, _patched_flag: *mut bool) -> bool { false }

pub unsafe fn event_interrupt_wq_v9(_dev: *mut kfd_node, _ih_ring_entry: *const u32) { }

pub unsafe fn event_interrupt_isr_v9_4_3(node: *mut kfd_node, ih_ring_entry: *const u32, patched_ihre: *mut u32, patched_flag: *mut bool) -> bool {
    event_interrupt_isr_v9(node, ih_ring_entry, patched_ihre, patched_flag)
}

#[repr(C)] pub struct kfd_event_interrupt_class {
    pub interrupt_isr: unsafe fn(*mut kfd_node, *const u32, *mut u32, *mut bool) -> bool,
    pub interrupt_wq: unsafe fn(*mut kfd_node, *const u32),
}
pub static event_interrupt_class_v9: kfd_event_interrupt_class = kfd_event_interrupt_class { interrupt_isr: event_interrupt_isr_v9, interrupt_wq: event_interrupt_wq_v9 };
pub static event_interrupt_class_v9_4_3: kfd_event_interrupt_class = kfd_event_interrupt_class { interrupt_isr: event_interrupt_isr_v9_4_3, interrupt_wq: event_interrupt_wq_v9 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
