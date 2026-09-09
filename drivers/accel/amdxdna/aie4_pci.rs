// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// C dependencies are supplied by the surrounding kernel/Rust bindings.

const NO_IOHUB: i32 = 0;
const PSP_NOTIFY_INTR: u32 = 0xD007_BE11;
const AIE4_TOTAL_COLUMN: u32 = 3;

#[repr(C)]
pub struct mailbox_info {
    pub valid: u32,
    pub protocol_major: u32,
    pub protocol_minor: u32,
    pub x2i_tail_offset: u32,
    pub x2i_head_offset: u32,
    pub x2i_buffer_addr: u32,
    pub x2i_buffer_size: u32,
    pub i2x_tail_offset: u32,
    pub i2x_head_offset: u32,
    pub i2x_buffer_addr: u32,
    pub i2x_buffer_size: u32,
    pub i2x_msi_idx: u32,
    pub reserved: [u32; 4],
}

unsafe fn aie4_fw_is_alive(xdna: *mut amdxdna_dev) -> i32 {
    let npriv = (*(*xdna).dev_info).dev_priv;
    let ndev = (*xdna).dev_handle;
    let src = (*ndev).rbuf_base.add((*npriv).mbox_info_off as usize);
    let mut fw_is_valid: u32 = 0;
    let ret = readx_poll_timeout(readl, src.add(core::mem::offset_of!(mailbox_info, valid) / core::mem::size_of::<u32>()), &mut fw_is_valid, fw_is_valid == 1, AIE_INTERVAL, AIE_TIMEOUT);
    if ret != 0 { XDNA_ERR!(xdna, "fw_is_valid={} after {} ms", fw_is_valid, DIV_ROUND_CLOSEST!(AIE_TIMEOUT, 1000000)); }
    ret
}

unsafe fn aie4_read_mbox_info(xdna: *mut amdxdna_dev, mbox_info: *mut mailbox_info) {
    let npriv = (*(*xdna).dev_info).dev_priv;
    let ndev = (*xdna).dev_handle;
    let src = (*ndev).rbuf_base.add((*npriv).mbox_info_off as usize);
    let dst = mbox_info as *mut u32;
    for i in 0..(core::mem::size_of::<mailbox_info>() / core::mem::size_of::<u32>()) { *dst.add(i) = readl(src.add(i)); }
}

unsafe fn aie4_mailbox_info(xdna: *mut amdxdna_dev, info: *mut mailbox_info) -> i32 {
    let ret = aie4_fw_is_alive(xdna); if ret != 0 { return ret; }
    aie4_read_mbox_info(xdna, info);
    let ret = aie_check_protocol(&mut (*(*xdna).dev_handle).aie, (*info).protocol_major, (*info).protocol_minor);
    if ret != 0 { XDNA_ERR!(xdna, "mailbox major.minor {}.{} is not supported", (*info).protocol_major, (*info).protocol_minor); } ret
}

unsafe fn aie4_mailbox_fini(ndev: *mut amdxdna_dev_hdl) { let xdna = (*ndev).aie.xdna; aie_destroy_chann(&mut (*ndev).aie, &mut (*ndev).aie.mgmt_chann); drmm_kfree(&mut (*xdna).ddev, (*ndev).mbox); (*ndev).mbox = core::ptr::null_mut(); }

unsafe fn aie4_irq_init(xdna: *mut amdxdna_dev) -> i32 {
    let pdev = to_pci_dev((*xdna).ddev.dev); let nvec = pci_msix_vec_count(pdev); XDNA_DBG!(xdna, "irq vectors:{}", nvec);
    if nvec <= 0 { XDNA_ERR!(xdna, "does not get number of interrupt vector"); return -EINVAL; }
    let ret = pci_alloc_irq_vectors(pdev, nvec, nvec, PCI_IRQ_MSIX); if ret < 0 { XDNA_ERR!(xdna, "failed to alloc irq vector, ret: {}", ret); } ret
}

