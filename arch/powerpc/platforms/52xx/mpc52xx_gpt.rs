// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of mpc52xx_gpt.c. Kernel dependencies are external. */

#[repr(C)]
pub struct mpc52xx_gpt_priv {
    pub list: list_head,
    pub dev: *mut device,
    pub regs: *mut mpc52xx_gpt,
    pub lock: raw_spinlock_t,
    pub irqhost: *mut irq_domain,
    pub ipb_freq: u32,
    pub wdt_mode: u8,
    #[cfg(CONFIG_GPIOLIB)]
    pub gc: gpio_chip,
}

extern "C" {
    static mut mpc52xx_gpt_list: list_head;
    static mut mpc52xx_gpt_list_mutex: mutex;
}

pub const MPC52xx_GPT_MODE_MS_MASK: u32 = 0x07;
pub const MPC52xx_GPT_MODE_MS_IC: u32 = 0x01;
pub const MPC52xx_GPT_MODE_MS_OC: u32 = 0x02;
pub const MPC52xx_GPT_MODE_MS_PWM: u32 = 0x03;
pub const MPC52xx_GPT_MODE_MS_GPIO: u32 = 0x04;
pub const MPC52xx_GPT_MODE_GPIO_MASK: u32 = 0x30;
pub const MPC52xx_GPT_MODE_GPIO_OUT_LOW: u32 = 0x20;
pub const MPC52xx_GPT_MODE_GPIO_OUT_HIGH: u32 = 0x30;
pub const MPC52xx_GPT_MODE_COUNTER_ENABLE: u32 = 0x1000;
pub const MPC52xx_GPT_MODE_CONTINUOUS: u32 = 0x0400;
pub const MPC52xx_GPT_MODE_OPEN_DRAIN: u32 = 0x0200;
pub const MPC52xx_GPT_MODE_IRQ_EN: u32 = 0x0100;
pub const MPC52xx_GPT_MODE_WDT_EN: u32 = 0x8000;
pub const MPC52xx_GPT_MODE_ICT_MASK: u32 = 0x030000;
pub const MPC52xx_GPT_MODE_ICT_RISING: u32 = 0x010000;
pub const MPC52xx_GPT_MODE_ICT_FALLING: u32 = 0x020000;
pub const MPC52xx_GPT_MODE_ICT_TOGGLE: u32 = 0x030000;
pub const MPC52xx_GPT_MODE_WDT_PING: u8 = 0xa5;
pub const MPC52xx_GPT_STATUS_IRQMASK: u32 = 0x000f;
pub const MPC52xx_GPT_CAN_WDT: u8 = 1 << 0;
pub const MPC52xx_GPT_IS_WDT: u8 = 1 << 1;

unsafe fn mpc52xx_gpt_irq_unmask(d: *mut irq_data) {
    let gpt = irq_data_get_irq_chip_data(d); let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*gpt).lock, &mut flags);
    setbits32(&mut (*(*gpt).regs).mode, MPC52xx_GPT_MODE_IRQ_EN);
    raw_spin_unlock_irqrestore(&mut (*gpt).lock, flags);
}
unsafe fn mpc52xx_gpt_irq_mask(d: *mut irq_data) {
    let gpt = irq_data_get_irq_chip_data(d); let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*gpt).lock, &mut flags);
    clrbits32(&mut (*(*gpt).regs).mode, MPC52xx_GPT_MODE_IRQ_EN);
    raw_spin_unlock_irqrestore(&mut (*gpt).lock, flags);
}
unsafe fn mpc52xx_gpt_irq_ack(d: *mut irq_data) {
    let gpt = irq_data_get_irq_chip_data(d);
    out_be32(&mut (*(*gpt).regs).status, MPC52xx_GPT_STATUS_IRQMASK);
}
unsafe fn mpc52xx_gpt_irq_set_type(d: *mut irq_data, flow_type: u32) -> i32 {
    let gpt = irq_data_get_irq_chip_data(d); let mut flags = 0;
    let mut reg = in_be32(&(*(*gpt).regs).mode) & !MPC52xx_GPT_MODE_ICT_MASK;
    raw_spin_lock_irqsave(&mut (*gpt).lock, &mut flags);
    if flow_type & IRQF_TRIGGER_RISING != 0 { reg |= MPC52xx_GPT_MODE_ICT_RISING; }
    if flow_type & IRQF_TRIGGER_FALLING != 0 { reg |= MPC52xx_GPT_MODE_ICT_FALLING; }
    out_be32(&mut (*(*gpt).regs).mode, reg);
    raw_spin_unlock_irqrestore(&mut (*gpt).lock, flags); 0
}
unsafe fn mpc52xx_gpt_irq_cascade(desc: *mut irq_desc) {
    let gpt = irq_desc_get_handler_data(desc);
    if in_be32(&(*(*gpt).regs).status) & MPC52xx_GPT_STATUS_IRQMASK != 0 {
        generic_handle_domain_irq((*gpt).irqhost, 0);
    }
}
unsafe fn mpc52xx_gpt_irq_map(h: *mut irq_domain, virq: u32, _hw: irq_hw_number_t) -> i32 {
    let gpt = (*h).host_data;
    irq_set_chip_data(virq, gpt);
    irq_set_chip_and_handler(virq, &mpc52xx_gpt_irq_chip, handle_edge_irq); 0
}
unsafe fn mpc52xx_gpt_irq_xlate(h: *mut irq_domain, _ct: *mut device_node, intspec: *const u32, intsize: u32, out_hwirq: *mut irq_hw_number_t, out_flags: *mut u32) -> i32 {
    let gpt = (*h).host_data;
    if intsize < 1 || *intspec > 3 { return -EINVAL; }
    *out_hwirq = 0; *out_flags = *intspec; 0
}
unsafe fn mpc52xx_gpt_irq_setup(gpt: *mut mpc52xx_gpt_priv, node: *mut device_node) {
    let cascade_virq = irq_of_parse_and_map(node, 0); if cascade_virq == 0 { return; }
    (*gpt).irqhost = irq_domain_create_linear(of_fwnode_handle(node), 1, &mpc52xx_gpt_irq_ops, gpt);
    if (*gpt).irqhost.is_null() { return; }
    irq_set_chained_handler_and_data(cascade_virq, mpc52xx_gpt_irq_cascade, gpt);
    let mut flags = 0; raw_spin_lock_irqsave(&mut (*gpt).lock, &mut flags);
    let mode = in_be32(&(*(*gpt).regs).mode);
    if mode & MPC52xx_GPT_MODE_MS_MASK == 0 {
        out_be32(&mut (*(*gpt).regs).mode, mode | MPC52xx_GPT_MODE_MS_IC);
    }
    raw_spin_unlock_irqrestore(&mut (*gpt).lock, flags);
}

