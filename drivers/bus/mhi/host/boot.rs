// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.
 */

// Linux kernel dependencies and symbols from internal.h are supplied externally.

pub unsafe fn mhi_rddm_prepare(mhi_cntrl: *mut mhi_controller, img_info: *mut image_info) -> i32 {
    let mut mhi_buf = (*img_info).mhi_buf;
    let mut bhi_vec = (*img_info).bhi_vec;
    let base = (*mhi_cntrl).bhie;
    let dev = &(*(*mhi_cntrl).mhi_dev).dev;
    let mut i: u32 = 0;
    while i < (*img_info).entries - 1 {
        (*bhi_vec).dma_addr = cpu_to_le64((*mhi_buf).dma_addr);
        (*bhi_vec).size = cpu_to_le64((*mhi_buf).len);
        i += 1; mhi_buf = mhi_buf.add(1); bhi_vec = bhi_vec.add(1);
    }
    dev_dbg(dev, "BHIe programming for RDDM\n");
    mhi_write_reg(mhi_cntrl, base, BHIE_RXVECADDR_HIGH_OFFS, upper_32_bits((*mhi_buf).dma_addr));
    mhi_write_reg(mhi_cntrl, base, BHIE_RXVECADDR_LOW_OFFS, lower_32_bits((*mhi_buf).dma_addr));
    mhi_write_reg(mhi_cntrl, base, BHIE_RXVECSIZE_OFFS, (*mhi_buf).len);
    let sequence_id = MHI_RANDOM_U32_NONZERO(BHIE_RXVECSTATUS_SEQNUM_BMSK);
    let ret = mhi_write_reg_field(mhi_cntrl, base, BHIE_RXVECDB_OFFS, BHIE_RXVECDB_SEQNUM_BMSK, sequence_id);
    if ret != 0 { dev_err(dev, "Failed to write sequence ID for BHIE_RXVECDB\n"); return ret; }
    dev_dbg(dev, "Address: %p and len: 0x%zx sequence: %u\n", &(*mhi_buf).dma_addr, (*mhi_buf).len, sequence_id);
    0
}

unsafe fn __mhi_download_rddm_in_panic(mhi_cntrl: *mut mhi_controller) -> i32 {
    let delayus: u32 = 2000; let rddm_timeout_us: u32 = 200000;
    let mut retry = ((*mhi_cntrl).timeout_ms * 1000) / delayus;
    let mut rddm_retry = rddm_timeout_us / delayus;
    let base = (*mhi_cntrl).bhie; let dev = &(*(*mhi_cntrl).mhi_dev).dev;
    let mut rx_status: u32 = 0; let mut ee;
    dev_dbg(dev, "Entered with pm_state:%s dev_state:%s ee:%s\n", to_mhi_pm_state_str((*mhi_cntrl).pm_state), mhi_state_str((*mhi_cntrl).dev_state), TO_MHI_EXEC_STR((*mhi_cntrl).ee));
    (*mhi_cntrl).pm_state = MHI_PM_LD_ERR_FATAL_DETECT; smp_wmb();
    ee = mhi_get_exec_env(mhi_cntrl); if ee == MHI_EE_MAX { return -EIO; }
    if ee != MHI_EE_RDDM {
        dev_dbg(dev, "Trigger device into RDDM mode using SYS ERR\n"); mhi_set_mhi_state(mhi_cntrl, MHI_STATE_SYS_ERR);
        dev_dbg(dev, "Waiting for device to enter RDDM\n");
        while rddm_retry > 0 { ee = mhi_get_exec_env(mhi_cntrl); if ee == MHI_EE_RDDM { break; } rddm_retry -= 1; udelay(delayus); }
        if rddm_retry == 0 { dev_dbg(dev, "Did not enter RDDM, do a host req reset\n"); mhi_soc_reset(mhi_cntrl); udelay(delayus); }
        ee = mhi_get_exec_env(mhi_cntrl);
    }
    dev_dbg(dev, "Waiting for RDDM image download via BHIe, current EE:%s\n", TO_MHI_EXEC_STR(ee));
    while retry > 0 { if mhi_read_reg_field(mhi_cntrl, base, BHIE_RXVECSTATUS_OFFS, BHIE_RXVECSTATUS_STATUS_BMSK, &mut rx_status) != 0 { return -EIO; } if rx_status == BHIE_RXVECSTATUS_STATUS_XFER_COMPL { return 0; } retry -= 1; udelay(delayus); }
    ee = mhi_get_exec_env(mhi_cntrl); mhi_read_reg(mhi_cntrl, base, BHIE_RXVECSTATUS_OFFS, &mut rx_status);
    dev_err(dev, "RXVEC_STATUS: 0x%x\n", rx_status); dev_err(dev, "RDDM transfer failed. Current EE: %s\n", TO_MHI_EXEC_STR(ee)); -EIO
}

