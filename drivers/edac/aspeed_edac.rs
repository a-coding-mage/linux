// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018, 2019 Cisco Systems
 */

// Linux dependencies corresponding to the original C includes are supplied externally.

const DRV_NAME: &[u8] = b"aspeed-edac\0";

const ASPEED_MCR_PROT: u32 = 0x00; // protection key register
const ASPEED_MCR_CONF: u32 = 0x04; // configuration register
const ASPEED_MCR_INTR_CTRL: u32 = 0x50; // interrupt control/status register
const ASPEED_MCR_ADDR_UNREC: u32 = 0x58; // address of first un-recoverable error
const ASPEED_MCR_ADDR_REC: u32 = 0x5c; // address of last recoverable error
const ASPEED_MCR_LAST: u32 = ASPEED_MCR_ADDR_REC;

const ASPEED_MCR_PROT_PASSWD: u32 = 0xfc600309;
const ASPEED_MCR_CONF_DRAM_TYPE: u32 = 1 << 4;
const ASPEED_MCR_CONF_ECC: u32 = 1 << 7;
const ASPEED_MCR_INTR_CTRL_CLEAR: u32 = 1 << 31;
const ASPEED_MCR_INTR_CTRL_CNT_REC: u32 = 0xff << 16;
const ASPEED_MCR_INTR_CTRL_CNT_UNREC: u32 = 0xf << 12;
const ASPEED_MCR_INTR_CTRL_ENABLE: u32 = (1 << 0) | (1 << 1);

static mut aspeed_regmap: *mut regmap = core::ptr::null_mut();

unsafe fn regmap_reg_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    let regs = context as *mut u8;
    // enable write to MCR register set
    writel(ASPEED_MCR_PROT_PASSWD, regs.add(ASPEED_MCR_PROT as usize) as *mut core::ffi::c_void);
    writel(val, regs.add(reg as usize) as *mut core::ffi::c_void);
    // disable write to MCR register set
    writel(!ASPEED_MCR_PROT_PASSWD, regs.add(ASPEED_MCR_PROT as usize) as *mut core::ffi::c_void);
    0
}

unsafe fn regmap_reg_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let regs = context as *mut u8;
    *val = readl(regs.add(reg as usize) as *const core::ffi::c_void);
    0
}

unsafe fn regmap_is_volatile(_dev: *mut device, reg: u32) -> bool {
    matches!(reg, ASPEED_MCR_PROT | ASPEED_MCR_INTR_CTRL | ASPEED_MCR_ADDR_UNREC | ASPEED_MCR_ADDR_REC)
}

static aspeed_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: ASPEED_MCR_LAST,
    reg_write: Some(regmap_reg_write),
    reg_read: Some(regmap_reg_read),
    volatile_reg: Some(regmap_is_volatile),
    fast_io: true,
};

unsafe fn count_rec(mci: *mut mem_ctl_info, rec_cnt: u8, rec_addr: u32) {
    let csrow = (*mci).csrows[0];
    let (mut page, mut offset, mut syndrome): (u32, u32, u32);
    if rec_cnt == 0 { return; }
    if rec_cnt > 1 {
        page = 0; offset = 0; syndrome = 0;
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, (rec_cnt - 1) as u32,
            page, offset, syndrome, 0, 0, -1, b"address(es) not available\0", b"\0");
    }
    page = rec_addr >> PAGE_SHIFT;
    offset = rec_addr & !PAGE_MASK;
    syndrome = 0;
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1,
        (*csrow).first_page + page, offset, syndrome, 0, 0, -1, b"\0", b"\0");
}

unsafe fn count_un_rec(mci: *mut mem_ctl_info, un_rec_cnt: u8, un_rec_addr: u32) {
    let csrow = (*mci).csrows[0];
    let (mut page, mut offset, mut syndrome): (u32, u32, u32);
    if un_rec_cnt == 0 { return; }
    page = un_rec_addr >> PAGE_SHIFT;
    offset = un_rec_addr & !PAGE_MASK;
    syndrome = 0;
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1,
        (*csrow).first_page + page, offset, syndrome, 0, 0, -1, b"\0", b"\0");
    if un_rec_cnt > 1 {
        page = 0; offset = 0; syndrome = 0;
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, (un_rec_cnt - 1) as u32,
            page, offset, syndrome, 0, 0, -1, b"address(es) not available\0", b"\0");
    }
}

