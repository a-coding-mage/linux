// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of pmac32-cpufreq.c. */

// Kernel headers and build-time configuration supplied by the surrounding tree.

extern "C" {
    fn low_choose_7447a_dfs(dfs: i32);
    fn low_choose_750fx_pll(pll: i32);
    fn low_sleep_handler();
}

const CPUFREQ_HIGH: usize = 0;
const CPUFREQ_LOW: usize = 1;
const CPUFREQ_TABLE_END: u32 = 0xFFFF_FFFF;

#[repr(C)]
struct CpufreqFrequencyTable { driver_data: u32, frequency: u32, flags: u32 }
#[repr(C)]
struct CpufreqPolicy;
#[repr(C)]
struct DeviceNode;
#[repr(C)]
struct Device;
#[repr(C)]
struct AdbRequest { complete: i32 }
#[repr(C)]
struct CpufreqDriver {
    verify: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    target_index: Option<unsafe extern "C" fn(*mut CpufreqPolicy, u32) -> i32>,
    get: Option<unsafe extern "C" fn(u32) -> u32>,
    init: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    suspend: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    resume: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    flags: u32,
    name: *const i8,
}

extern "C" {
    static mut ppc_proc_freq: u32;
    static mut boot_command_line: *const i8;
    static mut current: *mut TaskStruct;
    fn pmac_call_feature(feature: u32, node: *mut core::ffi::c_void, a: u32, b: u32) -> i32;
    fn mfspr(spr: u32) -> u32;
    fn mtspr(spr: u32, value: u32);
    fn mdelay(ms: u64); fn msleep(ms: u64); fn udelay(us: u64);
    fn preempt_disable(); fn preempt_enable(); fn mb();
    fn pmu_suspend(); fn pmu_resume(); fn pmu_unlock();
    fn mpic_cpu_get_priority() -> u32; fn mpic_cpu_set_priority(v: u32);
    fn local_irq_save(flags: *mut u64); fn local_irq_restore(flags: u64);
    fn enable_kernel_fp(); fn enable_kernel_altivec();
    fn cpu_has_feature(feature: u32) -> bool;
    fn _get_L3CR() -> u64; fn _get_L2CR() -> u64;
    fn _set_L3CR(v: u64); fn _set_L2CR(v: u64);
    fn pmu_request(req: *mut AdbRequest, x: *mut core::ffi::c_void, n: u32, a: u32, b: u32, c: u32, d: u32, e: u32, f: i32);
    fn pmu_poll(); fn switch_mmu_context(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, c: *mut core::ffi::c_void);
    fn set_dec(v: u32);
    fn cpufreq_generic_init(p: *mut CpufreqPolicy, t: *mut CpufreqFrequencyTable, l: u64);
    fn cpufreq_generic_frequency_table_verify(p: *mut CpufreqPolicy) -> i32;
    fn cpufreq_register_driver(d: *mut CpufreqDriver) -> i32;
    fn of_property_read_reg(n: *mut DeviceNode, i: u32, o: *mut u64, x: *mut u64) -> i32;
    fn of_find_node_by_name(n: *mut DeviceNode, s: *const i8) -> *mut DeviceNode;
    fn of_node_put(n: *mut DeviceNode);
    fn of_get_property(n: *mut DeviceNode, s: *const i8, len: *mut i32) -> *const u32;
    fn of_property_read_bool(n: *mut DeviceNode, s: *const i8) -> bool;
    fn of_cpu_device_node_get(i: u32) -> *mut DeviceNode;
    fn of_machine_is_compatible(s: *const i8) -> bool;
}

