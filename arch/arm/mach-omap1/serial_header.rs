/*
 * Copyright (C) 2009 Texas Instruments
 * Added OMAP4 support- Santosh Shilimkar <santosh.shilimkar@ti.com>
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 */

// C header guard: __ASM_ARCH_SERIAL_H
// Dependency: <linux/init.h>

/*
 * Memory entry used for the DEBUG_LL UART configuration, relative to
 * start of RAM. See also uncompress.h and debug-macro.S.
 *
 * Note that using a memory location for storing the UART configuration
 * has at least two limitations:
 *
 * 1. Kernel uncompress code cannot overlap OMAP_UART_INFO as the
 *    uncompress code could then partially overwrite itself
 * 2. We assume printascii is called at least once before paging_init,
 *    and addruart has a chance to read OMAP_UART_INFO
 */
pub const OMAP_UART_INFO_OFS: u32 = 0x3ffc;

pub const OMAP_PORT_SHIFT: u32 = 2;
pub const OMAP7XX_PORT_SHIFT: u32 = 0;

pub const OMAP1510_BASE_BAUD: u32 = 12000000 / 16;
pub const OMAP16XX_BASE_BAUD: u32 = 48000000 / 16;

/*
 * DEBUG_LL port encoding stored into the UART1 scratchpad register by
 * decomp_setup in uncompress.h
 */
pub const OMAP1UART1: u32 = 11;
pub const OMAP1UART2: u32 = 12;
pub const OMAP1UART3: u32 = 13;

// This declaration is present when compiling non-assembler code.
extern "C" {
    pub fn omap_serial_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
