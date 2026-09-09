// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-rpc/dma.c
 *
 *  Copyright (C) 1998 Russell King
 *
 *  DMA functions specific to RiscPC architecture
 */

const TRANSFER_SIZE: usize = 2;
const CURA: usize = 0;
const ENDA: usize = IOMD_IO0ENDA - IOMD_IO0CURA;
const CURB: usize = IOMD_IO0CURB - IOMD_IO0CURA;
const ENDB: usize = IOMD_IO0ENDB - IOMD_IO0CURA;
const CR: usize = IOMD_IO0CR - IOMD_IO0CURA;
const ST: usize = IOMD_IO0ST - IOMD_IO0CURA;

#[repr(C)]
struct iomd_dma {
    dma: dma_struct,
    base: *mut core::ffi::c_void,
    irq: core::ffi::c_int,
    state: core::ffi::c_uint,
    cur_addr: dma_addr_t,
    cur_len: core::ffi::c_uint,
    dma_addr: dma_addr_t,
    dma_len: core::ffi::c_uint,
}

unsafe fn iomd_get_next_sg(idma: *mut iomd_dma) {
    let mut end: core::ffi::c_ulong;
    let mut offset: core::ffi::c_ulong;
    let mut flags: core::ffi::c_ulong = 0;

    if !(*idma).dma.sg.is_null() {
        (*idma).cur_addr = (*idma).dma_addr;
        offset = (*idma).cur_addr & !PAGE_MASK;
        end = offset + (*idma).dma_len as core::ffi::c_ulong;
        if end > PAGE_SIZE { end = PAGE_SIZE; }
        if offset + TRANSFER_SIZE as core::ffi::c_ulong >= end { flags |= DMA_END_L as core::ffi::c_ulong; }
        (*idma).cur_len = (end - TRANSFER_SIZE as core::ffi::c_ulong) as core::ffi::c_uint;
        (*idma).dma_len -= (end - offset) as core::ffi::c_uint;
        (*idma).dma_addr += (end - offset) as dma_addr_t;
        if (*idma).dma_len == 0 {
            if (*idma).dma.sgcount > 1 {
                (*idma).dma.sg = sg_next((*idma).dma.sg);
                (*idma).dma_addr = (*(*idma).dma.sg).dma_address;
                (*idma).dma_len = (*(*idma).dma.sg).length;
                (*idma).dma.sgcount -= 1;
            } else {
                (*idma).dma.sg = core::ptr::null_mut();
                flags |= DMA_END_S as core::ffi::c_ulong;
            }
        }
    } else {
        flags = (DMA_END_S | DMA_END_L) as core::ffi::c_ulong;
        (*idma).cur_addr = 0;
        (*idma).cur_len = 0;
    }
    (*idma).cur_len |= flags as core::ffi::c_uint;
}

unsafe extern "C" fn iomd_dma_handle(irq: core::ffi::c_int, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let idma = dev_id as *mut iomd_dma;
    let base = (*idma).base as *mut u8;
    let mut state = (*idma).state;
    let mut status: core::ffi::c_uint;
    let mut cur: usize;
    let mut end: usize;
    loop {
        status = readb(base.add(ST)) as core::ffi::c_uint;
        if status & DMA_ST_INT == 0 { break; }
        if (state ^ status) & DMA_ST_AB != 0 { iomd_get_next_sg(idma); }
        // This efficiently implements state = OFL != AB ? AB : 0
        state = ((status >> 2) ^ status) & DMA_ST_AB;
        if state != 0 { cur = CURA; end = ENDA; } else { cur = CURB; end = ENDB; }
        writel((*idma).cur_addr, base.add(cur));
        writel((*idma).cur_len, base.add(end));
        if status & DMA_ST_OFL != 0 && (*idma).cur_len == (DMA_END_S | DMA_END_L) { break; }
    }
    state = !DMA_ST_AB;
    disable_irq_nosync(irq);
    (*idma).state = state;
    IRQ_HANDLED
}

unsafe extern "C" fn iomd_request_dma(_chan: core::ffi::c_uint, dma: *mut dma_t) -> core::ffi::c_int {
    let idma = container_of!(dma, iomd_dma, dma);
    request_irq((*idma).irq, iomd_dma_handle, 0, (*idma).dma.device_id, idma as *mut _)
}

unsafe extern "C" fn iomd_free_dma(_chan: core::ffi::c_uint, dma: *mut dma_t) {
    let idma = container_of!(dma, iomd_dma, dma);
    free_irq((*idma).irq, idma as *mut _);
}

static mut isa_dma_dev: device = device {
    init_name: b"fallback device\0".as_ptr() as *const _,
    coherent_dma_mask: !(0 as dma_addr_t),
    dma_mask: core::ptr::addr_of_mut!(isa_dma_dev.coherent_dma_mask),
};

