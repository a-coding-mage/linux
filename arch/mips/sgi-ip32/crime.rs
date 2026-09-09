/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001, 2003 Keith M Wesolowski
 * Copyright (C) 2005 Ilya A. Volynets <ilya@total-knowledge.com>
 */

// Declarations and constants below are supplied by the corresponding kernel
// headers and other translation units.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut crime: *mut sgi_crime;
    static mut mace: *mut sgi_mace;

    fn set_io_port_base(base: c_ulong);
    fn ioremap(addr: c_ulong, size: usize) -> *mut c_void;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn panic(fmt: *const c_char) -> !;
}

#[repr(C)]
pub struct sgi_crime {
    pub id: c_uint,
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct sgi_mace {
    pub _opaque: [u8; 0],
}

extern "C" {
    static MACEPCI_LOW_IO: c_ulong;
    static CRIME_BASE: c_ulong;
    static MACE_BASE: c_ulong;
    static CRIME_ID_REV: c_ulong;
    static CRIME_ID_IDBITS: c_ulong;
    static CRIME_MEM_ERROR_STAT_MASK: c_ulong;
    static CRIME_MEM_ERROR_ADDR_MASK: c_ulong;
    static CRIME_MEM_ERROR_INV: c_ulong;
    static CRIME_MEM_ERROR_ECC: c_ulong;
    static CRIME_MEM_ERROR_ECC_SYN_MASK: c_ulong;
    static CRIME_MEM_ERROR_ECC_CHK_MASK: c_ulong;
    static CRIME_MEM_ERROR_MULTIPLE: c_ulong;
    static CRIME_MEM_ERROR_HARD_ERR: c_ulong;
    static CRIME_MEM_ERROR_SOFT_ERR: c_ulong;
    static CRIME_MEM_ERROR_CPU_ACCESS: c_ulong;
    static CRIME_MEM_ERROR_VICE_ACCESS: c_ulong;
    static CRIME_MEM_ERROR_GBE_ACCESS: c_ulong;
    static CRIME_MEM_ERROR_RE_ACCESS: c_ulong;
    static CRIME_MEM_ERROR_RE_ID: c_ulong;
    static CRIME_MEM_ERROR_MACE_ACCESS: c_ulong;
    static CRIME_MEM_ERROR_MACE_ID: c_ulong;
    static CRIME_CPU_ERROR_MASK: c_ulong;
    static CRIME_CPU_ERROR_ADDR_MASK: c_ulong;
    static IRQ_HANDLED: irqreturn_t;
}

pub type irqreturn_t = c_int;

#[no_mangle]
pub unsafe extern "C" fn crime_init() {
    let mut id: c_uint;
    let mut rev: c_uint;
    let field: usize = 2 * core::mem::size_of::<c_ulong>();

    set_io_port_base(ioremap(MACEPCI_LOW_IO, 0x2000000) as c_ulong);
    crime = ioremap(CRIME_BASE, core::mem::size_of::<sgi_crime>()) as *mut sgi_crime;
    mace = ioremap(MACE_BASE, core::mem::size_of::<sgi_mace>()) as *mut sgi_mace;

    id = (*crime).id;
    rev = (id as c_ulong & CRIME_ID_REV) as c_uint;
    id = (((id as c_ulong & CRIME_ID_IDBITS) >> 4) as c_uint);
    printk(b"CRIME id %1x rev %d at 0x%0*lx\n\0".as_ptr() as *const c_char,
           id, rev, field, CRIME_BASE);
}

#[no_mangle]
pub unsafe extern "C" fn crime_memerr_intr(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let stat: c_ulong = (*crime).mem_error_stat & CRIME_MEM_ERROR_STAT_MASK;
    let addr: c_ulong = (*crime).mem_error_addr & CRIME_MEM_ERROR_ADDR_MASK;
    let mut fatal: c_int = 0;

    printk(b"CRIME memory error at 0x%08lx ST 0x%08lx<\0".as_ptr() as *const c_char, addr, stat);
    if stat & CRIME_MEM_ERROR_INV != 0 { printk(b"INV,\0".as_ptr() as *const c_char); }
    if stat & CRIME_MEM_ERROR_ECC != 0 {
        let ecc_syn = (*crime).mem_ecc_syn & CRIME_MEM_ERROR_ECC_SYN_MASK;
        let ecc_gen = (*crime).mem_ecc_chk & CRIME_MEM_ERROR_ECC_CHK_MASK;
        printk(b"ECC,SYN=0x%08lx,GEN=0x%08lx,\0".as_ptr() as *const c_char, ecc_syn, ecc_gen);
    }
    if stat & CRIME_MEM_ERROR_MULTIPLE != 0 { fatal = 1; printk(b"MULTIPLE,\0".as_ptr() as *const c_char); }
    if stat & CRIME_MEM_ERROR_HARD_ERR != 0 { fatal = 1; printk(b"HARD,\0".as_ptr() as *const c_char); }
    if stat & CRIME_MEM_ERROR_SOFT_ERR != 0 { printk(b"SOFT,\0".as_ptr() as *const c_char); }
    if stat & CRIME_MEM_ERROR_CPU_ACCESS != 0 { printk(b"CPU,\0".as_ptr() as *const c_char); }
    if stat & CRIME_MEM_ERROR_VICE_ACCESS != 0 { printk(b"VICE,\0".as_ptr() as *const c_char); }
    if stat & CRIME_MEM_ERROR_GBE_ACCESS != 0 { printk(b"GBE,\0".as_ptr() as *const c_char); }
    if stat & CRIME_MEM_ERROR_RE_ACCESS != 0 { printk(b"RE,REID=0x%02lx,\0".as_ptr() as *const c_char, (stat & CRIME_MEM_ERROR_RE_ID) >> 8); }
    if stat & CRIME_MEM_ERROR_MACE_ACCESS != 0 { printk(b"MACE,MACEID=0x%02lx,\0".as_ptr() as *const c_char, stat & CRIME_MEM_ERROR_MACE_ID); }
    (*crime).mem_error_stat = 0;
    if fatal != 0 { printk(b"FATAL>\n\0".as_ptr() as *const c_char); panic(b"Fatal memory error.\0".as_ptr() as *const c_char); }
    else { printk(b"NONFATAL>\n\0".as_ptr() as *const c_char); }
    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn crime_cpuerr_intr(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let stat = (*crime).cpu_error_stat & CRIME_CPU_ERROR_MASK;
    let mut addr = (*crime).cpu_error_addr & CRIME_CPU_ERROR_ADDR_MASK;
    addr <<= 2;
    printk(b"CRIME CPU error at 0x%09lx status 0x%08lx\n\0".as_ptr() as *const c_char, addr, stat);
    (*crime).cpu_error_stat = 0;
    IRQ_HANDLED
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
