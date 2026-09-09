// SPDX-License-Identifier: GPL-2.0
/* Intel Trusted Domain Extensions (TDX) support. */
// Kernel headers and generated metadata are supplied by the surrounding tree.

#[repr(C)]
struct TdxModuleState { initialized: bool, sysinit_done: bool, sysinit_ret: i32 }

static mut TDX_MODULE_STATE: TdxModuleState = TdxModuleState { initialized: false, sysinit_done: false, sysinit_ret: 0 };
static mut TDX_GLOBAL_KEYID: u32 = 0;
static mut TDX_GUEST_KEYID_START: u32 = 0;
static mut TDX_NR_GUEST_KEYIDS: u32 = 0;

extern "C" {
    static mut tdx_guest_keyid_pool: core::ffi::c_void;
    static mut tdx_lp_initialized: bool;
    static mut tdx_tdmr_list: tdmr_info_list;
    static mut tdx_memlist: list_head;
    static mut tdx_sysinfo: tdx_sys_info;
    fn seamcall_prerr(call: u64, args: *mut tdx_module_args) -> i32;
    fn seamcall_prerr_ret(call: u64, args: *mut tdx_module_args) -> i32;
    fn seamcall(call: u64, args: *mut tdx_module_args) -> u64;
    fn seamcall_ret(call: u64, args: *mut tdx_module_args) -> u64;
    fn __seamcall_dirty_cache(ret: unsafe extern "C" fn() -> u64, call: u64, args: *mut tdx_module_args) -> u64;
    fn __seamcall_saved_ret() -> u64;
    fn get_tdx_sys_info(info: *mut tdx_sys_info) -> i32;
    fn get_tdx_sys_info_handoff(info: *mut tdx_sys_info_handoff) -> i32;
    fn get_tdx_sys_info_version(version: *mut tdx_sys_info_version) -> i32;
}

// External kernel structures are intentionally referenced by their source names.
#[repr(C)] pub struct tdx_module_args { pub rcx:u64, pub rdx:u64, pub r8:u64, pub r9:u64 }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct tdmr_info_list { pub tdmrs:*mut u8, pub tdmr_sz:usize, pub max_tdmrs:i32, pub nr_consumed_tdmrs:i32 }
#[repr(C)] pub struct tdmr_info { pub base:u64, pub size:u64, pub pamt_4k_base:u64, pub pamt_4k_size:u64, pub pamt_2m_base:u64, pub pamt_2m_size:u64, pub pamt_1g_base:u64, pub pamt_1g_size:u64, pub reserved_areas:*mut tdmr_reserved_area }
#[repr(C)] pub struct tdmr_reserved_area { pub offset:u64, pub size:u64 }
#[repr(C)] pub struct tdx_memblock { pub list:list_head, pub start_pfn:usize, pub end_pfn:usize, pub nid:i32 }
#[repr(C)] pub struct tdx_sys_info { pub features:tdx_sys_info_features, pub tdmr:tdx_sys_info_tdmr, pub version:tdx_sys_info_version }
#[repr(C)] pub struct tdx_sys_info_features { pub tdx_features0:u64 }
#[repr(C)] pub struct tdx_sys_info_tdmr { pub max_reserved_per_tdmr:u16, pub max_tdmrs:i32, pub pamt_4k_entry_size:u16, pub pamt_2m_entry_size:u16, pub pamt_1g_entry_size:u16 }
#[repr(C)] pub struct tdx_sys_info_version { pub major:u16, pub minor:u16, pub build:u16, pub reserved:u16 }
#[repr(C)] pub struct tdx_sys_info_handoff { pub module_hv:u64 }
#[repr(C)] pub struct tdx_td { pub tdr_page:*mut page }
#[repr(C)] pub struct tdx_vp { pub tdvpr_pa:u64, pub tdvpr_page:*mut page }
#[repr(C)] pub struct page;
#[repr(C)] pub struct mce { pub addr:usize }

const TDMR_ALIGNMENT:u64=1<<30; const PAGE_SIZE:usize=4096; const PAGE_SHIFT:u32=12;
const TDX_PS_4K:usize=0; const TDX_PS_2M:usize=1; const TDX_PS_1G:usize=2; const TDX_PS_NR:usize=3;
const TDX_FEATURES0_NO_RBP_MOD:u64=1; const TDX_VERSION_SHIFT:u32=32;

unsafe fn tdmr_entry(l:*mut tdmr_info_list, i:i32)->*mut tdmr_info { (*l).tdmrs.add((*l).tdmr_sz*i as usize) as *mut tdmr_info }
unsafe fn tdmr_end(t:*mut tdmr_info)->u64 { (*t).base.wrapping_add((*t).size) }
unsafe fn tdmr_get_pamt(t:*mut tdmr_info,b:*mut usize,s:*mut usize){*b=(*t).pamt_4k_base as usize;*s=((*t).pamt_4k_size+(*t).pamt_2m_size+(*t).pamt_1g_size) as usize;}

