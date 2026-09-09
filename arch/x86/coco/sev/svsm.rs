// SPDX-License-Identifier: GPL-2.0-only
/*
 * SVSM support code
 */

// Linux/kernel dependencies supplied by other translation units.

/* For early boot SVSM communication */
#[repr(C)]
pub static mut boot_svsm_ca_page: svsm_ca = unsafe { core::mem::zeroed() };

/*
 * SVSM related information:
 *   During boot, the page tables are set up as identity mapped and later
 *   changed to use kernel virtual addresses. Maintain separate virtual and
 *   physical addresses for the CAA to allow SVSM functions to be used during
 *   early boot, both with identity mapped virtual addresses and proper kernel
 *   virtual addresses.
 */
pub static mut boot_svsm_caa_pa: u64 = 0;

// Per-CPU declarations corresponding to DEFINE_PER_CPU.
extern "C" {
    static mut svsm_caa: *mut svsm_ca;
    static mut svsm_caa_pa: u64;
}

unsafe fn svsm_perform_ghcb_protocol(ghcb: *mut ghcb, call: *mut svsm_call) -> i32 {
    let mut ctxt: es_em_ctxt = core::mem::zeroed();
    let mut pending: u8 = 0;

    vc_ghcb_invalidate(ghcb);

    /*
     * Fill in protocol and format specifiers. This can be called very early
     * in the boot, so use rip-relative references as needed.
     */
    (*ghcb).protocol_version = ghcb_version;
    (*ghcb).ghcb_usage = GHCB_DEFAULT_USAGE;

    ghcb_set_sw_exit_code(ghcb, SVM_VMGEXIT_SNP_RUN_VMPL);
    ghcb_set_sw_exit_info_1(ghcb, 0);
    ghcb_set_sw_exit_info_2(ghcb, 0);

    sev_es_wr_ghcb_msr(__pa(ghcb));

    svsm_issue_call(call, &mut pending);

    if pending != 0 {
        return -EINVAL;
    }

    match verify_exception_info(ghcb, &mut ctxt) {
        ES_OK => {}
        ES_EXCEPTION => {
            vc_forward_exception(&mut ctxt);
            return -EINVAL;
        }
        _ => return -EINVAL,
    }

    svsm_process_result_codes(call)
}

pub unsafe fn svsm_perform_call_protocol(call: *mut svsm_call) -> i32 {
    let mut state: ghcb_state = core::mem::zeroed();
    let flags: c_ulong = native_local_irq_save();
    let ghcb = __sev_get_ghcb(&mut state);
    let mut ret: i32;

    loop {
        ret = if !ghcb.is_null() {
            svsm_perform_ghcb_protocol(ghcb, call)
        } else {
            __pi_svsm_perform_msr_protocol(call)
        };
        if ret != -EAGAIN {
            break;
        }
    }

    __sev_put_ghcb(&mut state);
    native_local_irq_restore(flags);
    ret
}

unsafe fn svsm_build_ca_from_pfn_range(
    mut pfn: u64,
    pfn_end: u64,
    action: bool,
    pc: *mut svsm_pvalidate_call,
) -> u64 {
    (*pc).num_entries = 0;
    (*pc).cur_index = 0;

    let mut pe = (*pc).entry.as_mut_ptr();
    while pfn < pfn_end {
        (*pe).page_size = RMP_PG_SIZE_4K;
        (*pe).action = action;
        (*pe).ignore_cf = 0;
        (*pe).rsvd = 0;
        (*pe).pfn = pfn;
        pe = pe.add(1);
        pfn += 1;
        (*pc).num_entries += 1;
        if (*pc).num_entries == SVSM_PVALIDATE_MAX_COUNT {
            break;
        }
    }
    pfn
}

unsafe fn svsm_build_ca_from_psc_desc(
    desc: *mut snp_psc_desc,
    mut desc_entry: c_uint,
    pc: *mut svsm_pvalidate_call,
) -> c_int {
    (*pc).num_entries = 0;
    (*pc).cur_index = 0;
    let mut pe = (*pc).entry.as_mut_ptr();
    let mut e = (*desc).entries.as_mut_ptr().add(desc_entry as usize);

    while desc_entry <= (*desc).hdr.end_entry {
        (*pe).page_size = if (*e).pagesize != 0 { RMP_PG_SIZE_2M } else { RMP_PG_SIZE_4K };
        (*pe).action = (*e).operation == SNP_PAGE_STATE_PRIVATE;
        (*pe).ignore_cf = 0;
        (*pe).rsvd = 0;
        (*pe).pfn = (*e).gfn;
        pe = pe.add(1);
        e = e.add(1);
        desc_entry += 1;
        (*pc).num_entries += 1;
        if (*pc).num_entries == SVSM_PVALIDATE_MAX_COUNT {
            break;
        }
    }
    desc_entry as c_int
}

