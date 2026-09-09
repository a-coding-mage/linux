// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014, 2015 Intel Corporation
 *
 * Authors:
 * Jarkko Sakkinen <jarkko.sakkinen@linux.intel.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * This file contains TPM2 protocol implementations of the commands
 * used by the kernel internally.
 */

// Translated from tpm2-cmd.c. Kernel declarations and constants are supplied
// by the surrounding translation unit.

static mut disable_pcr_integrity: bool = false;

static static_tpm2_hash_map: [tpm2_hash; 5] = [
    tpm2_hash { crypto_id: HASH_ALGO_SHA1, tpm_id: TPM_ALG_SHA1 },
    tpm2_hash { crypto_id: HASH_ALGO_SHA256, tpm_id: TPM_ALG_SHA256 },
    tpm2_hash { crypto_id: HASH_ALGO_SHA384, tpm_id: TPM_ALG_SHA384 },
    tpm2_hash { crypto_id: HASH_ALGO_SHA512, tpm_id: TPM_ALG_SHA512 },
    tpm2_hash { crypto_id: HASH_ALGO_SM3_256, tpm_id: TPM_ALG_SM3_256 },
];

pub unsafe fn tpm2_find_hash_alg(crypto_id: u32) -> i32 {
    for i in 0..static_tpm2_hash_map.len() {
        if crypto_id == static_tpm2_hash_map[i].crypto_id {
            return static_tpm2_hash_map[i].tpm_id as i32;
        }
    }
    -EINVAL
}

pub unsafe fn tpm2_get_timeouts(chip: *mut tpm_chip) -> i32 {
    (*chip).timeout_a = msecs_to_jiffies(TPM2_TIMEOUT_A);
    (*chip).timeout_b = msecs_to_jiffies(TPM2_TIMEOUT_B);
    (*chip).timeout_c = msecs_to_jiffies(TPM2_TIMEOUT_C);
    (*chip).timeout_d = msecs_to_jiffies(TPM2_TIMEOUT_D);
    (*chip).flags |= TPM_CHIP_FLAG_HAVE_TIMEOUTS;
    0
}

/* Contains the maximum durations in milliseconds for TPM2 commands. */
static static_tpm2_ordinal_duration_map: [(u32, u64); 15] = [
    (TPM2_CC_STARTUP, 750), (TPM2_CC_SELF_TEST, 3000),
    (TPM2_CC_GET_RANDOM, 2000), (TPM2_CC_SEQUENCE_UPDATE, 750),
    (TPM2_CC_SEQUENCE_COMPLETE, 750), (TPM2_CC_EVENT_SEQUENCE_COMPLETE, 750),
    (TPM2_CC_HASH_SEQUENCE_START, 750), (TPM2_CC_VERIFY_SIGNATURE, 30000),
    (TPM2_CC_PCR_EXTEND, 750), (TPM2_CC_HIERARCHY_CONTROL, 2000),
    (TPM2_CC_HIERARCHY_CHANGE_AUTH, 2000), (TPM2_CC_GET_CAPABILITY, 750),
    (TPM2_CC_NV_READ, 2000), (TPM2_CC_CREATE_PRIMARY, 300000),
    (TPM2_CC_CREATE, 300000),
];

pub unsafe fn tpm2_calc_ordinal_duration(ordinal: u32) -> u64 {
    for &(cmd, duration) in &static_tpm2_ordinal_duration_map {
        if ordinal == cmd { return msecs_to_jiffies(duration); }
    }
    msecs_to_jiffies(TPM2_DURATION_DEFAULT)
}

