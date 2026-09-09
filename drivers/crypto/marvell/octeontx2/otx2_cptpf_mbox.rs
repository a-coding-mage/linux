// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 Marvell. */

/* Dependencies are supplied by the surrounding translation unit. */

const CPT_INLINE_RX_OPCODE: u8 = 0x26 | (1 << 6);
const CN10K_CPT_INLINE_RX_OPCODE: u8 = 0x29 | (1 << 6);
const OTX2_CPT_PF_DRV_VERSION: u32 = 0x1;

unsafe fn cpt_inline_rx_opcode(pdev: *mut pci_dev) -> u8 {
    if is_dev_otx2(pdev) { CPT_INLINE_RX_OPCODE } else { CN10K_CPT_INLINE_RX_OPCODE }
}

unsafe fn forward_to_af(cptpf: *mut otx2_cptpf_dev, vf: *mut otx2_cptvf_info,
                        req: *mut mbox_msghdr, size: i32) -> i32 {
    mutex_lock(&mut (*cptpf).lock);
    let msg = otx2_mbox_alloc_msg(&mut (*cptpf).afpf_mbox, 0, size);
    if msg.is_null() { mutex_unlock(&mut (*cptpf).lock); return -ENOMEM; }
    memcpy((msg as *mut u8).add(core::mem::size_of::<mbox_msghdr>()),
           (req as *mut u8).add(core::mem::size_of::<mbox_msghdr>()), size as usize);
    (*msg).id = (*req).id; (*msg).pcifunc = (*req).pcifunc;
    (*msg).sig = (*req).sig; (*msg).ver = (*req).ver;
    let ret = otx2_cpt_sync_mbox_msg(&mut (*cptpf).afpf_mbox);
    if ret == -EIO {
        dev_warn(&(*(*cptpf).pdev).dev, "AF not responding to VF%d messages\n", (*vf).vf_id);
        mutex_unlock(&mut (*cptpf).lock); return ret;
    }
    mutex_unlock(&mut (*cptpf).lock); 0
}

unsafe fn handle_msg_get_caps(cptpf: *mut otx2_cptpf_dev, vf: *mut otx2_cptvf_info,
                              req: *mut mbox_msghdr) -> i32 {
    let rsp = otx2_mbox_alloc_msg(&mut (*cptpf).vfpf_mbox, (*vf).vf_id,
                                  core::mem::size_of::<otx2_cpt_caps_rsp>()) as *mut otx2_cpt_caps_rsp;
    if rsp.is_null() { return -ENOMEM; }
    (*rsp).hdr.id = MBOX_MSG_GET_CAPS; (*rsp).hdr.sig = OTX2_MBOX_RSP_SIG;
    (*rsp).hdr.pcifunc = (*req).pcifunc;
    (*rsp).cpt_pf_drv_version = OTX2_CPT_PF_DRV_VERSION;
    (*rsp).cpt_revision = (*cptpf).eng_grps.rid;
    memcpy(&mut (*rsp).eng_caps as *mut _ as *mut u8, &(*cptpf).eng_caps as *const _ as *const u8,
           core::mem::size_of_val(&(*rsp).eng_caps)); 0
}

unsafe fn handle_msg_get_eng_grp_num(cptpf: *mut otx2_cptpf_dev, vf: *mut otx2_cptvf_info,
                                     req: *mut mbox_msghdr) -> i32 {
    let grp_req = req as *mut otx2_cpt_egrp_num_msg;
    let rsp = otx2_mbox_alloc_msg(&mut (*cptpf).vfpf_mbox, (*vf).vf_id,
                                  core::mem::size_of::<otx2_cpt_egrp_num_rsp>()) as *mut otx2_cpt_egrp_num_rsp;
    if rsp.is_null() { return -ENOMEM; }
    (*rsp).hdr.id = MBOX_MSG_GET_ENG_GRP_NUM; (*rsp).hdr.sig = OTX2_MBOX_RSP_SIG;
    (*rsp).hdr.pcifunc = (*req).pcifunc; (*rsp).eng_type = (*grp_req).eng_type;
    (*rsp).eng_grp_num = otx2_cpt_get_eng_grp(&(*cptpf).eng_grps, (*grp_req).eng_type); 0
}

