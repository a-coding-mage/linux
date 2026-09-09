// SPDX-License-Identifier: GPL-2.0-only
/*
 * kernel/power/suspend.c - Suspend to RAM and standby functionality.
 *
 * Direct Rust translation of the implementation source.
 */

// Linux headers and macros are supplied by the surrounding translation unit.

pub static PM_LABELS: [&'static str; PM_SUSPEND_MAX as usize] = [
    /* PM_SUSPEND_TO_IDLE */ "freeze",
    /* PM_SUSPEND_STANDBY */ "standby",
    /* PM_SUSPEND_MEM */ "mem",
];
pub static mut PM_STATES: [*const u8; PM_SUSPEND_MAX as usize] = [core::ptr::null(); PM_SUSPEND_MAX as usize];
static MEM_SLEEP_LABELS: [&'static str; PM_SUSPEND_MAX as usize] = [
    "s2idle", "shallow", "deep",
];
pub static mut MEM_SLEEP_STATES: [*const u8; PM_SUSPEND_MAX as usize] = [core::ptr::null(); PM_SUSPEND_MAX as usize];

pub static mut MEM_SLEEP_CURRENT: suspend_state_t = PM_SUSPEND_TO_IDLE;
pub static mut MEM_SLEEP_DEFAULT: suspend_state_t = PM_SUSPEND_MAX;
pub static mut PM_SUSPEND_TARGET_STATE: suspend_state_t = 0;
pub static mut PM_SUSPEND_GLOBAL_FLAGS: u32 = 0;

static mut SUSPEND_OPS: *const platform_suspend_ops = core::ptr::null();
static mut S2IDLE_OPS: *const platform_s2idle_ops = core::ptr::null();
static mut S2IDLE_WAIT_HEAD: swait_queue_head = DECLARE_SWAIT_QUEUE_HEAD!();
static mut S2IDLE_STATE: s2idle_states = S2IDLE_STATE_NONE;
static mut S2IDLE_LOCK: raw_spinlock_t = DEFINE_RAW_SPINLOCK!();

pub unsafe fn pm_suspend_default_s2idle() -> bool {
    MEM_SLEEP_CURRENT == PM_SUSPEND_TO_IDLE
}

pub unsafe fn s2idle_set_ops(ops: *const platform_s2idle_ops) {
    let sleep_flags = lock_system_sleep();
    S2IDLE_OPS = ops;
    unlock_system_sleep(sleep_flags);
}

unsafe fn s2idle_begin() { S2IDLE_STATE = S2IDLE_STATE_NONE; }

unsafe fn s2idle_enter() {
    trace_suspend_resume(TPS!("machine_suspend"), PM_SUSPEND_TO_IDLE, true);
    raw_spin_lock_irq(&mut S2IDLE_LOCK);
    if pm_wakeup_pending() { S2IDLE_STATE = S2IDLE_STATE_NONE; raw_spin_unlock_irq(&mut S2IDLE_LOCK); trace_suspend_resume(TPS!("machine_suspend"), PM_SUSPEND_TO_IDLE, false); return; }
    S2IDLE_STATE = S2IDLE_STATE_ENTER;
    raw_spin_unlock_irq(&mut S2IDLE_LOCK);
    wake_up_all_idle_cpus();
    swait_event_exclusive!(&mut S2IDLE_WAIT_HEAD, S2IDLE_STATE == S2IDLE_STATE_WAKE);
    wake_up_all_idle_cpus();
    raw_spin_lock_irq(&mut S2IDLE_LOCK);
    S2IDLE_STATE = S2IDLE_STATE_NONE;
    raw_spin_unlock_irq(&mut S2IDLE_LOCK);
    trace_suspend_resume(TPS!("machine_suspend"), PM_SUSPEND_TO_IDLE, false);
}

unsafe fn s2idle_loop() {
    pm_pr_dbg!("suspend-to-idle\n");
    loop {
        if !S2IDLE_OPS.is_null() && (*S2IDLE_OPS).wake.is_some() {
            if ((*S2IDLE_OPS).wake.unwrap())() { break; }
        } else if pm_wakeup_pending() { break; }
        if !S2IDLE_OPS.is_null() && (*S2IDLE_OPS).check.is_some() { ((*S2IDLE_OPS).check.unwrap())(); }
        s2idle_enter();
    }
    pm_pr_dbg!("resume from suspend-to-idle\n");
}

pub unsafe fn s2idle_wake() {
    let mut flags = 0ul;
    raw_spin_lock_irqsave(&mut S2IDLE_LOCK, &mut flags);
    if S2IDLE_STATE > S2IDLE_STATE_NONE { S2IDLE_STATE = S2IDLE_STATE_WAKE; swake_up_one(&mut S2IDLE_WAIT_HEAD); }
    raw_spin_unlock_irqrestore(&mut S2IDLE_LOCK, flags);
}

