// SPDX-License-Identifier: GPL-2.0
/*
 * ip27-irq.c: Highlevel interrupt handling for IP27 architecture.
 *
 * Copyright (C) 1999, 2000 Ralf Baechle (ralf@gnu.org)
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 1999 - 2001 Kanoj Sarcar
 */

// Linux and architecture-specific includes from the C source are external dependencies.

#[repr(C)]
struct HubIrqData {
    irq_mask: [*mut u64; 2],
    cpu: CpuidT,
}

static mut HUB_IRQ_MAP: [usize; IP27_HUB_IRQ_COUNT / (usize::BITS as usize) + 1] =
    [0; IP27_HUB_IRQ_COUNT / (usize::BITS as usize) + 1];

static mut IRQ_ENABLE_MASK: PerCpu<[c_ulong; 2]> = PerCpu::new();

unsafe fn alloc_level() -> c_int {
    let mut level: c_int;
    loop {
        level = find_first_zero_bit(HUB_IRQ_MAP.as_ptr() as *const c_ulong, IP27_HUB_IRQ_COUNT);
        if level >= IP27_HUB_IRQ_COUNT as c_int {
            return -ENOSPC;
        }
        if !test_and_set_bit(level as usize, HUB_IRQ_MAP.as_mut_ptr() as *mut c_ulong) {
            break;
        }
    }
    level
}

unsafe fn enable_hub_irq(d: *mut IrqData) {
    let hd = irq_data_get_irq_chip_data(d) as *mut HubIrqData;
    let mask = per_cpu(IRQ_ENABLE_MASK, (*hd).cpu);
    set_bit((*d).hwirq as usize, mask);
    __raw_writeq((*mask)[0], (*hd).irq_mask[0]);
    __raw_writeq((*mask)[1], (*hd).irq_mask[1]);
}

unsafe fn disable_hub_irq(d: *mut IrqData) {
    let hd = irq_data_get_irq_chip_data(d) as *mut HubIrqData;
    let mask = per_cpu(IRQ_ENABLE_MASK, (*hd).cpu);
    clear_bit((*d).hwirq as usize, mask);
    __raw_writeq((*mask)[0], (*hd).irq_mask[0]);
    __raw_writeq((*mask)[1], (*hd).irq_mask[1]);
}

unsafe fn setup_hub_mask(hd: *mut HubIrqData, mask: *const CpuMask) {
    let mut cpu: c_int = cpumask_first_and(mask, cpu_online_mask);
    if cpu >= nr_cpu_ids {
        cpu = cpumask_any(cpu_online_mask);
    }
    let nasid = cpu_to_node(cpu);
    (*hd).cpu = cpu;
    if cputoslice(cpu) == 0 {
        (*hd).irq_mask[0] = REMOTE_HUB_PTR(nasid, PI_INT_MASK0_A);
        (*hd).irq_mask[1] = REMOTE_HUB_PTR(nasid, PI_INT_MASK1_A);
    } else {
        (*hd).irq_mask[0] = REMOTE_HUB_PTR(nasid, PI_INT_MASK0_B);
        (*hd).irq_mask[1] = REMOTE_HUB_PTR(nasid, PI_INT_MASK1_B);
    }
}

unsafe fn set_affinity_hub_irq(d: *mut IrqData, mask: *const CpuMask, _force: bool) -> c_int {
    let hd = irq_data_get_irq_chip_data(d) as *mut HubIrqData;
    if hd.is_null() {
        return -EINVAL;
    }
    if irqd_is_started(d) {
        disable_hub_irq(d);
    }
    setup_hub_mask(hd, mask);
    if irqd_is_started(d) {
        enable_hub_irq(d);
    }
    irq_data_update_effective_affinity(d, cpumask_of((*hd).cpu));
    0
}

static mut HUB_IRQ_TYPE: IrqChip = IrqChip {
    name: b"HUB\0".as_ptr() as *const c_char,
    irq_mask: Some(disable_hub_irq),
    irq_unmask: Some(enable_hub_irq),
    irq_set_affinity: Some(set_affinity_hub_irq),
};

unsafe fn hub_domain_alloc(
    domain: *mut IrqDomain,
    virq: c_uint,
    nr_irqs: c_uint,
    arg: *mut c_void,
) -> c_int {
    let info = arg as *mut IrqAllocInfo;
    if nr_irqs > 1 || info.is_null() {
        return -EINVAL;
    }
    let hd = kzalloc_obj::<HubIrqData>();
    if hd.is_null() {
        return -ENOMEM;
    }
    let swlevel = alloc_level();
    if swlevel < 0 {
        kfree(hd as *mut c_void);
        return -EAGAIN;
    }
    irq_domain_set_info(domain, virq, swlevel as c_uint, &mut HUB_IRQ_TYPE, hd as *mut c_void,
                        Some(handle_level_irq), core::ptr::null_mut(), core::ptr::null_mut());

    // use CPU connected to nearest hub
    let hub = hub_data((*info).nasid);
    setup_hub_mask(hd, &(*hub).h_cpus);
    (*info).nasid = cpu_to_node((*hd).cpu);

    // Make sure it's not already pending when we connect it.
    REMOTE_HUB_CLR_INTR((*info).nasid, swlevel);

    let desc = irq_to_desc(virq);
    (*desc).irq_common_data.node = (*info).nasid;
    cpumask_copy((*desc).irq_common_data.affinity, &(*hub).h_cpus);
    0
}

unsafe fn hub_domain_free(domain: *mut IrqDomain, virq: c_uint, nr_irqs: c_uint) {
    if nr_irqs > 1 {
        return;
    }
    let irqd = irq_domain_get_irq_data(domain, virq);
    if !irqd.is_null() {
        kfree((*irqd).chip_data);
    }
}