unsafe extern "C" fn iomd_enable_dma(_chan: core::ffi::c_uint, dma: *mut dma_t) {
    let idma = container_of!(dma, iomd_dma, dma);
    let base = (*idma).base as *mut u8;
    let mut ctrl = TRANSFER_SIZE as core::ffi::c_uint | DMA_CR_E;
    if (*idma).dma.invalid != 0 {
        (*idma).dma.invalid = 0;
        if (*idma).dma.sg.is_null() {
            (*idma).dma.sg = core::ptr::addr_of_mut!((*idma).dma.buf);
            (*idma).dma.sgcount = 1;
            (*idma).dma.buf.length = (*idma).dma.count;
            (*idma).dma.buf.dma_address = dma_map_single(&mut isa_dma_dev, (*idma).dma.addr, (*idma).dma.count,
                if (*idma).dma.dma_mode == DMA_MODE_READ { DMA_FROM_DEVICE } else { DMA_TO_DEVICE });
        }
        (*idma).dma_addr = (*(*idma).dma.sg).dma_address;
        (*idma).dma_len = (*(*idma).dma.sg).length;
        writeb(DMA_CR_C, base.add(CR));
        (*idma).state = DMA_ST_AB;
    }
    if (*idma).dma.dma_mode == DMA_MODE_READ { ctrl |= DMA_CR_D; }
    writeb(ctrl, base.add(CR));
    enable_irq((*idma).irq);
}

unsafe extern "C" fn iomd_disable_dma(_chan: core::ffi::c_uint, dma: *mut dma_t) {
    let idma = container_of!(dma, iomd_dma, dma);
    let base = (*idma).base as *mut u8;
    let mut flags: core::ffi::c_ulong = 0;
    local_irq_save(&mut flags);
    if (*idma).state != !DMA_ST_AB { disable_irq((*idma).irq); }
    writeb(0, base.add(CR));
    local_irq_restore(flags);
}

unsafe extern "C" fn iomd_set_dma_speed(_chan: core::ffi::c_uint, _dma: *mut dma_t, cycle: core::ffi::c_int) -> core::ffi::c_int {
    let mut speed = if cycle < 188 { 3 } else if cycle <= 250 { 2 } else if cycle < 438 { 1 } else { 0 };
    let mut tcr = iomd_readb(IOMD_DMATCR);
    speed &= 3;
    match _chan { DMA_0 => tcr = (tcr & !0x03) | speed, DMA_1 => tcr = (tcr & !0x0c) | (speed << 2), DMA_2 => tcr = (tcr & !0x30) | (speed << 4), DMA_3 => tcr = (tcr & !0xc0) | (speed << 6), _ => {} }
    iomd_writeb(tcr, IOMD_DMATCR);
    speed
}

static mut iomd_dma_ops: dma_ops = dma_ops { type_: b"IOMD\0".as_ptr() as *const _, request: Some(iomd_request_dma), free: Some(iomd_free_dma), enable: Some(iomd_enable_dma), disable: Some(iomd_disable_dma), setspeed: Some(iomd_set_dma_speed), ..dma_ops::ZERO };

static mut fh: fiq_handler = fiq_handler { name: b"floppydma\0".as_ptr() as *const _ };

#[repr(C)] struct floppy_dma { dma: dma_struct, fiq: core::ffi::c_uint }

unsafe extern "C" fn floppy_enable_dma(_chan: core::ffi::c_uint, dma: *mut dma_t) {
    let fdma = container_of!(dma, floppy_dma, dma);
    let (fiqhandler_start, fiqhandler_length);
    let mut regs: pt_regs = core::mem::zeroed();
    if !(*fdma).dma.sg.is_null() { BUG(); }
    if (*fdma).dma.dma_mode == DMA_MODE_READ {
        extern "C" { static mut floppy_fiqin_start: u8; static mut floppy_fiqin_end: u8; }
        fiqhandler_start = &mut floppy_fiqin_start as *mut u8 as *mut core::ffi::c_void;
        fiqhandler_length = (&floppy_fiqin_end as *const u8 as usize) - (&floppy_fiqin_start as *const u8 as usize);
    } else {
        extern "C" { static mut floppy_fiqout_start: u8; static mut floppy_fiqout_end: u8; }
        fiqhandler_start = &mut floppy_fiqout_start as *mut u8 as *mut core::ffi::c_void;
        fiqhandler_length = (&floppy_fiqout_end as *const u8 as usize) - (&floppy_fiqout_start as *const u8 as usize);
    }
    regs.ARM_r9 = (*fdma).dma.count;
    regs.ARM_r10 = (*fdma).dma.addr as core::ffi::c_ulong;
    regs.ARM_fp = FLOPPYDMA_BASE as core::ffi::c_ulong;
    if claim_fiq(&mut fh) != 0 { printk(b"floppydma: couldn't claim FIQ.\n\0".as_ptr() as *const _); return; }
    set_fiq_handler(fiqhandler_start, fiqhandler_length);
    set_fiq_regs(&regs);
    enable_fiq((*fdma).fiq);
}

