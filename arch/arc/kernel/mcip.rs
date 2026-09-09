// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARC ARConnect (MultiCore IP) support (formerly known as MCIP)
 *
 * Copyright (C) 2013 Synopsys, Inc. (www.synopsys.com)
 */

// Dependencies supplied by the surrounding kernel translation.

static mut MCIP_LOCK: raw_spinlock_t = DEFINE_RAW_SPINLOCK!();

#[cfg(CONFIG_SMP)]
static mut SMP_CPUINFO_BUF: [u8; 128] = [0; 128];

#[cfg(CONFIG_SMP)]
unsafe fn mcip_update_gfrc_halt_mask(cpu: i32) {
    let mut gfrc: bcr_generic = core::mem::zeroed();
    let mut flags: c_ulong = 0;
    let mut gfrc_halt_mask: u32;
    READ_BCR!(ARC_REG_GFRC_BUILD, gfrc);
    if gfrc.ver < 0x3 { return; }
    raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags);
    __mcip_cmd!(CMD_GFRC_READ_CORE, 0);
    gfrc_halt_mask = read_aux_reg(ARC_REG_MCIP_READBACK);
    gfrc_halt_mask |= BIT(cpu);
    __mcip_cmd_data!(CMD_GFRC_SET_CORE, 0, gfrc_halt_mask);
    raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags);
}

#[cfg(CONFIG_SMP)]
unsafe fn mcip_update_debug_halt_mask(cpu: i32) {
    let mut mcip_mask: u32 = 0;
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags);
    __mcip_cmd!(CMD_DEBUG_READ_SELECT, 0);
    mcip_mask = read_aux_reg(ARC_REG_MCIP_READBACK);
    mcip_mask |= BIT(cpu);
    __mcip_cmd_data!(CMD_DEBUG_SET_SELECT, 0, mcip_mask);
    __mcip_cmd_data!(CMD_DEBUG_SET_MASK, 0xF, mcip_mask);
    raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags);
}

#[cfg(CONFIG_SMP)]
unsafe fn mcip_setup_per_cpu(cpu: i32) {
    let mut mp: mcip_bcr = core::mem::zeroed();
    READ_BCR!(ARC_REG_MCIP_BCR, mp);
    smp_ipi_irq_setup(cpu, IPI_IRQ);
    smp_ipi_irq_setup(cpu, SOFTIRQ_IRQ);
    if mp.gfrc { mcip_update_gfrc_halt_mask(cpu); }
    if mp.dbg { mcip_update_debug_halt_mask(cpu); }
}

#[cfg(CONFIG_SMP)]
unsafe fn mcip_ipi_send(cpu: i32) {
    let mut flags: c_ulong = 0;
    let ipi_was_pending: i32;
    if unlikely!(cpu == raw_smp_processor_id()) {
        arc_softirq_trigger(SOFTIRQ_IRQ);
        return;
    }
    raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags);
    __mcip_cmd!(CMD_INTRPT_READ_STATUS, cpu);
    ipi_was_pending = read_aux_reg(ARC_REG_MCIP_READBACK);
    if ipi_was_pending == 0 { __mcip_cmd!(CMD_INTRPT_GENERATE_IRQ, cpu); }
    raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags);
}

#[cfg(CONFIG_SMP)]
unsafe fn mcip_ipi_clear(irq: i32) {
    let mut cpu: u32;
    let mut c: u32;
    let mut flags: c_ulong = 0;
    if unlikely!(irq == SOFTIRQ_IRQ) { arc_softirq_clear(irq); return; }
    raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags);
    __mcip_cmd!(CMD_INTRPT_CHECK_SOURCE, 0);
    cpu = read_aux_reg(ARC_REG_MCIP_READBACK);
    loop {
        c = __ffs(cpu);
        __mcip_cmd!(CMD_INTRPT_GENERATE_ACK, c);
        cpu &= !(1u32 << c);
        if cpu == 0 { break; }
    }
    raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags);
}

#[cfg(CONFIG_SMP)]
unsafe fn mcip_probe_n_setup() {
    let mut mp: mcip_bcr = core::mem::zeroed();
    READ_BCR!(ARC_REG_MCIP_BCR, mp);
    sprintf!(SMP_CPUINFO_BUF.as_mut_ptr(), "Extn [SMP]\t: ARConnect (v%d): %d cores with %s%s%s%s\n", mp.ver, mp.num_cores, IS_AVAIL1!(mp.ipi, "IPI "), IS_AVAIL1!(mp.idu, "IDU "), IS_AVAIL1!(mp.dbg, "DEBUG "), IS_AVAIL1!(mp.gfrc, "GFRC"));
}

#[cfg(CONFIG_SMP)]
static mut PLAT_SMP_OPS: plat_smp_ops = plat_smp_ops {
    info: SMP_CPUINFO_BUF.as_ptr(), init_early_smp: Some(mcip_probe_n_setup),
    init_per_cpu: Some(mcip_setup_per_cpu), ipi_send: Some(mcip_ipi_send), ipi_clear: Some(mcip_ipi_clear),
};

unsafe fn idu_set_dest(cmn_irq: u32, cpu_mask: u32) { __mcip_cmd_data!(CMD_IDU_SET_DEST, cmn_irq, cpu_mask); }

unsafe fn idu_set_mode(cmn_irq: u32, set_lvl: bool, lvl: u32, set_distr: bool, distr: u32) {
    let mut word = __mcip_cmd_read(CMD_IDU_READ_MODE, cmn_irq);
    if set_distr { word = (word & !0x3) | (distr & 0x3); }
    if set_lvl { word = (word & !(1 << 4)) | ((lvl & 1) << 4); }
    __mcip_cmd_data!(CMD_IDU_SET_MODE, cmn_irq, word);
}

