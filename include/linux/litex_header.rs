/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common LiteX header providing
 * helper functions for accessing CSRs.
 *
 * Copyright (C) 2019-2020 Antmicro <www.antmicro.com>
 */

// C dependency: <linux/io.h>
// The following symbols are supplied by the surrounding Linux bindings.
extern "C" {
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn cpu_to_le32(value: u32) -> u32;
    fn le32_to_cpu(value: u32) -> u32;
}

unsafe fn _write_litex_subregister(val: u32, addr: *mut core::ffi::c_void) {
    writel(cpu_to_le32(val), addr);
}

unsafe fn _read_litex_subregister(addr: *mut core::ffi::c_void) -> u32 {
    le32_to_cpu(readl(addr))
}

/*
 * LiteX SoC Generator, depending on the configuration, can split a single
 * logical CSR (Control&Status Register) into a series of consecutive physical
 * registers.
 *
 * For example, in the configuration with 8-bit CSR Bus, a 32-bit aligned,
 * 32-bit wide logical CSR will be laid out as four 32-bit physical
 * subregisters, each one containing one byte of meaningful data.
 *
 * For Linux support, upstream LiteX enforces a 32-bit wide CSR bus, which
 * means that only larger-than-32-bit CSRs will be split across multiple
 * subregisters (e.g., a 64-bit CSR will be spread across two consecutive
 * 32-bit subregisters).
 *
 * For details see: https://github.com/enjoy-digital/litex/wiki/CSR-Bus
 */

unsafe fn litex_write8(reg: *mut core::ffi::c_void, val: u8) {
    _write_litex_subregister(val as u32, reg);
}

unsafe fn litex_write16(reg: *mut core::ffi::c_void, val: u16) {
    _write_litex_subregister(val as u32, reg);
}

unsafe fn litex_write32(reg: *mut core::ffi::c_void, val: u32) {
    _write_litex_subregister(val, reg);
}

unsafe fn litex_write64(reg: *mut core::ffi::c_void, val: u64) {
    _write_litex_subregister((val >> 32) as u32, reg);
    _write_litex_subregister(val as u32, (reg as *mut u8).add(4) as *mut core::ffi::c_void);
}

unsafe fn litex_read8(reg: *mut core::ffi::c_void) -> u8 {
    _read_litex_subregister(reg) as u8
}

unsafe fn litex_read16(reg: *mut core::ffi::c_void) -> u16 {
    _read_litex_subregister(reg) as u16
}

unsafe fn litex_read32(reg: *mut core::ffi::c_void) -> u32 {
    _read_litex_subregister(reg)
}

unsafe fn litex_read64(reg: *mut core::ffi::c_void) -> u64 {
    ((_read_litex_subregister(reg) as u64) << 32)
        | _read_litex_subregister((reg as *mut u8).add(4) as *mut core::ffi::c_void) as u64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
