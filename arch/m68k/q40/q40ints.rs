/*
 * arch/m68k/q40/q40ints.c
 *
 * Copyright (C) 1999,2001 Richard Zidlicky
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 *
 * .. used to be loosely based on bvme6000ints.c
 */

// Linux and architecture header dependencies are supplied by other files.

/*
 * Q40 IRQs are defined as follows:
 *            3,4,5,6,7,10,11,14,15 : ISA dev IRQs
 *            16-31: reserved
 *            32   : keyboard int
 *            33   : frame int (50/200 Hz periodic timer)
 *            34   : sample int (10/20 KHz periodic timer)
 */

extern "C" {
    fn m68k_setup_irq_controller(chip: *mut irq_chip, handler: unsafe extern "C" fn(), first: i32, last: i32);
    fn handle_simple_irq();
    fn m68k_setup_auto_interrupt(handler: unsafe extern "C" fn(u32, *mut pt_regs));
    fn m68k_irq_startup_irq(irq: u32);
    fn master_outb(value: i32, reg: usize);
    fn master_inb(reg: usize) -> u32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u32, name: *const u8, dev_id: *mut core::ffi::c_void) -> i32;
    fn panic(message: *const u8) -> !;
    fn do_IRQ(irq: i32, fp: *mut pt_regs);
    fn floppy_hardint();
    fn local_irq_save(flags: *mut usize);
    fn legacy_timer_tick(ticks: i32);
    fn timer_heartbeat();
    fn local_irq_restore(flags: usize);
    fn pr_warn(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn disable_irq(irq: u32);
    fn enable_irq(irq: u32);

    static mut DAC_LEFT: *mut u8;
    static mut DAC_RIGHT: *mut u8;
}

#[repr(C)]
pub struct pt_regs { pub sr: u32 }
pub type irqreturn_t = i32;
pub const IRQ_HANDLED: irqreturn_t = 1;

#[repr(C)]
pub struct irq_data { pub irq: u32 }
#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_startup: Option<unsafe extern "C" fn(*mut irq_data) -> u32>,
    pub irq_shutdown: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_enable: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_disable: Option<unsafe extern "C" fn(*mut irq_data)>,
}

pub static mut q40_ablecount: [u16; 35] = [0; 35];
pub static mut q40_state: [u16; 35] = [0; 35];

