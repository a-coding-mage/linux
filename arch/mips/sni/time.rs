// SPDX-License-Identifier: GPL-2.0
// External Linux/MIPS declarations supplied by other translation units.

use core::ffi::c_void;

const SNI_CLOCK_TICK_RATE: u32 = 3686400;
const SNI_COUNTER2_DIV: u32 = 64;
const SNI_COUNTER0_DIV: u32 = (SNI_CLOCK_TICK_RATE / SNI_COUNTER2_DIV) / HZ;
const SNI_8254_TICK_RATE: u32 = 1193182;
const SNI_8254_TCSAMP_COUNTER: u32 = (SNI_8254_TICK_RATE / HZ) + 255;

#[repr(C)]
struct ClockEventDevice {
    name: *const i8,
    features: u32,
    rating: i32,
    irq: i32,
    cpumask: *const c_void,
    set_state_periodic: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>,
    event_handler: Option<unsafe extern "C" fn(*mut ClockEventDevice)>,
}

extern "C" {
    static mut mips_hpt_frequency: u64;
    static sni_brd_type: i32;
    static A20R_PT_CLOCK_BASE: usize;
    static A20R_PT_TIM0_ACK: usize;
    static HZ: u32;
    static SNI_A20R_IRQ_TIMER: i32;
    static SNI_BRD_10: i32;
    static SNI_BRD_10NEW: i32;
    static SNI_BRD_TOWER_OASIC: i32;
    static SNI_BRD_MINITOWER: i32;
    static CLOCK_EVT_FEAT_PERIODIC: u32;
    static IRQ_HANDLED: i32;
    static IRQF_PERCPU: u32;
    static IRQF_TIMER: u32;
    fn wmb();
    fn clockevents_register_device(evt: *mut ClockEventDevice);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> i32,
                   flags: u32, name: *const i8, dev_id: *mut c_void) -> i32;
    fn smp_processor_id() -> u32;
    fn cpumask_of(cpu: u32) -> *const c_void;
    fn printk(fmt: *const i8, ...);
    fn pr_err(fmt: *const i8, ...);
    fn outb_p(value: u8, port: u16);
    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn read_c0_count() -> u32;
    fn setup_pit_timer();
}

unsafe extern "C" fn a20r_set_periodic(_evt: *mut ClockEventDevice) -> i32 {
    (A20R_PT_CLOCK_BASE as *mut u8).add(12).write_volatile(0x34);
    wmb();
    (A20R_PT_CLOCK_BASE as *mut u8).write_volatile((SNI_COUNTER0_DIV & 0xff) as u8);
    wmb();
    (A20R_PT_CLOCK_BASE as *mut u8).write_volatile((SNI_COUNTER0_DIV >> 8) as u8);
    wmb();
    (A20R_PT_CLOCK_BASE as *mut u8).add(12).write_volatile(0xb4);
    wmb();
    (A20R_PT_CLOCK_BASE as *mut u8).add(8).write_volatile((SNI_COUNTER2_DIV & 0xff) as u8);
    wmb();
    (A20R_PT_CLOCK_BASE as *mut u8).add(8).write_volatile((SNI_COUNTER2_DIV >> 8) as u8);
    wmb();
    0
}

static mut A20R_CLOCKEVENT_DEVICE: ClockEventDevice = ClockEventDevice {
    name: b"a20r-timer\0".as_ptr() as *const i8,
    features: CLOCK_EVT_FEAT_PERIODIC,
    rating: 300,
    irq: SNI_A20R_IRQ_TIMER,
    cpumask: core::ptr::null(),
    set_state_periodic: Some(a20r_set_periodic),
    event_handler: None,
};

unsafe extern "C" fn a20r_interrupt(_irq: i32, dev_id: *mut c_void) -> i32 {
    let cd = dev_id as *mut ClockEventDevice;
    (A20R_PT_TIM0_ACK as *mut u8).write_volatile(0);
    wmb();
    if let Some(handler) = (*cd).event_handler {
        handler(cd);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn sni_a20r_timer_setup() {
    let cd = &raw mut A20R_CLOCKEVENT_DEVICE;
    let cpu = smp_processor_id();
    (*cd).cpumask = cpumask_of(cpu);
    clockevents_register_device(cd);
    if request_irq(SNI_A20R_IRQ_TIMER, a20r_interrupt, IRQF_PERCPU | IRQF_TIMER,
                   b"a20r-timer\0".as_ptr() as *const i8, cd.cast()) != 0 {
        pr_err(b"Failed to register a20r-timer interrupt\n\0".as_ptr() as *const i8);
    }
}

unsafe fn dosample() -> usize {
    let mut ct0: u32;
    let mut ct1: u32;
    let mut msb: u8;
    outb_p(0x34, 0x43);
    outb_p((SNI_8254_TCSAMP_COUNTER & 0xff) as u8, 0x40);
    outb((SNI_8254_TCSAMP_COUNTER >> 8) as u8, 0x40);
    ct0 = read_c0_count();
    loop {
        outb(0x00, 0x43);
        let _ = inb(0x40);
        msb = inb(0x40);
        ct1 = read_c0_count();
        if msb == 0 { break; }
    }
    outb(0x38, 0x43);
    ((ct1.wrapping_sub(ct0) as usize) / (500000 / HZ as usize)) * (500000 / HZ as usize)
}

pub unsafe extern "C" fn plat_time_init() {
    let mut r4k_ticks = [0usize; 3];
    let r4k_tick: usize;
    printk(b"Calibrating system timer... \0".as_ptr() as *const i8);
    let _ = dosample();
    let _ = dosample();
    loop { r4k_ticks[0] = dosample(); if r4k_ticks[0] != 0 { break; } }
    loop { r4k_ticks[1] = dosample(); if r4k_ticks[1] != 0 { break; } }
    if r4k_ticks[0] != r4k_ticks[1] {
        printk(b"warning: timer counts differ, retrying... \0".as_ptr() as *const i8);
        r4k_ticks[2] = dosample();
        if r4k_ticks[2] == r4k_ticks[0] || r4k_ticks[2] == r4k_ticks[1] { r4k_tick = r4k_ticks[2]; }
        else {
            printk(b"disagreement, using average... \0".as_ptr() as *const i8);
            r4k_tick = (r4k_ticks[0] + r4k_ticks[1] + r4k_ticks[2]) / 3;
        }
    } else { r4k_tick = r4k_ticks[0]; }
    printk(b"%d [%d.%04d MHz CPU]\n\0".as_ptr() as *const i8, r4k_tick as i32,
           (r4k_tick / (500000 / HZ as usize)) as i32,
           (r4k_tick % (500000 / HZ as usize)) as i32);
    mips_hpt_frequency = (r4k_tick as u64) * HZ as u64;
    match sni_brd_type {
        x if x == SNI_BRD_10 || x == SNI_BRD_10NEW || x == SNI_BRD_TOWER_OASIC || x == SNI_BRD_MINITOWER => sni_a20r_timer_setup(),
        _ => {}
    }
    setup_pit_timer();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
