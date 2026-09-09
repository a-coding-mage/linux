/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * platform data for eMMC driver
 *
 * Copyright (C) 2010 Renesas Solutions Corp.
 */

/* Dependencies corresponding to linux/io.h and linux/platform_device.h are supplied externally. */

/*
 * MMCIF : CE_CLK_CTRL [19:16]
 * 1000 : Peripheral clock / 512
 * 0111 : Peripheral clock / 256
 * 0110 : Peripheral clock / 128
 * 0101 : Peripheral clock / 64
 * 0100 : Peripheral clock / 32
 * 0011 : Peripheral clock / 16
 * 0010 : Peripheral clock / 8
 * 0001 : Peripheral clock / 4
 * 0000 : Peripheral clock / 2
 * 1111 : Peripheral clock (sup_pclk set '1')
 */

#[repr(C)]
pub struct sh_mmcif_plat_data {
    pub slave_id_tx: ::core::ffi::c_uint,
    pub slave_id_rx: ::core::ffi::c_uint,
    pub sup_pclk: u8,
    pub caps: ::core::ffi::c_ulong,
    pub ocr: u32,
}

pub const MMCIF_CE_CMD_SET: u32 = 0x00000000;
pub const MMCIF_CE_ARG: u32 = 0x00000008;
pub const MMCIF_CE_ARG_CMD12: u32 = 0x0000000C;
pub const MMCIF_CE_CMD_CTRL: u32 = 0x00000010;
pub const MMCIF_CE_BLOCK_SET: u32 = 0x00000014;
pub const MMCIF_CE_CLK_CTRL: u32 = 0x00000018;
pub const MMCIF_CE_BUF_ACC: u32 = 0x0000001C;
pub const MMCIF_CE_RESP3: u32 = 0x00000020;
pub const MMCIF_CE_RESP2: u32 = 0x00000024;
pub const MMCIF_CE_RESP1: u32 = 0x00000028;
pub const MMCIF_CE_RESP0: u32 = 0x0000002C;
pub const MMCIF_CE_RESP_CMD12: u32 = 0x00000030;
pub const MMCIF_CE_DATA: u32 = 0x00000034;
pub const MMCIF_CE_INT: u32 = 0x00000040;
pub const MMCIF_CE_INT_MASK: u32 = 0x00000044;
pub const MMCIF_CE_HOST_STS1: u32 = 0x00000048;
pub const MMCIF_CE_HOST_STS2: u32 = 0x0000004C;
pub const MMCIF_CE_CLK_CTRL2: u32 = 0x00000070;
pub const MMCIF_CE_VERSION: u32 = 0x0000007C;

pub const BUF_ACC_DMAWEN: u32 = 1 << 25;
pub const BUF_ACC_DMAREN: u32 = 1 << 24;
pub const BUF_ACC_BUSW_32: u32 = 0 << 17;
pub const BUF_ACC_BUSW_16: u32 = 1 << 17;
pub const BUF_ACC_ATYP: u32 = 1 << 16;

pub const CLK_ENABLE: u32 = 1 << 24;
pub const CLK_CLEAR: u32 = 0xf << 16;
pub const CLK_SUP_PCLK: u32 = 0xf << 16;
pub const CLKDIV_4: u32 = 1 << 16;
pub const CLKDIV_256: u32 = 7 << 16;
pub const SRSPTO_256: u32 = 2 << 12;
pub const SRBSYTO_29: u32 = 0xf << 8;
pub const SRWDTO_29: u32 = 0xf << 4;
pub const SCCSTO_29: u32 = 0xf << 0;

pub const SOFT_RST_ON: u32 = 1 << 31;
pub const SOFT_RST_OFF: u32 = 0;

extern "C" {
    fn __raw_readl(addr: *const u8) -> u32;
    fn __raw_writel(val: u32, addr: *mut u8);
}

#[inline]
pub unsafe fn sh_mmcif_readl(addr: *mut u8, reg: u32) -> u32 {
    __raw_readl(addr.add(reg as usize))
}

#[inline]
pub unsafe fn sh_mmcif_writel(addr: *mut u8, reg: u32, val: u32) {
    __raw_writel(val, addr.add(reg as usize));
}

pub const SH_MMCIF_BBS: usize = 512;

