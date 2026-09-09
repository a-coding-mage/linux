// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by asm/contregs.h, asm/sun3mmu.h, asm/io.h, and
// sun3.h remain external to this translation unit.

extern "C" {
    fn GET_DFC(dfc: *mut u8);
    fn SET_DFC(dfc: u8);
    fn SET_CONTROL_BYTE(address: u32, byte: u8);
}

pub unsafe fn sun3_leds(byte: u8) {
    let mut dfc: u8 = 0;

    GET_DFC(&mut dfc as *mut u8);
    SET_DFC(FC_CONTROL);
    SET_CONTROL_BYTE(AC_LEDS, byte);
    SET_DFC(dfc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
