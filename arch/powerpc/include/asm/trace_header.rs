/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the Linux powerpc tracepoint header.
// The C tracepoint-generation macros have no direct Rust equivalent; the
// event payload layouts and externally visible declarations are preserved.

use core::ffi::{c_char, c_int, c_long, c_ulong};

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rtas_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Ppc64InterruptEvent {
    pub regs: *mut pt_regs,
}

pub const PPC64_INTERRUPT_EVENTS: &[&str] = &[
    "irq_entry",
    "irq_exit",
    "timer_interrupt_entry",
    "timer_interrupt_exit",
];

#[cfg(feature = "CONFIG_PPC_DOORBELL")]
pub const PPC64_DOORBELL_EVENTS: &[&str] = &["doorbell_entry", "doorbell_exit"];

#[cfg(feature = "CONFIG_PPC_PSERIES")]
extern "C" {
    pub fn hcall_tracepoint_regfunc() -> c_int;
    pub fn hcall_tracepoint_unregfunc();
}

#[cfg(feature = "CONFIG_PPC_PSERIES")]
#[repr(C)]
pub struct HcallEntryEvent {
    pub opcode: c_ulong,
}

#[cfg(feature = "CONFIG_PPC_PSERIES")]
#[repr(C)]
pub struct HcallExitEvent {
    pub opcode: c_ulong,
    pub retval: c_long,
}

#[cfg(feature = "CONFIG_PPC_PSERIES")]
pub const HCALL_TRACE_CONDITION: &str = "cpu_online(raw_smp_processor_id())";

#[cfg(feature = "CONFIG_PPC_RTAS")]
#[repr(C)]
pub struct RtasInputEvent {
    pub nargs: u32,
    // C __string(name, name) and __dynamic_array(__u32, inputs, nargs)
    // are variable-sized trace record fields.
}

#[cfg(feature = "CONFIG_PPC_RTAS")]
#[repr(C)]
pub struct RtasOutputEvent {
    pub nr_other: u32,
    pub status: i32,
    // C __string(name, name) and dynamic other_outputs follow the fixed fields.
}

#[cfg(feature = "CONFIG_PPC_RTAS")]
#[repr(C)]
pub struct RtasParameterBlockEvent {
    pub token: u32,
    pub nargs: u32,
    pub nret: u32,
    pub params: [u32; 16],
}

#[cfg(feature = "CONFIG_PPC_RTAS")]
pub const RTAS_PARAMETER_BLOCK_EVENTS: &[&str] = &["rtas_ll_entry", "rtas_ll_exit"];

#[cfg(feature = "CONFIG_PPC_POWERNV")]
extern "C" {
    pub fn opal_tracepoint_regfunc() -> c_int;
    pub fn opal_tracepoint_unregfunc();
}

#[cfg(feature = "CONFIG_PPC_POWERNV")]
#[repr(C)]
pub struct OpalEntryEvent {
    pub opcode: c_ulong,
}

#[cfg(feature = "CONFIG_PPC_POWERNV")]
#[repr(C)]
pub struct OpalExitEvent {
    pub opcode: c_ulong,
    pub retval: c_ulong,
}

#[cfg(feature = "CONFIG_PPC_64S_HASH_MMU")]
#[repr(C)]
pub struct HashFaultEvent {
    pub addr: c_ulong,
    pub access: c_ulong,
    pub trap: c_ulong,
}

#[repr(C)]
pub struct TlbieEvent {
    pub lpid: c_ulong,
    pub local: c_ulong,
    pub rb: c_ulong,
    pub rs: c_ulong,
    pub ric: c_ulong,
    pub prs: c_ulong,
    pub r: c_ulong,
}

#[repr(C)]
pub struct TlbiaEvent {
    pub id: c_ulong,
}

// Original trace formatting:
// irq/doorbell/timer: pt_regs=%p
// hcall_entry: opcode=%lu
// hcall_exit: opcode=%lu retval=%ld
// rtas_input: %s arguments: %s
// rtas_output: %s status: %d, other outputs: %s
// rtas_parameter_block: token=%u nargs=%u nret=%u params: [0..15]
// opal_entry: opcode=%lu
// opal_exit: opcode=%lu retval=%lu
// hash_fault: hash fault with addr 0x%lx and access = 0x%lx trap = 0x%lx
// tlbie: lpid=%ld, local=%ld, rb=0x%lx, rs=0x%lx, ric=0x%lx, prs=0x%lx, r=0x%lx
// tlbia: ctx.id=0x%lx

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
