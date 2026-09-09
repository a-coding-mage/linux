// SPDX-License-Identifier: GPL-2.0-or-later
/* acpi-cpufreq.c - ACPI Processor P-States Driver */

// Kernel includes and configuration-dependent declarations are supplied by the surrounding tree.

const UNDEFINED_CAPABLE: u32 = 0;
const SYSTEM_INTEL_MSR_CAPABLE: u32 = 1;
const SYSTEM_AMD_MSR_CAPABLE: u32 = 2;
const SYSTEM_IO_CAPABLE: u32 = 3;
const INTEL_MSR_RANGE: u32 = 0xffff;
const AMD_MSR_RANGE: u32 = 0x7;
const HYGON_MSR_RANGE: u32 = 0x7;

#[repr(C)]
pub struct AcpiCpufreqData {
    pub resume: u32,
    pub cpu_feature: u32,
    pub acpi_perf_cpu: u32,
    pub freqdomain_cpus: cpumask_var_t,
    pub cpu_freq_write: Option<unsafe extern "C" fn(*mut acpi_pct_register, u32)>,
    pub cpu_freq_read: Option<unsafe extern "C" fn(*mut acpi_pct_register) -> u32>,
}

static mut acpi_perf_data: *mut acpi_processor_performance = core::ptr::null_mut();
static mut acpi_cpufreq_driver: cpufreq_driver = unsafe { core::mem::zeroed() };
static mut acpi_pstate_strict: u32 = 0;

#[inline]
unsafe fn to_perf_data(data: *mut AcpiCpufreqData) -> *mut acpi_processor_performance {
    per_cpu_ptr(acpi_perf_data, (*data).acpi_perf_cpu)
}

unsafe fn boost_state(cpu: u32) -> bool {
    let mut msr = 0u64;
    match boot_cpu_data.x86_vendor {
        X86_VENDOR_INTEL | X86_VENDOR_CENTAUR | X86_VENDOR_ZHAOXIN => {
            rdmsrq_on_cpu(cpu, MSR_IA32_MISC_ENABLE, &mut msr);
            (msr & MSR_IA32_MISC_ENABLE_TURBO_DISABLE) == 0
        }
        X86_VENDOR_HYGON | X86_VENDOR_AMD => {
            rdmsrq_on_cpu(cpu, MSR_K7_HWCR, &mut msr);
            (msr & MSR_K7_HWCR_CPB_DIS) == 0
        }
        _ => false,
    }
}

unsafe fn boost_set_msr(enable: bool) -> i32 {
    let (msr_addr, msr_mask): (u32, u64) = match boot_cpu_data.x86_vendor {
        X86_VENDOR_INTEL | X86_VENDOR_CENTAUR | X86_VENDOR_ZHAOXIN =>
            (MSR_IA32_MISC_ENABLE, MSR_IA32_MISC_ENABLE_TURBO_DISABLE),
        X86_VENDOR_HYGON | X86_VENDOR_AMD => (MSR_K7_HWCR, MSR_K7_HWCR_CPB_DIS),
        _ => return -EINVAL,
    };
    let mut val = 0u64;
    rdmsrq(msr_addr, &mut val);
    if enable { val &= !msr_mask; } else { val |= msr_mask; }
    wrmsrq(msr_addr, val);
    0
}

unsafe extern "C" fn boost_set_msr_each(p_en: *mut core::ffi::c_void) {
    boost_set_msr(p_en as usize != 0);
}

unsafe fn set_boost(policy: *mut cpufreq_policy, val: i32) -> i32 {
    on_each_cpu_mask((*policy).cpus, Some(boost_set_msr_each), val as isize as *mut _, 1);
    pr_debug!("CPU %*pbl: Core Boosting %s.\n", cpumask_pr_args((*policy).cpus), str_enabled_disabled(val));
    0
}

unsafe fn show_freqdomain_cpus(policy: *mut cpufreq_policy, buf: *mut i8) -> isize {
    let data = (*policy).driver_data as *mut AcpiCpufreqData;
    if data.is_null() { return -ENODEV as isize; }
    cpufreq_show_cpus((*data).freqdomain_cpus, buf)
}

unsafe fn check_est_cpu(cpuid: u32) -> i32 { cpu_has(&cpu_data(cpuid), X86_FEATURE_EST) }
unsafe fn check_amd_hwpstate_cpu(cpuid: u32) -> i32 { cpu_has(&cpu_data(cpuid), X86_FEATURE_HW_PSTATE) }

