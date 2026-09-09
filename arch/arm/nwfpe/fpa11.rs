// SPDX-License-Identifier: GPL-2.0-or-later
/*
    NetWinder Floating Point Emulator
    (c) Rebel.COM, 1998,1999
    (c) Philip Blundell, 2001

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>
*/

// Dependencies supplied by fpa11.h, fpopcode.h, fpmodule.h, and fpmodule.inl

/* Reset the FPA11 chip.  Called to initialize and reset the emulator. */
unsafe fn resetFPA11() {
    let fpa11: *mut FPA11 = GET_FPA11();

    /* initialize the register type array */
    for i in 0..=7 {
        (*fpa11).fType[i] = typeNone;
    }

    /* FPSR: set system id to FP_EMULATOR, set AC, clear all other bits */
    (*fpa11).fpsr = FP_EMULATOR | BIT_AC;
}

pub fn SetRoundingMode(opcode: u32) -> i8 {
    match opcode & MASK_ROUNDING_MODE {
        ROUND_TO_PLUS_INFINITY => float_round_up,
        ROUND_TO_MINUS_INFINITY => float_round_down,
        ROUND_TO_ZERO => float_round_to_zero,
        _ => float_round_nearest_even,
    }
}

pub fn SetRoundingPrecision(opcode: u32) -> i8 {
    // CONFIG_FPE_NWFPE_XP may select the precision cases below at build time.
    #[cfg(CONFIG_FPE_NWFPE_XP)]
    {
        return match opcode & MASK_ROUNDING_PRECISION {
            ROUND_SINGLE => 32,
            ROUND_DOUBLE => 64,
            ROUND_EXTENDED => 80,
            _ => 80,
        };
    }
    80
}

pub unsafe fn nwfpe_init_fpa(fp: *mut fp_state) {
    let fpa11 = fp as *mut FPA11;
    // NWFPE_DEBUG: printk("NWFPE: setting up state.\n");
    core::ptr::write_bytes(fpa11 as *mut u8, 0, core::mem::size_of::<FPA11>());
    resetFPA11();
    (*fpa11).initflag = 1;
}

/* Emulate the instruction in the opcode. */
pub unsafe fn EmulateAll(opcode: u32) -> u32 {
    // NWFPE_DEBUG: printk("NWFPE: emulating opcode %08x\n", opcode);
    let mut code = opcode & 0x00000f00;
    if code == 0x00000100 || code == 0x00000200 {
        /* For coprocessor 1 or 2 (FPA11) */
        code = opcode & 0x0e000000;
        if code == 0x0e000000 {
            if opcode & 0x00000010 != 0 {
                /* Emulate conversion opcodes. */
                /* Emulate register transfer opcodes. */
                /* Emulate comparison opcodes. */
                return EmulateCPRT(opcode);
            } else {
                /* Emulate monadic arithmetic opcodes. */
                /* Emulate dyadic arithmetic opcodes. */
                return EmulateCPDO(opcode);
            }
        } else if code == 0x0c000000 {
            /* Emulate load/store opcodes. */
            /* Emulate load/store multiple opcodes. */
            return EmulateCPDT(opcode);
        }
    }

    /* Invalid instruction detected.  Return FALSE. */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