unsafe fn handle_msg_kvf_limits(cptpf: *mut otx2_cptpf_dev, vf: *mut otx2_cptvf_info,
                                req: *mut mbox_msghdr) -> i32 {
    let rsp = otx2_mbox_alloc_msg(&mut (*cptpf).vfpf_mbox, (*vf).vf_id,
                                  core::mem::size_of::<otx2_cpt_kvf_limits_rsp>()) as *mut otx2_cpt_kvf_limits_rsp;
    if rsp.is_null() { return -ENOMEM; }
    (*rsp).hdr.id = MBOX_MSG_GET_KVF_LIMITS; (*rsp).hdr.sig = OTX2_MBOX_RSP_SIG;
    (*rsp).hdr.pcifunc = (*req).pcifunc; (*rsp).kvf_limits = (*cptpf).kvf_limits; 0
}

unsafe fn send_inline_ipsec_inbound_msg(cptpf: *mut otx2_cptpf_dev, sso_pf_func: i32, slot: u8) -> i32 {
    let pdev = (*cptpf).pdev;
    let req = otx2_mbox_alloc_msg_rsp(&mut (*cptpf).afpf_mbox, 0,
        core::mem::size_of::<cpt_inline_ipsec_cfg_msg>(), core::mem::size_of::<msg_rsp>()) as *mut cpt_inline_ipsec_cfg_msg;
    if req.is_null() { dev_err(&(*pdev).dev, "RVU MBOX failed to get message.\n"); return -EFAULT; }
    memset(req as *mut u8, 0, core::mem::size_of::<cpt_inline_ipsec_cfg_msg>());
    (*req).hdr.id = MBOX_MSG_CPT_INLINE_IPSEC_CFG; (*req).hdr.sig = OTX2_MBOX_REQ_SIG;
    (*req).hdr.pcifunc = OTX2_CPT_RVU_PFFUNC(pdev, (*cptpf).pf_id, 0);
    (*req).dir = CPT_INLINE_INBOUND; (*req).slot = slot;
    (*req).sso_pf_func_ovrd = (*cptpf).sso_pf_func_ovrd; (*req).sso_pf_func = sso_pf_func; (*req).enable = 1;
    otx2_cpt_send_mbox_msg(&mut (*cptpf).afpf_mbox, pdev)
}

unsafe fn rx_inline_ipsec_lf_cfg(cptpf: *mut otx2_cptpf_dev, egrp: u8,
                                 req: *mut otx2_cpt_rx_inline_lf_cfg) -> i32 {
    let pdev = (*cptpf).pdev;
    let nix_req = otx2_mbox_alloc_msg_rsp(&mut (*cptpf).afpf_mbox, 0,
        core::mem::size_of::<nix_inline_ipsec_cfg>(), core::mem::size_of::<msg_rsp>()) as *mut nix_inline_ipsec_cfg;
    if nix_req.is_null() { dev_err(&(*pdev).dev, "RVU MBOX failed to get message.\n"); return -EFAULT; }
    memset(nix_req as *mut u8, 0, core::mem::size_of::<nix_inline_ipsec_cfg>());
    (*nix_req).hdr.id = MBOX_MSG_NIX_INLINE_IPSEC_CFG; (*nix_req).hdr.sig = OTX2_MBOX_REQ_SIG;
    (*nix_req).enable = 1; (*nix_req).credit_th = (*req).credit_th; (*nix_req).bpid = (*req).bpid;
    (*nix_req).cpt_credit = if (*req).credit == 0 || (*req).credit > OTX2_CPT_INST_QLEN_MSGS { OTX2_CPT_INST_QLEN_MSGS - 1 } else { (*req).credit - 1 };
    (*nix_req).gen_cfg.egrp = egrp;
    (*nix_req).gen_cfg.opcode = if (*req).opcode != 0 { (*req).opcode } else { cpt_inline_rx_opcode(pdev) };
    (*nix_req).gen_cfg.param1 = (*req).param1; (*nix_req).gen_cfg.param2 = (*req).param2;
    (*nix_req).inst_qsel.cpt_pf_func = OTX2_CPT_RVU_PFFUNC(pdev, (*cptpf).pf_id, 0); (*nix_req).inst_qsel.cpt_slot = 0;
    let ret = otx2_cpt_send_mbox_msg(&mut (*cptpf).afpf_mbox, pdev); if ret != 0 { return ret; }
    if (*cptpf).has_cpt1 { let ret = send_inline_ipsec_inbound_msg(cptpf, (*req).sso_pf_func, 1); if ret != 0 { return ret; } }
    send_inline_ipsec_inbound_msg(cptpf, (*req).sso_pf_func, 0)
}

