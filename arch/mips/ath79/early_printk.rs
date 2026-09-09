// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Atheros AR7XXX/AR9XXX SoC early printk support
 *
 *  Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 */

// Dependencies supplied by the surrounding kernel translation.

static mut _PROM_PUTCHAR: Option<unsafe extern "C" fn(char)> = None;

#[inline]
unsafe fn prom_putchar_wait(reg: *mut u8, val: u32) {
    let mut t: u32;

    loop {
        t = core::ptr::read_volatile(reg as *const u32);
        if (t & val) == val {
            break;
        }
    }
}

unsafe extern "C" fn prom_putchar_ar71xx(ch: char) {
    let base = KSEG1ADDR(AR71XX_UART_BASE) as *mut u8;

    prom_putchar_wait(base.add(UART_LSR as usize * 4), UART_LSR_BOTH_EMPTY);
    core::ptr::write_volatile(
        base.add(UART_TX as usize * 4) as *mut u32,
        (ch as u8) as u32,
    );
    prom_putchar_wait(base.add(UART_LSR as usize * 4), UART_LSR_BOTH_EMPTY);
}

unsafe extern "C" fn prom_putchar_ar933x(ch: char) {
    let base = KSEG1ADDR(AR933X_UART_BASE) as *mut u8;

    prom_putchar_wait(base.add(AR933X_UART_DATA_REG as usize), AR933X_UART_DATA_TX_CSR);
    core::ptr::write_volatile(
        base.add(AR933X_UART_DATA_REG as usize) as *mut u32,
        AR933X_UART_DATA_TX_CSR | (ch as u8) as u32,
    );
    prom_putchar_wait(base.add(AR933X_UART_DATA_REG as usize), AR933X_UART_DATA_TX_CSR);
}

unsafe extern "C" fn prom_putchar_dummy(_ch: char) {
    /* nothing to do */
}

unsafe fn prom_enable_uart(id: u32) {
    let uart_en: u32;
    let gpio_base: *mut u8;
    let mut t: u32;

    match id {
        REV_ID_MAJOR_AR71XX => uart_en = AR71XX_GPIO_FUNC_UART_EN,
        REV_ID_MAJOR_AR7240 | REV_ID_MAJOR_AR7241 | REV_ID_MAJOR_AR7242 => {
            uart_en = AR724X_GPIO_FUNC_UART_EN
        }
        REV_ID_MAJOR_AR913X => uart_en = AR913X_GPIO_FUNC_UART_EN,
        REV_ID_MAJOR_AR9330 | REV_ID_MAJOR_AR9331 => uart_en = AR933X_GPIO_FUNC_UART_EN,
        REV_ID_MAJOR_AR9341 | REV_ID_MAJOR_AR9342 | REV_ID_MAJOR_AR9344 => {
            /* TODO */
            return;
        }
        _ => return,
    }

    gpio_base = KSEG1ADDR(AR71XX_GPIO_BASE) as *mut u8;
    t = core::ptr::read_volatile(gpio_base.add(AR71XX_GPIO_REG_FUNC as usize) as *const u32);
    t |= uart_en;
    core::ptr::write_volatile(
        gpio_base.add(AR71XX_GPIO_REG_FUNC as usize) as *mut u32,
        t,
    );
}

unsafe fn prom_putchar_init() {
    let base = KSEG1ADDR(AR71XX_RESET_BASE) as *mut u8;
    let mut id = core::ptr::read_volatile(
        base.add(AR71XX_RESET_REG_REV_ID as usize) as *const u32,
    );
    id &= REV_ID_MAJOR_MASK;

    match id {
        REV_ID_MAJOR_AR71XX
        | REV_ID_MAJOR_AR7240
        | REV_ID_MAJOR_AR7241
        | REV_ID_MAJOR_AR7242
        | REV_ID_MAJOR_AR913X
        | REV_ID_MAJOR_AR9341
        | REV_ID_MAJOR_AR9342
        | REV_ID_MAJOR_AR9344
        | REV_ID_MAJOR_QCA9533
        | REV_ID_MAJOR_QCA9533_V2
        | REV_ID_MAJOR_QCA9556
        | REV_ID_MAJOR_QCA9558
        | REV_ID_MAJOR_TP9343
        | REV_ID_MAJOR_QCA956X
        | REV_ID_MAJOR_QCN550X => _PROM_PUTCHAR = Some(prom_putchar_ar71xx),
        REV_ID_MAJOR_AR9330 | REV_ID_MAJOR_AR9331 => _PROM_PUTCHAR = Some(prom_putchar_ar933x),
        _ => {
            _PROM_PUTCHAR = Some(prom_putchar_dummy);
            return;
        }
    }

    prom_enable_uart(id);
}

pub unsafe extern "C" fn prom_putchar(ch: char) {
    if _PROM_PUTCHAR.is_none() {
        prom_putchar_init();
    }

    (_PROM_PUTCHAR.unwrap())(ch);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