pub unsafe fn mhi_download_rddm_image(mhi_cntrl: *mut mhi_controller, in_panic: bool) -> i32 {
    if in_panic { return __mhi_download_rddm_in_panic(mhi_cntrl); }
    let base = (*mhi_cntrl).bhie; let dev = &(*(*mhi_cntrl).mhi_dev).dev; let mut rx_status: u32 = 0;
    dev_dbg(dev, "Waiting for RDDM image download via BHIe\n");
    wait_event_timeout((*mhi_cntrl).state_event, mhi_read_reg_field(mhi_cntrl, base, BHIE_RXVECSTATUS_OFFS, BHIE_RXVECSTATUS_STATUS_BMSK, &mut rx_status) != 0 || rx_status != 0, msecs_to_jiffies((*mhi_cntrl).timeout_ms));
    if rx_status == BHIE_RXVECSTATUS_STATUS_XFER_COMPL { 0 } else { -EIO }
}

unsafe fn mhi_fw_load_error_dump(mhi_cntrl: *mut mhi_controller) {
    let dev = &(*(*mhi_cntrl).mhi_dev).dev; let pm_lock = &mut (*mhi_cntrl).pm_lock; let base = (*mhi_cntrl).bhi; let mut val = 0; let regs = [("ERROR_CODE", BHI_ERRCODE), ("ERROR_DBG1", BHI_ERRDBG1), ("ERROR_DBG2", BHI_ERRDBG2), ("ERROR_DBG3", BHI_ERRDBG3)];
    read_lock_bh(pm_lock); if MHI_REG_ACCESS_VALID((*mhi_cntrl).pm_state) { for (name, offset) in regs { if mhi_read_reg(mhi_cntrl, base, offset, &mut val) != 0 { break; } dev_err(dev, "Reg: %s value: 0x%x\n", name, val); } } read_unlock_bh(pm_lock);
}

