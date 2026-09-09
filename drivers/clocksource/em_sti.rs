// SPDX-License-Identifier: GPL-2.0-only
/*
 * Emma Mobile Timer Support - STI
 *
 *  Copyright (C) 2012 Magnus Damm
 */

// Linux kernel dependencies supplied by the surrounding translation.

#[repr(u32)]
enum User { UserClocksource, UserClockevent, UserNr }

#[repr(C)]
struct EmStiPriv {
    base: *mut core::ffi::c_void,
    clk: *mut Clk,
    pdev: *mut PlatformDevice,
    active: [u32; User::UserNr as usize],
    rate: usize,
    lock: RawSpinlock,
    ced: ClockEventDevice,
    cs: Clocksource,
}

const STI_CONTROL: i32 = 0x00;
const STI_COMPA_H: i32 = 0x10;
const STI_COMPA_L: i32 = 0x14;
const STI_COMPB_H: i32 = 0x18;
const STI_COMPB_L: i32 = 0x1c;
const STI_COUNT_H: i32 = 0x20;
const STI_COUNT_L: i32 = 0x24;
const STI_COUNT_RAW_H: i32 = 0x28;
const STI_COUNT_RAW_L: i32 = 0x2c;
const STI_SET_H: i32 = 0x30;
const STI_SET_L: i32 = 0x34;
const STI_INTSTATUS: i32 = 0x40;
const STI_INTRAWSTATUS: i32 = 0x44;
const STI_INTENSET: i32 = 0x48;
const STI_INTENCLR: i32 = 0x4c;
const STI_INTFFCLR: i32 = 0x50;