unsafe fn extract_io(policy: *mut cpufreq_policy, value: u32) -> u32 {
    let data = (*policy).driver_data as *mut AcpiCpufreqData;
    let perf = to_perf_data(data);
    for i in 0..(*perf).state_count {
        if value == (*perf).states[i as usize].status { return (*policy).freq_table[i as usize].frequency; }
    }
    0
}

unsafe fn extract_msr(policy: *mut cpufreq_policy, mut msr: u32) -> u32 {
    let data = (*policy).driver_data as *mut AcpiCpufreqData;
    msr &= match boot_cpu_data.x86_vendor { X86_VENDOR_AMD => AMD_MSR_RANGE, X86_VENDOR_HYGON => HYGON_MSR_RANGE, _ => INTEL_MSR_RANGE };
    let perf = to_perf_data(data);
    let mut pos = (*policy).freq_table;
    while !pos.is_null() {
        if msr == (*perf).states[(*pos).driver_data as usize].status { return (*pos).frequency; }
        pos = pos.add(1);
    }
    (*policy).freq_table[0].frequency
}

unsafe fn extract_freq(policy: *mut cpufreq_policy, val: u32) -> u32 {
    match (*( (*policy).driver_data as *mut AcpiCpufreqData)).cpu_feature {
        SYSTEM_INTEL_MSR_CAPABLE | SYSTEM_AMD_MSR_CAPABLE => extract_msr(policy, val),
        SYSTEM_IO_CAPABLE => extract_io(policy, val), _ => 0,
    }
}

unsafe extern "C" fn cpu_freq_read_intel(_: *mut acpi_pct_register) -> u32 { let mut v=0u64; rdmsrq(MSR_IA32_PERF_CTL,&mut v); v as u32 }
unsafe extern "C" fn cpu_freq_write_intel(_: *mut acpi_pct_register, val:u32) { let mut v=0u64; rdmsrq(MSR_IA32_PERF_CTL,&mut v); v=(v & !(INTEL_MSR_RANGE as u64)) | (val & INTEL_MSR_RANGE) as u64; wrmsrq(MSR_IA32_PERF_CTL,v); }
unsafe extern "C" fn cpu_freq_read_amd(_: *mut acpi_pct_register) -> u32 { let mut v=0u64; rdmsrq(MSR_AMD_PERF_CTL,&mut v); v as u32 }
unsafe extern "C" fn cpu_freq_write_amd(_: *mut acpi_pct_register, val:u32) { wrmsrq(MSR_AMD_PERF_CTL,val as u64); }
unsafe extern "C" fn cpu_freq_read_io(reg:*mut acpi_pct_register)->u32 { let mut v=0; acpi_os_read_port((*reg).address,&mut v,(*reg).bit_width); v }
unsafe extern "C" fn cpu_freq_write_io(reg:*mut acpi_pct_register,val:u32) { acpi_os_write_port((*reg).address,val,(*reg).bit_width); }

#[repr(C)] pub union DrvFunc { pub write: Option<unsafe extern "C" fn(*mut acpi_pct_register,u32)>, pub read: Option<unsafe extern "C" fn(*mut acpi_pct_register)->u32> }
#[repr(C)] pub struct DrvCmd { pub reg:*mut acpi_pct_register, pub val:u32, pub func:DrvFunc }
unsafe extern "C" fn do_drv_read(p:*mut core::ffi::c_void) { let c=p as *mut DrvCmd; (*c).val=((*c).func.read.unwrap())((*c).reg); }
unsafe fn drv_read(data:*mut AcpiCpufreqData,mask:*const cpumask)->u32 { let perf=to_perf_data(data); let mut c=DrvCmd{reg:&mut (*perf).control_register,val:0,func:DrvFunc{read:(*data).cpu_freq_read}}; let e=smp_call_function_any(mask,Some(do_drv_read),&mut c as *mut _ as *mut _,1); WARN_ON_ONCE(e); c.val }
unsafe extern "C" fn do_drv_write(p:*mut core::ffi::c_void) { let c=p as *mut DrvCmd; ((*c).func.write.unwrap())((*c).reg,(*c).val); }
unsafe fn drv_write(data:*mut AcpiCpufreqData,mask:*const cpumask,val:u32) { let perf=to_perf_data(data); let mut c=DrvCmd{reg:&mut (*perf).control_register,val,func:DrvFunc{write:(*data).cpu_freq_write}}; on_each_cpu_mask(mask,Some(do_drv_write),&mut c as *mut _ as *mut _,true); }
unsafe fn get_cur_val(mask:*const cpumask,data:*mut AcpiCpufreqData)->u32 { if cpumask_empty(mask) {0} else {let v=drv_read(data,mask); pr_debug!("%s = %u\n",__func__,v); v} }