pub unsafe fn tpm2_pcr_read(chip: *mut tpm_chip, pcr_idx: u32,
                            digest: *mut tpm_digest,
                            digest_size_ptr: *mut u16) -> i32 {
    if pcr_idx >= TPM2_PLATFORM_PCR { return -EINVAL; }
    let mut expected_digest_size: u16 = 0;
    if digest_size_ptr.is_null() {
        let mut i = 0;
        while i < (*chip).nr_allocated_banks &&
              (*chip).allocated_banks[i].alg_id != (*digest).alg_id { i += 1; }
        if i == (*chip).nr_allocated_banks { return -EINVAL; }
        expected_digest_size = (*chip).allocated_banks[i].digest_size;
    }
    let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM; }
    tpm_buf_init(buf, TPM_BUFSIZE);
    tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_PCR_READ);
    let mut pcr_select = [0u8; TPM2_PCR_SELECT_MIN as usize];
    pcr_select[(pcr_idx >> 3) as usize] = 1 << (pcr_idx & 0x7);
    tpm_buf_append_u32(buf, 1);
    tpm_buf_append_u16(buf, (*digest).alg_id);
    tpm_buf_append_u8(buf, TPM2_PCR_SELECT_MIN);
    tpm_buf_append(buf, pcr_select.as_ptr(), pcr_select.len());
    let rc = tpm_transmit_cmd(chip, buf, 0, c"attempting to read a pcr value".as_ptr());
    if rc != 0 { kfree(buf); return rc; }
    let out = ( (*buf).data.add(TPM_HEADER_SIZE as usize) ) as *const tpm2_pcr_read_out;
    let digest_size = be16_to_cpu((*out).digest_size);
    if digest_size as usize > (*digest).digest.len() ||
       (digest_size_ptr.is_null() && digest_size != expected_digest_size) {
        kfree(buf); return -EINVAL;
    }
    if !digest_size_ptr.is_null() { *digest_size_ptr = digest_size; }
    memcpy((*digest).digest.as_mut_ptr(), (*out).digest.as_ptr(), digest_size as usize);
    kfree(buf); rc
}

pub unsafe fn tpm2_pcr_extend(chip: *mut tpm_chip, pcr_idx: u32,
                              digests: *const tpm_digest) -> i32 {
    let mut rc;
    if !disable_pcr_integrity {
        rc = tpm2_start_auth_session(chip); if rc != 0 { return rc; }
    }
    let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL);
    if buf.is_null() {
        if !disable_pcr_integrity { tpm2_end_auth_session(chip); }
        return -ENOMEM;
    }
    tpm_buf_init(buf, TPM_BUFSIZE);
    tpm_buf_reset(buf, TPM2_ST_SESSIONS, TPM2_CC_PCR_EXTEND);
    if !disable_pcr_integrity {
        rc = tpm_buf_append_name(chip, buf, pcr_idx, core::ptr::null());
        if rc != 0 { tpm2_end_auth_session(chip); kfree(buf); return rc; }
        tpm_buf_append_hmac_session(chip, buf, 0, core::ptr::null(), 0);
    } else {
        tpm_buf_append_handle(buf, pcr_idx);
        tpm_buf_append_auth(chip, buf, core::ptr::null(), 0);
    }
    tpm_buf_append_u32(buf, (*chip).nr_allocated_banks);
    for i in 0..(*chip).nr_allocated_banks {
        tpm_buf_append_u16(buf, (*digests.add(i)).alg_id);
        tpm_buf_append(buf, (*digests.add(i)).digest.as_ptr(), (*chip).allocated_banks[i].digest_size as usize);
    }
    if !disable_pcr_integrity { rc = tpm_buf_fill_hmac_session(chip, buf); if rc != 0 { kfree(buf); return rc; } }
    rc = tpm_transmit_cmd(chip, buf, 0, c"attempting extend a PCR value".as_ptr());
    if !disable_pcr_integrity { rc = tpm_buf_check_hmac_response(chip, buf, rc); }
    kfree(buf); rc
}

pub unsafe fn tpm2_get_random(chip: *mut tpm_chip, dest: *mut u8, max: usize) -> i32 {
    if max == 0 || max > TPM_MAX_RNG_DATA as usize { return -EINVAL; }
    let mut err = tpm2_start_auth_session(chip); if err != 0 { return err; }
    let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL); if buf.is_null() { tpm2_end_auth_session(chip); return -ENOMEM; }
    tpm_buf_init(buf, TPM_BUFSIZE);
    let mut num_bytes = max as u32; let mut total = 0i32; let mut retries = 5i32; let mut dest_ptr = dest;
    while { tpm_buf_reset(buf, TPM2_ST_SESSIONS, TPM2_CC_GET_RANDOM); if tpm2_chip_auth(chip) { tpm_buf_append_hmac_session(chip, buf, TPM2_SA_ENCRYPT | TPM2_SA_CONTINUE_SESSION, core::ptr::null(), 0); } else { tpm_buf_append_u16(buf, num_bytes as u16); }
        tpm_buf_append_u16(buf, num_bytes as u16); err = tpm_buf_fill_hmac_session(chip, buf); if err == 0 { err = tpm_transmit_cmd(chip, buf, 0, c"attempting get random".as_ptr()); err = tpm_buf_check_hmac_response(chip, buf, err); } if err != 0 { if err > 0 { err = -EIO; } tpm2_end_auth_session(chip); kfree(buf); return err; }
        let out = ( (*buf).data.add(TPM_HEADER_SIZE as usize) ) as *const tpm2_get_random_out; let recd = core::cmp::min(be16_to_cpu((*out).size) as u32, num_bytes); memcpy(dest_ptr, (*out).buffer.as_ptr(), recd as usize); dest_ptr = dest_ptr.add(recd as usize); total += recd as i32; num_bytes -= recd; retries -= 1; retries >= 0 && total < max as i32 } {}
    kfree(buf); if total != 0 { total } else { -EIO }
}

