// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2020 IBM Corporation
 *
 * Author: Ashley Lai <ashleydlai@gmail.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * Specifications at www.trustedcomputinggroup.org
 */

// Kernel dependencies supplied by the surrounding translation unit.

static TPM_IBMVTPM_DRIVER_NAME: &[u8] = b"tpm_ibmvtpm\0";

static TPM_IBMVTPM_DEVICE_TABLE: [vio_device_id; 3] = [
    vio_device_id { type_: b"IBM,vtpm\0", compat: b"IBM,vtpm\0" },
    vio_device_id { type_: b"IBM,vtpm\0", compat: b"IBM,vtpm20\0" },
    vio_device_id { type_: b"\0", compat: b"\0" },
];

unsafe fn ibmvtpm_send_crq_word(vdev: *mut vio_dev, w1: u64) -> i32 {
    plpar_hcall_norets(H_SEND_CRQ, (*vdev).unit_address, w1, 0)
}

unsafe fn ibmvtpm_send_crq(vdev: *mut vio_dev, valid: u8, msg: u8, len: u16, data: u32) -> i32 {
    let w1 = ((valid as u64) << 56) | ((msg as u64) << 48) | ((len as u64) << 32) | data as u64;
    ibmvtpm_send_crq_word(vdev, w1)
}

unsafe fn tpm_ibmvtpm_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> i32 {
    let ibmvtpm = dev_get_drvdata((*chip).dev) as *mut ibmvtpm_dev;
    if (*ibmvtpm).rtce_buf.is_null() { dev_err((*ibmvtpm).dev, "ibmvtpm device is not ready\n"); return 0; }
    let len = (*ibmvtpm).res_len;
    if count < len as usize { dev_err((*ibmvtpm).dev, "Invalid size in recv: count=%zd, crq_size=%d\n", count, len); return -EIO; }
    spin_lock(&mut (*ibmvtpm).rtce_lock);
    memcpy(buf as *mut _, (*ibmvtpm).rtce_buf as *const _, len as usize);
    memset((*ibmvtpm).rtce_buf, 0, len as usize);
    (*ibmvtpm).res_len = 0;
    spin_unlock(&mut (*ibmvtpm).rtce_lock);
    len as i32
}

unsafe fn ibmvtpm_crq_send_init(ibmvtpm: *mut ibmvtpm_dev) -> i32 {
    let rc = ibmvtpm_send_crq_word((*ibmvtpm).vdev, INIT_CRQ_CMD);
    if rc != H_SUCCESS { dev_err((*ibmvtpm).dev, "%s failed rc=%d\n", "ibmvtpm_crq_send_init", rc); }
    rc
}

unsafe fn tpm_ibmvtpm_resume(dev: *mut device) -> i32 {
    let chip = dev_get_drvdata(dev) as *mut tpm_chip;
    let ibmvtpm = dev_get_drvdata((*chip).dev) as *mut ibmvtpm_dev;
    let mut rc = 0;
    loop { if rc != 0 { msleep(100); } rc = plpar_hcall_norets(H_ENABLE_CRQ, (*ibmvtpm).vdev.as_ref().unwrap().unit_address); if !(rc == H_IN_PROGRESS || rc == H_BUSY || H_IS_LONG_BUSY(rc)) { break; } }
    if rc != 0 { dev_err(dev, "Error enabling ibmvtpm rc=%d\n", rc); return rc; }
    rc = vio_enable_interrupts((*ibmvtpm).vdev); if rc != 0 { dev_err(dev, "Error vio_enable_interrupts rc=%d\n", rc); return rc; }
    rc = ibmvtpm_crq_send_init(ibmvtpm); if rc != 0 { dev_err(dev, "Error send_init rc=%d\n", rc); } rc
}

