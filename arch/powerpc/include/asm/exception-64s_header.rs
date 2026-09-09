/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Extracted from head_64.S
 *
 * PowerPC low-level exception handlers and MMU support.
 * This is a Rust-level representation of the original header declarations
 * and assembler macro bodies.
 */

/* Dependency supplied by the surrounding PowerPC translation. */

/* PACA save area size in u64 units (exgen, exmc, etc) */
pub const EX_SIZE: usize = 10;

/* PACA save area offsets */
pub const EX_R9: usize = 0;
pub const EX_R10: usize = 8;
pub const EX_R11: usize = 16;
pub const EX_R12: usize = 24;
pub const EX_R13: usize = 32;
pub const EX_DAR: usize = 40;
pub const EX_DSISR: usize = 48;
pub const EX_CCR: usize = 52;
pub const EX_CFAR: usize = 56;
pub const EX_PPR: usize = 64;
pub const EX_CTR: usize = 72;

/* maximum recursive depth of MCE exceptions */
pub const MAX_MCE_DEPTH: usize = 4;

/*
 * The following assembler-only macros are retained as literal instruction
 * sequences.  Their fixup-section symbols are supplied by other headers.
 */
pub const STF_ENTRY_BARRIER_SLOT: &str =
    "STF_ENTRY_BARRIER_FIXUP_SECTION; nop; nop; nop";
pub const STF_EXIT_BARRIER_SLOT: &str =
    "STF_EXIT_BARRIER_FIXUP_SECTION; nop; nop; nop; nop; nop; nop";
pub const ENTRY_FLUSH_SLOT: &str = "ENTRY_FLUSH_FIXUP_SECTION; nop; nop; nop";
pub const SCV_ENTRY_FLUSH_SLOT: &str = "SCV_ENTRY_FLUSH_FIXUP_SECTION; nop; nop; nop";

/* r10 must be free to use, r13 must be paca */
pub const INTERRUPT_TO_KERNEL: &str =
    "STF_ENTRY_BARRIER_SLOT; ENTRY_FLUSH_SLOT";

/* r10, ctr must be free to use, r13 must be paca */
pub const SCV_INTERRUPT_TO_KERNEL: &str =
    "STF_ENTRY_BARRIER_SLOT; SCV_ENTRY_FLUSH_SLOT";

/*
 * Macros for annotating the expected destination of (h)rfid.
 * The nop instructions allow insertion of cache-flush instructions.
 */
pub const RFI_FLUSH_SLOT: &str = "RFI_FLUSH_FIXUP_SECTION; nop; nop; nop";
pub const RFI_TO_KERNEL: &str = "rfid";
pub const RFI_TO_USER: &str =
    "STF_EXIT_BARRIER_SLOT; RFI_FLUSH_SLOT; rfid; b rfi_flush_fallback";
pub const RFI_TO_USER_OR_KERNEL: &str =
    "STF_EXIT_BARRIER_SLOT; RFI_FLUSH_SLOT; rfid; b rfi_flush_fallback";
pub const RFI_TO_GUEST: &str =
    "STF_EXIT_BARRIER_SLOT; RFI_FLUSH_SLOT; rfid; b rfi_flush_fallback";
pub const HRFI_TO_KERNEL: &str = "hrfid";
pub const HRFI_TO_USER: &str =
    "STF_EXIT_BARRIER_SLOT; RFI_FLUSH_SLOT; hrfid; b hrfi_flush_fallback";
pub const HRFI_TO_USER_OR_KERNEL: &str =
    "STF_EXIT_BARRIER_SLOT; RFI_FLUSH_SLOT; hrfid; b hrfi_flush_fallback";
pub const HRFI_TO_GUEST: &str =
    "STF_EXIT_BARRIER_SLOT; RFI_FLUSH_SLOT; hrfid; b hrfi_flush_fallback";
pub const HRFI_TO_UNKNOWN: &str =
    "STF_EXIT_BARRIER_SLOT; RFI_FLUSH_SLOT; hrfid; b hrfi_flush_fallback";
pub const RFSCV_TO_USER: &str =
    "STF_EXIT_BARRIER_SLOT; RFI_FLUSH_SLOT; RFSCV; b rfscv_flush_fallback";

/* Prototype for function defined in exceptions-64s.S. */
unsafe extern "C" {
    pub fn do_uaccess_flush();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
