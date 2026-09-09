/*
 * linux/include/asm-mips/txx9/generic.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_uchar, c_void};

// Supplied by linux/ioport.h.
#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}

extern "C" {
    pub static mut txx9_ce_res: [resource; 0];
    pub static mut txx9_pcode: c_uint;
    pub static mut txx9_pcode_str: [c_char; 8];
    pub fn txx9_reg_res_init(pcode: c_uint, base: c_ulong, size: c_ulong);

    pub static mut txx9_master_clock: c_uint;
    pub static mut txx9_cpu_clock: c_uint;
    pub static mut txx9_gbus_clock: c_uint;

    pub static mut txx9_ccfg_toeon: c_int;
    pub fn early_serial_txx9_setup(port: *mut uart_port) -> c_int;

    pub static mut txx9_board_vec: *mut txx9_board_vec;
    pub static mut txx9_irq_dispatch: Option<unsafe extern "C" fn(pending: c_int) -> c_int>;
    pub fn prom_getenv(name: *const c_char) -> *const c_char;
    pub fn txx9_wdt_init(base: c_ulong);
    pub fn txx9_wdt_now(base: c_ulong);
    pub fn txx9_ethaddr_init(id: c_uint, ethaddr: *mut c_uchar);
    pub fn txx9_sio_init(
        baseaddr: c_ulong,
        irq: c_int,
        line: c_uint,
        sclk: c_uint,
        nocts: c_int,
    );

    pub fn txx9_physmap_flash_init(
        no: c_int,
        addr: c_ulong,
        size: c_ulong,
        pdata: *const physmap_flash_data,
    );

    pub fn txx9_iocled_init(
        baseaddr: c_ulong,
        num: c_uint,
        color: *const c_char,
        deftriggers: *mut *mut c_char,
    );
    pub fn txx9_aclc_init(
        baseaddr: c_ulong,
        irq: c_int,
        dmac_id: c_uint,
        dma_chan_out: c_uint,
        dma_chan_in: c_uint,
    );
    pub fn txx9_sramc_init(r: *mut resource);
}

// TXX9_CE(n): (unsigned long)(txx9_ce_res[(n)].start)
#[inline]
pub unsafe fn TXX9_CE(n: usize) -> c_ulong {
    (*txx9_ce_res.as_ptr().add(n)).start
}

// TXX9_IMCLK: txx9_gbus_clock / 2
#[inline]
pub unsafe fn TXX9_IMCLK() -> c_uint {
    txx9_gbus_clock / 2
}

#[repr(C)]
pub struct uart_port {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct txx9_board_vec {
    pub system: *const c_char,
    pub prom_init: Option<unsafe extern "C" fn()>,
    pub mem_setup: Option<unsafe extern "C" fn()>,
    pub irq_setup: Option<unsafe extern "C" fn()>,
    pub time_init: Option<unsafe extern "C" fn()>,
    pub arch_init: Option<unsafe extern "C" fn()>,
    pub device_init: Option<unsafe extern "C" fn()>,
    // Present when CONFIG_PCI is enabled: int (*pci_map_irq)(...)
    #[cfg(feature = "CONFIG_PCI")]
    pub pci_map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> c_int>,
}

#[repr(C)]
pub struct physmap_flash_data {
    _private: [u8; 0],
}

// Present when CONFIG_EARLY_PRINTK is enabled.
#[cfg(feature = "CONFIG_EARLY_PRINTK")]
extern "C" {
    pub static mut txx9_prom_putchar: Option<unsafe extern "C" fn(c_char)>;
}

#[cfg(feature = "CONFIG_EARLY_PRINTK")]
extern "C" {
    pub fn txx9_sio_putchar_init(baseaddr: c_ulong);
}

#[cfg(not(feature = "CONFIG_EARLY_PRINTK"))]
#[inline]
pub unsafe fn txx9_sio_putchar_init(_baseaddr: c_ulong) {}

/* 8 bit version of __fls(): find first bit set (returns 0..7) */
#[inline]
pub fn __fls8(mut x: c_uchar) -> c_uint {
    let mut r: c_int = 7;

    if (x & 0xf0) == 0 {
        r -= 4;
        x <<= 4;
    }
    if (x & 0xc0) == 0 {
        r -= 2;
        x <<= 2;
    }
    if (x & 0x80) == 0 {
        r -= 1;
    }
    r as c_uint
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
