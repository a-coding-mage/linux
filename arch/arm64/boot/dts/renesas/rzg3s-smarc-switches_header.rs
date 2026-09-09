/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * On-board switches for the Renesas RZ/G3S SMARC Module and RZ SMARC Carrier II
 * boards.
 *
 * Copyright (C) 2024 Renesas Electronics Corp.
 */

/*
 * On-board switches' states:
 * @SW_OFF: switch's state is OFF
 * @SW_ON:  switch's state is ON
 */
pub const SW_OFF: i32 = 0;
pub const SW_ON: i32 = 1;

/*
 * SW_CONFIG[x] switches' states:
 * @SW_CONFIG2:
 *	SW_OFF - SD0 is connected to eMMC
 *	SW_ON  - SD0 is connected to uSD0 card
 * @SW_CONFIG3:
 *	SW_OFF - SD2 is connected to SoC
 *	SW_ON  - SCIF1, SSI0, IRQ0, IRQ1 connected to SoC
 */
pub const SW_CONFIG2: i32 = SW_OFF;
pub const SW_CONFIG3: i32 = SW_ON;

/*
 * SW_OPT_MUX[x] switches' states:
 * @SW_OPT_MUX4:
 *	SW_OFF - The SMARC SER0 signals are routed to M.2 Key E UART
 *	SW_ON  - The SMARC SER0 signals are routed to PMOD1
 */
pub const SW_OPT_MUX4: i32 = SW_ON;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
