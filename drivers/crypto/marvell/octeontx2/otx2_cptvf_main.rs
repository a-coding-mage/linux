// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 Marvell. */

// Dependencies supplied by the surrounding kernel/Rust translation environment.

const OTX2_CPTVF_DRV_NAME: &str = "rvu_cptvf";

unsafe fn cptvf_enable_pfvf_mbox_intrs(cptvf: *mut otx2_cptvf_dev) {
    /* Clear interrupt if any */
    otx2_cpt_write64((*cptvf).reg_base, BLKADDR_RVUM, 0, OTX2_RVU_VF_INT,
                     0x1u64);

    /* Enable PF-VF interrupt */
    otx2_cpt_write64((*cptvf).reg_base, BLKADDR_RVUM, 0,
                     OTX2_RVU_VF_INT_ENA_W1S, 0x1u64);
}

unsafe fn cptvf_disable_pfvf_mbox_intrs(cptvf: *mut otx2_cptvf_dev) {
    /* Disable PF-VF interrupt */
    otx2_cpt_write64((*cptvf).reg_base, BLKADDR_RVUM, 0,
                     OTX2_RVU_VF_INT_ENA_W1C, 0x1u64);

    /* Clear interrupt if any */
    otx2_cpt_write64((*cptvf).reg_base, BLKADDR_RVUM, 0, OTX2_RVU_VF_INT,
                     0x1u64);
}

unsafe fn cptvf_register_interrupts(cptvf: *mut otx2_cptvf_dev) -> i32 {
    let num_vec = pci_msix_vec_count((*cptvf).pdev);
    if num_vec <= 0 { return -EINVAL; }

    /* Enable MSI-X */
    let ret = pci_alloc_irq_vectors((*cptvf).pdev, num_vec, num_vec, PCI_IRQ_MSIX);
    if ret < 0 {
        dev_err(&(*(*cptvf).pdev).dev, "Request for %d msix vectors failed\n", num_vec);
        return ret;
    }
    let irq = pci_irq_vector((*cptvf).pdev, OTX2_CPT_VF_INT_VEC_E_MBOX);
    /* Register VF<=>PF mailbox interrupt handler */
    let ret = devm_request_irq(&(*(*cptvf).pdev).dev, irq,
                               otx2_cptvf_pfvf_mbox_intr, 0,
                               "CPTPFVF Mbox", cptvf);
    if ret != 0 { return ret; }
    /* Enable PF-VF mailbox interrupts */
    cptvf_enable_pfvf_mbox_intrs(cptvf);

    let ret = otx2_cpt_send_ready_msg(&mut (*cptvf).pfvf_mbox, (*cptvf).pdev);
    if ret != 0 {
        dev_warn(&(*(*cptvf).pdev).dev,
                 "PF not responding to mailbox, deferring probe\n");
        cptvf_disable_pfvf_mbox_intrs(cptvf);
        return -EPROBE_DEFER;
    }
    0
}

unsafe fn cptvf_pfvf_mbox_init(cptvf: *mut otx2_cptvf_dev) -> i32 {
    let pdev = (*cptvf).pdev;
    let mut ret;
    (*cptvf).pfvf_mbox_wq = alloc_ordered_workqueue("cpt_pfvf_mailbox", WQ_HIGHPRI | WQ_MEM_RECLAIM);
    if (*cptvf).pfvf_mbox_wq.is_null() { return -ENOMEM; }

    if test_bit(CN10K_MBOX, &(*cptvf).cap_flag) {
        /* For cn10k platform, VF mailbox region is in its BAR2
         * register space
         */
        (*cptvf).pfvf_mbox_base = (*cptvf).reg_base + CN10K_CPT_VF_MBOX_REGION;
    } else {
        let offset = pci_resource_start(pdev, PCI_MBOX_BAR_NUM);
        let size = pci_resource_len(pdev, PCI_MBOX_BAR_NUM);
        /* Map PF-VF mailbox memory */
        (*cptvf).pfvf_mbox_base = devm_ioremap_wc(&(*pdev).dev, offset, size);
        if (*cptvf).pfvf_mbox_base.is_null() {
            dev_err(&(*pdev).dev, "Unable to map BAR4\n");
            ret = -ENOMEM;
            goto free_wqe;
        }
    }

    ret = otx2_mbox_init(&mut (*cptvf).pfvf_mbox, (*cptvf).pfvf_mbox_base,
                         pdev, (*cptvf).reg_base, MBOX_DIR_VFPF, 1);
    if ret != 0 { goto free_wqe; }
    ret = otx2_cpt_mbox_bbuf_init(cptvf, pdev);
    if ret != 0 { goto destroy_mbox; }
    INIT_WORK(&mut (*cptvf).pfvf_mbox_work, otx2_cptvf_pfvf_mbox_handler);
    return 0;

destroy_mbox:
    otx2_mbox_destroy(&mut (*cptvf).pfvf_mbox);
free_wqe:
    destroy_workqueue((*cptvf).pfvf_mbox_wq);
    ret
}

