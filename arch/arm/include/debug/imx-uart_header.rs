/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012-2015 Freescale Semiconductor, Inc.
 */

pub const IMX1_UART1_BASE_ADDR: usize = 0x00206000;
pub const IMX1_UART2_BASE_ADDR: usize = 0x00207000;

pub const IMX25_UART1_BASE_ADDR: usize = 0x43f90000;
pub const IMX25_UART2_BASE_ADDR: usize = 0x43f94000;
pub const IMX25_UART3_BASE_ADDR: usize = 0x5000c000;
pub const IMX25_UART4_BASE_ADDR: usize = 0x50008000;
pub const IMX25_UART5_BASE_ADDR: usize = 0x5002c000;

pub const IMX27_UART1_BASE_ADDR: usize = 0x1000a000;
pub const IMX27_UART2_BASE_ADDR: usize = 0x1000b000;
pub const IMX27_UART3_BASE_ADDR: usize = 0x1000c000;
pub const IMX27_UART4_BASE_ADDR: usize = 0x1000d000;

pub const IMX31_UART1_BASE_ADDR: usize = 0x43f90000;
pub const IMX31_UART2_BASE_ADDR: usize = 0x43f94000;
pub const IMX31_UART3_BASE_ADDR: usize = 0x5000c000;
pub const IMX31_UART4_BASE_ADDR: usize = 0x43fb0000;
pub const IMX31_UART5_BASE_ADDR: usize = 0x43fb4000;

pub const IMX35_UART1_BASE_ADDR: usize = 0x43f90000;
pub const IMX35_UART2_BASE_ADDR: usize = 0x43f94000;
pub const IMX35_UART3_BASE_ADDR: usize = 0x5000c000;

pub const IMX50_UART1_BASE_ADDR: usize = 0x53fbc000;
pub const IMX50_UART2_BASE_ADDR: usize = 0x53fc0000;
pub const IMX50_UART3_BASE_ADDR: usize = 0x5000c000;
pub const IMX50_UART4_BASE_ADDR: usize = 0x53ff0000;
pub const IMX50_UART5_BASE_ADDR: usize = 0x63f90000;

pub const IMX51_UART1_BASE_ADDR: usize = 0x73fbc000;
pub const IMX51_UART2_BASE_ADDR: usize = 0x73fc0000;
pub const IMX51_UART3_BASE_ADDR: usize = 0x7000c000;

pub const IMX53_UART1_BASE_ADDR: usize = 0x53fbc000;
pub const IMX53_UART2_BASE_ADDR: usize = 0x53fc0000;
pub const IMX53_UART3_BASE_ADDR: usize = 0x5000c000;
pub const IMX53_UART4_BASE_ADDR: usize = 0x53ff0000;
pub const IMX53_UART5_BASE_ADDR: usize = 0x63f90000;

pub const IMX6Q_UART1_BASE_ADDR: usize = 0x02020000;
pub const IMX6Q_UART2_BASE_ADDR: usize = 0x021e8000;
pub const IMX6Q_UART3_BASE_ADDR: usize = 0x021ec000;
pub const IMX6Q_UART4_BASE_ADDR: usize = 0x021f0000;
pub const IMX6Q_UART5_BASE_ADDR: usize = 0x021f4000;

pub const IMX6SL_UART1_BASE_ADDR: usize = 0x02020000;
pub const IMX6SL_UART2_BASE_ADDR: usize = 0x02024000;
pub const IMX6SL_UART3_BASE_ADDR: usize = 0x02034000;
pub const IMX6SL_UART4_BASE_ADDR: usize = 0x02038000;
pub const IMX6SL_UART5_BASE_ADDR: usize = 0x02018000;

pub const IMX6SX_UART1_BASE_ADDR: usize = 0x02020000;
pub const IMX6SX_UART2_BASE_ADDR: usize = 0x021e8000;
pub const IMX6SX_UART3_BASE_ADDR: usize = 0x021ec000;
pub const IMX6SX_UART4_BASE_ADDR: usize = 0x021f0000;
pub const IMX6SX_UART5_BASE_ADDR: usize = 0x021f4000;
pub const IMX6SX_UART6_BASE_ADDR: usize = 0x022a0000;

pub const IMX6UL_UART1_BASE_ADDR: usize = 0x02020000;
pub const IMX6UL_UART2_BASE_ADDR: usize = 0x021e8000;
pub const IMX6UL_UART3_BASE_ADDR: usize = 0x021ec000;
pub const IMX6UL_UART4_BASE_ADDR: usize = 0x021f0000;
pub const IMX6UL_UART5_BASE_ADDR: usize = 0x021f4000;
pub const IMX6UL_UART6_BASE_ADDR: usize = 0x021fc000;
pub const IMX6UL_UART7_BASE_ADDR: usize = 0x02018000;
pub const IMX6UL_UART8_BASE_ADDR: usize = 0x02024000;

pub const IMX7D_UART1_BASE_ADDR: usize = 0x30860000;
pub const IMX7D_UART2_BASE_ADDR: usize = 0x30890000;
pub const IMX7D_UART3_BASE_ADDR: usize = 0x30880000;
pub const IMX7D_UART4_BASE_ADDR: usize = 0x30a60000;
pub const IMX7D_UART5_BASE_ADDR: usize = 0x30a70000;
pub const IMX7D_UART6_BASE_ADDR: usize = 0x30a80000;
pub const IMX7D_UART7_BASE_ADDR: usize = 0x30a90000;

