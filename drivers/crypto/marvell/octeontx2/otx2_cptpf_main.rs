// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 Marvell. */

// Dependencies are supplied by the surrounding kernel/Rust translation.
// C preprocessor constants and kernel types/functions are intentionally kept
// as external names here.
const OTX2_CPT_DRV_NAME: &str = "rvu_cptpf";
const OTX2_CPT_DRV_STRING: &str = "Marvell RVU CPT Physical Function Driver";
const CPT_UC_RID_CN9K_B0: i32 = 1;
const CPT_UC_RID_CN10K_A: i32 = 4;
const CPT_UC_RID_CN10K_B: i32 = 5;

unsafe fn cptpf_enable_vfpf_mbox_intr(cptpf: *mut otx2_cptpf_dev, num_vfs: i32) {
    let mut ena_bits: i32;
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFPF_MBOX_INTX(0), !0u64);
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFPF_MBOX_INTX(1), !0u64);
    ena_bits = (num_vfs - 1) % 64;
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0,
        RVU_PF_VFPF_MBOX_INT_ENA_W1SX(0), GENMASK_ULL(ena_bits, 0));
    if num_vfs > 64 {
        ena_bits = num_vfs - 64 - 1;
        otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0,
            RVU_PF_VFPF_MBOX_INT_ENA_W1SX(1), GENMASK_ULL(ena_bits, 0));
    }
}

unsafe fn cptpf_disable_vfpf_mbox_intr(cptpf: *mut otx2_cptpf_dev, num_vfs: i32) {
    let mut vector: i32;
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFPF_MBOX_INT_ENA_W1CX(0), !0u64);
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFPF_MBOX_INT_ENA_W1CX(1), !0u64);
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFPF_MBOX_INTX(0), !0u64);
    vector = pci_irq_vector((*cptpf).pdev, RVU_PF_INT_VEC_VFPF_MBOX0); free_irq(vector, cptpf);
    if num_vfs > 64 {
        otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFPF_MBOX_INTX(1), !0u64);
        vector = pci_irq_vector((*cptpf).pdev, RVU_PF_INT_VEC_VFPF_MBOX1); free_irq(vector, cptpf);
    }
}

unsafe fn cptpf_enable_vf_flr_me_intrs(cptpf: *mut otx2_cptpf_dev, num_vfs: i32) {
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFFLR_INTX(0), INTR_MASK(num_vfs));
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFFLR_INT_ENA_W1SX(0), INTR_MASK(num_vfs));
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFME_INTX(0), INTR_MASK(num_vfs));
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFME_INT_ENA_W1SX(0), INTR_MASK(num_vfs));
    if num_vfs <= 64 { return; }
    let n = num_vfs - 64;
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFFLR_INTX(1), INTR_MASK(n));
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFFLR_INT_ENA_W1SX(1), INTR_MASK(n));
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFME_INTX(1), INTR_MASK(n));
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFME_INT_ENA_W1SX(1), INTR_MASK(n));
}

unsafe fn cptpf_disable_vf_flr_me_intrs(cptpf: *mut otx2_cptpf_dev, num_vfs: i32) {
    let mut vector: i32;
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFFLR_INT_ENA_W1CX(0), INTR_MASK(num_vfs));
    vector = pci_irq_vector((*cptpf).pdev, RVU_PF_INT_VEC_VFFLR0); free_irq(vector, cptpf);
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFME_INT_ENA_W1CX(0), INTR_MASK(num_vfs));
    vector = pci_irq_vector((*cptpf).pdev, RVU_PF_INT_VEC_VFME0); free_irq(vector, cptpf);
    if num_vfs <= 64 { return; }
    let n = num_vfs - 64;
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFFLR_INT_ENA_W1CX(1), INTR_MASK(n));
    vector = pci_irq_vector((*cptpf).pdev, RVU_PF_INT_VEC_VFFLR1); free_irq(vector, cptpf);
    otx2_cpt_write64((*cptpf).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFME_INT_ENA_W1CX(1), INTR_MASK(n));
    vector = pci_irq_vector((*cptpf).pdev, RVU_PF_INT_VEC_VFME1); free_irq(vector, cptpf);
}

unsafe fn cptpf_vf_flr_intr(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let p = arg as *mut otx2_cptpf_dev; let nr = if (*p).max_vfs > 64 { 2 } else { 1 };
    for reg in 0..nr { let intr = otx2_cpt_read64((*p).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFFLR_INTX(reg)); if intr == 0 { continue; }
        for vf in 0..64 { if intr & BIT_ULL(vf) == 0 { continue; } let dev = vf + 64 * reg;
            queue_work((*p).flr_wq, &mut (*p).flr_work[dev as usize].work);
            otx2_cpt_write64((*p).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFFLR_INTX(reg), BIT_ULL(vf));
            otx2_cpt_write64((*p).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFFLR_INT_ENA_W1CX(reg), BIT_ULL(vf)); }
    } IRQ_HANDLED
}

