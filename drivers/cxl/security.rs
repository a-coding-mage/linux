// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2022 Intel Corporation. All rights reserved. */

unsafe fn cxl_pmem_get_security_flags(
    nvdimm: *mut nvdimm,
    ptype: nvdimm_passphrase_type,
) -> c_ulong {
    let cxl_nvd: *mut cxl_nvdimm = nvdimm_provider_data(nvdimm);
    let cxlmd: *mut cxl_memdev = (*cxl_nvd).cxlmd;
    let cxl_mbox: *mut cxl_mailbox = &mut (*(*cxlmd).cxlds).cxl_mbox;
    let mds: *mut cxl_memdev_state = to_cxl_memdev_state((*cxlmd).cxlds);
    let mut security_flags: c_ulong = 0;
    let mut out = cxl_get_security_output { flags: 0 };
    let mut mbox_cmd: cxl_mbox_cmd;
    let sec_out: u32;
    let rc: c_int;

    mbox_cmd = cxl_mbox_cmd {
        opcode: CXL_MBOX_OP_GET_SECURITY_STATE,
        size_in: 0,
        size_out: core::mem::size_of::<cxl_get_security_output>(),
        payload_in: core::ptr::null_mut(),
        payload_out: &mut out as *mut cxl_get_security_output as *mut c_void,
    };

    rc = cxl_internal_send_cmd(cxl_mbox, &mut mbox_cmd);
    if rc < 0 {
        return 0;
    }

    sec_out = le32_to_cpu(out.flags);
    /* cache security state */
    (*mds).security.state = sec_out;

    if ptype == NVDIMM_MASTER {
        if sec_out & CXL_PMEM_SEC_STATE_MASTER_PASS_SET != 0 {
            set_bit(NVDIMM_SECURITY_UNLOCKED, &mut security_flags);
        } else {
            set_bit(NVDIMM_SECURITY_DISABLED, &mut security_flags);
        }
        if sec_out & CXL_PMEM_SEC_STATE_MASTER_PLIMIT != 0 {
            set_bit(NVDIMM_SECURITY_FROZEN, &mut security_flags);
        }
        return security_flags;
    }

    if sec_out & CXL_PMEM_SEC_STATE_USER_PASS_SET != 0 {
        if sec_out & CXL_PMEM_SEC_STATE_FROZEN != 0
            || sec_out & CXL_PMEM_SEC_STATE_USER_PLIMIT != 0
        {
            set_bit(NVDIMM_SECURITY_FROZEN, &mut security_flags);
        }

        if sec_out & CXL_PMEM_SEC_STATE_LOCKED != 0 {
            set_bit(NVDIMM_SECURITY_LOCKED, &mut security_flags);
        } else {
            set_bit(NVDIMM_SECURITY_UNLOCKED, &mut security_flags);
        }
    } else {
        set_bit(NVDIMM_SECURITY_DISABLED, &mut security_flags);
    }

    security_flags
}

unsafe fn cxl_pmem_security_change_key(
    nvdimm: *mut nvdimm,
    old_data: *const nvdimm_key_data,
    new_data: *const nvdimm_key_data,
    ptype: nvdimm_passphrase_type,
) -> c_int {
    let cxl_nvd = nvdimm_provider_data(nvdimm);
    let cxlmd = (*cxl_nvd).cxlmd;
    let cxl_mbox = &mut (*(*cxlmd).cxlds).cxl_mbox;
    let mut set_pass = cxl_set_pass {
        r#type: if ptype == NVDIMM_MASTER { CXL_PMEM_SEC_PASS_MASTER } else { CXL_PMEM_SEC_PASS_USER },
        old_pass: [0; NVDIMM_PASSPHRASE_LEN],
        new_pass: [0; NVDIMM_PASSPHRASE_LEN],
    };
    let mut mbox_cmd: cxl_mbox_cmd;

    memcpy(set_pass.old_pass.as_mut_ptr() as *mut c_void, (*old_data).data.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN);
    memcpy(set_pass.new_pass.as_mut_ptr() as *mut c_void, (*new_data).data.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN);
    mbox_cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_SET_PASSPHRASE, size_in: core::mem::size_of::<cxl_set_pass>(), size_out: 0, payload_in: &mut set_pass as *mut _ as *mut c_void, payload_out: core::ptr::null_mut() };
    cxl_internal_send_cmd(cxl_mbox, &mut mbox_cmd)
}

