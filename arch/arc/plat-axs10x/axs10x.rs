// SPDX-License-Identifier: GPL-2.0-only
/*
 * AXS101/AXS103 Software Development Platform
 *
 * Copyright (C) 2013-15 Synopsys, Inc. (www.synopsys.com)
 */

// Linux headers and build-time configuration supplied by the surrounding tree.

const AXS_MB_CGU: usize = 0xE0010000;
const AXS_MB_CREG: usize = 0xE0011000;
const CREG_MB_IRQ_MUX: usize = AXS_MB_CREG + 0x214;
const CREG_MB_SW_RESET: usize = AXS_MB_CREG + 0x220;
const CREG_MB_VER: usize = AXS_MB_CREG + 0x230;
const CREG_MB_CONFIG: usize = AXS_MB_CREG + 0x234;
const AXC001_CREG: usize = 0xF0001000;
const AXC001_GPIO_INTC: usize = 0xF0003000;

unsafe fn axs10x_enable_gpio_intc_wire() {
    const GPIO_INTEN: usize = AXC001_GPIO_INTC + 0x30;
    const GPIO_INTMASK: usize = AXC001_GPIO_INTC + 0x34;
    const GPIO_INTTYPE_LEVEL: usize = AXC001_GPIO_INTC + 0x38;
    const GPIO_INT_POLARITY: usize = AXC001_GPIO_INTC + 0x3c;
    const MB_TO_GPIO_IRQ: u32 = 12;

    iowrite32(!(1u32 << MB_TO_GPIO_IRQ), GPIO_INTMASK as *mut core::ffi::c_void);
    iowrite32(0, GPIO_INTTYPE_LEVEL as *mut core::ffi::c_void);
    iowrite32(!0, GPIO_INT_POLARITY as *mut core::ffi::c_void);
    iowrite32(1u32 << MB_TO_GPIO_IRQ, GPIO_INTEN as *mut core::ffi::c_void);
}

unsafe fn axs10x_print_board_ver(creg: u32, str_: *const core::ffi::c_char) {
    let val = ioread32(creg as usize as *mut core::ffi::c_void);
    let d = val & 0x1f;
    let m = (val >> 5) & 0xf;
    let y = (val >> 9) & 0xfff;
    pr_info("AXS: %s FPGA Date: %u-%u-%u\n", str_, d, m, y);
}

unsafe fn axs10x_early_init() {
    let mb_rev: i32 = if ioread32(CREG_MB_CONFIG as *mut core::ffi::c_void) & (1 << 28) != 0 { 3 } else { 2 };
    axs10x_enable_gpio_intc_wire();
    let mut mb = [0i8; 32];
    scnprintf(mb.as_mut_ptr(), 32, "MainBoard v%d\0", mb_rev);
    axs10x_print_board_ver(CREG_MB_VER as u32, mb.as_ptr());
}

#[cfg(CONFIG_AXS101)]
const CREG_CPU_ADDR_770: usize = AXC001_CREG + 0x20;
#[cfg(CONFIG_AXS101)]
const CREG_CPU_ADDR_TUNN: usize = AXC001_CREG + 0x60;
#[cfg(CONFIG_AXS101)]
const CREG_CPU_ADDR_770_UPD: usize = AXC001_CREG + 0x34;
#[cfg(CONFIG_AXS101)]
const CREG_CPU_ADDR_TUNN_UPD: usize = AXC001_CREG + 0x74;
#[cfg(CONFIG_AXS101)]
const CREG_CPU_ARC770_IRQ_MUX: usize = AXC001_CREG + 0x114;
#[cfg(CONFIG_AXS101)]
const CREG_CPU_GPIO_UART_MUX: usize = AXC001_CREG + 0x120;

#[cfg(CONFIG_AXS101)]
#[repr(C)]
struct aperture { slave_sel: u32, slave_off: u32, pad: u32 }

#[cfg(CONFIG_AXS101)]
const AXC001_SLV_NONE: u32 = 0;
#[cfg(CONFIG_AXS101)]
const AXC001_SLV_DDR_PORT0: u32 = 1;
#[cfg(CONFIG_AXS101)]
const AXC001_SLV_SRAM: u32 = 2;
#[cfg(CONFIG_AXS101)]
const AXC001_SLV_AXI_TUNNEL: u32 = 3;
#[cfg(CONFIG_AXS101)]
const AXC001_SLV_AXI2APB: u32 = 6;
#[cfg(CONFIG_AXS101)]
const AXC001_SLV_DDR_PORT1: u32 = 7;
#[cfg(CONFIG_AXS101)]
const AXS_MB_SLV_NONE: u32 = 0;
#[cfg(CONFIG_AXS101)]
const AXS_MB_SLV_AXI_TUNNEL_CPU: u32 = 1;
#[cfg(CONFIG_AXS101)]
const AXS_MB_SLV_AXI_TUNNEL_HAPS: u32 = 2;
#[cfg(CONFIG_AXS101)]
const AXS_MB_SLV_SRAM: u32 = 3;
#[cfg(CONFIG_AXS101)]
const AXS_MB_SLV_CONTROL: u32 = 4;
#[cfg(CONFIG_AXS101)]
const AXS_MB_MST_TUNNEL_CPU: usize = 0;
#[cfg(CONFIG_AXS101)]
const AXS_MB_MST_USB_OHCI: usize = 10;

