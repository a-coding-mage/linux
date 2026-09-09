// SPDX-License-Identifier: GPL-2.0-only
/* Direct source-level Rust translation of arm64/mm/fault.c. */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, unused_mut)]
use core::ffi::c_void;
#[repr(C)] pub struct pt_regs { pub pstate: usize, pub pc: usize }
#[repr(C)] pub struct mm_struct { pub pgd: *mut c_void }
#[repr(C)] pub struct vm_area_struct { pub vm_flags: usize }
#[repr(C)] pub struct page { _private: [u8;0] }
#[repr(C)] pub struct folio { _private: [u8;0] }
#[repr(C)] pub struct pte_t { pub val: usize }
#[repr(C)] pub struct fault_info {
    pub fn_: Option<unsafe extern "C" fn(usize,usize,*mut pt_regs)->i32>,
    pub sig: i32, pub code: i32, pub name: *const i8,
}
extern "C" {
    static fault_info_table: [fault_info;64];
    fn untagged_addr(a:usize)->usize;
    fn user_mode(r:*mut pt_regs)->bool;
    fn arm64_notify_die(n:*const i8,r:*mut pt_regs,s:i32,c:i32,a:usize,e:usize);
    fn die_kernel_fault(n:*const i8,a:usize,e:usize,r:*mut pt_regs);
    fn do_page_fault(far:usize,esr:usize,r:*mut pt_regs)->i32;
    fn do_bad_area(far:usize,esr:usize,r:*mut pt_regs);
    fn vma_alloc_folio(f:usize,o:usize,v:*mut vm_area_struct,a:usize)->*mut folio;
    fn system_supports_mte()->bool;
    fn try_page_mte_tagging(p:*mut page)->bool;
    fn page_address(p:*mut page)->*mut c_void;
    fn mte_zero_clear_page_tags(p:*mut c_void);
    fn mte_clear_page_tags(p:*mut c_void);
    fn set_page_mte_tagged(p:*mut page);
}
#[inline] unsafe fn esr_to_fault_info(esr:usize)->*const fault_info {
    fault_info_table.as_ptr().add(esr & ESR_ELx_FSC)
}
unsafe fn is_el1_instruction_abort(esr:usize)->bool { ESR_ELx_EC(esr)==ESR_ELx_EC_IABT_CUR }
unsafe fn is_el1_data_abort(esr:usize)->bool { ESR_ELx_EC(esr)==ESR_ELx_EC_DABT_CUR }
unsafe fn is_el0_instruction_abort(esr:usize)->bool { ESR_ELx_EC(esr)==ESR_ELx_EC_IABT_LOW }
unsafe fn is_write_abort(esr:usize)->bool { esr&ESR_ELx_WNR != 0 && esr&ESR_ELx_CM == 0 }
unsafe fn is_gcs_fault(esr:usize)->bool { esr_is_data_abort(esr) && ESR_ELx_ISS2(esr)&ESR_ELx_GCS != 0 }
unsafe fn is_invalid_gcs_access(vma:*mut vm_area_struct,esr:u64)->bool {
    if !system_supports_gcs(){return false;}
    if is_gcs_fault(esr as usize) { (*vma).vm_flags&VM_SHADOW_STACK==0 }
    else { (*vma).vm_flags&VM_SHADOW_STACK!=0 && esr_is_data_abort(esr as usize) && is_write_abort(esr as usize) }
}
unsafe fn do_translation_fault(far:usize,esr:usize,regs:*mut pt_regs)->i32 {
    let addr=untagged_addr(far); if is_ttbr0_addr(addr){return do_page_fault(far,esr,regs);}
    do_bad_area(far,esr,regs); 0
}
unsafe fn do_bad(_far:usize,_esr:usize,_regs:*mut pt_regs)->i32 { 1 }
unsafe fn do_alignment_fault(far:usize,esr:usize,regs:*mut pt_regs)->i32 { do_bad_area(far,esr,regs); 0 }
unsafe fn do_sea(far:usize,esr:usize,regs:*mut pt_regs)->i32 {
    let inf=&*esr_to_fault_info(esr); let addr=if esr&ESR_ELx_FnV!=0{0}else{untagged_addr(far)};
    arm64_notify_die(inf.name,regs,inf.sig,inf.code,addr,esr); 0
}
unsafe fn do_tag_check_fault(mut far:usize,esr:usize,regs:*mut pt_regs)->i32 {
    if !cpus_have_cap(ARM64_MTE_FAR){far=(__untagged_addr(far)&!MTE_TAG_MASK)|(far&MTE_TAG_MASK);}
    do_bad_area(far,esr,regs); 0
}
#[no_mangle] pub unsafe extern "C" fn do_mem_abort(far:usize,esr:usize,regs:*mut pt_regs) {
    let inf=&*esr_to_fault_info(esr); let addr=untagged_addr(far);
    if let Some(f)=inf.fn_ { if f(far,esr,regs)==0{return;} }
    if !user_mode(regs){die_kernel_fault(inf.name,addr,esr,regs);} else {arm64_notify_die(inf.name,regs,inf.sig,inf.code,addr,esr);}
}
#[no_mangle] pub unsafe extern "C" fn do_sp_pc_abort(addr:usize,esr:usize,regs:*mut pt_regs) {
    arm64_notify_die(b"SP/PC alignment exception\0".as_ptr() as _,regs,SIGBUS,BUS_ADRALN,addr,esr);
}
#[no_mangle] pub unsafe extern "C" fn vma_alloc_zeroed_movable_folio(vma:*mut vm_area_struct,vaddr:usize)->*mut folio {
    let mut flags=GFP_HIGHUSER_MOVABLE|__GFP_ZERO;
    if (*vma).vm_flags&VM_MTE!=0 {flags|=__GFP_ZEROTAGS;} vma_alloc_folio(flags,0,vma,vaddr)
}
#[no_mangle] pub unsafe extern "C" fn tag_clear_highpages(mut page:*mut page,numpages:i32,clear_pages:bool)->bool {
    if !system_supports_mte(){return clear_pages;}
    for _ in 0..numpages { WARN_ON_ONCE(!try_page_mte_tagging(page)); if clear_pages{mte_zero_clear_page_tags(page_address(page));}else{mte_clear_page_tags(page_address(page));} set_page_mte_tagged(page); page=page.add(1); } false
}
// The fault_info table and the architecture constants/functions below are
// supplied by the surrounding kernel translation unit.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
