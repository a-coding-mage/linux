// SPDX-License-Identifier: GPL-2.0

// C dependency includes and pr_fmt("x86/split lock detection: " fmt) are supplied by the kernel environment.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum SplitLockDetectState {
    SldOff = 0,
    SldWarn,
    SldFatal,
    SldRatelimit,
}

// Default to sld_off because most systems do not support split lock detection.
// sld_state_setup() will switch this to sld_warn on systems that support
// split lock/bus lock detect, unless there is a command line override.
static mut SLD_STATE: SplitLockDetectState = SplitLockDetectState::SldOff;
static mut MSR_TEST_CTRL_CACHE: u64 = 0;

// With a name like MSR_TEST_CTL it should go without saying, but don't touch
// MSR_TEST_CTL unless the CPU is one of the whitelisted models.  Writing it
// on CPUs that do not support SLD can cause fireworks, even when writing '0'.
static mut CPU_MODEL_SUPPORTS_SLD: bool = false;

#[repr(C)]
struct SldOption {
    option: *const core::ffi::c_char,
    state: SplitLockDetectState,
}

// __initconst table; string storage is supplied in the usual Rust static form.
static SLD_OPTIONS: [SldOption; 4] = [
    SldOption { option: c"off".as_ptr(), state: SplitLockDetectState::SldOff },
    SldOption { option: c"warn".as_ptr(), state: SplitLockDetectState::SldWarn },
    SldOption { option: c"fatal".as_ptr(), state: SplitLockDetectState::SldFatal },
    SldOption { option: c"ratelimit:".as_ptr(), state: SplitLockDetectState::SldRatelimit },
];

static mut BLD_RATELIMIT: ratelimit_state = unsafe { core::mem::zeroed() };
static mut SYSCTL_SLD_MITIGATE: u32 = 1;
static mut BUSLOCK_SEM: semaphore = unsafe { core::mem::zeroed() };

// CONFIG_SYSCTL conditional declarations are preserved; kernel registration is external.
#[cfg(CONFIG_SYSCTL)]
static SLD_SYSCTLS: [ctl_table; 1] = [ctl_table {
    procname: c"split_lock_mitigate".as_ptr(),
    data: unsafe { &raw mut SYSCTL_SLD_MITIGATE as *mut _ },
    maxlen: core::mem::size_of::<u32>(),
    mode: 0o644,
    proc_handler: Some(proc_douintvec_minmax),
    extra1: SYSCTL_ZERO,
    extra2: SYSCTL_ONE,
}];

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn sld_mitigate_sysctl_init() -> i32 {
    register_sysctl_init(c"kernel".as_ptr(), SLD_SYSCTLS.as_ptr());
    0
}

unsafe fn match_option(arg: *const u8, arglen: i32, opt: *const u8) -> bool {
    let len = strlen(opt) as i32;
    let mut ratelimit: i32 = 0;
    if strncmp(arg, opt, len as usize) != 0 { return false; }
    // Min ratelimit is 1 bus lock/sec. Max ratelimit is 1000 bus locks/sec.
    if sscanf_ratelimit(arg, &mut ratelimit) == 1 && ratelimit > 0 && ratelimit <= 1000 {
        ratelimit_state_init(&raw mut BLD_RATELIMIT, HZ, ratelimit);
        ratelimit_set_flags(&raw mut BLD_RATELIMIT, RATELIMIT_MSG_ON_RELEASE);
        return true;
    }
    len == arglen
}

unsafe fn split_lock_verify_msr(on: bool) -> bool {
    let mut ctrl = 0u64;
    let mut tmp = 0u64;
    if rdmsrq_safe(MSR_TEST_CTRL, &mut ctrl) != 0 { return false; }
    if on { ctrl |= MSR_TEST_CTRL_SPLIT_LOCK_DETECT; } else { ctrl &= !MSR_TEST_CTRL_SPLIT_LOCK_DETECT; }
    if wrmsrq_safe(MSR_TEST_CTRL, ctrl) != 0 { return false; }
    rdmsrq(MSR_TEST_CTRL, &mut tmp);
    ctrl == tmp
}