unsafe fn mhi_fw_load_bhie(mhi_cntrl: *mut mhi_controller, mhi_buf: *const mhi_buf) -> i32 {
    let base = (*mhi_cntrl).bhie; let dev = &(*(*mhi_cntrl).mhi_dev).dev; let pm_lock = &mut (*mhi_cntrl).pm_lock; let mut tx_status = 0;
    read_lock_bh(pm_lock); if !MHI_REG_ACCESS_VALID((*mhi_cntrl).pm_state) { read_unlock_bh(pm_lock); return -EIO; }
    let sequence_id = MHI_RANDOM_U32_NONZERO(BHIE_TXVECSTATUS_SEQNUM_BMSK); dev_dbg(dev, "Starting image download via BHIe. Sequence ID: %u\n", sequence_id);
    mhi_write_reg(mhi_cntrl, base, BHIE_TXVECADDR_HIGH_OFFS, upper_32_bits((*mhi_buf).dma_addr)); mhi_write_reg(mhi_cntrl, base, BHIE_TXVECADDR_LOW_OFFS, lower_32_bits((*mhi_buf).dma_addr)); mhi_write_reg(mhi_cntrl, base, BHIE_TXVECSIZE_OFFS, (*mhi_buf).len);
    let ret = mhi_write_reg_field(mhi_cntrl, base, BHIE_TXVECDB_OFFS, BHIE_TXVECDB_SEQNUM_BMSK, sequence_id); read_unlock_bh(pm_lock); if ret != 0 { return ret; }
    let wait_ret = wait_event_timeout((*mhi_cntrl).state_event, MHI_PM_IN_ERROR_STATE((*mhi_cntrl).pm_state) || mhi_read_reg_field(mhi_cntrl, base, BHIE_TXVECSTATUS_OFFS, BHIE_TXVECSTATUS_STATUS_BMSK, &mut tx_status) != 0 || tx_status != 0, msecs_to_jiffies((*mhi_cntrl).timeout_ms));
    if MHI_PM_IN_ERROR_STATE((*mhi_cntrl).pm_state) || tx_status != BHIE_TXVECSTATUS_STATUS_XFER_COMPL { -EIO } else if wait_ret == 0 { -ETIMEDOUT } else { 0 }
}

unsafe fn mhi_fw_load_bhi(mhi_cntrl: *mut mhi_controller, mhi_buf: *const mhi_buf) -> i32 {
    let dev = &(*(*mhi_cntrl).mhi_dev).dev; let pm_lock = &mut (*mhi_cntrl).pm_lock; let base = (*mhi_cntrl).bhi; let mut tx_status = 0;
    read_lock_bh(pm_lock); if !MHI_REG_ACCESS_VALID((*mhi_cntrl).pm_state) { read_unlock_bh(pm_lock); return -EIO; }
    let session_id = MHI_RANDOM_U32_NONZERO(BHI_TXDB_SEQNUM_BMSK); dev_dbg(dev, "Starting image download via BHI. Session ID: %u\n", session_id); mhi_write_reg(mhi_cntrl, base, BHI_STATUS, 0); mhi_write_reg(mhi_cntrl, base, BHI_IMGADDR_HIGH, upper_32_bits((*mhi_buf).dma_addr)); mhi_write_reg(mhi_cntrl, base, BHI_IMGADDR_LOW, lower_32_bits((*mhi_buf).dma_addr)); mhi_write_reg(mhi_cntrl, base, BHI_IMGSIZE, (*mhi_buf).len); mhi_write_reg(mhi_cntrl, base, BHI_IMGTXDB, session_id); read_unlock_bh(pm_lock);
    let ret = wait_event_timeout((*mhi_cntrl).state_event, MHI_PM_IN_ERROR_STATE((*mhi_cntrl).pm_state) || mhi_read_reg_field(mhi_cntrl, base, BHI_STATUS, BHI_STATUS_MASK, &mut tx_status) != 0 || tx_status != 0, msecs_to_jiffies((*mhi_cntrl).timeout_ms));
    if MHI_PM_IN_ERROR_STATE((*mhi_cntrl).pm_state) || tx_status == BHI_STATUS_ERROR { if tx_status == BHI_STATUS_ERROR { dev_err(dev, "Image transfer failed\n"); mhi_fw_load_error_dump(mhi_cntrl); } -EIO } else if ret == 0 { -ETIMEDOUT } else { 0 }
}

unsafe fn mhi_free_bhi_buffer(mhi_cntrl: *mut mhi_controller, image_info: *mut image_info) { let b = (*image_info).mhi_buf; dma_free_coherent((*mhi_cntrl).cntrl_dev, (*b).len, (*b).buf, (*b).dma_addr); kfree(image_info); }
pub unsafe fn mhi_free_bhie_table(mhi_cntrl: *mut mhi_controller, image_info: *mut image_info) { let mut b = (*image_info).mhi_buf; for _ in 0..(*image_info).entries { dma_free_coherent((*mhi_cntrl).cntrl_dev, (*b).len, (*b).buf, (*b).dma_addr); b = b.add(1); } kfree(image_info); }

