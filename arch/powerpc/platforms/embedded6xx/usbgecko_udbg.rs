// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/embedded6xx/usbgecko_udbg.c
 *
 * udbg serial input/output routines for the USB Gecko adapter.
 * Copyright (C) 2008-2009 The GameCube Linux Team
 * Copyright (C) 2008,2009 Albert Herranz
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/of_address.h, mm/mmu_decl.h, asm/io.h, asm/udbg.h, asm/fixmap.h,
// and usbgecko_udbg.h.

const EXI_CLK_32MHZ: u32 = 5;
const EXI_CSR: usize = 0x00;
const EXI_CSR_CLKMASK: u32 = 0x7 << 4;
const EXI_CSR_CLK_32MHZ: u32 = EXI_CLK_32MHZ << 4;
const EXI_CSR_CSMASK: u32 = 0x7 << 7;
const EXI_CSR_CS_0: u32 = 0x1 << 7; // Chip Select 001
const EXI_CR: usize = 0x0c;
const EXI_CR_TSTART: u32 = 1 << 0;
const EXI_CR_WRITE: u32 = 1 << 2;
const EXI_CR_READ_WRITE: u32 = 2 << 2;
const EXI_DATA: usize = 0x10;
const UG_READ_ATTEMPTS: i32 = 100;
const UG_WRITE_ATTEMPTS: i32 = 100;

static mut ug_io_base: *mut u8 = core::ptr::null_mut();

extern "C" {
    fn out_be32(addr: *mut u32, value: u32);
    fn in_be32(addr: *mut u32) -> u32;
    fn barrier();
    fn udbg_printf(format: *const u8, ...);
    static mut udbg_putc: Option<unsafe extern "C" fn(i8)>;
    static mut udbg_getc: Option<unsafe extern "C" fn() -> i32>;
    static mut udbg_getc_poll: Option<unsafe extern "C" fn() -> i32>;
    fn of_find_compatible_node(a: *mut u8, b: *mut u8, c: *const u8) -> *mut u8;
    fn of_iomap(node: *mut u8, index: i32) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn of_node_put(node: *mut u8);
    fn __fix_to_virt(index: usize) -> usize;
    fn setbat(n: i32, virt: usize, phys: usize, size: usize, flags: usize);
}

unsafe fn ug_io_transaction(input: u32) -> u32 {
    let csr_reg = ug_io_base.add(EXI_CSR) as *mut u32;
    let data_reg = ug_io_base.add(EXI_DATA) as *mut u32;
    let cr_reg = ug_io_base.add(EXI_CR) as *mut u32;
    let csr = EXI_CSR_CLK_32MHZ | EXI_CSR_CS_0;
    out_be32(csr_reg, csr);
    out_be32(data_reg, input);
    let cr = ((2 - 1) << 4) | EXI_CR_READ_WRITE | EXI_CR_TSTART;
    out_be32(cr_reg, cr);
    while in_be32(cr_reg) & EXI_CR_TSTART != 0 { barrier(); }
    out_be32(csr_reg, 0);
    in_be32(data_reg)
}

unsafe fn ug_is_adapter_present() -> i32 {
    if ug_io_base.is_null() { return 0; }
    (ug_io_transaction(0x90000000) == 0x04700000) as i32
}

unsafe fn ug_is_txfifo_ready() -> i32 { (ug_io_transaction(0xc0000000) & 0x04000000) as i32 }

unsafe fn ug_raw_putc(ch: i8) { ug_io_transaction(0xb0000000 | ((ch as u32) << 20)); }

unsafe fn ug_putc(ch: i8) {
    let mut count = UG_WRITE_ATTEMPTS;
    if ug_io_base.is_null() { return; }
    if ch == b'\n' as i8 { ug_putc(b'\r' as i8); }
    while ug_is_txfifo_ready() == 0 && count > 0 { count -= 1; barrier(); }
    if count >= 0 { ug_raw_putc(ch); }
}

