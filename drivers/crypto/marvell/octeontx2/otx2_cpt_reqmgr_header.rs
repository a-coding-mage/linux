/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2020 Marvell.
 */

// Dependency symbols from otx2_cpt_common.h and the kernel are intentionally external.

pub const OTX2_CPT_COMPLETION_CODE_SIZE: usize = 8;
pub const OTX2_CPT_COMPLETION_CODE_INIT: u32 = OTX2_CPT_COMP_E_NOTDONE;
pub const OTX2_CPT_MAX_SG_IN_CNT: usize = 50;
pub const OTX2_CPT_MAX_SG_OUT_CNT: usize = 50;
pub const OTX2_CPT_DMA_MODE_DIRECT: u32 = 0;
pub const OTX2_CPT_DMA_MODE_SG: u32 = 1;
pub const OTX2_CPT_FROM_CPTR: u32 = 0;
pub const OTX2_CPT_FROM_DPTR: u32 = 1;
pub const OTX2_CPT_MAX_REQ_SIZE: u32 = 65535;
pub const SG_COMPS_MAX: usize = 4;
pub const SGV2_COMPS_MAX: usize = 3;
pub const SG_COMP_3: usize = 3;
pub const SG_COMP_2: usize = 2;
pub const SG_COMP_1: usize = 1;
pub const OTX2_CPT_DPTR_RPTR_ALIGN: usize = 8;
pub const OTX2_CPT_RES_ADDR_ALIGN: usize = 32;

#[repr(C)]
pub union otx2_cpt_opcode { pub flags: u16, pub s: otx2_cpt_opcode_s }
#[repr(C)] pub struct otx2_cpt_opcode_s { pub major: u8, pub minor: u8 }

#[repr(C)]
pub struct otx2_cptvf_request { pub param1: u32, pub param2: u32, pub dlen: u16, pub opcode: otx2_cpt_opcode, pub cptr_dma: dma_addr_t, pub cptr: *mut core::ffi::c_void }

#[repr(C)]
pub union otx2_cpt_iq_cmd_word0 { pub u: u64, pub s: otx2_cpt_iq_cmd_word0_s }
#[repr(C)] pub struct otx2_cpt_iq_cmd_word0_s { pub opcode: __be16, pub param1: __be16, pub param2: __be16, pub dlen: __be16 }
#[repr(C)]
pub union otx2_cpt_iq_cmd_word3 { pub u: u64, pub s: otx2_cpt_iq_cmd_word3_s }
#[repr(C)] pub struct otx2_cpt_iq_cmd_word3_s { pub cptr: u64, pub grp: u64 }
#[repr(C)] pub struct otx2_cpt_iq_command { pub cmd: otx2_cpt_iq_cmd_word0, pub dptr: u64, pub rptr: u64, pub cptr: otx2_cpt_iq_cmd_word3 }

#[repr(C)] pub struct otx2_cpt_pending_entry { pub completion_addr: *mut core::ffi::c_void, pub info: *mut core::ffi::c_void, pub callback: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void, *mut core::ffi::c_void)>, pub areq: *mut crypto_async_request, pub resume_sender: u8, pub busy: u8 }
#[repr(C)] pub struct otx2_cpt_pending_queue { pub head: *mut otx2_cpt_pending_entry, pub front: u32, pub rear: u32, pub pending_count: u32, pub qlen: u32, pub lock: spinlock_t }
#[repr(C)] pub struct otx2_cpt_buf_ptr { pub vptr: *mut u8, pub dma_addr: dma_addr_t, pub size: u16 }

#[repr(C)] pub union otx2_cpt_ctrl_info { pub flags: u32, pub s: otx2_cpt_ctrl_info_s }
#[repr(C)] pub struct otx2_cpt_ctrl_info_s { pub se_req: u32, pub dma_mode: u32, pub grp: u32, pub reserved_6_31: u32 }