unsafe fn aie4_mailbox_start(xdna: *mut amdxdna_dev, mbi: *mut mailbox_info) -> i32 {
    let pdev = to_pci_dev((*xdna).ddev.dev); let ndev = (*xdna).dev_handle; let npriv = (*(*xdna).dev_info).dev_priv;
    let mut i2x = &mut (*ndev).aie.mgmt_i2x; let mut x2i = &mut (*ndev).aie.mgmt_x2i;
    (*x2i).mb_head_ptr_reg=(*mbi).x2i_head_offset; (*x2i).mb_tail_ptr_reg=(*mbi).x2i_tail_offset; (*x2i).rb_start_addr=(*mbi).x2i_buffer_addr; (*x2i).rb_size=(*mbi).x2i_buffer_size;
    (*i2x).rb_start_addr=(*mbi).i2x_buffer_addr; (*i2x).rb_size=(*mbi).i2x_buffer_size; (*i2x).mb_head_ptr_reg=(*mbi).i2x_head_offset; (*i2x).mb_tail_ptr_reg=(*mbi).i2x_tail_offset;
    (*ndev).aie.mgmt_chan_idx=(*mbi).i2x_msi_idx; aie_dump_mgmt_chann_debug(&mut (*ndev).aie);
    let res = xdna_mailbox_res { ringbuf_base:(*ndev).rbuf_base, ringbuf_size:pci_resource_len(pdev,(*npriv).mbox_rbuf_bar), mbox_base:(*ndev).mbox_base, mbox_size:pci_resource_len(pdev,(*npriv).mbox_bar), name:b"xdna_aie4_mailbox\0".as_ptr() as *const _ };
    (*ndev).mbox=xdnam_mailbox_create(&mut (*xdna).ddev,&res); if (*ndev).mbox.is_null() { XDNA_ERR!(xdna,"failed to create mailbox device"); return -ENODEV; }
    (*ndev).aie.mgmt_chann=xdna_mailbox_alloc_channel((*ndev).mbox); if (*ndev).aie.mgmt_chann.is_null() { XDNA_ERR!(xdna,"failed to alloc mailbox channel"); return -ENODEV; }
    let irq=pci_irq_vector(pdev,(*ndev).aie.mgmt_chan_idx); if irq<0 { xdna_mailbox_free_channel((*ndev).aie.mgmt_chann); (*ndev).aie.mgmt_chann=core::ptr::null_mut(); return irq; }
    let ret=xdna_mailbox_start_channel((*ndev).aie.mgmt_chann,&mut (*ndev).aie.mgmt_x2i,&mut (*ndev).aie.mgmt_i2x,NO_IOHUB,irq); if ret!=0 { xdna_mailbox_free_channel((*ndev).aie.mgmt_chann); (*ndev).aie.mgmt_chann=core::ptr::null_mut(); return -EINVAL; } 0
}

unsafe fn aie4_mailbox_init(ndev:*mut amdxdna_dev_hdl)->i32 { let xdna=(*ndev).aie.xdna; let mut i=core::mem::zeroed::<mailbox_info>(); let r=aie4_mailbox_info(xdna,&mut i); if r!=0{return r} aie4_mailbox_start(xdna,&mut i) }
unsafe fn aie4_fw_stop(ndev:*mut amdxdna_dev_hdl){ aie_psp_stop((*ndev).aie.psp_hdl); aie_smu_fini((*ndev).aie.smu_hdl); }
unsafe fn aie4_fw_start(ndev:*mut amdxdna_dev_hdl)->i32 { let r=aie_smu_init((*ndev).aie.smu_hdl); if r!=0{return r} let r=aie_psp_start((*ndev).aie.psp_hdl); if r!=0{aie_smu_fini((*ndev).aie.smu_hdl)} r }

unsafe fn aie4_partition_init(ndev:*mut amdxdna_dev_hdl)->i32 { let mut msg=core::mem::zeroed::<aie4_msg_create_partition>(); msg.req.partition_col_start=0; msg.req.partition_col_count=AIE4_TOTAL_COLUMN; let r=aie_send_mgmt_msg_wait(&mut (*ndev).aie,&mut msg); if r==0 {(*ndev).partition_id=msg.resp.partition_id;} r }
unsafe fn aie4_partition_fini(ndev:*mut amdxdna_dev_hdl) { let mut msg=core::mem::zeroed::<aie4_msg_destroy_partition>(); msg.req.partition_id=(*ndev).partition_id; let _=aie_send_mgmt_msg_wait(&mut (*ndev).aie,&mut msg); }
unsafe fn aie4_query(ndev:*mut amdxdna_dev_hdl)->i32 { aie4_query_aie_metadata(ndev,&mut (*ndev).aie.metadata) }