unsafe fn mhi_alloc_bhi_buffer(mhi_cntrl: *mut mhi_controller, image_info: *mut *mut image_info, alloc_size: usize) -> i32 { let img = kzalloc_flex::<image_info, mhi_buf>(1); if img.is_null() { return -ENOMEM; } let b = (*img).mhi_buf; (*b).len = alloc_size; (*b).buf = dma_alloc_coherent((*mhi_cntrl).cntrl_dev, (*b).len, &mut (*b).dma_addr, GFP_KERNEL); if (*b).buf.is_null() { kfree(img); return -ENOMEM; } (*img).bhi_vec = core::ptr::null_mut(); (*img).entries = 1; *image_info = img; 0 }

pub unsafe fn mhi_alloc_bhie_table(mhi_cntrl: *mut mhi_controller, image_info: *mut *mut image_info, alloc_size: usize) -> i32 {
    let seg_size = (*mhi_cntrl).seg_len; let segments = DIV_ROUND_UP(alloc_size, seg_size) + 1; let img = kzalloc_flex::<image_info, mhi_buf>(segments); if img.is_null() { return -ENOMEM; } (*img).entries = segments; let mut b = (*img).mhi_buf; let mut i = 0;
    while i < segments { let vec_size = if i == segments - 1 { core::mem::size_of::<bhi_vec_entry>() * i } else { seg_size }; (*b).len = vec_size; (*b).buf = dma_alloc_coherent((*mhi_cntrl).cntrl_dev, vec_size, &mut (*b).dma_addr, GFP_KERNEL); if (*b).buf.is_null() { while i > 0 { i -= 1; b = b.sub(1); dma_free_coherent((*mhi_cntrl).cntrl_dev, (*b).len, (*b).buf, (*b).dma_addr); } kfree(img); return -ENOMEM; } i += 1; b = b.add(1); }
    (*img).bhi_vec = (*img).mhi_buf.add(segments - 1) as *mut bhi_vec_entry; *image_info = img; 0
}

unsafe fn mhi_firmware_copy_bhie(_mhi_cntrl: *mut mhi_controller, mut buf: *const u8, mut remainder: usize, img_info: *mut image_info) { let mut b = (*img_info).mhi_buf; let mut v = (*img_info).bhi_vec; while remainder != 0 { let to_cpy = core::cmp::min(remainder, (*b).len); memcpy((*b).buf, buf, to_cpy); (*v).dma_addr = cpu_to_le64((*b).dma_addr); (*v).size = cpu_to_le64(to_cpy); buf = buf.add(to_cpy); remainder -= to_cpy; b = b.add(1); v = v.add(1); } }

unsafe fn mhi_fw_load_type_get(c: *const mhi_controller) -> mhi_fw_load_type { if (*c).fbc_download { MHI_FW_LOAD_FBC } else if (*c).seg_len != 0 { MHI_FW_LOAD_BHIE } else { MHI_FW_LOAD_BHI } }
unsafe fn mhi_load_image_bhi(c: *mut mhi_controller, fw: *const u8, size: usize) -> i32 { let mut image = core::ptr::null_mut(); let ret = mhi_alloc_bhi_buffer(c, &mut image, size); if ret != 0 { return ret; } memcpy((*image).mhi_buf as *mut _, fw, size); let ret = mhi_fw_load_bhi(c, (*image).mhi_buf); mhi_free_bhi_buffer(c, image); ret }
unsafe fn mhi_load_image_bhie(c: *mut mhi_controller, fw: *const u8, size: usize) -> i32 { let mut image = core::ptr::null_mut(); let ret = mhi_alloc_bhie_table(c, &mut image, size); if ret != 0 { return ret; } mhi_firmware_copy_bhie(c, fw, size, image); let ret = mhi_fw_load_bhie(c, (*image).mhi_buf.add((*image).entries - 1)); mhi_free_bhie_table(c, image); ret }

