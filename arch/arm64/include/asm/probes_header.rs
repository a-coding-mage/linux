/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm64/include/asm/probes.h
 *
 * Copyright (C) 2013 Linaro Limited
 */

// Dependency supplied by the surrounding architecture translation:
// <asm/insn.h>

pub type probes_handler_t = unsafe extern "C" fn(
    opcode: u32,
    addr: isize,
    regs: *mut pt_regs,
);

#[repr(C)]
pub struct arch_probe_insn {
    pub handler: Option<probes_handler_t>,
}

// CONFIG_KPROBES conditional preserved as a Rust feature condition.
#[cfg(feature = "CONFIG_KPROBES")]
pub type kprobe_opcode_t = __le32;

// CONFIG_KPROBES conditional preserved as a Rust feature condition.
#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct arch_specific_insn {
    pub api: arch_probe_insn,
    pub xol_insn: *mut kprobe_opcode_t,
    /* restore address after step xol */
    pub xol_restore: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
