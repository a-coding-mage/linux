// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Bus error event handling code for DECstation/DECsystem 3100
 * and 2100 (KN01) systems equipped with parity error detection
 * logic.
 *
 * Copyright (c) 2005, 2026  Maciej W. Rozycki
 */

// Kernel and architecture dependencies supplied by the surrounding tree.

/* CP0 hazard avoidance. */
macro_rules! barrier {
    () => {
        unsafe { core::arch::asm!("nop", options(nostack, preserves_flags)) }
    };
}

/*
 * Bits 7:0 of the Control Register are write-only -- the
 * corresponding bits of the Status Register have a different
 * meaning.  Hence we use a cache.  It speeds up things a bit
 * as well.
 *
 * There is no default value -- it has to be initialized.
 */
pub static mut cached_kn01_csr: u16 = 0;
static mut kn01_lock: RawSpinLock = RawSpinLock::new();

extern "C" {
    fn raw_spin_lock_irqsave(lock: *mut RawSpinLock, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut RawSpinLock, flags: usize);
    fn iob();
    fn get_irq_regs() -> *mut PtRegs;
    fn read_c0_entryhi() -> isize;
    fn write_c0_entryhi(value: isize);
    fn tlb_probe();
    fn tlb_read();
    fn read_c0_entrylo0() -> usize;
    fn __get_user(value: *mut u32, address: *const u32) -> i32;
    fn pr_alert_ratelimited(
        format: *const core::ffi::c_char,
        kind: *const core::ffi::c_char,
        agent: *const core::ffi::c_char,
        cycle: *const core::ffi::c_char,
        event: *const core::ffi::c_char,
        address: usize,
    );
    fn printk(format: *const core::ffi::c_char, epc: usize, ra: usize);
    fn die(message: *const core::ffi::c_char, regs: *mut PtRegs) -> !;
}

#[repr(C)]
pub struct RawSpinLock {
    _opaque: [u8; 0],
}

impl RawSpinLock {
    const fn new() -> Self { Self { _opaque: [] } }
}

#[repr(C)]
pub struct PtRegs {
    pub regs: [usize; 32],
    pub cp0_cause: usize,
    pub cp0_epc: usize,
}

#[repr(C)]
pub union MipsInstruction {
    pub word: u32,
    pub i_format: MipsIFormat,
}

#[repr(C)]
pub struct MipsIFormat {
    pub rs: usize,
    pub immediate: i16,
}

unsafe fn dec_kn01_be_ack() {
    let csr = (CKSEG1ADDR(KN01_SLOT_BASE + KN01_CSR)) as *mut u16;
    let mut flags = 0usize;

    raw_spin_lock_irqsave(&raw mut kn01_lock, &mut flags);
    *csr = cached_kn01_csr | KN01_CSR_MEMERR;
    iob();
    raw_spin_unlock_irqrestore(&raw mut kn01_lock, flags);
}

unsafe fn dec_kn01_be_backend(regs: *mut PtRegs, is_fixup: i32, invoker: i32) -> i32 {
    let kn01_erraddr = (CKSEG1ADDR(KN01_SLOT_BASE + KN01_ERRADDR)) as *const u32;
    let excstr = b"exception\0";
    let intstr = b"interrupt\0";
    let cpustr = b"CPU\0";
    let mreadstr = b"memory read\0";
    let readstr = b"read\0";
    let writestr = b"write\0";
    let timestr = b"timeout\0";
    let paritystr = b"parity error\0";

    let data = ((*regs).cp0_cause & 4) as i32;
    let pc = ((*regs).cp0_epc as *const u32).add((((*regs).cp0_cause & CAUSEF_BD) != 0) as usize);
    let mut insn = MipsInstruction { word: 0 };
    let (mut entrylo, mut offset): (usize, usize);
    let (mut asid, mut entryhi, mut vaddr): (isize, isize, isize);
    let (kind, agent, cycle, event): (&[u8], &[u8], &[u8], &[u8]);
    let address: usize;
    let erraddr = *kn01_erraddr;
    let mut action = MIPS_BE_FATAL;

    dec_kn01_be_ack();
    kind = if invoker != 0 { intstr } else { excstr };
    agent = cpustr;

    if invoker != 0 {
        address = erraddr as usize;
    } else {
        if data != 0 {
            __get_user((&mut insn.word) as *mut u32, pc);
            vaddr = (*regs).regs[insn.i_format.rs] as isize + insn.i_format.immediate as isize;
        } else {
            vaddr = pc as isize;
        }
        if KSEGX(vaddr) == CKSEG0 || KSEGX(vaddr) == CKSEG1 {
            address = CPHYSADDR(vaddr) as usize;
        } else {
            asid = read_c0_entryhi();
            entryhi = (asid & (PAGE_SIZE - 1)) | (vaddr & !(PAGE_SIZE - 1));
            write_c0_entryhi(entryhi);
            barrier!();
            tlb_probe();
            tlb_read();
            entrylo = read_c0_entrylo0();
            write_c0_entryhi(asid);
            offset = (vaddr as usize) & (PAGE_SIZE - 1);
            address = (entrylo & !(PAGE_SIZE - 1)) | offset;
        }
    }

    if address < 0x10000000 {
        cycle = mreadstr;
        event = paritystr;
    } else {
        cycle = if invoker != 0 { writestr } else { readstr };
        event = timestr;
    }
    if is_fixup != 0 { action = MIPS_BE_FIXUP; }
    if action != MIPS_BE_FIXUP {
        pr_alert_ratelimited(b"Bus error %s: %s %s %s at %#010lx\n\0".as_ptr() as *const _, kind.as_ptr() as *const _, agent.as_ptr() as *const _, cycle.as_ptr() as *const _, event.as_ptr() as *const _, address);
    }
    action
}

pub unsafe fn dec_kn01_be_handler(regs: *mut PtRegs, is_fixup: i32) -> i32 {
    dec_kn01_be_backend(regs, is_fixup, 0)
}

pub unsafe fn dec_kn01_be_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> IrqReturn {
    let csr = (CKSEG1ADDR(KN01_SLOT_BASE + KN01_CSR)) as *const u16;
    let regs = get_irq_regs();
    if (*csr & KN01_CSR_MEMERR) == 0 { return IRQ_NONE; }
    let action = dec_kn01_be_backend(regs, 0, 1);
    if action == MIPS_BE_DISCARD { return IRQ_HANDLED; }
    printk(b"Fatal bus interrupt, epc == %08lx, ra == %08lx\n\0".as_ptr() as *const _, (*regs).cp0_epc, (*regs).regs[31]);
    die(b"Unrecoverable bus error\0".as_ptr() as *const _, regs)
}

pub unsafe fn dec_kn01_be_init() {
    let csr = (CKSEG1ADDR(KN01_SLOT_BASE + KN01_CSR)) as *mut u16;
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&raw mut kn01_lock, &mut flags);
    cached_kn01_csr = *csr;
    cached_kn01_csr &= KN01_CSR_STATUS | KN01_CSR_PARDIS | KN01_CSR_TXDIS;
    cached_kn01_csr |= KN01_CSR_LEDS;
    cached_kn01_csr &= !KN01_CSR_PARDIS;
    *csr = cached_kn01_csr;
    iob();
    raw_spin_unlock_irqrestore(&raw mut kn01_lock, flags);
    dec_kn01_be_ack();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
