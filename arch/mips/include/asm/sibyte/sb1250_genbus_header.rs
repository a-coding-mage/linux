/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SB1250 Generic Bus Constants. Translated from sb1250_genbus.h. */
// Dependency: symbols from asm/sibyte/sb1250_defs.h are supplied externally.

/* The following feature-conditional declarations are retained unconditionally;
 * their original conditions were SIBYTE_HDR_FEATURE(1250, PASS2),
 * SIBYTE_HDR_FEATURE(112x, PASS1), or SIBYTE_HDR_FEATURE_CHIP(1480), as noted
 * in the source header. */

pub const S_IO_RDY_ACTIVE: u32 = 0;
pub const M_IO_RDY_ACTIVE: u64 = _SB_MAKEMASK1(S_IO_RDY_ACTIVE);
pub const S_IO_ENA_RDY: u32 = 1;
pub const M_IO_ENA_RDY: u64 = _SB_MAKEMASK1(S_IO_ENA_RDY);
pub const S_IO_WIDTH_SEL: u32 = 2;
pub const M_IO_WIDTH_SEL: u64 = _SB_MAKEMASK(2, S_IO_WIDTH_SEL);
pub const K_IO_WIDTH_SEL_1: u32 = 0;
pub const K_IO_WIDTH_SEL_2: u32 = 1;
pub const K_IO_WIDTH_SEL_1L: u32 = 2;
pub const K_IO_WIDTH_SEL_4: u32 = 3;
pub const S_IO_PARITY_ENA: u32 = 4;
pub const M_IO_PARITY_ENA: u64 = _SB_MAKEMASK1(S_IO_PARITY_ENA);
pub const S_IO_BURST_EN: u32 = 5;
pub const M_IO_BURST_EN: u64 = _SB_MAKEMASK1(S_IO_BURST_EN);
pub const S_IO_PARITY_ODD: u32 = 6;
pub const M_IO_PARITY_ODD: u64 = _SB_MAKEMASK1(S_IO_PARITY_ODD);
pub const S_IO_NONMUX: u32 = 7;
pub const M_IO_NONMUX: u64 = _SB_MAKEMASK1(S_IO_NONMUX);
pub const S_IO_TIMEOUT: u32 = 8;
pub const M_IO_TIMEOUT: u64 = _SB_MAKEMASK(8, S_IO_TIMEOUT);
pub const S_IO_MULT_SIZE: u32 = 0;
pub const M_IO_MULT_SIZE: u64 = _SB_MAKEMASK(12, S_IO_MULT_SIZE);
pub const S_IO_REGSIZE: u32 = 16;
pub const S_IO_START_ADDR: u32 = 0;
pub const M_IO_START_ADDR: u64 = _SB_MAKEMASK(14, S_IO_START_ADDR);
pub const S_IO_ADDRBASE: u32 = 16;
pub const M_IO_BLK_CACHE: u64 = _SB_MAKEMASK1(15);

pub const S_IO_ALE_WIDTH: u32 = 0;
pub const M_IO_ALE_WIDTH: u64 = _SB_MAKEMASK(3, S_IO_ALE_WIDTH);
pub const S_IO_ALE_TO_CS: u32 = 4;
pub const M_IO_ALE_TO_CS: u64 = _SB_MAKEMASK(2, S_IO_ALE_TO_CS);
pub const M_IO_EARLY_CS: u64 = _SB_MAKEMASK1(3);
pub const S_IO_BURST_WIDTH: u64 = _SB_MAKE64(6);
pub const M_IO_BURST_WIDTH: u64 = _SB_MAKEMASK(2, S_IO_BURST_WIDTH);
pub const S_IO_CS_WIDTH: u32 = 8;
pub const M_IO_CS_WIDTH: u64 = _SB_MAKEMASK(5, S_IO_CS_WIDTH);
pub const S_IO_RDY_SMPLE: u32 = 13;
pub const M_IO_RDY_SMPLE: u64 = _SB_MAKEMASK(3, S_IO_RDY_SMPLE);