#[repr(C)] pub struct otx2_cpt_req_info { pub callback: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void, *mut core::ffi::c_void)>, pub areq: *mut crypto_async_request, pub req: otx2_cptvf_request, pub ctrl: otx2_cpt_ctrl_info, pub in_: [otx2_cpt_buf_ptr; OTX2_CPT_MAX_SG_IN_CNT], pub out: [otx2_cpt_buf_ptr; OTX2_CPT_MAX_SG_OUT_CNT], pub iv_out: *mut u8, pub rlen: u16, pub in_cnt: u8, pub out_cnt: u8, pub req_type: u8, pub is_enc: u8, pub is_trunc_hmac: u8 }
#[repr(C)] pub struct otx2_cpt_inst_info { pub pentry: *mut otx2_cpt_pending_entry, pub req: *mut otx2_cpt_req_info, pub pdev: *mut pci_dev, pub completion_addr: *mut core::ffi::c_void, pub out_buffer: *mut u8, pub in_buffer: *mut u8, pub dptr_baddr: dma_addr_t, pub rptr_baddr: dma_addr_t, pub comp_baddr: dma_addr_t, pub time_in: c_ulong, pub dlen: u32, pub dma_len: u32, pub gthr_sz: u64, pub sctr_sz: u64, pub extra_time: u8 }
#[repr(C)] pub struct otx2_cpt_sglist_component { pub len0: __be16, pub len1: __be16, pub len2: __be16, pub len3: __be16, pub ptr0: __be64, pub ptr1: __be64, pub ptr2: __be64, pub ptr3: __be64 }
#[repr(C)] pub struct cn10kb_cpt_sglist_component { pub len0: u16, pub len1: u16, pub len2: u16, pub valid_segs: u16, pub ptr0: u64, pub ptr1: u64, pub ptr2: u64 }

// The inline implementations retain kernel operations and external types/functions.
pub unsafe fn otx2_cpt_info_destroy(pdev: *mut pci_dev, info: *mut otx2_cpt_inst_info) {
    let req: *mut otx2_cpt_req_info;
    if (*info).dptr_baddr != 0 { dma_unmap_single(&mut (*pdev).dev, (*info).dptr_baddr, (*info).dma_len, DMA_BIDIRECTIONAL); }
    if !(*info).req.is_null() { req = (*info).req; for i in 0..(*req).out_cnt as usize { if (*req).out[i].dma_addr != 0 { dma_unmap_single(&mut (*pdev).dev, (*req).out[i].dma_addr, (*req).out[i].size, DMA_BIDIRECTIONAL); } } for i in 0..(*req).in_cnt as usize { if (*req).in_[i].dma_addr != 0 { dma_unmap_single(&mut (*pdev).dev, (*req).in_[i].dma_addr, (*req).in_[i].size, DMA_BIDIRECTIONAL); } } }
    kfree(info as *mut core::ffi::c_void);
}

pub unsafe fn setup_sgio_components(pdev: *mut pci_dev, list: *mut otx2_cpt_buf_ptr, buf_count: i32, buffer: *mut u8) -> i32 {
    if list.is_null() { dev_err(&mut (*pdev).dev, "Input list pointer is NULL\n"); return -EINVAL; }
    let mut i = 0; while i < buf_count { if !(*list.add(i as usize)).vptr.is_null() { (*list.add(i as usize)).dma_addr = dma_map_single(&mut (*pdev).dev, (*list.add(i as usize)).vptr, (*list.add(i as usize)).size, DMA_BIDIRECTIONAL); if dma_mapping_error(&mut (*pdev).dev, (*list.add(i as usize)).dma_addr) { dev_err(&mut (*pdev).dev, "Dma mapping failed\n"); let mut j=0; while j<i { if (*list.add(j as usize)).dma_addr != 0 { dma_unmap_single(&mut (*pdev).dev, (*list.add(j as usize)).dma_addr, (*list.add(j as usize)).size, DMA_BIDIRECTIONAL); } (*list.add(j as usize)).dma_addr=0; j+=1; } return -EIO; } } i+=1; }
    let mut sg = buffer as *mut otx2_cpt_sglist_component; let components = buf_count / 4; for n in 0..components { let b=(n*4) as usize; (*sg).len0=cpu_to_be16((*list.add(b)).size); (*sg).len1=cpu_to_be16((*list.add(b+1)).size); (*sg).len2=cpu_to_be16((*list.add(b+2)).size); (*sg).len3=cpu_to_be16((*list.add(b+3)).size); (*sg).ptr0=cpu_to_be64((*list.add(b)).dma_addr); (*sg).ptr1=cpu_to_be64((*list.add(b+1)).dma_addr); (*sg).ptr2=cpu_to_be64((*list.add(b+2)).dma_addr); (*sg).ptr3=cpu_to_be64((*list.add(b+3)).dma_addr); sg=sg.add(1); }
    let rem=buf_count%4; let b=(components*4) as usize; if rem>=3 { (*sg).len2=cpu_to_be16((*list.add(b+2)).size); (*sg).ptr2=cpu_to_be64((*list.add(b+2)).dma_addr); } if rem>=2 { (*sg).len1=cpu_to_be16((*list.add(b+1)).size); (*sg).ptr1=cpu_to_be64((*list.add(b+1)).dma_addr); } if rem>=1 { (*sg).len0=cpu_to_be16((*list.add(b)).size); (*sg).ptr0=cpu_to_be64((*list.add(b)).dma_addr); } 0
}

