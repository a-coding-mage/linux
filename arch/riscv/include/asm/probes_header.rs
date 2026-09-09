/* SPDX-License-Identifier: GPL-2.0 */

pub type probe_opcode_t = u32;
pub type probes_handler_t = unsafe extern "C" fn(
    opcode: u32,
    addr: core::ffi::c_ulong,
    regs: *mut pt_regs,
) -> bool;

/* Opaque declaration supplied by the architecture's register definitions. */
pub struct pt_regs;

#[repr(C)]
pub struct arch_probe_insn {
    pub insn: *mut probe_opcode_t,
    pub handler: Option<probes_handler_t>,
    /* restore address after simulation */
    pub restore: core::ffi::c_ulong,
}

/* Corresponds to the C CONFIG_KPROBES build-time condition. */
#[cfg(feature = "CONFIG_KPROBES")]
pub type kprobe_opcode_t = u32;

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct arch_specific_insn {
    pub api: arch_probe_insn,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
