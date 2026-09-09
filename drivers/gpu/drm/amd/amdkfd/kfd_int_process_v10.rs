/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation.

const SQ_INTERRUPT_WORD_ENCODING_AUTO: u32 = 0x0;
const SQ_INTERRUPT_WORD_ENCODING_INST: u32 = 0x1;
const SQ_INTERRUPT_WORD_ENCODING_ERROR: u32 = 0x2;

const SQ_INTERRUPT_WORD_AUTO_CTXID0__THREAD_TRACE__SHIFT: u32 = 0;
const SQ_INTERRUPT_WORD_AUTO_CTXID0__WLT__SHIFT: u32 = 1;
const SQ_INTERRUPT_WORD_AUTO_CTXID0__THREAD_TRACE_BUF0_FULL__SHIFT: u32 = 2;
const SQ_INTERRUPT_WORD_AUTO_CTXID0__THREAD_TRACE_BUF1_FULL__SHIFT: u32 = 3;
const SQ_INTERRUPT_WORD_AUTO_CTXID0__THREAD_TRACE_UTC_ERROR__SHIFT: u32 = 7;
const SQ_INTERRUPT_WORD_AUTO_CTXID1__SE_ID__SHIFT: u32 = 4;
const SQ_INTERRUPT_WORD_AUTO_CTXID1__ENCODING__SHIFT: u32 = 6;
const SQ_INTERRUPT_WORD_AUTO_CTXID0__THREAD_TRACE_MASK: u32 = 0x00000001;
const SQ_INTERRUPT_WORD_AUTO_CTXID0__WLT_MASK: u32 = 0x00000002;
const SQ_INTERRUPT_WORD_AUTO_CTXID0__THREAD_TRACE_BUF0_FULL_MASK: u32 = 0x00000004;
const SQ_INTERRUPT_WORD_AUTO_CTXID0__THREAD_TRACE_BUF1_FULL_MASK: u32 = 0x00000008;
const SQ_INTERRUPT_WORD_AUTO_CTXID0__THREAD_TRACE_UTC_ERROR_MASK: u32 = 0x00000080;
const SQ_INTERRUPT_WORD_AUTO_CTXID1__SE_ID_MASK: u32 = 0x030;
const SQ_INTERRUPT_WORD_AUTO_CTXID1__ENCODING_MASK: u32 = 0x0c0;

const SQ_INTERRUPT_WORD_WAVE_CTXID0__DATA__SHIFT: u32 = 0;
const SQ_INTERRUPT_WORD_WAVE_CTXID0__SA_ID__SHIFT: u32 = 23;
const SQ_INTERRUPT_WORD_WAVE_CTXID0__PRIV__SHIFT: u32 = 24;
const SQ_INTERRUPT_WORD_WAVE_CTXID0__WAVE_ID__SHIFT: u32 = 25;
const SQ_INTERRUPT_WORD_WAVE_CTXID0__SIMD_ID__SHIFT: u32 = 30;
const SQ_INTERRUPT_WORD_WAVE_CTXID1__WGP_ID__SHIFT: u32 = 0;
const SQ_INTERRUPT_WORD_WAVE_CTXID1__SE_ID__SHIFT: u32 = 4;
const SQ_INTERRUPT_WORD_WAVE_CTXID1__ENCODING__SHIFT: u32 = 6;
const SQ_INTERRUPT_WORD_WAVE_CTXID0__DATA_MASK: u32 = 0x000007fffff;
const SQ_INTERRUPT_WORD_WAVE_CTXID0__SA_ID_MASK: u32 = 0x0000800000;
const SQ_INTERRUPT_WORD_WAVE_CTXID0__PRIV_MASK: u32 = 0x00001000000;
const SQ_INTERRUPT_WORD_WAVE_CTXID0__WAVE_ID_MASK: u32 = 0x0003e000000;
const SQ_INTERRUPT_WORD_WAVE_CTXID0__SIMD_ID_MASK: u32 = 0x000c0000000;
const SQ_INTERRUPT_WORD_WAVE_CTXID1__WGP_ID_MASK: u32 = 0x00f;
const SQ_INTERRUPT_WORD_WAVE_CTXID1__SE_ID_MASK: u32 = 0x030;
const SQ_INTERRUPT_WORD_WAVE_CTXID1__ENCODING_MASK: u32 = 0x0c0;
const KFD_CTXID0__ERR_TYPE_MASK: u32 = 0x780000;
const KFD_CTXID0__ERR_TYPE__SHIFT: u32 = 19;
const KFD_CONTEXT_ID1_ENC_TYPE_WAVE_MASK: u32 = 0x40;
const KFD_CONTEXT_ID0_PRIV_MASK: u32 = 0x1000000;
const KFD_CONTEXT_ID0_DEBUG_DOORBELL_MASK: u32 = 0x0003ff;
const KFD_CONTEXT_ID0_DEBUG_TRAP_CODE_SHIFT: u32 = 10;
const KFD_CONTEXT_ID0_DEBUG_TRAP_CODE_MASK: u32 = 0x07fc00;
const KFD_DEBUG_CP_BAD_OP_ECODE_MASK: u32 = 0x3fffc00;
const KFD_DEBUG_CP_BAD_OP_ECODE_SHIFT: u32 = 10;