unsafe fn sld_state_setup() {
    let mut state = SplitLockDetectState::SldWarn;
    let mut arg = [0u8; 20];
    if !boot_cpu_has(X86_FEATURE_SPLIT_LOCK_DETECT) && !boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT) { return; }
    let ret = cmdline_find_option(boot_command_line, c"split_lock_detect".as_ptr(), arg.as_mut_ptr(), arg.len());
    if ret >= 0 {
        for option in SLD_OPTIONS.iter() {
            if match_option(arg.as_ptr(), ret, option.option as *const u8) { state = option.state; break; }
        }
    }
    SLD_STATE = state;
}

unsafe extern "C" fn setup_split_lock_detect(_arg: *mut core::ffi::c_char) -> i32 { 1 }

unsafe fn __split_lock_setup() {
    if !split_lock_verify_msr(false) { pr_info(c"MSR access failed: Disabled\n"); return; }
    rdmsrq(MSR_TEST_CTRL, &mut MSR_TEST_CTRL_CACHE);
    if !split_lock_verify_msr(true) { pr_info(c"MSR access failed: Disabled\n"); return; }
    wrmsrq(MSR_TEST_CTRL, MSR_TEST_CTRL_CACHE);
    setup_force_cpu_cap(X86_FEATURE_SPLIT_LOCK_DETECT);
}

unsafe fn sld_update_msr(on: bool) {
    let mut test_ctrl_val = MSR_TEST_CTRL_CACHE;
    if on { test_ctrl_val |= MSR_TEST_CTRL_SPLIT_LOCK_DETECT; }
    wrmsrq(MSR_TEST_CTRL, test_ctrl_val);
}

pub unsafe fn split_lock_init() {
    if SLD_STATE == SplitLockDetectState::SldRatelimit { split_lock_verify_msr(false); return; }
    if CPU_MODEL_SUPPORTS_SLD { split_lock_verify_msr(SLD_STATE != SplitLockDetectState::SldOff); }
}

unsafe fn __split_lock_reenable_unlock(_work: *mut work_struct) { sld_update_msr(true); up(&raw mut BUSLOCK_SEM); }
unsafe fn __split_lock_reenable(_work: *mut work_struct) { sld_update_msr(true); }

static mut SL_REENABLE_UNLOCK: delayed_work = unsafe { core::mem::zeroed() };
static mut SL_REENABLE: delayed_work = unsafe { core::mem::zeroed() };

unsafe fn setup_split_lock_delayed_work() -> i32 {
    for_each_possible_cpu!(cpu, {
        let work = per_cpu_ptr(&raw mut SL_REENABLE, cpu);
        INIT_DELAYED_WORK(work, __split_lock_reenable);
    });
    0
}

unsafe fn splitlock_cpu_offline(_cpu: u32) -> i32 { sld_update_msr(true); 0 }

unsafe fn split_lock_warn(ip: u64) {
    let saved_sld_mitigate = READ_ONCE(SYSCTL_SLD_MITIGATE);
    if !(*current).reported_split_lock { pr_warn_ratelimited(c"#AC: %s/%d took a split_lock trap at address: 0x%lx\n", (*current).comm.as_ptr(), (*current).pid, ip); }
    (*current).reported_split_lock = 1;
    if saved_sld_mitigate != 0 {
        if msleep_interruptible(10) > 0 { return; }
        if down_interruptible(&raw mut BUSLOCK_SEM) == -EINTR { return; }
    }
    let cpu = get_cpu();
    let work = if saved_sld_mitigate != 0 { &raw mut sl_reenable_unlock } else { per_cpu_ptr(&raw mut sl_reenable, cpu) };
    schedule_delayed_work_on(cpu, work, 2);
    sld_update_msr(false);
    put_cpu();
}

pub unsafe fn handle_guest_split_lock(ip: u64) -> bool {
    if SLD_STATE == SplitLockDetectState::SldWarn { split_lock_warn(ip); return true; }
    pr_warn_once(c"#AC: %s/%d %s split_lock trap at address: 0x%lx\n", (*current).comm.as_ptr(), (*current).pid, if SLD_STATE == SplitLockDetectState::SldFatal { c"fatal".as_ptr() } else { c"bogus".as_ptr() }, ip);
    (*current).thread.error_code = 0;
    (*current).thread.trap_nr = X86_TRAP_AC;
    force_sig_fault(SIGBUS, BUS_ADRALN, core::ptr::null_mut());
    false
}

