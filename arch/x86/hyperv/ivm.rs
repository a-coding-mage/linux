// SPDX-License-Identifier: GPL-2.0
/* Hyper-V Isolation VM interface with paravisor and hypervisor */

// Kernel dependencies supplied by the surrounding translation unit are intentionally external.

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
const GHCB_USAGE_HYPERV_CALL: u32 = 1;

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[repr(C, packed)]
pub union hv_ghcb {
    pub ghcb: ghcb,
    pub hypercall: hv_ghcb_hypercall,
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[repr(C, packed)]
pub struct hv_ghcb_hypercall {
    pub hypercalldata: [u64; 509],
    pub outputgpa: u64,
    pub hypercallinput: hv_ghcb_hypercallinput,
    pub hypercalloutput: hv_ghcb_hypercalloutput,
    pub reserved2: u64,
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[repr(C)]
pub union hv_ghcb_hypercallinput {
    pub fields: hv_ghcb_hypercallinput_fields,
    pub asuint64: u64,
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[repr(C)]
pub struct hv_ghcb_hypercallinput_fields {
    pub callcode: u32,
    pub isfast: u32,
    pub reserved1: u32,
    pub isnested: u32,
    pub countofelements: u32,
    pub reserved2: u32,
    pub repstartindex: u32,
    pub reserved3: u32,
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[repr(C)]
pub union hv_ghcb_hypercalloutput {
    pub fields: hv_ghcb_hypercalloutput_fields,
    pub asunit64: u64,
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[repr(C)]
pub struct hv_ghcb_hypercalloutput_fields {
    pub callstatus: u16,
    pub reserved1: u16,
    pub elementsprocessed: u32,
    pub reserved2: u32,
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
static mut hv_ghcb_version: u16 = 0;

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn hv_ghcb_hypercall(control: u64, input: *mut core::ffi::c_void,
                                output: *mut core::ffi::c_void, input_size: u32) -> u64 {
    if hv_ghcb_pg.is_null() { return (-14i64) as u64; }
    WARN_ON(in_nmi());
    let flags: usize;
    local_irq_save(&mut { flags });
    let ghcb_base = this_cpu_ptr(hv_ghcb_pg) as *mut *mut core::ffi::c_void;
    let hv_ghcb = *ghcb_base as *mut hv_ghcb;
    if hv_ghcb.is_null() { local_irq_restore(flags); return (-14i64) as u64; }
    (*hv_ghcb).ghcb.protocol_version = GHCB_PROTOCOL_MAX;
    (*hv_ghcb).ghcb.ghcb_usage = GHCB_USAGE_HYPERV_CALL;
    (*hv_ghcb).hypercall.outputgpa = output as u64;
    (*hv_ghcb).hypercall.hypercallinput.asuint64 = 0;
    (*hv_ghcb).hypercall.hypercallinput.fields.callcode = control as u32;
    if input_size != 0 { memcpy((*hv_ghcb).hypercall.hypercalldata.as_mut_ptr() as *mut _, input, input_size as usize); }
    VMGEXIT();
    (*hv_ghcb).ghcb.ghcb_usage = 0xffff_ffff;
    memset((*hv_ghcb).ghcb.save.valid_bitmap.as_mut_ptr() as *mut _, 0, core::mem::size_of_val(&(*hv_ghcb).ghcb.save.valid_bitmap));
    let status = (*hv_ghcb).hypercall.hypercalloutput.fields.callstatus as u64;
    local_irq_restore(flags);
    status
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[inline] unsafe fn rd_ghcb_msr() -> u64 { native_rdmsrq(MSR_AMD64_SEV_ES_GHCB) }
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[inline] unsafe fn wr_ghcb_msr(val: u64) { native_wrmsrq(MSR_AMD64_SEV_ES_GHCB, val) }

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn hv_ghcb_hv_call(g: *mut ghcb, exit_code: u64, exit_info_1: u64, exit_info_2: u64) -> es_result {
    (*g).protocol_version = hv_ghcb_version;
    (*g).ghcb_usage = GHCB_DEFAULT_USAGE;
    ghcb_set_sw_exit_code(g, exit_code);
    ghcb_set_sw_exit_info_1(g, exit_info_1);
    ghcb_set_sw_exit_info_2(g, exit_info_2);
    VMGEXIT();
    if (*g).save.sw_exit_info_1 & GENMASK_ULL(31, 0) != 0 { ES_VMM_ERROR } else { ES_OK }
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn hv_ghcb_terminate(set: u32, reason: u32) -> ! {
    let mut val = GHCB_MSR_TERM_REQ;
    val |= GHCB_SEV_TERM_REASON(set, reason);
    wr_ghcb_msr(val); VMGEXIT();
    loop { core::arch::asm!("hlt", options(nostack, preserves_flags)); }
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn hv_ghcb_negotiate_protocol() -> bool {
    let ghcb_gpa = rd_ghcb_msr();
    wr_ghcb_msr(GHCB_MSR_SEV_INFO_REQ); VMGEXIT();
    let val = rd_ghcb_msr();
    if GHCB_MSR_INFO(val) != GHCB_MSR_SEV_INFO_RESP { return false; }
    if GHCB_MSR_PROTO_MAX(val) < GHCB_PROTOCOL_MIN || GHCB_MSR_PROTO_MIN(val) > GHCB_PROTOCOL_MAX { return false; }
    hv_ghcb_version = core::cmp::min(GHCB_MSR_PROTO_MAX(val), GHCB_PROTOCOL_MAX);
    wr_ghcb_msr(ghcb_gpa); VMGEXIT(); true
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn hv_ghcb_msr_write(msr: u64, value: u64) {
    if hv_ghcb_pg.is_null() { return; }
    WARN_ON(in_nmi()); let flags: usize; local_irq_save(&mut { flags });
    let hv = *(this_cpu_ptr(hv_ghcb_pg) as *mut *mut hv_ghcb);
    if hv.is_null() { local_irq_restore(flags); return; }
    ghcb_set_rcx(&mut (*hv).ghcb, msr); ghcb_set_rax(&mut (*hv).ghcb, lower_32_bits(value)); ghcb_set_rdx(&mut (*hv).ghcb, upper_32_bits(value));
    if hv_ghcb_hv_call(&mut (*hv).ghcb, SVM_EXIT_MSR, 1, 0) != ES_OK { pr_warn!("Fail to write msr via ghcb %llx.\n", msr); }
    local_irq_restore(flags);
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn hv_ghcb_msr_read(msr: u64, value: *mut u64) {
    if hv_ghcb_pg.is_null() { return; }
    WARN_ON(in_nmi()); let flags: usize; local_irq_save(&mut { flags });
    let hv = *(this_cpu_ptr(hv_ghcb_pg) as *mut *mut hv_ghcb);
    if hv.is_null() { local_irq_restore(flags); return; }
    ghcb_set_rcx(&mut (*hv).ghcb, msr);
    if hv_ghcb_hv_call(&mut (*hv).ghcb, SVM_EXIT_MSR, 0, 0) != ES_OK { pr_warn!("Fail to read msr via ghcb %llx.\n", msr); }
    else { *value = lower_32_bits((*hv).ghcb.save.rax) as u64 | ((lower_32_bits((*hv).ghcb.save.rdx) as u64) << 32); }
    local_irq_restore(flags);
}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
unsafe fn hv_ghcb_msr_write(_: u64, _: u64) {}
#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
unsafe fn hv_ghcb_msr_read(_: u64, _: *mut u64) {}

pub unsafe fn hv_snp_hypercall(control: u64, param1: u64, param2: u64) -> u64 {
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)] { let mut a = control; let mut d = param1; let mut r8 = param2; core::arch::asm!("vmmcall", inout("rax") a => _, inout("rcx") a, inout("rdx") d, inout("r8") r8, lateout("r9") _, lateout("r10") _, lateout("r11") _, options(nostack)); a }
    #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))] { U64_MAX }
}

#[cfg(CONFIG_INTEL_TDX_GUEST)]
unsafe fn hv_tdx_msr_write(msr: u64, val: u64) { let mut args = tdx_module_args { r10: TDX_HYPERCALL_STANDARD, r11: EXIT_REASON_MSR_WRITE, r12: msr, r13: val, ..core::mem::zeroed() }; let ret = __tdx_hypercall(&mut args); WARN_ONCE(ret != 0, "Failed to emulate MSR write: %lld\n", ret); }
#[cfg(CONFIG_INTEL_TDX_GUEST)]
unsafe fn hv_tdx_msr_read(msr: u64, val: *mut u64) { let mut args = tdx_module_args { r10: TDX_HYPERCALL_STANDARD, r11: EXIT_REASON_MSR_READ, r12: msr, ..core::mem::zeroed() }; let ret = __tdx_hypercall(&mut args); if WARN_ONCE(ret != 0, "Failed to emulate MSR read: %lld\n", ret) { *val = 0; } else { *val = args.r11; } }
#[cfg(CONFIG_INTEL_TDX_GUEST)]
pub unsafe fn hv_tdx_hypercall(control: u64, param1: u64, param2: u64) -> u64 { let mut args: tdx_module_args = core::mem::zeroed(); args.r10=control; args.rdx=param1; args.r8=param2; let _ = __tdx_hypercall(&mut args); args.r11 }
#[cfg(not(CONFIG_INTEL_TDX_GUEST))]
unsafe fn hv_tdx_msr_write(_: u64, _: u64) {}
#[cfg(not(CONFIG_INTEL_TDX_GUEST))]
unsafe fn hv_tdx_msr_read(_: u64, _: *mut u64) {}
#[cfg(not(CONFIG_INTEL_TDX_GUEST))]
pub unsafe fn hv_tdx_hypercall(_: u64, _: u64, _: u64) -> u64 { U64_MAX }

#[cfg(any(CONFIG_AMD_MEM_ENCRYPT, CONFIG_INTEL_TDX_GUEST))]
pub unsafe fn hv_ivm_msr_write(msr: u64, value: u64) { if !ms_hyperv.paravisor_present { return; } if hv_isolation_type_tdx() { hv_tdx_msr_write(msr,value); } else if hv_isolation_type_snp() { hv_ghcb_msr_write(msr,value); } }
#[cfg(any(CONFIG_AMD_MEM_ENCRYPT, CONFIG_INTEL_TDX_GUEST))]
pub unsafe fn hv_ivm_msr_read(msr: u64, value: *mut u64) { if !ms_hyperv.paravisor_present { return; } if hv_isolation_type_tdx() { hv_tdx_msr_read(msr,value); } else if hv_isolation_type_snp() { hv_ghcb_msr_read(msr,value); } }

pub unsafe fn hv_get_isolation_type() -> hv_isolation_type { if ms_hyperv.priv_high & HV_ISOLATION == 0 { HV_ISOLATION_TYPE_NONE } else { FIELD_GET(HV_ISOLATION_TYPE, ms_hyperv.isolation_config_b) } }
pub unsafe fn hv_is_isolation_supported() -> bool { cpu_feature_enabled(X86_FEATURE_HYPERVISOR) && hypervisor_is_type(X86_HYPER_MS_HYPERV) && hv_get_isolation_type() != HV_ISOLATION_TYPE_NONE }
pub unsafe fn hv_isolation_type_snp() -> bool { static_branch_unlikely(&isolation_type_snp) }
pub unsafe fn hv_isolation_type_tdx() -> bool { static_branch_unlikely(&isolation_type_tdx) }

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
static mut ap_start_input_arg: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
static mut ap_start_stack: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn snp_set_vmsa(va: *mut core::ffi::c_void, vmsa: bool) -> u64 {
    let mut attrs = 1u64;
    if vmsa { attrs |= RMPADJUST_VMSA_PAGE_BIT; }
    rmpadjust(va as usize, RMP_PG_SIZE_4K, attrs)
}
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn snp_cleanup_vmsa(vmsa: *mut sev_es_save_area) {
    let err = snp_set_vmsa(vmsa as *mut _, false);
    if err != 0 { pr_err!("clear VMSA page failed (%u), leaking page\n", err); }
    else { free_page(vmsa as usize); }
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn hv_snp_boot_ap(apic_id: u32, _start_ip: usize, cpu: u32) -> u64 {
    let vmsa = __get_free_page(GFP_KERNEL | __GFP_ZERO) as *mut sev_es_save_area;
    if vmsa.is_null() { return (-12i64) as u64; }
    let vp_index = hv_apicid_to_vp_index(apic_id);
    if vp_index < 0 || vp_index > ms_hyperv.max_vp_index { return (-22i64) as u64; }
    let mut gdtr: desc_ptr = core::mem::zeroed(); native_store_gdt(&mut gdtr);
    (*vmsa).gdtr.base = gdtr.address; (*vmsa).gdtr.limit = gdtr.size;
    savesegment!(es, (*vmsa).es.selector); savesegment!(cs, (*vmsa).cs.selector);
    savesegment!(ss, (*vmsa).ss.selector); savesegment!(ds, (*vmsa).ds.selector);
    (*vmsa).efer=native_read_msr(MSR_EFER); (*vmsa).cr4=native_read_cr4(); (*vmsa).cr3=__native_read_cr3(); (*vmsa).cr0=native_read_cr0();
    (*vmsa).xcr0=1; (*vmsa).g_pat=HV_AP_INIT_GPAT_DEFAULT; (*vmsa).rip=secondary_startup_64_no_verify as u64; (*vmsa).rsp=ap_start_stack.as_ptr().add(PAGE_SIZE) as u64;
    (*vmsa).vmpl=0; (*vmsa).sev_features=sev_status >> 2;
    let ret=snp_set_vmsa(vmsa as *mut _, true); if ret != 0 { free_page(vmsa as usize); return ret; }
    let flags: usize; local_irq_save(&mut { flags });
    let input = ap_start_input_arg.as_mut_ptr() as *mut hv_enable_vp_vtl; memset(input as *mut _,0,core::mem::size_of::<hv_enable_vp_vtl>());
    (*input).partition_id = -1i64 as u64; (*input).vp_index=vp_index as u32; (*input).target_vtl.target_vtl=ms_hyperv.vtl; (*input).vp_context=__pa(vmsa) | 1;
    let mut retry=5; let mut ret2;
    loop { ret2=hv_do_hypercall(HVCALL_START_VP,input as *mut _,core::ptr::null_mut()); if hv_result(ret2)!=HV_STATUS_TIME_OUT || retry==0 { break; } retry-=1; }
    local_irq_restore(flags); if !hv_result_success(ret2) { snp_cleanup_vmsa(vmsa); }
    per_cpu(hv_sev_vmsa,cpu); ret2
}

#[cfg(any(CONFIG_AMD_MEM_ENCRYPT, CONFIG_INTEL_TDX_GUEST))]
unsafe fn hv_vtom_clear_present(kbuffer: usize, pagecount: i32, _enc: bool) -> i32 { set_memory_np(kbuffer,pagecount) }
#[cfg(any(CONFIG_AMD_MEM_ENCRYPT, CONFIG_INTEL_TDX_GUEST))]
unsafe fn hv_vtom_tlb_flush_required(_: bool) -> bool { false }
#[cfg(any(CONFIG_AMD_MEM_ENCRYPT, CONFIG_INTEL_TDX_GUEST))]
unsafe fn hv_vtom_cache_flush_required() -> bool { false }
#[cfg(any(CONFIG_AMD_MEM_ENCRYPT, CONFIG_INTEL_TDX_GUEST))]
unsafe fn hv_is_private_mmio(addr: u64) -> bool { (addr >= HV_IOAPIC_BASE_ADDRESS && addr < HV_IOAPIC_BASE_ADDRESS+PAGE_SIZE as u64) || (addr >= VTPM_BASE_ADDRESS && addr < VTPM_BASE_ADDRESS+PAGE_SIZE as u64) }

#[cfg(any(CONFIG_AMD_MEM_ENCRYPT, CONFIG_INTEL_TDX_GUEST))]
pub unsafe fn hv_vtom_init() {
    let typ=hv_get_isolation_type();
    match typ { HV_ISOLATION_TYPE_VBS => {}, HV_ISOLATION_TYPE_SNP => { #[cfg(CONFIG_AMD_MEM_ENCRYPT)] { sev_status=MSR_AMD64_SNP_VTOM; cc_vendor=CC_VENDOR_AMD; } }, HV_ISOLATION_TYPE_TDX => { cc_vendor=CC_VENDOR_INTEL; }, _ => panic!("hv_vtom_init: unsupported isolation type %d\n",typ) }
    cc_set_mask(ms_hyperv.shared_gpa_boundary); physical_mask &= ms_hyperv.shared_gpa_boundary-1;
    x86_platform.hyper.is_private_mmio=Some(hv_is_private_mmio); x86_platform.guest.enc_cache_flush_required=Some(hv_vtom_cache_flush_required); x86_platform.guest.enc_tlb_flush_required=Some(hv_vtom_tlb_flush_required); x86_platform.guest.enc_status_change_prepare=Some(hv_vtom_clear_present); guest_force_mtrr_state(core::ptr::null_mut(),0,MTRR_TYPE_WRBACK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
