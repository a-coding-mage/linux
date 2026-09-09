/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency declarations supplied by the surrounding kernel translation. */
extern "C" {
    static MRSHPC_CSR: usize;
    static MRSHPC_CPWCR: usize;
    static MRSHPC_MW0CR1: usize;
    static MRSHPC_MW0CR2: usize;
    static MRSHPC_MW1CR1: usize;
    static MRSHPC_MW1CR2: usize;
    static MRSHPC_IOWCR1: usize;
    static MRSHPC_IOWCR2: usize;
    static MRSHPC_CDCR: usize;
    static MRSHPC_ICR: usize;
    static PA_MRSHPC_MW2: usize;

    fn __raw_readw(addr: *const u16) -> u16;
    fn __raw_writew(value: u16, addr: *mut u16);
    fn __raw_writeb(value: u8, addr: *mut u8);
}

#[inline]
pub unsafe fn mrshpc_setup_windows() {
    if (__raw_readw(MRSHPC_CSR as *const u16) & 0x000c) != 0 {
        return; /* Not detected */
    }

    if (__raw_readw(MRSHPC_CSR as *const u16) & 0x0080) == 0 {
        __raw_writew(0x0674, MRSHPC_CPWCR as *mut u16); /* Card Vcc is 3.3v? */
    } else {
        __raw_writew(0x0678, MRSHPC_CPWCR as *mut u16); /* Card Vcc is 5V */
    }

    /*
     * PC-Card window open
     * flag == COMMON/ATTRIBUTE/IO
     */
    /* common window open */
    __raw_writew(0x8a84, MRSHPC_MW0CR1 as *mut u16);
    if (__raw_readw(MRSHPC_CSR as *const u16) & 0x4000) != 0 {
        /* common mode & bus width 16bit SWAP = 1 */
        __raw_writew(0x0b00, MRSHPC_MW0CR2 as *mut u16);
    } else {
        /* common mode & bus width 16bit SWAP = 0 */
        __raw_writew(0x0300, MRSHPC_MW0CR2 as *mut u16);
    }

    /* attribute window open */
    __raw_writew(0x8a85, MRSHPC_MW1CR1 as *mut u16);
    if (__raw_readw(MRSHPC_CSR as *const u16) & 0x4000) != 0 {
        /* attribute mode & bus width 16bit SWAP = 1 */
        __raw_writew(0x0a00, MRSHPC_MW1CR2 as *mut u16);
    } else {
        /* attribute mode & bus width 16bit SWAP = 0 */
        __raw_writew(0x0200, MRSHPC_MW1CR2 as *mut u16);
    }

    /* I/O window open */
    __raw_writew(0x8a86, MRSHPC_IOWCR1 as *mut u16);
    __raw_writew(0x0008, MRSHPC_CDCR as *mut u16); /* I/O card mode */
    if (__raw_readw(MRSHPC_CSR as *const u16) & 0x4000) != 0 {
        __raw_writew(0x0a00, MRSHPC_IOWCR2 as *mut u16); /* bus width 16bit SWAP = 1 */
    } else {
        __raw_writew(0x0200, MRSHPC_IOWCR2 as *mut u16); /* bus width 16bit SWAP = 0 */
    }

    __raw_writew(0x2000, MRSHPC_ICR as *mut u16);
    __raw_writeb(0x00, (PA_MRSHPC_MW2 + 0x206) as *mut u8);
    __raw_writeb(0x42, (PA_MRSHPC_MW2 + 0x200) as *mut u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