pub unsafe fn sgv2io_components_setup(pdev: *mut pci_dev, list: *mut otx2_cpt_buf_ptr, buf_count: i32, buffer: *mut u8) -> i32 {
    if list.is_null() { dev_err(&mut (*pdev).dev, "Input list pointer is NULL\n"); return -EFAULT; }
    let mut i=0; while i<buf_count { if !(*list.add(i as usize)).vptr.is_null() { (*list.add(i as usize)).dma_addr=dma_map_single(&mut (*pdev).dev,(*list.add(i as usize)).vptr,(*list.add(i as usize)).size,DMA_BIDIRECTIONAL); if dma_mapping_error(&mut (*pdev).dev,(*list.add(i as usize)).dma_addr) { dev_err(&mut (*pdev).dev,"Dma mapping failed\n"); let mut j=0; while j<i { if (*list.add(j as usize)).dma_addr!=0 { dma_unmap_single(&mut (*pdev).dev,(*list.add(j as usize)).dma_addr,(*list.add(j as usize)).size,DMA_BIDIRECTIONAL); } (*list.add(j as usize)).dma_addr=0; j+=1; } return -EIO; } } i+=1; }
    let mut sg=buffer as *mut cn10kb_cpt_sglist_component; let groups=buf_count/3; for n in 0..groups { let b=(n*3) as usize; (*sg).len0=(*list.add(b)).size; (*sg).len1=(*list.add(b+1)).size; (*sg).len2=(*list.add(b+2)).size; (*sg).ptr0=(*list.add(b)).dma_addr; (*sg).ptr1=(*list.add(b+1)).dma_addr; (*sg).ptr2=(*list.add(b+2)).dma_addr; (*sg).valid_segs=3; sg=sg.add(1); } let rem=buf_count%3; let b=(groups*3) as usize; (*sg).valid_segs=rem as u16; if rem>=2 { (*sg).len1=(*list.add(b+1)).size; (*sg).ptr1=(*list.add(b+1)).dma_addr; } if rem>=1 { (*sg).len0=(*list.add(b)).size; (*sg).ptr0=(*list.add(b)).dma_addr; } 0
}

pub unsafe fn cn10k_sgv2_info_create(pdev: *mut pci_dev, req: *mut otx2_cpt_req_info, gfp: gfp_t) -> *mut otx2_cpt_inst_info {
    let info_len=core::mem::size_of::<otx2_cpt_inst_info>();
    let g_len=(((*req).in_cnt as usize+2)/3)*core::mem::size_of::<cn10kb_cpt_sglist_component>();
    let s_len=(((*req).out_cnt as usize+2)/3)*core::mem::size_of::<cn10kb_cpt_sglist_component>();
    let sg_len=g_len+s_len; let total=ALIGN(ALIGN(info_len,OTX2_CPT_DPTR_RPTR_ALIGN)+(ARCH_DMA_MINALIGN-1)&!(OTX2_CPT_DPTR_RPTR_ALIGN-1)+ALIGN(sg_len,OTX2_CPT_RES_ADDR_ALIGN)+core::mem::size_of::<otx2_cpt_res_s>(),1);
    let info=kzalloc(total,gfp) as *mut otx2_cpt_inst_info; if info.is_null(){return core::ptr::null_mut();}
    let mut dlen=0u32; for i in 0..(*req).in_cnt as usize { dlen+=(*req).in_[i].size as u32; } (*info).dlen=dlen;
    (*info).in_buffer=PTR_ALIGN((info as *mut u8).add(info_len),ARCH_DMA_MINALIGN); (*info).out_buffer=(*info).in_buffer.add(g_len); (*info).gthr_sz=(*req).in_cnt as u64; (*info).sctr_sz=(*req).out_cnt as u64;
    if sgv2io_components_setup(pdev,(*req).in_.as_mut_ptr(),(*req).in_cnt as i32,(*info).in_buffer)!=0 || sgv2io_components_setup(pdev,(*req).out.as_mut_ptr(),(*req).out_cnt as i32,(*info).out_buffer)!=0 { otx2_cpt_info_destroy(pdev,info); return core::ptr::null_mut(); }
    (*info).dma_len=(total-info_len) as u32; (*info).dptr_baddr=dma_map_single(&mut (*pdev).dev,(*info).in_buffer,(*info).dma_len,DMA_BIDIRECTIONAL); if dma_mapping_error(&mut (*pdev).dev,(*info).dptr_baddr){otx2_cpt_info_destroy(pdev,info);return core::ptr::null_mut();} (*info).rptr_baddr=(*info).dptr_baddr+g_len as u64; (*info).completion_addr=PTR_ALIGN((*info).in_buffer.add(sg_len),OTX2_CPT_RES_ADDR_ALIGN) as *mut _; (*info).comp_baddr=ALIGN((*info).dptr_baddr+sg_len as u64,OTX2_CPT_RES_ADDR_ALIGN); info
}