unsafe fn em_sti_read(p: *mut EmStiPriv, offs: i32) -> usize {
    ioread32((*p).base.add(offs as usize)) as usize
}
unsafe fn em_sti_write(p: *mut EmStiPriv, offs: i32, value: usize) {
    iowrite32(value as u32, (*p).base.add(offs as usize));
}
unsafe fn em_sti_enable(p: *mut EmStiPriv) -> i32 {
    let ret = clk_enable((*p).clk);
    if ret != 0 { dev_err(&(*p).pdev, "cannot enable clock\n"); return ret; }
    em_sti_write(p, STI_SET_H, 0x40000000);
    em_sti_write(p, STI_SET_L, 0);
    em_sti_write(p, STI_INTENCLR, 3);
    em_sti_write(p, STI_INTFFCLR, 3);
    em_sti_write(p, STI_CONTROL, 1);
    0
}
unsafe fn em_sti_disable(p: *mut EmStiPriv) {
    em_sti_write(p, STI_INTENCLR, 3);
    clk_disable((*p).clk);
}
unsafe fn em_sti_count(p: *mut EmStiPriv) -> u64 {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut (*p).lock, &mut flags);
    let mut ticks = ((em_sti_read(p, STI_COUNT_H) & 0xffff) as u64) << 32;
    ticks |= em_sti_read(p, STI_COUNT_L) as u64;
    raw_spin_unlock_irqrestore(&mut (*p).lock, flags);
    ticks
}
unsafe fn em_sti_set_next(p: *mut EmStiPriv, next: u64) -> u64 {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut (*p).lock, &mut flags);
    em_sti_write(p, STI_INTENCLR, 1);
    em_sti_write(p, STI_COMPA_H, (next >> 32) as usize);
    em_sti_write(p, STI_COMPA_L, (next & 0xffff_ffff) as usize);
    em_sti_write(p, STI_INTFFCLR, 1);
    em_sti_write(p, STI_INTENSET, 1);
    raw_spin_unlock_irqrestore(&mut (*p).lock, flags);
    next
}
unsafe extern "C" fn em_sti_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> IrqReturn {
    let p = dev_id as *mut EmStiPriv;
    ((*p).ced.event_handler.unwrap())(&mut (*p).ced);
    IRQ_HANDLED
}
unsafe fn em_sti_start(p: *mut EmStiPriv, user: usize) -> i32 {
    let mut flags = 0usize;
    let mut ret = 0;
    raw_spin_lock_irqsave(&mut (*p).lock, &mut flags);
    let used_before = (*p).active[0] | (*p).active[1];
    if used_before == 0 { ret = em_sti_enable(p); }
    if ret == 0 { (*p).active[user] = 1; }
    raw_spin_unlock_irqrestore(&mut (*p).lock, flags);
    ret
}
unsafe fn em_sti_stop(p: *mut EmStiPriv, user: usize) {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut (*p).lock, &mut flags);
    let used_before = (*p).active[0] | (*p).active[1];
    (*p).active[user] = 0;
    let used_after = (*p).active[0] | (*p).active[1];
    if used_before != 0 && used_after == 0 { em_sti_disable(p); }
    raw_spin_unlock_irqrestore(&mut (*p).lock, flags);
}
unsafe fn cs_to_em_sti(cs: *mut Clocksource) -> *mut EmStiPriv {
    (cs as *mut u8).sub(core::mem::offset_of!(EmStiPriv, cs)) as *mut EmStiPriv
}
unsafe extern "C" fn em_sti_clocksource_read(cs: *mut Clocksource) -> u64 { em_sti_count(cs_to_em_sti(cs)) }
unsafe extern "C" fn em_sti_clocksource_enable(cs: *mut Clocksource) -> i32 { em_sti_start(cs_to_em_sti(cs), 0) }
unsafe extern "C" fn em_sti_clocksource_disable(cs: *mut Clocksource) { em_sti_stop(cs_to_em_sti(cs), 0); }
unsafe extern "C" fn em_sti_clocksource_resume(cs: *mut Clocksource) { em_sti_clocksource_enable(cs); }
unsafe fn em_sti_register_clocksource(p: *mut EmStiPriv) -> i32 {
    let cs = &mut (*p).cs;
    cs.name = dev_name(&(*p).pdev); cs.rating = 200;
    cs.read = Some(em_sti_clocksource_read); cs.enable = Some(em_sti_clocksource_enable);
    cs.disable = Some(em_sti_clocksource_disable); cs.suspend = Some(em_sti_clocksource_disable);
    cs.resume = Some(em_sti_clocksource_resume); cs.mask = clocksource_mask(48);
    cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    dev_info(&(*p).pdev, "used as clock source\n");
    clocksource_register_hz(cs, (*p).rate); 0
}
unsafe fn ced_to_em_sti(ced: *mut ClockEventDevice) -> *mut EmStiPriv {
    (ced as *mut u8).sub(core::mem::offset_of!(EmStiPriv, ced)) as *mut EmStiPriv
}
unsafe extern "C" fn em_sti_clock_event_shutdown(ced: *mut ClockEventDevice) -> i32 { em_sti_stop(ced_to_em_sti(ced), 1); 0 }
unsafe extern "C" fn em_sti_clock_event_set_oneshot(ced: *mut ClockEventDevice) -> i32 {
    let p = ced_to_em_sti(ced);
    dev_info(&(*p).pdev, "used for oneshot clock events\n"); em_sti_start(p, 1); 0
}
unsafe extern "C" fn em_sti_clock_event_next(delta: usize, ced: *mut ClockEventDevice) -> i32 {
    let p = ced_to_em_sti(ced);
    let next = em_sti_set_next(p, em_sti_count(p).wrapping_add(delta as u64));
    (!(em_sti_count(p) < next.wrapping_sub(1))) as i32
}
unsafe fn em_sti_register_clockevent(p: *mut EmStiPriv) {
    let ced = &mut (*p).ced;
    ced.name = dev_name(&(*p).pdev); ced.features = CLOCK_EVT_FEAT_ONESHOT; ced.rating = 200;
    ced.cpumask = cpu_possible_mask; ced.set_next_event = Some(em_sti_clock_event_next);
    ced.set_state_shutdown = Some(em_sti_clock_event_shutdown);
    ced.set_state_oneshot = Some(em_sti_clock_event_set_oneshot);
    dev_info(&(*p).pdev, "used for clock events\n");
    clockevents_config_and_register(ced, (*p).rate, 2, 0xffff_ffff);
}

// External kernel declarations and driver registration are supplied by the surrounding translation.
unsafe extern "C" fn em_sti_probe(pdev: *mut PlatformDevice) -> i32;
unsafe extern "C" fn em_sti_init() -> i32;
unsafe extern "C" fn em_sti_exit();


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