pub unsafe fn otx2_inline_cptlf_setup(cptpf: *mut otx2_cptpf_dev, lfs: *mut otx2_cptlfs_info, egrp: u8, _num_lfs: i32) -> i32 {
    let mut ret = otx2_cptlf_init(lfs, 1 << egrp, OTX2_CPT_QUEUE_HI_PRIO, 1);
    if ret != 0 { dev_err(&(*(*cptpf).pdev).dev, "LF configuration failed for RX inline ipsec.\n"); return ret; }
    ret = otx2_cpt_msix_offset_msg(lfs); if ret != 0 { otx2_cptlf_shutdown(lfs); return ret; }
    ret = otx2_cptlf_register_misc_interrupts(lfs); if ret != 0 { otx2_cptlf_unregister_misc_interrupts(lfs); otx2_cptlf_shutdown(lfs); return ret; } 0
}

pub unsafe fn otx2_inline_cptlf_cleanup(lfs: *mut otx2_cptlfs_info) { otx2_cptlf_unregister_misc_interrupts(lfs); otx2_cptlf_shutdown(lfs); }

unsafe fn handle_msg_rx_inline_ipsec_lf_cfg(cptpf: *mut otx2_cptpf_dev, req: *mut mbox_msghdr) -> i32 {
    let cfg = req as *mut otx2_cpt_rx_inline_lf_cfg; let num_lfs = 1; let mut ret; 
    if (*cptpf).lfs.lfs_num != 0 { dev_err(&(*(*cptpf).pdev).dev, "LF is already configured for RX inline ipsec.\n"); return -EEXIST; }
    let egrp = otx2_cpt_get_eng_grp(&(*cptpf).eng_grps, OTX2_CPT_IE_TYPES);
    if egrp == OTX2_CPT_INVALID_CRYPTO_ENG_GRP { dev_err(&(*(*cptpf).pdev).dev, "Engine group for inline ipsec is not available\n"); return -ENOENT; }
    (*cptpf).lfs.global_slot=0; (*cptpf).lfs.ctx_ilen_ovrd=(*cfg).ctx_ilen_valid; (*cptpf).lfs.ctx_ilen=(*cfg).ctx_ilen;
    ret=otx2_inline_cptlf_setup(cptpf,&mut (*cptpf).lfs,egrp,num_lfs); if ret!=0{return ret;}
    if (*cptpf).has_cpt1 { (*cptpf).rsrc_req_blkaddr=BLKADDR_CPT1; (*cptpf).cpt1_lfs.global_slot=num_lfs; (*cptpf).cpt1_lfs.ctx_ilen_ovrd=(*cfg).ctx_ilen_valid; (*cptpf).cpt1_lfs.ctx_ilen=(*cfg).ctx_ilen; ret=otx2_inline_cptlf_setup(cptpf,&mut (*cptpf).cpt1_lfs,egrp,num_lfs); if ret!=0 { otx2_inline_cptlf_cleanup(&mut (*cptpf).lfs); return ret; } (*cptpf).rsrc_req_blkaddr=0; }
    ret=rx_inline_ipsec_lf_cfg(cptpf,egrp,cfg); if ret!=0 { if (*cptpf).has_cpt1 { otx2_inline_cptlf_cleanup(&mut (*cptpf).cpt1_lfs); } otx2_inline_cptlf_cleanup(&mut (*cptpf).lfs); } ret
}

unsafe fn cptpf_handle_vf_req(cptpf:*mut otx2_cptpf_dev,vf:*mut otx2_cptvf_info,req:*mut mbox_msghdr,size:i32)->i32 {
    if (*req).sig != OTX2_MBOX_REQ_SIG { otx2_reply_invalid_msg(&mut (*cptpf).vfpf_mbox,(*vf).vf_id,0,(*req).id); otx2_mbox_msg_send(&mut (*cptpf).vfpf_mbox,(*vf).vf_id); return 0; }
    match (*req).id { MBOX_MSG_GET_ENG_GRP_NUM=>handle_msg_get_eng_grp_num(cptpf,vf,req), MBOX_MSG_GET_CAPS=>handle_msg_get_caps(cptpf,vf,req), MBOX_MSG_GET_KVF_LIMITS=>handle_msg_kvf_limits(cptpf,vf,req), MBOX_MSG_RX_INLINE_IPSEC_LF_CFG=>handle_msg_rx_inline_ipsec_lf_cfg(cptpf,req), _=>forward_to_af(cptpf,vf,req,size) }
}