pub const SG_LIST_HDR_SIZE: usize = 8;
pub unsafe fn otx2_sg_info_create(pdev: *mut pci_dev, req: *mut otx2_cpt_req_info, gfp: gfp_t) -> *mut otx2_cpt_inst_info {
    if (*req).in_cnt as usize>OTX2_CPT_MAX_SG_IN_CNT || (*req).out_cnt as usize>OTX2_CPT_MAX_SG_OUT_CNT { dev_err(&mut (*pdev).dev,"Error too many sg components\n"); return core::ptr::null_mut(); }
    let info_len=core::mem::size_of::<otx2_cpt_inst_info>(); let g_len=(((*req).in_cnt as usize+3)/4)*core::mem::size_of::<otx2_cpt_sglist_component>(); let s_len=(((*req).out_cnt as usize+3)/4)*core::mem::size_of::<otx2_cpt_sglist_component>(); let dlen=g_len+s_len+SG_LIST_HDR_SIZE; let total=ALIGN(ALIGN(info_len,OTX2_CPT_DPTR_RPTR_ALIGN)+(ARCH_DMA_MINALIGN-1)&!(OTX2_CPT_DPTR_RPTR_ALIGN-1)+ALIGN(dlen,OTX2_CPT_RES_ADDR_ALIGN)+core::mem::size_of::<otx2_cpt_res_s>(),1); let info=kzalloc(total,gfp) as *mut otx2_cpt_inst_info; if info.is_null(){return core::ptr::null_mut();} (*info).dlen=dlen as u32; (*info).in_buffer=PTR_ALIGN((info as *mut u8).add(info_len),ARCH_DMA_MINALIGN); (*info).out_buffer=(*info).in_buffer.add(SG_LIST_HDR_SIZE+g_len); let h=(*info).in_buffer as *mut u16; *h=(*req).out_cnt as u16; *h.add(1)=(*req).in_cnt as u16; cpu_to_be64s(h as *mut u64); if setup_sgio_components(pdev,(*req).in_.as_mut_ptr(),(*req).in_cnt as i32,(*info).in_buffer.add(8))!=0 || setup_sgio_components(pdev,(*req).out.as_mut_ptr(),(*req).out_cnt as i32,(*info).out_buffer)!=0 {otx2_cpt_info_destroy(pdev,info);return core::ptr::null_mut();} (*info).dma_len=(total-info_len) as u32; (*info).dptr_baddr=dma_map_single(&mut (*pdev).dev,(*info).in_buffer,(*info).dma_len,DMA_BIDIRECTIONAL); if dma_mapping_error(&mut (*pdev).dev,(*info).dptr_baddr){otx2_cpt_info_destroy(pdev,info);return core::ptr::null_mut();} (*info).completion_addr=PTR_ALIGN((*info).in_buffer.add(dlen),OTX2_CPT_RES_ADDR_ALIGN) as *mut _; (*info).comp_baddr=ALIGN((*info).dptr_baddr+dlen as u64,OTX2_CPT_RES_ADDR_ALIGN); info
}
extern "C" { pub fn otx2_cpt_do_request(pdev: *mut pci_dev, req: *mut otx2_cpt_req_info, cpu_num: i32) -> i32; pub fn otx2_cpt_post_process(wqe: *mut otx2_cptlf_wqe); pub fn otx2_cpt_get_eng_grp_num(pdev: *mut pci_dev, eng_type: otx2_cpt_eng_type) -> i32; }
pub struct otx2_cptlf_wqe;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
