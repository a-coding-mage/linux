/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM is defined as `pci_controller` in the C header.
// The C header guard and tracepoint includes are intentionally omitted; their
// declarations are supplied by the surrounding translation unit.

use core::ffi::c_char;

// RATE is the local symbolic-rate table used by the trace event.
// The enum constants are supplied by the corresponding PCI headers.
#[allow(non_upper_case_globals)]
pub const RATE: &[(u32, &str)] = &[
    (PCIE_SPEED_2_5GT as u32, "2.5 GT/s"),
    (PCIE_SPEED_5_0GT as u32, "5.0 GT/s"),
    (PCIE_SPEED_8_0GT as u32, "8.0 GT/s"),
    (PCIE_SPEED_16_0GT as u32, "16.0 GT/s"),
    (PCIE_SPEED_32_0GT as u32, "32.0 GT/s"),
    (PCIE_SPEED_64_0GT as u32, "64.0 GT/s"),
    (PCI_SPEED_UNKNOWN as u32, "Unknown"),
];

// The following constants correspond to the PCI enum values imported by the
// C header.  They remain external dependencies rather than being redefined.
extern "C" {
    pub static PCIE_SPEED_2_5GT: u32;
    pub static PCIE_SPEED_5_0GT: u32;
    pub static PCIE_SPEED_8_0GT: u32;
    pub static PCIE_SPEED_16_0GT: u32;
    pub static PCIE_SPEED_32_0GT: u32;
    pub static PCIE_SPEED_64_0GT: u32;
    pub static PCI_SPEED_UNKNOWN: u32;
}

#[repr(C)]
pub struct PcieLtssmStateTransitionEntry {
    pub dev_name: *const c_char,
    pub state: *const c_char,
    pub rate: u32,
}

// Equivalent to:
// TRACE_EVENT(pcie_ltssm_state_transition,
//     TP_PROTO(const char *dev_name, const char *state, u32 rate),
//     TP_ARGS(dev_name, state, rate),
//     TP_STRUCT__entry(
//         __string(dev_name, dev_name)
//         __string(state, state)
//         __field(u32, rate)
//     ),
//     TP_fast_assign(
//         __assign_str(dev_name);
//         __assign_str(state);
//         __entry->rate = rate;
//     ),
//     TP_printk("dev: %s state: %s rate: %s",
//         __get_str(dev_name), __get_str(state),
//         __print_symbolic(__entry->rate, RATE))
// );
// The tracepoint declaration and generated registration/printing machinery are
// provided by the external tracepoint dependency.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