#[inline]
fn kfd_debug_doorbell_id(ctxid0: u32) -> u32 { ctxid0 & KFD_CONTEXT_ID0_DEBUG_DOORBELL_MASK }
#[inline]
fn kfd_debug_trap_code(ctxid0: u32) -> u32 { (ctxid0 & KFD_CONTEXT_ID0_DEBUG_TRAP_CODE_MASK) >> KFD_CONTEXT_ID0_DEBUG_TRAP_CODE_SHIFT }
#[inline]
fn kfd_debug_cp_bad_op_ecode(ctxid0: u32) -> u32 { (ctxid0 & KFD_DEBUG_CP_BAD_OP_ECODE_MASK) >> KFD_DEBUG_CP_BAD_OP_ECODE_SHIFT }

// The following functions and types are external declarations from included kernel headers.
extern "C" {
    fn event_interrupt_isr_v10(dev: *mut kfd_node, ih_ring_entry: *const u32, patched_ihre: *mut u32, patched_flag: *mut bool) -> bool;
}

// Literal translation of the implementation; field extraction/logging helpers are supplied by dependencies.
unsafe fn event_interrupt_isr_v10_impl(dev: *mut kfd_node, ih_ring_entry: *const u32, _patched_ihre: *mut u32, _patched_flag: *mut bool) -> bool {
    let source_id = SOC15_SOURCE_ID_FROM_IH_ENTRY(ih_ring_entry);
    let client_id = SOC15_CLIENT_ID_FROM_IH_ENTRY(ih_ring_entry);
    let vmid = SOC15_VMID_FROM_IH_ENTRY(ih_ring_entry);
    if !KFD_IRQ_IS_FENCE(client_id, source_id) && (vmid < (*dev).vm_info.first_vmid_kfd || vmid > (*dev).vm_info.last_vmid_kfd) { return false; }
    let pasid = SOC15_PASID_FROM_IH_ENTRY(ih_ring_entry);
    if client_id != SOC15_IH_CLIENTID_GRBM_CP && client_id != SOC15_IH_CLIENTID_SDMA0 && client_id != SOC15_IH_CLIENTID_SDMA1 && client_id != SOC15_IH_CLIENTID_SDMA2 && client_id != SOC15_IH_CLIENTID_SDMA3 && client_id != SOC15_IH_CLIENTID_SDMA4 && client_id != SOC15_IH_CLIENTID_SDMA5 && client_id != SOC15_IH_CLIENTID_SDMA6 && client_id != SOC15_IH_CLIENTID_SDMA7 && client_id != SOC15_IH_CLIENTID_VMC && client_id != SOC15_IH_CLIENTID_VMC1 && client_id != SOC15_IH_CLIENTID_UTCL2 && client_id != SOC15_IH_CLIENTID_SE0SH && client_id != SOC15_IH_CLIENTID_SE1SH && client_id != SOC15_IH_CLIENTID_SE2SH && client_id != SOC15_IH_CLIENTID_SE3SH { return false; }
    let data = ih_ring_entry;
    dev_dbg((*dev).adev.dev, "client id 0x%x, source id %d, vmid %d, pasid 0x%x. raw data:\n", client_id, source_id, vmid, pasid);
    dev_dbg((*dev).adev.dev, "%8X, %8X, %8X, %8X, %8X, %8X, %8X, %8X.\n", *data.add(0), *data.add(1), *data.add(2), *data.add(3), *data.add(4), *data.add(5), *data.add(6), *data.add(7));
    if pasid == 0 { return false; }
    source_id == SOC15_INTSRC_CP_END_OF_PIPE || source_id == SOC15_INTSRC_SDMA_TRAP || source_id == SOC15_INTSRC_SQ_INTERRUPT_MSG || source_id == SOC15_INTSRC_CP_BAD_OPCODE || client_id == SOC15_IH_CLIENTID_VMC || client_id == SOC15_IH_CLIENTID_VMC1 || client_id == SOC15_IH_CLIENTID_UTCL2 || KFD_IRQ_IS_FENCE(client_id, source_id)
}