unsafe extern "C" fn floppy_disable_dma(_chan: core::ffi::c_uint, dma: *mut dma_t) { let fdma = container_of!(dma, floppy_dma, dma); disable_fiq((*fdma).fiq); release_fiq(&mut fh); }
unsafe extern "C" fn floppy_get_residue(_chan: core::ffi::c_uint, _dma: *mut dma_t) -> core::ffi::c_int { let mut regs: pt_regs = core::mem::zeroed(); get_fiq_regs(&mut regs); regs.ARM_r9 as core::ffi::c_int }

static mut floppy_dma_ops: dma_ops = dma_ops { type_: b"FIQDMA\0".as_ptr() as *const _, enable: Some(floppy_enable_dma), disable: Some(floppy_disable_dma), residue: Some(floppy_get_residue), ..dma_ops::ZERO };
// This is virtual DMA - we don't need anything here.
unsafe extern "C" fn sound_enable_disable_dma(_chan: core::ffi::c_uint, _dma: *mut dma_t) {}
static mut sound_dma_ops: dma_ops = dma_ops { type_: b"VIRTUAL\0".as_ptr() as *const _, enable: Some(sound_enable_disable_dma), disable: Some(sound_enable_disable_dma), ..dma_ops::ZERO };

static mut iomd_dma: [iomd_dma; 6] = [unsafe { core::mem::zeroed() }; 6];
static mut floppy_dma: floppy_dma = floppy_dma { dma: dma_struct { d_ops: unsafe { &mut floppy_dma_ops }, ..unsafe { core::mem::zeroed() } }, fiq: FIQ_FLOPPYDATA };
static mut sound_dma: dma_t = dma_t { d_ops: unsafe { &mut sound_dma_ops }, ..unsafe { core::mem::zeroed() } };

unsafe extern "C" fn rpc_dma_init() -> core::ffi::c_int {
    iomd_writeb(0, IOMD_IO0CR); iomd_writeb(0, IOMD_IO1CR); iomd_writeb(0, IOMD_IO2CR); iomd_writeb(0, IOMD_IO3CR); iomd_writeb(0xa0, IOMD_DMATCR);
    iomd_writeb(DMA_EXT_IO3 | DMA_EXT_IO2, IOMD_DMAEXT);
    (*core::ptr::addr_of_mut!(iomd_dma[DMA_0])).base = (IOMD_BASE + IOMD_IO0CURA) as *mut _; (*core::ptr::addr_of_mut!(iomd_dma[DMA_0])).irq = IRQ_DMA0;
    (*core::ptr::addr_of_mut!(iomd_dma[DMA_1])).base = (IOMD_BASE + IOMD_IO1CURA) as *mut _; (*core::ptr::addr_of_mut!(iomd_dma[DMA_1])).irq = IRQ_DMA1;
    (*core::ptr::addr_of_mut!(iomd_dma[DMA_2])).base = (IOMD_BASE + IOMD_IO2CURA) as *mut _; (*core::ptr::addr_of_mut!(iomd_dma[DMA_2])).irq = IRQ_DMA2;
    (*core::ptr::addr_of_mut!(iomd_dma[DMA_3])).base = (IOMD_BASE + IOMD_IO3CURA) as *mut _; (*core::ptr::addr_of_mut!(iomd_dma[DMA_3])).irq = IRQ_DMA3;
    (*core::ptr::addr_of_mut!(iomd_dma[DMA_S0])).base = (IOMD_BASE + IOMD_SD0CURA) as *mut _; (*core::ptr::addr_of_mut!(iomd_dma[DMA_S0])).irq = IRQ_DMAS0;
    (*core::ptr::addr_of_mut!(iomd_dma[DMA_S1])).base = (IOMD_BASE + IOMD_SD1CURA) as *mut _; (*core::ptr::addr_of_mut!(iomd_dma[DMA_S1])).irq = IRQ_DMAS1;
    let mut i = DMA_0; while i <= DMA_S1 { (*core::ptr::addr_of_mut!(iomd_dma[i])).dma.d_ops = &mut iomd_dma_ops; let ret = isa_dma_add(i, core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(iomd_dma[i])).dma)); if ret != 0 { printk(b"IOMDDMA%u: unable to register: %d\n\0".as_ptr() as *const _, i, ret); } i += 1; }
    let ret = isa_dma_add(DMA_VIRTUAL_FLOPPY, core::ptr::addr_of_mut!(floppy_dma.dma)); if ret != 0 { printk(b"IOMDFLOPPY: unable to register: %d\n\0".as_ptr() as *const _, ret); }
    let ret = isa_dma_add(DMA_VIRTUAL_SOUND, core::ptr::addr_of_mut!(sound_dma)); if ret != 0 { printk(b"IOMDSOUND: unable to register: %d\n\0".as_ptr() as *const _, ret); }
    0
}

core_initcall!(rpc_dma_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
