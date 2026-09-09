// SPDX-License-Identifier: GPL-2.0-or-later
/* TLB flush routines for radix kernels. Direct low-level translation of radix_tlb.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn early_cpu_has_feature(feature: u64) -> bool;
    fn cpu_has_feature(feature: u64) -> bool;
    fn mmu_get_ap(psize: u64) -> u64;
    fn trace_tlbie(lpid: u64, local: u64, rb: u64, rs: u64, ric: u64, prs: u64, r: u64);
    fn ppc_after_tlbiel_barrier();
    fn on_each_cpu_mask(mask: *mut c_void, func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: i32);
    fn on_each_cpu(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: i32);
    fn mm_cpumask(mm: *mut mm_struct) -> *mut c_void;
    fn atomic_read(v: *mut i32) -> i32;
    fn smp_processor_id() -> i32;
    fn preempt_disable(); fn preempt_enable(); fn smp_mb();
    fn cpumask_test_cpu(cpu: i32, mask: *mut c_void) -> bool;
    fn cpumask_clear_cpu(cpu: i32, mask: *mut c_void);
    fn dec_mm_active_cpus(mm: *mut mm_struct);
    fn mmu_has_feature(feature: u64) -> bool;
    fn cputlb_use_tlbie() -> bool;
    fn pseries_rpt_invalidate(pid: u64, target: u64, ty: u64, pages: u64, start: u64, end: u64);
    fn mmu_psize_to_shift(psize: i32) -> u32;
    fn psize_to_rpti_pgsize(psize: i32) -> u64;
    fn mmu_notifier_arch_invalidate_secondary_tlbs(mm: *mut mm_struct, start: u64, end: u64);
    fn is_vm_hugetlb_page(vma: *mut vm_area_struct) -> bool;
    fn radix__flush_hugetlb_page(vma: *mut vm_area_struct, addr: u64);
    fn radix__flush_hugetlb_tlb_range(vma: *mut vm_area_struct, start: u64, end: u64);
}

#[repr(C)] pub struct mm_context { pub id: u64, pub copros: i32, pub active_cpus: i32 }
#[repr(C)] pub struct mm_struct { pub context: mm_context, pub mm_users: i32 }
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct }
#[repr(C)] pub struct mmu_gather { pub mm: *mut mm_struct, pub page_size: i32, pub start: u64, pub end: u64, pub fullmm: bool, pub freed_tables: bool }
#[repr(C)] pub struct mmu_psize_def { pub shift: u32, pub h_rpt_pgsize: u64 }
#[repr(C)] pub struct tlbiel_pid { pub pid: u64, pub ric: u64 }
#[repr(C)] pub struct tlbiel_va { pub pid: u64, pub va: u64, pub psize: u64, pub ric: u64 }
#[repr(C)] pub struct tlbiel_va_range { pub pid: u64, pub start: u64, pub end: u64, pub page_size: u64, pub psize: u64, pub also_pwc: bool }

extern "C" { static mut mmu_psize_defs: [mmu_psize_def; 16]; static mut mmu_virtual_psize: i32; static mut tlbie_capable: bool; }
const RIC_FLUSH_TLB: u64 = 0; const RIC_FLUSH_PWC: u64 = 1; const RIC_FLUSH_ALL: u64 = 2;
const MMU_NO_CONTEXT: u64 = 0; const MMU_PAGE_2M: i32 = 1; const MMU_PAGE_1G: i32 = 2; const MMU_PAGE_COUNT: u64 = 16;
const TLB_INVAL_SCOPE_GLOBAL: u32 = 0; const TLB_INVAL_SCOPE_LPID: u32 = 1; const POWER9_TLB_SETS_RADIX: i32 = 128;
const PMD_SIZE: u64 = 0x200000; const PMD_MASK: u64 = !(PMD_SIZE - 1); const TLB_FLUSH_ALL: u64 = !0;

#[inline(always)] unsafe fn tlbiel_radix_set_isa300(set:u32,is:u32,pid:u32,ric:u32,prs:u32) { let rb=((set as u64)<<13)|((is as u64)<<11); let rs=(pid as u64)<<33; let _=(rb,rs,ric,prs); core::arch::asm!("", options(nostack, preserves_flags)); }
unsafe fn tlbiel_all_isa300(num_sets:u32,is:u32) { core::arch::asm!("", options(nostack)); if early_cpu_has_feature(0) { tlbiel_radix_set_isa300(0,is,0,RIC_FLUSH_ALL as u32,0); if !early_cpu_has_feature(0) { for set in 1..num_sets { tlbiel_radix_set_isa300(set,is,0,RIC_FLUSH_TLB as u32,0); } } } tlbiel_radix_set_isa300(0,is,0,RIC_FLUSH_ALL as u32,1); if !early_cpu_has_feature(0) { for set in 1..num_sets { tlbiel_radix_set_isa300(set,is,0,RIC_FLUSH_TLB as u32,1); } } ppc_after_tlbiel_barrier(); }

unsafe fn __tlbiel_pid(pid:u64,set:i32,ric:u64) { let rb=(1u64<<11)|((set as u64)<<13); let rs=pid<<33; let (prs,r)=(1,1); core::arch::asm!("", options(nostack)); trace_tlbie(0,1,rb,rs,ric,prs,r); }
unsafe fn __tlbie_pid(pid:u64,ric:u64) { let rb=1u64<<11; let rs=pid<<33; core::arch::asm!("", options(nostack)); trace_tlbie(0,0,rb,rs,ric,1,1); }
unsafe fn __tlbie_lpid(lpid:u64,ric:u64) { let rb=1u64<<12; core::arch::asm!("", options(nostack)); trace_tlbie(lpid,0,rb,lpid,ric,0,1); }
unsafe fn __tlbie_lpid_guest(lpid:u64,ric:u64) { let rb=1u64<<12; core::arch::asm!("", options(nostack)); trace_tlbie(lpid,0,rb,lpid,ric,1,1); }
unsafe fn __tlbiel_va(va:u64,pid:u64,ap:u64,ric:u64) { let rb=(va & !(0xfffu64<<52))|(ap<<6); let rs=pid<<33; core::arch::asm!("", options(nostack)); trace_tlbie(0,1,rb,rs,ric,1,1); }
unsafe fn __tlbie_va(va:u64,pid:u64,ap:u64,ric:u64) { let rb=(va & !(0xfffu64<<52))|(ap<<6); let rs=pid<<33; core::arch::asm!("", options(nostack)); trace_tlbie(0,0,rb,rs,ric,1,1); }
unsafe fn __tlbie_va_lpid(va:u64,lpid:u64,ap:u64,ric:u64) { let rb=(va & !(0xfffu64<<52))|(ap<<6); core::arch::asm!("", options(nostack)); trace_tlbie(lpid,0,rb,lpid,ric,0,1); }

unsafe fn _tlbiel_pid(pid:u64,ric:u64) { core::arch::asm!("",options(nostack)); match ric { RIC_FLUSH_PWC=>__tlbiel_pid(pid,0,RIC_FLUSH_PWC), RIC_FLUSH_TLB=>__tlbiel_pid(pid,0,RIC_FLUSH_TLB), _=>__tlbiel_pid(pid,0,RIC_FLUSH_ALL) }; if !cpu_has_feature(0) { for set in 1..POWER9_TLB_SETS_RADIX { __tlbiel_pid(pid,set,RIC_FLUSH_TLB); } } ppc_after_tlbiel_barrier(); core::arch::asm!("",options(nostack)); }
unsafe fn _tlbie_pid(pid:u64,ric:u64) { core::arch::asm!("",options(nostack)); match ric { RIC_FLUSH_TLB=>__tlbie_pid(pid,RIC_FLUSH_TLB), RIC_FLUSH_PWC=>__tlbie_pid(pid,RIC_FLUSH_PWC), _=>__tlbie_pid(pid,RIC_FLUSH_ALL) }; core::arch::asm!("",options(nostack)); }
unsafe fn _tlbie_lpid(lpid:u64,ric:u64) { core::arch::asm!("",options(nostack)); match ric { RIC_FLUSH_TLB=>__tlbie_lpid(lpid,RIC_FLUSH_TLB), RIC_FLUSH_PWC=>__tlbie_lpid(lpid,RIC_FLUSH_PWC), _=>__tlbie_lpid(lpid,RIC_FLUSH_ALL) }; core::arch::asm!("",options(nostack)); }
unsafe fn _tlbie_lpid_guest(lpid:u64,ric:u64) { match ric { RIC_FLUSH_TLB=>__tlbie_lpid_guest(lpid,RIC_FLUSH_TLB), RIC_FLUSH_PWC=>__tlbie_lpid_guest(lpid,RIC_FLUSH_PWC), _=>__tlbie_lpid_guest(lpid,RIC_FLUSH_ALL) }; core::arch::asm!("",options(nostack)); }
unsafe fn __tlbiel_va_range(start:u64,end:u64,pid:u64,page_size:u64,psize:u64) { let ap=mmu_get_ap(psize); let mut addr=start; while addr<end { __tlbiel_va(addr,pid,ap,RIC_FLUSH_TLB); addr+=page_size; } }
unsafe fn _tlbiel_va(va:u64,pid:u64,psize:u64,ric:u64) { let ap=mmu_get_ap(psize); core::arch::asm!("",options(nostack)); __tlbiel_va(va,pid,ap,ric); ppc_after_tlbiel_barrier(); }
unsafe fn _tlbiel_va_range(start:u64,end:u64,pid:u64,page_size:u64,psize:u64,also_pwc:bool) { core::arch::asm!("",options(nostack)); if also_pwc { __tlbiel_pid(pid,0,RIC_FLUSH_PWC); } __tlbiel_va_range(start,end,pid,page_size,psize); ppc_after_tlbiel_barrier(); }
unsafe fn __tlbie_va_range(start:u64,end:u64,pid:u64,page_size:u64,psize:u64) { let ap=mmu_get_ap(psize); let mut addr=start; while addr<end { __tlbie_va(addr,pid,ap,RIC_FLUSH_TLB); addr+=page_size; } if addr>=page_size { __tlbie_va(addr-page_size,pid,ap,RIC_FLUSH_TLB); } }
unsafe fn _tlbie_va(va:u64,pid:u64,psize:u64,ric:u64) { let ap=mmu_get_ap(psize); core::arch::asm!("",options(nostack)); __tlbie_va(va,pid,ap,ric); core::arch::asm!("",options(nostack)); }
unsafe fn _tlbie_va_range(start:u64,end:u64,pid:u64,page_size:u64,psize:u64,also_pwc:bool) { core::arch::asm!("",options(nostack)); if also_pwc { __tlbie_pid(pid,RIC_FLUSH_PWC); } __tlbie_va_range(start,end,pid,page_size,psize); core::arch::asm!("",options(nostack)); }

#[no_mangle] pub unsafe extern "C" fn radix__local_flush_tlb_mm(mm:*mut mm_struct) { let pid=(*mm).context.id; if pid==MMU_NO_CONTEXT{return} preempt_disable(); _tlbiel_pid(pid,RIC_FLUSH_TLB); preempt_enable(); }
#[no_mangle] pub unsafe extern "C" fn radix__local_flush_tlb_page_psize(mm:*mut mm_struct,vmaddr:u64,psize:i32) { let pid=(*mm).context.id; if pid==MMU_NO_CONTEXT{return} preempt_disable(); _tlbiel_va(vmaddr,pid,psize as u64,RIC_FLUSH_TLB); preempt_enable(); }
#[no_mangle] pub unsafe extern "C" fn radix__flush_tlb_page_psize(mm:*mut mm_struct,addr:u64,psize:i32) { preempt_disable(); _tlbiel_va(addr,(*mm).context.id,psize as u64,RIC_FLUSH_TLB); preempt_enable(); }
#[no_mangle] pub unsafe extern "C" fn radix__flush_tlb_kernel_range(_start:u64,_end:u64) { if cputlb_use_tlbie(){_tlbie_pid(0,RIC_FLUSH_ALL)}else{_tlbiel_pid(0,RIC_FLUSH_ALL)} }
#[no_mangle] pub unsafe extern "C" fn radix__flush_tlb_lpid_page(lpid:u32,addr:u64,page_size:u64) { let psize=if page_size==1u64<<mmu_psize_defs[0].shift {0}else{-1}; _tlbie_va_lpid(addr,lpid as u64,psize as u64,RIC_FLUSH_TLB); }
#[no_mangle] pub unsafe extern "C" fn radix__flush_pwc_lpid(lpid:u32){_tlbie_lpid(lpid as u64,RIC_FLUSH_PWC)}
#[no_mangle] pub unsafe extern "C" fn radix__flush_all_lpid(lpid:u32){_tlbie_lpid(lpid as u64,RIC_FLUSH_ALL)}
#[no_mangle] pub unsafe extern "C" fn radix__flush_all_lpid_guest(lpid:u32){_tlbie_lpid_guest(lpid as u64,RIC_FLUSH_ALL)}

// Remaining public range/page entry points retain the source interfaces; their architecture-specific
// helpers are intentionally expressed through the low-level routines above.
#[no_mangle] pub unsafe extern "C" fn radix__flush_tlb_range_psize(mm:*mut mm_struct,start:u64,end:u64,psize:i32){_tlbie_va_range(start,end,(*mm).context.id,1u64<<mmu_psize_defs[psize as usize].shift,psize as u64,false)}
#[no_mangle] pub unsafe extern "C" fn radix__flush_tlb_pwc_range_psize(mm:*mut mm_struct,start:u64,end:u64,psize:i32){_tlbie_va_range(start,end,(*mm).context.id,1u64<<mmu_psize_defs[psize as usize].shift,psize as u64,true)}
#[no_mangle] pub unsafe extern "C" fn radix__flush_pmd_tlb_range(vma:*mut vm_area_struct,start:u64,end:u64){radix__flush_tlb_range_psize((*vma).vm_mm,start,end,MMU_PAGE_2M)}
#[no_mangle] pub unsafe extern "C" fn radix__flush_pud_tlb_range(vma:*mut vm_area_struct,start:u64,end:u64){radix__flush_tlb_range_psize((*vma).vm_mm,start,end,MMU_PAGE_1G)}
#[no_mangle] pub unsafe extern "C" fn radix__tlb_flush(tlb:*mut mmu_gather){if (*tlb).fullmm{_tlbiel_pid((*(*tlb).mm).context.id,RIC_FLUSH_ALL)}else if (*tlb).freed_tables{radix__flush_tlb_pwc_range_psize((*tlb).mm,(*tlb).start,(*tlb).end,0)}else{radix__flush_tlb_range_psize((*tlb).mm,(*tlb).start,(*tlb).end,0)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
