/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/notifier.h, linux/types.h, asm/break.h, and asm/inst.h.

/*
 * We want this to be defined as union mips_instruction but that makes the
 * generic code blow up.
 */
pub type uprobe_opcode_t = u32;

/*
 * Classic MIPS (note this implementation doesn't consider microMIPS yet)
 * instructions are always 4 bytes but in order to deal with branches and
 * their delay slots, we treat instructions as having 8 bytes maximum.
 */
pub const MAX_UINSN_BYTES: u32 = 8;
pub const UPROBE_XOL_SLOT_BYTES: u32 = 128; /* Max. cache line size */

pub const UPROBE_BRK_UPROBE: u32 = 0x000d000d; /* break 13 */
pub const UPROBE_BRK_UPROBE_XOL: u32 = 0x000e000d; /* break 14 */

pub const UPROBE_SWBP_INSN: u32 = UPROBE_BRK_UPROBE;
pub const UPROBE_SWBP_INSN_SIZE: u32 = 4;

#[repr(C)]
pub struct arch_uprobe {
    pub resume_epc: ::core::ffi::c_ulong,
    pub insn: [u32; 2],
    pub ixol: [u32; 2],
}

#[repr(C)]
pub struct arch_uprobe_task {
    pub saved_trap_nr: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