pub unsafe fn tpm2_flush_context(chip: *mut tpm_chip, handle: u32) {
    let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL); if buf.is_null() { dev_warn(&(*chip).dev, c"0x%08x was not flushed, out of memory\n".as_ptr(), handle); return; }
    tpm_buf_init(buf, TPM_BUFSIZE); tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_FLUSH_CONTEXT); tpm_buf_append_u32(buf, handle); tpm_transmit_cmd(chip, buf, 0, c"flushing context".as_ptr()); kfree(buf);
}

pub unsafe fn tpm2_get_tpm_pt(chip: *mut tpm_chip, property_id: u32, value: *mut u32, desc: *const i8) -> isize {
    let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM as isize; }
    tpm_buf_init(buf, TPM_BUFSIZE); tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_GET_CAPABILITY); tpm_buf_append_u32(buf, TPM2_CAP_TPM_PROPERTIES); tpm_buf_append_u32(buf, property_id); tpm_buf_append_u32(buf, 1);
    let mut rc = tpm_transmit_cmd(chip, buf, 0, core::ptr::null()); if rc == 0 { let out = (*buf).data.add(TPM_HEADER_SIZE as usize) as *const tpm2_get_cap_out; if be32_to_cpu((*out).property_cnt) > 0 { *value = be32_to_cpu((*out).value); } else { rc = -ENODATA; } } kfree(buf); rc as isize
}

pub unsafe fn tpm2_shutdown(chip: *mut tpm_chip, shutdown_type: u16) { let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL); if buf.is_null() { return; } tpm_buf_init(buf, TPM_BUFSIZE); tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_SHUTDOWN); tpm_buf_append_u16(buf, shutdown_type); tpm_transmit_cmd(chip, buf, 0, c"stopping the TPM".as_ptr()); kfree(buf); }

static unsafe fn tpm2_do_selftest(chip: *mut tpm_chip) -> i32 { let mut rc = 0; for full in 0..2 { let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM; } tpm_buf_init(buf, TPM_BUFSIZE); tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_SELF_TEST); tpm_buf_append_u8(buf, full); rc = tpm_transmit_cmd(chip, buf, 0, c"attempting the self test".as_ptr()); kfree(buf); if rc == TPM2_RC_TESTING { rc = TPM2_RC_SUCCESS; } if rc == TPM2_RC_INITIALIZE || rc == TPM2_RC_SUCCESS { return rc; } } rc }

pub unsafe fn tpm2_probe(chip: *mut tpm_chip) -> i32 { let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM; } tpm_buf_init(buf, TPM_BUFSIZE); tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_GET_CAPABILITY); tpm_buf_append_u32(buf, TPM2_CAP_TPM_PROPERTIES); tpm_buf_append_u32(buf, TPM_PT_TOTAL_COMMANDS); tpm_buf_append_u32(buf, 1); let rc = tpm_transmit_cmd(chip, buf, 0, core::ptr::null()); if rc >= 0 && be16_to_cpu(*( (*buf).data as *const tpm_header).as_ref().unwrap()).tag == TPM2_ST_NO_SESSIONS { (*chip).flags |= TPM_CHIP_FLAG_TPM2; } kfree(buf); 0 }

static unsafe fn tpm2_init_bank_info(chip: *mut tpm_chip, bank_index: u32) -> i32 { let bank = (*chip).allocated_banks.add(bank_index as usize); let mut digest = tpm_digest { alg_id: (*bank).alg_id, ..core::mem::zeroed() }; for i in 0..static_tpm2_hash_map.len() { if (*bank).alg_id != static_tpm2_hash_map[i].tpm_id { continue; } (*bank).digest_size = hash_digest_size[static_tpm2_hash_map[i].crypto_id as usize]; (*bank).crypto_id = static_tpm2_hash_map[i].crypto_id; return 0; } (*bank).crypto_id = HASH_ALGO__LAST; tpm2_pcr_read(chip, 0, &mut digest, &mut (*bank).digest_size) }

