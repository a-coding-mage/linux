// SPDX-License-Identifier: GPL-2.0-only
/*
 * Bridge between MCE and APEI
 *
 * On some machine, corrected memory errors are reported via APEI
 * generic hardware error source (GHES) instead of corrected Machine
 * Check. These corrected memory errors can be reported to user space
 * through /dev/mcelog via faking a corrected Machine Check, so that
 * the error memory page can be offlined by /sbin/mcelog if the error
 * count for one page is beyond the threshold.
 *
 * For fatal MCE, save MCE record into persistent storage via ERST, so
 * that the MCE record can be logged after reboot via ERST.
 *
 * Copyright 2010 Intel Corp.
 *   Author: Huang Ying <ying.huang@intel.com>
 */

// External kernel declarations supplied by the surrounding crate.

pub unsafe fn apei_mce_report_mem_error(severity: i32, mem_err: *mut cper_sec_mem_err) {
    let mut err: mce_hw_err = core::mem::zeroed();
    let m: *mut mce = &mut err.m;
    let mut lsb: i32;

    if (*mem_err).validation_bits & CPER_MEM_VALID_PA == 0 {
        return;
    }

    /*
     * Even if the ->validation_bits are set for address mask,
     * to be extra safe, check and reject an error radius '0',
     * and fall back to the default page size.
     */
    if (*mem_err).validation_bits & CPER_MEM_VALID_PA_MASK != 0 {
        lsb = find_first_bit(
            &(*mem_err).physical_addr_mask as *const _ as *const core::ffi::c_void,
            PAGE_SHIFT,
        ) as i32;
    } else {
        lsb = PAGE_SHIFT as i32;
    }

    mce_prep_record(&mut err);
    (*m).bank = -1;
    /* Fake a memory read error with unknown channel */
    (*m).status = MCI_STATUS_VAL | MCI_STATUS_EN | MCI_STATUS_ADDRV | MCI_STATUS_MISCV | 0x9f;
    (*m).misc = (MCI_MISC_ADDR_PHYS << 6) | lsb as u64;

    if severity >= GHES_SEV_RECOVERABLE {
        (*m).status |= MCI_STATUS_UC;
    }

    if severity >= GHES_SEV_PANIC {
        (*m).status |= MCI_STATUS_PCC;
        (*m).tsc = rdtsc();
    }

    (*m).addr = (*mem_err).physical_addr;
    mce_log(&mut err);
}

pub unsafe fn apei_smca_report_x86_error(ctx_info: *mut cper_ia_proc_ctx, lapic_id: u64) -> i32 {
    let i_mce = (ctx_info.add(1)) as *const u64;
    let mut cpu: u32 = 0;
    let mut num_regs: u32;
    let mut apicid_found = false;
    let mut err: mce_hw_err = core::mem::zeroed();
    let m: *mut mce = &mut err.m;

    if !boot_cpu_has(X86_FEATURE_SMCA) {
        return -EINVAL;
    }

    /* Match any MCi_STATUS register by turning off bank numbers. */
    if (*ctx_info).msr_addr & MSR_AMD64_SMCA_MC0_STATUS != MSR_AMD64_SMCA_MC0_STATUS {
        return -EINVAL;
    }

    /* Sanity-check registers array size. */
    num_regs = (*ctx_info).reg_arr_size >> 3;
    if num_regs == 0 {
        return -EINVAL;
    }

    for_each_possible_cpu!(cpu) {
        if cpu_data(cpu).topo.initial_apicid == lapic_id {
            apicid_found = true;
            break;
        }
    }

    if !apicid_found {
        return -EINVAL;
    }

    core::ptr::write_bytes(&mut err as *mut mce_hw_err, 0, 1);
    mce_prep_record_common(m);
    mce_prep_record_per_cpu(cpu, m);
    (*m).bank = ((*ctx_info).msr_addr >> 4) & 0xFF;

    if num_regs > 15 {
        num_regs = 15;
    }

    match num_regs {
        15 => { err.vendor.amd.synd2 = *i_mce.add(14); }
        _ => {}
    }
    if num_regs >= 14 { err.vendor.amd.synd1 = *i_mce.add(13); }
    if num_regs >= 6 { (*m).synd = *i_mce.add(5); }
    if num_regs >= 5 { (*m).ipid = *i_mce.add(4); }
    if num_regs >= 3 { (*m).misc = *i_mce.add(2); }
    if num_regs >= 2 { (*m).addr = *i_mce.add(1); }
    (*m).status = *i_mce;

    mce_log(&mut err);
    0
}