unsafe fn __cxl_pmem_security_disable(nvdimm: *mut nvdimm, key_data: *const nvdimm_key_data, ptype: nvdimm_passphrase_type) -> c_int {
    let cxl_nvd = nvdimm_provider_data(nvdimm);
    let cxlmd = (*cxl_nvd).cxlmd;
    let cxl_mbox = &mut (*(*cxlmd).cxlds).cxl_mbox;
    let mut dis_pass = cxl_disable_pass { r#type: if ptype == NVDIMM_MASTER { CXL_PMEM_SEC_PASS_MASTER } else { CXL_PMEM_SEC_PASS_USER }, pass: [0; NVDIMM_PASSPHRASE_LEN] };
    memcpy(dis_pass.pass.as_mut_ptr() as *mut c_void, (*key_data).data.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN);
    let mut mbox_cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_DISABLE_PASSPHRASE, size_in: core::mem::size_of::<cxl_disable_pass>(), size_out: 0, payload_in: &mut dis_pass as *mut _ as *mut c_void, payload_out: core::ptr::null_mut() };
    cxl_internal_send_cmd(cxl_mbox, &mut mbox_cmd)
}

unsafe fn cxl_pmem_security_disable(nvdimm: *mut nvdimm, key_data: *const nvdimm_key_data) -> c_int { __cxl_pmem_security_disable(nvdimm, key_data, NVDIMM_USER) }
unsafe fn cxl_pmem_security_disable_master(nvdimm: *mut nvdimm, key_data: *const nvdimm_key_data) -> c_int { __cxl_pmem_security_disable(nvdimm, key_data, NVDIMM_MASTER) }

unsafe fn cxl_pmem_security_freeze(nvdimm: *mut nvdimm) -> c_int {
    let cxl_nvd = nvdimm_provider_data(nvdimm);
    let cxlmd = (*cxl_nvd).cxlmd;
    let cxl_mbox = &mut (*(*cxlmd).cxlds).cxl_mbox;
    let mut mbox_cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_FREEZE_SECURITY, size_in: 0, size_out: 0, payload_in: core::ptr::null_mut(), payload_out: core::ptr::null_mut() };
    cxl_internal_send_cmd(cxl_mbox, &mut mbox_cmd)
}

unsafe fn cxl_pmem_security_unlock(nvdimm: *mut nvdimm, key_data: *const nvdimm_key_data) -> c_int {
    let cxl_nvd = nvdimm_provider_data(nvdimm);
    let cxlmd = (*cxl_nvd).cxlmd;
    let cxl_mbox = &mut (*(*cxlmd).cxlds).cxl_mbox;
    let mut pass = [0u8; NVDIMM_PASSPHRASE_LEN];
    memcpy(pass.as_mut_ptr() as *mut c_void, (*key_data).data.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN);
    let mut mbox_cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_UNLOCK, size_in: NVDIMM_PASSPHRASE_LEN, size_out: 0, payload_in: pass.as_mut_ptr() as *mut c_void, payload_out: core::ptr::null_mut() };
    cxl_internal_send_cmd(cxl_mbox, &mut mbox_cmd)
}

unsafe fn cxl_pmem_security_passphrase_erase(nvdimm: *mut nvdimm, key: *const nvdimm_key_data, ptype: nvdimm_passphrase_type) -> c_int {
    let cxl_nvd = nvdimm_provider_data(nvdimm);
    let cxlmd = (*cxl_nvd).cxlmd;
    let cxl_mbox = &mut (*(*cxlmd).cxlds).cxl_mbox;
    let mut erase = cxl_pass_erase { r#type: if ptype == NVDIMM_MASTER { CXL_PMEM_SEC_PASS_MASTER } else { CXL_PMEM_SEC_PASS_USER }, pass: [0; NVDIMM_PASSPHRASE_LEN] };
    memcpy(erase.pass.as_mut_ptr() as *mut c_void, (*key).data.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN);
    let mut mbox_cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_PASSPHRASE_SECURE_ERASE, size_in: core::mem::size_of::<cxl_pass_erase>(), size_out: 0, payload_in: &mut erase as *mut _ as *mut c_void, payload_out: core::ptr::null_mut() };
    cxl_internal_send_cmd(cxl_mbox, &mut mbox_cmd)
}

static mut __CXL_SECURITY_OPS: nvdimm_security_ops = nvdimm_security_ops {
    get_flags: Some(cxl_pmem_get_security_flags),
    change_key: Some(cxl_pmem_security_change_key),
    disable: Some(cxl_pmem_security_disable),
    freeze: Some(cxl_pmem_security_freeze),
    unlock: Some(cxl_pmem_security_unlock),
    erase: Some(cxl_pmem_security_passphrase_erase),
    disable_master: Some(cxl_pmem_security_disable_master),
};

pub static mut cxl_security_ops: *const nvdimm_security_ops = unsafe { &raw const __CXL_SECURITY_OPS };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