unsafe fn tpm_ibmvtpm_send(chip: *mut tpm_chip, buf: *mut u8, _bufsiz: usize, count: usize) -> i32 {
    let ibmvtpm = dev_get_drvdata((*chip).dev) as *mut ibmvtpm_dev;
    if (*ibmvtpm).rtce_buf.is_null() { dev_err((*ibmvtpm).dev, "ibmvtpm device is not ready\n"); return 0; }
    if count > (*ibmvtpm).rtce_size as usize { dev_err((*ibmvtpm).dev, "Invalid size in send: count=%zd, rtce_size=%d\n", count, (*ibmvtpm).rtce_size); return -EIO; }
    if (*ibmvtpm).tpm_processing_cmd != 0 { dev_info((*ibmvtpm).dev, "Need to wait for TPM to finish\n"); let sig = wait_event_interruptible(&mut (*ibmvtpm).wq, (*ibmvtpm).tpm_processing_cmd == 0); if sig != 0 { return -EINTR; } }
    spin_lock(&mut (*ibmvtpm).rtce_lock); (*ibmvtpm).res_len = 0; memcpy((*ibmvtpm).rtce_buf as *mut _, buf as *const _, count); (*ibmvtpm).tpm_processing_cmd = 1;
    let mut retry = true;
    loop { let rc = ibmvtpm_send_crq((*ibmvtpm).vdev, IBMVTPM_VALID_CMD, VTPM_TPM_COMMAND, count as u16, (*ibmvtpm).rtce_dma_handle); if rc == H_SUCCESS { break; } if rc == H_CLOSED && retry { tpm_ibmvtpm_resume((*ibmvtpm).dev); retry = false; continue; } dev_err((*ibmvtpm).dev, "tpm_ibmvtpm_send failed rc=%d\n", rc); (*ibmvtpm).tpm_processing_cmd = 0; break; }
    spin_unlock(&mut (*ibmvtpm).rtce_lock); 0
}

unsafe fn tpm_ibmvtpm_cancel(_chip: *mut tpm_chip) {}
unsafe fn tpm_ibmvtpm_status(chip: *mut tpm_chip) -> u8 { let d = dev_get_drvdata((*chip).dev) as *mut ibmvtpm_dev; (*d).tpm_processing_cmd }

unsafe fn ibmvtpm_crq_get_rtce_size(d: *mut ibmvtpm_dev) -> i32 { let rc = ibmvtpm_send_crq((*d).vdev, IBMVTPM_VALID_CMD, VTPM_GET_RTCE_BUFFER_SIZE, 0, 0); if rc != H_SUCCESS { dev_err((*d).dev, "ibmvtpm_crq_get_rtce_size failed rc=%d\n", rc); } rc }
unsafe fn ibmvtpm_crq_get_version(d: *mut ibmvtpm_dev) -> i32 { let rc = ibmvtpm_send_crq((*d).vdev, IBMVTPM_VALID_CMD, VTPM_GET_VERSION, 0, 0); if rc != H_SUCCESS { dev_err((*d).dev, "ibmvtpm_crq_get_version failed rc=%d\n", rc); } rc }
unsafe fn ibmvtpm_crq_send_init_complete(d: *mut ibmvtpm_dev) -> i32 { let rc = ibmvtpm_send_crq_word((*d).vdev, INIT_CRQ_COMP_CMD); if rc != H_SUCCESS { dev_err((*d).dev, "ibmvtpm_crq_send_init_complete failed rc=%d\n", rc); } rc }

unsafe fn tpm_ibmvtpm_req_canceled(_chip: *mut tpm_chip, status: u8) -> bool { status == 0 }

unsafe fn tpm_ibmvtpm_remove(vdev: *mut vio_dev) {
    let chip = dev_get_drvdata((*vdev).dev) as *mut tpm_chip;
    let d = dev_get_drvdata((*chip).dev) as *mut ibmvtpm_dev;
    tpm_chip_unregister(chip); free_irq((*vdev).irq, d);
    let mut rc = 0; loop { if rc != 0 { msleep(100); } rc = plpar_hcall_norets(H_FREE_CRQ, (*vdev).unit_address); if !(rc == H_BUSY || H_IS_LONG_BUSY(rc)) { break; } }
    dma_unmap_single((*d).dev, (*d).crq_dma_handle, CRQ_RES_BUF_SIZE, DMA_BIDIRECTIONAL); free_page((*d).crq_queue.crq_addr as usize);
    if !(*d).rtce_buf.is_null() { dma_unmap_single((*d).dev, (*d).rtce_dma_handle, (*d).rtce_size, DMA_BIDIRECTIONAL); kfree((*d).rtce_buf); }
    kfree(d as *mut _); dev_set_drvdata((*vdev).dev, core::ptr::null_mut());
}