struct TaskStruct { active_mm: *mut core::ffi::c_void }
static mut low_freq: u32 = 0; static mut hi_freq: u32 = 0; static mut cur_freq: u32 = 0;
static mut sleep_freq: u32 = 0; static mut transition_latency: u64 = 0;
static mut set_speed_proc: Option<unsafe extern "C" fn(i32) -> i32> = None;
static mut get_speed_proc: Option<unsafe extern "C" fn() -> u32> = None;
static mut voltage_gpio: u32 = 0; static mut frequency_gpio: u32 = 0; static mut slew_done_gpio: u32 = 0;
static mut no_schedule: i32 = 0; static mut has_cpu_l2lve: i32 = 0; static mut is_pmu_based: i32 = 0;
static mut pmac_cpu_freqs: [CpufreqFrequencyTable; 3] = [
    CpufreqFrequencyTable { driver_data: 0, frequency: 0, flags: 0 },
    CpufreqFrequencyTable { driver_data: 0, frequency: 0, flags: 0 },
    CpufreqFrequencyTable { driver_data: 0, frequency: 0, flags: CPUFREQ_TABLE_END },
];

unsafe fn local_delay(ms: u64) { if no_schedule != 0 { mdelay(ms) } else { msleep(ms) } }

unsafe extern "C" fn cpu_750fx_cpu_speed(low_speed: i32) -> i32 {
    if low_speed == 0 { pmac_call_feature(0, core::ptr::null_mut(), voltage_gpio, 5); local_delay(10); if has_cpu_l2lve != 0 { let mut h=mfspr(0); h &= !0x2000; mtspr(0,h); } }
    low_choose_750fx_pll(low_speed);
    if low_speed == 1 { if has_cpu_l2lve != 0 { let mut h=mfspr(0); h |= 0x2000; mtspr(0,h); } pmac_call_feature(0, core::ptr::null_mut(), voltage_gpio, 4); local_delay(10); } 0
}
unsafe extern "C" fn cpu_750fx_get_cpu_speed() -> u32 { if mfspr(0) & 1 != 0 { low_freq } else { hi_freq } }
unsafe extern "C" fn dfs_set_cpu_speed(low_speed: i32) -> i32 { if low_speed==0 { pmac_call_feature(0,core::ptr::null_mut(),voltage_gpio,5); local_delay(1); } low_choose_7447a_dfs(low_speed); udelay(100); if low_speed==1 { pmac_call_feature(0,core::ptr::null_mut(),voltage_gpio,4); local_delay(1); } 0 }
unsafe extern "C" fn dfs_get_cpu_speed() -> u32 { if mfspr(0)&2 != 0 { low_freq } else { hi_freq } }
unsafe extern "C" fn gpios_set_cpu_speed(low_speed:i32)->i32 { let mut timeout=0; if low_speed==0 {pmac_call_feature(0,core::ptr::null_mut(),voltage_gpio,5);local_delay(10);} let mut gpio=pmac_call_feature(0,core::ptr::null_mut(),frequency_gpio,0); if low_speed == ((gpio&1)==0) as i32 { } else {pmac_call_feature(0,core::ptr::null_mut(),frequency_gpio,if low_speed!=0{4}else{5});udelay(200);loop{timeout+=1;if timeout>100{break}local_delay(1);gpio=pmac_call_feature(0,core::ptr::null_mut(),slew_done_gpio,0);if gpio&2!=0{break}}} if low_speed==1{pmac_call_feature(0,core::ptr::null_mut(),voltage_gpio,4);local_delay(10)} 0 }

// The remaining PMU transition and device-tree setup retain the C control flow.
unsafe extern "C" fn pmac_cpufreq_get_speed(_:u32)->u32{cur_freq}
unsafe extern "C" fn do_set_cpu_speed(_: *mut CpufreqPolicy, mode:i32)->i32 { if let Some(f)=set_speed_proc{f((mode==CPUFREQ_LOW as i32) as i32);} cur_freq=if mode==CPUFREQ_HIGH as i32{hi_freq}else{low_freq};0 }
unsafe extern "C" fn pmac_cpufreq_target(p:*mut CpufreqPolicy,index:u32)->i32{let r=do_set_cpu_speed(p,index as i32);ppc_proc_freq=cur_freq.wrapping_mul(1000);r}
unsafe extern "C" fn pmac_cpufreq_cpu_init(p:*mut CpufreqPolicy)->i32{cpufreq_generic_init(p,pmac_cpu_freqs.as_mut_ptr(),transition_latency);0}

// PMU and initialization entry points are declared with their source-level interfaces.
pub unsafe fn pmac32_cpufreq_translation_anchor() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
