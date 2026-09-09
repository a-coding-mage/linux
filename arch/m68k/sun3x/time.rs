// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/sun3x/time.c
 *
 *  Sun3x-specific time handling
 */

// Linux and architecture header dependencies are supplied by other files.

#[repr(C)]
pub struct rtc_time {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
}

#[repr(C)]
pub struct mostek_dt {
    pub csr: u8,
    pub sec: u8,
    pub min: u8,
    pub hour: u8,
    pub wday: u8,
    pub mday: u8,
    pub month: u8,
    pub year: u8,
}

const M_CONTROL: usize = 0xf8;
const M_SEC: usize = 0xf9;
const M_MIN: usize = 0xfa;
const M_HOUR: usize = 0xfb;
const M_DAY: usize = 0xfc;
const M_DATE: usize = 0xfd;
const M_MONTH: usize = 0xfe;
const M_YEAR: usize = 0xff;

const C_WRITE: u8 = 0x80;
const C_READ: u8 = 0x40;
const C_SIGN: u8 = 0x20;
const C_CALIB: u8 = 0x1f;

extern "C" {
    static mut SUN3X_EEPROM: usize;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn sun3_disable_interrupts();
    fn sun3_disable_irq(irq: i32);
    fn sun3_enable_irq(irq: i32);
    fn sun3_enable_interrupts();
    fn bin2bcd(value: i32) -> u8;
    fn bcd2bin(value: u8) -> i32;
}

pub unsafe extern "C" fn sun3x_hwclk(set: i32, t: *mut rtc_time) -> i32 {
    let h = (SUN3X_EEPROM + M_CONTROL) as *mut mostek_dt;
    let mut flags: usize = 0;

    local_irq_save(&mut flags);

    if set != 0 {
        (*h).csr |= C_WRITE;
        (*h).sec = bin2bcd((*t).tm_sec);
        (*h).min = bin2bcd((*t).tm_min);
        (*h).hour = bin2bcd((*t).tm_hour);
        (*h).wday = bin2bcd((*t).tm_wday);
        (*h).mday = bin2bcd((*t).tm_mday);
        (*h).month = bin2bcd((*t).tm_mon + 1);
        (*h).year = bin2bcd((*t).tm_year % 100);
        (*h).csr &= !C_WRITE;
    } else {
        (*h).csr |= C_READ;
        (*t).tm_sec = bcd2bin((*h).sec);
        (*t).tm_min = bcd2bin((*h).min);
        (*t).tm_hour = bcd2bin((*h).hour);
        (*t).tm_wday = bcd2bin((*h).wday);
        (*t).tm_mday = bcd2bin((*h).mday);
        (*t).tm_mon = bcd2bin((*h).month) - 1;
        (*t).tm_year = bcd2bin((*h).year);
        (*h).csr &= !C_READ;
        if (*t).tm_year < 70 {
            (*t).tm_year += 100;
        }
    }

    local_irq_restore(flags);

    0
}

/*
#if 0
static irqreturn_t sun3x_timer_tick(int irq, void *dev_id)
{
    unsigned long flags;

    local_irq_save(flags);
    // Clear the pending interrupt - pulse the enable line low
    disable_irq(5);
    enable_irq(5);
    legacy_timer_tick(1);
    local_irq_restore(flags);

    return IRQ_HANDLED;
}
#endif
*/

pub unsafe extern "C" fn sun3x_sched_init() {
    sun3_disable_interrupts();

    /* Pulse enable low to get the clock started */
    sun3_disable_irq(5);
    sun3_enable_irq(5);
    sun3_enable_interrupts();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
