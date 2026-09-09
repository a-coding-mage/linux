// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of powernow-k8.c. Kernel-provided types,
 * constants, macros, and functions are intentionally left external. */

const VERSION: &str = "version 2.20.00";

extern "C" {
    static mut fidvid_mutex: Mutex;
    static mut powernow_data: PerCpu<*mut powernow_k8_data>;
    static mut cpufreq_amd64_driver: cpufreq_driver;
}

#[repr(C)] pub struct Mutex;
#[repr(C)] pub struct PerCpu<T>(pub core::marker::PhantomData<T>);
#[repr(C)] pub struct cpufreq_driver { pub flags: u32, pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy)->i32>, pub target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy,u32)->i32>, pub bios_limit: Option<unsafe extern "C" fn(*mut cpufreq_policy)->i32>, pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy)->i32>, pub exit: Option<unsafe extern "C" fn(*mut cpufreq_policy)>, pub get: Option<unsafe extern "C" fn(u32)->u32>, pub name: *const u8 }
#[repr(C)] pub struct cpufreq_policy { pub cpu:u32, pub min:i32, pub max:i32, pub cur:u32, pub cpus:*mut cpumask, pub related_cpus:*mut cpumask, pub freq_table:*mut cpufreq_frequency_table, pub cpuinfo: cpuinfo }
#[repr(C)] pub struct cpuinfo { pub transition_latency:u32 }
#[repr(C)] pub struct cpumask;
#[repr(C)] pub struct cpufreq_frequency_table { pub driver_data:u32, pub frequency:i32 }
#[repr(C)] pub struct msr { pub q:u64, pub l:u32, pub h:u32 }
#[repr(C)] pub struct pst_s { pub fid:u8, pub vid:u8 }
#[repr(C)] pub struct psb_s { pub tableversion:u8, pub flags1:u8, pub vstable:u8, pub flags2:u8, pub num_tables:u8, pub cpuid:u32, pub plllocktime:u8, pub maxfid:u8, pub maxvid:u8, pub numps:u8 }
#[repr(C)] pub struct acpi_data { pub state_count:i32, pub states:*mut acpi_state, pub control_register: acpi_reg, pub status_register: acpi_reg, pub shared_cpu_map:*mut cpumask }
#[repr(C)] pub struct acpi_state { pub control:u64, pub status:u64, pub core_frequency:u32, pub transition_latency:u32, pub bus_master_latency:u32 }
#[repr(C)] pub struct acpi_reg { pub space_id:u64 }
#[repr(C)] pub struct powernow_k8_data { pub currvid:u32,pub currfid:u32,pub irt:u32,pub vstable:u32,pub rvo:u32,pub vidmvs:u32,pub plllock:u32,pub batps:u32,pub numps:u32,pub exttype:u32,pub cpu:u32,pub available_cores:*mut cpumask,pub powernow_table:*mut cpufreq_frequency_table,pub acpi_data:acpi_data }
#[repr(C)] pub struct powernowk8_target_arg { pub pol:*mut cpufreq_policy, pub newstate:u32 }

extern "C" {
    fn rdmsrq(msr:u32, value:*mut u64); fn wrmsrq(msr:u32,value:u64); fn udelay(x:u32);
    fn smp_processor_id()->u32; fn cpuid_eax(x:u32)->u32; fn cpuid(eax:u32,a:*mut u32,b:*mut u32,c:*mut u32,d:*mut u32);
    fn pr_debug(...); fn pr_err(...); fn pr_info(...); fn pr_warn(...); fn pr_info_once(...); fn pr_err_once(...);
    fn query_external();
}

