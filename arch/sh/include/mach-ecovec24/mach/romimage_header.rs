/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The __ASSEMBLER__ branch contains SH assembly for the ROM image.  Rust has
 * no direct equivalent for this assembler-only translation unit; preserve the
 * source and its build-time intent here for the assembler configuration.
 */
#[cfg(__ASSEMBLER__)]
mod assembler_source {
    /* EcoVec board specific boot code:
     * converts the "partner-jet-script.txt" script into assembly
     * the assembly code is the first code to be executed in the romImage
     */

    /* execute icbi after enabling cache */
    // mov.l 1f, r0
    // icbi   @r0

    /* jump to cached area */
    // mova  2f, r0
    // jmp   @r0
    // nop

    // .align 2
    // 1: .long 0xa8000000
    // 2:
}

#[cfg(not(__ASSEMBLER__))]
mod c_header {
    /* Ecovec board specific information:
     *
     * Set the following to enable MMCIF boot from the MMC card in CN12:
     *
     * DS1.5 = OFF (SH BOOT pin set to L)
     * DS2.6 = OFF (Select MMCIF on CN12 instead of SDHI1)
     * DS2.7 = ON  (Select MMCIF on CN12 instead of SDHI1)
     *
     */
    pub const HIZCRA: usize = 0xa4050158;
    pub const PGDR: usize = 0xa405012c;

    extern "C" {
        pub fn __raw_readw(addr: usize) -> u16;
        pub fn __raw_writew(value: u16, addr: usize);
        pub fn __raw_writeb(value: u8, addr: usize);
    }

    pub unsafe fn mmcif_update_progress(nr: i32) {
        /* disable Hi-Z for LED pins */
        __raw_writew(__raw_readw(HIZCRA) & !(1u16 << 1), HIZCRA);

        /* update progress on LED4, LED5, LED6 and LED7 */
        __raw_writeb((1i32 << (nr - 1)) as u8, PGDR);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
