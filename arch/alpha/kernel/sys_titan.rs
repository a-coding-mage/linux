// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_titan.c
 *
 * Code supporting TITAN systems (EV6+TITAN), currently:
 * Privateer, Falcon, Granite
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

/* Titan supports up to 4 CPUs. */
static mut titan_cpu_irq_affinity: [c_ulong; 4] = [!0; 4];

/* Mask is set (1) if enabled. */
static mut titan_cached_irq_mask: c_ulong = 0;

/* Need SMP-safe access to interrupt CSRs. */
// DEFINE_SPINLOCK(titan_irq_lock);
static mut titan_irq_lock: spinlock_t = spinlock_t { _private: [] };

unsafe fn titan_update_irq_hw(mask: c_ulong) {
    let cchip: *mut titan_cchip = TITAN_cchip;
    let isa_enable: c_ulong = 1u64 << 55;
    let bcpu: c_int = boot_cpuid;

    // CONFIG_SMP selects the four-CPU path in the kernel build.
    #[cfg(CONFIG_SMP)]
    {
        let mut cpm: cpumask_t = core::mem::zeroed();
        let mut dim0: *mut c_ulong;
        let mut dim1: *mut c_ulong;
        let mut dim2: *mut c_ulong;
        let mut dim3: *mut c_ulong;
        let mut mask0: c_ulong = mask & !isa_enable;
        let mut mask1: c_ulong = mask & !isa_enable;
        let mut mask2: c_ulong = mask & !isa_enable;
        let mut mask3: c_ulong = mask & !isa_enable;
        let mut dummy: c_ulong = 0;

        cpumask_copy(&mut cpm, cpu_present_mask);
        mask0 &= titan_cpu_irq_affinity[0];
        mask1 &= titan_cpu_irq_affinity[1];
        mask2 &= titan_cpu_irq_affinity[2];
        mask3 &= titan_cpu_irq_affinity[3];

        if bcpu == 0 { mask0 |= isa_enable; }
        else if bcpu == 1 { mask1 |= isa_enable; }
        else if bcpu == 2 { mask2 |= isa_enable; }
        else { mask3 |= isa_enable; }

        dim0 = &mut (*cchip).dim0.csr;
        dim1 = &mut (*cchip).dim1.csr;
        dim2 = &mut (*cchip).dim2.csr;
        dim3 = &mut (*cchip).dim3.csr;
        if !cpumask_test_cpu(0, &cpm) { dim0 = &mut dummy; }
        if !cpumask_test_cpu(1, &cpm) { dim1 = &mut dummy; }
        if !cpumask_test_cpu(2, &cpm) { dim2 = &mut dummy; }
        if !cpumask_test_cpu(3, &cpm) { dim3 = &mut dummy; }

        core::ptr::write_volatile(dim0, mask0);
        core::ptr::write_volatile(dim1, mask1);
        core::ptr::write_volatile(dim2, mask2);
        core::ptr::write_volatile(dim3, mask3);
        mb();
        let _ = core::ptr::read_volatile(dim0);
        let _ = core::ptr::read_volatile(dim1);
        let _ = core::ptr::read_volatile(dim2);
        let _ = core::ptr::read_volatile(dim3);
    }
    #[cfg(not(CONFIG_SMP))]
    {
        let mut dim_b: *mut c_ulong = &mut (*cchip).dim0.csr;
        if bcpu == 1 { dim_b = &mut (*cchip).dim1.csr; }
        else if bcpu == 2 { dim_b = &mut (*cchip).dim2.csr; }
        else if bcpu == 3 { dim_b = &mut (*cchip).dim3.csr; }
        core::ptr::write_volatile(dim_b, mask | isa_enable);
        mb();
        let _ = core::ptr::read_volatile(dim_b);
    }
}

unsafe fn titan_enable_irq(d: *mut irq_data) {
    let irq = (*d).irq;
    spin_lock(&mut titan_irq_lock);
    titan_cached_irq_mask |= 1u64 << (irq - 16);
    titan_update_irq_hw(titan_cached_irq_mask);
    spin_unlock(&mut titan_irq_lock);
}

unsafe fn titan_disable_irq(d: *mut irq_data) {
    let irq = (*d).irq;
    spin_lock(&mut titan_irq_lock);
    titan_cached_irq_mask &= !(1u64 << (irq - 16));
    titan_update_irq_hw(titan_cached_irq_mask);
    spin_unlock(&mut titan_irq_lock);
}

unsafe fn titan_cpu_set_irq_affinity(irq: c_uint, affinity: cpumask_t) {
    for cpu in 0..4 {
        if cpumask_test_cpu(cpu, &affinity) {
            titan_cpu_irq_affinity[cpu as usize] |= 1u64 << irq;
        } else {
            titan_cpu_irq_affinity[cpu as usize] &= !(1u64 << irq);
        }
    }
}

unsafe fn titan_set_irq_affinity(d: *mut irq_data, affinity: *const cpumask_t, _force: bool) -> c_int {
    let irq = (*d).irq;
    spin_lock(&mut titan_irq_lock);
    titan_cpu_set_irq_affinity(irq - 16, *affinity);
    titan_update_irq_hw(titan_cached_irq_mask);
    spin_unlock(&mut titan_irq_lock);
    0
}

unsafe fn titan_device_interrupt(_vector: c_ulong) {
    printk(cstr!("titan_device_interrupt: NOT IMPLEMENTED YET!!\n"));
}

unsafe fn titan_srm_device_interrupt(vector: c_ulong) {
    let irq = (vector - 0x800) >> 4;
    handle_irq(irq as c_uint);
}