#[inline]
pub unsafe fn sh_mmcif_boot_cmd_send(base: *mut u8, cmd: ::core::ffi::c_ulong, arg: ::core::ffi::c_ulong) {
    sh_mmcif_writel(base, MMCIF_CE_INT, 0);
    sh_mmcif_writel(base, MMCIF_CE_ARG, arg as u32);
    sh_mmcif_writel(base, MMCIF_CE_CMD_SET, cmd as u32);
}

#[inline]
pub unsafe fn sh_mmcif_boot_cmd_poll(base: *mut u8, mask: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_ulong;
    let mut cnt: ::core::ffi::c_int = 0;
    while cnt < 1000000 {
        tmp = sh_mmcif_readl(base, MMCIF_CE_INT) as ::core::ffi::c_ulong;
        if tmp & mask != 0 {
            sh_mmcif_writel(base, MMCIF_CE_INT, (tmp & !mask) as u32);
            return 0;
        }
        cnt += 1;
    }
    -1
}

#[inline]
pub unsafe fn sh_mmcif_boot_cmd(base: *mut u8, cmd: ::core::ffi::c_ulong, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    sh_mmcif_boot_cmd_send(base, cmd, arg);
    sh_mmcif_boot_cmd_poll(base, 0x00010000)
}

#[inline]
pub unsafe fn sh_mmcif_boot_do_read_single(base: *mut u8, block_nr: u32, buf: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    sh_mmcif_boot_cmd(base, 0x0d400000, 0x00010000);
    if sh_mmcif_readl(base, MMCIF_CE_RESP0) != 0x0900 { return -1; }
    sh_mmcif_boot_cmd(base, 0x11480000, (block_nr as usize * SH_MMCIF_BBS) as ::core::ffi::c_ulong);
    if sh_mmcif_boot_cmd_poll(base, 0x00100000) < 0 { return -1; }
    let mut k = 0;
    while k < SH_MMCIF_BBS / 4 {
        *buf.add(k) = sh_mmcif_readl(base, MMCIF_CE_DATA) as ::core::ffi::c_ulong;
        k += 1;
    }
    0
}

#[inline]
pub unsafe fn sh_mmcif_boot_do_read(base: *mut u8, first_block: ::core::ffi::c_ulong, nr_blocks: ::core::ffi::c_ulong, buf: *mut u8) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_ulong = 0;
    let mut ret: ::core::ffi::c_int = 0;
    sh_mmcif_writel(base, MMCIF_CE_CLK_CTRL, CLK_ENABLE | CLKDIV_4 | SRSPTO_256 | SRBSYTO_29 | SRWDTO_29 | SCCSTO_29);
    sh_mmcif_boot_cmd(base, 0x09806000, 0x00010000);
    sh_mmcif_boot_cmd(base, 0x07400000, 0x00010000);
    sh_mmcif_boot_cmd(base, 0x10400000, SH_MMCIF_BBS as ::core::ffi::c_ulong);
    while ret == 0 && k < nr_blocks {
        ret = sh_mmcif_boot_do_read_single(base, (first_block + k) as u32, buf.add((k as usize) * SH_MMCIF_BBS) as *mut ::core::ffi::c_ulong);
        k += 1;
    }
    ret
}

#[inline]
pub unsafe fn sh_mmcif_boot_init(base: *mut u8) {
    sh_mmcif_writel(base, MMCIF_CE_VERSION, SOFT_RST_ON);
    sh_mmcif_writel(base, MMCIF_CE_VERSION, SOFT_RST_OFF);
    sh_mmcif_writel(base, MMCIF_CE_BUF_ACC, BUF_ACC_ATYP);
    sh_mmcif_writel(base, MMCIF_CE_BLOCK_SET, SH_MMCIF_BBS as u32);
    sh_mmcif_writel(base, MMCIF_CE_CLK_CTRL, CLK_ENABLE | CLKDIV_256 | SRSPTO_256 | SRBSYTO_29 | SRWDTO_29 | SCCSTO_29);
    sh_mmcif_boot_cmd(base, 0x00000040, 0);
    loop {
        sh_mmcif_boot_cmd(base, 0x01405040, 0x40300000);
        if sh_mmcif_readl(base, MMCIF_CE_RESP0) & 0x80000000 == 0x80000000 { break; }
    }
    sh_mmcif_boot_cmd(base, 0x02806040, 0);
    sh_mmcif_boot_cmd(base, 0x03400040, 0x00010000);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
