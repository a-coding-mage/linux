// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Cirrus Logic CLPS711X CLK driver
 *
 *  Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
 */

// External kernel declarations and constants are supplied by the surrounding
// translation unit.

const CLPS711X_SYSCON1: usize = 0x0100;
const CLPS711X_SYSCON2: usize = 0x1100;
const CLPS711X_SYSFLG2: usize = CLPS711X_SYSCON2 + SYSFLG_OFFSET;
const CLPS711X_PLLR: usize = 0xa5a8;

const CLPS711X_EXT_FREQ: u32 = 13000000;
const CLPS711X_OSC_FREQ: u32 = 3686400;

static SPI_DIV_TABLE: [ClkDivTable; 5] = [
    ClkDivTable { val: 0, div: 32 },
    ClkDivTable { val: 1, div: 8 },
    ClkDivTable { val: 2, div: 2 },
    ClkDivTable { val: 3, div: 1 },
    ClkDivTable { val: 0, div: 0 }, // sentinel
];

static TIMER_DIV_TABLE: [ClkDivTable; 3] = [
    ClkDivTable { val: 0, div: 256 },
    ClkDivTable { val: 1, div: 1 },
    ClkDivTable { val: 0, div: 0 }, // sentinel
];

#[repr(C)]
struct Clps711xClk {
    lock: Spinlock,
    clk_data: ClkHwOnecellData,
}

unsafe fn clps711x_clk_init_dt(np: *mut DeviceNode) {
    let mut tmp: u32;
    let (mut f_cpu, mut f_pll, mut f_bus, mut f_tim, mut f_pwm, mut f_spi):
        (u32, u32, u32, u32, u32, u32);
    let mut fref: u32 = 0;
    let mut clps711x_clk: *mut Clps711xClk;
    let base: *mut u8;

    WARN_ON(of_property_read_u32(np, b"startup-frequency\0".as_ptr(), &mut fref));

    base = of_iomap(np, 0);
    BUG_ON(base.is_null());

    clps711x_clk = kzalloc_flex::<Clps711xClk>(CLPS711X_CLK_MAX);
    BUG_ON(clps711x_clk.is_null());

    spin_lock_init(&mut (*clps711x_clk).lock);

    /* Read PLL multiplier value and sanity check */
    tmp = readl(base.add(CLPS711X_PLLR)) >> 24;
    if ((tmp >= 10 && tmp <= 50) || fref == 0) {
        f_pll = div_round_up(CLPS711X_OSC_FREQ.wrapping_mul(tmp), 2);
    } else {
        f_pll = fref;
    }

    tmp = readl(base.add(CLPS711X_SYSFLG2));
    if (tmp & SYSFLG2_CKMODE != 0) {
        f_cpu = CLPS711X_EXT_FREQ;
        f_bus = CLPS711X_EXT_FREQ;
        f_spi = div_round_closest(CLPS711X_EXT_FREQ, 96);
        f_pll = 0;
        f_pwm = div_round_closest(CLPS711X_EXT_FREQ, 128);
    } else {
        f_cpu = f_pll;
        if f_cpu > 36864000 {
            f_bus = div_round_up(f_cpu, 2);
        } else {
            f_bus = 36864000 / 2;
        }
        f_spi = div_round_closest(f_cpu, 576);
        f_pwm = div_round_closest(f_cpu, 768);
    }

    if (tmp & SYSFLG2_CKMODE != 0) {
        if readl(base.add(CLPS711X_SYSCON2)) & SYSCON2_OSTB != 0 {
            f_tim = div_round_closest(CLPS711X_EXT_FREQ, 26);
        } else {
            f_tim = div_round_closest(CLPS711X_EXT_FREQ, 24);
        }
    } else {
        f_tim = div_round_closest(f_cpu, 144);
    }

    tmp = readl(base.add(CLPS711X_SYSCON1));
    /* Timer1 in free running mode.
     * Counter will wrap around to 0xffff when it underflows
     * and will continue to count down.
     */
    tmp &= !(SYSCON1_TC1M | SYSCON1_TC1S);
    /* Timer2 in prescale mode.
     * Value written is automatically re-loaded when
     * the counter underflows.
     */
    tmp |= SYSCON1_TC2M | SYSCON1_TC2S;
    writel(tmp, base.add(CLPS711X_SYSCON1));

    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_DUMMY] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"dummy\0".as_ptr(), core::ptr::null(), 0, 0);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_CPU] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"cpu\0".as_ptr(), core::ptr::null(), 0, f_cpu);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_BUS] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"bus\0".as_ptr(), core::ptr::null(), 0, f_bus);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_PLL] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"pll\0".as_ptr(), core::ptr::null(), 0, f_pll);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_TIMERREF] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"timer_ref\0".as_ptr(), core::ptr::null(), 0, f_tim);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_TIMER1] = clk_hw_register_divider_table(core::ptr::null_mut(), b"timer1\0".as_ptr(), b"timer_ref\0".as_ptr(), 0, base.add(CLPS711X_SYSCON1), 5, 1, 0, TIMER_DIV_TABLE.as_ptr(), &mut (*clps711x_clk).lock);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_TIMER2] = clk_hw_register_divider_table(core::ptr::null_mut(), b"timer2\0".as_ptr(), b"timer_ref\0".as_ptr(), 0, base.add(CLPS711X_SYSCON1), 7, 1, 0, TIMER_DIV_TABLE.as_ptr(), &mut (*clps711x_clk).lock);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_PWM] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"pwm\0".as_ptr(), core::ptr::null(), 0, f_pwm);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_SPIREF] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"spi_ref\0".as_ptr(), core::ptr::null(), 0, f_spi);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_SPI] = clk_hw_register_divider_table(core::ptr::null_mut(), b"spi\0".as_ptr(), b"spi_ref\0".as_ptr(), 0, base.add(CLPS711X_SYSCON1), 16, 2, 0, SPI_DIV_TABLE.as_ptr(), &mut (*clps711x_clk).lock);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_UART] = clk_hw_register_fixed_factor(core::ptr::null_mut(), b"uart\0".as_ptr(), b"bus\0".as_ptr(), 0, 1, 10);
    (*clps711x_clk).clk_data.hws[CLPS711X_CLK_TICK] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"tick\0".as_ptr(), core::ptr::null(), 0, 64);
    tmp = 0;
    while tmp < CLPS711X_CLK_MAX {
        if is_err((*clps711x_clk).clk_data.hws[tmp as usize]) {
            pr_err(b"clk %i: register failed with %ld\n\0".as_ptr(), tmp, ptr_err((*clps711x_clk).clk_data.hws[tmp as usize]));
        }
        tmp += 1;
    }

    (*clps711x_clk).clk_data.num = CLPS711X_CLK_MAX;
    of_clk_add_hw_provider(np, of_clk_hw_onecell_get, &mut (*clps711x_clk).clk_data);
}

// CLK_OF_DECLARE(clps711x, "cirrus,ep7209-clk", clps711x_clk_init_dt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
