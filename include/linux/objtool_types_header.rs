/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This struct is used by asm and inline asm code to manually annotate the
 * location of registers on the stack.
 *
 * The C header excludes this declaration when __ASSEMBLER__ is defined.
 */
#[repr(C)]
pub struct unwind_hint {
    pub ip: u32,
    pub sp_offset: i16,
    pub sp_reg: u8,
    pub type_: u8,
    pub signal: u8,
}

/*
 * UNWIND_HINT_TYPE_UNDEFINED: A blind spot in ORC coverage which can result in
 * a truncated and unreliable stack unwind.
 *
 * UNWIND_HINT_TYPE_END_OF_STACK: The end of the kernel stack unwind before
 * hitting user entry, boot code, or fork entry (when there are no pt_regs
 * available).
 *
 * UNWIND_HINT_TYPE_CALL: Indicates that sp_reg+sp_offset resolves to PREV_SP
 * (the caller's SP right before it made the call).  Used for all callable
 * functions, i.e. all C code and all callable asm functions.
 *
 * UNWIND_HINT_TYPE_REGS: Used in entry code to indicate that sp_reg+sp_offset
 * points to a fully populated pt_regs from a syscall, interrupt, or exception.
 *
 * UNWIND_HINT_TYPE_REGS_PARTIAL: Used in entry code to indicate that
 * sp_reg+sp_offset points to the iret return frame.
 *
 * UNWIND_HINT_TYPE_FUNC: Generate the unwind metadata of a callable function.
 * Useful for code which doesn't have an ELF function annotation.
 *
 * UNWIND_HINT_TYPE_{SAVE,RESTORE}: Save the unwind metadata at a certain
 * location so that it can be restored later.
 */
pub const UNWIND_HINT_TYPE_UNDEFINED: u32 = 0;
pub const UNWIND_HINT_TYPE_END_OF_STACK: u32 = 1;
pub const UNWIND_HINT_TYPE_CALL: u32 = 2;
pub const UNWIND_HINT_TYPE_REGS: u32 = 3;
pub const UNWIND_HINT_TYPE_REGS_PARTIAL: u32 = 4;
/* The below hint types don't have corresponding ORC types */
pub const UNWIND_HINT_TYPE_FUNC: u32 = 5;
pub const UNWIND_HINT_TYPE_SAVE: u32 = 6;
pub const UNWIND_HINT_TYPE_RESTORE: u32 = 7;

/*
 * Annotate types
 */
pub const ANNOTYPE_NOENDBR: u32 = 1;
pub const ANNOTYPE_RETPOLINE_SAFE: u32 = 2;
pub const ANNOTYPE_INSTR_BEGIN: u32 = 3;
pub const ANNOTYPE_INSTR_END: u32 = 4;
pub const ANNOTYPE_UNRET_BEGIN: u32 = 5;
pub const ANNOTYPE_IGNORE_ALTS: u32 = 6;
pub const ANNOTYPE_INTRA_FUNCTION_CALL: u32 = 7;
pub const ANNOTYPE_REACHABLE: u32 = 8;
pub const ANNOTYPE_NOCFI: u32 = 9;

pub const ANNOTYPE_DATA_SPECIAL: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
