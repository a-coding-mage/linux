/* SPDX-License-Identifier: GPL-2.0 */

/*
 * These bits define which access sizes a platform can support, rather
 * than the maximal access size.  So, if your platform can do 16-bit
 * and 32-bit accesses to the SMC91x device, but not 8-bit, set both
 * SMC91X_USE_16BIT and SMC91X_USE_32BIT.
 *
 * The SMC91x driver requires at least one of SMC91X_USE_8BIT or
 * SMC91X_USE_16BIT to be supported - just setting SMC91X_USE_32BIT is
 * an invalid configuration.
 */
pub const SMC91X_USE_8BIT: u32 = 1 << 0;
pub const SMC91X_USE_16BIT: u32 = 1 << 1;
pub const SMC91X_USE_32BIT: u32 = 1 << 2;

pub const SMC91X_NOWAIT: u32 = 1 << 3;

/* two bits for IO_SHIFT, let's hope later designs will keep this sane */
pub const SMC91X_IO_SHIFT_0: u32 = 0 << 4;
pub const SMC91X_IO_SHIFT_1: u32 = 1 << 4;
pub const SMC91X_IO_SHIFT_2: u32 = 2 << 4;
pub const SMC91X_IO_SHIFT_3: u32 = 3 << 4;

#[inline]
pub const fn SMC91X_IO_SHIFT(x: u32) -> u32 {
    (x >> 4) & 0x3
}

pub const SMC91X_USE_DMA: u32 = 1 << 6;

pub const RPC_LED_100_10: u32 = 0x00; /* LED = 100Mbps OR's with 10Mbps link detect */
pub const RPC_LED_RES: u32 = 0x01; /* LED = Reserved */
pub const RPC_LED_10: u32 = 0x02; /* LED = 10Mbps link detect */
pub const RPC_LED_FD: u32 = 0x03; /* LED = Full Duplex Mode */
pub const RPC_LED_TX_RX: u32 = 0x04; /* LED = TX or RX packet occurred */
pub const RPC_LED_100: u32 = 0x05; /* LED = 100Mbps link detect */
pub const RPC_LED_TX: u32 = 0x06; /* LED = TX packet occurred */
pub const RPC_LED_RX: u32 = 0x07; /* LED = RX packet occurred */

#[repr(C)]
pub struct smc91x_platdata {
    pub flags: core::ffi::c_ulong,
    pub leda: u8,
    pub ledb: u8,
    pub pxa_u16_align4: bool, /* PXA buggy u16 writes on 4*n+2 addresses */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
