// SPDX-License-Identifier: GPL-2.0
/*
 * ip22-berr.c: Bus error handling.
 *
 * Copyright (C) 2002, 2003 Ladislav Michl (ladis@linux-mips.org)
 */

// Linux and SGI platform headers supplying the declarations below.

use core::ffi::c_char;

#[repr(C)]
pub struct PtRegs {
    pub regs: [c_ulong; 32],
    pub cp0_epc: c_ulong,
    pub cp0_cause: c_ulong,
}

type c_ulong = usize;

#[repr(C)]
struct Sgimc { cerr: u32, cstat: u32, gerr: u32, gstat: u32 }
#[repr(C)]
struct Sgioc { extio: u32 }
#[repr(C)]
struct Sgint { errstat: u32 }
#[repr(C)]
struct Hpc3 { bestat: u32 }

extern "C" {
    static mut sgimc: *mut Sgimc;
    static mut sgioc: *mut Sgioc;
    static mut sgint: *mut Sgint;
    static mut hpc3c0: *mut Hpc3;
    fn ip22_is_fullhouse() -> bool;
    fn get_irq_regs() -> *mut PtRegs;
    fn mips_set_be_handler(handler: unsafe extern "C" fn(*mut PtRegs, i32) -> i32);
    fn die_if_kernel(msg: *const c_char, regs: *mut PtRegs);
    fn force_sig(sig: i32);
    fn printk(fmt: *const c_char, ...);
}

// Constants supplied by the platform headers.
extern "C" {
    static EXTIO_MC_BUSERR: u32;
    static EXTIO_HPC3_BUSERR: u32;
    static EXTIO_EISA_BUSERR: u32;
    static HPC3_BESTAT_PIDMASK: u32;
    static HPC3_BESTAT_PIDSHIFT: u32;
    static HPC3_BESTAT_CTYPE: u32;
    static HPC3_BESTAT_BLMASK: u32;
    static SGIMC_CSTAT_RD: u32;
    static SGIMC_CSTAT_PAR: u32;
    static SGIMC_CSTAT_ADDR: u32;
    static SGIMC_CSTAT_SYSAD_PAR: u32;
    static SGIMC_CSTAT_SYSCMD_PAR: u32;
    static SGIMC_CSTAT_BAD_DATA: u32;
    static SGIMC_GSTAT_RD: u32;
    static SGIMC_GSTAT_WR: u32;
    static SGIMC_GSTAT_TIME: u32;
    static SGIMC_GSTAT_PROM: u32;
    static SGIMC_GSTAT_ADDR: u32;
    static SGIMC_GSTAT_BC: u32;
    static SGIMC_GSTAT_PIO_RD: u32;
    static SGIMC_GSTAT_PIO_WR: u32;
}

static mut cpu_err_stat: u32 = 0; // Status reg for CPU
static mut gio_err_stat: u32 = 0; // Status reg for GIO
static mut cpu_err_addr: u32 = 0; // Error address reg for CPU
static mut gio_err_addr: u32 = 0; // Error address reg for GIO
static mut extio_stat: u32 = 0;
static mut hpc3_berr_stat: u32 = 0; // Bus error interrupt status

unsafe fn save_and_clear_buserr() {
    cpu_err_addr = (*sgimc).cerr;
    cpu_err_stat = (*sgimc).cstat;
    gio_err_addr = (*sgimc).gerr;
    gio_err_stat = (*sgimc).gstat;
    extio_stat = if ip22_is_fullhouse() { (*sgioc).extio } else { (*sgint).errstat << 4 };
    hpc3_berr_stat = (*hpc3c0).bestat;
    (*sgimc).cstat = 0;
    (*sgimc).gstat = 0;
}

const GIO_ERRMASK: u32 = 0xff00;
const CPU_ERRMASK: u32 = 0x3f00;