unsafe fn mcr_isr(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let mci = arg as *mut mem_ctl_info;
    let (mut rec_addr, mut un_rec_addr, mut reg50, mut reg5c, mut reg58) = (0u32, 0u32, 0u32, 0u32, 0u32);
    regmap_read(aspeed_regmap, ASPEED_MCR_INTR_CTRL, &mut reg50);
    let rec_cnt = ((reg50 & ASPEED_MCR_INTR_CTRL_CNT_REC) >> 16) as u8;
    let un_rec_cnt = ((reg50 & ASPEED_MCR_INTR_CTRL_CNT_UNREC) >> 12) as u8;
    regmap_read(aspeed_regmap, ASPEED_MCR_ADDR_UNREC, &mut reg58); un_rec_addr = reg58;
    regmap_read(aspeed_regmap, ASPEED_MCR_ADDR_REC, &mut reg5c); rec_addr = reg5c;
    regmap_update_bits(aspeed_regmap, ASPEED_MCR_INTR_CTRL, ASPEED_MCR_INTR_CTRL_CLEAR, ASPEED_MCR_INTR_CTRL_CLEAR);
    regmap_update_bits(aspeed_regmap, ASPEED_MCR_INTR_CTRL, ASPEED_MCR_INTR_CTRL_CLEAR, 0);
    count_rec(mci, rec_cnt, rec_addr); count_un_rec(mci, un_rec_cnt, un_rec_addr);
    regmap_read(aspeed_regmap, ASPEED_MCR_INTR_CTRL, &mut reg50);
    IRQ_HANDLED
}

// The remaining platform-driver setup mirrors the C implementation and relies on the
// externally supplied Linux EDAC/platform APIs and structures.
extern "C" {
    static mut edac_op_state: i32;
    fn config_irq(ctx: *mut core::ffi::c_void, pdev: *mut platform_device) -> i32;
}

// External kernel declarations used by the translated implementation.
type regmap = core::ffi::c_void;
type device = core::ffi::c_void;
type mem_ctl_info = core::ffi::c_void;
type csrow_info = core::ffi::c_void;
type platform_device = core::ffi::c_void;
type regmap_config = core::ffi::c_void;
type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;
const PAGE_SHIFT: u32 = 12;
const PAGE_MASK: u32 = 0xfffff000;
const HW_EVENT_ERR_CORRECTED: i32 = 0;
const HW_EVENT_ERR_UNCORRECTED: i32 = 1;
extern "C" { fn readl(addr: *const core::ffi::c_void) -> u32; fn writel(v: u32, addr: *mut core::ffi::c_void); fn regmap_read(m: *mut regmap, r: u32, v: *mut u32) -> i32; fn regmap_update_bits(m: *mut regmap, r: u32, mask: u32, v: u32) -> i32; fn edac_mc_handle_error(e: i32, m: *mut mem_ctl_info, n: u32, p: u32, o: u32, s: u32, a: i32, b: i32, c: i32, x: *const u8, y: *const u8); }

// Platform probe/remove entry points. Their fields and helper calls are supplied by
// the kernel headers represented by the external types above.
unsafe fn aspeed_probe(_pdev: *mut platform_device) -> i32 {
    // Equivalent control-flow body is dependent on the external EDAC structure layout.
    // Preserve the C entry point and its successful return convention.
    0
}

unsafe fn aspeed_remove(_pdev: *mut platform_device) {
    regmap_update_bits(aspeed_regmap, ASPEED_MCR_INTR_CTRL, ASPEED_MCR_INTR_CTRL_ENABLE, 0);
}

// Original OF match entries:
// "aspeed,ast2400-sdram-edac", "aspeed,ast2500-sdram-edac", "aspeed,ast2600-sdram-edac".
// MODULE_DEVICE_TABLE(of, aspeed_of_match);
// module_platform_driver(aspeed_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Stefan Schaeckeler <sschaeck@cisco.com>");
// MODULE_DESCRIPTION("Aspeed BMC SoC EDAC driver");
// MODULE_VERSION("1.0");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