unsafe extern "C" fn q40_irq_startup(data: *mut irq_data) -> u32 {
    let irq = (*data).irq;
    match irq {
        1 | 2 | 8 | 9 | 11 | 12 | 13 => {
            pr_warn(b"%s: ISA IRQ %d not implemented by HW\0".as_ptr(), b"q40_irq_startup\0".as_ptr(), irq);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn q40_irq_shutdown(_data: *mut irq_data) {}

static mut q40_irq_chip: irq_chip = irq_chip {
    name: b"q40\0".as_ptr(), irq_startup: Some(q40_irq_startup),
    irq_shutdown: Some(q40_irq_shutdown), irq_enable: Some(q40_irq_enable),
    irq_disable: Some(q40_irq_disable),
};

static mut disabled: i32 = 0;

pub unsafe extern "C" fn q40_init_IRQ() {
    m68k_setup_irq_controller(&raw mut q40_irq_chip, handle_simple_irq, 1, Q40_IRQ_MAX);
    m68k_setup_auto_interrupt(q40_irq_handler);
    m68k_irq_startup_irq(IRQ_AUTO_2);
    m68k_irq_startup_irq(IRQ_AUTO_4);
    master_outb(1, EXT_ENABLE_REG);
    master_outb(0, KEY_IRQ_ENABLE_REG);
}

pub static mut ql_ticks: i32 = 0;
static mut sound_ticks: u32 = 0;
const SVOL: u8 = 45;

pub unsafe extern "C" fn q40_mksound(hz: u32, ticks: u32) {
    if hz == 0 {
        if sound_ticks != 0 { sound_ticks = 1; }
        *DAC_LEFT = 128; *DAC_RIGHT = 128;
        return;
    }
    if sound_ticks == 0 { sound_ticks = 1000; }
    sound_ticks = ticks << 1;
}

unsafe extern "C" fn q40_timer_int(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    ql_ticks = if ql_ticks != 0 { 0 } else { 1 };
    if sound_ticks != 0 {
        let sval = if (sound_ticks & 1) != 0 { 128 - SVOL } else { 128 + SVOL };
        sound_ticks -= 1; *DAC_LEFT = sval; *DAC_RIGHT = sval;
    }
    if ql_ticks == 0 {
        let mut flags = 0usize; local_irq_save(&mut flags);
        legacy_timer_tick(1); timer_heartbeat(); local_irq_restore(flags);
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn q40_sched_init() {
    let timer_irq = Q40_IRQ_FRAME;
    if request_irq(timer_irq, q40_timer_int, 0, b"timer\0".as_ptr(), core::ptr::null_mut()) != 0 {
        panic(b"Couldn't register timer int\0".as_ptr());
    }
    master_outb(-1, FRAME_CLEAR_REG); master_outb(1, FRAME_RATE_REG);
}

#[repr(C)] struct IRQ_TABLE { mask: u32, irq: i32 }
static mut eirqs: [IRQ_TABLE; 9] = [
    IRQ_TABLE{mask:Q40_IRQ3_MASK,irq:3}, IRQ_TABLE{mask:Q40_IRQ4_MASK,irq:4},
    IRQ_TABLE{mask:Q40_IRQ14_MASK,irq:14}, IRQ_TABLE{mask:Q40_IRQ15_MASK,irq:15},
    IRQ_TABLE{mask:Q40_IRQ6_MASK,irq:6}, IRQ_TABLE{mask:Q40_IRQ7_MASK,irq:7},
    IRQ_TABLE{mask:Q40_IRQ5_MASK,irq:5}, IRQ_TABLE{mask:Q40_IRQ10_MASK,irq:10}, IRQ_TABLE{mask:0,irq:0}
];
const IRQ_INPROGRESS: u16 = 1;
static mut ccleirq: i32 = 60;
static mut mext_disabled: i32 = 0;
static mut aliased_irq: i32 = 0;

unsafe extern "C" fn q40_irq_handler(mut irq: u32, fp: *mut pt_regs) {
    let mir = master_inb(IIRQ_REG);
    if (mir & Q40_IRQ_EXT_MASK) != 0 && (master_inb(EIRQ_REG) & Q40_IRQ6_MASK) != 0 { floppy_hardint(); return; }
    if irq == 4 || irq == 6 { do_IRQ(Q40_IRQ_SAMPLE, fp); return; }
    if (mir & Q40_IRQ_FRAME_MASK) != 0 { do_IRQ(Q40_IRQ_FRAME, fp); master_outb(-1, FRAME_CLEAR_REG); }
    if (mir & (Q40_IRQ_SER_MASK | Q40_IRQ_EXT_MASK)) != 0 {
        let mer = master_inb(EIRQ_REG);
        for i in 0..8 {
            if (mer & eirqs[i].mask) != 0 {
                irq = eirqs[i].irq as u32;
                if irq > 4 && irq <= 15 && mext_disabled != 0 { break; }
                if (q40_state[irq as usize] & IRQ_INPROGRESS) != 0 {
                    (*fp).sr = (((*fp).sr & !0x700) + 0x200); disabled = 1; break;
                }
                q40_state[irq as usize] |= IRQ_INPROGRESS; do_IRQ(irq as i32, fp); q40_state[irq as usize] &= !IRQ_INPROGRESS;
                if disabled != 0 { disabled = 0; }
                return;
            }
        }
        if mer != 0 && ccleirq > 0 && aliased_irq == 0 { pr_warn(b"ISA interrupt from unknown source? EIRQ_REG = %x\n\0".as_ptr(), mer); ccleirq -= 1; }
    }
    let mir = master_inb(IIRQ_REG);
    if (mir & Q40_IRQ_KEYB_MASK) != 0 { do_IRQ(Q40_IRQ_KEYBOARD, fp); }
}

pub unsafe extern "C" fn q40_irq_enable(data: *mut irq_data) { if (*data).irq >= 5 && (*data).irq <= 15 { mext_disabled -= 1; if mext_disabled > 0 { pr_warn(b"q40_irq_enable : nested disable/enable\n\0".as_ptr()); } if mext_disabled == 0 { master_outb(1, EXT_ENABLE_REG); } } }
pub unsafe extern "C" fn q40_irq_disable(data: *mut irq_data) { if (*data).irq >= 5 && (*data).irq <= 15 { master_outb(0, EXT_ENABLE_REG); mext_disabled += 1; if mext_disabled > 1 { pr_info(b"disable_irq nesting count %d\n\0".as_ptr(), mext_disabled); } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