static HUB_DOMAIN_OPS: IrqDomainOps = IrqDomainOps {
    alloc: Some(hub_domain_alloc),
    free: Some(hub_domain_free),
};

/*
 * This code is unnecessarily complex, because we do
 * intr enabling. Basically, once we grab the set of intrs we need
 * to service, we must mask _all_ these interrupts; firstly, to make
 * sure the same intr does not intr again, causing recursion that
 * can lead to stack overflow. Secondly, we can not just mask the
 * one intr we are do_IRQing, because the non-masked intrs in the
 * first set might intr again, causing multiple servicings of the
 * same intr. This effect is mostly seen for intercpu intrs.
 * Kanoj 05.13.00
 */

unsafe fn ip27_do_irq_mask0(desc: *mut IrqDesc) {
    let cpu = smp_processor_id();
    let mask = per_cpu(IRQ_ENABLE_MASK, cpu);
    let mut pend0: u64;
    // copied from Irix intpend0()
    pend0 = LOCAL_HUB_L(PI_INT_PEND0);
    pend0 &= (*mask)[0];
    if pend0 == 0 { return; }

    // CONFIG_SMP conditional from the C source.
    if pend0 & (1u64 << CPU_RESCHED_A_IRQ) != 0 {
        LOCAL_HUB_CLR_INTR(CPU_RESCHED_A_IRQ); scheduler_ipi();
    } else if pend0 & (1u64 << CPU_RESCHED_B_IRQ) != 0 {
        LOCAL_HUB_CLR_INTR(CPU_RESCHED_B_IRQ); scheduler_ipi();
    } else if pend0 & (1u64 << CPU_CALL_A_IRQ) != 0 {
        LOCAL_HUB_CLR_INTR(CPU_CALL_A_IRQ); generic_smp_call_function_interrupt();
    } else if pend0 & (1u64 << CPU_CALL_B_IRQ) != 0 {
        LOCAL_HUB_CLR_INTR(CPU_CALL_B_IRQ); generic_smp_call_function_interrupt();
    } else {
        let domain = irq_desc_get_handler_data(desc);
        let ret = generic_handle_domain_irq(domain, __ffs(pend0));
        if ret != 0 { spurious_interrupt(); }
    }
    LOCAL_HUB_L(PI_INT_PEND0);
}

unsafe fn ip27_do_irq_mask1(desc: *mut IrqDesc) {
    let cpu = smp_processor_id();
    let mask = per_cpu(IRQ_ENABLE_MASK, cpu);
    // copied from Irix intpend0()
    let mut pend1 = LOCAL_HUB_L(PI_INT_PEND1);
    pend1 &= (*mask)[1];
    if pend1 == 0 { return; }
    let domain = irq_desc_get_handler_data(desc);
    let ret = generic_handle_domain_irq(domain, __ffs(pend1) + 64);
    if ret != 0 { spurious_interrupt(); }
    LOCAL_HUB_L(PI_INT_PEND1);
}

pub unsafe fn install_ipi() {
    let cpu = smp_processor_id();
    let mask = per_cpu(IRQ_ENABLE_MASK, cpu);
    let slice = LOCAL_HUB_L(PI_CPU_NUM);
    let resched = CPU_RESCHED_A_IRQ + slice;
    set_bit(resched as usize, mask);
    LOCAL_HUB_CLR_INTR(resched);
    let call = CPU_CALL_A_IRQ + slice;
    set_bit(call as usize, mask);
    LOCAL_HUB_CLR_INTR(call);
    if slice == 0 {
        LOCAL_HUB_S(PI_INT_MASK0_A, (*mask)[0]);
        LOCAL_HUB_S(PI_INT_MASK1_A, (*mask)[1]);
    } else {
        LOCAL_HUB_S(PI_INT_MASK0_B, (*mask)[0]);
        LOCAL_HUB_S(PI_INT_MASK1_B, (*mask)[1]);
    }
}

pub unsafe fn arch_init_irq() {
    let mut domain: *mut IrqDomain;
    let fn_handle: *mut FwnodeHandle;
    mips_cpu_irq_init();
    // Some interrupts are reserved by hardware or by software convention.
    // Mark these as reserved right away so they won't be used accidentally later.
    bitmap_set(HUB_IRQ_MAP.as_mut_ptr() as *mut c_ulong, 0, CPU_CALL_B_IRQ + 1);
    bitmap_set(HUB_IRQ_MAP.as_mut_ptr() as *mut c_ulong, NI_BRDCAST_ERR_A,
               MSC_PANIC_INTR - NI_BRDCAST_ERR_A + 1);
    fn_handle = irq_domain_alloc_named_fwnode(b"HUB\0".as_ptr() as *const c_char);
    if WARN_ON(fn_handle.is_null()) { return; }
    domain = irq_domain_create_linear(fn_handle, IP27_HUB_IRQ_COUNT, &HUB_DOMAIN_OPS, core::ptr::null_mut());
    if WARN_ON(domain.is_null()) { return; }
    irq_set_default_domain(domain);
    irq_set_percpu_devid(IP27_HUB_PEND0_IRQ);
    irq_set_chained_handler_and_data(IP27_HUB_PEND0_IRQ, Some(ip27_do_irq_mask0), domain as *mut c_void);
    irq_set_percpu_devid(IP27_HUB_PEND1_IRQ);
    irq_set_chained_handler_and_data(IP27_HUB_PEND1_IRQ, Some(ip27_do_irq_mask1), domain as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
