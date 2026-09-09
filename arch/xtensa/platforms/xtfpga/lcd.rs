/*
 * Driver for the LCD display on the Tensilica XTFPGA board family.
 * http://www.mytechcorp.com/cfdata/productFile/File1/MOC-16216B-B-A0A04.pdf
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001, 2006 Tensilica Inc.
 * Copyright (C) 2015 Cadence Design Systems Inc.
 */

// Dependencies supplied by the platform and kernel environment.
extern "C" {
    fn mdelay(msecs: u64);
    fn udelay(usecs: u64);
    fn IOADDR(addr: usize) -> usize;
}

/* LCD instruction and data addresses. */
const LCD_INSTR_ADDR: *mut u8 = IOADDR(CONFIG_XTFPGA_LCD_BASE_ADDR) as *mut u8;
const LCD_DATA_ADDR: *mut u8 = unsafe { LCD_INSTR_ADDR.add(4) };

const LCD_CLEAR: u8 = 0x1;
const LCD_DISPLAY_ON: u8 = 0xc;

/* 8bit and 2 lines display */
const LCD_DISPLAY_MODE8BIT: u8 = 0x38;
const LCD_DISPLAY_MODE4BIT: u8 = 0x28;
const LCD_DISPLAY_POS: u8 = 0x80;
const LCD_SHIFT_LEFT: u8 = 0x18;
const LCD_SHIFT_RIGHT: u8 = 0x1c;

unsafe fn lcd_put_byte(addr: *mut u8, data: u8) {
    // CONFIG_XTFPGA_LCD_8BIT_ACCESS selects the direct one-byte access path.
    #[cfg(CONFIG_XTFPGA_LCD_8BIT_ACCESS)]
    {
        core::ptr::write_volatile(addr, data);
    }
    #[cfg(not(CONFIG_XTFPGA_LCD_8BIT_ACCESS))]
    {
        core::ptr::write_volatile(addr, data & 0xf0);
        core::ptr::write_volatile(addr, data.wrapping_shl(4) & 0xf0);
    }
}

unsafe fn lcd_init() -> i32 {
    core::ptr::write_volatile(LCD_INSTR_ADDR, LCD_DISPLAY_MODE8BIT);
    mdelay(5);
    core::ptr::write_volatile(LCD_INSTR_ADDR, LCD_DISPLAY_MODE8BIT);
    udelay(200);
    core::ptr::write_volatile(LCD_INSTR_ADDR, LCD_DISPLAY_MODE8BIT);
    udelay(50);
    // CONFIG_XTFPGA_LCD_8BIT_ACCESS selects whether the four-bit setup runs.
    #[cfg(not(CONFIG_XTFPGA_LCD_8BIT_ACCESS))]
    {
        core::ptr::write_volatile(LCD_INSTR_ADDR, LCD_DISPLAY_MODE4BIT);
        udelay(50);
        lcd_put_byte(LCD_INSTR_ADDR, LCD_DISPLAY_MODE4BIT);
        udelay(50);
    }
    lcd_put_byte(LCD_INSTR_ADDR, LCD_DISPLAY_ON);
    udelay(50);
    lcd_put_byte(LCD_INSTR_ADDR, LCD_CLEAR);
    mdelay(10);
    lcd_disp_at_pos(b"XTENSA LINUX\0".as_ptr() as *mut u8, 0);
    0
}

pub unsafe fn lcd_disp_at_pos(mut str_: *mut u8, pos: u8) {
    lcd_put_byte(LCD_INSTR_ADDR, LCD_DISPLAY_POS | pos);
    udelay(100);
    while core::ptr::read(str_) != 0 {
        lcd_put_byte(LCD_DATA_ADDR, core::ptr::read(str_));
        udelay(200);
        str_ = str_.add(1);
    }
}

pub unsafe fn lcd_shiftleft() {
    lcd_put_byte(LCD_INSTR_ADDR, LCD_SHIFT_LEFT);
    udelay(50);
}

pub unsafe fn lcd_shiftright() {
    lcd_put_byte(LCD_INSTR_ADDR, LCD_SHIFT_RIGHT);
    udelay(50);
}

// arch_initcall(lcd_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
