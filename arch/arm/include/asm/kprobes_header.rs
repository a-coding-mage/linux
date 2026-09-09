/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/kprobes.h
 *
 * Copyright (C) 2006, 2007 Motorola Inc.
 */

// #include <asm-generic/kprobes.h>

// The following declarations are enabled when CONFIG_KPROBES is defined in
// the C build. Rust feature configuration should provide the equivalent gate.
#[cfg(feature = "CONFIG_KPROBES")]
pub mod config_kprobes {
    // #include <linux/types.h>
    // #include <linux/ptrace.h>
    // #include <linux/notifier.h>

    pub const MAX_INSN_SIZE: usize = 2;

    #[inline]
    pub unsafe fn flush_insn_slot<T>(_p: *mut T) {}

    pub const kretprobe_blacklist_size: usize = 0;

    pub type kprobe_opcode_t = u32;

    // Forward declaration supplied by the surrounding kprobes implementation.
    #[repr(C)]
    pub struct kprobe {
        _private: [u8; 0],
    }

    // #include <asm/probes.h>
    // C alias: #define arch_specific_insn arch_probes_insn
    pub type arch_specific_insn = arch_probes_insn;
    // The definition of arch_probes_insn is supplied by asm/probes.h.
    pub type arch_probes_insn = core::ffi::c_void;

    #[repr(C)]
    pub struct prev_kprobe {
        pub kp: *mut kprobe,
        pub status: u32,
    }

    /* per-cpu kprobe control block */
    #[repr(C)]
    pub struct kprobe_ctlblk {
        pub kprobe_status: u32,
        pub prev_kprobe: prev_kprobe,
    }

    unsafe extern "C" {
        pub fn arch_remove_kprobe(kp: *mut kprobe);
        pub fn kprobe_fault_handler(regs: *mut pt_regs, fsr: u32) -> i32;

        /* optinsn template addresses */
        pub static optprobe_template_entry: [kprobe_opcode_t; 0];
        pub static optprobe_template_val: [kprobe_opcode_t; 0];
        pub static optprobe_template_call: [kprobe_opcode_t; 0];
        pub static optprobe_template_end: [kprobe_opcode_t; 0];
        pub static optprobe_template_sub_sp: [kprobe_opcode_t; 0];
        pub static optprobe_template_add_sp: [kprobe_opcode_t; 0];
        pub static optprobe_template_restore_begin: [kprobe_opcode_t; 0];
        pub static optprobe_template_restore_orig_insn: [kprobe_opcode_t; 0];
        pub static optprobe_template_restore_end: [kprobe_opcode_t; 0];
    }

    // Forward declaration supplied by linux/ptrace.h.
    #[repr(C)]
    pub struct pt_regs {
        _private: [u8; 0],
    }

    pub const MAX_OPTIMIZED_LENGTH: usize = 4;

    #[inline]
    pub unsafe fn max_optinsn_size() -> usize {
        (core::ptr::addr_of!(optprobe_template_end) as usize)
            .wrapping_sub(core::ptr::addr_of!(optprobe_template_entry) as usize)
    }

    pub const RELATIVEJUMP_SIZE: usize = 4;

    pub const MAX_COPIED_INSN: usize =
        (RELATIVEJUMP_SIZE + core::mem::size_of::<kprobe_opcode_t>() - 1)
            / core::mem::size_of::<kprobe_opcode_t>();

    #[repr(C)]
    pub struct arch_optimized_insn {
        /* copy of the original instructions.
         * Different from x86, ARM kprobe_opcode_t is u32. */
        pub copied_insn: [kprobe_opcode_t; MAX_COPIED_INSN],
        /* detour code buffer */
        pub insn: *mut kprobe_opcode_t,
        /*
         * We always copy one instruction on ARM,
         * so size will always be 4, and unlike x86, there is no
         * need for a size field.
         */
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
