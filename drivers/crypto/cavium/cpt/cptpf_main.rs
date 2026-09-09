// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2016 Cavium, Inc. */

// Linux dependencies and symbols from cptpf.h are supplied externally.

const DRV_NAME: &str = "thunder-cpt";
const DRV_VERSION: &str = "1.0";
static mut num_vfs: u32 = 4;

unsafe fn cpt_disable_cores(cpt: *mut cpt_device, mut coremask: u64, type_: u8, mut grp: u8) {
    let mut pf_exe_ctl: u64;
    let mut timeout: u32 = 100;
    let mut grpmask: u64 = 0;
    let dev = &(*(*cpt).pdev).dev;
    if type_ == AE_TYPES { coremask = coremask << (*cpt).max_se_cores; }
    grpmask = cpt_read_csr64((*cpt).reg_base, CPTX_PF_GX_EN(0, grp));
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_GX_EN(0, grp), grpmask & !coremask);
    udelay(CSR_DELAY);
    grp = cpt_read_csr64((*cpt).reg_base, CPTX_PF_EXEC_BUSY(0)) as u8;
    while (u64::from(grp) & coremask != 0) {
        dev_err(dev, "Cores still busy %llx", coremask);
        grp = cpt_read_csr64((*cpt).reg_base, CPTX_PF_EXEC_BUSY(0)) as u8;
        timeout = timeout.wrapping_sub(1);
        if timeout == u32::MAX { break; }
        udelay(CSR_DELAY);
    }
    pf_exe_ctl = cpt_read_csr64((*cpt).reg_base, CPTX_PF_EXE_CTL(0));
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_EXE_CTL(0), pf_exe_ctl & !coremask);
    udelay(CSR_DELAY);
}

unsafe fn cpt_enable_cores(cpt: *mut cpt_device, mut coremask: u64, type_: u8) {
    if type_ == AE_TYPES { coremask = coremask << (*cpt).max_se_cores; }
    let pf_exe_ctl = cpt_read_csr64((*cpt).reg_base, CPTX_PF_EXE_CTL(0));
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_EXE_CTL(0), pf_exe_ctl | coremask);
    udelay(CSR_DELAY);
}

unsafe fn cpt_configure_group(cpt: *mut cpt_device, grp: u8, mut coremask: u64, type_: u8) {
    if type_ == AE_TYPES { coremask = coremask << (*cpt).max_se_cores; }
    let pf_gx_en = cpt_read_csr64((*cpt).reg_base, CPTX_PF_GX_EN(0, grp));
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_GX_EN(0, grp), pf_gx_en | coremask);
    udelay(CSR_DELAY);
}

unsafe fn cpt_disable_mbox_interrupts(cpt: *mut cpt_device) { cpt_write_csr64((*cpt).reg_base, CPTX_PF_MBOX_ENA_W1CX(0, 0), !0u64); }
unsafe fn cpt_disable_ecc_interrupts(cpt: *mut cpt_device) { cpt_write_csr64((*cpt).reg_base, CPTX_PF_ECC0_ENA_W1C(0), !0u64); }
unsafe fn cpt_disable_exec_interrupts(cpt: *mut cpt_device) { cpt_write_csr64((*cpt).reg_base, CPTX_PF_EXEC_ENA_W1C(0), !0u64); }
unsafe fn cpt_disable_all_interrupts(cpt: *mut cpt_device) { cpt_disable_mbox_interrupts(cpt); cpt_disable_ecc_interrupts(cpt); cpt_disable_exec_interrupts(cpt); }
unsafe fn cpt_enable_mbox_interrupts(cpt: *mut cpt_device) { cpt_write_csr64((*cpt).reg_base, CPTX_PF_MBOX_ENA_W1SX(0, 0), !0u64); }

unsafe fn cpt_load_microcode(cpt: *mut cpt_device, mcode: *mut microcode) -> i32 {
    let dev = &(*(*cpt).pdev).dev;
    if mcode.is_null() || (*mcode).code.is_null() { dev_err(dev, "Either the mcode is null or data is NULL\n"); return -EINVAL; }
    if (*mcode).code_size == 0 { dev_err(dev, "microcode size is 0\n"); return -EINVAL; }
    let (mut core, total_cores) = if (*mcode).is_ae { (CPT_MAX_SE_CORES as i32, CPT_MAX_TOTAL_CORES) } else { (0, CPT_MAX_SE_CORES) };
    let mut shift = 0;
    while core < total_cores as i32 {
        if (*mcode).core_mask & (1u64 << shift) != 0 { cpt_write_csr64((*cpt).reg_base, CPTX_PF_ENGX_UCODE_BASE(0, core as u32), (*mcode).phys_base); }
        core += 1; shift += 1;
    }
    0
}