pub const CPER_CREATOR_MCE: guid_t = GUID_INIT!(0x75a574e3, 0x5052, 0x4b29, 0x8a, 0x8e, 0xbe, 0x2c, 0x64, 0x90, 0xb8, 0x9d);
pub const CPER_SECTION_TYPE_MCE: guid_t = GUID_INIT!(0xfe08ffbe, 0x95e4, 0x4be7, 0xbc, 0x73, 0x40, 0x96, 0x04, 0x4a, 0x38, 0xfc);

/* CPER specification requires byte-packed. */
#[repr(C, packed)]
pub struct cper_mce_record {
    pub hdr: cper_record_header,
    pub sec_hdr: cper_section_descriptor,
    pub mce: mce,
}

pub unsafe fn apei_write_mce(m: *mut mce) -> i32 {
    let mut rcd: cper_mce_record = core::mem::zeroed();
    core::ptr::write_bytes(&mut rcd as *mut cper_mce_record, 0, 1);
    core::ptr::copy_nonoverlapping(CPER_SIG_RECORD.as_ptr(), rcd.hdr.signature.as_mut_ptr(), CPER_SIG_SIZE);
    rcd.hdr.revision = CPER_RECORD_REV;
    rcd.hdr.signature_end = CPER_SIG_END;
    rcd.hdr.section_count = 1;
    rcd.hdr.error_severity = CPER_SEV_FATAL;
    rcd.hdr.validation_bits = 0;
    rcd.hdr.record_length = core::mem::size_of::<cper_mce_record>() as _;
    rcd.hdr.creator_id = CPER_CREATOR_MCE;
    rcd.hdr.notification_type = CPER_NOTIFY_MCE;
    rcd.hdr.record_id = cper_next_record_id();
    rcd.hdr.flags = CPER_HW_ERROR_FLAGS_PREVERR;
    rcd.sec_hdr.section_offset = core::mem::offset_of!(cper_mce_record, mce) as _;
    rcd.sec_hdr.section_length = core::mem::size_of::<mce>() as _;
    rcd.sec_hdr.revision = CPER_SEC_REV;
    rcd.sec_hdr.validation_bits = 0;
    rcd.sec_hdr.flags = CPER_SEC_PRIMARY;
    rcd.sec_hdr.section_type = CPER_SECTION_TYPE_MCE;
    rcd.sec_hdr.section_severity = CPER_SEV_FATAL;
    core::ptr::copy_nonoverlapping(m, &mut rcd.mce, 1);
    erst_write(&mut rcd.hdr)
}

pub unsafe fn apei_read_mce(m: *mut mce, record_id: *mut u64) -> isize {
    let mut rcd: cper_mce_record = core::mem::zeroed();
    let mut pos: i32 = 0;
    let mut rc = erst_get_record_id_begin(&mut pos);
    if rc != 0 { return rc as isize; }
    loop {
        rc = erst_get_record_id_next(&mut pos, record_id);
        if rc != 0 { break; }
        if *record_id == APEI_ERST_INVALID_RECORD_ID { break; }
        rc = erst_read_record(*record_id, &mut rcd.hdr, core::mem::size_of::<cper_mce_record>(), core::mem::size_of::<cper_mce_record>(), &CPER_CREATOR_MCE);
        if rc == -ENOENT { continue; }
        if rc < 0 { break; }
        core::ptr::copy_nonoverlapping(&rcd.mce, m, 1);
        rc = core::mem::size_of::<mce>() as i32;
        break;
    }
    erst_get_record_id_end();
    rc as isize
}

/* Check whether there is record in ERST */
pub unsafe fn apei_check_mce() -> i32 { erst_get_record_count() }

pub unsafe fn apei_clear_mce(record_id: u64) -> i32 { erst_clear(record_id) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