unsafe fn cptvf_pfvf_mbox_destroy(cptvf: *mut otx2_cptvf_dev) {
    destroy_workqueue((*cptvf).pfvf_mbox_wq);
    otx2_mbox_destroy(&mut (*cptvf).pfvf_mbox);
}

unsafe fn cptlf_work_handler(data: usize) {
    otx2_cpt_post_process(data as *mut otx2_cptlf_wqe);
}

unsafe fn cleanup_tasklet_work(lfs: *mut otx2_cptlfs_info) {
    for i in 0..(*lfs).lfs_num {
        if (*lfs).lf[i].wqe.is_null() { continue; }
        tasklet_kill(&mut (*(*lfs).lf[i].wqe).work);
        kfree((*lfs).lf[i].wqe);
        (*lfs).lf[i].wqe = core::ptr::null_mut();
    }
}

unsafe fn init_tasklet_work(lfs: *mut otx2_cptlfs_info) -> i32 {
    for i in 0..(*lfs).lfs_num {
        let wqe = kzalloc_obj::<otx2_cptlf_wqe>();
        if wqe.is_null() { cleanup_tasklet_work(lfs); return -ENOMEM; }
        tasklet_init(&mut (*wqe).work, cptlf_work_handler, wqe as u64);
        (*wqe).lfs = lfs;
        (*wqe).lf_num = i;
        (*lfs).lf[i].wqe = wqe;
    }
    0
}

unsafe fn free_pending_queues(lfs: *mut otx2_cptlfs_info) {
    for i in 0..(*lfs).lfs_num {
        kfree((*lfs).lf[i].pqueue.head);
        (*lfs).lf[i].pqueue.head = core::ptr::null_mut();
    }
}

unsafe fn alloc_pending_queues(lfs: *mut otx2_cptlfs_info) -> i32 {
    if (*lfs).lfs_num == 0 { return -EINVAL; }
    for i in 0..(*lfs).lfs_num {
        (*lfs).lf[i].pqueue.qlen = OTX2_CPT_INST_QLEN_MSGS;
        let size = (*lfs).lf[i].pqueue.qlen * core::mem::size_of::<otx2_cpt_pending_entry>();
        (*lfs).lf[i].pqueue.head = kzalloc(size, GFP_KERNEL);
        if (*lfs).lf[i].pqueue.head.is_null() { free_pending_queues(lfs); return -ENOMEM; }
        spin_lock_init(&mut (*lfs).lf[i].pqueue.lock);
    }
    0
}

unsafe fn lf_sw_cleanup(lfs: *mut otx2_cptlfs_info) {
    cleanup_tasklet_work(lfs);
    free_pending_queues(lfs);
}

unsafe fn lf_sw_init(lfs: *mut otx2_cptlfs_info) -> i32 {
    let mut ret = alloc_pending_queues(lfs);
    if ret != 0 { dev_err(&(*(*lfs).pdev).dev, "Allocating pending queues failed\n"); return ret; }
    ret = init_tasklet_work(lfs);
    if ret != 0 { dev_err(&(*(*lfs).pdev).dev, "Tasklet work init failed\n"); free_pending_queues(lfs); }
    ret
}

