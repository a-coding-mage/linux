/* Direct Rust translation of irq.c. */

static mut ipic_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut epic_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut irq_stat_addr: [u32; 2] = [0; 2];
static mut irq_mask_addr: [u32; 2] = [0; 2];
static mut dispatch_internal: Option<unsafe extern "C" fn(i32)> = None;
static mut is_ext_irq_cascaded: i32 = 0;
static mut ext_irq_count: u32 = 0;
static mut ext_irq_start: u32 = 0;
static mut ext_irq_end: u32 = 0;
static mut ext_irq_cfg_reg1: u32 = 0;
static mut ext_irq_cfg_reg2: u32 = 0;
static mut internal_irq_mask: Option<unsafe extern "C" fn(*mut irq_data)> = None;
static mut internal_irq_unmask: Option<unsafe extern "C" fn(*mut irq_data, *const cpumask)> = None;

#[inline]
unsafe fn get_ext_irq_perf_reg(irq: i32) -> u32 {
    if irq < 4 { ext_irq_cfg_reg1 } else { ext_irq_cfg_reg2 }
}

#[inline]
unsafe fn handle_internal(intbit: i32) {
    if is_ext_irq_cascaded != 0 && intbit as u32 >= ext_irq_start && intbit as u32 <= ext_irq_end {
        do_IRQ(intbit - ext_irq_start as i32 + IRQ_EXTERNAL_BASE);
    } else {
        do_IRQ(intbit + IRQ_INTERNAL_BASE);
    }
}

#[inline]
unsafe fn enable_irq_for_cpu(cpu: i32, d: *mut irq_data, m: *const cpumask) -> bool {
    let mut enable = cpu_online(cpu);
    #[cfg(CONFIG_SMP)]
    {
        if !m.is_null() { enable &= cpumask_test_cpu(cpu, m); }
        else if irqd_affinity_was_set(d) { enable &= cpumask_test_cpu(cpu, irq_data_get_affinity_mask(d)); }
    }
    enable
}

/* dispatch internal devices IRQ (uart, enet, watchdog, ...). */
macro_rules! build_ipic_internal {
    ($width:expr, $dispatch:ident, $mask:ident, $unmask:ident) => {
        unsafe extern "C" fn $dispatch(cpu: i32) {
            let mut pending = [0u32; $width / 32];
            let mut irqs_pending = false;
            static mut i: [u32; 2] = [0; 2];
            let next = &mut i[cpu as usize];
            let flags: c_ulong;
            spin_lock_irqsave(&mut ipic_lock, &flags);
            let mut tgt = $width / 32;
            for src in 0..($width / 32) {
                let mut val = bcm_readl(irq_stat_addr[cpu as usize] + src as u32 * core::mem::size_of::<u32>() as u32);
                val &= bcm_readl(irq_mask_addr[cpu as usize] + src as u32 * core::mem::size_of::<u32>() as u32);
                tgt -= 1;
                pending[tgt] = val;
                if val != 0 { irqs_pending = true; }
            }
            spin_unlock_irqrestore(&mut ipic_lock, flags);
            if !irqs_pending { return; }
            loop {
                let to_call = *next;
                *next = (*next + 1) & ($width - 1);
                if pending[(to_call / 32) as usize] & (1u32 << (to_call & 0x1f)) != 0 {
                    handle_internal(to_call as i32);
                    break;
                }
            }
        }
        unsafe extern "C" fn $mask(d: *mut irq_data) {
            let irq = (*d).irq - IRQ_INTERNAL_BASE;
            let reg = (irq / 32) ^ ($width / 32 - 1);
            let bit = irq & 0x1f;
            let flags: c_ulong;
            spin_lock_irqsave(&mut ipic_lock, &flags);
            let mut cpu = 0;
            for_each_present_cpu!(cpu) {
                if irq_mask_addr[cpu as usize] == 0 { break; }
                let mut val = bcm_readl(irq_mask_addr[cpu as usize] + reg as u32 * core::mem::size_of::<u32>() as u32);
                val &= !(1u32 << bit);
                bcm_writel(val, irq_mask_addr[cpu as usize] + reg as u32 * core::mem::size_of::<u32>() as u32);
            }
            spin_unlock_irqrestore(&mut ipic_lock, flags);
        }
        unsafe extern "C" fn $unmask(d: *mut irq_data, m: *const cpumask) {
            let irq = (*d).irq - IRQ_INTERNAL_BASE;
            let reg = (irq / 32) ^ ($width / 32 - 1);
            let bit = irq & 0x1f;
            let flags: c_ulong;
            spin_lock_irqsave(&mut ipic_lock, &flags);
            let mut cpu = 0;
            for_each_present_cpu!(cpu) {
                if irq_mask_addr[cpu as usize] == 0 { break; }
                let addr = irq_mask_addr[cpu as usize] + reg as u32 * core::mem::size_of::<u32>() as u32;
                let mut val = bcm_readl(addr);
                if enable_irq_for_cpu(cpu, d, m) { val |= 1u32 << bit; } else { val &= !(1u32 << bit); }
                bcm_writel(val, addr);
            }
            spin_unlock_irqrestore(&mut ipic_lock, flags);
        }
    };
}
build_ipic_internal!(32, __dispatch_internal_32, __internal_irq_mask_32, __internal_irq_unmask_32);
build_ipic_internal!(64, __dispatch_internal_64, __internal_irq_mask_64, __internal_irq_unmask_64);