unsafe fn ug_is_rxfifo_ready() -> i32 { (ug_io_transaction(0xd0000000) & 0x04000000) as i32 }

unsafe fn ug_raw_getc() -> i32 {
    let data = ug_io_transaction(0xa0000000);
    if data & 0x08000000 != 0 { ((data >> 16) & 0xff) as i32 } else { -1 }
}

unsafe fn ug_getc() -> i32 {
    let mut count = UG_READ_ATTEMPTS;
    if ug_io_base.is_null() { return -1; }
    while ug_is_rxfifo_ready() == 0 && count > 0 { count -= 1; barrier(); }
    ug_raw_getc()
}

unsafe extern "C" fn ug_udbg_putc(ch: i8) { ug_putc(ch); }
unsafe extern "C" fn ug_udbg_getc() -> i32 {
    let mut ch;
    loop { ch = ug_getc(); if ch != -1 { return ch; } barrier(); }
}
unsafe extern "C" fn ug_udbg_getc_poll() -> i32 {
    if ug_is_rxfifo_ready() == 0 { -1 } else { ug_getc() }
}

unsafe fn ug_udbg_probe(exi_io_base: *mut u8) -> *mut u8 {
    let mut i = 0;
    while i < 2 {
        ug_io_base = exi_io_base.add(0x14 * i);
        if ug_is_adapter_present() != 0 { break; }
        i += 1;
    }
    if i == 2 { ug_io_base = core::ptr::null_mut(); }
    ug_io_base
}

pub unsafe extern "C" fn ug_udbg_init() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"nintendo,flipper-exi\0".as_ptr());
    if np.is_null() { udbg_printf(b"%s: EXI node not found\0".as_ptr()); return; }
    let exi_io_base = of_iomap(np, 0);
    if exi_io_base.is_null() { udbg_printf(b"%s: failed to setup EXI io base\n\0".as_ptr()); of_node_put(np); return; }
    if ug_udbg_probe(exi_io_base).is_null() { udbg_printf(b"usbgecko_udbg: not found\n\0".as_ptr()); iounmap(exi_io_base); }
    else { udbg_putc = Some(ug_udbg_putc); udbg_getc = Some(ug_udbg_getc); udbg_getc_poll = Some(ug_udbg_getc_poll); udbg_printf(b"usbgecko_udbg: ready\n\0".as_ptr()); }
    of_node_put(np);
}

// Preserved conditional intent: this section is compiled when
// CONFIG_PPC_EARLY_DEBUG_USBGECKO is enabled.
#[cfg(feature = "CONFIG_PPC_EARLY_DEBUG_USBGECKO")]
unsafe fn ug_early_grab_io_addr() -> usize {
    // CONFIG_GAMECUBE and CONFIG_WII select the platform-specific address.
    #[cfg(feature = "CONFIG_GAMECUBE")]
    { return 0x0c000000; }
    #[cfg(feature = "CONFIG_WII")]
    { return 0x0d000000; }
    panic!("Invalid platform for USB Gecko based early debugging.");
}

#[cfg(feature = "CONFIG_PPC_EARLY_DEBUG_USBGECKO")]
pub unsafe extern "C" fn udbg_init_usbgecko() {
    // The BAT uses a virtual address range reserved at the fixmap.
    let early_debug_area = __fix_to_virt(FIX_EARLY_DEBUG_BASE) as *mut u8;
    let exi_io_base = early_debug_area.add(0x00006800);
    if ug_udbg_probe(exi_io_base).is_null() { return; }
    udbg_putc = Some(ug_udbg_putc);
    udbg_getc = Some(ug_udbg_getc);
    udbg_getc_poll = Some(ug_udbg_getc_poll);
    setbat(1, early_debug_area as usize, ug_early_grab_io_addr(), 128 * 1024, PAGE_KERNEL_NCG);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