pub const fn imx1_uart_base(port: usize) -> usize {
    match port { 1 => IMX1_UART1_BASE_ADDR, 2 => IMX1_UART2_BASE_ADDR, _ => 0 }
}
pub const fn imx25_uart_base(port: usize) -> usize {
    match port { 1 => IMX25_UART1_BASE_ADDR, 2 => IMX25_UART2_BASE_ADDR, 3 => IMX25_UART3_BASE_ADDR, 4 => IMX25_UART4_BASE_ADDR, 5 => IMX25_UART5_BASE_ADDR, _ => 0 }
}
pub const fn imx27_uart_base(port: usize) -> usize {
    match port { 1 => IMX27_UART1_BASE_ADDR, 2 => IMX27_UART2_BASE_ADDR, 3 => IMX27_UART3_BASE_ADDR, 4 => IMX27_UART4_BASE_ADDR, _ => 0 }
}
pub const fn imx31_uart_base(port: usize) -> usize {
    match port { 1 => IMX31_UART1_BASE_ADDR, 2 => IMX31_UART2_BASE_ADDR, 3 => IMX31_UART3_BASE_ADDR, 4 => IMX31_UART4_BASE_ADDR, 5 => IMX31_UART5_BASE_ADDR, _ => 0 }
}
pub const fn imx35_uart_base(port: usize) -> usize {
    match port { 1 => IMX35_UART1_BASE_ADDR, 2 => IMX35_UART2_BASE_ADDR, 3 => IMX35_UART3_BASE_ADDR, _ => 0 }
}
pub const fn imx50_uart_base(port: usize) -> usize {
    match port { 1 => IMX50_UART1_BASE_ADDR, 2 => IMX50_UART2_BASE_ADDR, 3 => IMX50_UART3_BASE_ADDR, 4 => IMX50_UART4_BASE_ADDR, 5 => IMX50_UART5_BASE_ADDR, _ => 0 }
}
pub const fn imx51_uart_base(port: usize) -> usize {
    match port { 1 => IMX51_UART1_BASE_ADDR, 2 => IMX51_UART2_BASE_ADDR, 3 => IMX51_UART3_BASE_ADDR, _ => 0 }
}
pub const fn imx53_uart_base(port: usize) -> usize {
    match port { 1 => IMX53_UART1_BASE_ADDR, 2 => IMX53_UART2_BASE_ADDR, 3 => IMX53_UART3_BASE_ADDR, 4 => IMX53_UART4_BASE_ADDR, 5 => IMX53_UART5_BASE_ADDR, _ => 0 }
}
pub const fn imx6q_uart_base(port: usize) -> usize {
    match port { 1 => IMX6Q_UART1_BASE_ADDR, 2 => IMX6Q_UART2_BASE_ADDR, 3 => IMX6Q_UART3_BASE_ADDR, 4 => IMX6Q_UART4_BASE_ADDR, 5 => IMX6Q_UART5_BASE_ADDR, _ => 0 }
}
pub const fn imx6sl_uart_base(port: usize) -> usize {
    match port { 1 => IMX6SL_UART1_BASE_ADDR, 2 => IMX6SL_UART2_BASE_ADDR, 3 => IMX6SL_UART3_BASE_ADDR, 4 => IMX6SL_UART4_BASE_ADDR, 5 => IMX6SL_UART5_BASE_ADDR, _ => 0 }
}
pub const fn imx6sx_uart_base(port: usize) -> usize {
    match port { 1 => IMX6SX_UART1_BASE_ADDR, 2 => IMX6SX_UART2_BASE_ADDR, 3 => IMX6SX_UART3_BASE_ADDR, 4 => IMX6SX_UART4_BASE_ADDR, 5 => IMX6SX_UART5_BASE_ADDR, 6 => IMX6SX_UART6_BASE_ADDR, _ => 0 }
}
pub const fn imx6ul_uart_base(port: usize) -> usize {
    match port { 1 => IMX6UL_UART1_BASE_ADDR, 2 => IMX6UL_UART2_BASE_ADDR, 3 => IMX6UL_UART3_BASE_ADDR, 4 => IMX6UL_UART4_BASE_ADDR, 5 => IMX6UL_UART5_BASE_ADDR, 6 => IMX6UL_UART6_BASE_ADDR, 7 => IMX6UL_UART7_BASE_ADDR, 8 => IMX6UL_UART8_BASE_ADDR, _ => 0 }
}
pub const fn imx7d_uart_base(port: usize) -> usize {
    match port { 1 => IMX7D_UART1_BASE_ADDR, 2 => IMX7D_UART2_BASE_ADDR, 3 => IMX7D_UART3_BASE_ADDR, 4 => IMX7D_UART4_BASE_ADDR, 5 => IMX7D_UART5_BASE_ADDR, 6 => IMX7D_UART6_BASE_ADDR, 7 => IMX7D_UART7_BASE_ADDR, _ => 0 }
}

// The C token-pasting base-address macros are represented by the corresponding
// per-SoC constants above. UART_PADDR is selected by build-time configuration:
// CONFIG_DEBUG_IMX{1,25,27,31,35,50,51,53,6Q,6SL,6SX,6UL,7D}_UART and
// CONFIG_DEBUG_IMX_UART_PORT.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
