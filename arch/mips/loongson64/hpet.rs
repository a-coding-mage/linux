// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

const SMBUS_CFG_BASE: usize = loongson_sysconf.ht_control_base + 0x0300a000;
const SMBUS_PCI_REG40: i32 = 0x40;
const SMBUS_PCI_REG64: i32 = 0x64;
const SMBUS_PCI_REGB4: i32 = 0xb4;

const HPET_MIN_CYCLES: i32 = 16;
const HPET_MIN_PROG_DELTA: i32 = HPET_MIN_CYCLES * 12;

static mut hpet_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut hpet_clockevent_device: per_cpu::<clock_event_device> = DEFINE_PER_CPU!();

unsafe fn smbus_read(offset: i32) -> u32 {
    core::ptr::read_volatile((SMBUS_CFG_BASE as *const u8).add(offset as usize) as *const u32)
}

unsafe fn smbus_write(offset: i32, data: i32) {
    core::ptr::write_volatile((SMBUS_CFG_BASE as *mut u8).add(offset as usize) as *mut u32, data as u32);
}

unsafe fn smbus_enable(offset: i32, bit: i32) {
    let mut cfg = smbus_read(offset);
    cfg |= bit as u32;
    smbus_write(offset, cfg as i32);
}

unsafe fn hpet_read(offset: i32) -> i32 {
    core::ptr::read_volatile((HPET_MMIO_ADDR as *const u8).add(offset as usize) as *const u32) as i32
}

unsafe fn hpet_write(offset: i32, data: i32) {
    core::ptr::write_volatile((HPET_MMIO_ADDR as *mut u8).add(offset as usize) as *mut u32, data as u32);
}

unsafe fn hpet_start_counter() {
    let mut cfg = hpet_read(HPET_CFG);
    cfg |= HPET_CFG_ENABLE;
    hpet_write(HPET_CFG, cfg);
}

unsafe fn hpet_stop_counter() {
    let mut cfg = hpet_read(HPET_CFG);
    cfg &= !HPET_CFG_ENABLE;
    hpet_write(HPET_CFG, cfg);
}

unsafe fn hpet_reset_counter() {
    hpet_write(HPET_COUNTER, 0);
    hpet_write(HPET_COUNTER + 4, 0);
}

unsafe fn hpet_restart_counter() {
    hpet_stop_counter();
    hpet_reset_counter();
    hpet_start_counter();
}

unsafe fn hpet_enable_legacy_int() {
    // Do nothing on Loongson-3
}

unsafe fn hpet_set_state_periodic(evt: *mut clock_event_device) -> i32 {
    let _ = evt;
    spin_lock(&mut hpet_lock);
    pr_info!("set clock event to periodic mode!\n");
    hpet_stop_counter();
    let mut cfg = hpet_read(HPET_T0_CFG);
    cfg &= !HPET_TN_LEVEL;
    cfg |= HPET_TN_ENABLE | HPET_TN_PERIODIC | HPET_TN_SETVAL | HPET_TN_32BIT;
    hpet_write(HPET_T0_CFG, cfg);
    hpet_write(HPET_T0_CMP, HPET_COMPARE_VAL);
    udelay(1);
    hpet_write(HPET_T0_CMP, HPET_COMPARE_VAL);
    hpet_start_counter();
    spin_unlock(&mut hpet_lock);
    0
}

unsafe fn hpet_set_state_shutdown(evt: *mut clock_event_device) -> i32 {
    let _ = evt;
    spin_lock(&mut hpet_lock);
    let mut cfg = hpet_read(HPET_T0_CFG);
    cfg &= !HPET_TN_ENABLE;
    hpet_write(HPET_T0_CFG, cfg);
    spin_unlock(&mut hpet_lock);
    0
}

unsafe fn hpet_set_state_oneshot(evt: *mut clock_event_device) -> i32 {
    let _ = evt;
    spin_lock(&mut hpet_lock);
    pr_info!("set clock event to one shot mode!\n");
    let mut cfg = hpet_read(HPET_T0_CFG);
    // Timer0 type: 1 is periodic interrupt; 0 is non-periodic (oneshot) interrupt.
    cfg &= !HPET_TN_PERIODIC;
    cfg |= HPET_TN_ENABLE | HPET_TN_32BIT;
    hpet_write(HPET_T0_CFG, cfg);
    spin_unlock(&mut hpet_lock);
    0
}