pub unsafe extern "C" fn plat_irq_dispatch() {
    loop {
        let cause = read_c0_cause() & read_c0_status() & ST0_IM;
        if cause == 0 { break; }
        if cause & CAUSEF_IP7 != 0 { do_IRQ(7); }
        if cause & CAUSEF_IP0 != 0 { do_IRQ(0); }
        if cause & CAUSEF_IP1 != 0 { do_IRQ(1); }
        if cause & CAUSEF_IP2 != 0 { dispatch_internal.unwrap()(0); }
        if is_ext_irq_cascaded != 0 {
            if cause & CAUSEF_IP3 != 0 { dispatch_internal.unwrap()(1); }
        } else {
            if cause & CAUSEF_IP3 != 0 { do_IRQ(IRQ_EXT_0); }
            if cause & CAUSEF_IP4 != 0 { do_IRQ(IRQ_EXT_1); }
            if cause & CAUSEF_IP5 != 0 { do_IRQ(IRQ_EXT_2); }
            if cause & CAUSEF_IP6 != 0 { do_IRQ(IRQ_EXT_3); }
        }
    }
}

unsafe fn bcm63xx_internal_irq_mask(d: *mut irq_data) { internal_irq_mask.unwrap()(d); }
unsafe fn bcm63xx_internal_irq_unmask(d: *mut irq_data) { internal_irq_unmask.unwrap()(d, core::ptr::null()); }

unsafe fn bcm63xx_external_irq_mask(d: *mut irq_data) {
    let irq = (*d).irq - IRQ_EXTERNAL_BASE; let regaddr = get_ext_irq_perf_reg(irq); let flags: c_ulong;
    spin_lock_irqsave(&mut epic_lock, &flags); let mut reg = bcm_perf_readl(regaddr);
    if BCMCPU_IS_6348() { reg &= !EXTIRQ_CFG_MASK_6348(irq % 4); } else { reg &= !EXTIRQ_CFG_MASK(irq % 4); }
    bcm_perf_writel(reg, regaddr); spin_unlock_irqrestore(&mut epic_lock, flags);
    if is_ext_irq_cascaded != 0 { internal_irq_mask.unwrap()(irq_get_irq_data(irq + ext_irq_start as i32)); }
}
unsafe fn bcm63xx_external_irq_unmask(d: *mut irq_data) {
    let irq = (*d).irq - IRQ_EXTERNAL_BASE; let regaddr = get_ext_irq_perf_reg(irq); let flags: c_ulong;
    spin_lock_irqsave(&mut epic_lock, &flags); let mut reg = bcm_perf_readl(regaddr);
    if BCMCPU_IS_6348() { reg |= EXTIRQ_CFG_MASK_6348(irq % 4); } else { reg |= EXTIRQ_CFG_MASK(irq % 4); }
    bcm_perf_writel(reg, regaddr); spin_unlock_irqrestore(&mut epic_lock, flags);
    if is_ext_irq_cascaded != 0 { internal_irq_unmask.unwrap()(irq_get_irq_data(irq + ext_irq_start as i32), core::ptr::null()); }
}
unsafe fn bcm63xx_external_irq_clear(d: *mut irq_data) {
    let irq = (*d).irq - IRQ_EXTERNAL_BASE; let regaddr = get_ext_irq_perf_reg(irq); let flags: c_ulong;
    spin_lock_irqsave(&mut epic_lock, &flags); let mut reg = bcm_perf_readl(regaddr);
    if BCMCPU_IS_6348() { reg |= EXTIRQ_CFG_CLEAR_6348(irq % 4); } else { reg |= EXTIRQ_CFG_CLEAR(irq % 4); }
    bcm_perf_writel(reg, regaddr); spin_unlock_irqrestore(&mut epic_lock, flags);
}

