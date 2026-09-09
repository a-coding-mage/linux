// SPDX-License-Identifier: GPL-2.0-only
/* AMD SVM-SEV Host Support. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

#[repr(C, packed)]
pub struct rmpentry { pub gpa: u64, pub assigned: u8, pub pagesize: u8, pub immutable: u8, pub rsvd4: u8, pub asid: u32 }
#[repr(C, packed)]
pub struct rmpentry_raw { pub lo: u64, pub hi: u64 }
#[repr(C)]
pub struct rmp_segment_desc { pub rmp_entry: *mut rmpentry_raw, pub max_index: u64, pub size: u64 }

const RMPTABLE_CPU_BOOKKEEPING_SZ: u64 = 0x4000;
const RMPTABLE_NON_SEGMENTED_SHIFT: u32 = 52;
const RST_SIZE: u64 = 0x1000;
const PFN_PMD_MASK: u64 = !((1u64 << (21 - 12)) - 1);
const RMP_ADDR_MASK: u64 = (((1u64 << 52) - 1) & !((1u64 << 13) - 1));

extern "C" {
    static mut rmp_segment_table: *mut *mut rmp_segment_desc;
    static mut rst_max_index: u32;
    static mut rmp_segment_shift: u32;
    static mut rmp_segment_size: u64;
    static mut rmp_segment_mask: u64;
    static mut rmp_cfg: u64;
    static mut rmp_bookkeeping: *mut c_void;
    static mut probed_rmp_base: u64;
    static mut probed_rmp_size: u64;
    static mut snp_nr_leaked_pages: c_ulong;
}
type c_ulong = usize;

extern "C" {
    fn cc_platform_has(x: u32) -> bool; fn rdmsrq(msr: u32, val: *mut u64); fn wrmsrq(msr: u32, val: u64);
    fn msr_set_bit(msr: u32, bit: u32); fn msr_clear_bit(msr: u32, bit: u32);
    fn e820__mapped_any(a: u64, b: u64, t: u32) -> bool; fn e820__range_update(a:u64,b:u64,f:u32,t:u32);
    fn e820__range_update_table(tab:*mut c_void,a:u64,b:u64,f:u32,t:u32); fn memblock_is_region_reserved(a:u64,b:u64)->bool; fn memblock_reserve(a:u64,b:u64);
    fn early_memremap(a:u64,b:u64)->*mut u64; fn early_memunmap(p:*mut u64,b:u64); fn memremap(a:u64,b:u64,f:u32)->*mut c_void; fn memunmap(p:*mut c_void);
    fn kzalloc(s:usize)->*mut rmp_segment_desc; fn kfree(p:*mut rmp_segment_desc); fn alloc_page(f:u32)->*mut c_void; fn page_address(p:*mut c_void)->*mut *mut rmp_segment_desc; fn free_page(p:usize);
    fn cpus_read_lock(); fn cpus_read_unlock(); fn wbinvd_on_all_cpus(); fn on_each_cpu(f:unsafe extern "C" fn(*mut c_void),a:*mut c_void,w:i32);
    fn cpumask_equal(a:*const c_void,b:*const c_void)->bool; static cpu_online_mask:c_void; static cpu_present_mask:c_void;
    fn cpuid_eax(x:u32)->u32; fn cpuid_ebx(x:u32)->u32; fn cpu_feature_enabled(x:u32)->bool; fn array_index_nospec(x:u64,y:u64)->u64;
    fn pfn_valid(p:u64)->bool; fn pfn_to_kaddr(p:u64)->usize; fn lookup_address(v:usize,l:*mut u32)->*mut c_void; fn set_memory_4k(v:usize,n:i32)->i32;
    fn page_level_size(l:i32)->usize; fn dump_stack(); fn wbinvd();
}

const CC_ATTR_HOST_SEV_SNP:u32=0; const MSR_AMD64_SYSCFG:u32=0; const MSR_AMD64_SYSCFG_MFDM_BIT:u32=0; const MSR_AMD64_SYSCFG_SNP_EN:u64=1; const MSR_AMD64_SYSCFG_SNP_VMPL_EN:u64=2; const MSR_VM_HSAVE_PA:u32=0; const MSR_AMD64_RMP_BASE:u32=0; const MSR_AMD64_RMP_END:u32=0; const MSR_AMD64_RMP_CFG:u32=0; const MSR_AMD64_SEG_RMP_ENABLED:u64=1; const X86_FEATURE_SEGMENTED_RMP:u32=0; const X86_FEATURE_RMPREAD:u32=0; const PAGE_SHIFT:u32=12; const PG_LEVEL_2M:i32=2; const PG_LEVEL_4K:i32=1; const RMPUPDATE_FAIL_OVERLAP:i32=1;

#[inline] unsafe fn rst_entry_mapped_size(x:u64)->u64 { x & ((1<<20)-1) }
#[inline] unsafe fn rst_entry_segment_base(x:u64)->u64 { x & (((1<<52)-1)&!((1<<20)-1)) }
#[inline] unsafe fn rst_entry_index(x:u64)->u64 { x >> rmp_segment_shift }
#[inline] unsafe fn rmp_entry_index(x:u64)->u64 { ((x & rmp_segment_mask)>>PAGE_SHIFT) }

unsafe extern "C" fn mfd_reconfigure(arg:*mut c_void) { if !cc_platform_has(CC_ATTR_HOST_SEV_SNP){return;} if !arg.is_null(){msr_set_bit(MSR_AMD64_SYSCFG,MSR_AMD64_SYSCFG_MFDM_BIT)}else{msr_clear_bit(MSR_AMD64_SYSCFG,MSR_AMD64_SYSCFG_MFDM_BIT)} }
unsafe extern "C" fn snp_enable(_: *mut c_void) { if !cc_platform_has(CC_ATTR_HOST_SEV_SNP){return;} let mut v=0; rdmsrq(MSR_AMD64_SYSCFG,&mut v); wrmsrq(MSR_AMD64_SYSCFG,v|MSR_AMD64_SYSCFG_SNP_EN|MSR_AMD64_SYSCFG_SNP_VMPL_EN); }
unsafe extern "C" fn clear_hsave_pa(_: *mut c_void){wrmsrq(MSR_VM_HSAVE_PA,0)}

unsafe fn snp_fixup(pa:u64){ if pa%(1<<21)==0{return;} let p=pa & !((1<<21)-1); if e820__mapped_any(p,p+(1<<21),1){e820__range_update(p,1<<21,1,2); if !memblock_is_region_reserved(p,1<<21){memblock_reserve(p,1<<21)}} }
pub unsafe extern "C" fn snp_fixup_e820_tables(){snp_fixup(probed_rmp_base); if rmp_cfg&MSR_AMD64_SEG_RMP_ENABLED!=0{snp_fixup(probed_rmp_base+RMPTABLE_CPU_BOOKKEEPING_SZ+RST_SIZE)}else{snp_fixup(probed_rmp_base+probed_rmp_size)}}

unsafe fn clear_rmp(){if !cc_platform_has(CC_ATTR_HOST_SEV_SNP){return;} let mut v=0;rdmsrq(MSR_AMD64_SYSCFG,&mut v);if v&MSR_AMD64_SYSCFG_SNP_EN!=0{return;} core::ptr::write_bytes(rmp_bookkeeping,0,RMPTABLE_CPU_BOOKKEEPING_SZ as usize); for i in 0..rst_max_index {let d=*rmp_segment_table.add(i as usize);if !d.is_null(){core::ptr::write_bytes((*d).rmp_entry as *mut u8,0,(*d).size as usize)}}}

unsafe fn set_rmp_segment_info(s:u32){rmp_segment_shift=s;rmp_segment_size=1u64<<s;rmp_segment_mask=rmp_segment_size-1}
pub unsafe extern "C" fn snp_probe_rmptable_info()->bool{let mut b=0;let mut e=0;rdmsrq(MSR_AMD64_RMP_BASE,&mut b);rdmsrq(MSR_AMD64_RMP_END,&mut e);if b&RMP_ADDR_MASK==0{return false;}if rmp_cfg&MSR_AMD64_SEG_RMP_ENABLED!=0{rst_max_index=512;set_rmp_segment_info(((rmp_cfg>>0)&63) as u32);probed_rmp_base=b;probed_rmp_size=0;true}else{if e&RMP_ADDR_MASK==0||b>e{return false;}rst_max_index=1;set_rmp_segment_info(RMPTABLE_NON_SEGMENTED_SHIFT);probed_rmp_base=b;probed_rmp_size=e-b+1;true}}

unsafe fn get_raw_rmpentry(pfn:u64)->*mut rmpentry_raw{if rmp_segment_table.is_null(){return (-19isize) as *mut rmpentry_raw}let p=pfn<<PAGE_SHIFT;let ri=rst_entry_index(p);if ri>=rst_max_index{return (-14isize) as *mut rmpentry_raw}let d=*rmp_segment_table.add(ri as usize);if d.is_null(){return (-14isize) as *mut rmpentry_raw}let si=rmp_entry_index(p);if si>=(*d).max_index{return (-14isize) as *mut rmpentry_raw}(*d).rmp_entry.add(si as usize)}

unsafe fn get_rmpentry(pfn:u64,e:*mut rmpentry)->i32{let r=get_raw_rmpentry(pfn);if (r as isize)<0{return r as isize as i32;}core::ptr::write_bytes(e,0,1);(*e).gpa=(*r).lo<<PAGE_SHIFT;0}
unsafe fn lookup(pfn:u64,e:*mut rmpentry,level:*mut i32)->i32{if !cc_platform_has(CC_ATTR_HOST_SEV_SNP){return -19}let x=get_rmpentry(pfn,e);if x!=0{return x}let mut large=rmpentry{gpa:0,assigned:0,pagesize:0,immutable:0,rsvd4:0,asid:0};let x=get_rmpentry(pfn&PFN_PMD_MASK,&mut large);if x==0{*level=if large.pagesize!=0{PG_LEVEL_2M}else{PG_LEVEL_4K}}x}
pub unsafe extern "C" fn snp_lookup_rmpentry(pfn:u64,assigned:*mut bool,level:*mut i32)->i32{let mut e=rmpentry{gpa:0,assigned:0,pagesize:0,immutable:0,rsvd4:0,asid:0};let r=lookup(pfn,&mut e,level);if r==0{*assigned=e.assigned!=0}r}

pub unsafe extern "C" fn psmash(pfn:u64)->i32{if !cc_platform_has(CC_ATTR_HOST_SEV_SNP){return -19}if !pfn_valid(pfn){return -22}0}
unsafe fn adjust_direct_map(pfn:u64,level:i32)->i32{if !pfn_valid(pfn){return -22}if level==PG_LEVEL_2M{return 0}let v=pfn_to_kaddr(pfn);let mut l=0;let p=lookup_address(v,&mut l);if p.is_null()||l==PG_LEVEL_4K{return 0}set_memory_4k(v,(page_level_size(level)/4096) as i32)}
#[repr(C)] pub struct rmp_state{pub assigned:u8,pub pagesize:u8,pub immutable:bool,pub gpa:u64,pub asid:u32}
unsafe fn rmpupdate(pfn:u64,s:*mut rmp_state)->i32{if !cc_platform_has(CC_ATTR_HOST_SEV_SNP){return -19}let l=if (*s).pagesize!=0{PG_LEVEL_2M}else{PG_LEVEL_4K};if adjust_direct_map(pfn,l)!=0{return -14}0}
pub unsafe extern "C" fn rmp_make_private(pfn:u64,gpa:u64,level:i32,asid:u32,immutable:bool)->i32{let mut s=rmp_state{assigned:1,pagesize:if level==PG_LEVEL_2M{1}else{0},immutable,gpa,asid};rmpupdate(pfn,&mut s)}
pub unsafe extern "C" fn rmp_make_shared(pfn:u64,level:i32)->i32{let mut s=rmp_state{assigned:0,pagesize:if level==PG_LEVEL_2M{1}else{0},immutable:false,gpa:0,asid:0};rmpupdate(pfn,&mut s)}
pub unsafe extern "C" fn snp_dump_hva_rmpentry(_:usize){}
pub unsafe extern "C" fn __snp_leak_pages(_:u64,_:u32,_:bool){}
pub unsafe extern "C" fn kdump_sev_callback(){if cc_platform_has(CC_ATTR_HOST_SEV_SNP){wbinvd()}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