unsafe fn svsm_pval_terminate(pc: *mut svsm_pvalidate_call, ret: c_int, svsm_ret: u64) {
    let entry = &(*pc).entry[(*pc).cur_index as usize];
    __pval_terminate(entry.pfn, entry.action, entry.page_size, ret, svsm_ret);
}

pub unsafe fn svsm_pval_pages(desc: *mut snp_psc_desc) {
    let mut pv_4k: [svsm_pvalidate_entry; VMGEXIT_PSC_MAX_ENTRY as usize] = core::mem::zeroed();
    let mut i: c_uint;
    let mut pv_4k_count: c_uint = 0;
    let mut call: svsm_call = core::mem::zeroed();
    let flags = native_local_irq_save();
    let mut action: bool;
    let mut ret: c_int;
    let pc = svsm_get_caa().cast::<svsm_pvalidate_call>();
    let pc_pa = svsm_get_caa_pa() + core::mem::offset_of!(svsm_ca, svsm_buffer) as u64;

    call.caa = svsm_get_caa();
    call.rax = SVSM_CORE_CALL(SVSM_CORE_PVALIDATE);
    call.rcx = pc_pa;

    i = 0;
    while i <= (*desc).hdr.end_entry {
        i = svsm_build_ca_from_psc_desc(desc, i, pc) as c_uint;
        loop {
            ret = svsm_perform_call_protocol(&mut call);
            if ret == 0 {
                continue;
            }
            if call.rax_out == SVSM_PVALIDATE_FAIL_SIZEMISMATCH
                && (*pc).entry[(*pc).cur_index as usize].page_size == RMP_PG_SIZE_2M
            {
                pv_4k[pv_4k_count as usize] = (*pc).entry[(*pc).cur_index as usize];
                pv_4k_count += 1;
                (*pc).cur_index += 1;
                ret = if (*pc).cur_index < (*pc).num_entries { -EAGAIN } else { 0 };
            }
            if ret != -EAGAIN { break; }
        }
        if ret != 0 { svsm_pval_terminate(pc, ret, call.rax_out); }
    }

    for n in 0..pv_4k_count as usize {
        action = pv_4k[n].action;
        let mut pfn = pv_4k[n].pfn;
        let pfn_end = pfn + 512;
        while pfn < pfn_end {
            pfn = svsm_build_ca_from_pfn_range(pfn, pfn_end, action, pc);
            ret = svsm_perform_call_protocol(&mut call);
            if ret != 0 { svsm_pval_terminate(pc, ret, call.rax_out); }
        }
    }
    native_local_irq_restore(flags);
}

unsafe fn update_attest_input(call: *mut svsm_call, input: *mut svsm_attest_call) {
    if (*call).rcx_out != (*call).rcx { (*input).manifest_buf.len = (*call).rcx_out; }
    if (*call).rdx_out != (*call).rdx { (*input).certificates_buf.len = (*call).rdx_out; }
    if (*call).r8_out != (*call).r8 { (*input).report_buf.len = (*call).r8_out; }
}

pub unsafe fn snp_issue_svsm_attest_req(
    call_id: u64, call: *mut svsm_call, input: *mut svsm_attest_call,
) -> c_int {
    if snp_vmpl == 0 { return -EINVAL; }
    let flags = local_irq_save();
    (*call).caa = svsm_get_caa();
    let ac = (*call).caa.cast::<svsm_attest_call>();
    let attest_call_pa = svsm_get_caa_pa() + core::mem::offset_of!(svsm_ca, svsm_buffer) as u64;
    *ac = *input;
    (*call).rax = call_id;
    (*call).rcx = attest_call_pa;
    (*call).rdx = u64::MAX;
    (*call).r8 = u64::MAX;
    let ret = svsm_perform_call_protocol(call);
    update_attest_input(call, input);
    local_irq_restore(flags);
    ret
}

pub unsafe fn snp_svsm_vtpm_send_command(buffer: *mut u8) -> c_int {
    let mut call: svsm_call = core::mem::zeroed();
    call.caa = svsm_get_caa();
    call.rax = SVSM_VTPM_CALL(SVSM_VTPM_CMD);
    call.rcx = __pa(buffer);
    svsm_perform_call_protocol(&mut call)
}

pub unsafe fn snp_svsm_vtpm_probe() -> bool {
    let mut call: svsm_call = core::mem::zeroed();
    if snp_vmpl == 0 { return false; }
    call.caa = svsm_get_caa();
    call.rax = SVSM_VTPM_CALL(SVSM_VTPM_QUERY);
    if svsm_perform_call_protocol(&mut call) != 0 { return false; }
    (call.rcx_out & (1u64 << 8)) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
