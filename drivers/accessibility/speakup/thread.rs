// SPDX-License-Identifier: GPL-2.0
//
// C dependencies: <linux/kthread.h>, <linux/wait.h>, "spk_types.h",
// "speakup.h", and "spk_priv.h".

use core::ffi::c_void;

#[repr(C)]
pub struct Bleep {
    pub active: i32,
    pub freq: i32,
    pub jiffies: i32,
}

#[repr(C)]
pub struct SpinLock;

#[repr(C)]
pub struct SpeakupInfo {
    pub spinlock: SpinLock,
    pub flushing: i32,
}

#[repr(C)]
pub struct Synth {
    pub catch_up: Option<unsafe extern "C" fn(*mut Synth)>,
    pub alive: i32,
}

#[repr(C)]
pub struct Wait;

unsafe extern "C" {
    pub static mut speakup_info: SpeakupInfo;
    pub static mut spk_unprocessed_sound: Bleep;
    pub static mut spk_mutex: c_void;
    pub static mut synth: *mut Synth;

    pub fn mutex_lock(lock: *mut c_void);
    pub fn mutex_unlock(lock: *mut c_void);
    pub fn spin_lock_irqsave(lock: *mut SpinLock, flags: *mut usize);
    pub fn spin_unlock_irqrestore(lock: *mut SpinLock, flags: usize);
    pub fn prepare_to_wait(event: *mut c_void, wait: *mut Wait, state: i32);
    pub fn finish_wait(event: *mut c_void, wait: *mut Wait);
    pub fn kthread_should_stop() -> bool;
    pub fn schedule();
    pub fn kd_mksound(freq: i32, jiffies: i32);
    pub fn synth_buffer_empty() -> bool;
    pub fn speakup_start_ttys();

    pub static mut speakup_event: c_void;
}

// Linux task state constant supplied by <linux/sched.h>.
const TASK_INTERRUPTIBLE: i32 = 1;

#[no_mangle]
pub unsafe extern "C" fn speakup_thread(_data: *mut c_void) -> i32 {
    let mut flags: usize;
    let mut should_break: bool;
    let mut our_sound = Bleep {
        active: 0,
        freq: 0,
        jiffies: 0,
    };

    mutex_lock(&raw mut spk_mutex);
    loop {
        // DEFINE_WAIT(wait)
        let mut wait = Wait;

        loop {
            spin_lock_irqsave(&raw mut speakup_info.spinlock, &raw mut flags);
            our_sound = spk_unprocessed_sound;
            spk_unprocessed_sound.active = 0;
            prepare_to_wait(
                &raw mut speakup_event,
                &raw mut wait,
                TASK_INTERRUPTIBLE,
            );
            let current_synth = synth;
            should_break = kthread_should_stop()
                || our_sound.active != 0
                || (!current_synth.is_null()
                    && (*current_synth).catch_up.is_some()
                    && (*current_synth).alive != 0
                    && (speakup_info.flushing != 0 || !synth_buffer_empty()));
            spin_unlock_irqrestore(&raw mut speakup_info.spinlock, flags);
            if should_break {
                break;
            }
            mutex_unlock(&raw mut spk_mutex);
            schedule();
            mutex_lock(&raw mut spk_mutex);
        }
        finish_wait(&raw mut speakup_event, &raw mut wait);
        if kthread_should_stop() {
            break;
        }

        if our_sound.active != 0 {
            kd_mksound(our_sound.freq, our_sound.jiffies);
        }
        let current_synth = synth;
        if !current_synth.is_null()
            && (*current_synth).catch_up.is_some()
            && (*current_synth).alive != 0
        {
            // It is up to the callee to take the lock, so that it can sleep whenever it likes.
            ((*current_synth).catch_up.unwrap())(current_synth);
        }

        speakup_start_ttys();
    }
    mutex_unlock(&raw mut spk_mutex);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
