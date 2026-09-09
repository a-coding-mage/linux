// SPDX-License-Identifier: GPL-2.0-only
/* cpufreq driver for Enhanced SpeedStep, as found in Intel Pentium M. */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation environment.

const MAINTAINER: *const u8 = b"linux-pm@vger.kernel.org\0".as_ptr();
const INTEL_MSR_RANGE: u32 = 0xffff;

#[repr(C)]
pub struct cpu_id { pub x86: u8, pub x86_model: u8, pub x86_stepping: u8 }

pub const CPU_BANIAS: usize = 0;
pub const CPU_DOTHAN_A1: usize = 1;
pub const CPU_DOTHAN_A2: usize = 2;
pub const CPU_DOTHAN_B0: usize = 3;
pub const CPU_MP4HT_D0: usize = 4;
pub const CPU_MP4HT_E0: usize = 5;

static cpu_ids: [cpu_id; 6] = [
    cpu_id { x86: 6, x86_model: 9, x86_stepping: 5 },
    cpu_id { x86: 6, x86_model: 13, x86_stepping: 1 },
    cpu_id { x86: 6, x86_model: 13, x86_stepping: 2 },
    cpu_id { x86: 6, x86_model: 13, x86_stepping: 6 },
    cpu_id { x86: 15, x86_model: 3, x86_stepping: 4 },
    cpu_id { x86: 15, x86_model: 4, x86_stepping: 1 },
];
const N_IDS: usize = cpu_ids.len();

#[repr(C)]
pub struct cpu_model {
    pub cpu_id: *const cpu_id,
    pub model_name: *const u8,
    pub max_freq: u32,
    pub op_points: *mut cpufreq_frequency_table,
}