unsafe fn do_cpt_init(cpt: *mut cpt_device, mcode: *mut microcode) -> i32 {
    let dev = &(*(*cpt).pdev).dev; (*cpt).flags &= !CPT_FLAG_DEVICE_READY; cpt_disable_all_interrupts(cpt);
    if (*mcode).num_cores > if (*mcode).is_ae { (*cpt).max_ae_cores } else { (*cpt).max_se_cores } { dev_err(dev, "Requested for more cores than available cores\n"); return -EINVAL; }
    if (*cpt).next_group >= CPT_MAX_CORE_GROUPS { dev_err(dev, "Can't load, all eight microcode groups in use"); return -ENFILE; }
    (*mcode).group = (*cpt).next_group; (*mcode).core_mask = GENMASK((*mcode).num_cores, 0);
    let typ = if (*mcode).is_ae { AE_TYPES } else { SE_TYPES };
    cpt_disable_cores(cpt, (*mcode).core_mask, typ, (*mcode).group);
    let ret = cpt_load_microcode(cpt, mcode); if ret != 0 { return ret; }
    (*cpt).next_group += 1; cpt_configure_group(cpt, (*mcode).group, (*mcode).core_mask, typ); cpt_enable_cores(cpt, (*mcode).core_mask, typ);
    cpt_enable_mbox_interrupts(cpt); (*cpt).flags |= CPT_FLAG_DEVICE_READY; 0
}

#[repr(C)] struct ucode_header { version: [u8; CPT_UCODE_VERSION_SZ], code_length: u32, data_length: u32, sram_address: u64 }

unsafe fn cpt_ucode_load_fw(cpt: *mut cpt_device, fw: *const u8, is_ae: bool) -> i32 {
    let dev = &(*(*cpt).pdev).dev; let mut fw_entry: *const firmware = core::ptr::null(); let ret = request_firmware(&mut fw_entry, fw, dev); if ret != 0 { return ret; }
    let ucode = (*fw_entry).data as *mut ucode_header; let mcode = &mut (*cpt).mcode[(*cpt).next_mc_idx as usize];
    core::ptr::copy_nonoverlapping((*fw_entry).data, mcode.version.as_mut_ptr(), CPT_UCODE_VERSION_SZ);
    let code_length = u32::from_be((*ucode).code_length); if code_length == 0 || code_length >= (i32::MAX as u32)/2 { release_firmware(fw_entry); return -EINVAL; }
    mcode.code_size = code_length * 2; mcode.is_ae = is_ae; mcode.core_mask = 0; mcode.num_cores = if is_ae { 6 } else { 10 };
    mcode.code = dma_alloc_coherent(&(*cpt).pdev.dev, mcode.code_size, &mut mcode.phys_base, GFP_KERNEL); if mcode.code.is_null() { release_firmware(fw_entry); return -ENOMEM; }
    core::ptr::copy_nonoverlapping((*fw_entry).data.add(core::mem::size_of::<ucode_header>()), mcode.code as *mut u8, mcode.code_size as usize);
    let ret = do_cpt_init(cpt, mcode); if ret != 0 { dma_free_coherent(&(*cpt).pdev.dev, mcode.code_size, mcode.code, mcode.phys_base); release_firmware(fw_entry); return ret; }
    mcode.is_mc_valid = 1; (*cpt).next_mc_idx += 1; release_firmware(fw_entry); 0
}

unsafe fn cpt_ucode_load(cpt: *mut cpt_device) -> i32 { let r = cpt_ucode_load_fw(cpt, b"cpt8x-mc-ae.out\0".as_ptr(), true); if r != 0 { return r; } cpt_ucode_load_fw(cpt, b"cpt8x-mc-se.out\0".as_ptr(), false) }
unsafe fn cpt_mbx0_intr_handler(_irq: i32, cpt_irq: *mut core::ffi::c_void) -> irqreturn_t { cpt_mbox_intr_handler(cpt_irq as *mut cpt_device, 0); IRQ_HANDLED }
unsafe fn cpt_reset(cpt: *mut cpt_device) { cpt_write_csr64((*cpt).reg_base, CPTX_PF_RESET(0), 1); }
unsafe fn cpt_find_max_enabled_cores(cpt: *mut cpt_device) { let mut x: cptx_pf_constants = core::mem::zeroed(); x.u = cpt_read_csr64((*cpt).reg_base, CPTX_PF_CONSTANTS(0)); (*cpt).max_se_cores = x.s.se; (*cpt).max_ae_cores = x.s.ae; }
unsafe fn cpt_check_bist_status(cpt: *mut cpt_device) -> u32 { cpt_read_csr64((*cpt).reg_base, CPTX_PF_BIST_STATUS(0)) as u32 }
unsafe fn cpt_check_exe_bist_status(cpt: *mut cpt_device) -> u64 { cpt_read_csr64((*cpt).reg_base, CPTX_PF_EXE_BIST_STATUS(0)) }