pub const S_IO_ALE_TO_WRITE: u32 = 0;
pub const M_IO_ALE_TO_WRITE: u64 = _SB_MAKEMASK(3, S_IO_ALE_TO_WRITE);
pub const M_IO_RDY_SYNC: u64 = _SB_MAKEMASK1(3);
pub const S_IO_WRITE_WIDTH: u32 = 4;
pub const M_IO_WRITE_WIDTH: u64 = _SB_MAKEMASK(4, S_IO_WRITE_WIDTH);
pub const S_IO_IDLE_CYCLE: u32 = 8;
pub const M_IO_IDLE_CYCLE: u64 = _SB_MAKEMASK(4, S_IO_IDLE_CYCLE);
pub const S_IO_OE_TO_CS: u32 = 12;
pub const M_IO_OE_TO_CS: u64 = _SB_MAKEMASK(2, S_IO_OE_TO_CS);
pub const S_IO_CS_TO_OE: u32 = 14;
pub const M_IO_CS_TO_OE: u64 = _SB_MAKEMASK(2, S_IO_CS_TO_OE);

pub const M_IO_CS_ERR_INT: u64 = _SB_MAKEMASK(0, 8);
pub const M_IO_CS0_ERR_INT: u64 = _SB_MAKEMASK1(0);
pub const M_IO_CS1_ERR_INT: u64 = _SB_MAKEMASK1(1);
pub const M_IO_CS2_ERR_INT: u64 = _SB_MAKEMASK1(2);
pub const M_IO_CS3_ERR_INT: u64 = _SB_MAKEMASK1(3);
pub const M_IO_CS4_ERR_INT: u64 = _SB_MAKEMASK1(4);
pub const M_IO_CS5_ERR_INT: u64 = _SB_MAKEMASK1(5);
pub const M_IO_CS6_ERR_INT: u64 = _SB_MAKEMASK1(6);
pub const M_IO_CS7_ERR_INT: u64 = _SB_MAKEMASK1(7);
pub const M_IO_RD_PAR_INT: u64 = _SB_MAKEMASK1(9);
pub const M_IO_TIMEOUT_INT: u64 = _SB_MAKEMASK1(10);
pub const M_IO_ILL_ADDR_INT: u64 = _SB_MAKEMASK1(11);
pub const M_IO_MULT_CS_INT: u64 = _SB_MAKEMASK1(12);
pub const M_IO_COH_ERR: u64 = _SB_MAKEMASK1(14);

/* Output drive control fields. */
pub const S_IO_SLEW0: u32 = 0; pub const S_IO_DRV_A: u32 = 2; pub const S_IO_DRV_B: u32 = 6;
pub const S_IO_DRV_C: u32 = 10; pub const S_IO_DRV_D: u32 = 14;
pub const S_IO_DRV_E: u32 = 2; pub const S_IO_DRV_F: u32 = 6; pub const S_IO_SLEW1: u32 = 8;
pub const S_IO_DRV_G: u32 = 10; pub const S_IO_SLEW2: u32 = 12; pub const S_IO_DRV_H: u32 = 14;
pub const S_IO_DRV_J: u32 = 2; pub const S_IO_DRV_K: u32 = 6; pub const S_IO_DRV_L: u32 = 10; pub const S_IO_DRV_M: u32 = 14;
pub const S_IO_SLEW3: u32 = 0; pub const S_IO_DRV_N: u32 = 2; pub const S_IO_DRV_P: u32 = 6; pub const S_IO_DRV_Q: u32 = 10; pub const S_IO_DRV_R: u32 = 14;

