/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright 2007 Sony Corporation
 */

// C dependencies: linux/atomic.h and linux/perf_event.h.

#[cfg(feature = "CONFIG_PPC_EMULATED_STATS")]
#[repr(C)]
pub struct ppc_emulated_entry {
    pub name: *const core::ffi::c_char,
    pub val: atomic_t,
}

#[cfg(feature = "CONFIG_PPC_EMULATED_STATS")]
#[repr(C)]
pub struct ppc_emulated {
    #[cfg(feature = "CONFIG_ALTIVEC")]
    pub altivec: ppc_emulated_entry,
    pub dcba: ppc_emulated_entry,
    pub dcbz: ppc_emulated_entry,
    pub fp_pair: ppc_emulated_entry,
    pub isel: ppc_emulated_entry,
    pub mcrxr: ppc_emulated_entry,
    pub mfpvr: ppc_emulated_entry,
    pub multiple: ppc_emulated_entry,
    pub popcntb: ppc_emulated_entry,
    pub spe: ppc_emulated_entry,
    pub string: ppc_emulated_entry,
    pub sync: ppc_emulated_entry,
    pub unaligned: ppc_emulated_entry,
    #[cfg(feature = "CONFIG_MATH_EMULATION")]
    pub math: ppc_emulated_entry,
    #[cfg(feature = "CONFIG_VSX")]
    pub vsx: ppc_emulated_entry,
    #[cfg(feature = "CONFIG_PPC64")]
    pub mfdscr: ppc_emulated_entry,
    #[cfg(feature = "CONFIG_PPC64")]
    pub mtdscr: ppc_emulated_entry,
    #[cfg(feature = "CONFIG_PPC64")]
    pub lq_stq: ppc_emulated_entry,
    #[cfg(feature = "CONFIG_PPC64")]
    pub lxvw4x: ppc_emulated_entry,
    #[cfg(feature = "CONFIG_PPC64")]
    pub lxvh8x: ppc_emulated_entry,
    #[cfg(feature = "CONFIG_PPC64")]
    pub lxvd2x: ppc_emulated_entry,
    #[cfg(feature = "CONFIG_PPC64")]
    pub lxvb16x: ppc_emulated_entry,
}

#[cfg(feature = "CONFIG_PPC_EMULATED_STATS")]
extern "C" {
    pub static mut ppc_emulated: ppc_emulated;
    pub static mut ppc_warn_emulated: u32;
    pub fn ppc_warn_emulated_print(type_: *const core::ffi::c_char);
}

#[cfg(feature = "CONFIG_PPC_EMULATED_STATS")]
#[macro_export]
macro_rules! __PPC_WARN_EMULATED {
    ($type:ident) => {{
        unsafe {
            atomic_inc(&mut $crate::ppc_emulated.$type.val);
            if $crate::ppc_warn_emulated != 0 {
                $crate::ppc_warn_emulated_print($crate::ppc_emulated.$type.name);
            }
        }
    }};
}

#[cfg(not(feature = "CONFIG_PPC_EMULATED_STATS"))]
#[macro_export]
macro_rules! __PPC_WARN_EMULATED {
    ($type:ident) => {{}};
}

#[macro_export]
macro_rules! PPC_WARN_EMULATED {
    ($type:ident, $regs:expr) => {{
        perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, $regs, 0);
        $crate::__PPC_WARN_EMULATED!($type);
    }};
}

#[macro_export]
macro_rules! PPC_WARN_ALIGNMENT {
    ($type:ident, $regs:expr) => {{
        perf_sw_event(PERF_COUNT_SW_ALIGNMENT_FAULTS, 1, $regs, unsafe { (*$regs).dar });
        $crate::__PPC_WARN_EMULATED!($type);
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