unsafe fn cpt_disable_all_cores(cpt: *mut cpt_device) { for grp in 0..CPT_MAX_CORE_GROUPS { cpt_write_csr64((*cpt).reg_base, CPTX_PF_GX_EN(0, grp), 0); udelay(CSR_DELAY); } let mut timeout=100u32; while cpt_read_csr64((*cpt).reg_base, CPTX_PF_EXEC_BUSY(0)) != 0 { timeout=timeout.wrapping_sub(1); if timeout==u32::MAX { break; } udelay(CSR_DELAY); } cpt_write_csr64((*cpt).reg_base, CPTX_PF_EXE_CTL(0), 0); }
unsafe fn cpt_unload_microcode(cpt: *mut cpt_device) { for grp in 0..CPT_MAX_CORE_GROUPS { let m=&mut (*cpt).mcode[grp as usize]; if !m.code.is_null() { dma_free_coherent(&(*cpt).pdev.dev,m.code_size,m.code,m.phys_base); } m.code=core::ptr::null_mut(); } for core in 0..CPT_MAX_TOTAL_CORES { cpt_write_csr64((*cpt).reg_base,CPTX_PF_ENGX_UCODE_BASE(0,core),0); } }

unsafe fn cpt_device_init(cpt: *mut cpt_device) -> i32 {
    cpt_reset(cpt); msleep(100); if cpt_check_bist_status(cpt) != 0 || cpt_check_exe_bist_status(cpt) != 0 { return -ENODEV; }
    cpt_find_max_enabled_cores(cpt); cpt_disable_all_cores(cpt); (*cpt).next_mc_idx=0; (*cpt).next_group=0; (*cpt).flags|=CPT_FLAG_DEVICE_READY; 0
}
unsafe fn cpt_register_interrupts(cpt: *mut cpt_device) -> i32 {
    let ret=pci_alloc_irq_vectors((*cpt).pdev,CPT_PF_MSIX_VECTORS,CPT_PF_MSIX_VECTORS,PCI_IRQ_MSIX); if ret<0{return ret;}
    let ret=request_irq(pci_irq_vector((*cpt).pdev,CPT_PF_INT_VEC_E_MBOXX(0)),cpt_mbx0_intr_handler,0,b"CPT Mbox0\0".as_ptr(),cpt as *mut _); if ret!=0 { pci_disable_msix((*cpt).pdev); return ret; } cpt_enable_mbox_interrupts(cpt); 0
}
unsafe fn cpt_unregister_interrupts(cpt: *mut cpt_device) { free_irq(pci_irq_vector((*cpt).pdev,CPT_PF_INT_VEC_E_MBOXX(0)),cpt as *mut _); pci_disable_msix((*cpt).pdev); }
unsafe fn cpt_sriov_init(cpt: *mut cpt_device, mut requested: i32) -> i32 { let pos=pci_find_ext_capability((*cpt).pdev,PCI_EXT_CAP_ID_SRIOV); if pos==0{return -ENODEV;} (*cpt).num_vf_en=requested; let mut total=0u16; pci_read_config_word((*cpt).pdev,pos+PCI_SRIOV_TOTAL_VF,&mut total); if i32::from(total)<requested{(*cpt).num_vf_en=i32::from(total);} if total==0{return 0;} let e=pci_enable_sriov((*cpt).pdev,(*cpt).num_vf_en); if e!=0{(*cpt).num_vf_en=0;return e;} (*cpt).flags|=CPT_FLAG_SRIOV_ENABLED; 0 }
unsafe fn cpt_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 { if num_vfs>16||num_vfs<4{num_vfs=4;} let cpt=devm_kzalloc(&(*pdev).dev,core::mem::size_of::<cpt_device>(),GFP_KERNEL) as *mut cpt_device; if cpt.is_null(){return -ENOMEM;} pci_set_drvdata(pdev,cpt as *mut _);(*cpt).pdev=pdev; let mut e=pci_enable_device(pdev); if e!=0{return e;} e=pci_request_regions(pdev,DRV_NAME.as_ptr() as *const _); if e!=0{return e;} (*cpt).reg_base=pcim_iomap(pdev,0,0); if (*cpt).reg_base.is_null(){return -ENOMEM;} cpt_device_init(cpt); e=cpt_register_interrupts(cpt); if e!=0{return e;} e=cpt_ucode_load(cpt); if e!=0{return e;} cpt_sriov_init(cpt,num_vfs) }
unsafe fn cpt_remove(pdev: *mut pci_dev) { let cpt=pci_get_drvdata(pdev) as *mut cpt_device; cpt_disable_all_cores(cpt); cpt_unload_microcode(cpt); cpt_unregister_interrupts(cpt); pci_disable_sriov(pdev); pci_release_regions(pdev); pci_disable_device(pdev); pci_set_drvdata(pdev,core::ptr::null_mut()); }
unsafe fn cpt_shutdown(pdev: *mut pci_dev) { let cpt=pci_get_drvdata(pdev) as *mut cpt_device; if cpt.is_null(){return;} cpt_unregister_interrupts(cpt); pci_release_regions(pdev); pci_disable_device(pdev); pci_set_drvdata(pdev,core::ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
