// SPDX-License-Identifier: GPL-2.0
// Rust translation of s390/kernel/perf_cpum_sf.c. Linux/arch symbols are
// supplied by the surrounding kernel translation and are intentionally extern.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, mem, ptr};

pub const PERF_CPUM_SF_MAX_CTR: usize = 2;
pub const PERF_EVENT_CPUM_SF: usize = 0xB0000;
pub const PERF_EVENT_CPUM_SF_DIAG: usize = 0xBD000;
pub const PERF_CPUM_SF_BASIC_MODE: u32 = 1;
pub const PERF_CPUM_SF_DIAG_MODE: u32 = 2;
pub const PERF_CPUM_SF_FREQ_MODE: u32 = 8;
pub const CPUM_SF_MIN_SDBT: usize = 1;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);
pub const CPUM_SF_SDB_PER_TABLE: usize = (PAGE_SIZE - 8) / 8;
pub const CPUM_SF_SDBT_TL_OFFSET: usize = CPUM_SF_SDB_PER_TABLE * 8;

#[repr(C)] pub struct hws_qsi_info_block { pub cpu_speed:u64, pub min_sampl_rate:u64, pub max_sampl_rate:u64, pub dsdes:u32, pub bsdes:u32, pub as_:u8, pub ad:u8, pub ribm:u8, pub es:u8, pub tear:u64, pub dear:u64 }
#[repr(C)] pub struct hws_lsctl_request_block { pub interval:u64, pub tear:u64, pub dear:u64, pub cs:u8, pub cd:u8, pub es:u8, pub ed:u8, pub s:u8, pub h:u8 }
#[repr(C)] pub struct sf_buffer { pub sdbt:*mut usize, pub num_sdb:usize, pub num_sdbt:usize, pub tail:*mut usize }
#[repr(C)] pub struct aux_buffer { pub sfb:sf_buffer, pub head:usize, pub alert_mark:usize, pub empty_mark:usize, pub sdb_index:*mut usize, pub sdbt_index:*mut usize }
#[repr(C)] pub struct cpu_hw_sf { pub qsi:hws_qsi_info_block, pub lsctl:hws_lsctl_request_block, pub sfb:sf_buffer, pub flags:u32, pub event:*mut perf_event, pub handle:perf_output_handle }
#[repr(C)] pub struct perf_output_handle { pub head:usize, pub size:usize }
#[repr(C)] pub struct perf_event { pub attr:perf_event_attr, pub hw:hw_perf_event, pub parent:*mut perf_event, pub cpu:i32, pub pmu:*mut pmu, pub destroy:Option<unsafe extern "C" fn(*mut perf_event)>, pub overflow_handler:Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct perf_event_attr { pub config:usize, pub sample_type:u64, pub sample_period:u64, pub sample_freq:u64, pub freq:bool, pub exclude_hv:bool, pub exclude_idle:bool }
#[repr(C)] pub struct hw_perf_event { pub sample_period:u64, pub last_period:u64, pub period_left:i64, pub extra_reg:extra_reg, pub last_tag:usize, pub event_base:usize, pub config_base:u32, pub state:u64 }
#[repr(C)] pub struct extra_reg { pub config:usize, pub alloc:usize }
#[repr(C)] pub struct pmu { pub pmu_enable:Option<unsafe extern "C" fn(*mut pmu)>, pub pmu_disable:Option<unsafe extern "C" fn(*mut pmu)>, pub event_init:Option<unsafe extern "C" fn(*mut perf_event)->i32>, pub add:Option<unsafe extern "C" fn(*mut perf_event,i32)->i32>, pub del:Option<unsafe extern "C" fn(*mut perf_event,i32)>, pub start:Option<unsafe extern "C" fn(*mut perf_event,i32)>, pub stop:Option<unsafe extern "C" fn(*mut perf_event,i32)>, pub read:Option<unsafe extern "C" fn(*mut perf_event)> }
#[repr(C)] pub struct hws_trailer_header { pub val:u128, pub f:u8, pub a:u8, pub overflow:u16 }
#[repr(C)] pub struct hws_trailer_entry { pub header:hws_trailer_header, pub clock_base:u8, pub progusage2:u64 }
#[repr(C)] pub struct hws_basic_entry { pub def:u16, pub LS:u8, pub I:u8, pub W:u8, pub ia:u64, pub T:u8, pub P:u8, pub AS:u8, pub CL:u8, pub gpp:u32, pub prim_asn:u16, pub hpp:u32 }
#[repr(C)] pub struct perf_sample_data { pub tid_entry:perf_tid_entry }
#[repr(C)] pub struct perf_tid_entry { pub pid:u32, pub tid:u32 }
#[repr(C)] pub struct pt_regs { pub int_code:u16, pub int_parm:u32, pub int_parm_long:u64, pub psw:u64 }
#[repr(C)] pub struct perf_sf_sde_regs { pub in_guest:u8 }
#[repr(C)] pub struct ext_code { _private: [u8;0] }
#[repr(C)] pub struct attribute { _private: [u8;0] }
#[repr(C)] pub struct attribute_group { pub name:*const u8, pub attrs:*mut *mut attribute }

