/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

// Dependencies supplied by the surrounding kernel and BCM63xx implementation.

static mut TIMER_REG_LOCK: RawSpinlock = DEFINE_RAW_SPINLOCK!();
static mut TIMER_DATA_LOCK: RawSpinlock = DEFINE_RAW_SPINLOCK!();
static mut PERIPH_CLK: *mut Clk = core::ptr::null_mut();

#[repr(C)]
struct TimerData {
    cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    data: *mut core::ffi::c_void,
}

static mut TIMER_DATA: [TimerData; BCM63XX_TIMER_COUNT as usize] = [
    TimerData { cb: None, data: core::ptr::null_mut() };
    BCM63XX_TIMER_COUNT as usize
];

unsafe extern "C" fn timer_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> IrqReturn {
    let mut stat: u32;
    let mut i: i32;

    raw_spin_lock(&raw mut TIMER_REG_LOCK);
    stat = bcm_timer_readl(TIMER_IRQSTAT_REG);
    bcm_timer_writel(stat, TIMER_IRQSTAT_REG);
    raw_spin_unlock(&raw mut TIMER_REG_LOCK);

    i = 0;
    while i < BCM63XX_TIMER_COUNT {
        if (stat & TIMER_IRQSTAT_TIMER_CAUSE(i)) == 0 {
            i += 1;
            continue;
        }

        raw_spin_lock(&raw mut TIMER_DATA_LOCK);
        if TIMER_DATA[i as usize].cb.is_none() {
            raw_spin_unlock(&raw mut TIMER_DATA_LOCK);
            i += 1;
            continue;
        }

        if let Some(callback) = TIMER_DATA[i as usize].cb {
            callback(TIMER_DATA[i as usize].data);
        }
        raw_spin_unlock(&raw mut TIMER_DATA_LOCK);
        i += 1;
    }

    IRQ_HANDLED
}

pub unsafe extern "C" fn bcm63xx_timer_enable(id: i32) -> i32 {
    let mut reg: u32;
    let mut flags: UnsignedLong;

    if id >= BCM63XX_TIMER_COUNT { return -EINVAL; }

    raw_spin_lock_irqsave(&raw mut TIMER_REG_LOCK, &mut flags);
    reg = bcm_timer_readl(TIMER_CTLx_REG(id));
    reg |= TIMER_CTL_ENABLE_MASK;
    bcm_timer_writel(reg, TIMER_CTLx_REG(id));
    reg = bcm_timer_readl(TIMER_IRQSTAT_REG);
    reg |= TIMER_IRQSTAT_TIMER_IR_EN(id);
    bcm_timer_writel(reg, TIMER_IRQSTAT_REG);
    raw_spin_unlock_irqrestore(&raw mut TIMER_REG_LOCK, flags);
    0
}

pub unsafe extern "C" fn bcm63xx_timer_disable(id: i32) -> i32 {
    let mut reg: u32;
    let mut flags: UnsignedLong;

    if id >= BCM63XX_TIMER_COUNT { return -EINVAL; }
    raw_spin_lock_irqsave(&raw mut TIMER_REG_LOCK, &mut flags);
    reg = bcm_timer_readl(TIMER_CTLx_REG(id));
    reg &= !TIMER_CTL_ENABLE_MASK;
    bcm_timer_writel(reg, TIMER_CTLx_REG(id));
    reg = bcm_timer_readl(TIMER_IRQSTAT_REG);
    reg &= !TIMER_IRQSTAT_TIMER_IR_EN(id);
    bcm_timer_writel(reg, TIMER_IRQSTAT_REG);
    raw_spin_unlock_irqrestore(&raw mut TIMER_REG_LOCK, flags);
    0
}

pub unsafe extern "C" fn bcm63xx_timer_register(
    id: i32,
    callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mut flags: UnsignedLong;
    let mut ret = 0;
    if id >= BCM63XX_TIMER_COUNT || callback.is_none() { return -EINVAL; }
    raw_spin_lock_irqsave(&raw mut TIMER_DATA_LOCK, &mut flags);
    if TIMER_DATA[id as usize].cb.is_some() {
        ret = -EBUSY;
    } else {
        TIMER_DATA[id as usize].cb = callback;
        TIMER_DATA[id as usize].data = data;
    }
    raw_spin_unlock_irqrestore(&raw mut TIMER_DATA_LOCK, flags);
    ret
}

pub unsafe extern "C" fn bcm63xx_timer_unregister(id: i32) {
    let mut flags: UnsignedLong;
    if id >= BCM63XX_TIMER_COUNT { return; }
    raw_spin_lock_irqsave(&raw mut TIMER_DATA_LOCK, &mut flags);
    TIMER_DATA[id as usize].cb = None;
    raw_spin_unlock_irqrestore(&raw mut TIMER_DATA_LOCK, flags);
}

pub unsafe extern "C" fn bcm63xx_timer_countdown(countdown_us: u32) -> u32 {
    (clk_get_rate(PERIPH_CLK) / (1000 * 1000)) * countdown_us
}

pub unsafe extern "C" fn bcm63xx_timer_set(id: i32, monotonic: i32, countdown_us: u32) -> i32 {
    let mut reg: u32;
    let countdown = bcm63xx_timer_countdown(countdown_us);
    let mut flags: UnsignedLong;
    if id >= BCM63XX_TIMER_COUNT || (countdown & !TIMER_CTL_COUNTDOWN_MASK) != 0 { return -EINVAL; }
    raw_spin_lock_irqsave(&raw mut TIMER_REG_LOCK, &mut flags);
    reg = bcm_timer_readl(TIMER_CTLx_REG(id));
    if monotonic != 0 { reg &= !TIMER_CTL_MONOTONIC_MASK; } else { reg |= TIMER_CTL_MONOTONIC_MASK; }
    reg &= !TIMER_CTL_COUNTDOWN_MASK;
    reg |= countdown;
    bcm_timer_writel(reg, TIMER_CTLx_REG(id));
    raw_spin_unlock_irqrestore(&raw mut TIMER_REG_LOCK, flags);
    0
}

unsafe extern "C" fn bcm63xx_timer_init() -> i32 {
    let mut reg = bcm_timer_readl(TIMER_IRQSTAT_REG);
    reg &= !TIMER_IRQSTAT_TIMER0_IR_EN;
    reg &= !TIMER_IRQSTAT_TIMER1_IR_EN;
    reg &= !TIMER_IRQSTAT_TIMER2_IR_EN;
    bcm_timer_writel(reg, TIMER_IRQSTAT_REG);
    PERIPH_CLK = clk_get(core::ptr::null_mut(), c"periph".as_ptr());
    if IS_ERR(PERIPH_CLK) { return -ENODEV; }
    let irq = bcm63xx_get_irq_number(IRQ_TIMER);
    let ret = request_irq(irq, Some(timer_interrupt), 0, c"bcm63xx_timer".as_ptr(), core::ptr::null_mut());
    if ret != 0 { pr_err!("{}: failed to register irq\n", "bcm63xx_timer_init"); return ret; }
    0
}

arch_initcall!(bcm63xx_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