// The remaining firmware-handler control flow is preserved below; external kernel symbols are intentionally unresolved.
pub unsafe fn mhi_fw_load_handler(c: *mut mhi_controller) {
    let dev = &(*(*c).mhi_dev).dev; if MHI_PM_IN_ERROR_STATE((*c).pm_state) { dev_err(dev, "Device MHI is not in valid state\n"); return; }
    if mhi_read_reg(c, (*c).bhi, BHI_SERIALNU, &mut (*c).serial_number) != 0 { dev_err(dev, "Could not capture serial number via BHI\n"); }
    if !MHI_FW_LOAD_CAPABLE((*c).ee) { let _ = mhi_ready_state_transition(c); return; }
    let fw_name = if (*c).ee == MHI_EE_EDL { (*c).edl_image } else { (*c).fw_image }; let mut firmware = core::ptr::null(); let mut size; let mut fw_data; let mut fw_sz;
    if fw_name.is_null() && (*c).fbc_download && !(*c).fw_data.is_null() && (*c).fw_sz != 0 { if (*c).sbl_size == 0 { dev_err(dev, "fw_data provided but no sbl_size\n"); return; } size = (*c).sbl_size; fw_data = (*c).fw_data; fw_sz = (*c).fw_sz; } else { if fw_name.is_null() || ((*c).fbc_download && ((*c).sbl_size == 0 || (*c).seg_len == 0)) { dev_err(dev, "No firmware image defined or !sbl_size || !seg_len\n"); return; } if request_firmware(&mut firmware, fw_name, dev) != 0 { dev_err(dev, "Error loading firmware\n"); return; } size = if (*c).fbc_download { (*c).sbl_size } else { (*firmware).size }; if size > (*firmware).size { size = (*firmware).size; } fw_data = (*firmware).data; fw_sz = (*firmware).size; }
    let typ = mhi_fw_load_type_get(c); let ret = if typ == MHI_FW_LOAD_BHIE { mhi_load_image_bhie(c, fw_data, size) } else { mhi_load_image_bhi(c, fw_data, size) }; if ret != 0 { dev_err(dev, "MHI did not load image over BHI, ret: %d\n", ret); release_firmware(firmware); return; }
    if !fw_name.is_null() && fw_name == (*c).edl_image { release_firmware(firmware); let _ = mhi_ready_state_transition(c); return; }
    write_lock_irq(&mut (*c).pm_lock); (*c).dev_state = MHI_STATE_RESET; write_unlock_irq(&mut (*c).pm_lock); release_firmware(firmware);
    if typ == MHI_FW_LOAD_FBC { if !memcmp(fw_data.add((*c).sbl_size), ELFMAG, SELFMAG) { fw_data = fw_data.add((*c).sbl_size); fw_sz -= (*c).sbl_size; } if mhi_alloc_bhie_table(c, &mut (*c).fbc_image, fw_sz) != 0 { return; } mhi_firmware_copy_bhie(c, fw_data, fw_sz, (*c).fbc_image); }
    if mhi_ready_state_transition(c) != 0 { if !(*c).fbc_image.is_null() { mhi_free_bhie_table(c, (*c).fbc_image); (*c).fbc_image = core::ptr::null_mut(); } }
}

pub unsafe fn mhi_download_amss_image(c: *mut mhi_controller) -> i32 { let image = (*c).fbc_image; let dev = &(*(*c).mhi_dev).dev; if image.is_null() { return -EIO; } let ret = mhi_fw_load_bhie(c, (*image).mhi_buf.add((*image).entries - 1)); if ret != 0 { dev_err(dev, "MHI did not load AMSS, ret:%d\n", ret); } ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