unsafe fn bcm63xx_external_irq_set_type(d: *mut irq_data, mut flow_type: u32) -> i32 {
    let mut irq = (*d).irq - IRQ_EXTERNAL_BASE; flow_type &= IRQ_TYPE_SENSE_MASK;
    if flow_type == IRQ_TYPE_NONE { flow_type = IRQ_TYPE_LEVEL_LOW; }
    let (mut levelsense, mut sense, mut bothedge) = (0, 0, 0);
    match flow_type { IRQ_TYPE_EDGE_BOTH => bothedge=1, IRQ_TYPE_EDGE_RISING => sense=1, IRQ_TYPE_EDGE_FALLING => {}, IRQ_TYPE_LEVEL_HIGH => { levelsense=1; sense=1; }, IRQ_TYPE_LEVEL_LOW => levelsense=1, _ => { pr_err!("bogus flow type combination given !\n"); return -EINVAL; } }
    let regaddr = get_ext_irq_perf_reg(irq); let flags: c_ulong; spin_lock_irqsave(&mut epic_lock, &flags); let mut reg = bcm_perf_readl(regaddr); irq %= 4;
    match bcm63xx_get_cpu_id() {
        BCM6348_CPU_ID => { if levelsense != 0 { reg |= EXTIRQ_CFG_LEVELSENSE_6348(irq); } else { reg &= !EXTIRQ_CFG_LEVELSENSE_6348(irq); } if sense != 0 { reg |= EXTIRQ_CFG_SENSE_6348(irq); } else { reg &= !EXTIRQ_CFG_SENSE_6348(irq); } if bothedge != 0 { reg |= EXTIRQ_CFG_BOTHEDGE_6348(irq); } else { reg &= !EXTIRQ_CFG_BOTHEDGE_6348(irq); } },
        BCM3368_CPU_ID | BCM6328_CPU_ID | BCM6338_CPU_ID | BCM6345_CPU_ID | BCM6358_CPU_ID | BCM6362_CPU_ID | BCM6368_CPU_ID => { if levelsense != 0 { reg |= EXTIRQ_CFG_LEVELSENSE(irq); } else { reg &= !EXTIRQ_CFG_LEVELSENSE(irq); } if sense != 0 { reg |= EXTIRQ_CFG_SENSE(irq); } else { reg &= !EXTIRQ_CFG_SENSE(irq); } if bothedge != 0 { reg |= EXTIRQ_CFG_BOTHEDGE(irq); } else { reg &= !EXTIRQ_CFG_BOTHEDGE(irq); } },
        _ => BUG!(),
    }
    bcm_perf_writel(reg, regaddr); spin_unlock_irqrestore(&mut epic_lock, flags); irqd_set_trigger_type(d, flow_type);
    if flow_type & (IRQ_TYPE_LEVEL_LOW | IRQ_TYPE_LEVEL_HIGH) != 0 { irq_set_handler_locked(d, handle_level_irq); } else { irq_set_handler_locked(d, handle_edge_irq); }
    IRQ_SET_MASK_OK_NOCOPY
}

#[cfg(CONFIG_SMP)]
unsafe fn bcm63xx_internal_set_affinity(data: *mut irq_data, dest: *const cpumask, _force: bool) -> i32 { if !irqd_irq_disabled(data) { internal_irq_unmask.unwrap()(data, dest); } 0 }