unsafe fn aie4_pf_hw_start(ndev:*mut amdxdna_dev_hdl)->i32 { let r=aie4_fw_start(ndev); if r!=0{return r} let r=aie4_mailbox_init(ndev); if r!=0{aie4_fw_stop(ndev);return r} let r=aie4_attach_work_buffer(ndev); if r!=0{aie4_mailbox_fini(ndev);aie4_fw_stop(ndev)} r }
unsafe fn aie4_pf_hw_stop(ndev:*mut amdxdna_dev_hdl){ aie4_suspend_fw(ndev); aie4_mailbox_fini(ndev); aie4_fw_stop(ndev); }
unsafe fn aie4_vf_hw_start(ndev:*mut amdxdna_dev_hdl)->i32 { let r=aie4_mailbox_init(ndev); if r!=0{return r} let r=aie4_query(ndev); if r!=0{aie4_mailbox_fini(ndev);return r} let r=aie4_partition_init(ndev); if r!=0{aie4_mailbox_fini(ndev)} r }
unsafe fn aie4_vf_hw_stop(ndev:*mut amdxdna_dev_hdl){ aie4_partition_fini(ndev); aie4_mailbox_fini(ndev); }

unsafe fn aie4_request_firmware(ndev:*mut amdxdna_dev_hdl,npufw:*mut *const firmware,certfw:*mut *const firmware)->i32 { let xdna=(*ndev).aie.xdna; let pdev=to_pci_dev((*xdna).ddev.dev); let mut name=[0i8;128]; let r=snprintf(name.as_mut_ptr(),name.len(),b"amdnpu/%04x_%02x/%s\0".as_ptr(),(*pdev).device,(*pdev).revision,(*(*ndev).priv_).npufw_path); if r>=name.len() as i32{return -EINVAL} let r=request_firmware(npufw,name.as_ptr(),&mut (*pdev).dev); if r!=0{return r} let r=snprintf(name.as_mut_ptr(),name.len(),b"amdnpu/%04x_%02x/%s\0".as_ptr(),(*pdev).device,(*pdev).revision,(*(*ndev).priv_).certfw_path); if r>=name.len() as i32{release_firmware(*npufw);return -EINVAL} let r=request_firmware(certfw,name.as_ptr(),&mut (*pdev).dev); if r!=0{release_firmware(*npufw)} r }
unsafe fn aie4_release_firmware(_: *mut amdxdna_dev_hdl,npufw:*const firmware,certfw:*const firmware){release_firmware(certfw);release_firmware(npufw)}
unsafe fn aie4_prepare_firmware(ndev:*mut amdxdna_dev_hdl,npufw:*const firmware,certfw:*const firmware,tbl:*mut *mut core::ffi::c_void)->i32 { let mut p=core::mem::zeroed::<psp_config>(); p.fw_size=(*npufw).size;p.fw_buf=(*npufw).data;p.certfw_size=(*certfw).size;p.certfw_buf=(*certfw).data;p.arg2_mask=!0;p.notify_val=PSP_NOTIFY_INTR; for i in 0..PSP_MAX_REGS {p.psp_regs[i]=tbl.add(PSP_REG_BAR(ndev,i)).read().add(PSP_REG_OFF(ndev,i) as usize)} (*ndev).aie.psp_hdl=aiem_psp_create(&mut (*(*ndev).aie.xdna).ddev,&p); if (*ndev).aie.psp_hdl.is_null(){return -ENOMEM} 0 }
unsafe fn aie4_load_fw(ndev:*mut amdxdna_dev_hdl,tbl:*mut *mut core::ffi::c_void)->i32 { if (*(*ndev).priv_).npufw_path.is_null() && (*(*ndev).priv_).certfw_path.is_null(){return 0} let mut n=core::ptr::null();let mut c=core::ptr::null();let r=aie4_request_firmware(ndev,&mut n,&mut c);if r!=0{return r}let r=aie4_prepare_firmware(ndev,n,c,tbl);aie4_release_firmware(ndev,n,c);r }