unsafe fn try_init_module_global()->i32 { let mut a=tdx_module_args{rcx:0,rdx:0,r8:0,r9:0}; if TDX_MODULE_STATE.sysinit_done{return TDX_MODULE_STATE.sysinit_ret;} let r=seamcall_prerr(TDH_SYS_INIT,&mut a); TDX_MODULE_STATE.sysinit_done=true; TDX_MODULE_STATE.sysinit_ret=r; r }
pub unsafe fn tdx_cpu_enable()->i32 { if tdx_lp_initialized{return 0} let r=try_init_module_global(); if r!=0{return r} let r=seamcall_prerr(TDH_SYS_LP_INIT,&mut tdx_module_args{rcx:0,rdx:0,r8:0,r9:0}); if r==0{tdx_lp_initialized=true} r }

unsafe fn tdmr_get_pamt_sz(t:*mut tdmr_info, p:usize, es:u16)->usize { let n=match p {TDX_PS_4K=>(*t).size as usize>>PAGE_SHIFT,TDX_PS_2M=>(*t).size as usize>>21,TDX_PS_1G=>(*t).size as usize>>30,_=>0}; (n*es as usize+PAGE_SIZE-1)&!(PAGE_SIZE-1) }
unsafe fn tdmr_add_rsvd_area(t:*mut tdmr_info, idx:*mut i32, addr:u64,size:u64,max:u16)->i32 { if addr&((PAGE_SIZE as u64)-1)!=0||size&((PAGE_SIZE as u64)-1)!=0{return -22} if *idx>=max as i32{return -28} (*t).reserved_areas.add(*idx as usize).write(tdmr_reserved_area{offset:addr-(*t).base,size});*idx+=1;0 }

pub unsafe fn tdh_vp_enter(td:*mut tdx_vp,args:*mut tdx_module_args)->u64 {(*args).rcx=(*td).tdvpr_pa;__seamcall_dirty_cache(__seamcall_saved_ret,TDH_VP_ENTER,args)}
pub unsafe fn tdh_mng_addcx(td:*mut tdx_td,p:*mut page)->u64 {let mut a=tdx_module_args{rcx:page_to_phys(p),rdx:tdx_tdr_pa(td),r8:0,r9:0};tdx_clflush_page(p);seamcall(TDH_MNG_ADDCX,&mut a)}
pub unsafe fn tdh_mem_page_add(td:*mut tdx_td,gpa:u64,pfn:usize,src:*mut page,e1:*mut u64,e2:*mut u64)->u64 {let mut a=tdx_module_args{rcx:gpa,rdx:tdx_tdr_pa(td),r8:pfn<<PAGE_SHIFT,r9:page_to_phys(src)};tdx_clflush_pfn(pfn);let r=seamcall_ret(TDH_MEM_PAGE_ADD,&mut a);*e1=a.rcx;*e2=a.rdx;r}
unsafe fn tdx_tdr_pa(td:*mut tdx_td)->u64 {page_to_phys((*td).tdr_page)}
unsafe fn tdx_clflush_page(p:*mut page){clflush_cache_range(page_to_virt(p),PAGE_SIZE)}
unsafe fn tdx_clflush_pfn(p:usize){clflush_cache_range(__va((p<<PAGE_SHIFT) as u64),PAGE_SIZE)}

