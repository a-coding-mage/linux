/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright 2017 IBM Corp.
 *
 * This is a source-level Rust translation of the Linux UAPI header.
 */

// Dependency intent: equivalent to <linux/ioctl.h> and <linux/types.h>.

/* Window types */
pub const ASPEED_LPC_CTRL_WINDOW_FLASH: u8 = 1;
pub const ASPEED_LPC_CTRL_WINDOW_MEMORY: u8 = 2;

/*
 * This driver provides a window for the host to access a BMC resource
 * across the BMC <-> Host LPC bus.
 *
 * window_type: The BMC resource that the host will access through the
 * window. BMC flash and BMC RAM.
 *
 * window_id: For each window type there may be multiple windows,
 * these are referenced by ID.
 *
 * flags: Reserved for future use, this field is expected to be
 * zeroed.
 *
 * addr: Address on the host LPC bus that the specified window should
 * be mapped. This address must be power of two aligned.
 *
 * offset: Offset into the BMC window that should be mapped to the
 * host (at addr). This must be a multiple of size.
 *
 * size: The size of the mapping. The smallest possible size is 64K.
 * This must be power of two aligned.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_lpc_ctrl_mapping {
    pub window_type: u8,
    pub window_id: u8,
    pub flags: u16,
    pub addr: u32,
    pub offset: u32,
    pub size: u32,
}

pub const __ASPEED_LPC_CTRL_IOCTL_MAGIC: u8 = 0xb2;

// Equivalent Linux _IOC encoding used by the ioctl declarations below.
const fn __ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << 30) | (size << 16) | (ty << 8) | nr
}

const fn __iowr(ty: u32, nr: u32, size: u32) -> u32 {
    __ioc(3, ty, nr, size)
}

const fn __iow(ty: u32, nr: u32, size: u32) -> u32 {
    __ioc(1, ty, nr, size)
}

pub const ASPEED_LPC_CTRL_IOCTL_GET_SIZE: u32 =
    __iowr(__ASPEED_LPC_CTRL_IOCTL_MAGIC as u32, 0x00, core::mem::size_of::<aspeed_lpc_ctrl_mapping>() as u32);

pub const ASPEED_LPC_CTRL_IOCTL_MAP: u32 =
    __iow(__ASPEED_LPC_CTRL_IOCTL_MAGIC as u32, 0x01, core::mem::size_of::<aspeed_lpc_ctrl_mapping>() as u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