unsafe fn cptvf_lf_shutdown(lfs: *mut otx2_cptlfs_info) {
    atomic_set(&mut (*lfs).state, OTX2_CPTLF_IN_RESET);
    /* Remove interrupts affinity */
    otx2_cptlf_free_irqs_affinity(lfs);
    /* Disable instruction queue */
    otx2_cptlf_disable_iqueues(lfs);
    /* Unregister crypto algorithms */
    otx2_cpt_crypto_exit((*lfs).pdev, THIS_MODULE);
    /* Unregister LFs interrupts */
    otx2_cptlf_unregister_misc_interrupts(lfs);
    otx2_cptlf_unregister_done_interrupts(lfs);
    /* Cleanup LFs software side */
    lf_sw_cleanup(lfs);
    /* Free instruction queues */
    otx2_cpt_free_instruction_queues(lfs);
    /* Send request to detach LFs */
    otx2_cpt_detach_rsrcs_msg(lfs);
    (*lfs).lfs_num = 0;
}

unsafe fn cptvf_lf_init(cptvf: *mut otx2_cptvf_dev) -> i32 {
    let lfs = &mut (*cptvf).lfs as *mut otx2_cptlfs_info;
    let dev = &(*(*cptvf).pdev).dev;
    (*lfs).kcrypto_se_eng_grp_num = OTX2_CPT_INVALID_CRYPTO_ENG_GRP;
    let mut ret = otx2_cptvf_send_eng_grp_num_msg(cptvf, OTX2_CPT_SE_TYPES);
    if ret != 0 { return ret; }
    if (*lfs).kcrypto_se_eng_grp_num == OTX2_CPT_INVALID_CRYPTO_ENG_GRP {
        dev_err(dev, "Symmetric Engine group for crypto not available\n"); return -ENOENT;
    }
    (*lfs).kcrypto_ae_eng_grp_num = OTX2_CPT_INVALID_CRYPTO_ENG_GRP;
    ret = otx2_cptvf_send_eng_grp_num_msg(cptvf, OTX2_CPT_AE_TYPES);
    if ret != 0 { return ret; }
    if (*lfs).kcrypto_ae_eng_grp_num == OTX2_CPT_INVALID_CRYPTO_ENG_GRP {
        dev_err(dev, "Asymmetric Engine group for crypto not available\n"); return -ENOENT;
    }
    let eng_grp_msk = BIT((*lfs).kcrypto_se_eng_grp_num) | BIT((*lfs).kcrypto_ae_eng_grp_num);
    ret = otx2_cptvf_send_kvf_limits_msg(cptvf);
    if ret != 0 { return ret; }
    let lfs_num = (*lfs).kvf_limits;
    ret = otx2_cptlf_init(lfs, eng_grp_msk, OTX2_CPT_QUEUE_HI_PRIO, lfs_num);
    if ret != 0 { return ret; }
    ret = otx2_cpt_msix_offset_msg(lfs); if ret != 0 { goto cleanup_lf; }
    ret = lf_sw_init(lfs); if ret != 0 { goto cleanup_lf; }
    ret = otx2_cptlf_register_misc_interrupts(lfs); if ret != 0 { goto cleanup_lf; }
    ret = otx2_cptlf_register_done_interrupts(lfs); if ret != 0 { goto cleanup_lf_sw; }
    ret = otx2_cptlf_set_irqs_affinity(lfs); if ret != 0 { goto unregister_intr; }
    atomic_set(&mut (*lfs).state, OTX2_CPTLF_STARTED);
    ret = otx2_cpt_crypto_init((*lfs).pdev, THIS_MODULE, lfs_num, 1);
    if ret != 0 { dev_err(&(*(*lfs).pdev).dev, "algorithms registration failed\n"); goto disable_irqs; }
    return 0;
disable_irqs:
    otx2_cptlf_free_irqs_affinity(lfs);
unregister_intr:
    otx2_cptlf_unregister_misc_interrupts(lfs);
    otx2_cptlf_unregister_done_interrupts(lfs);
cleanup_lf_sw:
    lf_sw_cleanup(lfs);
cleanup_lf:
    otx2_cptlf_shutdown(lfs);
    ret
}

