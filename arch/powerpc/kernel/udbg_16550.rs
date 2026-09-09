// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * udbg for NS16550 compatible serial ports
 *
 * Copyright (C) 2001-2005 PPC 64 Team, IBM Corp
 */

type U8 = u8;

extern "C" {
    fn real_readb(addr: *mut U8) -> U8;
    fn real_writeb(data: U8, addr: *mut U8);
    fn real_205_readb(addr: *mut U8) -> U8;
    fn real_205_writeb(data: U8, addr: *mut U8);
    fn cpu_relax();
    fn inb(addr: usize) -> U8;
    fn outb(data: U8, addr: usize);
    fn in_8(addr: *mut U8) -> U8;
    fn out_8(addr: *mut U8, data: U8);
    fn early_ioremap(addr: usize, size: usize) -> *mut U8;
    fn ioremap(addr: usize, size: usize) -> *mut U8;
    fn early_iounmap(addr: *mut U8, size: usize);
    fn warn_on(condition: bool) -> bool;
}

extern "C" {
    static mut udbg_putc: Option<unsafe extern "C" fn(u8)>;
    static mut udbg_flush: Option<unsafe extern "C" fn()>;
    static mut udbg_getc: Option<unsafe extern "C" fn() -> i32>;
    static mut udbg_getc_poll: Option<unsafe extern "C" fn() -> i32>;
}

const UART_RBR: u32 = 0;
const UART_IER: u32 = 1;
const UART_FCR: u32 = 2;
const UART_LCR: u32 = 3;
const UART_MCR: u32 = 4;
const UART_LSR: u32 = 5;
const UART_THR: u32 = UART_RBR;
const UART_DLL: u32 = UART_RBR;
const UART_DLM: u32 = UART_IER;
const LSR_DR: u8 = 0x01;
const LSR_THRE: u8 = 0x20;
const LCR_DLAB: u8 = 0x80;

static mut udbg_uart_in: Option<unsafe extern "C" fn(u32) -> U8> = None;
static mut udbg_uart_out: Option<unsafe extern "C" fn(u32, U8)> = None;

unsafe extern "C" fn udbg_uart_flush() {
    if let Some(input) = udbg_uart_in {
        while input(UART_LSR) & LSR_THRE == 0 {
            cpu_relax();
        }
    }
}

unsafe extern "C" fn udbg_uart_putc(c: u8) {
    let Some(output) = udbg_uart_out else { return };
    if c == b'\n' {
        udbg_uart_putc(b'\r');
    }
    udbg_uart_flush();
    output(UART_THR, c);
}

unsafe extern "C" fn udbg_uart_getc_poll() -> i32 {
    let Some(input) = udbg_uart_in else { return -1 };
    if input(UART_LSR) & LSR_DR == 0 { input(UART_RBR) as i32 } else { -1 }
}

unsafe extern "C" fn udbg_uart_getc() -> i32 {
    let Some(input) = udbg_uart_in else { return -1 };
    while input(UART_LSR) & LSR_DR == 0 { cpu_relax(); }
    input(UART_RBR) as i32
}

unsafe fn udbg_use_uart() {
    udbg_putc = Some(udbg_uart_putc);
    udbg_flush = Some(udbg_uart_flush);
    udbg_getc = Some(udbg_uart_getc);
    udbg_getc_poll = Some(udbg_uart_getc_poll);
}

#[no_mangle]
pub unsafe extern "C" fn udbg_uart_setup(mut speed: u32, mut clock: u32) {
    let Some(output) = udbg_uart_out else { return };
    if clock == 0 { clock = 1843200; }
    if speed == 0 { speed = 9600; }
    let base_bauds = clock / 16;
    let dll = base_bauds / speed;
    output(UART_LCR, 0x00); output(UART_IER, 0xff); output(UART_IER, 0x00);
    output(UART_LCR, LCR_DLAB); output(UART_DLL, (dll & 0xff) as u8);
    output(UART_DLM, (dll >> 8) as u8); output(UART_LCR, 0x3);
    output(UART_MCR, 0x3); output(UART_FCR, 0x7);
}

#[no_mangle]
pub unsafe extern "C" fn udbg_probe_uart_speed(clock: u32) -> u32 {
    let input = udbg_uart_in.unwrap(); let output = udbg_uart_out.unwrap();
    let old_lcr = input(UART_LCR);
    output(UART_LCR, old_lcr | LCR_DLAB);
    let divisor = ((input(UART_DLM) as u32) << 8) | input(UART_DLL) as u32;
    let prescaler = if input(UART_MCR) & 0x80 != 0 { 4 } else { 1 };
    output(UART_LCR, old_lcr);
    let mut speed = (clock / prescaler) / (divisor * 16);
    if speed > clock / 16 { speed = 9600; }
    speed
}

#[repr(C)]
union UdbgUart { mmio_base: *mut U8, pio_base: usize }
static mut udbg_uart: UdbgUart = UdbgUart { pio_base: 0 };
static mut udbg_uart_stride: usize = 1;