unsafe fn init_titan_irqs(ops: *mut irq_chip, imin: c_int, imax: c_int) {
    let mut i = imin;
    while i <= imax {
        irq_set_chip_and_handler(i as c_uint, ops, handle_level_irq);
        irq_set_status_flags(i as c_uint, IRQ_LEVEL);
        i += 1;
    }
}

static mut titan_irq_type: irq_chip = irq_chip {
    name: cstr!("TITAN"),
    irq_unmask: Some(titan_enable_irq), irq_mask: Some(titan_disable_irq),
    irq_mask_ack: Some(titan_disable_irq), irq_set_affinity: Some(titan_set_irq_affinity),
};

unsafe fn titan_intr_nop(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t { IRQ_HANDLED }

unsafe fn titan_init_irq() {
    if alpha_using_srm && alpha_mv.device_interrupt.is_none() { alpha_mv.device_interrupt = Some(titan_srm_device_interrupt); }
    if alpha_mv.device_interrupt.is_none() { alpha_mv.device_interrupt = Some(titan_device_interrupt); }
    titan_update_irq_hw(0);
    init_titan_irqs(&mut titan_irq_type, 16, 63 + 16);
}

unsafe fn titan_legacy_init_irq() {
    outb(0, DMA1_RESET_REG); outb(0, DMA2_RESET_REG);
    outb(DMA_MODE_CASCADE, DMA2_MODE_REG); outb(0, DMA2_MASK_REG);
    init_i8259a_irqs(); titan_init_irq();
}

pub unsafe fn titan_dispatch_irqs(mut mask: u64) {
    mask &= titan_cpu_irq_affinity[smp_processor_id() as usize];
    while mask != 0 {
        let mut vector = 63 - __kernel_ctlz(mask);
        mask &= !(1u64 << vector);
        vector = 0x900 + (vector << 4);
        if let Some(f) = alpha_mv.device_interrupt { f(vector); }
    }
}

unsafe fn titan_request_irq(irq: c_uint, handler: irq_handler_t, irqflags: c_ulong, devname: *const c_char, dev_id: *mut c_void) {
    let err = request_irq(irq, handler, irqflags, devname, dev_id);
    if err != 0 { printk(cstr!("titan_request_irq returned error; ignoring\n")); }
}

unsafe fn titan_late_init() {
    titan_request_irq(63 + 16, Some(titan_intr_nop), 0, cstr!("CChip Error"), core::ptr::null_mut());
    titan_request_irq(62 + 16, Some(titan_intr_nop), 0, cstr!("PChip 0 H_Error"), core::ptr::null_mut());
    titan_request_irq(61 + 16, Some(titan_intr_nop), 0, cstr!("PChip 1 H_Error"), core::ptr::null_mut());
    titan_request_irq(60 + 16, Some(titan_intr_nop), 0, cstr!("PChip 0 C_Error"), core::ptr::null_mut());
    titan_request_irq(59 + 16, Some(titan_intr_nop), 0, cstr!("PChip 1 C_Error"), core::ptr::null_mut());
    titan_register_error_handlers(); cdl_check_console_data_log();
}

unsafe fn titan_map_irq(dev: *const pci_dev, _slot: u8, _pin: u8) -> c_int {
    let mut intline: u8 = 0;
    pci_read_config_byte(dev, PCI_INTERRUPT_LINE, &mut intline);
    let irq = intline as c_int;
    if (irq & 0xF0) == 0xE0 { irq } else { irq + 16 }
}

unsafe fn titan_init_pci() {
    titan_late_init(); pci_set_flags(PCI_PROBE_ONLY); common_init_pci();
    SMC669_Init(0); locate_and_init_vga(core::ptr::null_mut());
}

unsafe fn privateer_init_pci() {
    titan_request_irq(53 + 16, Some(titan_intr_nop), 0, cstr!("NMI"), core::ptr::null_mut());
    titan_request_irq(50 + 16, Some(titan_intr_nop), 0, cstr!("Temperature Warning"), core::ptr::null_mut());
    titan_init_pci();
}

// The C initializers use external architecture macros and symbols.  Their
// machine-vector layout and field values are retained here for the dependent
// kernel bindings to instantiate.
static mut titan_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: cstr!("TITAN"), machine_check: Some(titan_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS, min_io_address: DEFAULT_IO_BASE,
    min_mem_address: DEFAULT_MEM_BASE, pci_dac_offset: TITAN_DAC_OFFSET, nr_irqs: 80,
    agp_info: Some(titan_agp_info), init_arch: Some(titan_init_arch),
    init_irq: Some(titan_legacy_init_irq), init_rtc: Some(common_init_rtc),
    init_pci: Some(titan_init_pci), kill_arch: Some(titan_kill_arch),
    pci_map_irq: Some(titan_map_irq), pci_swizzle: Some(common_swizzle), ..unsafe { core::mem::zeroed() }
};

static mut privateer_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: cstr!("PRIVATEER"), machine_check: Some(privateer_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS, min_io_address: DEFAULT_IO_BASE,
    min_mem_address: DEFAULT_MEM_BASE, pci_dac_offset: TITAN_DAC_OFFSET, nr_irqs: 80,
    agp_info: Some(titan_agp_info), init_arch: Some(titan_init_arch),
    init_irq: Some(titan_legacy_init_irq), init_rtc: Some(common_init_rtc),
    init_pci: Some(privateer_init_pci), kill_arch: Some(titan_kill_arch),
    pci_map_irq: Some(titan_map_irq), pci_swizzle: Some(common_swizzle), ..unsafe { core::mem::zeroed() }
};

// ALIAS_MV(titan). Privateer intentionally has no alpha_mv alias.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
