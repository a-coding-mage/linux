/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994, 1995, 1996, 1999, 2000 by Ralf Baechle
 * Copyright (C) 1999, 2000 by Silicon Graphics
 * Copyright (C) 2002  Maciej W. Rozycki
 */

// Linux and MIPS header dependencies are supplied by the surrounding system.

#[repr(C)]
pub struct pt_regs {
    pub cp0_cause: u32,
    pub cp0_epc: usize,
}

extern "C" {
    fn printk(fmt: *const u8, ...);
    fn show_regs(regs: *mut pt_regs);
    fn dump_tlb_all();
    fn force_sig(sig: i32);
    fn mips_set_be_handler(handler: unsafe extern "C" fn(*mut pt_regs, i32) -> i32);
}

// Supplied by the MIPS trap definitions.
extern "C" {
    static MIPS_BE_FIXUP: i32;
    static SIGBUS: i32;
}

unsafe extern "C" fn ip32_be_handler(regs: *mut pt_regs, is_fixup: i32) -> i32 {
    let data: i32 = ((*regs).cp0_cause & 4) as i32;

    if is_fixup != 0 {
        return MIPS_BE_FIXUP;
    }

    let kind = if data != 0 { b'd' } else { b'i' };
    let format = b"Got %cbe at 0x%lx\n\0";
    printk(format.as_ptr(), kind as i32, (*regs).cp0_epc);
    show_regs(regs);
    dump_tlb_all();
    loop {}
    force_sig(SIGBUS);
}

pub unsafe extern "C" fn ip32_be_init() {
    mips_set_be_handler(ip32_be_handler);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
