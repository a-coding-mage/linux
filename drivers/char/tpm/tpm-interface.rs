// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004 IBM Corporation
 * Copyright (C) 2014 Intel Corporation
 *
 * Authors:
 * Leendert van Doorn <leendert@watson.ibm.com>
 * Dave Safford <safford@watson.ibm.com>
 * Reiner Sailer <sailer@watson.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * Specifications at www.trustedcomputinggroup.org
 *
 * Note, the TPM chip is not interrupt driven (only polling)
 * and can have very long timeouts (minutes!). Hence the unusual
 * calls to msleep.
 */

/* Linux kernel headers and "tpm.h" are supplied by other translation units. */

static mut tpm_suspend_pcr: u32 = 0;

pub unsafe fn tpm_calc_ordinal_duration(chip: *mut tpm_chip, ordinal: u32) -> c_ulong {
    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 {
        tpm2_calc_ordinal_duration(ordinal)
    } else {
        tpm1_calc_ordinal_duration(chip, ordinal)
    }
}

unsafe fn tpm_chip_cancel(chip: *mut tpm_chip) {
    if (*(*chip).ops).cancel.is_none() { return; }
    ((*(*chip).ops).cancel.unwrap())(chip);
}

unsafe fn tpm_chip_status(chip: *mut tpm_chip) -> u8 {
    if (*(*chip).ops).status.is_none() { return 0; }
    ((*(*chip).ops).status.unwrap())(chip)
}

unsafe fn tpm_chip_req_canceled(chip: *mut tpm_chip, status: u8) -> bool {
    if (*(*chip).ops).req_canceled.is_none() { return false; }
    ((*(*chip).ops).req_canceled.unwrap())(chip, status)
}

unsafe fn tpm_transmit_completed(status: u8, chip: *mut tpm_chip) -> bool {
    let status_masked = status & (*(*chip).ops).req_complete_mask;
    status_masked == (*(*chip).ops).req_complete_val
}

unsafe fn tpm_try_transmit(chip: *mut tpm_chip, buf: *mut c_void, mut bufsiz: usize) -> isize {
    let header = buf as *mut tpm_header;
    let mut rc: i32;
    let mut len: isize = 0;
    let count = be32_to_cpu((*header).length);
    let ordinal = be32_to_cpu((*header).ordinal);
    if bufsiz < TPM_HEADER_SIZE { return -EINVAL as isize; }
    if bufsiz > TPM_BUFSIZE { bufsiz = TPM_BUFSIZE; }
    if count == 0 { return -ENODATA as isize; }
    if count as usize > bufsiz { return -E2BIG as isize; }
    rc = ((*(*chip).ops).send)(chip, buf, bufsiz, count);
    if rc < 0 { return rc as isize; }
    if (*chip).flags & TPM_CHIP_FLAG_SYNC != 0 { len = rc as isize; rc = 0; }
    else {
        if rc > 0 { rc = 0; }
        if (*chip).flags & TPM_CHIP_FLAG_IRQ == 0 {
            let stop = jiffies + tpm_calc_ordinal_duration(chip, ordinal);
            loop {
                let status = tpm_chip_status(chip);
                if tpm_transmit_completed(status, chip) { break; }
                if tpm_chip_req_canceled(chip, status) { return -ECANCELED as isize; }
                tpm_msleep(TPM_TIMEOUT_POLL); rmb();
                if !time_before(jiffies, stop) { tpm_chip_cancel(chip); return -ETIME as isize; }
            }
        }
        len = ((*(*chip).ops).recv)(chip, buf, bufsiz) as isize;
        if len < 0 { return len; }
    }
    if len < TPM_HEADER_SIZE as isize || len != be32_to_cpu((*header).length) as isize { rc = -EFAULT; }
    if rc != 0 { rc as isize } else { len }
}

pub unsafe fn tpm_transmit(chip: *mut tpm_chip, buf: *mut u8, bufsiz: usize) -> isize {
    let header = buf as *mut tpm_header;
    let mut save = [0u8; TPM_HEADER_SIZE + 3 * core::mem::size_of::<u32>()];
    let mut delay_msec = TPM2_DURATION_SHORT;
    let mut ret: isize;
    let save_size = core::cmp::min(save.len(), bufsiz);
    let cc = be32_to_cpu((*header).return_code);
    core::ptr::copy_nonoverlapping(buf, save.as_mut_ptr(), save_size);
    loop {
        ret = tpm_try_transmit(chip, buf as *mut c_void, bufsiz);
        if ret < 0 { break; }
        let rc = be32_to_cpu((*header).return_code);
        if rc != TPM2_RC_RETRY && rc != TPM2_RC_TESTING { break; }
        if rc == TPM2_RC_TESTING && cc == TPM2_CC_SELF_TEST { break; }
        if delay_msec > TPM2_DURATION_LONG { break; }
        tpm_msleep(delay_msec); delay_msec *= 2;
        core::ptr::copy_nonoverlapping(save.as_ptr(), buf, save_size);
    }
    ret
}

