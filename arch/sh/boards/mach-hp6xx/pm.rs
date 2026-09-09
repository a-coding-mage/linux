// SPDX-License-Identifier: GPL-2.0
/*
 * hp6x0 Power Management Routines
 *
 * Copyright (c) 2006 Andriy Skulysh <askulsyh@gmail.com>
 */

// Dependencies supplied by the kernel and architecture-specific headers.
use core::ptr;

const INTR_OFFSET: usize = 0x600;

const STBCR: usize = 0xffffff82;
const STBCR2: usize = 0xffffff88;

const STBCR_STBY: u8 = 0x80;
const STBCR_MSTP2: u8 = 0x04;

const MCR: usize = 0xffffff68;
const RTCNT: usize = 0xffffff70;

const MCR_RMODE: u16 = 2;
const MCR_RFSH: u16 = 4;

extern "C" {
    static mut wakeup_start: u8;
    static mut wakeup_end: u8;

    fn set_bl_bit();
    fn sh_wdt_read_csr() -> u8;
    fn sh_wdt_write_csr(value: u8);
    fn sh_wdt_write_cnt(value: u8);
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
    fn __raw_readb(addr: usize) -> u8;
    fn __raw_writeb(value: u8, addr: usize);
    fn get_zeroed_page(flags: usize) -> usize;
    fn udelay(usecs: usize);
    fn cpu_sleep();
    fn free_page(addr: usize);
    fn clear_bl_bit();

    fn outb(value: u8, port: usize);
    fn inb(port: usize) -> u8;
    fn outw(value: u16, port: usize);
    fn inw(port: usize) -> u16;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;
}

type suspend_state_t = i32;

#[repr(C)]
struct platform_suspend_ops {
    enter: Option<unsafe extern "C" fn(suspend_state_t) -> i32>,
    valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
}

unsafe fn pm_enter() {
    let mut stbcr: u8;
    let mut csr: u8;
    let mut frqcr: u16;
    let mut mcr: u16;
    let mut vbr_new: usize;
    let mut vbr_old: usize;

    set_bl_bit();

    /* set wdt */
    csr = sh_wdt_read_csr();
    csr &= !WTCSR_TME;
    csr |= WTCSR_CKS_4096;
    sh_wdt_write_csr(csr);
    csr = sh_wdt_read_csr();
    let _ = csr;
    sh_wdt_write_cnt(0);

    /* disable PLL1 */
    frqcr = __raw_readw(FRQCR);
    frqcr &= !(FRQCR_PLLEN | FRQCR_PSTBY);
    __raw_writew(frqcr, FRQCR);

    /* enable standby */
    stbcr = __raw_readb(STBCR);
    __raw_writeb(stbcr | STBCR_STBY | STBCR_MSTP2, STBCR);

    /* set self-refresh */
    mcr = __raw_readw(MCR);
    __raw_writew(mcr & !MCR_RFSH, MCR);

    /* set interrupt handler */
    core::arch::asm!("stc vbr, {0}", out(reg) vbr_old);
    vbr_new = get_zeroed_page(0);
    udelay(50);
    let wakeup_len = (&raw const wakeup_end as usize) - (&raw const wakeup_start as usize);
    ptr::copy_nonoverlapping(
        &raw const wakeup_start,
        (vbr_new + INTR_OFFSET) as *mut u8,
        wakeup_len,
    );
    core::arch::asm!("ldc {0}, vbr", in(reg) vbr_new);

    __raw_writew(0, RTCNT);
    __raw_writew(mcr | MCR_RFSH | MCR_RMODE, MCR);

    cpu_sleep();

    core::arch::asm!("ldc {0}, vbr", in(reg) vbr_old);

    free_page(vbr_new);

    /* enable PLL1 */
    frqcr = __raw_readw(FRQCR);
    frqcr |= FRQCR_PSTBY;
    __raw_writew(frqcr, FRQCR);
    udelay(50);
    frqcr |= FRQCR_PLLEN;
    __raw_writew(frqcr, FRQCR);

    __raw_writeb(stbcr, STBCR);

    clear_bl_bit();
}

unsafe extern "C" fn hp6x0_pm_enter(_state: suspend_state_t) -> i32 {
    let stbcr: u8;
    let stbcr2: u8;

    #[cfg(feature = "CONFIG_HD64461_ENABLER")]
    {
        let mut scr: u8;
        let mut hd64461_stbcr: u16;

        outb(0, HD64461_PCC1CSCIER);
        scr = inb(HD64461_PCC1SCR);
        scr |= HD64461_PCCSCR_VCC1;
        outb(scr, HD64461_PCC1SCR);
        hd64461_stbcr = inw(HD64461_STBCR);
        hd64461_stbcr |= HD64461_STBCR_SPC1ST;
        outw(hd64461_stbcr, HD64461_STBCR);
    }

    __raw_writeb(0x1f, DACR);

    stbcr = __raw_readb(STBCR);
    __raw_writeb(0x01, STBCR);

    stbcr2 = __raw_readb(STBCR2);
    __raw_writeb(0x7f, STBCR2);

    outw(0xf07f, HD64461_SCPUCR);

    pm_enter();

    outw(0, HD64461_SCPUCR);
    __raw_writeb(stbcr, STBCR);
    __raw_writeb(stbcr2, STBCR2);

    #[cfg(feature = "CONFIG_HD64461_ENABLER")]
    {
        let mut hd64461_stbcr = inw(HD64461_STBCR);
        hd64461_stbcr &= !HD64461_STBCR_SPC1ST;
        outw(hd64461_stbcr, HD64461_STBCR);
        outb(0x4c, HD64461_PCC1CSCIER);
        outb(0x00, HD64461_PCC1CSCR);
    }

    0
}

static HP6X0_PM_OPS: platform_suspend_ops = platform_suspend_ops {
    enter: Some(hp6x0_pm_enter),
    valid: Some(suspend_valid_only_mem),
};

unsafe extern "C" fn hp6x0_pm_init() -> i32 {
    suspend_set_ops(&raw const HP6X0_PM_OPS);
    0
}

// late_initcall(hp6x0_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