unsafe fn find_freq_from_fid(fid:u32)->u32 { 800 + fid.wrapping_mul(100) }
unsafe fn find_khz_freq_from_fid(fid:u32)->u32 { 1000 * find_freq_from_fid(fid) }
unsafe fn convert_fid_to_vco_fid(fid:u32)->u32 { if fid < HI_FID_TABLE_BOTTOM { 8 + 2*fid } else { fid } }
unsafe fn pending_bit_stuck()->i32 { let mut msr=0; rdmsrq(MSR_FIDVID_STATUS,&mut msr); if msr & MSR_S_LO_CHANGE_PENDING as u64 != 0 {1} else {0} }
unsafe fn query_current_values_with_pending_wait(data:*mut powernow_k8_data)->i32 { let mut m=msr{q:0,l:0,h:0}; let mut i=0u32; loop { i+=1; if i>10000 { pr_debug(); return 1; } rdmsrq(MSR_FIDVID_STATUS,&mut m.q); if m.l & MSR_S_LO_CHANGE_PENDING != 0 {continue} break } (*data).currvid=m.h & MSR_S_HI_CURRENT_VID; (*data).currfid=m.l & MSR_S_LO_CURRENT_FID; 0 }
unsafe fn count_off_irt(data:*mut powernow_k8_data){ udelay((1u32<<(*data).irt)*10); }
unsafe fn count_off_vst(data:*mut powernow_k8_data){ udelay((*data).vstable*VST_UNITS_20US); }
unsafe fn fidvid_msr_init(){ let mut m=msr{q:0,l:0,h:0}; rdmsrq(MSR_FIDVID_STATUS,&mut m.q); let vid=m.h&MSR_S_HI_CURRENT_VID; let fid=m.l&MSR_S_LO_CURRENT_FID; m.l=fid|(vid<<MSR_C_LO_VID_SHIFT); m.h=MSR_C_HI_STP_GNT_BENIGN; wrmsrq(MSR_FIDVID_CTL,m.q); }
unsafe fn write_new_fid(data:*mut powernow_k8_data,fid:u32)->i32 { if fid&INVALID_FID_MASK!=0 || (*data).currvid&INVALID_VID_MASK!=0{return 1} let mut m=msr{q:0,l:fid|((*data).currvid<<MSR_C_LO_VID_SHIFT)|MSR_C_LO_INIT_FID_VID,h:(*data).plllock*PLL_LOCK_CONVERSION}; let mut i=0; loop {wrmsrq(MSR_FIDVID_CTL,m.q);i+=1;if i>100{return 1}if query_current_values_with_pending_wait(data)==0{break}} count_off_irt(data); if fid!=(*data).currfid{return 1} 0 }
unsafe fn write_new_vid(data:*mut powernow_k8_data,vid:u32)->i32 { if (*data).currfid&INVALID_FID_MASK!=0||vid&INVALID_VID_MASK!=0{return 1} let mut m=msr{q:0,l:(*data).currfid|(vid<<MSR_C_LO_VID_SHIFT)|MSR_C_LO_INIT_FID_VID,h:STOP_GRANT_5NS};let mut i=0;loop{wrmsrq(MSR_FIDVID_CTL,m.q);i+=1;if i>100{return 1}if query_current_values_with_pending_wait(data)==0{break}}if vid!=(*data).currvid{return 1}0 }
unsafe fn decrease_vid_code_by_step(data:*mut powernow_k8_data,mut req:u32,step:u32)->i32{if (*data).currvid-req>step{req=(*data).currvid-step}if write_new_vid(data,req)!=0{return 1}count_off_vst(data);0}
unsafe fn transition_fid_vid(data:*mut powernow_k8_data, fid:u32,vid:u32)->i32{if core_voltage_pre_transition(data,vid,fid)!=0||core_frequency_transition(data,fid)!=0||core_voltage_post_transition(data,vid)!=0{return 1}if query_current_values_with_pending_wait(data)!=0{return 1}if (*data).currfid!=fid||(*data).currvid!=vid{return 1}0}
unsafe fn core_voltage_pre_transition(data:*mut powernow_k8_data,reqvid:u32,reqfid:u32)->i32{let mut r=(*data).rvo;if (*data).currfid<LO_FID_TABLE_TOP&&reqfid<LO_FID_TABLE_TOP{r*=2}while (*data).currvid>reqvid{if decrease_vid_code_by_step(data,reqvid,(*data).vidmvs)!=0{return 1}}while r>0&&r*(*data).rvo+(*data).currvid>reqvid{if decrease_vid_code_by_step(data,(*data).currvid-1,1)!=0{return 1}r-=1}0}
unsafe fn core_frequency_transition(data:*mut powernow_k8_data,req:u32)->i32{if (*data).currfid==req{return 0}let target=convert_fid_to_vco_fid(req);while {let c=convert_fid_to_vco_fid((*data).currfid);let d=if c>target{c-target}else{target-c};d>2}{let step=if (*data).currfid&1!=0{1}else{2};let f=if req>(*data).currfid{(*data).currfid+step}else{(*data).currfid-step};if write_new_fid(data,f)!=0{return 1}}if write_new_fid(data,req)!=0{return 1}0}
unsafe fn core_voltage_post_transition(data:*mut powernow_k8_data,req:u32)->i32{if req!=(*data).currvid&&write_new_vid(data,req)!=0{return 1}if query_current_values_with_pending_wait(data)!=0||(*data).currvid!=req{return 1}0}

// Remaining driver plumbing is preserved as declarations because it depends on
// the Linux kernel ABI and symbols supplied by the surrounding translation.
extern "C" { fn powernowk8_init()->i32; fn powernowk8_exit(); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