pub unsafe fn tpm_transmit_cmd(chip: *mut tpm_chip, buf: *mut tpm_buf,
                               min_rsp_body_length: usize, desc: *const c_char) -> isize {
    let header = (*buf).data.as_ptr() as *const tpm_header;
    let len = tpm_transmit(chip, (*buf).data.as_mut_ptr(), PAGE_SIZE);
    if len < 0 { return len; }
    let err = be32_to_cpu((*header).return_code) as i32;
    if err != 0 { return err as isize; }
    if len < (min_rsp_body_length + TPM_HEADER_SIZE) as isize { return -EFAULT as isize; }
    (*buf).length = len as usize; 0
}

pub unsafe fn tpm_get_timeouts(chip: *mut tpm_chip) -> i32 {
    if (*chip).flags & TPM_CHIP_FLAG_HAVE_TIMEOUTS != 0 { return 0; }
    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 { tpm2_get_timeouts(chip) } else { tpm1_get_timeouts(chip) }
}

pub unsafe fn tpm_is_tpm2(chip: *mut tpm_chip) -> i32 {
    if chip.is_null() { return -ENODEV; }
    let rc = tpm_try_get_ops(chip); if rc != 0 { return rc; }
    let result = if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 { 1 } else { 0 };
    tpm_put_ops(chip); result
}

pub unsafe fn tpm_pcr_read(chip: *mut tpm_chip, pcr_idx: u32, digest: *mut tpm_digest) -> i32 {
    if chip.is_null() { return -ENODEV; }
    let rc = tpm_try_get_ops(chip); if rc != 0 { return rc; }
    let result = if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 { tpm2_pcr_read(chip, pcr_idx, digest, core::ptr::null_mut()) } else { tpm1_pcr_read(chip, pcr_idx, (*digest).digest.as_mut_ptr()) };
    tpm_put_ops(chip); result
}

pub unsafe fn tpm_pcr_extend(chip: *mut tpm_chip, pcr_idx: u32, digests: *mut tpm_digest) -> i32 {
    if chip.is_null() { return -ENODEV; }
    let mut rc = tpm_try_get_ops(chip); if rc != 0 { return rc; }
    for i in 0..(*chip).nr_allocated_banks as usize { if (*digests.add(i)).alg_id != (*(*chip).allocated_banks.add(i)).alg_id { rc = -EINVAL; tpm_put_ops(chip); return rc; } }
    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 { rc = tpm2_pcr_extend(chip, pcr_idx, digests); }
    else { rc = tpm1_pcr_extend(chip, pcr_idx, (*digests).digest.as_ptr(), b"attempting extend a PCR value\0".as_ptr() as *const c_char); }
    tpm_put_ops(chip); rc
}

pub unsafe fn tpm_auto_startup(chip: *mut tpm_chip) -> i32 {
    if (*(*chip).ops).flags & TPM_OPS_AUTO_STARTUP == 0 { return 0; }
    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 { tpm2_auto_startup(chip) } else { tpm1_auto_startup(chip) }
}

pub unsafe fn tpm_pm_suspend(dev: *mut device) -> i32 {
    let chip = dev_get_drvdata(dev) as *mut tpm_chip; if chip.is_null() { return -ENODEV; }
    let mut rc = tpm_try_get_ops(chip);
    if rc != 0 { (*chip).flags |= TPM_CHIP_FLAG_SUSPENDED; return 0; }
    if (*chip).flags & TPM_CHIP_FLAG_ALWAYS_POWERED == 0 && !((*chip).flags & TPM_CHIP_FLAG_FIRMWARE_POWER_MANAGED != 0 && !pm_suspend_via_firmware()) {
        if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 { tpm2_end_auth_session(chip); tpm2_shutdown(chip, TPM2_SU_STATE); }
        else { rc = tpm1_pm_suspend(chip, tpm_suspend_pcr); }
    }
    (*chip).flags |= TPM_CHIP_FLAG_SUSPENDED; tpm_put_ops(chip); let _ = rc; 0
}

pub unsafe fn tpm_pm_resume(dev: *mut device) -> i32 {
    let chip = dev_get_drvdata(dev) as *mut tpm_chip; if chip.is_null() { return -ENODEV; }
    (*chip).flags &= !TPM_CHIP_FLAG_SUSPENDED; wmb(); 0
}

pub unsafe fn tpm_get_random(chip: *mut tpm_chip, out: *mut u8, max: usize) -> i32 {
    if out.is_null() || max > TPM_MAX_RNG_DATA || chip.is_null() { return -EINVAL; }
    let rc = tpm_try_get_ops(chip); if rc != 0 { return rc; }
    let result = if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 { tpm2_get_random(chip, out, max) } else { tpm1_get_random(chip, out, max) };
    tpm_put_ops(chip); result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