unsafe fn hpet_tick_resume(evt: *mut clock_event_device) -> i32 {
    let _ = evt;
    spin_lock(&mut hpet_lock);
    hpet_enable_legacy_int();
    spin_unlock(&mut hpet_lock);
    0
}

unsafe fn hpet_next_event(delta: c_ulong, evt: *mut clock_event_device) -> i32 {
    let _ = evt;
    let mut cnt = hpet_read(HPET_COUNTER) as u32;
    cnt = cnt.wrapping_add(delta as u32);
    hpet_write(HPET_T0_CMP, cnt as i32);
    let res = (cnt as i32).wrapping_sub(hpet_read(HPET_COUNTER));
    if res < HPET_MIN_CYCLES { -ETIME } else { 0 }
}

unsafe fn hpet_irq_handler(irq: i32, data: *mut c_void) -> irqreturn_t {
    let _ = (irq, data);
    let is_irq = hpet_read(HPET_STATUS);
    if is_irq & HPET_T0_IRS != 0 {
        hpet_write(HPET_STATUS, HPET_T0_IRS);
        let cpu = smp_processor_id();
        let cd = per_cpu_ptr(&mut hpet_clockevent_device, cpu);
        ((*cd).event_handler)(cd);
        return IRQ_HANDLED;
    }
    IRQ_NONE
}

unsafe fn hpet_setup() {
    smbus_write(SMBUS_PCI_REGB4, HPET_ADDR);
    smbus_enable(SMBUS_PCI_REG40, 1 << 28);
    smbus_enable(SMBUS_PCI_REG64, 1 << 10);
    hpet_enable_legacy_int();
}

pub unsafe fn setup_hpet_timer() {
    let flags = IRQF_NOBALANCING | IRQF_TIMER;
    let cpu = smp_processor_id();
    hpet_setup();
    let cd = per_cpu_ptr(&mut hpet_clockevent_device, cpu);
    (*cd).name = "hpet";
    (*cd).rating = 100;
    (*cd).features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    (*cd).set_state_shutdown = Some(hpet_set_state_shutdown);
    (*cd).set_state_periodic = Some(hpet_set_state_periodic);
    (*cd).set_state_oneshot = Some(hpet_set_state_oneshot);
    (*cd).tick_resume = Some(hpet_tick_resume);
    (*cd).set_next_event = Some(hpet_next_event);
    (*cd).irq = HPET_T0_IRQ;
    (*cd).cpumask = cpumask_of(cpu);
    clockevent_set_clock(cd, HPET_FREQ);
    (*cd).max_delta_ns = clockevent_delta2ns(0x7fffffff, cd);
    (*cd).max_delta_ticks = 0x7fffffff;
    (*cd).min_delta_ns = clockevent_delta2ns(HPET_MIN_PROG_DELTA, cd);
    (*cd).min_delta_ticks = HPET_MIN_PROG_DELTA;
    clockevents_register_device(cd);
    if request_irq(HPET_T0_IRQ, Some(hpet_irq_handler), flags, "hpet", core::ptr::null_mut()) != 0 {
        pr_err!("Failed to request irq %d (hpet)\n", HPET_T0_IRQ);
    }
    pr_info!("hpet clock event device register\n");
}

unsafe fn hpet_read_counter(cs: *mut clocksource) -> u64 {
    let _ = cs;
    hpet_read(HPET_COUNTER) as u64
}

unsafe fn hpet_suspend(cs: *mut clocksource) { let _ = cs; }

unsafe fn hpet_resume(cs: *mut clocksource) {
    let _ = cs;
    hpet_setup();
    hpet_restart_counter();
}

static mut csrc_hpet: clocksource = clocksource {
    name: "hpet",
    // MIPS clocksource rating is less than 300, so HPET is better.
    rating: 300,
    read: Some(hpet_read_counter),
    mask: CLOCKSOURCE_MASK(32),
    // Oneshoot mode works normally with this flag.
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    suspend: Some(hpet_suspend),
    resume: Some(hpet_resume),
    mult: 0,
    shift: 10,
};

pub unsafe fn init_hpet_clocksource() -> i32 {
    csrc_hpet.mult = clocksource_hz2mult(HPET_FREQ, csrc_hpet.shift);
    clocksource_register_hz(&mut csrc_hpet, HPET_FREQ)
}

// arch_initcall(init_hpet_clocksource);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