unsafe fn idu_irq_mask_raw(hwirq: irq_hw_number_t) { let mut flags=0; raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags); __mcip_cmd_data!(CMD_IDU_SET_MASK, hwirq, 1); raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags); }
unsafe fn idu_irq_mask(data: *mut irq_data) { idu_irq_mask_raw((*data).hwirq); }
unsafe fn idu_irq_unmask(data: *mut irq_data) { let mut flags=0; raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags); __mcip_cmd_data!(CMD_IDU_SET_MASK, (*data).hwirq, 0); raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags); }
unsafe fn idu_irq_ack(data: *mut irq_data) { let mut flags=0; raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags); __mcip_cmd!(CMD_IDU_ACK_CIRQ, (*data).hwirq); raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags); }
unsafe fn idu_irq_mask_ack(data: *mut irq_data) { let mut flags=0; raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags); __mcip_cmd_data!(CMD_IDU_SET_MASK, (*data).hwirq, 1); __mcip_cmd!(CMD_IDU_ACK_CIRQ, (*data).hwirq); raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags); }

unsafe fn idu_irq_set_affinity(data: *mut irq_data, cpumask: *const cpumask, _force: bool) -> i32 {
    let mut online: cpumask_t = core::mem::zeroed();
    if cpumask_and(&mut online, cpumask, cpu_online_mask) == 0 { return -EINVAL; }
    let mut flags=0; raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags);
    let destination_bits = cpumask_bits(&online)[0]; idu_set_dest((*data).hwirq, destination_bits);
    let distribution_mode = if ffs(destination_bits) == fls(destination_bits) { IDU_M_DISTRI_DEST } else { IDU_M_DISTRI_RR };
    idu_set_mode((*data).hwirq, false, 0, true, distribution_mode);
    raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags); IRQ_SET_MASK_OK
}

unsafe fn idu_irq_set_type(data: *mut irq_data, irq_type: u32) -> i32 {
    if irq_type & !(IRQ_TYPE_EDGE_RISING | IRQ_TYPE_LEVEL_HIGH) != 0 { return -EINVAL; }
    let mut flags=0; raw_spin_lock_irqsave!(&raw mut MCIP_LOCK, flags);
    idu_set_mode((*data).hwirq, true, if irq_type & IRQ_TYPE_EDGE_RISING != 0 { IDU_M_TRIG_EDGE } else { IDU_M_TRIG_LEVEL }, false, 0);
    raw_spin_unlock_irqrestore!(&raw mut MCIP_LOCK, flags); 0
}

unsafe fn idu_irq_enable(data: *mut irq_data) { idu_irq_set_affinity(data, cpu_online_mask, false); idu_irq_unmask(data); }

unsafe fn idu_cascade_isr(desc: *mut irq_desc) {
    let idu_domain = irq_desc_get_handler_data(desc); let core_chip = irq_desc_get_chip(desc);
    let core_hwirq = irqd_to_hwirq(irq_desc_get_irq_data(desc)); let idu_hwirq = core_hwirq - FIRST_EXT_IRQ;
    chained_irq_enter(core_chip, desc); generic_handle_domain_irq(idu_domain, idu_hwirq); chained_irq_exit(core_chip, desc);
}

// The remaining IRQ-domain glue retains the kernel interfaces and control flow.
unsafe fn idu_irq_map(_d: *mut irq_domain, virq: u32, _hwirq: irq_hw_number_t) -> i32 { irq_set_chip_and_handler(virq, &raw mut IDU_IRQ_CHIP, handle_level_irq); 0 }
static mut IDU_IRQ_CHIP: irq_chip = irq_chip { name: c_str!("MCIP IDU Intc"), irq_mask: Some(idu_irq_mask), irq_unmask: Some(idu_irq_unmask), irq_ack: Some(idu_irq_ack), irq_mask_ack: Some(idu_irq_mask_ack), irq_enable: Some(idu_irq_enable), irq_set_type: Some(idu_irq_set_type), irq_set_affinity: Some(idu_irq_set_affinity), ..unsafe { core::mem::zeroed() } };

static IDU_IRQ_OPS: irq_domain_ops = irq_domain_ops { xlate: Some(irq_domain_xlate_onetwocell), map: Some(idu_irq_map), ..unsafe { core::mem::zeroed() } };

#[init]
unsafe fn idu_of_init(intc: *mut device_node, _parent: *mut device_node) -> i32 {
    let mut mp: mcip_bcr = core::mem::zeroed(); let mut idu_bcr: mcip_idu_bcr = core::mem::zeroed();
    READ_BCR!(ARC_REG_MCIP_BCR, mp); if !mp.idu { panic!("IDU not detected, but DeviceTree using it"); }
    READ_BCR!(ARC_REG_MCIP_IDU_BCR, idu_bcr); let nr_irqs = mcip_idu_bcr_to_nr_irqs(idu_bcr);
    pr_info!("MCIP: IDU supports %u common irqs\n", nr_irqs);
    let domain = irq_domain_create_linear(of_fwnode_handle(intc), nr_irqs, &IDU_IRQ_OPS, core::ptr::null_mut());
    for i in 0..nr_irqs { idu_irq_mask_raw(i); let virq=irq_create_mapping(core::ptr::null_mut(), i + FIRST_EXT_IRQ); BUG_ON!(virq == 0); irq_set_chained_handler_and_data(virq, idu_cascade_isr, domain); }
    __mcip_cmd!(CMD_IDU_ENABLE, 0); 0
}

// IRQCHIP_DECLARE(arcv2_idu_intc, "snps,archs-idu-intc", idu_of_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