unsafe fn valid_state(state: suspend_state_t) -> bool {
    !SUSPEND_OPS.is_null() && (*SUSPEND_OPS).valid.is_some() && ((*SUSPEND_OPS).valid.unwrap())(state) && (*SUSPEND_OPS).enter.is_some()
}

pub unsafe fn pm_states_init() {
    PM_STATES[PM_SUSPEND_MEM as usize] = PM_LABELS[PM_SUSPEND_MEM as usize].as_ptr();
    PM_STATES[PM_SUSPEND_TO_IDLE as usize] = PM_LABELS[PM_SUSPEND_TO_IDLE as usize].as_ptr();
    MEM_SLEEP_STATES[PM_SUSPEND_TO_IDLE as usize] = MEM_SLEEP_LABELS[PM_SUSPEND_TO_IDLE as usize].as_ptr();
}

unsafe fn mem_sleep_default_setup(str_: *mut u8) -> i32 {
    let mut state = PM_SUSPEND_TO_IDLE;
    while state <= PM_SUSPEND_MEM {
        if strcmp(str_, MEM_SLEEP_LABELS[state as usize].as_ptr()) == 0 { MEM_SLEEP_DEFAULT = state; MEM_SLEEP_CURRENT = state; break; }
        state += 1;
    }
    1
}

pub unsafe fn suspend_set_ops(ops: *const platform_suspend_ops) {
    let sleep_flags = lock_system_sleep(); SUSPEND_OPS = ops;
    if valid_state(PM_SUSPEND_STANDBY) { MEM_SLEEP_STATES[PM_SUSPEND_STANDBY as usize] = MEM_SLEEP_LABELS[PM_SUSPEND_STANDBY as usize].as_ptr(); PM_STATES[PM_SUSPEND_STANDBY as usize] = PM_LABELS[PM_SUSPEND_STANDBY as usize].as_ptr(); if MEM_SLEEP_DEFAULT == PM_SUSPEND_STANDBY { MEM_SLEEP_CURRENT = PM_SUSPEND_STANDBY; } }
    if valid_state(PM_SUSPEND_MEM) { MEM_SLEEP_STATES[PM_SUSPEND_MEM as usize] = MEM_SLEEP_LABELS[PM_SUSPEND_MEM as usize].as_ptr(); if MEM_SLEEP_DEFAULT >= PM_SUSPEND_MEM { MEM_SLEEP_CURRENT = PM_SUSPEND_MEM; } }
    unlock_system_sleep(sleep_flags);
}

pub unsafe fn suspend_valid_only_mem(state: suspend_state_t) -> i32 { (state == PM_SUSPEND_MEM) as i32 }
unsafe fn sleep_state_supported(state: suspend_state_t) -> bool { state == PM_SUSPEND_TO_IDLE || (valid_state(state) && !cxl_mem_active()) }

