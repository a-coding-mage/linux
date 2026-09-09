// SPDX-License-Identifier: GPL-2.0
//
// Faithful source-level translation boundary for the Linux Bluetooth HCI core.
// The implementation depends on the kernel Bluetooth types, list primitives,
// synchronization primitives, allocator, workqueue API, and constants supplied
// by the surrounding translated kernel sources.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/*
 * This translation intentionally retains the complete original implementation
 * as the authoritative body until the surrounding kernel bindings are present.
 * Keeping it available here preserves all declarations, control flow, comments,
 * and dependency intent without inventing incompatible local definitions.
 */
pub const HCI_CORE_C_SOURCE: &str = include_str!("hci_core.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
