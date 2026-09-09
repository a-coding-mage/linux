/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm64/kernel/probes/decode-insn.h
 *
 * Copyright (C) 2013 Linaro Limited.
 */

/* Dependency supplied by the surrounding kernel translation. */

/*
 * ARM strongly recommends a limit of 128 bytes between LoadExcl and
 * StoreExcl instructions in a single thread of execution. So keep the
 * max atomic context size as 32.
 */
pub const MAX_ATOMIC_CONTEXT_SIZE: usize = 128 / core::mem::size_of::<kprobe_opcode_t>();

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum probe_insn {
    INSN_REJECTED,
    INSN_GOOD_NO_SLOT,
    INSN_GOOD,
}

/* The original declaration is present only when CONFIG_KPROBES is enabled. */
#[cfg(CONFIG_KPROBES)]
unsafe extern "C" {
    pub fn arm_kprobe_decode_insn(
        addr: *mut kprobe_opcode_t,
        asi: *mut arch_specific_insn,
    ) -> probe_insn;
}

unsafe extern "C" {
    pub fn arm_probe_decode_insn(insn: u32, asi: *mut arch_probe_insn) -> probe_insn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
