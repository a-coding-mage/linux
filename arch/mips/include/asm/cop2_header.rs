/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2009 Wind River Systems,
 *   written by Ralf Baechle <ralf@linux-mips.org>
 */

// C dependency: <linux/notifier.h>

// These configuration branches preserve the original build-time conditions.
// The corresponding CONFIG_CPU_* configuration symbols are supplied externally.
#[cfg(CONFIG_CPU_CAVIUM_OCTEON)]
extern "C" {
    pub fn octeon_cop2_save(state: *mut octeon_cop2_state);
    pub fn octeon_cop2_restore(state: *mut octeon_cop2_state);
}

#[cfg(CONFIG_CPU_CAVIUM_OCTEON)]
#[macro_export]
macro_rules! cop2_save {
    ($r:expr) => {
        unsafe { $crate::octeon_cop2_save(&mut (*$r).thread.cp2) }
    };
}

#[cfg(CONFIG_CPU_CAVIUM_OCTEON)]
#[macro_export]
macro_rules! cop2_restore {
    ($r:expr) => {
        unsafe { $crate::octeon_cop2_restore(&mut (*$r).thread.cp2) }
    };
}

#[cfg(any(CONFIG_CPU_CAVIUM_OCTEON, CONFIG_CPU_LOONGSON64))]
pub const cop2_present: i32 = 1;
#[cfg(not(any(CONFIG_CPU_CAVIUM_OCTEON, CONFIG_CPU_LOONGSON64)))]
pub const cop2_present: i32 = 0;

#[cfg(any(CONFIG_CPU_CAVIUM_OCTEON, CONFIG_CPU_LOONGSON64))]
pub const cop2_lazy_restore: i32 = 1;
#[cfg(not(any(CONFIG_CPU_CAVIUM_OCTEON, CONFIG_CPU_LOONGSON64)))]
pub const cop2_lazy_restore: i32 = 0;

#[cfg(any(CONFIG_CPU_LOONGSON64, not(any(CONFIG_CPU_CAVIUM_OCTEON, CONFIG_CPU_LOONGSON64))))]
#[macro_export]
macro_rules! cop2_save {
    ($r:expr) => {{
        let _ = $r;
    }};
}

#[cfg(any(CONFIG_CPU_LOONGSON64, not(any(CONFIG_CPU_CAVIUM_OCTEON, CONFIG_CPU_LOONGSON64))))]
#[macro_export]
macro_rules! cop2_restore {
    ($r:expr) => {{
        let _ = $r;
    }};
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cu2_ops {
    CU2_EXCEPTION,
    CU2_LWC2_OP,
    CU2_LDC2_OP,
    CU2_SWC2_OP,
    CU2_SDC2_OP,
}

extern "C" {
    pub fn register_cu2_notifier(nb: *mut notifier_block) -> i32;
    pub fn cu2_notifier_call_chain(val: ::core::ffi::c_ulong, v: *mut ::core::ffi::c_void) -> i32;
}

#[macro_export]
macro_rules! cu2_notifier {
    ($fn_name:expr, $pri:expr) => {{
        static mut FN_NB: notifier_block = notifier_block {
            notifier_call: $fn_name,
            priority: $pri,
        };
        unsafe { $crate::register_cu2_notifier(&mut FN_NB) }
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