// The workqueue implementation below retains the original dispatch and data operations.
unsafe fn event_interrupt_wq_v10(dev: *mut kfd_node, ih_ring_entry: *const u32) {
    let source_id = SOC15_SOURCE_ID_FROM_IH_ENTRY(ih_ring_entry);
    let client_id = SOC15_CLIENT_ID_FROM_IH_ENTRY(ih_ring_entry);
    let pasid = SOC15_PASID_FROM_IH_ENTRY(ih_ring_entry);
    let vmid = SOC15_VMID_FROM_IH_ENTRY(ih_ring_entry);
    let context_id0 = SOC15_CONTEXT_ID0_FROM_IH_ENTRY(ih_ring_entry);
    let context_id1 = SOC15_CONTEXT_ID1_FROM_IH_ENTRY(ih_ring_entry);
    if (client_id == SOC15_IH_CLIENTID_GRBM_CP || client_id == SOC15_IH_CLIENTID_SE0SH || client_id == SOC15_IH_CLIENTID_SE1SH || client_id == SOC15_IH_CLIENTID_SE2SH || client_id == SOC15_IH_CLIENTID_SE3SH) && source_id == SOC15_INTSRC_CP_END_OF_PIPE { kfd_signal_event_interrupt(pasid, context_id0, 32, true); }
    else if (client_id == SOC15_IH_CLIENTID_GRBM_CP || client_id == SOC15_IH_CLIENTID_SE0SH || client_id == SOC15_IH_CLIENTID_SE1SH || client_id == SOC15_IH_CLIENTID_SE2SH || client_id == SOC15_IH_CLIENTID_SE3SH) && source_id == SOC15_INTSRC_SQ_INTERRUPT_MSG {
        let encoding = REG_GET_FIELD(context_id1, SQ_INTERRUPT_WORD_WAVE_CTXID1, ENCODING);
        match encoding {
            SQ_INTERRUPT_WORD_ENCODING_AUTO => { dev_dbg_ratelimited((*dev).adev.dev, "sq_intr: auto"); }
            SQ_INTERRUPT_WORD_ENCODING_INST => { dev_dbg_ratelimited((*dev).adev.dev, "sq_intr: inst"); if context_id0 & SQ_INTERRUPT_WORD_WAVE_CTXID0__PRIV_MASK != 0 { if kfd_set_dbg_ev_from_interrupt(dev, pasid, kfd_debug_doorbell_id(context_id0), kfd_debug_trap_code(context_id0), core::ptr::null(), 0) { return; } } }
            SQ_INTERRUPT_WORD_ENCODING_ERROR => { let sq_intr_err_type = REG_GET_FIELD(context_id0, KFD_CTXID0, ERR_TYPE); dev_warn_ratelimited((*dev).adev.dev, "sq_intr: error, err_type %d", sq_intr_err_type); }
            _ => {}
        }
        kfd_signal_event_interrupt(pasid, context_id0 & 0x7fffff, 23, true);
    }
    else if (client_id == SOC15_IH_CLIENTID_SDMA0 || client_id == SOC15_IH_CLIENTID_SDMA1 || client_id == SOC15_IH_CLIENTID_SDMA2 || client_id == SOC15_IH_CLIENTID_SDMA3 || client_id == SOC15_IH_CLIENTID_SDMA4 || client_id == SOC15_IH_CLIENTID_SDMA5 || client_id == SOC15_IH_CLIENTID_SDMA6 || client_id == SOC15_IH_CLIENTID_SDMA7) && source_id == SOC15_INTSRC_SDMA_TRAP { kfd_signal_event_interrupt(pasid, context_id0 & 0xfffffff, 28, true); }
    else if (client_id == SOC15_IH_CLIENTID_VMC || client_id == SOC15_IH_CLIENTID_VMC1 || client_id == SOC15_IH_CLIENTID_UTCL2) { let mut info = kfd_vm_fault_info::default(); info.vmid = vmid; info.mc_id = client_id; info.page_addr = *ih_ring_entry.add(4) as u64 | ((*ih_ring_entry.add(5) & 0xf) as u64) << 32; let ring_id = SOC15_RING_ID_FROM_IH_ENTRY(ih_ring_entry); info.prot_valid = ring_id & 8; info.prot_read = ring_id & 0x10; info.prot_write = ring_id & 0x20; let mut exception_data = kfd_hsa_memory_exception_data::default(); exception_data.gpu_id = (*dev).id; exception_data.va = info.page_addr << PAGE_SHIFT; exception_data.failure.NotPresent = if info.prot_valid != 0 { 1 } else { 0 }; exception_data.failure.NoExecute = if info.prot_exec != 0 { 1 } else { 0 }; exception_data.failure.ReadOnly = if info.prot_write != 0 { 1 } else { 0 }; kfd_set_dbg_ev_from_interrupt(dev, pasid, -1, KFD_EC_MASK(EC_DEVICE_MEMORY_VIOLATION), &mut exception_data, core::mem::size_of::<kfd_hsa_memory_exception_data>()); }
    else if KFD_IRQ_IS_FENCE(client_id, source_id) { kfd_process_close_interrupt_drain(pasid); }
}

const event_interrupt_class_v10: kfd_event_interrupt_class = kfd_event_interrupt_class { interrupt_isr: event_interrupt_isr_v10_impl, interrupt_wq: event_interrupt_wq_v10 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