unsafe fn platform_suspend_prepare(state: suspend_state_t) -> i32 { if state != PM_SUSPEND_TO_IDLE && (*SUSPEND_OPS).prepare.is_some() { ((*SUSPEND_OPS).prepare.unwrap())() } else { 0 } }
unsafe fn platform_suspend_prepare_late(state: suspend_state_t) -> i32 { if state == PM_SUSPEND_TO_IDLE && !S2IDLE_OPS.is_null() && (*S2IDLE_OPS).prepare.is_some() { ((*S2IDLE_OPS).prepare.unwrap())() } else { 0 } }
unsafe fn platform_suspend_prepare_noirq(state: suspend_state_t) -> i32 { if state == PM_SUSPEND_TO_IDLE { if !S2IDLE_OPS.is_null() && (*S2IDLE_OPS).prepare_late.is_some() { ((*S2IDLE_OPS).prepare_late.unwrap())() } else { 0 } } else if (*SUSPEND_OPS).prepare_late.is_some() { ((*SUSPEND_OPS).prepare_late.unwrap())() } else { 0 } }
unsafe fn platform_resume_noirq(state: suspend_state_t) { if state == PM_SUSPEND_TO_IDLE { if !S2IDLE_OPS.is_null() && (*S2IDLE_OPS).restore_early.is_some() { ((*S2IDLE_OPS).restore_early.unwrap())(); } } else if (*SUSPEND_OPS).wake.is_some() { ((*SUSPEND_OPS).wake.unwrap())(); } }
unsafe fn platform_resume_early(state: suspend_state_t) { if state == PM_SUSPEND_TO_IDLE && !S2IDLE_OPS.is_null() && (*S2IDLE_OPS).restore.is_some() { ((*S2IDLE_OPS).restore.unwrap())(); } }
unsafe fn platform_resume_finish(state: suspend_state_t) { if state != PM_SUSPEND_TO_IDLE && (*SUSPEND_OPS).finish.is_some() { ((*SUSPEND_OPS).finish.unwrap())(); } }
unsafe fn platform_suspend_begin(state: suspend_state_t) -> i32 { if state == PM_SUSPEND_TO_IDLE && !S2IDLE_OPS.is_null() && (*S2IDLE_OPS).begin.is_some() { ((*S2IDLE_OPS).begin.unwrap())() } else if !SUSPEND_OPS.is_null() && (*SUSPEND_OPS).begin.is_some() { ((*SUSPEND_OPS).begin.unwrap())(state) } else { 0 } }
unsafe fn platform_resume_end(state: suspend_state_t) { if state == PM_SUSPEND_TO_IDLE && !S2IDLE_OPS.is_null() && (*S2IDLE_OPS).end.is_some() { ((*S2IDLE_OPS).end.unwrap())(); } else if !SUSPEND_OPS.is_null() && (*SUSPEND_OPS).end.is_some() { ((*SUSPEND_OPS).end.unwrap())(); } }
unsafe fn platform_recover(state: suspend_state_t) { if state != PM_SUSPEND_TO_IDLE && (*SUSPEND_OPS).recover.is_some() { ((*SUSPEND_OPS).recover.unwrap())(); } }
unsafe fn platform_suspend_again(state: suspend_state_t) -> bool { state != PM_SUSPEND_TO_IDLE && (*SUSPEND_OPS).suspend_again.is_some() && ((*SUSPEND_OPS).suspend_again.unwrap())() }

#[cfg(feature = "CONFIG_PM_DEBUG")]
static mut PM_TEST_DELAY: u32 = 5;

unsafe fn suspend_test(level: i32) -> i32 {
    #[cfg(feature = "CONFIG_PM_DEBUG")]
    if pm_test_level == level { pr_info!("suspend debug: Waiting for %d second(s).\n", PM_TEST_DELAY); let mut i = 0; while i < PM_TEST_DELAY && !pm_wakeup_pending() { if level > TEST_CORE { msleep(1000); } else { mdelay(1000); } i += 1; } return 1; }
    0
}

unsafe fn suspend_prepare(state: suspend_state_t) -> i32 {
    if !sleep_state_supported(state) { return -EPERM; }
    pm_prepare_console();
    let error = pm_notifier_call_chain_robust(PM_SUSPEND_PREPARE, PM_POST_SUSPEND); if error != 0 { pm_restore_console(); return error; }
    filesystems_freeze(filesystem_freeze_enabled); trace_suspend_resume(TPS!("freeze_processes"), 0, true); let error = suspend_freeze_processes(); trace_suspend_resume(TPS!("freeze_processes"), 0, false); if error == 0 { return 0; }
    dpm_save_failed_step(SUSPEND_FREEZE); filesystems_thaw(); pm_notifier_call_chain(PM_POST_SUSPEND); pm_restore_console(); error
}

pub unsafe fn arch_suspend_disable_irqs() { local_irq_disable(); }
pub unsafe fn arch_suspend_enable_irqs() { local_irq_enable(); }

unsafe fn suspend_enter(state: suspend_state_t, wakeup: *mut bool) -> i32 {
    let mut error = platform_suspend_prepare(state); if error != 0 { platform_resume_finish(state); return error; }
    error = dpm_suspend_late(PMSG_SUSPEND); if error != 0 { pr_err!("late suspend of devices failed\n"); platform_resume_finish(state); return error; }
    error = platform_suspend_prepare_late(state); if error != 0 { dpm_resume_early(PMSG_RESUME); platform_resume_finish(state); return error; }
    error = dpm_suspend_noirq(PMSG_SUSPEND); if error != 0 { pr_err!("noirq suspend of devices failed\n"); platform_resume_early(state); dpm_resume_early(PMSG_RESUME); platform_resume_finish(state); return error; }
    error = platform_suspend_prepare_noirq(state); if error != 0 { platform_resume_noirq(state); dpm_resume_noirq(PMSG_RESUME); platform_resume_early(state); dpm_resume_early(PMSG_RESUME); platform_resume_finish(state); return error; }
    if suspend_test(TEST_PLATFORM) != 0 { platform_resume_noirq(state); dpm_resume_noirq(PMSG_RESUME); platform_resume_early(state); dpm_resume_early(PMSG_RESUME); platform_resume_finish(state); return error; }
    if state == PM_SUSPEND_TO_IDLE { s2idle_loop(); } else { error = pm_sleep_disable_secondary_cpus(); if error == 0 && suspend_test(TEST_CPUS) == 0 { arch_suspend_disable_irqs(); BUG_ON!(!irqs_disabled()); system_state = SYSTEM_SUSPEND; error = syscore_suspend(); if error == 0 { *wakeup = pm_wakeup_pending(); if suspend_test(TEST_CORE) == 0 && !*wakeup { trace_suspend_resume(TPS!("machine_suspend"), state, true); error = ((*SUSPEND_OPS).enter.unwrap())(state); trace_suspend_resume(TPS!("machine_suspend"), state, false); } else if *wakeup { error = -EBUSY; } syscore_resume(); } system_state = SYSTEM_RUNNING; arch_suspend_enable_irqs(); BUG_ON!(irqs_disabled()); } pm_sleep_enable_secondary_cpus(); }
    platform_resume_noirq(state); dpm_resume_noirq(PMSG_RESUME); platform_resume_early(state); dpm_resume_early(PMSG_RESUME); platform_resume_finish(state); error
}