pub unsafe fn tdh_mng_key_config(td:*mut tdx_td)->u64{let mut a=tdx_module_args{rcx:tdx_tdr_pa(td),rdx:0,r8:0,r9:0};seamcall(TDH_MNG_KEY_CONFIG,&mut a)}
pub unsafe fn tdh_mng_create(td:*mut tdx_td,h:u16)->u64{let mut a=tdx_module_args{rcx:tdx_tdr_pa(td),rdx:h as u64,r8:0,r9:0};tdx_clflush_page((*td).tdr_page);seamcall(TDH_MNG_CREATE,&mut a)}
pub unsafe fn tdh_vp_create(td:*mut tdx_td,vp:*mut tdx_vp)->u64{let mut a=tdx_module_args{rcx:(*vp).tdvpr_pa,rdx:tdx_tdr_pa(td),r8:0,r9:0};tdx_clflush_page((*vp).tdvpr_page);seamcall(TDH_VP_CREATE,&mut a)}
pub unsafe fn tdh_mr_finalize(td:*mut tdx_td)->u64{let mut a=tdx_module_args{rcx:tdx_tdr_pa(td),rdx:0,r8:0,r9:0};seamcall(TDH_MR_FINALIZE,&mut a)}
pub unsafe fn tdh_vp_flush(vp:*mut tdx_vp)->u64{let mut a=tdx_module_args{rcx:(*vp).tdvpr_pa,rdx:0,r8:0,r9:0};seamcall(TDH_VP_FLUSH,&mut a)}
pub unsafe fn tdh_mng_vpflushdone(td:*mut tdx_td)->u64{let mut a=tdx_module_args{rcx:tdx_tdr_pa(td),rdx:0,r8:0,r9:0};seamcall(TDH_MNG_VPFLUSHDONE,&mut a)}
pub unsafe fn tdh_mng_key_freeid(td:*mut tdx_td)->u64{let mut a=tdx_module_args{rcx:tdx_tdr_pa(td),rdx:0,r8:0,r9:0};seamcall(TDH_MNG_KEY_FREEID,&mut a)}
pub unsafe fn tdh_mem_sept_add(td:*mut tdx_td,gpa:u64,level:u64,p:*mut page,e1:*mut u64,e2:*mut u64)->u64{let mut a=tdx_module_args{rcx:gpa|level,rdx:tdx_tdr_pa(td),r8:page_to_phys(p),r9:0};tdx_clflush_page(p);let r=seamcall_ret(TDH_MEM_SEPT_ADD,&mut a);*e1=a.rcx;*e2=a.rdx;r}
pub unsafe fn tdh_vp_addcx(vp:*mut tdx_vp,p:*mut page)->u64{let mut a=tdx_module_args{rcx:page_to_phys(p),rdx:(*vp).tdvpr_pa,r8:0,r9:0};tdx_clflush_page(p);seamcall(TDH_VP_ADDCX,&mut a)}
pub unsafe fn tdh_mem_page_aug(td:*mut tdx_td,gpa:u64,level:u64,pfn:usize,e1:*mut u64,e2:*mut u64)->u64{let mut a=tdx_module_args{rcx:gpa|level,rdx:tdx_tdr_pa(td),r8:(pfn<<PAGE_SHIFT) as u64,r9:0};tdx_clflush_pfn(pfn);let r=seamcall_ret(TDH_MEM_PAGE_AUG,&mut a);*e1=a.rcx;*e2=a.rdx;r}
pub unsafe fn tdh_mem_range_block(td:*mut tdx_td,gpa:u64,level:u64,e1:*mut u64,e2:*mut u64)->u64{let mut a=tdx_module_args{rcx:gpa|level,rdx:tdx_tdr_pa(td),r8:0,r9:0};let r=seamcall_ret(TDH_MEM_RANGE_BLOCK,&mut a);*e1=a.rcx;*e2=a.rdx;r}
pub unsafe fn tdh_mng_rd(td:*mut tdx_td,f:u64,d:*mut u64)->u64{let mut a=tdx_module_args{rcx:tdx_tdr_pa(td),rdx:f,r8:0,r9:0};let r=seamcall_ret(TDH_MNG_RD,&mut a);*d=a.r8;r}
pub unsafe fn tdh_mr_extend(td:*mut tdx_td,gpa:u64,e1:*mut u64,e2:*mut u64)->u64{let mut a=tdx_module_args{rcx:gpa,rdx:tdx_tdr_pa(td),r8:0,r9:0};let r=seamcall_ret(TDH_MR_EXTEND,&mut a);*e1=a.rcx;*e2=a.rdx;r}
pub unsafe fn tdh_mng_init(td:*mut tdx_td,p:u64,e:*mut u64)->u64{let mut a=tdx_module_args{rcx:tdx_tdr_pa(td),rdx:p,r8:0,r9:0};let r=seamcall_ret(TDH_MNG_INIT,&mut a);*e=a.rcx;r}
pub unsafe fn tdh_vp_rd(vp:*mut tdx_vp,f:u64,d:*mut u64)->u64{let mut a=tdx_module_args{rcx:(*vp).tdvpr_pa,rdx:f,r8:0,r9:0};let r=seamcall_ret(TDH_VP_RD,&mut a);*d=a.r8;r}
pub unsafe fn tdh_vp_wr(vp:*mut tdx_vp,f:u64,d:u64,m:u64)->u64{let mut a=tdx_module_args{rcx:(*vp).tdvpr_pa,rdx:f,r8:d,r9:m};seamcall(TDH_VP_WR,&mut a)}
pub unsafe fn tdh_vp_init(vp:*mut tdx_vp,rcx:u64,id:u32)->u64{let mut a=tdx_module_args{rcx:(*vp).tdvpr_pa,rdx:rcx,r8:id as u64,r9:0};seamcall(TDH_VP_INIT|(1u64<<TDX_VERSION_SHIFT),&mut a)}
extern "C" { fn page_to_phys(p:*mut page)->u64; fn page_to_virt(p:*mut page)->*mut core::ffi::c_void; fn clflush_cache_range(p:*mut core::ffi::c_void,n:usize); fn __va(p:u64)->*mut core::ffi::c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
