/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (C) 2024 Mobileye Vision Technologies Ltd.
 */

pub const EQ5C_PLL_CPU: i32 = 0;
pub const EQ5C_PLL_VMP: i32 = 1;
pub const EQ5C_PLL_PMA: i32 = 2;
pub const EQ5C_PLL_VDI: i32 = 3;
pub const EQ5C_PLL_DDR0: i32 = 4;
pub const EQ5C_PLL_PCI: i32 = 5;
pub const EQ5C_PLL_PER: i32 = 6;
pub const EQ5C_PLL_PMAC: i32 = 7;
pub const EQ5C_PLL_MPC: i32 = 8;
pub const EQ5C_PLL_DDR1: i32 = 9;

pub const EQ5C_DIV_OSPI: i32 = 10;

/* EQ5C_PLL_CPU children */
pub const EQ5C_CPU_CORE0: i32 = 11;
pub const EQ5C_CPU_CORE1: i32 = 12;
pub const EQ5C_CPU_CORE2: i32 = 13;
pub const EQ5C_CPU_CORE3: i32 = 14;

/* EQ5C_PLL_PER children */
pub const EQ5C_PER_OCC: i32 = 15;
pub const EQ5C_PER_UART: i32 = 16;
pub const EQ5C_PER_SPI: i32 = 17;
pub const EQ5C_PER_I2C: i32 = 18;
pub const EQ5C_PER_GPIO: i32 = 19;
pub const EQ5C_PER_EMMC: i32 = 20;
pub const EQ5C_PER_OCC_PCI: i32 = 21;

pub const EQ6LC_PLL_DDR: i32 = 0;
pub const EQ6LC_PLL_CPU: i32 = 1;
pub const EQ6LC_PLL_PER: i32 = 2;
pub const EQ6LC_PLL_VDI: i32 = 3;

pub const EQ6HC_CENTRAL_PLL_CPU: i32 = 0;
pub const EQ6HC_CENTRAL_CPU_OCC: i32 = 1;

pub const EQ6HC_WEST_PLL_PER: i32 = 0;
pub const EQ6HC_WEST_PER_OCC: i32 = 1;
pub const EQ6HC_WEST_PER_UART: i32 = 2;

pub const EQ6HC_SOUTH_PLL_VDI: i32 = 0;
pub const EQ6HC_SOUTH_PLL_PCIE: i32 = 1;
pub const EQ6HC_SOUTH_PLL_PER: i32 = 2;
pub const EQ6HC_SOUTH_PLL_ISP: i32 = 3;

pub const EQ6HC_SOUTH_DIV_EMMC: i32 = 4;
pub const EQ6HC_SOUTH_DIV_OSPI_REF: i32 = 5;
pub const EQ6HC_SOUTH_DIV_OSPI_SYS: i32 = 6;
pub const EQ6HC_SOUTH_DIV_TSU: i32 = 7;

pub const EQ6HC_ACC_PLL_XNN: i32 = 0;
pub const EQ6HC_ACC_PLL_VMP: i32 = 1;
pub const EQ6HC_ACC_PLL_PMA: i32 = 2;
pub const EQ6HC_ACC_PLL_MPC: i32 = 3;
pub const EQ6HC_ACC_PLL_NOC: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