pub unsafe fn suspend_devices_and_enter(state: suspend_state_t) -> i32 {
    if !sleep_state_supported(state) { return -ENOSYS; }
    PM_SUSPEND_TARGET_STATE = state; if state == PM_SUSPEND_TO_IDLE { pm_set_suspend_no_platform(); }
    let mut error = platform_suspend_begin(state); if error != 0 { platform_resume_end(state); PM_SUSPEND_TARGET_STATE = PM_SUSPEND_ON; return error; }
    console_suspend_all(); suspend_test_start(); error = dpm_suspend_start(PMSG_SUSPEND); if error != 0 { platform_recover(state); } else { suspend_test_finish!("suspend devices"); if suspend_test(TEST_DEVICES) == 0 { let mut wakeup = false; loop { error = suspend_enter(state, &mut wakeup); if error != 0 || wakeup || !platform_suspend_again(state) { break; } } } }
    suspend_test_start(); dpm_resume_end(PMSG_RESUME); suspend_test_finish!("resume devices"); trace_suspend_resume(TPS!("console_resume_all"), state, true); console_resume_all(); trace_suspend_resume(TPS!("console_resume_all"), state, false); platform_resume_end(state); PM_SUSPEND_TARGET_STATE = PM_SUSPEND_ON; error
}

unsafe fn suspend_finish() { suspend_thaw_processes(); filesystems_thaw(); pm_notifier_call_chain(PM_POST_SUSPEND); pm_restore_console(); }

unsafe fn enter_state(state: suspend_state_t) -> i32 {
    trace_suspend_resume(TPS!("suspend_enter"), state, true); if state == PM_SUSPEND_TO_IDLE { #[cfg(feature = "CONFIG_PM_DEBUG")] if pm_test_level != TEST_NONE && pm_test_level <= TEST_CPUS { pr_warn!("Unsupported test mode for suspend to idle, please choose none/freezer/devices/platform.\n"); return -EAGAIN; } } else if !valid_state(state) { return -EINVAL; }
    if !mutex_trylock(&mut system_transition_mutex) { return -EBUSY; } if state == PM_SUSPEND_TO_IDLE { s2idle_begin(); }
    let mut error = 0; if sync_on_suspend_enabled { trace_suspend_resume(TPS!("sync_filesystems"), 0, true); error = pm_sleep_fs_sync(); if error != 0 { mutex_unlock(&mut system_transition_mutex); return error; } trace_suspend_resume(TPS!("sync_filesystems"), 0, false); }
    pm_pr_dbg!("Preparing system for sleep (%s)\n", MEM_SLEEP_LABELS[state as usize].as_ptr()); pm_suspend_clear_flags(); error = suspend_prepare(state); if error == 0 && suspend_test(TEST_FREEZER) == 0 { trace_suspend_resume(TPS!("suspend_enter"), state, false); pm_pr_dbg!("Suspending system (%s)\n", MEM_SLEEP_LABELS[state as usize].as_ptr()); error = suspend_devices_and_enter(state); }
    events_check_enabled = false; pm_pr_dbg!("Finishing wakeup.\n"); suspend_finish(); mutex_unlock(&mut system_transition_mutex); error
}

pub unsafe fn pm_suspend(state: suspend_state_t) -> i32 {
    if state <= PM_SUSPEND_ON || state >= PM_SUSPEND_MAX { return -EINVAL; }
    pr_info!("suspend entry (%s)\n", MEM_SLEEP_LABELS[state as usize].as_ptr()); let error = enter_state(state); dpm_save_errno(error); pr_info!("suspend exit\n"); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