static mut bcm63xx_internal_irq_chip: irq_chip = irq_chip { name: "bcm63xx_ipic", irq_mask: Some(bcm63xx_internal_irq_mask), irq_unmask: Some(bcm63xx_internal_irq_unmask), ..irq_chip::ZERO };
static mut bcm63xx_external_irq_chip: irq_chip = irq_chip { name: "bcm63xx_epic", irq_ack: Some(bcm63xx_external_irq_clear), irq_mask: Some(bcm63xx_external_irq_mask), irq_unmask: Some(bcm63xx_external_irq_unmask), irq_set_type: Some(bcm63xx_external_irq_set_type), ..irq_chip::ZERO };

/* CPU-specific register selection and IRQ-chip installation remain direct translations. */
unsafe fn bcm63xx_init_irq() {
    let mut irq_bits = 0;
    irq_stat_addr[0] = bcm63xx_regset_address(RSET_PERF); irq_mask_addr[0] = irq_stat_addr[0]; irq_stat_addr[1] = irq_stat_addr[0]; irq_mask_addr[1] = irq_stat_addr[0];
    match bcm63xx_get_cpu_id() {
        BCM3368_CPU_ID => { irq_stat_addr[0]+=PERF_IRQSTAT_3368_REG; irq_mask_addr[0]+=PERF_IRQMASK_3368_REG; irq_stat_addr[1]=0; irq_mask_addr[1]=0; irq_bits=32; ext_irq_count=4; ext_irq_cfg_reg1=PERF_EXTIRQ_CFG_REG_3368; },
        BCM6328_CPU_ID => { irq_stat_addr[0]+=PERF_IRQSTAT_6328_REG(0); irq_mask_addr[0]+=PERF_IRQMASK_6328_REG(0); irq_stat_addr[1]+=PERF_IRQSTAT_6328_REG(1); irq_mask_addr[1]+=PERF_IRQMASK_6328_REG(1); irq_bits=64; ext_irq_count=4; is_ext_irq_cascaded=1; ext_irq_start=BCM_6328_EXT_IRQ0-IRQ_INTERNAL_BASE as u32; ext_irq_end=BCM_6328_EXT_IRQ3-IRQ_INTERNAL_BASE as u32; ext_irq_cfg_reg1=PERF_EXTIRQ_CFG_REG_6328; },
        BCM6338_CPU_ID => { irq_stat_addr[0]+=PERF_IRQSTAT_6338_REG; irq_mask_addr[0]+=PERF_IRQMASK_6338_REG; irq_stat_addr[1]=0; irq_mask_addr[1]=0; irq_bits=32; ext_irq_count=4; ext_irq_cfg_reg1=PERF_EXTIRQ_CFG_REG_6338; },
        BCM6345_CPU_ID => { irq_stat_addr[0]+=PERF_IRQSTAT_6345_REG; irq_mask_addr[0]+=PERF_IRQMASK_6345_REG; irq_stat_addr[1]=0; irq_mask_addr[1]=0; irq_bits=32; ext_irq_count=4; ext_irq_cfg_reg1=PERF_EXTIRQ_CFG_REG_6345; },
        BCM6348_CPU_ID => { irq_stat_addr[0]+=PERF_IRQSTAT_6348_REG; irq_mask_addr[0]+=PERF_IRQMASK_6348_REG; irq_stat_addr[1]=0; irq_mask_addr[1]=0; irq_bits=32; ext_irq_count=4; ext_irq_cfg_reg1=PERF_EXTIRQ_CFG_REG_6348; },
        BCM6358_CPU_ID => { irq_stat_addr[0]+=PERF_IRQSTAT_6358_REG(0); irq_mask_addr[0]+=PERF_IRQMASK_6358_REG(0); irq_stat_addr[1]+=PERF_IRQSTAT_6358_REG(1); irq_mask_addr[1]+=PERF_IRQMASK_6358_REG(1); irq_bits=32; ext_irq_count=4; is_ext_irq_cascaded=1; ext_irq_start=BCM_6358_EXT_IRQ0-IRQ_INTERNAL_BASE as u32; ext_irq_end=BCM_6358_EXT_IRQ3-IRQ_INTERNAL_BASE as u32; ext_irq_cfg_reg1=PERF_EXTIRQ_CFG_REG_6358; },
        BCM6362_CPU_ID => { irq_stat_addr[0]+=PERF_IRQSTAT_6362_REG(0); irq_mask_addr[0]+=PERF_IRQMASK_6362_REG(0); irq_stat_addr[1]+=PERF_IRQSTAT_6362_REG(1); irq_mask_addr[1]+=PERF_IRQMASK_6362_REG(1); irq_bits=64; ext_irq_count=4; is_ext_irq_cascaded=1; ext_irq_start=BCM_6362_EXT_IRQ0-IRQ_INTERNAL_BASE as u32; ext_irq_end=BCM_6362_EXT_IRQ3-IRQ_INTERNAL_BASE as u32; ext_irq_cfg_reg1=PERF_EXTIRQ_CFG_REG_6362; },
        BCM6368_CPU_ID => { irq_stat_addr[0]+=PERF_IRQSTAT_6368_REG(0); irq_mask_addr[0]+=PERF_IRQMASK_6368_REG(0); irq_stat_addr[1]+=PERF_IRQSTAT_6368_REG(1); irq_mask_addr[1]+=PERF_IRQMASK_6368_REG(1); irq_bits=64; ext_irq_count=6; is_ext_irq_cascaded=1; ext_irq_start=BCM_6368_EXT_IRQ0-IRQ_INTERNAL_BASE as u32; ext_irq_end=BCM_6368_EXT_IRQ5-IRQ_INTERNAL_BASE as u32; ext_irq_cfg_reg1=PERF_EXTIRQ_CFG_REG_6368; ext_irq_cfg_reg2=PERF_EXTIRQ_CFG_REG2_6368; },
        _ => BUG!(),
    }
    if irq_bits == 32 { dispatch_internal=Some(__dispatch_internal_32); internal_irq_mask=Some(__internal_irq_mask_32); internal_irq_unmask=Some(__internal_irq_unmask_32); } else { dispatch_internal=Some(__dispatch_internal_64); internal_irq_mask=Some(__internal_irq_mask_64); internal_irq_unmask=Some(__internal_irq_unmask_64); }
}