pub const M_PCMCIA_CFG_ATTRMEM: u64 = _SB_MAKEMASK1(0); pub const M_PCMCIA_CFG_3VEN: u64 = _SB_MAKEMASK1(1);
pub const M_PCMCIA_CFG_5VEN: u64 = _SB_MAKEMASK1(2); pub const M_PCMCIA_CFG_VPPEN: u64 = _SB_MAKEMASK1(3);
pub const M_PCMCIA_CFG_RESET: u64 = _SB_MAKEMASK1(4); pub const M_PCMCIA_CFG_APWRONEN: u64 = _SB_MAKEMASK1(5);
pub const M_PCMCIA_CFG_CDMASK: u64 = _SB_MAKEMASK1(6); pub const M_PCMCIA_CFG_WPMASK: u64 = _SB_MAKEMASK1(7);
pub const M_PCMCIA_CFG_RDYMASK: u64 = _SB_MAKEMASK1(8); pub const M_PCMCIA_CFG_PWRCTL: u64 = _SB_MAKEMASK1(9);
pub const S_PCMCIA_MODE: u32 = 16; pub const M_PCMCIA_MODE: u64 = _SB_MAKEMASK(3, S_PCMCIA_MODE);
pub const K_PCMCIA_MODE_PCMA_NOB: u32 = 0; pub const K_PCMCIA_MODE_IDEA_NOB: u32 = 1; pub const K_PCMCIA_MODE_PCMIOA_NOB: u32 = 2;
pub const K_PCMCIA_MODE_PCMA_PCMB: u32 = 4; pub const K_PCMCIA_MODE_IDEA_PCMB: u32 = 5; pub const K_PCMCIA_MODE_PCMA_IDEB: u32 = 6; pub const K_PCMCIA_MODE_IDEA_IDEB: u32 = 7;
pub const M_PCMCIA_STATUS_CD1: u64 = _SB_MAKEMASK1(0); pub const M_PCMCIA_STATUS_CD2: u64 = _SB_MAKEMASK1(1);
pub const M_PCMCIA_STATUS_VS1: u64 = _SB_MAKEMASK1(2); pub const M_PCMCIA_STATUS_VS2: u64 = _SB_MAKEMASK1(3);
pub const M_PCMCIA_STATUS_WP: u64 = _SB_MAKEMASK1(4); pub const M_PCMCIA_STATUS_RDY: u64 = _SB_MAKEMASK1(5);
pub const M_PCMCIA_STATUS_3VEN: u64 = _SB_MAKEMASK1(6); pub const M_PCMCIA_STATUS_5VEN: u64 = _SB_MAKEMASK1(7);
pub const M_PCMCIA_STATUS_CDCHG: u64 = _SB_MAKEMASK1(8); pub const M_PCMCIA_STATUS_WPCHG: u64 = _SB_MAKEMASK1(9); pub const M_PCMCIA_STATUS_RDYCHG: u64 = _SB_MAKEMASK1(10);

pub const K_GPIO_INTR_DISABLE: u32 = 0; pub const K_GPIO_INTR_EDGE: u32 = 1; pub const K_GPIO_INTR_LEVEL: u32 = 2; pub const K_GPIO_INTR_SPLIT: u32 = 3;

macro_rules! gpio_type_fields { ($($n:ident => $s:expr),* $(,)?) => { $(
    pub const $n: u32 = $s;
)* }; }
gpio_type_fields!(S_GPIO_INTR_TYPE0 => 0, S_GPIO_INTR_TYPE2 => 2, S_GPIO_INTR_TYPE4 => 4, S_GPIO_INTR_TYPE6 => 6,
    S_GPIO_INTR_TYPE8 => 8, S_GPIO_INTR_TYPE10 => 10, S_GPIO_INTR_TYPE12 => 12, S_GPIO_INTR_TYPE14 => 14);

macro_rules! gpio_atype_fields { ($($n:ident => $s:expr),* $(,)?) => { $(pub const $n: u32 = $s;)* }; }
pub const K_GPIO_INTR_BOTHEDGE: u32 = 0; pub const K_GPIO_INTR_RISEEDGE: u32 = 1;
pub const K_GPIO_INTR_UNPRED1: u32 = 2; pub const K_GPIO_INTR_UNPRED2: u32 = 3;
gpio_atype_fields!(S_GPIO_INTR_ATYPE0 => 0, S_GPIO_INTR_ATYPE2 => 2, S_GPIO_INTR_ATYPE4 => 4, S_GPIO_INTR_ATYPE6 => 6,
    S_GPIO_INTR_ATYPE8 => 8, S_GPIO_INTR_ATYPE10 => 10, S_GPIO_INTR_ATYPE12 => 12, S_GPIO_INTR_ATYPE14 => 14);

