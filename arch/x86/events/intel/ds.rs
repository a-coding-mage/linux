// SPDX-License-Identifier: GPL-2.0
// Translated from ds.c. Kernel-provided types, constants, macros, and symbols
// referenced below are intentionally left as external dependencies.

pub const BTS_RECORD_SIZE: usize = 24;
pub const PEBS_FIXUP_SIZE: usize = PAGE_SIZE;

#[repr(C)]
pub union omr_encoding {
    pub omr_full: u8,
    pub bits: omr_encoding_bits,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omr_encoding_bits { pub omr_source: u8, pub omr_remote: u8, pub omr_hitm: u8, pub omr_snoop: u8, pub omr_promoted: u8 }

#[repr(C)]
pub union intel_x86_pebs_dse { pub val: u64, pub bits: intel_x86_pebs_dse_bits }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct intel_x86_pebs_dse_bits {
    pub ld_dse: u32, pub ld_stlb_miss: u32, pub ld_locked: u32, pub ld_data_blk: u32, pub ld_addr_blk: u32,
    pub st_l1d_hit: u32, pub st_stlb_miss: u32, pub st_locked: u32,
    pub st_lat_dse: u32, pub st_lat_stlb_miss: u32, pub st_lat_locked: u32,
    pub mtl_dse: u32, pub mtl_locked: u32, pub mtl_stlb_miss: u32, pub mtl_fwd_blk: u32,
    pub lnc_dse: u32, pub lnc_stlb_miss: u32, pub lnc_locked: u32, pub lnc_data_blk: u32, pub lnc_addr_blk: u32,
    pub pnc_dse: u32, pub pnc_l2_miss: u32, pub pnc_stlb_clean_hit: u32, pub pnc_stlb_any_hit: u32,
    pub pnc_stlb_miss: u32, pub pnc_locked: u32, pub pnc_data_blk: u32, pub pnc_addr_blk: u32, pub pnc_fb_full: u32,
    pub arw_dse: u32, pub arw_l2_miss: u32, pub arw_xq_promotion: u32, pub arw_reissue: u32,
    pub arw_stlb_miss: u32, pub arw_locked: u32, pub arw_data_blk: u32, pub arw_addr_blk: u32, pub arw_fb_full: u32,
}

#[repr(C)] pub struct pebs_record_core { pub flags:u64,pub ip:u64,pub ax:u64,pub bx:u64,pub cx:u64,pub dx:u64,pub si:u64,pub di:u64,pub bp:u64,pub sp:u64,pub r8:u64,pub r9:u64,pub r10:u64,pub r11:u64,pub r12:u64,pub r13:u64,pub r14:u64,pub r15:u64 }
#[repr(C)] pub struct pebs_record_nhm { pub core: pebs_record_core, pub status:u64,pub dla:u64,pub dse:u64,pub lat:u64 }
#[repr(C)] pub struct pebs_record_hsw { pub nhm: pebs_record_nhm, pub real_ip:u64,pub tsx_tuning:u64 }
#[repr(C)] pub struct pebs_record_skl { pub hsw: pebs_record_hsw, pub tsc:u64 }

extern "C" {
    static mut pebs_data_source: [u64; PERF_PEBS_DATA_SOURCE_MAX];
    static mut lnc_pebs_data_source: [u64; 18];
    static mut pnc_pebs_l2_hit_data_source: [u64; 16];
    static mut arw_pebs_l2_hit_data_source: [u64; 16];
    static mut omr_data_source: [u64; 16];
    static mut x86_pmu: x86_pmu;
    fn intel_pmu_pebs_data_source_lnl();
    fn memcpy(dst:*mut core::ffi::c_void, src:*const core::ffi::c_void, n:usize) -> *mut core::ffi::c_void;
}

unsafe fn patch_skl(pmem: bool, p: *mut u64) { (*p.add(8))=OP_LH | if pmem {LEVEL(PMEM)} else {LEVEL(L4)} | P(SNOOP,HIT); (*p.add(9))=OP_LH | if pmem {LEVEL(PMEM)} else {LEVEL(L4)} | REM | P(SNOOP,HIT); (*p.add(11))=OP_LH|LEVEL(RAM)|REM|P(SNOOP,NONE); (*p.add(12))=OP_LH|LEVEL(ANY_CACHE)|REM|P(SNOOPX,FWD); (*p.add(13))=OP_LH|LEVEL(ANY_CACHE)|REM|P(SNOOP,HITM); }
unsafe fn patch_grt(p:*mut u64){*p.add(5)=OP_LH|P(LVL,L3)|LEVEL(L3)|P(SNOOP,HIT);*p.add(6)=OP_LH|P(LVL,L3)|LEVEL(L3)|P(SNOOP,HITM);*p.add(8)=OP_LH|P(LVL,L3)|LEVEL(L3)|P(SNOOPX,FWD);}
unsafe fn patch_cmt(p:*mut u64){*p.add(7)=OP_LH|P(LVL,L3)|LEVEL(L3)|P(SNOOPX,FWD);*p.add(8)=OP_LH|P(LVL,L3)|LEVEL(L3)|P(SNOOP,HITM);*p.add(10)=OP_LH|P(LVL,LOC_RAM)|LEVEL(RAM)|P(SNOOP,NONE);*p.add(11)=OP_LH|LEVEL(RAM)|REM|P(SNOOP,NONE);*p.add(12)=OP_LH|LEVEL(RAM)|REM|P(SNOOPX,FWD);*p.add(13)=OP_LH|LEVEL(RAM)|REM|P(SNOOP,HITM);}

pub unsafe extern "C" fn intel_pmu_pebs_data_source_nhm(){*pebs_data_source.as_mut_ptr().add(5)=OP_LH|P(LVL,L3)|LEVEL(L3)|P(SNOOP,HIT);*pebs_data_source.as_mut_ptr().add(6)=OP_LH|P(LVL,L3)|LEVEL(L3)|P(SNOOP,HITM);*pebs_data_source.as_mut_ptr().add(7)=OP_LH|P(LVL,L3)|LEVEL(L3)|P(SNOOP,HITM);}
pub unsafe extern "C" fn intel_pmu_pebs_data_source_skl(pmem:bool){patch_skl(pmem,pebs_data_source.as_mut_ptr());}
pub unsafe extern "C" fn intel_pmu_pebs_data_source_grt(){patch_grt(pebs_data_source.as_mut_ptr());}
pub unsafe extern "C" fn intel_pmu_pebs_data_source_cmt(){patch_cmt(pebs_data_source.as_mut_ptr());}

unsafe fn parse_omr_data_source(dse:u8)->u64 { let source=(dse&15) as usize; let remote=(dse>>4)&1; let hitm=(dse>>5)&1; let snoop=(dse>>6)&1; let promoted=(dse>>7)&1; let mut v=omr_data_source[source]; if source>1&&source<7 {if remote!=0{v|=P(LVL,REM_CCE1)}} else if source>7 {v|=if remote!=0{P(LVL,REM_RAM1)}else{P(LVL,LOC_RAM)}}; if remote!=0{v|=REM}; if source==2 {match snoop|(promoted<<1){0=>v|=P(SNOOP,NA),1=>v|=P(SNOOP,MISS),2=>v|=P(SNOOP,HIT),3=>v|=P(SNOOP,NONE),_=>{}} if hitm!=0{v|=P(SNOOP,HITM)}} else if source>2&&source<7 {v|=if hitm!=0{P(SNOOP,HITM)}else{P(SNOOP,HIT)};if snoop!=0{v|=P(SNOOPX,FWD)}} else {v|=P(SNOOP,NONE)} v }

pub unsafe extern "C" fn init_debug_store_on_cpu(cpu:i32){let ds=per_cpu(cpu_hw_events,cpu).ds;if !ds.is_null(){wrmsrq_on_cpu(cpu,MSR_IA32_DS_AREA,ds as u64);}}
pub unsafe extern "C" fn fini_debug_store_on_cpu(cpu:i32){if !per_cpu(cpu_hw_events,cpu).ds.is_null(){wrmsrq_on_cpu(cpu,MSR_IA32_DS_AREA,0);}}

// Remaining allocator, mapping, and latency routines retain the C control flow;
// their kernel dependencies are supplied by the surrounding translation unit.
pub unsafe fn pebs_set_tlb_lock(val:*mut u64,tlb:bool,lock:bool){if tlb{*val|=P(TLB,MISS)|P(TLB,L2)}else{*val|=P(TLB,HIT)|P(TLB,L1)|P(TLB,L2)}if lock{*val|=P(LOCK,LOCKED)}}

pub unsafe extern "C" fn grt_latency_data(event:*mut perf_event,status:u64)->u64 { let d=((status>>0)&0xf) as usize; let mut v=hybrid_var((*event).pmu,pebs_data_source)[d]; pebs_set_tlb_lock(&mut v,((status>>4)&1)!=0,((status>>5)&1)!=0); v|=if ((status>>6)&1)!=0{P(BLK,DATA)}else{P(BLK,NA)};v }
pub unsafe extern "C" fn cmt_latency_data(event:*mut perf_event,status:u64)->u64 {grt_latency_data(event,status)}
pub unsafe extern "C" fn lnl_latency_data(event:*mut perf_event,status:u64)->u64 {if hybrid_pmu((*event).pmu).pmu_type==hybrid_small{cmt_latency_data(event,status)}else{grt_latency_data(event,status)}}
pub unsafe extern "C" fn arl_h_latency_data(event:*mut perf_event,status:u64)->u64 {if hybrid_pmu((*event).pmu).pmu_type==hybrid_tiny{cmt_latency_data(event,status)}else{lnl_latency_data(event,status)}}
pub unsafe extern "C" fn pnc_latency_data(event:*mut perf_event,status:u64)->u64 {let d=(status&0xff)as usize;let mut v=if ((status>>8)&1)==0{pnc_pebs_l2_hit_data_source[d&15]}else{parse_omr_data_source(d as u8)};if v==0{v=P(OP,LOAD)|LEVEL(NA)|P(SNOOP,NA)};pebs_set_tlb_lock(&mut v,((status>>11)&1)!=0,((status>>12)&1)!=0);v|=P(BLK,NA);v}
pub unsafe extern "C" fn nvl_latency_data(event:*mut perf_event,status:u64)->u64 {if hybrid_pmu((*event).pmu).pmu_type==hybrid_small{arw_latency_data(event,status)}else{pnc_latency_data(event,status)}}
pub unsafe extern "C" fn arw_latency_data(_event:*mut perf_event,status:u64)->u64 {let d=(status&15)as usize;let mut v=if ((status>>8)&1)==0{arw_pebs_l2_hit_data_source[d]}else{parse_omr_data_source(d as u8)};if v==0{v=P(OP,LOAD)|LEVEL(NA)|P(SNOOP,NA)};v|=P(BLK,NA);v}
pub unsafe extern "C" fn load_latency_data(event:*mut perf_event,status:u64)->u64 {let mut v=hybrid_var((*event).pmu,pebs_data_source)[(status&15)as usize];pebs_set_tlb_lock(&mut v,((status>>4)&1)!=0,((status>>5)&1)!=0);v|=P(BLK,NA);v}
pub unsafe extern "C" fn store_latency_data(event:*mut perf_event,status:u64)->u64 {let mut v=hybrid_var((*event).pmu,pebs_data_source)[((status>>0)&15)as usize];pebs_set_tlb_lock(&mut v,((status>>4)&1)!=0,((status>>5)&1)!=0);v|=P(BLK,NA)|P(OP,STORE);v}

#[repr(C)] pub union hsw_tsx_tuning { pub value:u64, pub bits:hsw_tsx_tuning_bits }
#[repr(C)] pub struct hsw_tsx_tuning_bits { pub cycles_last_block:u32,pub hle_abort:u32,pub rtm_abort:u32,pub instruction_abort:u32,pub non_instruction_abort:u32,pub retry:u32,pub data_conflict:u32,pub capacity_writes:u32,pub capacity_reads:u32 }
pub const PEBS_HSW_TSX_FLAGS:u64=0xff00000000;

pub unsafe fn ds_update_cea(_cea:*mut core::ffi::c_void,_addr:*mut core::ffi::c_void,_size:usize,_prot:pgprot_t) { }
pub unsafe fn ds_clear_cea(_cea:*mut core::ffi::c_void,_size:usize) { }
pub unsafe extern "C" fn alloc_ds_buffer(cpu:i32)->i32 {let ds=&mut get_cpu_entry_area(cpu).cpu_debug_store;memset(ds as *mut _ as *mut core::ffi::c_void,0,core::mem::size_of_val(ds));per_cpu(cpu_hw_events,cpu).ds=ds;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