pub unsafe fn otx2_cptpf_vfpf_mbox_intr(_irq:i32,arg:*mut core::ffi::c_void)->irqreturn_t { let cptpf=arg as *mut otx2_cptpf_dev; for i in 0..2 { let intr=otx2_cpt_read64((*cptpf).reg_base,BLKADDR_RVUM,0,RVU_PF_VFPF_MBOX_INTX(i)); for vf_idx in (i*64)..(*cptpf).enabled_vfs { let vf=&mut (*cptpf).vf[vf_idx as usize]; if intr & (1u64<<vf.intr_idx)!=0 { queue_work((*cptpf).vfpf_mbox_wq,&mut vf.vfpf_mbox_work); otx2_cpt_write64((*cptpf).reg_base,BLKADDR_RVUM,0,RVU_PF_VFPF_MBOX_INTX(i),1u64<<vf.intr_idx); } } } IRQ_HANDLED }

pub unsafe fn otx2_cptpf_vfpf_mbox_handler(work:*mut work_struct) { let vf=container_of(work,otx2_cptvf_info,vfpf_mbox_work); let cptpf=(*vf).cptpf; let mbox=&mut (*cptpf).vfpf_mbox; smp_rmb(); let mdev=&mut mbox.dev[(*vf).vf_id as usize]; let hdr=(mdev.mbase.add(mbox.rx_start as usize)) as *mut mbox_hdr; let mut offset=mbox.rx_start+ALIGN(core::mem::size_of::<mbox_hdr>() as i32,MBOX_MSG_ALIGN); for _ in 0..(*hdr).num_msgs { let msg=mdev.mbase.add(offset as usize) as *mut mbox_msghdr; (*msg).pcifunc=rvu_make_pcifunc((*cptpf).pdev,(*cptpf).pf_id,(*vf).vf_id+1); let err=cptpf_handle_vf_req(cptpf,vf,msg,(*msg).next_msgoff-offset); if err==-ENOMEM||err==-EIO {break;} offset=(*msg).next_msgoff; smp_wmb(); } if mdev.num_msgs!=0 {otx2_mbox_msg_send(mbox,(*vf).vf_id);} }

pub unsafe fn otx2_cptpf_afpf_mbox_intr(_irq:i32,arg:*mut core::ffi::c_void)->irqreturn_t { let cptpf=arg as *mut otx2_cptpf_dev; let intr=otx2_cpt_read64((*cptpf).reg_base,BLKADDR_RVUM,0,RVU_PF_INT); if intr&1!=0 { let mbox=&mut (*cptpf).afpf_mbox; let mdev=&mut mbox.dev[0]; let hdr=mdev.mbase.add(mbox.rx_start as usize) as *mut mbox_hdr; if (*hdr).num_msgs!=0 {queue_work((*cptpf).afpf_mbox_wq,&mut (*cptpf).afpf_mbox_work);} let up=&mut (*cptpf).afpf_mbox_up; let umdev=&mut up.dev[0]; let uhdr=umdev.mbase.add(up.rx_start as usize) as *mut mbox_hdr; if (*uhdr).num_msgs!=0 {queue_work((*cptpf).afpf_mbox_wq,&mut (*cptpf).afpf_mbox_up_work);} otx2_cpt_write64((*cptpf).reg_base,BLKADDR_RVUM,0,RVU_PF_INT,1); } IRQ_HANDLED }

