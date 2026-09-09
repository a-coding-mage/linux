/* SPDX-License-Identifier: GPL-2.0 */

pub const DR7_RESET_VALUE: u64 = 0x400;

extern "C" {
    pub static mut sev_hv_features: u64;
    pub static mut sev_secrets_pa: u64;
}

/* #VC handler runtime per-CPU data */
#[repr(C)]
pub struct sev_es_runtime_data {
    pub ghcb_page: ghcb,

    /*
     * Reserve one page per CPU as backup storage for the unencrypted GHCB.
     * It is needed when an NMI happens while the #VC handler uses the real
     * GHCB, and the NMI handler itself is causing another #VC exception. In
     * that case the GHCB content of the first handler needs to be backed up
     * and restored.
     */
    pub backup_ghcb: ghcb,

    /*
     * Mark the per-cpu GHCBs as in-use to detect nested #VC exceptions.
     * There is no need for it to be atomic, because nothing is written to
     * the GHCB between the read and the write of ghcb_active. So it is safe
     * to use it when a nested #VC exception happens before the write.
     *
     * This is necessary for example in the #VC->NMI->#VC case when the NMI
     * happens while the first #VC handler uses the GHCB. When the NMI code
     * raises a second #VC handler it might overwrite the contents of the
     * GHCB written by the first handler. To avoid this the content of the
     * GHCB is saved and restored when the GHCB is detected to be in use
     * already.
     */
    pub ghcb_active: bool,
    pub backup_ghcb_active: bool,

    /*
     * Cached DR7 value - write it on DR7 writes and return it on reads.
     * That value will never make it to the real hardware DR7 as debugging
     * is currently unsupported in SEV-ES guests.
     */
    pub dr7: usize,
}

#[repr(C)]
pub struct ghcb_state {
    pub ghcb: *mut ghcb,
}

extern "C" {
    pub static mut boot_svsm_ca_page: svsm_ca;

    pub fn __sev_get_ghcb(state: *mut ghcb_state) -> *mut ghcb;
    pub fn __sev_put_ghcb(state: *mut ghcb_state);

    pub static mut runtime_data: *mut sev_es_runtime_data;
    pub static mut sev_vmsa: *mut sev_es_save_area;

    pub fn early_set_pages_state(
        vaddr: usize,
        paddr: usize,
        npages: usize,
        desc: *const psc_desc,
    );

    pub static mut svsm_caa: *mut svsm_ca;
    pub static mut svsm_caa_pa: u64;
    pub static mut boot_svsm_caa_pa: u64;

    pub fn verify_exception_info(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt) -> es_result;
    pub fn vc_forward_exception(ctxt: *mut es_em_ctxt);
    pub fn svsm_pval_pages(desc: *mut snp_psc_desc);
    pub fn svsm_perform_call_protocol(call: *mut svsm_call) -> i32;
    pub fn snp_svsm_vtpm_probe() -> bool;

    pub fn kernel_exc_vmm_communication(regs: *mut pt_regs, error_code: usize);
    pub fn user_exc_vmm_communication(regs: *mut pt_regs, error_code: usize);

    pub fn native_rdmsrq(msr: u32) -> u64;
    pub fn native_wrmsr(msr: u32, low: u32, high: u32);

    pub fn __vc_handle_msr(
        ghcb: *mut ghcb,
        ctxt: *mut es_em_ctxt,
        write: bool,
    ) -> es_result;

    pub fn get_hv_features() -> u64;
    pub fn snp_cpuid_get_table() -> *const snp_cpuid_table;
}

#[inline]
pub unsafe fn sev_es_rd_ghcb_msr() -> u64 {
    native_rdmsrq(MSR_AMD64_SEV_ES_GHCB)
}

#[inline(always)]
pub unsafe fn sev_es_wr_ghcb_msr(val: u64) {
    let low = val as u32;
    let high = (val >> 32) as u32;

    native_wrmsr(MSR_AMD64_SEV_ES_GHCB, low, high);
}

#[inline]
pub unsafe fn svsm_get_caa() -> *mut svsm_ca {
    if sev_cfg.use_cas {
        svsm_caa
    } else {
        &mut boot_svsm_ca_page
    }
}

#[inline]
pub unsafe fn svsm_get_caa_pa() -> u64 {
    if sev_cfg.use_cas {
        svsm_caa_pa
    } else {
        boot_svsm_caa_pa
    }
}

#[inline(always)]
pub unsafe fn __pval_terminate(
    pfn: u64,
    action: bool,
    page_size: u32,
    ret: i32,
    svsm_ret: u64,
) {
    WARN!(
        1,
        "PVALIDATE failure: pfn: 0x%llx, action: %u, size: %u, ret: %d, svsm_ret: 0x%llx\n",
        pfn,
        action,
        page_size,
        ret,
        svsm_ret
    );

    sev_es_terminate(SEV_TERM_SET_LINUX, GHCB_TERM_PVALIDATE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