pub unsafe fn bus_lock_init() {
    let mut val = 0u64;
    if !boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT) { return; }
    rdmsrq(MSR_IA32_DEBUGCTLMSR, &mut val);
    if (boot_cpu_has(X86_FEATURE_SPLIT_LOCK_DETECT) && (SLD_STATE == SplitLockDetectState::SldWarn || SLD_STATE == SplitLockDetectState::SldFatal)) || SLD_STATE == SplitLockDetectState::SldOff { val &= !DEBUGCTLMSR_BUS_LOCK_DETECT; } else { val |= DEBUGCTLMSR_BUS_LOCK_DETECT; }
    wrmsrq(MSR_IA32_DEBUGCTLMSR, val);
}

pub unsafe fn handle_user_split_lock(regs: *mut pt_regs, _error_code: i64) -> bool {
    if ((*regs).flags & X86_EFLAGS_AC) != 0 || SLD_STATE == SplitLockDetectState::SldFatal { return false; }
    split_lock_warn((*regs).ip); true
}

pub unsafe fn handle_bus_lock(regs: *mut pt_regs) {
    match SLD_STATE {
        SplitLockDetectState::SldOff => {},
        SplitLockDetectState::SldRatelimit => { while !__ratelimit(&raw mut BLD_RATELIMIT) { msleep(20); } pr_warn_ratelimited(c"#DB: %s/%d took a bus_lock trap at address: 0x%lx\n", (*current).comm.as_ptr(), (*current).pid, (*regs).ip); },
        SplitLockDetectState::SldWarn => pr_warn_ratelimited(c"#DB: %s/%d took a bus_lock trap at address: 0x%lx\n", (*current).comm.as_ptr(), (*current).pid, (*regs).ip),
        SplitLockDetectState::SldFatal => force_sig_fault(SIGBUS, BUS_ADRALN, core::ptr::null_mut()),
    }
}

unsafe fn split_lock_setup(c: *mut cpuinfo_x86) {
    if boot_cpu_has(X86_FEATURE_HYPERVISOR) { return; }
    let m = x86_match_cpu(split_lock_cpu_ids);
    if !m.is_null() { CPU_MODEL_SUPPORTS_SLD = true; __split_lock_setup(); return; }
    if !cpu_has(c, X86_FEATURE_CORE_CAPABILITIES) { return; }
    let mut ia32_core_caps = 0u64;
    rdmsrq(MSR_IA32_CORE_CAPS, &mut ia32_core_caps);
    if ia32_core_caps & MSR_IA32_CORE_CAPS_SPLIT_LOCK_DETECT != 0 {
        CPU_MODEL_SUPPORTS_SLD = true;
        __split_lock_setup();
    }
}

unsafe fn sld_state_show() {
    let mut action = c"warning".as_ptr();
    if (!boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT) && !boot_cpu_has(X86_FEATURE_SPLIT_LOCK_DETECT)) || SLD_STATE == SplitLockDetectState::SldOff { return; }
    if SLD_STATE == SplitLockDetectState::SldRatelimit {
        if boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT) { pr_info(c"#DB: setting system wide bus lock rate limit to %u/sec\n", BLD_RATELIMIT.burst); }
        return;
    } else if SLD_STATE == SplitLockDetectState::SldFatal { action = c"sending SIGBUS".as_ptr(); }
    if boot_cpu_has(X86_FEATURE_SPLIT_LOCK_DETECT) {
        pr_info(c"#AC: crashing the kernel on kernel split_locks and %s on user-space split_locks\n", action);
        if cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, c"x86/splitlock".as_ptr(), core::ptr::null_mut(), splitlock_cpu_offline) < 0 { pr_warn(c"No splitlock CPU offline handler\n"); }
    } else if boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT) { pr_info(c"#DB: %s on user-space bus_locks\n", action); }
}

pub unsafe fn sld_setup(c: *mut cpuinfo_x86) {
    split_lock_setup(c);
    sld_state_setup();
    sld_state_show();
}

// External kernel declarations, CPU tables, per-CPU delayed-work initialization,
// and registration macros remain supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
