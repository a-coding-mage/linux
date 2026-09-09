/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the SPARC assembly header.  The macro values retain the
// original instruction sequences for use by the assembly-generating code.

pub const KERNBASE: u32 = 0xf000_0000; // First address the kernel will eventually be

pub const WRITE_PAUSE: &str = "nop; nop; nop;"; // Have to do this after %wim/%psr chg

// Here are some trap goodies

// Generic trap entry.
#[macro_export]
macro_rules! TRAP_ENTRY {
    ($type:expr, $label:ident) => {
        concat!("rd %psr, %l0; b ", stringify!($label), "; rd %wim, %l3; nop;")
    };
}

// Data/text faults
pub const SRMMU_TFAULT: &str = "rd %psr, %l0; rd %wim, %l3; b srmmu_fault; mov 1, %l7;";
pub const SRMMU_DFAULT: &str = "rd %psr, %l0; rd %wim, %l3; b srmmu_fault; mov 0, %l7;";

// This is for traps we should NEVER get.
#[macro_export]
macro_rules! BAD_TRAP {
    ($num:expr) => {
        concat!("rd %psr, %l0; mov ", stringify!($num), "; b bad_trap_handler; rd %wim, %l3;")
    };
}

// This is for traps when we want just skip the instruction which caused it
#[macro_export]
macro_rules! SKIP_TRAP {
    ($type:expr, $name:ident) => { "jmpl %l2, %g0; rett %l2 + 4; nop; nop;" };
}

// Software trap for Linux system calls.
pub const LINUX_SYSCALL_TRAP: &str =
    "sethi %hi(sys_call_table), %l7; or %l7, %lo(sys_call_table), %l7; b linux_sparc_syscall; rd %psr, %l0;";

pub const BREAKPOINT_TRAP: &str = "b breakpoint_trap; rd %psr,%l0; nop; nop;";

// CONFIG_KGDB selects the low-level KGDB trap; otherwise this is a bad trap.
#[cfg(feature = "CONFIG_KGDB")]
#[macro_export]
macro_rules! KGDB_TRAP {
    ($num:expr) => {
        concat!("mov ", stringify!($num), ", %l7; b kgdb_trap_low; rd %psr,%l0; nop;")
    };
}

#[cfg(not(feature = "CONFIG_KGDB"))]
#[macro_export]
macro_rules! KGDB_TRAP {
    ($num:expr) => { $crate::BAD_TRAP!($num) };
}

// The Get Condition Codes software trap for userland.
pub const GETCC_TRAP: &str = "b getcc_trap_handler; rd %psr, %l0; nop; nop;";

// The Set Condition Codes software trap for userland.
pub const SETCC_TRAP: &str = "b setcc_trap_handler; rd %psr, %l0; nop; nop;";

// The Get PSR software trap for userland.
pub const GETPSR_TRAP: &str = "rd %psr, %i0; jmp %l2; rett %l2 + 4; nop;";

// Hard interrupts from level 1-14; 15 is non-maskable (nmi).
#[macro_export]
macro_rules! TRAP_ENTRY_INTERRUPT {
    ($int_level:expr) => {
        concat!("mov ", stringify!($int_level), ", %l7; rd %psr, %l0; b real_irq_entry; rd %wim, %l3;")
    };
}

// Window overflows/underflows are special.
pub const WINDOW_SPILL: &str =
    "rd %psr, %l0; rd %wim, %l3; b spill_window_entry; andcc %l0, PSR_PS, %g0;";
pub const WINDOW_FILL: &str =
    "rd %psr, %l0; rd %wim, %l3; b fill_window_entry; andcc %l0, PSR_PS, %g0;";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
