// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Common powerpc suspend code for 32 and 64 bits
 *
 * Copyright 2007\tJohannes Berg <johannes@sipsolutions.net>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct MmStruct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TaskStruct {
    pub active_mm: *mut MmStruct,
}

extern "C" {
    fn current() -> *mut TaskStruct;
    fn flush_all_to_thread(task: *mut TaskStruct);
    fn hard_irq_disable();
    fn switch_mmu_context(
        old: *mut MmStruct,
        new: *mut MmStruct,
        zero: *mut c_void,
    );
}

pub unsafe fn save_processor_state() {
    /*
     * flush out all the special registers so we don't need
     * to save them in the snapshot
     */
    flush_all_to_thread(current());

    // CONFIG_PPC64
    #[cfg(CONFIG_PPC64)]
    hard_irq_disable();
}

pub unsafe fn restore_processor_state() {
    // CONFIG_PPC32
    #[cfg(CONFIG_PPC32)]
    {
        let mm = (*current()).active_mm;
        switch_mmu_context(mm, mm, core::ptr::null_mut());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