unsafe fn otx2_cptvf_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let dev = &(*pdev).dev;
    let cptvf = devm_kzalloc(dev, core::mem::size_of::<otx2_cptvf_dev>(), GFP_KERNEL) as *mut otx2_cptvf_dev;
    if cptvf.is_null() { return -ENOMEM; }
    let mut ret = pcim_enable_device(pdev);
    if ret != 0 { dev_err(dev, "Failed to enable PCI device\n"); goto clear_drvdata; }
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(48));
    if ret != 0 { dev_err(dev, "Unable to get usable DMA configuration\n"); goto clear_drvdata; }
    ret = pcim_request_all_regions(pdev, OTX2_CPTVF_DRV_NAME);
    if ret != 0 { dev_err(dev, "Couldn't get PCI resources 0x%x\n", ret); goto clear_drvdata; }
    pci_set_master(pdev); pci_set_drvdata(pdev, cptvf); (*cptvf).pdev = pdev;
    (*cptvf).reg_base = pcim_iomap(pdev, PCI_PF_REG_BAR_NUM, 0);
    if (*cptvf).reg_base.is_null() { ret = -ENOMEM; dev_err(dev, "Couldn't ioremap PCI resource 0x%x\n", ret); goto clear_drvdata; }
    otx2_cpt_set_hw_caps(pdev, &mut (*cptvf).cap_flag);
    ret = cptvf_pfvf_mbox_init(cptvf); if ret != 0 { goto clear_drvdata; }
    ret = cptvf_register_interrupts(cptvf); if ret != 0 { goto destroy_pfvf_mbox; }
    (*cptvf).blkaddr = BLKADDR_CPT0;
    cptvf_hw_ops_get(cptvf);
    otx2_cptlf_set_dev_info(&mut (*cptvf).lfs, (*cptvf).pdev, (*cptvf).reg_base, &mut (*cptvf).pfvf_mbox, (*cptvf).blkaddr);
    ret = otx2_cptvf_send_caps_msg(cptvf);
    if ret != 0 { dev_err(&(*pdev).dev, "Couldn't get CPT engine capabilities.\n"); goto unregister_interrupts; }
    if (*cptvf).eng_caps[OTX2_CPT_SE_TYPES] & BIT_ULL(35) != 0 { (*cptvf).lfs.ops.cpt_sg_info_create = cn10k_sgv2_info_create; }
    ret = cn10k_cptvf_lmtst_init(cptvf); if ret != 0 { goto unregister_interrupts; }
    ret = cptvf_lf_init(cptvf); if ret != 0 { goto free_lmtst; }
    return 0;
free_lmtst:
    cn10k_cpt_lmtst_free(pdev, &mut (*cptvf).lfs);
unregister_interrupts:
    cptvf_disable_pfvf_mbox_intrs(cptvf);
destroy_pfvf_mbox:
    cptvf_pfvf_mbox_destroy(cptvf);
clear_drvdata:
    pci_set_drvdata(pdev, core::ptr::null_mut());
    ret
}

unsafe fn otx2_cptvf_remove(pdev: *mut pci_dev) {
    let cptvf = pci_get_drvdata(pdev) as *mut otx2_cptvf_dev;
    if cptvf.is_null() { dev_err(&(*pdev).dev, "Invalid CPT VF device.\n"); return; }
    cptvf_lf_shutdown(&mut (*cptvf).lfs);
    /* Disable PF-VF mailbox interrupt */
    cptvf_disable_pfvf_mbox_intrs(cptvf);
    /* Destroy PF-VF mbox */
    cptvf_pfvf_mbox_destroy(cptvf);
    /* Free LMTST memory */
    cn10k_cpt_lmtst_free(pdev, &mut (*cptvf).lfs);
    pci_set_drvdata(pdev, core::ptr::null_mut());
}

/* Supported devices */
static OTX2_CPTVF_ID_TABLE: [pci_device_id; 3] = [
    PCI_VDEVICE!(CAVIUM, OTX2_CPT_PCI_VF_DEVICE_ID),
    PCI_VDEVICE!(CAVIUM, CN10K_CPT_PCI_VF_DEVICE_ID),
    pci_device_id::default(), /* end of table */
];

static mut otx2_cptvf_pci_driver: pci_driver = pci_driver {
    name: OTX2_CPTVF_DRV_NAME,
    id_table: OTX2_CPTVF_ID_TABLE.as_ptr(),
    probe: Some(otx2_cptvf_probe),
    remove: Some(otx2_cptvf_remove),
};

// module_pci_driver(otx2_cptvf_pci_driver);
// MODULE_IMPORT_NS("CRYPTO_DEV_OCTEONTX2_CPT");
// MODULE_AUTHOR("Marvell");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