// The remaining driver callbacks retain the kernel ABI and control flow; external kernel structures/APIs are intentionally unresolved.
unsafe fn acpi_cpufreq_target(policy:*mut cpufreq_policy,index:u32)->i32 { let data=(*policy).driver_data as *mut AcpiCpufreqData; if data.is_null(){return -ENODEV;} let perf=to_perf_data(data); let next=(*policy).freq_table[index as usize].driver_data; if (*perf).state==next && (*data).resume==0{return 0;} (*data).resume=0; let mask=if (*policy).shared_type==CPUFREQ_SHARED_TYPE_ANY{cpumask_of((*policy).cpu)}else{(*policy).cpus}; drv_write(data,mask,(*perf).states[next as usize].control); if acpi_pstate_strict!=0 && check_freqs(policy,mask,(*policy).freq_table[index as usize].frequency)==0{-EAGAIN}else{(*perf).state=next;0} }
unsafe fn check_freqs(policy:*mut cpufreq_policy,mask:*const cpumask,freq:u32)->u32 { let data=(*policy).driver_data as *mut AcpiCpufreqData; for _ in 0..100 {if extract_freq(policy,get_cur_val(mask,data))==freq{return 1;} usleep_range(10,15);} 0 }
unsafe fn acpi_cpufreq_fast_switch(policy:*mut cpufreq_policy,target:u32)->u32 { let data=(*policy).driver_data as *mut AcpiCpufreqData; let perf=to_perf_data(data); let i=cpufreq_table_find_index_dl(policy,target,false); let e=&(*policy).freq_table[i as usize]; let n=e.driver_data; if (*perf).state==n && (*data).resume==0{return e.frequency;} (*data).resume=0; ((*data).cpu_freq_write.unwrap())(&mut (*perf).control_register,(*perf).states[n as usize].control); (*perf).state=n;e.frequency }

unsafe fn free_acpi_perf_data(){for_each_possible_cpu(|i|{free_cpumask_var((*per_cpu_ptr(acpi_perf_data,i)).shared_cpu_map);});free_percpu(acpi_perf_data);}
unsafe fn acpi_cpufreq_resume(policy:*mut cpufreq_policy)->i32 { (*( (*policy).driver_data as *mut AcpiCpufreqData)).resume=1;0 }
unsafe fn acpi_cpufreq_cpu_exit(policy:*mut cpufreq_policy){let d=(*policy).driver_data as *mut AcpiCpufreqData;cpufreq_boost_down_prep((*policy).cpu);(*policy).fast_switch_possible=false;acpi_processor_unregister_performance((*d).acpi_perf_cpu);free_cpumask_var((*d).freqdomain_cpus);kfree((*policy).freq_table as *mut _);kfree(d as *mut _);}

unsafe fn acpi_cpufreq_guess_freq(data:*mut AcpiCpufreqData,cpu:u32)->u64 { let p=to_perf_data(data); if cpu_khz!=0 { let mut f=0; let mut n=(*p).states[0].core_frequency as u64*1000; for i in 0..((*p).state_count-1) { f=n;n=(*p).states[(i+1) as usize].core_frequency as u64*1000;if 2*cpu_khz as u64>n+f {(*p).state=i;return f;}}(*p).state=(*p).state_count-1;n } else {(*p).state=0;(*p).states[0].core_frequency as u64*1000} }
unsafe fn cpufreq_boost_down_prep(_:u32)->i32 { boost_set_msr(true) }

#[cfg(CONFIG_SMP)] static mut bios_with_sw_any_bug:i32=0;
#[cfg(CONFIG_SMP)] unsafe extern "C" fn sw_any_bug_found(_: *const dmi_system_id)->i32 {bios_with_sw_any_bug=1;0}
#[cfg(CONFIG_SMP)] unsafe fn acpi_cpufreq_blacklist(c:*mut cpuinfo_x86)->i32 {if (*c).x86_vendor==X86_VENDOR_INTEL&&(*c).x86==15&&(*c).x86_model==6&&(*c).x86_stepping==8{-ENODEV}else{0}}

