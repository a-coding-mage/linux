// SPDX-License-Identifier: GPL-2.0
/*
 * AMD Encrypted Register State Support
 *
 * This file is shared between pre-decompression boot code and the running
 * Linux kernel.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

// C headers and build-time configuration are supplied by the including codebase.

extern "C" {
    static mut ghcb_version: u64;
    static mut sev_snp_needs_sfw: bool;
    static mut snp_vmpl: u8;
    static mut boot_svsm_caa_pa: u64;

    fn sev_es_wr_ghcb_msr(value: u64);
    fn sev_es_rd_ghcb_msr() -> u64;
    fn vmgexit();
    fn xchg(ptr: *mut u8, value: u8) -> u8;
    fn cpuid_function_is_indexed(function: u32) -> bool;
    fn native_read_cr4() -> u64;
    fn xgetbv(index: u32) -> u64;
    fn native_local_irq_save() -> usize;
    fn native_local_irq_restore(flags: usize);
    fn pvalidate(vaddr: usize, page_size: u64, validate: bool) -> i32;
    fn sev_evict_cache(addr: *mut c_void, pages: usize);
    fn rmpadjust(addr: usize, page_size: u64, vmpl: u32) -> i32;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

const EAGAIN: i32 = 11;
const EINVAL: i32 = 22;
const EIO: i32 = 5;
const EOPNOTSUPP: i32 = 95;
const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: usize = 12;
const RMP_PG_SIZE_4K: u64 = 0;
const XCR_XFEATURE_ENABLED_MASK: u32 = 0;
const MSR_IA32_XSS: u32 = 0xDA0;

// External constants/macros from the surrounding kernel sources.
extern "C" {
    static GHCB_MSR_TERM_REQ: u64;
    static GHCB_MSR_HV_FT_REQ: u64;
    static GHCB_MSR_HV_FT_RESP: u64;
    static GHCB_MSR_VMPL_RESP: u64;
    static GHCB_MSR_CPUID_RESP: u64;
    static GHCB_MSR_PSC_RESP: u64;
}

#[repr(C)]
pub struct cpuid_leaf { pub fn_: u32, pub subfn: u32, pub eax: u32, pub ebx: u32, pub ecx: u32, pub edx: u32 }
#[repr(C)] pub struct setup_data { pub next: u64, pub type_: u32, pub len: u32 }
#[repr(C)] pub struct boot_params { pub hdr: boot_params_hdr }
#[repr(C)] pub struct boot_params_hdr { pub setup_data: u64 }
#[repr(C)] pub struct cc_setup_data { pub header: setup_data, pub cc_blob_address: u32 }
#[repr(C)] pub struct cc_blob_sev_info { pub cpuid_phys: u64, pub cpuid_len: u32, pub secrets_phys: u64, pub secrets_len: u32 }
#[repr(C)] pub struct svsm_ca { pub call_pending: u8, pub svsm_buffer: [u8; 4096] }
#[repr(C)] pub struct svsm_call { pub rax: usize, pub rcx: usize, pub rdx: usize, pub r8: usize, pub r9: usize, pub caa: *mut svsm_ca, pub rax_out: usize, pub rcx_out: usize, pub rdx_out: usize, pub r8_out: usize, pub r9_out: usize }
#[repr(C)] pub struct psc_desc { pub op: u32, pub ca: *mut svsm_ca, pub caa_pa: u64 }
#[repr(C)] pub struct pt_regs { pub ax: usize, pub bx: usize, pub cx: usize, pub dx: usize, pub ip: usize }
#[repr(C)] pub struct snp_cpuid_fn { pub eax_in: u32, pub ecx_in: u32, pub xcr0_in: u64, pub xss_in: u64, pub eax: u32, pub ebx: u32, pub ecx: u32, pub edx: u32 }
#[repr(C)] pub struct snp_cpuid_table { pub count: u32, pub fn_: [snp_cpuid_fn; 64] }
#[repr(C)] pub struct snp_secrets_page { pub svsm_size: u64, pub svsm_guest_vmpl: u8, pub _pad: [u8; 7], pub svsm_caa: u64 }
#[repr(C)] pub struct svsm_pvalidate_call { pub num_entries: u32, pub cur_index: u32, pub entry: [svsm_pvalidate_entry; 1] }
#[repr(C)] pub struct svsm_pvalidate_entry { pub page_size: u8, pub action: bool, pub ignore_cf: u8, pub rsvd: u8, pub pfn: u64 }

static mut cpuid_table_copy: snp_cpuid_table = unsafe { core::mem::zeroed() };
static mut cpuid_std_range_max: u32 = 0;
static mut cpuid_hyp_range_max: u32 = 0;
static mut cpuid_ext_range_max: u32 = 0;

#[inline] unsafe fn ghcb_resp_code(v: u64) -> u64 { v & 0xffff_ffff }
#[inline] unsafe fn lower_bits(v: usize) -> u32 { v as u32 }

pub unsafe fn sev_es_terminate(set: u32, reason: u32) -> ! {
    let mut val = GHCB_MSR_TERM_REQ;
    val |= ((set as u64) << 32) | reason as u64;
    sev_es_wr_ghcb_msr(val); vmgexit();
    loop { core::arch::asm!("hlt", options(nostack, preserves_flags)); }
}

pub unsafe fn get_hv_features() -> u64 {
    if ghcb_version < 2 { return 0; }
    sev_es_wr_ghcb_msr(GHCB_MSR_HV_FT_REQ); vmgexit();
    let val = sev_es_rd_ghcb_msr();
    if ghcb_resp_code(val) != GHCB_MSR_HV_FT_RESP { 0 } else { val >> 32 }
}

pub unsafe fn svsm_process_result_codes(call: *const svsm_call) -> i32 {
    match (*call).rax_out as u64 { 0 => 0, 1 | 2 => -EAGAIN, _ => -EINVAL }
}

pub unsafe fn svsm_issue_call(call: *mut svsm_call, pending: *mut u8) {
    (*(*call).caa).call_pending = 1;
    let mut rax = (*call).rax; let mut rcx = (*call).rcx; let mut rdx = (*call).rdx; let mut r8 = (*call).r8; let mut r9 = (*call).r9;
    core::arch::asm!("rep; vmmcall", inout("rax") rax, inout("rcx") rcx, inout("rdx") rdx, inout("r8") r8, inout("r9") r9, options(nostack));
    *pending = xchg(&mut (*(*call).caa).call_pending, *pending);
    (*call).rax_out = rax; (*call).rcx_out = rcx; (*call).rdx_out = rdx; (*call).r8_out = r8; (*call).r9_out = r9;
}

pub unsafe fn svsm_perform_msr_protocol(call: *mut svsm_call) -> i32 {
    let mut pending = 0; let val = sev_es_rd_ghcb_msr(); sev_es_wr_ghcb_msr(0); svsm_issue_call(call, &mut pending); let resp = sev_es_rd_ghcb_msr(); sev_es_wr_ghcb_msr(val);
    if pending != 0 || ghcb_resp_code(resp) != GHCB_MSR_VMPL_RESP || (resp >> 32) != 0 { return -EINVAL; } svsm_process_result_codes(call)
}

unsafe fn __sev_cpuid_hv(fn_: u32, reg_idx: u64, reg: *mut u32) -> i32 { sev_es_wr_ghcb_msr(((fn_ as u64) << 32) | reg_idx); vmgexit(); let val = sev_es_rd_ghcb_msr(); if ghcb_resp_code(val) != GHCB_MSR_CPUID_RESP { return -EIO; } *reg = (val >> 32) as u32; 0 }
unsafe fn __sev_cpuid_hv_msr(leaf: *mut cpuid_leaf) -> i32 { if cpuid_function_is_indexed((*leaf).fn_) && (*leaf).subfn != 0 { return -EINVAL; } let mut ret = __sev_cpuid_hv((*leaf).fn_, 0, &mut (*leaf).eax); if ret == 0 { ret = __sev_cpuid_hv((*leaf).fn_, 1, &mut (*leaf).ebx); } if ret == 0 { ret = __sev_cpuid_hv((*leaf).fn_, 2, &mut (*leaf).ecx); } if ret == 0 { ret = __sev_cpuid_hv((*leaf).fn_, 3, &mut (*leaf).edx); } ret }

pub unsafe fn snp_cpuid_get_table() -> *const snp_cpuid_table { &cpuid_table_copy }

unsafe fn snp_cpuid_calc_xsave_size(xfeatures_en: u64, compacted: bool) -> u32 { let t = &*snp_cpuid_get_table(); let mut found = 0; let mut size = 0x240; for i in 0..t.count as usize { let e=&t.fn_[i]; if !(e.eax_in==0xd && e.ecx_in>1 && e.ecx_in<64) || xfeatures_en & (1u64<<e.ecx_in)==0 || found & (1u64<<e.ecx_in)!=0 { continue; } found |= 1u64<<e.ecx_in; if compacted { size += e.eax; } else { size = core::cmp::max(size, e.eax + e.ebx); } } if found != (xfeatures_en & 0xffff_ffff_ffff_fffc) { 0 } else { size } }

unsafe fn snp_cpuid_get_validated_func(leaf: *mut cpuid_leaf) -> bool { let t=&*snp_cpuid_get_table(); for i in 0..t.count as usize { let e=&t.fn_[i]; if e.eax_in != (*leaf).fn_ || (cpuid_function_is_indexed((*leaf).fn_) && e.ecx_in != (*leaf).subfn) { continue; } if e.eax_in==0xd && (e.ecx_in==0 || e.ecx_in==1) && !(e.xcr0_in==1 || e.xcr0_in==3) { continue; } (*leaf).eax=e.eax; (*leaf).ebx=e.ebx; (*leaf).ecx=e.ecx; (*leaf).edx=e.edx; return true; } false }

unsafe fn snp_cpuid_hv_msr(_ctx: *mut c_void, leaf: *mut cpuid_leaf) { if __sev_cpuid_hv_msr(leaf) != 0 { sev_es_terminate(0, 0); } }
unsafe fn snp_cpuid_postprocess(_f: unsafe fn(*mut c_void,*mut cpuid_leaf), _ctx:*mut c_void, _leaf:*mut cpuid_leaf)->i32 { 0 }
pub unsafe fn snp_cpuid(f: unsafe fn(*mut c_void,*mut cpuid_leaf), ctx:*mut c_void, leaf:*mut cpuid_leaf)->i32 { if (*snp_cpuid_get_table()).count==0 { return -EOPNOTSUPP; } if !snp_cpuid_get_validated_func(leaf) { (*leaf).eax=0;(*leaf).ebx=0;(*leaf).ecx=0;(*leaf).edx=0; if !((*leaf).fn_<=cpuid_std_range_max || ((*leaf).fn_>=0x40000000 && (*leaf).fn_<=cpuid_hyp_range_max) || ((*leaf).fn_>=0x80000000 && (*leaf).fn_<=cpuid_ext_range_max)) { return 0; } } snp_cpuid_postprocess(f,ctx,leaf) }

pub unsafe fn do_vc_no_ghcb(regs:*mut pt_regs, exit_code:u32) { let fn_=lower_bits((*regs).ax); let subfn=lower_bits((*regs).cx); let opcode=*( (*regs).ip as *const u16); let mut leaf=cpuid_leaf{fn_,subfn,eax:0,ebx:0,ecx:0,edx:0}; if exit_code!=0 || opcode!=0xa20f { sev_es_terminate(0,0); } let ret=snp_cpuid(snp_cpuid_hv_msr,core::ptr::null_mut(),&mut leaf); if ret==-EOPNOTSUPP { if __sev_cpuid_hv_msr(&mut leaf)!=0 { sev_es_terminate(0,0); } } else if ret!=0 { sev_es_terminate(0,0); } (*regs).ax=leaf.eax as usize;(*regs).bx=leaf.ebx as usize;(*regs).cx=leaf.ecx as usize;(*regs).dx=leaf.edx as usize; if (fn_==0x80000000 && (*regs).ax<0x8000001f) || (fn_==0x8000001f && (*regs).ax & 2 == 0) { sev_es_terminate(0,0); } (*regs).ip+=2; }

pub unsafe fn find_cc_blob_setup_data(bp:*const boot_params)->*mut cc_blob_sev_info { let mut hdr=(*(bp)).hdr.setup_data as *mut setup_data; while !hdr.is_null() { if (*hdr).type_==0x12 { return (*(hdr as *mut cc_setup_data)).cc_blob_address as u64 as *mut cc_blob_sev_info; } hdr=(*hdr).next as *mut setup_data; } core::ptr::null_mut() }

pub unsafe fn setup_cpuid_table(cc_info:*const cc_blob_sev_info) { if cc_info.is_null() || (*cc_info).cpuid_phys==0 || (*cc_info).cpuid_len < PAGE_SIZE as u32 { sev_es_terminate(0,0); } let fw=(*cc_info).cpuid_phys as *const snp_cpuid_table; if (*fw).count==0 || (*fw).count>64 { sev_es_terminate(0,0); } memcpy(&mut cpuid_table_copy as *mut _ as *mut c_void,fw as *const c_void,core::mem::size_of::<snp_cpuid_table>()); for i in 0..cpuid_table_copy.count as usize { let f=&cpuid_table_copy.fn_[i]; if f.eax_in==0 { cpuid_std_range_max=f.eax; } else if f.eax_in==0x40000000 { cpuid_hyp_range_max=f.eax; } else if f.eax_in==0x80000000 { cpuid_ext_range_max=f.eax; } } }

unsafe fn svsm_call_msr_protocol(call:*mut svsm_call)->i32 { let mut ret; loop { ret=svsm_perform_msr_protocol(call); if ret!=-EAGAIN { return ret; } } }
unsafe fn svsm_pval_4k_page(paddr:usize,validate:bool,caa:*mut svsm_ca,caa_pa:u64) { let flags=native_local_irq_save(); let mut call:svsm_call=core::mem::zeroed(); call.caa=caa; let pc=(*caa).svsm_buffer.as_mut_ptr() as *mut svsm_pvalidate_call; (*pc).num_entries=1;(*pc).cur_index=0;(*pc).entry[0].page_size=0;(*pc).entry[0].action=validate;(*pc).entry[0].pfn=(paddr>>PAGE_SHIFT) as u64; call.rax=1;call.rcx=caa_pa+core::mem::offset_of!(svsm_ca,svsm_buffer) as u64; if svsm_call_msr_protocol(&mut call)!=0 { sev_es_terminate(0,0); } native_local_irq_restore(flags); }
unsafe fn pvalidate_4k_page(vaddr:usize,paddr:usize,validate:bool,caa:*mut svsm_ca,caa_pa:u64) { if snp_vmpl!=0 { svsm_pval_4k_page(paddr,validate,caa,caa_pa); } else if pvalidate(vaddr,0,validate)!=0 { sev_es_terminate(0,0); } if validate && sev_snp_needs_sfw { sev_evict_cache(vaddr as *mut c_void,1); } }
pub unsafe fn __page_state_change(vaddr:usize,paddr:usize,desc:*const psc_desc) { if (*desc).op==1 { pvalidate_4k_page(vaddr,paddr,false,(*desc).ca,(*desc).caa_pa); } let msr=sev_es_rd_ghcb_msr(); sev_es_wr_ghcb_msr(((paddr>>PAGE_SHIFT) as u64)<<12 | (*desc).op as u64); vmgexit(); let val=sev_es_rd_ghcb_msr(); if ghcb_resp_code(val)!=GHCB_MSR_PSC_RESP || val>>32!=0 { sev_es_terminate(0,0); } sev_es_wr_ghcb_msr(msr); if (*desc).op==2 { pvalidate_4k_page(vaddr,paddr,true,(*desc).ca,(*desc).caa_pa); } }
pub unsafe fn svsm_setup_ca(cc_info:*const cc_blob_sev_info,page:*mut c_void)->bool { if rmpadjust(page as usize,0,1)==0 { return false; } if cc_info.is_null() || (*cc_info).secrets_phys==0 || (*cc_info).secrets_len as usize!=PAGE_SIZE { sev_es_terminate(0,0); } let s=&*((*cc_info).secrets_phys as *const snp_secrets_page); if s.svsm_size==0 || s.svsm_guest_vmpl==0 { sev_es_terminate(0,0); } snp_vmpl=s.svsm_guest_vmpl; boot_svsm_caa_pa=s.svsm_caa; if s.svsm_caa & (PAGE_SIZE as u64-1)!=0 { sev_es_terminate(0,0); } let t=&mut cpuid_table_copy; for i in 0..t.count as usize { if t.fn_[i].eax_in==0x8000001f { t.fn_[i].eax|=1<<28; } } true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