#[cfg(CONFIG_AXS101)]
const axc001_memmap: [aperture; 16] = [
    aperture{slave_sel:3,slave_off:0,pad:0}, aperture{slave_sel:3,slave_off:1,pad:0}, aperture{slave_sel:2,slave_off:0,pad:0},
    aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0},
    aperture{slave_sel:1,slave_off:0,pad:0}, aperture{slave_sel:1,slave_off:1,pad:0}, aperture{slave_sel:1,slave_off:2,pad:0}, aperture{slave_sel:1,slave_off:3,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:3,slave_off:13,pad:0}, aperture{slave_sel:3,slave_off:14,pad:0}, aperture{slave_sel:6,slave_off:0,pad:0}];
#[cfg(CONFIG_AXS101)]
const axc001_axi_tunnel_memmap: [aperture; 16] = axc001_memmap;
#[cfg(CONFIG_AXS101)]
const axs_mb_memmap: [aperture; 16] = [
    aperture{slave_sel:3,slave_off:0,pad:0}, aperture{slave_sel:3,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:1,slave_off:8,pad:0}, aperture{slave_sel:1,slave_off:9,pad:0}, aperture{slave_sel:1,slave_off:10,pad:0}, aperture{slave_sel:1,slave_off:11,pad:0}, aperture{slave_sel:0,slave_off:0,pad:0}, aperture{slave_sel:2,slave_off:13,pad:0}, aperture{slave_sel:4,slave_off:0,pad:0}, aperture{slave_sel:1,slave_off:15,pad:0}];

#[cfg(CONFIG_AXS101)]
unsafe fn axs101_set_memmap(base: *mut core::ffi::c_void, map: *const aperture) {
    let mut slave_select = 0u32;
    let mut slave_offset = 0u32;
    for i in 0..8usize { slave_select |= (*map.add(i)).slave_sel << (i << 2); slave_offset |= (*map.add(i)).slave_off << (i << 2); }
    iowrite32(slave_select, (base as usize) as *mut core::ffi::c_void);
    iowrite32(slave_offset, (base as usize + 8) as *mut core::ffi::c_void);
    slave_select = 0; slave_offset = 0;
    for i in 0..8usize { slave_select |= (*map.add(i + 8)).slave_sel << (i << 2); slave_offset |= (*map.add(i + 8)).slave_off << (i << 2); }
    iowrite32(slave_select, (base as usize + 4) as *mut core::ffi::c_void);
    iowrite32(slave_offset, (base as usize + 12) as *mut core::ffi::c_void);
}

#[cfg(CONFIG_AXS101)]
unsafe fn axs101_early_init() {
    axs101_set_memmap(CREG_CPU_ADDR_770 as *mut _, axc001_memmap.as_ptr());
    iowrite32(1, CREG_CPU_ADDR_770_UPD as *mut core::ffi::c_void);
    axs101_set_memmap(CREG_CPU_ADDR_TUNN as *mut _, axc001_axi_tunnel_memmap.as_ptr());
    for i in AXS_MB_MST_TUNNEL_CPU..=AXS_MB_MST_USB_OHCI { axs101_set_memmap((AXS_MB_CREG + (i << 4)) as *mut _, axs_mb_memmap.as_ptr()); }
    iowrite32(0x3ff, (AXS_MB_CREG + 0x100) as *mut core::ffi::c_void);
    iowrite32(1, CREG_CPU_GPIO_UART_MUX as *mut core::ffi::c_void);
    iowrite32(1, CREG_MB_IRQ_MUX as *mut core::ffi::c_void);
    iowrite32(0x18, CREG_MB_SW_RESET as *mut core::ffi::c_void);
    iowrite32(0x52, CREG_CPU_ARC770_IRQ_MUX as *mut core::ffi::c_void);
    axs10x_early_init();
}

#[cfg(CONFIG_AXS103)]
const AXC003_CREG: usize = 0xF0001000;
#[cfg(CONFIG_AXS103)]
const CREG_CPU_AXI_M0_IRQ_MUX: usize = AXC003_CREG + 0x440;
#[cfg(CONFIG_AXS103)]
const CREG_CPU_TUN_IO_CTRL: usize = AXC003_CREG + 0x494;
#[cfg(CONFIG_AXS103)]
unsafe fn axs103_early_init() {
    iowrite32(1, (AXC003_CREG + 0x480) as *mut _);
    iowrite32(0x00100000 | 0x000C0000 | 0x00003322, CREG_CPU_TUN_IO_CTRL as *mut _);
    iowrite32(12, (CREG_CPU_AXI_M0_IRQ_MUX + 4) as *mut _);
    iowrite32(1, CREG_MB_IRQ_MUX as *mut _);
    axs10x_print_board_ver((AXC003_CREG + 4088) as u32, b"AXC003 CPU Card\0".as_ptr() as *const _);
    axs10x_early_init();
}

#[cfg(CONFIG_AXS103)]
static mut coware_swa_pid_offset: [u8; 0] = [];
#[cfg(CONFIG_AXS103)]
static mut coware_swa_comm_offset: [u8; 0] = [];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