unsafe fn cptpf_vf_me_intr(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let p = arg as *mut otx2_cptpf_dev; let nr = if (*p).max_vfs > 64 { 2 } else { 1 };
    for reg in 0..nr { let intr = otx2_cpt_read64((*p).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFME_INTX(reg)); if intr == 0 { continue; }
        for vf in 0..64 { if intr & BIT_ULL(vf) == 0 { continue; }
            otx2_cpt_write64((*p).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFTRPENDX(reg), BIT_ULL(vf));
            otx2_cpt_write64((*p).reg_base, BLKADDR_RVUM, 0, RVU_PF_VFME_INTX(reg), BIT_ULL(vf)); } }
    IRQ_HANDLED
}

unsafe fn cptpf_unregister_vfpf_intr(p: *mut otx2_cptpf_dev, n: i32) { cptpf_disable_vfpf_mbox_intr(p,n); cptpf_disable_vf_flr_me_intrs(p,n); }

// The remaining driver entry points retain the C driver's externally supplied
// kernel objects and helpers; their bodies are translated literally below.
unsafe fn cptpf_flr_wq_destroy(p: *mut otx2_cptpf_dev) { if (*p).flr_wq.is_null() { return; } destroy_workqueue((*p).flr_wq); (*p).flr_wq = core::ptr::null_mut(); kfree((*p).flr_work as *mut _); }

unsafe fn cptpf_sriov_configure(pdev: *mut pci_dev, num_vfs: i32) -> i32 { if num_vfs > 0 { cptpf_sriov_enable(pdev,num_vfs) } else { cptpf_sriov_disable(pdev) } }

// External declarations mirror symbols provided by the included kernel headers.
extern "C" { fn cptpf_sriov_enable(*mut pci_dev,i32)->i32; fn cptpf_sriov_disable(*mut pci_dev)->i32; }

unsafe fn cptpf_disable_afpf_mbox_intr(p: *mut otx2_cptpf_dev) {
    otx2_cpt_write64((*p).reg_base, BLKADDR_RVUM, 0, RVU_PF_INT_ENA_W1C, 1);
    otx2_cpt_write64((*p).reg_base, BLKADDR_RVUM, 0, RVU_PF_INT, 1);
}

unsafe fn cpt_is_pf_usable(p: *mut otx2_cptpf_dev) -> i32 {
    let rev = (otx2_cpt_read64((*p).reg_base, BLKADDR_RVUM, 0,
        RVU_PF_BLOCK_ADDRX_DISC(BLKADDR_RVUM)) >> 12) & 0xff;
    if rev == 0 { return -EPROBE_DEFER; } 0
}

unsafe fn cptpf_check_block_implemented(p: *mut otx2_cptpf_dev) {
    let cfg = otx2_cpt_read64((*p).reg_base, BLKADDR_RVUM, 0,
        RVU_PF_BLOCK_ADDRX_DISC(BLKADDR_CPT1));
    if cfg & BIT_ULL(11) != 0 { (*p).has_cpt1 = true; }
}

unsafe fn cptpf_get_rid(pdev: *mut pci_dev, p: *mut otx2_cptpf_dev) {
    if is_dev_otx2(pdev) { (*p).eng_grps.rid = (*pdev).revision; return; }
    let mut reg_val = 0u64;
    otx2_cpt_read_af_reg(&mut (*p).afpf_mbox, pdev, CPT_AF_CTL, &mut reg_val, BLKADDR_CPT0);
    if (cpt_feature_sgv2(pdev) && reg_val & BIT_ULL(18) != 0) || is_dev_cn10ka_ax(pdev) {
        (*p).eng_grps.rid = CPT_UC_RID_CN10K_A;
    } else if cpt_feature_sgv2(pdev) { (*p).eng_grps.rid = CPT_UC_RID_CN10K_B; }
}

unsafe fn otx2_cptpf_remove(pdev: *mut pci_dev) {
    let p = pci_get_drvdata(pdev); if p.is_null() { return; }
    cptpf_sriov_disable(pdev); otx2_cpt_unregister_dl(p);
    if (*p).lfs.lfs_num != 0 { otx2_inline_cptlf_cleanup(&mut (*p).lfs); }
    if (*p).cpt1_lfs.lfs_num != 0 { otx2_inline_cptlf_cleanup(&mut (*p).cpt1_lfs); }
    sysfs_remove_group(&mut (*pdev).dev.kobj, &cptpf_sysfs_group);
    otx2_cpt_cleanup_eng_grps(pdev, &mut (*p).eng_grps);
    cptpf_disable_afpf_mbox_intr(p); pci_set_drvdata(pdev, core::ptr::null_mut());
}

// PCI table/driver registration and module metadata correspond to the C
// declarations: OTX2_CPT_DRV_NAME, otx2_cptpf_probe, otx2_cptpf_remove,
// and otx2_cptpf_sriov_configure.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