unsafe fn print_buserr() {
    if extio_stat & EXTIO_MC_BUSERR != 0 { printk(b"MC Bus Error\n\0".as_ptr() as *const c_char); }
    if extio_stat & EXTIO_HPC3_BUSERR != 0 { printk(b"HPC3 Bus Error 0x%x:<id=0x%x,%s,lane=0x%x>\n\0".as_ptr() as *const c_char, hpc3_berr_stat, (hpc3_berr_stat & HPC3_BESTAT_PIDMASK) >> HPC3_BESTAT_PIDSHIFT, if hpc3_berr_stat & HPC3_BESTAT_CTYPE != 0 { b"PIO\0".as_ptr() } else { b"DMA\0".as_ptr() }, hpc3_berr_stat & HPC3_BESTAT_BLMASK); }
    if extio_stat & EXTIO_EISA_BUSERR != 0 { printk(b"EISA Bus Error\n\0".as_ptr() as *const c_char); }
    if cpu_err_stat & CPU_ERRMASK != 0 { printk(b"CPU error 0x%x<%s%s%s%s%s%s> @ 0x%08x\n\0".as_ptr() as *const c_char, cpu_err_stat, if cpu_err_stat & SGIMC_CSTAT_RD != 0 { b"RD \0".as_ptr() } else { b"\0".as_ptr() }, if cpu_err_stat & SGIMC_CSTAT_PAR != 0 { b"PAR \0".as_ptr() } else { b"\0".as_ptr() }, if cpu_err_stat & SGIMC_CSTAT_ADDR != 0 { b"ADDR \0".as_ptr() } else { b"\0".as_ptr() }, if cpu_err_stat & SGIMC_CSTAT_SYSAD_PAR != 0 { b"SYSAD \0".as_ptr() } else { b"\0".as_ptr() }, if cpu_err_stat & SGIMC_CSTAT_SYSCMD_PAR != 0 { b"SYSCMD \0".as_ptr() } else { b"\0".as_ptr() }, if cpu_err_stat & SGIMC_CSTAT_BAD_DATA != 0 { b"BAD_DATA \0".as_ptr() } else { b"\0".as_ptr() }, cpu_err_addr); }
    if gio_err_stat & GIO_ERRMASK != 0 { printk(b"GIO error 0x%x:<%s%s%s%s%s%s%s%s> @ 0x%08x\n\0".as_ptr() as *const c_char, gio_err_stat, if gio_err_stat & SGIMC_GSTAT_RD != 0 { b"RD \0".as_ptr() } else { b"\0".as_ptr() }, if gio_err_stat & SGIMC_GSTAT_WR != 0 { b"WR \0".as_ptr() } else { b"\0".as_ptr() }, if gio_err_stat & SGIMC_GSTAT_TIME != 0 { b"TIME \0".as_ptr() } else { b"\0".as_ptr() }, if gio_err_stat & SGIMC_GSTAT_PROM != 0 { b"PROM \0".as_ptr() } else { b"\0".as_ptr() }, if gio_err_stat & SGIMC_GSTAT_ADDR != 0 { b"ADDR \0".as_ptr() } else { b"\0".as_ptr() }, if gio_err_stat & SGIMC_GSTAT_BC != 0 { b"BC \0".as_ptr() } else { b"\0".as_ptr() }, if gio_err_stat & SGIMC_GSTAT_PIO_RD != 0 { b"PIO_RD \0".as_ptr() } else { b"\0".as_ptr() }, if gio_err_stat & SGIMC_GSTAT_PIO_WR != 0 { b"PIO_WR \0".as_ptr() } else { b"\0".as_ptr() }, gio_err_addr); }
}

pub unsafe extern "C" fn ip22_be_interrupt(_irq: i32) {
    let field = 2 * core::mem::size_of::<c_ulong>();
    let regs = get_irq_regs();
    save_and_clear_buserr();
    print_buserr();
    printk(b"%s bus error, epc == %0*lx, ra == %0*lx\n\0".as_ptr() as *const c_char, if (*regs).cp0_cause & 4 != 0 { b"Data\0".as_ptr() } else { b"Instruction\0".as_ptr() }, field, (*regs).cp0_epc, field, (*regs).regs[31]);
    die_if_kernel(b"Oops\0".as_ptr() as *const c_char, regs);
    force_sig(7);
}

unsafe extern "C" fn ip22_be_handler(regs: *mut PtRegs, is_fixup: i32) -> i32 {
    save_and_clear_buserr();
    if is_fixup != 0 { return MIPS_BE_FIXUP; }
    print_buserr();
    MIPS_BE_FATAL
}

pub unsafe extern "C" fn ip22_be_init() {
    mips_set_be_handler(ip22_be_handler);
}

extern "C" {
    static MIPS_BE_FIXUP: i32;
    static MIPS_BE_FATAL: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