unsafe fn mpc52xx_gpt_do_start(gpt: *mut mpc52xx_gpt_priv, period: u64, continuous: i32, as_wdt: i32) -> i32 {
    let mut clear = MPC52xx_GPT_MODE_MS_MASK | MPC52xx_GPT_MODE_CONTINUOUS;
    let mut set = MPC52xx_GPT_MODE_MS_GPIO | MPC52xx_GPT_MODE_COUNTER_ENABLE;
    if as_wdt != 0 { clear |= MPC52xx_GPT_MODE_IRQ_EN; set |= MPC52xx_GPT_MODE_WDT_EN; }
    else if continuous != 0 { set |= MPC52xx_GPT_MODE_CONTINUOUS; }
    let mut clocks = period.wrapping_mul((*gpt).ipb_freq as u64) / 1_000_000_000;
    if clocks > 0xffff_ffff { return -EINVAL; }
    let prescale = (clocks >> 16) + 1; clocks /= prescale;
    if clocks > 0xffff { return -EINVAL; }
    let mut flags = 0; raw_spin_lock_irqsave(&mut (*gpt).lock, &mut flags);
    if as_wdt != 0 { (*gpt).wdt_mode |= MPC52xx_GPT_IS_WDT; }
    else if (*gpt).wdt_mode & MPC52xx_GPT_IS_WDT != 0 {
        raw_spin_unlock_irqrestore(&mut (*gpt).lock, flags); return -EBUSY;
    }
    out_be32(&mut (*(*gpt).regs).count, ((prescale << 16) | clocks) as u32);
    clrsetbits_be32(&mut (*(*gpt).regs).mode, clear, set);
    raw_spin_unlock_irqrestore(&mut (*gpt).lock, flags); 0
}
pub unsafe fn mpc52xx_gpt_start_timer(gpt: *mut mpc52xx_gpt_priv, period: u64, continuous: i32) -> i32 {
    mpc52xx_gpt_do_start(gpt, period, continuous, 0)
}
pub unsafe fn mpc52xx_gpt_stop_timer(gpt: *mut mpc52xx_gpt_priv) -> i32 {
    let mut flags = 0; raw_spin_lock_irqsave(&mut (*gpt).lock, &mut flags);
    if (*gpt).wdt_mode & MPC52xx_GPT_IS_WDT != 0 {
        raw_spin_unlock_irqrestore(&mut (*gpt).lock, flags); return -EBUSY;
    }
    clrbits32(&mut (*(*gpt).regs).mode, MPC52xx_GPT_MODE_COUNTER_ENABLE);
    raw_spin_unlock_irqrestore(&mut (*gpt).lock, flags); 0
}
pub unsafe fn mpc52xx_gpt_timer_period(gpt: *mut mpc52xx_gpt_priv) -> u64 {
    let mut flags = 0; raw_spin_lock_irqsave(&mut (*gpt).lock, &mut flags);
    let mut period = in_be32(&(*(*gpt).regs).count) as u64;
    raw_spin_unlock_irqrestore(&mut (*gpt).lock, flags);
    let mut prescale = period >> 16; period &= 0xffff;
    if prescale == 0 { prescale = 0x10000; }
    period * prescale * 1_000_000_000 / (*gpt).ipb_freq as u64
}

extern "C" {
    fn mpc52xx_gpt_gpio_setup(gpt: *mut mpc52xx_gpt_priv);
    fn mpc52xx_gpt_wdt_init() -> i32;
    fn mpc52xx_gpt_probe(ofdev: *mut platform_device) -> i32;
    static mut mpc52xx_gpt_irq_chip: irq_chip;
    static mut mpc52xx_gpt_irq_ops: irq_domain_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
