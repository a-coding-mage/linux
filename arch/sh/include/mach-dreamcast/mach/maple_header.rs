/* SPDX-License-Identifier: GPL-2.0 */

/* `HZ` and `PAGE_SHIFT` are supplied by the surrounding kernel translation. */
pub const MAPLE_PORTS: u32 = 4;
pub const MAPLE_PNP_INTERVAL: u32 = HZ;
pub const MAPLE_MAXPACKETS: u32 = 8;
pub const MAPLE_DMA_ORDER: u32 = 14;
pub const MAPLE_DMA_SIZE: u32 = 1u32 << MAPLE_DMA_ORDER;
pub const MAPLE_DMA_PAGES: u32 = if MAPLE_DMA_ORDER > PAGE_SHIFT {
    MAPLE_DMA_ORDER - PAGE_SHIFT
} else {
    0
};

/* Maple Bus registers */
pub const MAPLE_BASE: u32 = 0xa05f6c00;
pub const MAPLE_DMAADDR: u32 = MAPLE_BASE + 0x04;
pub const MAPLE_TRIGTYPE: u32 = MAPLE_BASE + 0x10;
pub const MAPLE_ENABLE: u32 = MAPLE_BASE + 0x14;
pub const MAPLE_STATE: u32 = MAPLE_BASE + 0x18;
pub const MAPLE_SPEED: u32 = MAPLE_BASE + 0x80;
pub const MAPLE_RESET: u32 = MAPLE_BASE + 0x8c;

pub const MAPLE_MAGIC: u32 = 0x6155404f;
pub const MAPLE_2MBPS: u32 = 0;

pub const fn MAPLE_TIMEOUT(n: u32) -> u32 {
    n << 15
}

/* Function codes */
pub const MAPLE_FUNC_CONTROLLER: u32 = 0x001;
pub const MAPLE_FUNC_MEMCARD: u32 = 0x002;
pub const MAPLE_FUNC_LCD: u32 = 0x004;
pub const MAPLE_FUNC_CLOCK: u32 = 0x008;
pub const MAPLE_FUNC_MICROPHONE: u32 = 0x010;
pub const MAPLE_FUNC_ARGUN: u32 = 0x020;
pub const MAPLE_FUNC_KEYBOARD: u32 = 0x040;
pub const MAPLE_FUNC_LIGHTGUN: u32 = 0x080;
pub const MAPLE_FUNC_PURUPURU: u32 = 0x100;
pub const MAPLE_FUNC_MOUSE: u32 = 0x200;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
