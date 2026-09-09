/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2013 Samsung Electronics Co., Ltd.
 * Copyright (c) 2013 Linaro Ltd.
 *
 * Common Clock Framework support for all PLL's in Samsung platforms
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum samsung_pll_type {
    pll_2126,
    pll_3000,
    pll_35xx,
    pll_36xx,
    pll_2550,
    pll_2650,
    pll_4500,
    pll_4502,
    pll_4508,
    pll_4600,
    pll_4650,
    pll_4650c,
    pll_6552,
    pll_6552_s3c2416,
    pll_6553,
    pll_2550x,
    pll_2550xx,
    pll_2650x,
    pll_2650xx,
    pll_1417x,
    pll_1418x,
    pll_1450x,
    pll_1451x,
    pll_1452x,
    pll_1460x,
    pll_0818x,
    pll_0822x,
    pll_0831x,
    pll_142xx,
    pll_0516x,
    pll_0517x,
    pll_0518x,
    pll_531x,
    pll_1051x,
    pll_1052x,
    pll_0717x,
    pll_0718x,
    pll_0732x,
    pll_4311,
    pll_1017x,
    pll_1031x,
    pll_a9fracm,
    pll_a9fraco,
}

#[inline]
pub const fn pll_rate(fin: u64, m: u64, p: u64, s: u32, k: u64, ks: u32) -> u64 {
    fin * ((1u64 << ks) * m + k) / (1u64 << ks) / (p << s)
}

#[inline]
pub const fn pll_fraco_rate(fin: u64, m: u64, p: u64, s: u64, k: u64, ks: u32) -> u64 {
    fin * ((1u64 << ks) * m + k) / (1u64 << ks) / (p * (s + 1))
}

macro_rules! PLL_RATE { ($fin:expr, $m:expr, $p:expr, $s:expr, $k:expr, $ks:expr) => {
    pll_rate($fin as u64, $m as u64, $p as u64, $s as u32, $k as u64, $ks as u32)
} }
macro_rules! PLL_VALID_RATE { ($fin:expr, $fout:expr, $m:expr, $p:expr, $s:expr, $k:expr, $ks:expr) => {{
    const RATE: u64 = PLL_RATE!($fin, $m, $p, $s, $k, $ks);
    const _: () = assert!(RATE == $fout as u64);
    $fout
}} }
macro_rules! PLL_FRACO_RATE { ($fin:expr, $m:expr, $p:expr, $s:expr, $k:expr, $ks:expr) => {
    pll_fraco_rate($fin as u64, $m as u64, $p as u64, $s as u64, $k as u64, $ks as u32)
} }
macro_rules! PLL_FRACO_VALID_RATE { ($fin:expr, $fout:expr, $m:expr, $p:expr, $s:expr, $k:expr, $ks:expr) => {{
    const RATE: u64 = PLL_FRACO_RATE!($fin, $m, $p, $s, $k, $ks);
    const _: () = assert!(RATE == $fout as u64);
    $fout
}} }

macro_rules! PLL_35XX_RATE { ($fin:expr, $rate:expr, $m:expr, $p:expr, $s:expr) => {
    samsung_pll_rate_table { rate: PLL_VALID_RATE!($fin, $rate, $m, $p, $s, 0, 16), mdiv: $m, pdiv: $p, sdiv: $s, ..Default::default() }
} }
macro_rules! PLL_36XX_RATE { ($fin:expr, $rate:expr, $m:expr, $p:expr, $s:expr, $k:expr) => {
    samsung_pll_rate_table { rate: PLL_VALID_RATE!($fin, $rate, $m, $p, $s, $k, 16), mdiv: $m, pdiv: $p, sdiv: $s, kdiv: $k, ..Default::default() }
} }
macro_rules! PLL_4508_RATE { ($fin:expr, $rate:expr, $m:expr, $p:expr, $s:expr, $afc:expr) => {
    samsung_pll_rate_table { rate: PLL_VALID_RATE!($fin, $rate, $m, $p, $s - 1, 0, 16), mdiv: $m, pdiv: $p, sdiv: $s, afc: $afc, ..Default::default() }
} }
macro_rules! PLL_4600_RATE { ($fin:expr, $rate:expr, $m:expr, $p:expr, $s:expr, $k:expr, $vsel:expr) => {
    samsung_pll_rate_table { rate: PLL_VALID_RATE!($fin, $rate, $m, $p, $s, $k, 16), mdiv: $m, pdiv: $p, sdiv: $s, kdiv: $k, vsel: $vsel, ..Default::default() }
} }
macro_rules! PLL_4650_RATE { ($fin:expr, $rate:expr, $m:expr, $p:expr, $s:expr, $k:expr, $mfr:expr, $mrr:expr, $vsel:expr) => {
    samsung_pll_rate_table { rate: PLL_VALID_RATE!($fin, $rate, $m, $p, $s, $k, 10), mdiv: $m, pdiv: $p, sdiv: $s, kdiv: $k, mfr: $mfr, mrr: $mrr, vsel: $vsel, ..Default::default() }
} }
macro_rules! PLL_A9FRACO_RATE { ($fin:expr, $rate:expr, $m:expr, $p:expr, $s:expr, $k:expr) => {
    samsung_pll_rate_table { rate: PLL_FRACO_VALID_RATE!($fin, $rate, $m, $p, $s, $k, 24), mdiv: $m, pdiv: $p, sdiv: $s, kdiv: $k, ..Default::default() }
} }

/* NOTE: Rate table should be kept sorted in descending order. */
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct samsung_pll_rate_table {
    pub rate: u32,
    pub pdiv: u32,
    pub mdiv: u32,
    pub sdiv: u32,
    pub kdiv: u32,
    pub afc: u32,
    pub mfr: u32,
    pub mrr: u32,
    pub vsel: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