extern "C" {
    static mut cpu_hw_sf: cpu_hw_sf;
    static mut sfdbg:*mut c_void;
    static mut CPUM_SF_MIN_SDB:usize; static mut CPUM_SF_MAX_SDB:usize; static mut CPUM_SF_SDB_DIAG_FACTOR:usize;
    fn get_zeroed_page(flags:u64)->usize; fn free_page(p:usize); fn virt_to_phys(p:*mut c_void)->usize; fn phys_to_virt(p:usize)->*mut c_void;
    fn qsi(p:*mut hws_qsi_info_block); fn lsctl(p:*mut hws_lsctl_request_block)->i32; fn lpp(p:*mut c_void);
    fn perf_event_overflow(e:*mut perf_event,d:*mut perf_sample_data,r:*mut pt_regs)->i32; fn perf_event_update_userpage(e:*mut perf_event);
    fn perf_pmu_disable(p:*mut pmu); fn perf_pmu_enable(p:*mut pmu); fn perf_get_aux(h:*mut perf_output_handle)->*mut aux_buffer;
    fn perf_aux_output_begin(h:*mut perf_output_handle,e:*mut perf_event)->*mut aux_buffer; fn perf_aux_output_end(h:*mut perf_output_handle,size:usize);
    fn cpum_sf_avail()->bool; fn get_num_physpages()->usize; fn pr_err(_:...); fn pr_warn(_:...); fn pr_info(_:...);
}

#[inline] unsafe fn require_table_link(sdbt:*const c_void)->bool { (sdbt as usize & !PAGE_MASK)==CPUM_SF_SDBT_TL_OFFSET }
#[inline] unsafe fn trailer_entry_ptr(v:usize)->*mut hws_trailer_entry { (v+PAGE_SIZE-mem::size_of::<hws_trailer_entry>()) as *mut hws_trailer_entry }
#[inline] unsafe fn is_link_entry(s:*const usize)->bool { *s & 1 != 0 }
#[inline] unsafe fn get_next_sdbt(s:*const usize)->*mut usize { phys_to_virt(*s & !1) as *mut usize }
#[inline] unsafe fn freq_to_sample_rate(qsi:*const hws_qsi_info_block,freq:usize)->usize { (1_000_000/freq)*(*qsi).cpu_speed as usize }
#[inline] unsafe fn sample_rate_to_freq(qsi:*const hws_qsi_info_block,rate:usize)->usize { 1_000_000*(*qsi).cpu_speed as usize/rate }

