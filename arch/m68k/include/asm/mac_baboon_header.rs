/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for the "Baboon" custom IC on the PowerBook 190.
 */

pub const BABOON_BASE: u32 = 0x50F1A000; /* same as IDE controller base */

#[repr(C)]
pub struct baboon {
    pub pad1: [i8; 208], /* generic IDE registers, not used here */
    pub mb_control: i16, /* Control register:
                          * bit 5 : slot 2 power control
                          * bit 6 : slot 1 power control
                          */
    pub pad2: [i8; 2],
    pub mb_status: i16, /* (0xD4) media bay status register:
                         *
                         * bit 0: ????
                         * bit 1: IDE interrupt active?
                         * bit 2: bay status, 0 = full, 1 = empty
                         * bit 3: ????
                         */
    pub pad3: [i8; 2], /* (0xD6) not used */
    pub mb_ifr: i16, /* (0xD8) media bay interrupt flags register:
                      *
                      * bit 0: ????
                      * bit 1: IDE controller interrupt
                      * bit 2: media bay status change interrupt
                      */
}

extern "C" {
    pub static mut baboon_present: i32;

    pub fn baboon_register_interrupts();
    pub fn baboon_irq_enable(arg: i32);
    pub fn baboon_irq_disable(arg: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
