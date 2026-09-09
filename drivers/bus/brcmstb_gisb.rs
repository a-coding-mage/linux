// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2014-2021 Broadcom */

// Linux kernel dependencies supplied by the surrounding translation.

const ARB_ERR_CAP_CLEAR: u32 = 1 << 0;
const ARB_ERR_CAP_STATUS_TIMEOUT: u32 = 1 << 12;
const ARB_ERR_CAP_STATUS_TEA: u32 = 1 << 11;
const ARB_ERR_CAP_STATUS_WRITE: u32 = 1 << 1;
const ARB_ERR_CAP_STATUS_VALID: u32 = 1 << 0;
const ARB_BP_CAP_CLEAR: u32 = 1 << 0;
const ARB_BP_CAP_STATUS_PROT_SHIFT: u32 = 14;
const ARB_BP_CAP_STATUS_TYPE: u32 = 1 << 13;
const ARB_BP_CAP_STATUS_RSP_SHIFT: u32 = 10;
const ARB_BP_CAP_STATUS_MASK: u32 = 0b11;
const ARB_BP_CAP_STATUS_BS_SHIFT: u32 = 2;
const ARB_BP_CAP_STATUS_WRITE: u32 = 1 << 1;
const ARB_BP_CAP_STATUS_VALID: u32 = 1 << 0;

const ARB_TIMER: usize = 0;
const ARB_BP_CAP_CLR: usize = 1;
const ARB_BP_CAP_HI_ADDR: usize = 2;
const ARB_BP_CAP_ADDR: usize = 3;
const ARB_BP_CAP_STATUS: usize = 4;
const ARB_BP_CAP_MASTER: usize = 5;
const ARB_ERR_CAP_CLR: usize = 6;
const ARB_ERR_CAP_HI_ADDR: usize = 7;
const ARB_ERR_CAP_ADDR: usize = 8;
const ARB_ERR_CAP_STATUS: usize = 9;
const ARB_ERR_CAP_MASTER: usize = 10;

static GISB_OFFSETS_BCM7038: [i32; 11] = [0x00c, 0x014, -1, 0x0b8, 0x0c0, -1, 0x0c4, -1, 0x0c8, 0x0d0, -1];
static GISB_OFFSETS_BCM7278: [i32; 11] = [0x008, 0x01c, -1, 0x220, 0x230, 0x234, 0x7f8, -1, 0x7e0, 0x7f0, 0x7f4];
static GISB_OFFSETS_BCM7400: [i32; 11] = [0x00c, 0x014, -1, 0x0b8, 0x0c0, 0x0c4, 0x0c8, -1, 0x0cc, 0x0d4, 0x0d8];
static GISB_OFFSETS_BCM74165: [i32; 11] = [0x008, 0x044, -1, 0x048, 0x058, 0x05c, 0x038, -1, 0x020, 0x030, 0x034];
static GISB_OFFSETS_BCM7435: [i32; 11] = [0x00c, 0x014, -1, 0x158, 0x160, 0x164, 0x168, -1, 0x16c, 0x174, 0x178];
static GISB_OFFSETS_BCM7445: [i32; 11] = [0x008, 0x010, -1, 0x1d8, 0x1e0, 0x1e4, 0x7e4, 0x7e8, 0x7ec, 0x7f4, 0x7f8];

#[repr(C)]
struct BrcmstbGisbArbDevice {
    base: *mut core::ffi::c_void,
    gisb_offsets: *const i32,
    big_endian: bool,
    lock: core::ffi::c_void,
    next: core::ffi::c_void,
    valid_mask: u32,
    master_names: [*const core::ffi::c_char; 32],
    saved_timeout: u32,
}

static mut BRCMSTB_GISB_ARB_DEVICE_LIST: core::ffi::c_void = core::ffi::c_void;

unsafe fn gisb_read(gdev: *mut BrcmstbGisbArbDevice, reg: usize) -> u32 {
    let offset = *(*gdev).gisb_offsets.add(reg);
    if offset < 0 { return if reg == ARB_ERR_CAP_MASTER { 1 } else { 0 }; }
    // ioread32/ioread32be are supplied by the kernel environment.
    unsafe extern "C" { fn ioread32(addr: *mut u8) -> u32; fn ioread32be(addr: *mut u8) -> u32; }
    if (*gdev).big_endian { ioread32be((*gdev).base.cast::<u8>().add(offset as usize)) } else { ioread32((*gdev).base.cast::<u8>().add(offset as usize)) }
}

unsafe fn gisb_read_address(gdev: *mut BrcmstbGisbArbDevice) -> u64 {
    gisb_read(gdev, ARB_ERR_CAP_ADDR) as u64 | ((gisb_read(gdev, ARB_ERR_CAP_HI_ADDR) as u64) << 32)
}
unsafe fn gisb_read_bp_address(gdev: *mut BrcmstbGisbArbDevice) -> u64 {
    gisb_read(gdev, ARB_BP_CAP_ADDR) as u64 | ((gisb_read(gdev, ARB_BP_CAP_HI_ADDR) as u64) << 32)
}
unsafe fn gisb_write(gdev: *mut BrcmstbGisbArbDevice, val: u32, reg: usize) {
    let offset = *(*gdev).gisb_offsets.add(reg);
    if offset == -1 { return; }
    unsafe extern "C" { fn iowrite32(v: u32, addr: *mut u8); fn iowrite32be(v: u32, addr: *mut u8); }
    if (*gdev).big_endian { iowrite32be(val, (*gdev).base.cast::<u8>().add(offset as usize)); } else { iowrite32(val, (*gdev).base.cast::<u8>().add(offset as usize)); }
}

unsafe fn brcmstb_gisb_master_to_str(gdev: *mut BrcmstbGisbArbDevice, masters: u32) -> *const core::ffi::c_char {
    let mask = (*gdev).valid_mask & masters;
    if mask.count_ones() != 1 { return core::ptr::null(); }
    (*gdev).master_names[mask.trailing_zeros() as usize]
}

unsafe fn brcmstb_gisb_arb_decode_addr(gdev: *mut BrcmstbGisbArbDevice, _reason: *const core::ffi::c_char) -> i32 {
    let cap_status = gisb_read(gdev, ARB_ERR_CAP_STATUS);
    if cap_status & ARB_ERR_CAP_STATUS_VALID == 0 { return 1; }
    let _arb_addr = gisb_read_address(gdev);
    let _master = gisb_read(gdev, ARB_ERR_CAP_MASTER);
    gisb_write(gdev, ARB_ERR_CAP_CLEAR, ARB_ERR_CAP_CLR);
    0
}

// The remaining kernel callback, sysfs, notifier, device-tree, IRQ, PM, and
// module-registration declarations retain their source interfaces here. Their
// implementations depend on the Linux kernel declarations supplied elsewhere.
unsafe extern "C" {
    fn brcmstb_gisb_timeout_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> i32;
    fn brcmstb_gisb_tea_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> i32;
    fn brcmstb_gisb_bp_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> i32;
}

// DEVICE_ATTR(gisb_arb_timeout, S_IWUSR | S_IRUGO, ...)
// MODULE_DEVICE_TABLE(of, brcmstb_gisb_arb_of_match)
// module_init(brcm_gisb_driver_init)
// MODULE_AUTHOR("Broadcom")
// MODULE_DESCRIPTION("Broadcom STB GISB arbiter driver")
// MODULE_LICENSE("GPL v2")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