pub unsafe fn tpm2_get_pcr_allocation(chip: *mut tpm_chip) -> isize { let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM as isize; } tpm_buf_init(buf, TPM_BUFSIZE); tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_GET_CAPABILITY); tpm_buf_append_u32(buf, TPM2_CAP_PCRS); tpm_buf_append_u32(buf, 0); tpm_buf_append_u32(buf, 1); let rc = tpm_transmit_cmd(chip, buf, 9, c"get tpm pcr allocation".as_ptr()); if rc != 0 { kfree(buf); return rc as isize; } let n = be32_to_cpup((*buf).data.add(TPM_HEADER_SIZE as usize + 5) as *const u32); if n > TPM2_MAX_PCR_BANKS { kfree(buf); return -ENOMEM as isize; } let mut marker = (*buf).data.add(TPM_HEADER_SIZE as usize + 9); let end = (*buf).data.add(be32_to_cpup((*buf).data.add(2) as *const u32) as usize); let mut nr = 0; for _ in 0..n { let sel = marker as *const tpm2_pcr_selection; if marker.add(core::mem::offset_of!(tpm2_pcr_selection, size_of_select)) >= end { kfree(buf); return -EFAULT as isize; } if (*sel).pcr_select.iter().any(|&x| x != 0) { (*chip).allocated_banks[nr].alg_id = be16_to_cpu((*sel).hash_alg); let r = tpm2_init_bank_info(chip, nr as u32); if r < 0 { kfree(buf); return r as isize; } nr += 1; } marker = marker.add(4 + (*sel).size_of_select as usize); } (*chip).nr_allocated_banks = nr; kfree(buf); rc as isize }

pub unsafe fn tpm2_get_cc_attrs_tbl(chip: *mut tpm_chip) -> i32 { let mut nr = 0; let mut rc = tpm2_get_tpm_pt(chip, TPM_PT_TOTAL_COMMANDS, &mut nr, core::ptr::null()); if rc != 0 { return rc as i32; } if nr > 0xFFFFF { return -EFAULT; } (*chip).cc_attrs_tbl = devm_kcalloc(&mut (*chip).dev, 4, nr, GFP_KERNEL); if (*chip).cc_attrs_tbl.is_null() { return -ENOMEM; } let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM; } tpm_buf_init(buf, TPM_BUFSIZE); tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_GET_CAPABILITY); tpm_buf_append_u32(buf, TPM2_CAP_COMMANDS); tpm_buf_append_u32(buf, TPM2_CC_FIRST); tpm_buf_append_u32(buf, nr); rc = tpm_transmit_cmd(chip, buf, 9 + 4 * nr, core::ptr::null()); if rc == 0 { (*chip).nr_commands = nr; for i in 0..nr as usize { (*chip).cc_attrs_tbl[i] = be32_to_cpup((*buf).data.add(TPM_HEADER_SIZE as usize + 9 + 4*i) as *const u32); } } kfree(buf); if rc > 0 { -ENODEV } else { rc as i32 } }

static unsafe fn tpm2_startup(chip: *mut tpm_chip) -> i32 { dev_info(&(*chip).dev, c"starting up the TPM manually\n".as_ptr()); let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM; } tpm_buf_init(buf, TPM_BUFSIZE); tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_STARTUP); tpm_buf_append_u16(buf, TPM2_SU_CLEAR); let rc = tpm_transmit_cmd(chip, buf, 0, c"attempting to start the TPM".as_ptr()); kfree(buf); rc }

pub unsafe fn tpm2_auto_startup(chip: *mut tpm_chip) -> i32 { let mut rc = tpm2_get_timeouts(chip); if rc == 0 { rc = tpm2_do_selftest(chip); } if rc == TPM2_RC_INITIALIZE { rc = tpm2_startup(chip); if rc == 0 { rc = tpm2_do_selftest(chip); } } if rc == 0 { rc = tpm2_get_cc_attrs_tbl(chip); } if rc == TPM2_RC_FAILURE || (rc < 0 && rc != -ENOMEM) { (*chip).flags |= TPM_CHIP_FLAG_FIRMWARE_UPGRADE; rc = 0; } if rc == 0 { rc = tpm2_sessions_init(chip); } if rc == TPM2_RC_UPGRADE || rc == -ENODATA { (*chip).flags |= TPM_CHIP_FLAG_FIRMWARE_UPGRADE; rc = 0; } if rc > 0 { -ENODEV } else { rc } }

pub unsafe fn tpm2_find_cc(chip: *mut tpm_chip, cc: u32) -> i32 { let mask = (1 << TPM2_CC_ATTR_VENDOR) | GENMASK(15, 0); for i in 0..(*chip).nr_commands as usize { if cc == ((*chip).cc_attrs_tbl[i] & mask) { return i as i32; } } -1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
