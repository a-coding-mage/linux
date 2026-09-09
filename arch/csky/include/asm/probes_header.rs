/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard and include dependencies are omitted; `u32` and `pt_regs`
// are supplied by the surrounding translation unit.

pub type probe_opcode_t = u32;
pub type probes_handler_t = unsafe extern "C" fn(
    opcode: u32,
    addr: core::ffi::c_long,
    regs: *mut pt_regs,
);

/// Architecture-specific copy of the original instruction.
#[repr(C)]
pub struct arch_probe_insn {
    pub insn: *mut probe_opcode_t,
    pub handler: *mut probes_handler_t,
    /// Restore address after simulation.
    pub restore: core::ffi::c_ulong,
}

// Preserved from the C build-time condition: compile the following declarations
// when CONFIG_KPROBES is enabled in the surrounding build.
#[cfg(CONFIG_KPROBES)]
pub type kprobe_opcode_t = u32;

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct arch_specific_insn {
    pub api: arch_probe_insn,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