unsafe fn tpm_ibmvtpm_get_desired_dma(vdev: *mut vio_dev) -> usize {
    let chip = dev_get_drvdata((*vdev).dev) as *mut tpm_chip;
    if chip.is_null() { return CRQ_RES_BUF_SIZE + PAGE_SIZE; }
    let d = dev_get_drvdata((*chip).dev) as *mut ibmvtpm_dev; CRQ_RES_BUF_SIZE + (*d).rtce_size as usize
}

unsafe fn tpm_ibmvtpm_suspend(dev: *mut device) -> i32 {
    let chip = dev_get_drvdata(dev) as *mut tpm_chip; let d = dev_get_drvdata((*chip).dev) as *mut ibmvtpm_dev;
    let rc = ibmvtpm_send_crq((*d).vdev, IBMVTPM_VALID_CMD, VTPM_PREPARE_TO_SUSPEND, 0, 0); if rc != H_SUCCESS { dev_err((*d).dev, "tpm_ibmvtpm_suspend failed rc=%d\n", rc); } rc
}

unsafe fn ibmvtpm_reset_crq(d: *mut ibmvtpm_dev) -> i32 {
    let mut rc = 0; loop { if rc != 0 { msleep(100); } rc = plpar_hcall_norets(H_FREE_CRQ, (*d).vdev.as_ref().unwrap().unit_address); if !(rc == H_BUSY || H_IS_LONG_BUSY(rc)) { break; } }
    memset((*d).crq_queue.crq_addr as *mut _, 0, CRQ_RES_BUF_SIZE); (*d).crq_queue.index = 0;
    plpar_hcall_norets(H_REG_CRQ, (*d).vdev.as_ref().unwrap().unit_address, (*d).crq_dma_handle, CRQ_RES_BUF_SIZE)
}

unsafe fn ibmvtpm_crq_get_next(d: *mut ibmvtpm_dev) -> *mut ibmvtpm_crq { let q = &mut (*d).crq_queue; let c = &mut q.crq_addr[q.index as usize]; if c.valid & VTPM_MSG_RES != 0 { q.index += 1; if q.index == q.num_entry { q.index = 0; } smp_rmb(); c } else { core::ptr::null_mut() } }

unsafe fn ibmvtpm_crq_process(c: *mut ibmvtpm_crq, d: *mut ibmvtpm_dev) {
    match (*c).valid { VALID_INIT_CRQ => match (*c).msg { INIT_CRQ_RES => { dev_info((*d).dev, "CRQ initialized\n"); let rc = ibmvtpm_crq_send_init_complete(d); if rc != 0 { dev_err((*d).dev, "Unable to send CRQ init complete rc=%d\n", rc); } }, INIT_CRQ_COMP_RES => dev_info((*d).dev, "CRQ initialization completed\n"), _ => dev_err((*d).dev, "Unknown crq message type: %d\n", (*c).msg) }, IBMVTPM_VALID_CMD => match (*c).msg { VTPM_GET_RTCE_BUFFER_SIZE_RES => { let n = be16_to_cpu((*c).len); if n == 0 { dev_err((*d).dev, "Invalid rtce size\n"); return; } (*d).rtce_size = n; (*d).rtce_buf = kmalloc(n as usize, GFP_ATOMIC); if (*d).rtce_buf.is_null() { dev_err((*d).dev, "Failed to allocate memory for rtce buffer\n"); return; } (*d).rtce_dma_handle = dma_map_single((*d).dev, (*d).rtce_buf, n as usize, DMA_BIDIRECTIONAL); if dma_mapping_error((*d).dev, (*d).rtce_dma_handle) { kfree((*d).rtce_buf); (*d).rtce_buf = core::ptr::null_mut(); dev_err((*d).dev, "Failed to dma map rtce buffer\n"); } }, VTPM_GET_VERSION_RES => (*d).vtpm_version = be32_to_cpu((*c).data), VTPM_TPM_COMMAND_RES => { (*d).res_len = be16_to_cpu((*c).len); (*d).tpm_processing_cmd = 0; wake_up_interruptible(&mut (*d).wq); }, _ => {} }, _ => {} }
}

unsafe fn ibmvtpm_interrupt(_irq: i32, instance: *mut core::ffi::c_void) -> irqreturn_t { let d = instance as *mut ibmvtpm_dev; loop { let c = ibmvtpm_crq_get_next(d); if c.is_null() { break; } ibmvtpm_crq_process(c, d); wake_up_interruptible(&mut (*d).crq_queue.wq); (*c).valid = 0; smp_wmb(); } IRQ_HANDLED }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
