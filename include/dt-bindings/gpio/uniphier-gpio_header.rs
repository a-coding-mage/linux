/*
 * Copyright (C) 2017 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

pub const UNIPHIER_GPIO_LINES_PER_BANK: i32 = 8;

pub const UNIPHIER_GPIO_IRQ_OFFSET: i32 = UNIPHIER_GPIO_LINES_PER_BANK * 15;

#[inline]
pub const fn UNIPHIER_GPIO_PORT(bank: i32, line: i32) -> i32 {
    UNIPHIER_GPIO_LINES_PER_BANK * bank + line
}

#[inline]
pub const fn UNIPHIER_GPIO_IRQ(n: i32) -> i32 {
    UNIPHIER_GPIO_IRQ_OFFSET + n
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
