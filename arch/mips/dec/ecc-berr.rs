// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Bus error event handling code for systems equipped with ECC handling logic.
 */

use core::ffi::{c_char, c_void};

extern "C" {
    static mut cached_kn02_csr: u32;
    static mut mips_machtype: i32;

    fn iob();
    fn fast_iob();
    fn current_cpu_type() -> i32;
    fn get_irq_regs() -> *mut pt_regs;
    fn die(reason: *const c_char, regs: *mut pt_regs) -> !;
    fn __ratelimit(state: *mut ratelimit_state) -> bool;
    fn printk(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct pt_regs {
    pub regs: [u64; 32],
    pub cp0_epc: u64,
}

#[repr(C)]
pub struct ratelimit_state {
    _private: [u8; 0],
}

pub type irqreturn_t = i32;

extern "C" {
    static mut kn0x_erraddr: *mut u32;
    static mut kn0x_chksyn: *mut u32;
}

static mut KN0X_ERRADDR: *mut u32 = core::ptr::null_mut();
static mut KN0X_CHKSYN: *mut u32 = core::ptr::null_mut();

unsafe fn dec_ecc_be_ack() {
    core::ptr::write_volatile(KN0X_ERRADDR, 0);
    iob();
}

unsafe fn dec_ecc_be_backend(regs: *mut pt_regs, is_fixup: i32, invoker: i32) -> i32 {
    let excstr = b"exception\0";
    let intstr = b"interrupt\0";
    let cpustr = b"CPU\0";
    let dmastr = b"DMA\0";
    let readstr = b"read\0";
    let mreadstr = b"memory read\0";
    let writestr = b"write\0";
    let mwritstr = b"partial memory write\0";
    let timestr = b"timeout\0";
    let overstr = b"overrun\0";
    let eccstr = b"ECC error\0";

    let mut kind: *const c_char;
    let mut agent: *const c_char;
    let mut cycle: *const c_char;
    let mut event: *const c_char;
    let mut status: *const c_char = b"\0".as_ptr() as *const c_char;
    let mut xbit: *const c_char = b"\0".as_ptr() as *const c_char;
    let mut fmt: *const c_char = b"\0".as_ptr() as *const c_char;
    let mut address: u64;
    let mut syn: u16 = 0;
    let mut sngl: u16;
    let mut i: i32 = 0;

    let erraddr = core::ptr::read_volatile(KN0X_ERRADDR);
    let chksyn = core::ptr::read_volatile(KN0X_CHKSYN);
    let mut action = MIPS_BE_FATAL;

    if (erraddr & (KN0X_EAR_VALID | KN0X_EAR_ECCERR)) == KN0X_EAR_VALID {
        dec_ecc_be_ack();
    }

    kind = if invoker != 0 { intstr.as_ptr() as _ } else { excstr.as_ptr() as _ };
    if erraddr & KN0X_EAR_VALID == 0 {
        printk(b"Unidentified bus error %s\n\0".as_ptr() as _, kind);
        return action;
    }

    agent = if erraddr & KN0X_EAR_CPU != 0 { cpustr.as_ptr() as _ } else { dmastr.as_ptr() as _ };
    if erraddr & KN0X_EAR_ECCERR != 0 {
        cycle = if erraddr & KN0X_EAR_WRITE != 0 { mwritstr.as_ptr() as _ } else { mreadstr.as_ptr() as _ };
        event = eccstr.as_ptr() as _;
    } else {
        cycle = if erraddr & KN0X_EAR_WRITE != 0 { writestr.as_ptr() as _ } else { readstr.as_ptr() as _ };
        event = if erraddr & KN0X_EAR_CPU != 0 { timestr.as_ptr() as _ } else { overstr.as_ptr() as _ };
    }

    address = (erraddr & KN0X_EAR_ADDRESS) as u64;
    if (erraddr & (KN0X_EAR_WRITE | KN0X_EAR_ECCERR)) == KN0X_EAR_ECCERR {
        address = (address & !0xfff) | ((address.wrapping_sub(5)) & 0xfff);
    }
    address <<= 2;
    if erraddr & KN0X_EAR_CPU != 0 && is_fixup != 0 { action = MIPS_BE_FIXUP; }

    if erraddr & KN0X_EAR_ECCERR != 0 {
        const DATA_SBIT: [u8; 32] = [0x4f,0x4a,0x52,0x54,0x57,0x58,0x5b,0x5d,0x23,0x25,0x26,0x29,0x2a,0x2c,0x31,0x34,0x0e,0x0b,0x13,0x15,0x16,0x19,0x1a,0x1c,0x62,0x64,0x67,0x68,0x6b,0x6d,0x70,0x75];
        const DATA_MBIT: [u8; 25] = [0x07,0x0d,0x1f,0x2f,0x32,0x37,0x38,0x3b,0x3d,0x3e,0x43,0x45,0x46,0x49,0x4c,0x51,0x5e,0x61,0x6e,0x73,0x76,0x79,0x7a,0x7c,0x7f];
        if address & 4 == 0 { syn = chksyn as u16; } else { syn = (chksyn >> 16) as u16; }
        if syn & KN0X_ESR_VLDLO == 0 { dec_ecc_be_ack(); }
        else {
            sngl = syn & KN0X_ESR_SNGLO; syn &= KN0X_ESR_SYNLO;
            for n in 0..25 { if syn == DATA_MBIT[n] as u16 { i = n; break; } }
            if i < 25 { status = b"uncorrectable multiple\0".as_ptr() as _; }
            else if sngl == 0 { status = b"uncorrectable double\0".as_ptr() as _; }
            else { let ptr = CKSEG1ADDR(address) as *mut u32; let v = core::ptr::read_volatile(ptr); core::ptr::write_volatile(ptr, v); iob(); status = b"corrected single\0".as_ptr() as _; action = MIPS_BE_DISCARD; }
            dec_ecc_be_ack();
            if syn != 0 && syn == (syn & (!syn).wrapping_add(1)) { i = (syn >> 2) as i32; }
            else { i = 32; for n in 0..32 { if syn == DATA_SBIT[n] as u16 { i = n; break; } } }
        }
    }
    if action != MIPS_BE_FIXUP { printk(b"Bus error %s: %s %s %s at %#010lx\n\0".as_ptr() as _, kind, agent, cycle, event, address); }
    action
}

pub unsafe fn dec_ecc_be_handler(regs: *mut pt_regs, is_fixup: i32) -> i32 { dec_ecc_be_backend(regs, is_fixup, 0) }

pub unsafe fn dec_ecc_be_interrupt(_irq: i32, _dev_id: *mut c_void) -> irqreturn_t {
    let regs = get_irq_regs();
    let action = dec_ecc_be_backend(regs, 0, 1);
    if action == MIPS_BE_DISCARD { return IRQ_HANDLED; }
    printk(b"Fatal bus interrupt, epc == %08lx, ra == %08lx\n\0".as_ptr() as _, (*regs).cp0_epc, (*regs).regs[31]);
    die(b"Unrecoverable bus error\0".as_ptr() as _, regs)
}

pub unsafe fn dec_ecc_be_init() {
    if mips_machtype == MACH_DS5000_200 { dec_kn02_be_init(); } else { dec_kn03_be_init(); }
    dec_ecc_be_ack();
}

unsafe fn dec_kn02_be_init() { let csr = CKSEG1ADDR(KN02_SLOT_BASE + KN02_CSR) as *mut u32; KN0X_ERRADDR = CKSEG1ADDR(KN02_SLOT_BASE + KN02_ERRADDR) as *mut u32; KN0X_CHKSYN = CKSEG1ADDR(KN02_SLOT_BASE + KN02_CHKSYN) as *mut u32; cached_kn02_csr = core::ptr::read_volatile(csr) | KN02_CSR_LEDS; cached_kn02_csr &= !(KN02_CSR_DIAGCHK | KN02_CSR_DIAGGEN); cached_kn02_csr |= KN02_CSR_CORRECT; core::ptr::write_volatile(csr, cached_kn02_csr); iob(); }
unsafe fn dec_kn03_be_init() { let mcr = CKSEG1ADDR(KN03_SLOT_BASE + IOASIC_MCR) as *mut u32; let mbcs = CKSEG1ADDR(KN4K_SLOT_BASE + KN4K_MB_CSR) as *mut u32; KN0X_ERRADDR = CKSEG1ADDR(KN03_SLOT_BASE + IOASIC_ERRADDR) as *mut u32; KN0X_CHKSYN = CKSEG1ADDR(KN03_SLOT_BASE + IOASIC_CHKSYN) as *mut u32; let v = core::ptr::read_volatile(mcr); core::ptr::write_volatile(mcr, (v & !(KN03_MCR_DIAGCHK | KN03_MCR_DIAGGEN)) | KN03_MCR_CORRECT); if current_cpu_type() == CPU_R4400SC { let v = core::ptr::read_volatile(mbcs); core::ptr::write_volatile(mbcs, v | KN4K_MB_CSR_EE); } fast_iob(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