macro_rules! field_ops { ($v:ident, $g:ident, $s:ident, $m:ident) => {
    #[inline] pub const fn $v(x: u64) -> u64 { _SB_MAKEVALUE(x, $s) }
    #[inline] pub const fn $g(x: u64) -> u64 { _SB_GETVALUE(x, $s, $m) }
}; }

field_ops!(V_IO_WIDTH_SEL, G_IO_WIDTH_SEL, S_IO_WIDTH_SEL, M_IO_WIDTH_SEL);
field_ops!(V_IO_TIMEOUT, G_IO_TIMEOUT, S_IO_TIMEOUT, M_IO_TIMEOUT);
field_ops!(V_IO_MULT_SIZE, G_IO_MULT_SIZE, S_IO_MULT_SIZE, M_IO_MULT_SIZE);
field_ops!(V_IO_START_ADDR, G_IO_START_ADDR, S_IO_START_ADDR, M_IO_START_ADDR);
field_ops!(V_IO_ALE_WIDTH, G_IO_ALE_WIDTH, S_IO_ALE_WIDTH, M_IO_ALE_WIDTH);
field_ops!(V_IO_ALE_TO_CS, G_IO_ALE_TO_CS, S_IO_ALE_TO_CS, M_IO_ALE_TO_CS);
field_ops!(V_IO_CS_WIDTH, G_IO_CS_WIDTH, S_IO_CS_WIDTH, M_IO_CS_WIDTH);
field_ops!(V_IO_RDY_SMPLE, G_IO_RDY_SMPLE, S_IO_RDY_SMPLE, M_IO_RDY_SMPLE);
field_ops!(V_IO_ALE_TO_WRITE, G_IO_ALE_TO_WRITE, S_IO_ALE_TO_WRITE, M_IO_ALE_TO_WRITE);
field_ops!(V_IO_WRITE_WIDTH, G_IO_WRITE_WIDTH, S_IO_WRITE_WIDTH, M_IO_WRITE_WIDTH);
field_ops!(V_IO_IDLE_CYCLE, G_IO_IDLE_CYCLE, S_IO_IDLE_CYCLE, M_IO_IDLE_CYCLE);
field_ops!(V_IO_OE_TO_CS, G_IO_OE_TO_CS, S_IO_OE_TO_CS, M_IO_OE_TO_CS);
field_ops!(V_IO_CS_TO_OE, G_IO_CS_TO_OE, S_IO_CS_TO_OE, M_IO_CS_TO_OE);
field_ops!(V_PCMCIA_MODE, G_PCMCIA_MODE, S_PCMCIA_MODE, M_PCMCIA_MODE);