// External kernel types, globals, macros, and functions are provided elsewhere.
extern "C" {
    fn centrino_verify_cpu_id(c: *const cpuinfo_x86, x: *const cpu_id) -> i32;
    static mut centrino_driver: cpufreq_driver;
    fn cpu_data(cpu: u32) -> *mut cpuinfo_x86;
    fn per_cpu_model(cpu: u32) -> *mut cpu_model;
    fn per_cpu_cpu(cpu: u32) -> *mut *const cpu_id;
    fn rdmsrq_on_cpu(cpu: u32, msr: u32, val: *mut u64) -> i32;
    fn wrmsrq_on_cpu(cpu: u32, msr: u32, val: u64) -> i32;
    fn rdmsrq(msr: u32, val: *mut u64) -> i32;
    fn wrmsrq(msr: u32, val: u64) -> i32;
    fn cpufreq_register_driver(d: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(d: *mut cpufreq_driver);
    fn x86_match_cpu(ids: *const x86_cpu_id) -> bool;
    fn cpufreq_generic_frequency_table_verify(p: *mut cpufreq_policy) -> i32;
}

#[repr(C)] pub struct cpufreq_frequency_table { pub frequency: u32, pub driver_data: u32 }
#[repr(C)] pub struct cpuinfo_x86 { pub x86_vendor: u32, pub x86: u8, pub x86_model: u8, pub x86_stepping: u8, pub x86_model_id: *const u8 }
#[repr(C)] pub struct msr { pub q: u64 }
#[repr(C)] pub struct cpufreq_policy { pub cpu: u32, pub cpus: *const core::ffi::c_void, pub shared_type: u32, pub cpuinfo: cpufreq_cpuinfo, pub freq_table: *mut cpufreq_frequency_table }
#[repr(C)] pub struct cpufreq_cpuinfo { pub transition_latency: u32 }
#[repr(C)] pub struct cpufreq_driver { pub name: *const u8, pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>, pub exit: Option<unsafe extern "C" fn(*mut cpufreq_policy)>, pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>, pub target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> i32>, pub get: Option<unsafe extern "C" fn(u32) -> u32>, pub flags: u32 }
#[repr(C)] pub struct x86_cpu_id { pub dummy: u64 }

const CPUFREQ_TABLE_END: u32 = u32::MAX;
const MSR_IA32_PERF_STATUS: u32 = 0x198;
const MSR_IA32_PERF_CTL: u32 = 0x199;
const MSR_IA32_MISC_ENABLE: u32 = 0x1a0;
const MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP: u64 = 1 << 16;

macro_rules! op { ($mhz:expr, $mv:expr) => { cpufreq_frequency_table { frequency: ($mhz) * 1000, driver_data: ((($mhz) / 100) << 8) | (($mv - 700) / 16) } }; }

static mut banias_900: [cpufreq_frequency_table; 4] = [op!(600,844),op!(800,988),op!(900,1004),cpufreq_frequency_table{frequency:CPUFREQ_TABLE_END,driver_data:0}];
static mut banias_1000: [cpufreq_frequency_table; 5] = [op!(600,844),op!(800,972),op!(900,988),op!(1000,1004),cpufreq_frequency_table{frequency:CPUFREQ_TABLE_END,driver_data:0}];
static mut banias_1100: [cpufreq_frequency_table; 6] = [op!(600,956),op!(800,1020),op!(900,1100),op!(1000,1164),op!(1100,1180),cpufreq_frequency_table{frequency:CPUFREQ_TABLE_END,driver_data:0}];
static mut banias_1200: [cpufreq_frequency_table; 7] = [op!(600,956),op!(800,1004),op!(900,1020),op!(1000,1100),op!(1100,1164),op!(1200,1180),cpufreq_frequency_table{frequency:CPUFREQ_TABLE_END,driver_data:0}];
static mut banias_1300: [cpufreq_frequency_table; 6] = [op!(600,956),op!(800,1260),op!(1000,1292),op!(1200,1356),op!(1300,1388),cpufreq_frequency_table{frequency:CPUFREQ_TABLE_END,driver_data:0}];
static mut banias_1400: [cpufreq_frequency_table; 6] = [op!(600,956),op!(800,1180),op!(1000,1308),op!(1200,1436),op!(1400,1484),cpufreq_frequency_table{frequency:CPUFREQ_TABLE_END,driver_data:0}];
static mut banias_1500: [cpufreq_frequency_table; 7] = [op!(600,956),op!(800,1116),op!(1000,1228),op!(1200,1356),op!(1400,1452),op!(1500,1484),cpufreq_frequency_table{frequency:CPUFREQ_TABLE_END,driver_data:0}];
static mut banias_1600: [cpufreq_frequency_table; 7] = [op!(600,956),op!(800,1036),op!(1000,1164),op!(1200,1276),op!(1400,1420),op!(1600,1484),cpufreq_frequency_table{frequency:CPUFREQ_TABLE_END,driver_data:0}];
static mut banias_1700: [cpufreq_frequency_table; 7] = [op!(600,956),op!(800,1004),op!(1000,1116),op!(1200,1228),op!(1400,1308),op!(1700,1484),cpufreq_frequency_table{frequency:CPUFREQ_TABLE_END,driver_data:0}];

unsafe fn centrino_cpu_init_table(policy: *mut cpufreq_policy) -> i32 {
    let cpu = cpu_data((*policy).cpu); let mut model: *mut cpu_model = core::ptr::null_mut();
    let mut i = 0; while i < 14 { let p = &mut models[i]; if !p.cpu_id.is_null() && centrino_verify_cpu_id(cpu,p.cpu_id) != 0 { model=p; break; } i+=1; }
    if model.is_null() { return -2; } if (*model).op_points.is_null() { return -2; }
    *per_cpu_model((*policy).cpu)=*model; 0
}

static mut models: [cpu_model; 15] = [
    cpu_model{cpu_id:&cpu_ids[0],model_name:b"Intel(R) Pentium(R) M processor  900\0".as_ptr(),max_freq:900000,op_points:banias_900.as_mut_ptr()},
    cpu_model{cpu_id:&cpu_ids[0],model_name:b"Intel(R) Pentium(R) M processor 1000MHz\0".as_ptr(),max_freq:1000000,op_points:banias_1000.as_mut_ptr()},
    cpu_model{cpu_id:&cpu_ids[0],model_name:b"Intel(R) Pentium(R) M processor 1100MHz\0".as_ptr(),max_freq:1100000,op_points:banias_1100.as_mut_ptr()},
    cpu_model{cpu_id:&cpu_ids[0],model_name:b"Intel(R) Pentium(R) M processor 1200MHz\0".as_ptr(),max_freq:1200000,op_points:banias_1200.as_mut_ptr()},
    cpu_model{cpu_id:&cpu_ids[0],model_name:b"Intel(R) Pentium(R) M processor 1300MHz\0".as_ptr(),max_freq:1300000,op_points:banias_1300.as_mut_ptr()},
    cpu_model{cpu_id:&cpu_ids[0],model_name:b"Intel(R) Pentium(R) M processor 1400MHz\0".as_ptr(),max_freq:1400000,op_points:banias_1400.as_mut_ptr()},
    cpu_model{cpu_id:&cpu_ids[0],model_name:b"Intel(R) Pentium(R) M processor 1500MHz\0".as_ptr(),max_freq:1500000,op_points:banias_1500.as_mut_ptr()},
    cpu_model{cpu_id:&cpu_ids[0],model_name:b"Intel(R) Pentium(R) M processor 1600MHz\0".as_ptr(),max_freq:1600000,op_points:banias_1600.as_mut_ptr()},
    cpu_model{cpu_id:&cpu_ids[0],model_name:b"Intel(R) Pentium(R) M processor 1700MHz\0".as_ptr(),max_freq:1700000,op_points:banias_1700.as_mut_ptr()},
    cpu_model{cpu_id:&cpu_ids[1],model_name:core::ptr::null(),max_freq:0,op_points:core::ptr::null_mut()}, cpu_model{cpu_id:&cpu_ids[2],model_name:core::ptr::null(),max_freq:0,op_points:core::ptr::null_mut()}, cpu_model{cpu_id:&cpu_ids[3],model_name:core::ptr::null(),max_freq:0,op_points:core::ptr::null_mut()}, cpu_model{cpu_id:&cpu_ids[4],model_name:core::ptr::null(),max_freq:0,op_points:core::ptr::null_mut()}, cpu_model{cpu_id:&cpu_ids[5],model_name:core::ptr::null(),max_freq:0,op_points:core::ptr::null_mut()}, cpu_model{cpu_id:core::ptr::null(),model_name:core::ptr::null(),max_freq:0,op_points:core::ptr::null_mut()},
];

unsafe fn extract_clock(mut msr: u32, cpu: u32, failsafe: bool) -> u32 { let id=*per_cpu_cpu(cpu); if id==&cpu_ids[0] || id==&cpu_ids[1] || id==&cpu_ids[3] { return ((msr>>8)&0xff)*100000; } let m=*per_cpu_model(cpu); if m.is_null() || (*m).op_points.is_null(){return 0;} msr&=0xffff; let mut i=0; while (*m).op_points.add(i).as_ref().unwrap().frequency!=CPUFREQ_TABLE_END { if (*m).op_points.add(i).as_ref().unwrap().driver_data==msr{return (*m).op_points.add(i).as_ref().unwrap().frequency;} i+=1;} if failsafe {(*m).op_points.add(i-1).as_ref().unwrap().frequency} else {0} }
unsafe extern "C" fn get_cur_freq(cpu:u32)->u32 { let mut v=msr{q:0}; rdmsrq_on_cpu(cpu,MSR_IA32_PERF_STATUS,&mut v.q); let mut f=extract_clock(v.q as u32,cpu,false); if f==0 {rdmsrq_on_cpu(cpu,MSR_IA32_PERF_CTL,&mut v.q);f=extract_clock(v.q as u32,cpu,true)} f }

unsafe extern "C" fn centrino_cpu_init(policy:*mut cpufreq_policy)->i32 { let c=cpu_data((*policy).cpu); if (*c).x86_vendor!=0 || centrino_cpu_init_table(policy)!=0{return -19;} (*policy).cpuinfo.transition_latency=10000; (*policy).freq_table=(*per_cpu_model((*policy).cpu)).op_points; 0 }
unsafe extern "C" fn centrino_cpu_exit(policy:*mut cpufreq_policy){*per_cpu_model((*policy).cpu)=core::mem::zeroed();}
unsafe extern "C" fn centrino_target(policy:*mut cpufreq_policy,index:u32)->i32 { let m=*per_cpu_model((*policy).cpu); if m.is_null(){return -19;} let op=(*m).op_points.add(index); let mut old=msr{q:0}; rdmsrq_on_cpu((*policy).cpu,MSR_IA32_PERF_CTL,&mut old.q); if (old.q as u32&0xffff)==(*op).driver_data{return 0;} old.q=(old.q&!0xffff)|((*op).driver_data as u64); wrmsrq_on_cpu((*policy).cpu,MSR_IA32_PERF_CTL,old.q); 0 }

static mut centrino_driver_def: cpufreq_driver = cpufreq_driver{name:b"centrino\0".as_ptr(),init:Some(centrino_cpu_init),exit:Some(centrino_cpu_exit),verify:Some(cpufreq_generic_frequency_table_verify),target_index:Some(centrino_target),get:Some(get_cur_freq),flags:0};

static centrino_ids: [x86_cpu_id; 5] = [x86_cpu_id{dummy:0};5];
unsafe extern "C" fn centrino_init()->i32 { if !x86_match_cpu(centrino_ids.as_ptr()){return -19;} cpufreq_register_driver(&mut centrino_driver_def) }
unsafe extern "C" fn centrino_exit(){cpufreq_unregister_driver(&mut centrino_driver);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