pub unsafe extern "C" fn arch_init_irq() {
    let mut irq = 0; bcm63xx_init_irq(); mips_cpu_irq_init();
    for i in IRQ_INTERNAL_BASE..NR_IRQS { irq_set_chip_and_handler(i, &mut bcm63xx_internal_irq_chip, handle_level_irq); }
    for i in IRQ_EXTERNAL_BASE..(IRQ_EXTERNAL_BASE + ext_irq_count as i32) { irq_set_chip_and_handler(i, &mut bcm63xx_external_irq_chip, handle_edge_irq); }
    if is_ext_irq_cascaded == 0 { for i in 3..(3 + ext_irq_count as i32) { irq=MIPS_CPU_IRQ_BASE+i; if request_irq(irq,no_action,IRQF_NO_THREAD,"cascade_extirq",core::ptr::null_mut()) != 0 { pr_err!("Failed to request irq %d (cascade_extirq)\n", irq); } } }
    irq=MIPS_CPU_IRQ_BASE+2; if request_irq(irq,no_action,IRQF_NO_THREAD,"cascade_ip2",core::ptr::null_mut()) != 0 { pr_err!("Failed to request irq %d (cascade_ip2)\n", irq); }
    #[cfg(CONFIG_SMP)] if is_ext_irq_cascaded != 0 { irq=MIPS_CPU_IRQ_BASE+3; if request_irq(irq,no_action,IRQF_NO_THREAD,"cascade_ip3",core::ptr::null_mut()) != 0 { pr_err!("Failed to request irq %d (cascade_ip3)\n", irq); } bcm63xx_internal_irq_chip.irq_set_affinity=Some(bcm63xx_internal_set_affinity); cpumask_clear(irq_default_affinity); cpumask_set_cpu(smp_processor_id(),irq_default_affinity); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