macro_rules! drive_field { ($m:ident, $v:ident, $g:ident, $s:ident) => {
    pub const $m: u64 = _SB_MAKEMASK(2, $s);
    #[inline] pub const fn $v(x: u64) -> u64 { _SB_MAKEVALUE(x, $s) }
    #[inline] pub const fn $g(x: u64) -> u64 { _SB_GETVALUE(x, $s, $m) }
}; }
drive_field!(M_IO_SLEW0, V_IO_SLEW0, G_IO_SLEW0, S_IO_SLEW0);
drive_field!(M_IO_DRV_A, V_IO_DRV_A, G_IO_DRV_A, S_IO_DRV_A);
drive_field!(M_IO_DRV_B, V_IO_DRV_B, G_IO_DRV_B, S_IO_DRV_B);
drive_field!(M_IO_DRV_C, V_IO_DRV_C, G_IO_DRV_C, S_IO_DRV_C);
drive_field!(M_IO_DRV_D, V_IO_DRV_D, G_IO_DRV_D, S_IO_DRV_D);
drive_field!(M_IO_DRV_E, V_IO_DRV_E, G_IO_DRV_E, S_IO_DRV_E);
drive_field!(M_IO_DRV_F, V_IO_DRV_F, G_IO_DRV_F, S_IO_DRV_F);
drive_field!(M_IO_SLEW1, V_IO_SLEW1, G_IO_SLEW1, S_IO_SLEW1);
drive_field!(M_IO_DRV_G, V_IO_DRV_G, G_IO_DRV_G, S_IO_DRV_G);
drive_field!(M_IO_SLEW2, V_IO_SLEW2, G_IO_SLEW2, S_IO_SLEW2);
drive_field!(M_IO_DRV_H, V_IO_DRV_H, G_IO_DRV_H, S_IO_DRV_H);
drive_field!(M_IO_DRV_J, V_IO_DRV_J, G_IO_DRV_J, S_IO_DRV_J);
drive_field!(M_IO_DRV_K, V_IO_DRV_K, G_IO_DRV_K, S_IO_DRV_K);
drive_field!(M_IO_DRV_L, V_IO_DRV_L, G_IO_DRV_L, S_IO_DRV_L);
drive_field!(M_IO_DRV_M, V_IO_DRV_M, G_IO_DRV_M, S_IO_DRV_M);
drive_field!(M_IO_SLEW3, V_IO_SLEW3, G_IO_SLEW3, S_IO_SLEW3);
drive_field!(M_IO_DRV_N, V_IO_DRV_N, G_IO_DRV_N, S_IO_DRV_N);
drive_field!(M_IO_DRV_P, V_IO_DRV_P, G_IO_DRV_P, S_IO_DRV_P);
drive_field!(M_IO_DRV_Q, V_IO_DRV_Q, G_IO_DRV_Q, S_IO_DRV_Q);
drive_field!(M_IO_DRV_R, V_IO_DRV_R, G_IO_DRV_R, S_IO_DRV_R);
field_ops!(V_IO_BURST_WIDTH, G_IO_BURST_WIDTH, S_IO_BURST_WIDTH, M_IO_BURST_WIDTH);

pub const M_GPIO_INTR_TYPE0: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_TYPE0);
pub const M_GPIO_INTR_TYPE2: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_TYPE2);
pub const M_GPIO_INTR_TYPE4: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_TYPE4);
pub const M_GPIO_INTR_TYPE6: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_TYPE6);
pub const M_GPIO_INTR_TYPE8: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_TYPE8);
pub const M_GPIO_INTR_TYPE10: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_TYPE10);
pub const M_GPIO_INTR_TYPE12: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_TYPE12);
pub const M_GPIO_INTR_TYPE14: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_TYPE14);
pub const M_GPIO_INTR_ATYPE0: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_ATYPE0);
pub const M_GPIO_INTR_ATYPE2: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_ATYPE2);
pub const M_GPIO_INTR_ATYPE4: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_ATYPE4);
pub const M_GPIO_INTR_ATYPE6: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_ATYPE6);
pub const M_GPIO_INTR_ATYPE8: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_ATYPE8);
pub const M_GPIO_INTR_ATYPE10: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_ATYPE10);
pub const M_GPIO_INTR_ATYPE12: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_ATYPE12);
pub const M_GPIO_INTR_ATYPE14: u64 = _SB_MAKEMASK(2, S_GPIO_INTR_ATYPE14);