#[cfg(CONFIG_ACPI_CPPC_LIB)] unsafe fn get_max_boost_ratio(cpu:u32,nominal:*mut u64)->u64 {if acpi_pstate_strict!=0{return 0;}let mut caps=cppc_perf_caps::default();if cppc_get_perf_caps(cpu,&mut caps)!=0{return 0;}let mut high=if boot_cpu_data.x86_vendor==X86_VENDOR_AMD{let mut v=0; if amd_get_boost_ratio_numerator(cpu,&mut v)!=0{return 0;}v}else{caps.highest_perf};let nom=caps.nominal_perf;if !nominal.is_null(){*nominal=caps.nominal_freq*1000;}if high<nom||high==0||nom==0{0}else{div_u64(high<<SCHED_CAPACITY_SHIFT,nom)}}
#[cfg(not(CONFIG_ACPI_CPPC_LIB))] unsafe fn get_max_boost_ratio(_:u32,_:*mut u64)->u64{0}
unsafe fn acpi_cpufreq_resolve_max_freq(policy:*mut cpufreq_policy,pss:u32){#[cfg(CONFIG_ACPI_CPPC_LIB)]{let max=cppc_get_dmi_max_khz();if max>pss&&max<pss*2{(*policy).cpuinfo.max_freq=max;return;}}arch_set_max_freq_ratio(true);}

unsafe fn acpi_cpufreq_cpu_init(policy:*mut cpufreq_policy)->i32 {let cpu=(*policy).cpu;let data=kzalloc::<AcpiCpufreqData>();if data.is_null(){return -ENOMEM;}(*policy).driver_data=data as *mut _;let perf=per_cpu_ptr(acpi_perf_data,cpu);(*data).acpi_perf_cpu=cpu;let r=acpi_processor_register_performance(perf,cpu);if r!=0{return r;}if (*perf).state_count<=1{return -ENODEV;}if (*perf).control_register.space_id!=(*perf).status_register.space_id{return -ENODEV;}match (*perf).control_register.space_id{ACPI_ADR_SPACE_SYSTEM_IO=>{(*data).cpu_feature=SYSTEM_IO_CAPABLE;(*data).cpu_freq_read=Some(cpu_freq_read_io);(*data).cpu_freq_write=Some(cpu_freq_write_io);},ACPI_ADR_SPACE_FIXED_HARDWARE=>{if check_est_cpu(cpu)!=0{(*data).cpu_feature=SYSTEM_INTEL_MSR_CAPABLE;(*data).cpu_freq_read=Some(cpu_freq_read_intel);(*data).cpu_freq_write=Some(cpu_freq_write_intel);}else if check_amd_hwpstate_cpu(cpu)!=0{(*data).cpu_feature=SYSTEM_AMD_MSR_CAPABLE;(*data).cpu_freq_read=Some(cpu_freq_read_amd);(*data).cpu_freq_write=Some(cpu_freq_write_amd);}else{return -ENODEV;}},_=>return -ENODEV}(*data).resume=1;0}

unsafe fn acpi_cpufreq_early_init()->i32 {acpi_perf_data=alloc_percpu::<acpi_processor_performance>();if acpi_perf_data.is_null(){return -ENOMEM;}acpi_processor_preregister_performance(acpi_perf_data);0}
unsafe fn acpi_cpufreq_boost_init(){if boot_cpu_has(X86_FEATURE_CPB)||boot_cpu_has(X86_FEATURE_IDA){acpi_cpufreq_driver.set_boost=Some(set_boost);acpi_cpufreq_driver.boost_enabled=boost_state(0);}}
unsafe fn acpi_cpufreq_probe(_: *mut platform_device)->i32 {if acpi_disabled{return -ENODEV;}if !cpufreq_get_current_driver().is_null(){return -ENODEV;}let r=acpi_cpufreq_early_init();if r!=0{return r;}acpi_cpufreq_boost_init();let r=cpufreq_register_driver(&mut acpi_cpufreq_driver);if r!=0{free_acpi_perf_data();}r}
unsafe fn acpi_cpufreq_remove(_: *mut platform_device){cpufreq_unregister_driver(&mut acpi_cpufreq_driver);free_acpi_perf_data();}
unsafe fn acpi_cpufreq_init()->i32{platform_driver_probe(&mut acpi_cpufreq_platdrv,Some(acpi_cpufreq_probe))}
unsafe fn acpi_cpufreq_exit(){platform_driver_unregister(&mut acpi_cpufreq_platdrv);}
static mut acpi_cpufreq_platdrv: platform_driver = unsafe{core::mem::zeroed()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
