/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 2010 Cavium Networks, Inc.
 */

/* Dependencies supplied by the surrounding kernel translation. */
use crate::asm::cacheflush::flush_icache_range;
use crate::asm::inst::{bc6_op, j_op, mm_j32_op, mips_instruction, msk_isa16_mode};
use crate::linux::cpu::*;
use crate::linux::jump_label::*;
use crate::linux::kernel::*;
use crate::linux::memory::*;
use crate::linux::mutex::{mutex_lock, mutex_unlock, text_mutex};
use crate::linux::types::*;

/*
 * Define parameters for the standard MIPS and the microMIPS jump
 * instruction encoding respectively:
 *
 * - the ISA bit of the target, either 0 or 1 respectively,
 *
 * - the amount the jump target address is shifted right to fit in the
 *   immediate field of the machine instruction, either 2 or 1,
 *
 * - the mask determining the size of the jump region relative to the
 *   delay-slot instruction, either 256MB or 128MB,
 *
 * - the jump target alignment, either 4 or 2 bytes.
 */
const J_ISA_BIT: usize = if cfg!(feature = "cpu_micromips") { 1 } else { 0 };
const J_RANGE_SHIFT: usize = 2 - J_ISA_BIT;
const J_RANGE_MASK: usize = (1usize << (26 + J_RANGE_SHIFT)) - 1;
const J_ALIGN_MASK: usize = (1usize << J_RANGE_SHIFT) - 1;

pub unsafe fn arch_jump_label_transform(
    e: *mut jump_entry,
    type_: jump_label_type,
) {
    let mut insn_p: *mut mips_instruction;
    let mut insn: mips_instruction = core::mem::zeroed();
    let mut offset: isize;

    insn_p = msk_isa16_mode((*e).code as usize) as *mut mips_instruction;

    /* Target must have the right alignment and ISA must be preserved. */
    BUG_ON(((*e).target as usize & J_ALIGN_MASK) != J_ISA_BIT);

    if type_ == JUMP_LABEL_JMP {
        if !cfg!(feature = "cpu_micromips") && MIPS_ISA_REV >= 6 {
            offset = ((*e).target as isize)
                .wrapping_sub((insn_p as usize as isize).wrapping_add(4));
            offset >>= 2;

            /*
             * The branch offset must fit in the instruction's 26
             * bit field.
             */
            WARN_ON(offset >= (1isize << 25) || offset < -(1isize << 25));

            insn.j_format.opcode = bc6_op;
            insn.j_format.target = offset as _;
        } else {
            /*
             * Jump only works within an aligned region its delay
             * slot is in.
             */
            WARN_ON(((*e).target as usize & !J_RANGE_MASK)
                != (((*e).code as usize + 4) & !J_RANGE_MASK));

            insn.j_format.opcode = if J_ISA_BIT != 0 { mm_j32_op } else { j_op };
            insn.j_format.target = ((*e).target as usize >> J_RANGE_SHIFT) as _;
        }
    } else {
        insn.word = 0; /* nop */
    }

    mutex_lock(&mut text_mutex);
    if cfg!(feature = "cpu_micromips") {
        (*insn_p).halfword[0] = insn.word >> 16;
        (*insn_p).halfword[1] = insn.word;
    } else {
        *insn_p = insn;
    }

    flush_icache_range(insn_p as usize, insn_p as usize + core::mem::size_of::<mips_instruction>());

    mutex_unlock(&mut text_mutex);
}

#[cfg(feature = "modules")]
pub unsafe fn jump_label_apply_nops(mod_: *mut module) {
    let iter_start: *mut jump_entry = (*mod_).jump_entries;
    let iter_stop: *mut jump_entry = iter_start.add((*mod_).num_jump_entries as usize);
    let mut iter: *mut jump_entry;

    /* if the module doesn't have jump label entries, just return */
    if iter_start == iter_stop {
        return;
    }

    iter = iter_start;
    while iter < iter_stop {
        /* Only write NOPs for arch_branch_static(). */
        if jump_label_init_type(iter) == JUMP_LABEL_NOP {
            arch_jump_label_transform(iter, JUMP_LABEL_NOP);
        }
        iter = iter.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
