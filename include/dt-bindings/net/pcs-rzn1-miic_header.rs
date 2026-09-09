/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (C) 2022 Schneider-Electric
 *
 * Clément Léger <clement.leger@bootlin.com>
 */

/*
 * Refer to the datasheet [1] section 8.2.1, Internal Connection of Ethernet
 * Ports to check the available combination
 *
 * [1] REN_r01uh0750ej0140-rzn1-introduction_MAT_20210228.pdf
 */

pub const MIIC_GMAC1_PORT: i32 = 0;
pub const MIIC_GMAC2_PORT: i32 = 1;
pub const MIIC_RTOS_PORT: i32 = 2;
pub const MIIC_SERCOS_PORTA: i32 = 3;
pub const MIIC_SERCOS_PORTB: i32 = 4;
pub const MIIC_ETHERCAT_PORTA: i32 = 5;
pub const MIIC_ETHERCAT_PORTB: i32 = 6;
pub const MIIC_ETHERCAT_PORTC: i32 = 7;
pub const MIIC_SWITCH_PORTA: i32 = 8;
pub const MIIC_SWITCH_PORTB: i32 = 9;
pub const MIIC_SWITCH_PORTC: i32 = 10;
pub const MIIC_SWITCH_PORTD: i32 = 11;
pub const MIIC_HSR_PORTA: i32 = 12;
pub const MIIC_HSR_PORTB: i32 = 13;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