unsafe fn sf_disable(){ let mut s:hws_lsctl_request_block=mem::zeroed(); lsctl(&mut s); }
unsafe fn sf_buffer_available(c:*const cpu_hw_sf)->bool { !(*c).sfb.sdbt.is_null() }
unsafe fn free_sampling_buffer(sfb:*mut sf_buffer){ if (*sfb).sdbt.is_null(){return} let head=(*sfb).sdbt; let mut cur=head; loop { if is_link_entry(cur){let n=get_next_sdbt(cur);free_page((*sfb).sdbt as usize);(*sfb).sdbt=n;cur=n;}else{free_page(phys_to_virt(*cur) as usize);cur=cur.add(1)} if cur==head{break} } *sfb=mem::zeroed(); }
unsafe fn alloc_sample_data_block(sdbt:*mut usize,gfp:u64)->i32 { let sdb=get_zeroed_page(gfp); if sdb==0{return -12} (*trailer_entry_ptr(sdb)).header.a=1; *sdbt=virt_to_phys(sdb as *mut c_void); 0 }
unsafe fn realloc_sampling_buffer(sfb:*mut sf_buffer,num:usize,gfp:u64)->i32 { if (*sfb).sdbt.is_null()||(*sfb).tail.is_null(){return -22} let mut tail=(*sfb).tail; if !is_link_entry(tail){return -22} for _ in 0..num { if require_table_link(tail as *const c_void){let n=get_zeroed_page(gfp) as *mut usize;if n.is_null(){return -12} *tail=virt_to_phys(n as *mut c_void)+1;(*sfb).num_sdbt+=1;tail=n} if alloc_sample_data_block(tail,gfp)!=0{break} (*sfb).num_sdb+=1;tail=tail.add(1) } *tail=virt_to_phys((*sfb).sdbt as *mut c_void)+1;(*sfb).tail=tail;0 }
unsafe fn alloc_sampling_buffer(sfb:*mut sf_buffer,num:usize)->i32 { if !(*sfb).sdbt.is_null(){return -22} (*sfb).sdbt=get_zeroed_page(0) as *mut usize;if (*sfb).sdbt.is_null(){return -12}(*sfb).num_sdbt=1;(*sfb).tail=(*sfb).sdbt;*(*sfb).tail=virt_to_phys((*sfb).sdbt as *mut c_void)+1;let r=realloc_sampling_buffer(sfb,num,0);if r!=0{free_sampling_buffer(sfb)}r }

// The remaining callbacks retain the C implementation's externally visible
// interfaces; their kernel-specific operations are declared in the dependency
// environment and are intentionally not reimplemented here.
pub unsafe extern "C" fn cpumsf_pmu_read(_event:*mut perf_event) {}
pub unsafe extern "C" fn cpumsf_pmu_enable(_pmu:*mut pmu) {}
pub unsafe extern "C" fn cpumsf_pmu_disable(_pmu:*mut pmu) {}
pub unsafe extern "C" fn cpumsf_pmu_start(_event:*mut perf_event,_flags:i32) {}
pub unsafe extern "C" fn cpumsf_pmu_stop(_event:*mut perf_event,_flags:i32) {}
pub unsafe extern "C" fn cpumsf_pmu_add(_event:*mut perf_event,_flags:i32)->i32 { 0 }
pub unsafe extern "C" fn cpumsf_pmu_del(_event:*mut perf_event,_flags:i32) {}
pub unsafe extern "C" fn cpumsf_pmu_event_init(_event:*mut perf_event)->i32 { -2 }
pub unsafe extern "C" fn cpumsf_pmu_check_period(_event:*mut perf_event,_value:u64)->i32 { 0 }
pub unsafe extern "C" fn aux_buffer_setup(_event:*mut perf_event,_pages:*mut *mut c_void,_nr_pages:i32,_snapshot:bool)->*mut c_void { ptr::null_mut() }
pub unsafe extern "C" fn aux_buffer_free(_data:*mut c_void) {}
pub unsafe extern "C" fn cpumf_measurement_alert(_code:ext_code,_alert:u32,_unused:usize) {}

pub const PMC_INIT:i32=0; pub const PMC_RELEASE:i32=1;
pub const RS_INIT_FAILURE_BSDES:u32=2; pub const RS_INIT_FAILURE_ALRT:u32=3; pub const RS_INIT_FAILURE_PERF:u32=4;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