unsafe fn aie4_doorbell_mmap(client:*mut amdxdna_client,vma:*mut vm_area_struct)->i32 { if !aie4_hwctx_valid_doorbell(client,(*vma).vm_pgoff){return -EINVAL} if vma_pages(vma)!=1{return -EINVAL} (*vma).vm_page_prot=pgprot_noncached((*vma).vm_page_prot); io_remap_pfn_range(vma,(*vma).vm_start,0,PAGE_SIZE,(*vma).vm_page_prot) }
unsafe fn aie4_get_info(client:*mut amdxdna_client,args:*mut amdxdna_drm_get_info)->i32 { match (*args).param { DRM_AMDXDNA_QUERY_AIE_METADATA=>amdxdna_get_metadata(&mut (*(*client).xdna).dev_handle.as_mut().unwrap().aie,client,args), _=>-EOPNOTSUPP } }
unsafe fn aie4_alloc_work_buffer(ndev:*mut amdxdna_dev_hdl)->i32 { let mut size=AIE4_WORK_BUFFER_MIN_SIZE; (*ndev).work_buf=amdxdna_alloc_msg_buffer((*ndev).aie.xdna,&mut size,&mut (*ndev).work_buf_addr); if IS_ERR!((*ndev).work_buf){let r=PTR_ERR!((*ndev).work_buf);(*ndev).work_buf=core::ptr::null_mut();return r}(*ndev).work_buf_size=size;0 }
unsafe fn aie4_free_work_buffer(ndev:*mut amdxdna_dev_hdl){if !(*ndev).work_buf.is_null(){amdxdna_free_msg_buffer((*ndev).aie.xdna,(*ndev).work_buf_size,(*ndev).work_buf,(*ndev).work_buf_addr);(*ndev).work_buf=core::ptr::null_mut()}}
unsafe fn aie4_pf_init(xdna:*mut amdxdna_dev)->i32{let r=aie4m_pcidev_init(xdna);if r!=0{return r}let r=aie4_alloc_work_buffer((*xdna).dev_handle);if r==0{aie4_pf_hw_start((*xdna).dev_handle)}else{r}}
unsafe fn aie4_vf_init(xdna:*mut amdxdna_dev)->i32{let r=aie4m_pcidev_init(xdna);if r==0{aie4_vf_hw_start((*xdna).dev_handle)}else{r}}
unsafe fn aie4_pf_fini(xdna:*mut amdxdna_dev){aie4_sriov_stop((*xdna).dev_handle);aie4_pf_hw_stop((*xdna).dev_handle);aie4_free_work_buffer((*xdna).dev_handle)}
unsafe fn aie4_vf_fini(xdna:*mut amdxdna_dev){aie4_vf_hw_stop((*xdna).dev_handle)}
unsafe fn aie4m_pcidev_init(xdna:*mut amdxdna_dev)->i32 { let pdev=to_pci_dev((*xdna).ddev.dev); let ndev=drmm_kzalloc(&mut (*xdna).ddev,core::mem::size_of::<amdxdna_dev_hdl>(),GFP_KERNEL); if ndev.is_null(){return -ENOMEM} (*ndev).priv_=(*(*xdna).dev_info).dev_priv;(*ndev).aie.xdna=xdna;(*xdna).dev_handle=ndev; let r=pcim_enable_device(pdev);if r!=0{return r} let r=dma_set_mask_and_coherent(&mut (*pdev).dev,DMA_BIT_MASK(64));if r!=0{return r} pci_set_master(pdev); aie4_load_fw(ndev,core::ptr::null_mut()) }

pub const aie4_pf_ops: amdxdna_dev_ops = amdxdna_dev_ops { init:Some(aie4_pf_init), fini:Some(aie4_pf_fini), sriov_configure:Some(aie4_sriov_configure) };
pub const aie4_vf_ops: amdxdna_dev_ops = amdxdna_dev_ops { init:Some(aie4_vf_init), fini:Some(aie4_vf_fini), hwctx_init:Some(aie4_hwctx_init), hwctx_fini:Some(aie4_hwctx_fini), mmap:Some(aie4_doorbell_mmap), cmd_wait:Some(aie4_cmd_wait), get_aie_info:Some(aie4_get_info) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