macro_rules! gpio_accessors { ($v:ident, $g:ident, $s:ident, $m:ident) => {
    #[inline] pub const fn $v(x: u64) -> u64 { _SB_MAKEVALUE(x, $s) }
    #[inline] pub const fn $g(x: u64) -> u64 { _SB_GETVALUE(x, $s, $m) }
}; }
gpio_accessors!(V_GPIO_INTR_TYPE0, G_GPIO_INTR_TYPE0, S_GPIO_INTR_TYPE0, M_GPIO_INTR_TYPE0);
gpio_accessors!(V_GPIO_INTR_TYPE2, G_GPIO_INTR_TYPE2, S_GPIO_INTR_TYPE2, M_GPIO_INTR_TYPE2);
gpio_accessors!(V_GPIO_INTR_TYPE4, G_GPIO_INTR_TYPE4, S_GPIO_INTR_TYPE4, M_GPIO_INTR_TYPE4);
gpio_accessors!(V_GPIO_INTR_TYPE6, G_GPIO_INTR_TYPE6, S_GPIO_INTR_TYPE6, M_GPIO_INTR_TYPE6);
gpio_accessors!(V_GPIO_INTR_TYPE8, G_GPIO_INTR_TYPE8, S_GPIO_INTR_TYPE8, M_GPIO_INTR_TYPE8);
gpio_accessors!(V_GPIO_INTR_TYPE10, G_GPIO_INTR_TYPE10, S_GPIO_INTR_TYPE10, M_GPIO_INTR_TYPE10);
gpio_accessors!(V_GPIO_INTR_TYPE12, G_GPIO_INTR_TYPE12, S_GPIO_INTR_TYPE12, M_GPIO_INTR_TYPE12);
gpio_accessors!(V_GPIO_INTR_TYPE14, G_GPIO_INTR_TYPE14, S_GPIO_INTR_TYPE14, M_GPIO_INTR_TYPE14);
gpio_accessors!(V_GPIO_INTR_ATYPE0, G_GPIO_INTR_ATYPE0, S_GPIO_INTR_ATYPE0, M_GPIO_INTR_ATYPE0);
gpio_accessors!(V_GPIO_INTR_ATYPE2, G_GPIO_INTR_ATYPE2, S_GPIO_INTR_ATYPE2, M_GPIO_INTR_ATYPE2);
gpio_accessors!(V_GPIO_INTR_ATYPE4, G_GPIO_INTR_ATYPE4, S_GPIO_INTR_ATYPE4, M_GPIO_INTR_ATYPE4);
gpio_accessors!(V_GPIO_INTR_ATYPE6, G_GPIO_INTR_ATYPE6, S_GPIO_INTR_ATYPE6, M_GPIO_INTR_ATYPE6);
gpio_accessors!(V_GPIO_INTR_ATYPE8, G_GPIO_INTR_ATYPE8, S_GPIO_INTR_ATYPE8, M_GPIO_INTR_ATYPE8);
gpio_accessors!(V_GPIO_INTR_ATYPE10, G_GPIO_INTR_ATYPE10, S_GPIO_INTR_ATYPE10, M_GPIO_INTR_ATYPE10);
gpio_accessors!(V_GPIO_INTR_ATYPE12, G_GPIO_INTR_ATYPE12, S_GPIO_INTR_ATYPE12, M_GPIO_INTR_ATYPE12);
gpio_accessors!(V_GPIO_INTR_ATYPE14, G_GPIO_INTR_ATYPE14, S_GPIO_INTR_ATYPE14, M_GPIO_INTR_ATYPE14);

#[inline] pub const fn S_GPIO_INTR_TYPEX(n: u64) -> u64 { (n / 2) * 2 }
#[inline] pub const fn M_GPIO_INTR_TYPEX(n: u64) -> u64 { _SB_MAKEMASK(2, S_GPIO_INTR_TYPEX(n)) }
#[inline] pub const fn V_GPIO_INTR_TYPEX(n: u64, x: u64) -> u64 { _SB_MAKEVALUE(x, S_GPIO_INTR_TYPEX(n)) }
#[inline] pub const fn G_GPIO_INTR_TYPEX(n: u64, x: u64) -> u64 { _SB_GETVALUE(x, S_GPIO_INTR_TYPEX(n), M_GPIO_INTR_TYPEX(n)) }
#[inline] pub const fn S_GPIO_INTR_ATYPEX(n: u64) -> u64 { (n / 2) * 2 }
#[inline] pub const fn M_GPIO_INTR_ATYPEX(n: u64) -> u64 { _SB_MAKEMASK(2, S_GPIO_INTR_ATYPEX(n)) }
#[inline] pub const fn V_GPIO_INTR_ATYPEX(n: u64, x: u64) -> u64 { _SB_MAKEVALUE(x, S_GPIO_INTR_ATYPEX(n)) }
#[inline] pub const fn G_GPIO_INTR_ATYPEX(n: u64, x: u64) -> u64 { _SB_GETVALUE(x, S_GPIO_INTR_ATYPEX(n), M_GPIO_INTR_ATYPEX(n)) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
