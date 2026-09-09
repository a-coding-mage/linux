// SPDX-License-Identifier: GPL-2.0
/*
 *    Common helper functions for kprobes and uprobes
 *
 *    Copyright IBM Corp. 2014
 */

// Dependencies supplied by the surrounding kernel translation:
// <linux/errno.h>, <asm/kprobes.h>, and <asm/dis.h>
extern "C" {
    fn is_known_insn(insn: *const u8) -> bool;
}

pub unsafe fn probe_is_prohibited_opcode(insn: *mut u16) -> i32 {
    if !is_known_insn(insn as *const u8) {
        return -EINVAL;
    }
    match *insn >> 8 {
        0x0c => return -EINVAL, // bassm
        0x0b => return -EINVAL, // bsm
        0x83 => return -EINVAL, // diag
        0x44 => return -EINVAL, // ex
        0xac => return -EINVAL, // stnsm
        0xad => return -EINVAL, // stosm
        0xc6 => match *insn & 0x0f {
            0x00 => return -EINVAL, // exrl
            _ => {}
        },
        _ => {}
    }
    match *insn {
        0x0101 => return -EINVAL, // pr
        0xb25a => return -EINVAL, // bsa
        0xb240 => return -EINVAL, // bakr
        0xb258 => return -EINVAL, // bsg
        0xb218 => return -EINVAL, // pc
        0xb228 => return -EINVAL, // pt
        0xb98d => return -EINVAL, // epsw
        0xe560 => return -EINVAL, // tbegin
        0xe561 => return -EINVAL, // tbeginc
        0xb2f8 => return -EINVAL, // tend
        _ => {}
    }
    0
}

pub unsafe fn probe_get_fixup_type(insn: *mut u16) -> i32 {
    let mut fixup = FIXUP_PSW_NORMAL;
    match *insn >> 8 {
        0x05 | 0x0d => {
            fixup = FIXUP_RETURN_REGISTER;
            if *insn & 0x0f == 0 { fixup |= FIXUP_BRANCH_NOT_TAKEN; }
        }
        0x06 | 0x07 => fixup = FIXUP_BRANCH_NOT_TAKEN,
        0x45 | 0x4d => fixup = FIXUP_RETURN_REGISTER,
        0x47 | 0x46 | 0x86 | 0x87 => fixup = FIXUP_BRANCH_NOT_TAKEN,
        0x82 => fixup = FIXUP_NOT_REQUIRED,
        0xb2 => if *insn & 0xff == 0xb2 { fixup = FIXUP_NOT_REQUIRED; },
        0xa7 => if *insn & 0x0f == 0x05 { fixup |= FIXUP_RETURN_REGISTER; },
        0xc0 => if *insn & 0x0f == 0x05 { fixup |= FIXUP_RETURN_REGISTER; },
        0xeb => match *insn.add(1) & 0xff {
            0x44 | 0x45 => fixup = FIXUP_BRANCH_NOT_TAKEN,
            _ => {}
        },
        0xe3 => if *insn.add(1) & 0xff == 0x46 { fixup = FIXUP_BRANCH_NOT_TAKEN; },
        0xec => match *insn.add(1) & 0xff {
            0xe5 | 0xe6 | 0xf6 | 0xf7 | 0xfc | 0xfd | 0xfe | 0xff => fixup = FIXUP_BRANCH_NOT_TAKEN,
            _ => {}
        },
        _ => {}
    }
    fixup
}

pub unsafe fn probe_is_insn_relative_long(insn: *mut u16) -> bool {
    match *insn >> 8 {
        0xc0 => (*insn & 0x0f) == 0x00,
        0xc4 => matches!(*insn & 0x0f, 0x02 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f),
        0xc6 => matches!(*insn & 0x0f, 0x02 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x0a | 0x0c | 0x0d | 0x0e | 0x0f),
        _ => false
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
