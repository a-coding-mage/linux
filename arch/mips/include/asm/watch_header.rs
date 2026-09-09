/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 David Daney
 */

// Dependency intent from <linux/bitops.h> and <asm/mipsregs.h> is preserved
// through the external symbols used below.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpuinfo_mips {
    _private: [u8; 0],
}

extern "C" {
    pub fn mips_install_watch_registers(t: *mut task_struct);
    pub fn mips_read_watch_registers();
    pub fn mips_clear_watch_registers();
    pub fn mips_probe_watch_registers(c: *mut cpuinfo_mips);
}

// Build-time condition preserved from CONFIG_HARDWARE_WATCHPOINTS.
#[cfg(feature = "CONFIG_HARDWARE_WATCHPOINTS")]
#[macro_export]
macro_rules! __restore_watch {
    ($task:expr) => {
        if unlikely(test_bit(
            TIF_LOAD_WATCH,
            &task_thread_info($task).flags,
        )) {
            unsafe {
                mips_install_watch_registers($task);
            }
        }
    };
}

#[cfg(not(feature = "CONFIG_HARDWARE_WATCHPOINTS"))]
#[macro_export]
macro_rules! __restore_watch {
    ($task:expr) => {{}};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
