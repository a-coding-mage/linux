/*
 *	TURBOchannel architecture calls.
 *
 *	Copyright (c) Harald Koerfgen, 1998
 *	Copyright (c) 2001, 2003, 2005, 2006  Maciej W. Rozycki
 *	Copyright (c) 2005  James Simmons
 *
 *	This file is subject to the terms and conditions of the GNU
 *	General Public License.  See the file "COPYING" in the main
 *	directory of this archive for more details.
 */

// Dependencies are supplied by the surrounding translation unit.

/*
 * Protected read byte from TURBOchannel slot space.
 */
pub unsafe fn tc_preadb(valp: *mut u8, addr: *mut core::ffi::c_void) -> i32 {
    unsafe { get_dbe(*valp, addr as *mut u8) }
}

/*
 * Get TURBOchannel bus information as specified by the spec, plus
 * the slot space base address and the number of slots.
 */
pub unsafe fn tc_bus_get_info(tbus: *mut tc_bus) -> i32 {
    unsafe {
        if dec_tc_bus == 0 {
            return -ENXIO;
        }

        core::ptr::copy_nonoverlapping(rex_gettcinfo(), &mut (*tbus).info, 1);
        (*tbus).slot_base = CPHYSADDR(rex_slot_address(0) as isize);

        match mips_machtype {
            MACH_DS5000_200 => {
                (*tbus).num_tcslots = 7;
            }
            MACH_DS5000_2X0 | MACH_DS5900 => {
                (*tbus).ext_slot_base = 0x20000000;
                (*tbus).ext_slot_size = 0x20000000;
                (*tbus).num_tcslots = 3;
            }
            MACH_DS5000_1XX => {
                (*tbus).num_tcslots = 3;
            }
            MACH_DS5000_XX => {
                (*tbus).num_tcslots = 2;
            }
            _ => {}
        }
        0
    }
}

/*
 * Get the IRQ for the specified slot.
 */
pub unsafe fn tc_device_get_irq(tdev: *mut tc_dev) {
    unsafe {
        (*tdev).interrupt = match (*tdev).slot {
            0 => dec_interrupt[DEC_IRQ_TC0],
            1 => dec_interrupt[DEC_IRQ_TC1],
            2 => dec_interrupt[DEC_IRQ_TC2],
            /*
             * Yuck! DS5000/200 onboard devices
             */
            5 => dec_interrupt[DEC_IRQ_TC5],
            6 => dec_interrupt[DEC_IRQ_TC6],
            _ => -1,
        };
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