unsafe extern "C" fn udbg_uart_in_pio(reg: u32) -> U8 { inb(udbg_uart.pio_base + reg as usize * udbg_uart_stride) }
unsafe extern "C" fn udbg_uart_out_pio(reg: u32, data: U8) { outb(data, udbg_uart.pio_base + reg as usize * udbg_uart_stride); }

#[no_mangle]
pub unsafe extern "C" fn udbg_uart_init_pio(port: usize, stride: u32) {
    if port == 0 { return; }
    udbg_uart.pio_base = port; udbg_uart_stride = stride as usize;
    udbg_uart_in = Some(udbg_uart_in_pio); udbg_uart_out = Some(udbg_uart_out_pio); udbg_use_uart();
}

unsafe extern "C" fn udbg_uart_in_mmio(reg: u32) -> U8 { in_8(udbg_uart.mmio_base.add(reg as usize * udbg_uart_stride)) }
unsafe extern "C" fn udbg_uart_out_mmio(reg: u32, data: U8) { out_8(udbg_uart.mmio_base.add(reg as usize * udbg_uart_stride), data); }

#[no_mangle]
pub unsafe extern "C" fn udbg_uart_init_mmio(addr: *mut U8, stride: u32) {
    if addr.is_null() { return; }
    udbg_uart.mmio_base = addr; udbg_uart_stride = stride as usize;
    udbg_uart_in = Some(udbg_uart_in_mmio); udbg_uart_out = Some(udbg_uart_out_mmio); udbg_use_uart();
}

// CONFIG_PPC_PASEMI conditionally includes the following platform-specific implementation.
#[cfg(feature = "CONFIG_PPC_PASEMI")]
mod pasemi {
    use super::*;
    const UDBG_UART_PAS_ADDR: *mut U8 = 0xfcff03f8 as *mut U8;
    unsafe extern "C" fn input(reg: u32) -> U8 { real_205_readb(UDBG_UART_PAS_ADDR.add(reg as usize)) }
    unsafe extern "C" fn output(reg: u32, val: U8) { real_205_writeb(val, UDBG_UART_PAS_ADDR.add(reg as usize)); }
    #[no_mangle] pub unsafe extern "C" fn udbg_init_pas_realmode() { udbg_uart_in = Some(input); udbg_uart_out = Some(output); udbg_use_uart(); }
}

// CONFIG_PPC_EARLY_DEBUG_44x and CONFIG_PPC_EARLY_DEBUG_16550 are build-time conditions.
#[cfg(feature = "CONFIG_PPC_EARLY_DEBUG_44x")]
mod early_44x {
    use super::*;
    extern "C" {
        static PPC44x_EARLY_DEBUG_VIRTADDR: usize;
        fn as1_readb(addr: *mut U8) -> U8;
        fn as1_writeb(data: U8, addr: *mut U8);
    }
    unsafe extern "C" fn input(reg: u32) -> U8 { as1_readb((PPC44x_EARLY_DEBUG_VIRTADDR + reg as usize) as *mut U8) }
    unsafe extern "C" fn output(reg: u32, val: U8) { as1_writeb(val, (PPC44x_EARLY_DEBUG_VIRTADDR + reg as usize) as *mut U8); }
    #[no_mangle] pub unsafe extern "C" fn udbg_init_44x_as1() { udbg_uart_in = Some(input); udbg_uart_out = Some(output); udbg_use_uart(); }
}

#[cfg(feature = "CONFIG_PPC_EARLY_DEBUG_16550")]
mod early_16550 {
    use super::*;
    extern "C" {
        static CONFIG_PPC_EARLY_DEBUG_16550_PHYSADDR: usize;
        static CONFIG_PPC_EARLY_DEBUG_16550_STRIDE: usize;
        fn early_initcall(f: unsafe extern "C" fn() -> i32);
    }
    static mut udbg_uart_early_addr: *mut U8 = core::ptr::null_mut();

    #[no_mangle]
    pub unsafe extern "C" fn udbg_init_debug_16550() {
        udbg_uart_early_addr = early_ioremap(CONFIG_PPC_EARLY_DEBUG_16550_PHYSADDR, 0x1000);
        udbg_uart_init_mmio(udbg_uart_early_addr, CONFIG_PPC_EARLY_DEBUG_16550_STRIDE as u32);
    }

    unsafe extern "C" fn udbg_init_debug_16550_ioremap() -> i32 {
        if udbg_uart_early_addr.is_null() { return 0; }
        let addr = ioremap(CONFIG_PPC_EARLY_DEBUG_16550_PHYSADDR, 0x1000);
        if addr.is_null() {
            return -12; // -ENOMEM
        }
        udbg_uart_init_mmio(addr, CONFIG_PPC_EARLY_DEBUG_16550_STRIDE as u32);
        early_iounmap(udbg_uart_early_addr, 0x1000);
        udbg_uart_early_addr = core::ptr::null_mut();
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
