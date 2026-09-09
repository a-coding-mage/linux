/* SPDX-License-Identifier: GPL-2.0
 * Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub const OTX_CPT_MAX_SG_IN_CNT: usize = 50;
pub const OTX_CPT_MAX_SG_OUT_CNT: usize = 50;
pub const OTX_CPT_DMA_DIRECT_DIRECT: u32 = 0;
pub const OTX_CPT_DMA_GATHER_SCATTER: u32 = 1;
pub const OTX_CPT_FROM_CPTR: u32 = 0;
pub const OTX_CPT_FROM_DPTR: u32 = 1;
pub const OTX_CPT_INST_Q_ALIGNMENT: u32 = 128;
pub const OTX_CPT_MAX_REQ_SIZE: u32 = 65535;
pub const OTX_CPT_COMMAND_TIMEOUT: u32 = 4;
pub const OTX_CPT_TIMER_HOLD: u32 = 0x03F;
pub const OTX_CPT_COUNT_HOLD: u32 = 32;
pub const OTX_CPT_TIME_IN_RESET_COUNT: u32 = 5;
pub const OTX_CPT_COALESC_MIN_TIME_WAIT: u32 = 0x0;
pub const OTX_CPT_COALESC_MAX_TIME_WAIT: u32 = (1 << 16) - 1;
pub const OTX_CPT_COALESC_MIN_NUM_WAIT: u32 = 0x0;
pub const OTX_CPT_COALESC_MAX_NUM_WAIT: u32 = (1 << 20) - 1;

#[repr(C)]
pub union otx_cpt_opcode_info {
    pub flags: u16,
    pub s: otx_cpt_opcode_info_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_opcode_info_s { pub major: u8, pub minor: u8 }

#[repr(C)]
pub struct otx_cptvf_request {
    pub param1: u32, pub param2: u32, pub dlen: u16,
    pub opcode: otx_cpt_opcode_info,
}

#[repr(C)]
pub struct otx_cpt_buf_ptr {
    pub vptr: *mut u8, pub dma_addr: dma_addr_t, pub size: u16,
}

#[repr(C)]
pub union otx_cpt_ctrl_info { pub flags: u32, pub s: otx_cpt_ctrl_info_s }
#[repr(C)]
pub struct otx_cpt_ctrl_info_s {
    pub se_req: u32, pub dma_mode: u32, pub grp: u32, pub reserved0: u32,
}

#[repr(C)]
pub union otx_cpt_iq_cmd_word0 { pub u64: u64, pub s: otx_cpt_iq_cmd_word0_s }
#[repr(C)]
pub struct otx_cpt_iq_cmd_word0_s {
    pub opcode: __be16, pub param1: __be16, pub param2: __be16, pub dlen: __be16,
}

#[repr(C)]
pub union otx_cpt_iq_cmd_word3 { pub u64: u64, pub s: otx_cpt_iq_cmd_word3_s }
#[repr(C)]
pub struct otx_cpt_iq_cmd_word3_s { pub cptr: u64, pub grp: u64 }

#[repr(C)]
pub struct otx_cpt_iq_cmd {
    pub cmd: otx_cpt_iq_cmd_word0, pub dptr: u64, pub rptr: u64,
    pub cptr: otx_cpt_iq_cmd_word3,
}

#[repr(C)]
pub struct otx_cpt_sglist_component {
    pub u: otx_cpt_sglist_component_u,
    pub ptr0: __be64, pub ptr1: __be64, pub ptr2: __be64, pub ptr3: __be64,
}
#[repr(C)]
pub union otx_cpt_sglist_component_u { pub len: u64, pub s: otx_cpt_sglist_component_s }
#[repr(C)]
pub struct otx_cpt_sglist_component_s {
    pub len0: __be16, pub len1: __be16, pub len2: __be16, pub len3: __be16,
}

#[repr(C)]
pub struct otx_cpt_pending_entry {
    pub completion_addr: *mut u64,
    pub info: *mut otx_cpt_info_buffer,
    pub callback: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void, *mut core::ffi::c_void)>,
    pub areq: *mut crypto_async_request,
    pub resume_sender: u8, pub busy: u8,
}

#[repr(C)]
pub struct otx_cpt_pending_queue {
    pub head: *mut otx_cpt_pending_entry, pub front: u32, pub rear: u32,
    pub pending_count: u32, pub qlen: u32, pub lock: spinlock_t,
}

#[repr(C)]
pub struct otx_cpt_req_info {
    pub callback: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void, *mut core::ffi::c_void)>,
    pub areq: *mut crypto_async_request, pub req: otx_cptvf_request,
    pub ctrl: otx_cpt_ctrl_info,
    pub in_: [otx_cpt_buf_ptr; OTX_CPT_MAX_SG_IN_CNT],
    pub out: [otx_cpt_buf_ptr; OTX_CPT_MAX_SG_OUT_CNT],
    pub iv_out: *mut u8, pub rlen: u16, pub incnt: u8, pub outcnt: u8,
    pub req_type: u8, pub is_enc: u8, pub is_trunc_hmac: u8,
}

#[repr(C)]
pub struct otx_cpt_info_buffer {
    pub pentry: *mut otx_cpt_pending_entry, pub req: *mut otx_cpt_req_info,
    pub pdev: *mut pci_dev, pub completion_addr: *mut u64, pub out_buffer: *mut u8,
    pub in_buffer: *mut u8, pub dptr_baddr: dma_addr_t, pub rptr_baddr: dma_addr_t,
    pub comp_baddr: dma_addr_t, pub time_in: usize, pub dlen: u32, pub dma_len: u32,
    pub extra_time: u8,
}

pub unsafe fn do_request_cleanup(pdev: *mut pci_dev, info: *mut otx_cpt_info_buffer) {
    if (*info).dptr_baddr != 0 { dma_unmap_single(pdev, (*info).dptr_baddr, (*info).dma_len, DMA_BIDIRECTIONAL); }
    if !(*info).req.is_null() {
        let req = (*info).req;
        for i in 0..(*req).outcnt as usize { if (*req).out[i].dma_addr != 0 { dma_unmap_single(pdev, (*req).out[i].dma_addr, (*req).out[i].size as u32, DMA_BIDIRECTIONAL); } }
        for i in 0..(*req).incnt as usize { if (*req).in_[i].dma_addr != 0 { dma_unmap_single(pdev, (*req).in_[i].dma_addr, (*req).in_[i].size as u32, DMA_BIDIRECTIONAL); } }
    }
    kfree_sensitive(info);
}

pub struct otx_cptvf_wqe;
pub unsafe extern "C" fn otx_cpt_dump_sg_list(pdev: *mut pci_dev, req: *mut otx_cpt_req_info);
pub unsafe extern "C" fn otx_cpt_post_process(wqe: *mut otx_cptvf_wqe);
pub unsafe extern "C" fn otx_cpt_do_request(pdev: *mut pci_dev, req: *mut otx_cpt_req_info, cpu_num: i32) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
