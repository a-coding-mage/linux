// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2014 Imagination Technologies Ltd.
 *
 * CPU PM notifiers for saving/restoring general CPU state.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct MipsStaticSuspendState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32>,
}

extern "C" {
    fn lose_fpu(mode: i32);
    fn save_dsp(task: *mut c_void);
    fn restore_dsp(task: *mut c_void);
    fn smp_processor_id() -> u32;
    fn cpu_asid(cpu: u32, mm: *mut c_void) -> u32;
    fn write_c0_entryhi(value: u32);
    fn write_c0_userlocal(value: usize);
    fn current_thread_info() -> *mut thread_info;
    fn __restore_watch(task: *mut c_void);
    fn cpu_pm_register_notifier(nb: *mut notifier_block) -> i32;

    static mut current: *mut c_void;
    static cpu_has_userlocal: bool;
}

#[repr(C)]
struct thread_info {
    tp_value: usize,
}

#[repr(C)]
struct task_struct {
    mm: *mut c_void,
}

const CPU_PM_ENTER: usize = 0;
const CPU_PM_ENTER_FAILED: usize = 1;
const CPU_PM_EXIT: usize = 2;
const NOTIFY_STOP: i32 = 0x8002;
const NOTIFY_OK: i32 = 0x0001;

/* Used by PM helper macros in asm/pm.h */
#[no_mangle]
pub static mut mips_static_suspend_state: MipsStaticSuspendState = MipsStaticSuspendState {
    _private: [],
};

/**
 * mips_cpu_save() - Save general CPU state.
 * Ensures that general CPU context is saved, notably FPU and DSP.
 */
unsafe fn mips_cpu_save() -> i32 {
    /* Save FPU state */
    lose_fpu(1);

    /* Save DSP state */
    save_dsp(current);

    0
}

/**
 * mips_cpu_restore() - Restore general CPU state.
 * Restores important CPU context.
 */
unsafe fn mips_cpu_restore() {
    let cpu: u32 = smp_processor_id();
    let task = current as *mut task_struct;

    /* Restore ASID */
    if !(*task).mm.is_null() {
        write_c0_entryhi(cpu_asid(cpu, (*task).mm));
    }

    /* Restore DSP state */
    restore_dsp(current);

    /* Restore UserLocal */
    if cpu_has_userlocal {
        write_c0_userlocal((*current_thread_info()).tp_value);
    }

    /* Restore watch registers */
    __restore_watch(current);
}

/**
 * mips_pm_notifier() - Notifier for preserving general CPU context.
 * @self:\tNotifier block.
 * @cmd:\tCPU PM event.
 * @v:\t\tPrivate data (unused).
 *
 * This is called when a CPU power management event occurs, and is used to
 * ensure that important CPU context is preserved across a CPU power down.
 */
unsafe extern "C" fn mips_pm_notifier(
    _self: *mut notifier_block,
    cmd: usize,
    _v: *mut c_void,
) -> i32 {
    let ret: i32;

    match cmd {
        CPU_PM_ENTER => {
            ret = mips_cpu_save();
            if ret != 0 {
                return NOTIFY_STOP;
            }
        }
        CPU_PM_ENTER_FAILED | CPU_PM_EXIT => {
            mips_cpu_restore();
        }
        _ => {}
    }

    NOTIFY_OK
}

static mut mips_pm_notifier_block: notifier_block = notifier_block {
    notifier_call: Some(mips_pm_notifier),
};

unsafe fn mips_pm_init() -> i32 {
    cpu_pm_register_notifier(&mut mips_pm_notifier_block)
}

// arch_initcall(mips_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