unsafe fn process_afpf_mbox_msg(cptpf:*mut otx2_cptpf_dev,msg:*mut mbox_msghdr) { if (*msg).id>=MBOX_MSG_MAX || (*msg).sig!=OTX2_MBOX_RSP_SIG{return;} let lfs=if (*cptpf).rsrc_req_blkaddr==BLKADDR_CPT1 {&mut (*cptpf).cpt1_lfs} else {&mut (*cptpf).lfs}; match (*msg).id { MBOX_MSG_READY=>{(*cptpf).pf_id=rvu_get_pf((*cptpf).pdev,(*msg).pcifunc);}, MBOX_MSG_ATTACH_RESOURCES=>{if (*msg).rc==0{lfs.are_lfs_attached=1;}}, MBOX_MSG_DETACH_RESOURCES=>{if (*msg).rc==0{lfs.are_lfs_attached=0;}}, MBOX_MSG_CPT_INLINE_IPSEC_CFG|MBOX_MSG_NIX_INLINE_IPSEC_CFG|MBOX_MSG_CPT_LF_RESET|MBOX_MSG_LMTST_TBL_SETUP=>{}, _=>{} } }
unsafe fn forward_to_vf(cptpf:*mut otx2_cptpf_dev,msg:*mut mbox_msghdr,vf_id:i32,size:i32) { if (*msg).id>=MBOX_MSG_MAX||(*msg).sig!=OTX2_MBOX_RSP_SIG{return;} let id=vf_id-1; if id>=(*cptpf).enabled_vfs||(*msg).id==MBOX_MSG_VF_FLR{return;} let fwd=otx2_mbox_alloc_msg(&mut (*cptpf).vfpf_mbox,id,size); if fwd.is_null(){return;} memcpy((fwd as *mut u8).add(core::mem::size_of::<mbox_msghdr>()),(msg as *mut u8).add(core::mem::size_of::<mbox_msghdr>()),size as usize); (*fwd).id=(*msg).id;(*fwd).pcifunc=(*msg).pcifunc;(*fwd).sig=(*msg).sig;(*fwd).ver=(*msg).ver;(*fwd).rc=(*msg).rc; }
unsafe fn handle_msg_cpt_inst_lmtst(cptpf:*mut otx2_cptpf_dev,msg:*mut mbox_msghdr) { let req=msg as *mut cpt_inst_lmtst_req; if (*cptpf).lfs.lfs_num!=0 {(*(*cptpf).lfs.ops).send_cmd((*req).inst as *mut union_otx2_cpt_inst_s,1,&mut (*cptpf).lfs.lf[0]);} let rsp=otx2_mbox_alloc_msg(&mut (*cptpf).afpf_mbox_up,0,core::mem::size_of::<msg_rsp>()) as *mut msg_rsp; if !rsp.is_null(){(*rsp).hdr.id=(*msg).id;(*rsp).hdr.sig=OTX2_MBOX_RSP_SIG;(*rsp).hdr.pcifunc=0;(*rsp).hdr.rc=0;} }
unsafe fn process_afpf_mbox_up_msg(cptpf:*mut otx2_cptpf_dev,msg:*mut mbox_msghdr){if (*msg).id==MBOX_MSG_CPT_INST_LMTST{handle_msg_cpt_inst_lmtst(cptpf,msg)}else{otx2_reply_invalid_msg(&mut (*cptpf).afpf_mbox_up,0,0,(*msg).id)}}
pub unsafe fn otx2_cptpf_afpf_mbox_handler(work:*mut work_struct) { let cptpf=container_of(work,otx2_cptpf_dev,afpf_mbox_work); let mbox=&mut (*cptpf).afpf_mbox; let mdev=&mut mbox.dev[0]; smp_wmb(); let hdr=mdev.mbase.add(mbox.rx_start as usize) as *mut mbox_hdr; let mut offset=ALIGN(core::mem::size_of::<mbox_hdr>() as i32,MBOX_MSG_ALIGN); for _ in 0..(*hdr).num_msgs { let msg=mdev.mbase.add((mbox.rx_start+offset) as usize) as *mut mbox_msghdr; let vf_id=((*msg).pcifunc>>RVU_PFVF_FUNC_SHIFT)&RVU_PFVF_FUNC_MASK; if vf_id>0 {forward_to_vf(cptpf,msg,vf_id,(*msg).next_msgoff-offset);} else {process_afpf_mbox_msg(cptpf,msg);} offset=(*msg).next_msgoff; smp_wmb(); mdev.msgs_acked+=1; } otx2_mbox_reset(mbox,0); }

pub unsafe fn otx2_cptpf_afpf_mbox_up_handler(work:*mut work_struct) { let cptpf=container_of(work,otx2_cptpf_dev,afpf_mbox_up_work); let mbox=&mut (*cptpf).afpf_mbox_up; let mdev=&mut mbox.dev[0]; smp_wmb(); let hdr=mdev.mbase.add(mbox.rx_start as usize) as *mut mbox_hdr; let mut offset=mbox.rx_start+ALIGN(core::mem::size_of::<mbox_hdr>() as i32,MBOX_MSG_ALIGN); for _ in 0..(*hdr).num_msgs { let msg=mdev.mbase.add(offset as usize) as *mut mbox_msghdr; process_afpf_mbox_up_msg(cptpf,msg); offset=mbox.rx_start+(*msg).next_msgoff; } otx2_mbox_msg_send(mbox,0); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
