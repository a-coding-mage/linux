// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/arch/alpha/kernel/sys_dp264.c. */

// Kernel and architecture headers supplied by the surrounding translation unit.

static mut CACHED_IRQ_MASK: c_ulong = 0;
static mut CPU_IRQ_AFFINITY: [c_ulong; 4] = [0, 0, 0, 0];

static mut DP264_IRQ_LOCK: RawSpinlock = RawSpinlock::new();

unsafe fn tsunami_update_irq_hw(mut mask: c_ulong) {
    let cchip = TSUNAMI_cchip;
    let isa_enable: c_ulong = 1u64 << 55;
    let bcpu = boot_cpuid;

    // CONFIG_SMP selects the four-dispatch-register implementation.
    #[cfg(CONFIG_SMP)]
    {
        mask &= !isa_enable;
        let mut mask0 = mask & CPU_IRQ_AFFINITY[0];
        let mut mask1 = mask & CPU_IRQ_AFFINITY[1];
        let mut mask2 = mask & CPU_IRQ_AFFINITY[2];
        let mut mask3 = mask & CPU_IRQ_AFFINITY[3];
        if bcpu == 0 { mask0 |= isa_enable; }
        else if bcpu == 1 { mask1 |= isa_enable; }
        else if bcpu == 2 { mask2 |= isa_enable; }
        else { mask3 |= isa_enable; }
        let mut dummy: c_ulong = 0;
        let mut dim0 = &mut (*cchip).dim0.csr as *mut c_ulong;
        let mut dim1 = &mut (*cchip).dim1.csr as *mut c_ulong;
        let mut dim2 = &mut (*cchip).dim2.csr as *mut c_ulong;
        let mut dim3 = &mut (*cchip).dim3.csr as *mut c_ulong;
        if !cpu_possible(0) { dim0 = &mut dummy; }
        if !cpu_possible(1) { dim1 = &mut dummy; }
        if !cpu_possible(2) { dim2 = &mut dummy; }
        if !cpu_possible(3) { dim3 = &mut dummy; }
        core::ptr::write_volatile(dim0, mask0); core::ptr::write_volatile(dim1, mask1);
        core::ptr::write_volatile(dim2, mask2); core::ptr::write_volatile(dim3, mask3);
        mb(); let _ = core::ptr::read_volatile(dim0); let _ = core::ptr::read_volatile(dim1);
        let _ = core::ptr::read_volatile(dim2); let _ = core::ptr::read_volatile(dim3);
    }
    #[cfg(not(CONFIG_SMP))]
    {
        let dim = if bcpu == 0 { &mut (*cchip).dim0.csr }
            else if bcpu == 1 { &mut (*cchip).dim1.csr }
            else if bcpu == 2 { &mut (*cchip).dim2.csr }
            else { &mut (*cchip).dim3.csr };
        core::ptr::write_volatile(dim, mask | isa_enable);
        mb(); let _ = core::ptr::read_volatile(dim);
    }
}

unsafe fn dp264_enable_irq(d: *mut irq_data) { let mut flags = 0; raw_spin_lock_irqsave(&mut DP264_IRQ_LOCK, &mut flags); CACHED_IRQ_MASK |= 1 << (*d).irq; tsunami_update_irq_hw(CACHED_IRQ_MASK); raw_spin_unlock_irqrestore(&mut DP264_IRQ_LOCK, flags); }
unsafe fn dp264_disable_irq(d: *mut irq_data) { let mut flags = 0; raw_spin_lock_irqsave(&mut DP264_IRQ_LOCK, &mut flags); CACHED_IRQ_MASK &= !(1 << (*d).irq); tsunami_update_irq_hw(CACHED_IRQ_MASK); raw_spin_unlock_irqrestore(&mut DP264_IRQ_LOCK, flags); }
unsafe fn clipper_enable_irq(d: *mut irq_data) { let mut flags = 0; raw_spin_lock_irqsave(&mut DP264_IRQ_LOCK, &mut flags); CACHED_IRQ_MASK |= 1 << ((*d).irq - 16); tsunami_update_irq_hw(CACHED_IRQ_MASK); raw_spin_unlock_irqrestore(&mut DP264_IRQ_LOCK, flags); }
unsafe fn clipper_disable_irq(d: *mut irq_data) { let mut flags = 0; raw_spin_lock_irqsave(&mut DP264_IRQ_LOCK, &mut flags); CACHED_IRQ_MASK &= !(1 << ((*d).irq - 16)); tsunami_update_irq_hw(CACHED_IRQ_MASK); raw_spin_unlock_irqrestore(&mut DP264_IRQ_LOCK, flags); }

unsafe fn cpu_set_irq_affinity(irq: c_uint, affinity: cpumask_t) {
    for cpu in 0..4 { let mut aff = CPU_IRQ_AFFINITY[cpu]; if cpumask_test_cpu(cpu, &affinity) { aff |= 1 << irq; } else { aff &= !(1 << irq); } CPU_IRQ_AFFINITY[cpu] = aff; }
}
unsafe fn dp264_set_affinity(d: *mut irq_data, affinity: *const cpumask, _force: bool) -> c_int { let mut flags=0; raw_spin_lock_irqsave(&mut DP264_IRQ_LOCK,&mut flags); cpu_set_irq_affinity((*d).irq, *affinity); tsunami_update_irq_hw(CACHED_IRQ_MASK); raw_spin_unlock_irqrestore(&mut DP264_IRQ_LOCK,flags); 0 }
unsafe fn clipper_set_affinity(d: *mut irq_data, affinity: *const cpumask, _force: bool) -> c_int { let mut flags=0; raw_spin_lock_irqsave(&mut DP264_IRQ_LOCK,&mut flags); cpu_set_irq_affinity((*d).irq-16, *affinity); tsunami_update_irq_hw(CACHED_IRQ_MASK); raw_spin_unlock_irqrestore(&mut DP264_IRQ_LOCK,flags); 0 }

static mut DP264_IRQ_TYPE: irq_chip = irq_chip { name: "DP264", irq_unmask: Some(dp264_enable_irq), irq_mask: Some(dp264_disable_irq), irq_mask_ack: Some(dp264_disable_irq), irq_set_affinity: Some(dp264_set_affinity) };
static mut CLIPPER_IRQ_TYPE: irq_chip = irq_chip { name: "CLIPPER", irq_unmask: Some(clipper_enable_irq), irq_mask: Some(clipper_disable_irq), irq_mask_ack: Some(clipper_disable_irq), irq_set_affinity: Some(clipper_set_affinity) };

unsafe fn dp264_device_interrupt(vector: c_ulong) { let mut pld = (*TSUNAMI_cchip).dir0.csr; while pld != 0 { let i = (!pld).trailing_zeros(); pld &= pld - 1; if i == 55 { isa_device_interrupt(vector); } else { handle_irq(16 + i); } } }
unsafe fn dp264_srm_device_interrupt(vector: c_ulong) { let mut irq = (vector - 0x800) >> 4; if irq >= 32 { irq -= 16; } handle_irq(irq); }
unsafe fn clipper_srm_device_interrupt(vector: c_ulong) { let irq = (vector - 0x800) >> 4; handle_irq(irq); }

unsafe fn init_tsunami_irqs(ops: *mut irq_chip, imin: c_int, imax: c_int) { let mut i=imin; while i<=imax { irq_set_chip_and_handler(i,ops,handle_level_irq); irq_set_status_flags(i,IRQ_LEVEL); i+=1; } }
unsafe fn dp264_init_irq() { outb(0,DMA1_RESET_REG); outb(0,DMA2_RESET_REG); outb(DMA_MODE_CASCADE,DMA2_MODE_REG); outb(0,DMA2_MASK_REG); if alpha_using_srm { alpha_mv.device_interrupt=Some(dp264_srm_device_interrupt); } tsunami_update_irq_hw(0); init_i8259a_irqs(); init_tsunami_irqs(&mut DP264_IRQ_TYPE,16,47); }
unsafe fn clipper_init_irq() { outb(0,DMA1_RESET_REG); outb(0,DMA2_RESET_REG); outb(DMA_MODE_CASCADE,DMA2_MODE_REG); outb(0,DMA2_MASK_REG); if alpha_using_srm { alpha_mv.device_interrupt=Some(clipper_srm_device_interrupt); } tsunami_update_irq_hw(0); init_i8259a_irqs(); init_tsunami_irqs(&mut CLIPPER_IRQ_TYPE,24,63); }

// PCI routing tables and COMMON_TABLE_LOOKUP are retained as direct table-driven translations.
unsafe fn isa_irq_fixup(dev: *const pci_dev, irq: c_int) -> c_int { if irq > 0 { return irq; } let mut irq8=0u8; pci_read_config_byte(dev,PCI_INTERRUPT_LINE,&mut irq8); (irq8 & 0xf) as c_int }
unsafe fn dp264_map_irq(dev:*const pci_dev, slot:u8, pin:u8)->c_int { let t=[[ -1,-1,-1,-1,-1],[19,19,18,18,18],[31,31,30,29,28],[27,27,26,25,24],[23,23,22,21,20],[19,19,18,17,16]]; let irq=common_table_lookup(&t,5,10,5,slot,pin); let hose=(*dev).sysdata; isa_irq_fixup(dev,if irq>0 {irq+16*(*hose).index} else {irq}) }
unsafe fn monet_map_irq(dev:*const pci_dev, slot:u8, pin:u8)->c_int { let t=[[45;5],[-1;5],[-1;5],[47;5],[-1;5],[-1;5],[28,28,29,30,31],[24,24,25,26,27],[40,40,41,42,43],[36,36,37,38,39],[32,32,33,34,35],[28,28,29,30,31],[24,24,25,26,27]]; isa_irq_fixup(dev,common_table_lookup(&t,3,15,5,slot,pin)) }
unsafe fn webbrick_map_irq(dev:*const pci_dev,slot:u8,pin:u8)->c_int { let t=[[-1;5],[-1;5],[29;5],[-1;5],[30;5],[-1;5],[-1;5],[35,35,34,33,32],[39,39,38,37,36],[43,43,42,41,40],[47,47,46,45,44],[-1;5],[-1;5]]; isa_irq_fixup(dev,common_table_lookup(&t,7,17,5,slot,pin)) }
unsafe fn clipper_map_irq(dev:*const pci_dev,slot:u8,pin:u8)->c_int { let t=[[24,24,25,26,27],[28,28,29,30,31],[32,32,33,34,35],[36,36,37,38,39],[40,40,41,42,43],[44,44,45,46,47],[-1;5]]; let irq=common_table_lookup(&t,1,7,5,slot,pin); let hose=(*dev).sysdata; isa_irq_fixup(dev,if irq>0 {irq+16*(*hose).index}else{irq}) }

unsafe fn monet_swizzle(dev:*mut pci_dev,pinp:*mut u8)->u8 { let hose=(*dev).sysdata; let mut pin=*pinp as c_int; let slot; if (*(*dev).bus).parent.is_null() { slot=PCI_SLOT((*dev).devfn); } else if (*hose).index==1 && PCI_SLOT((*(*dev).bus).self_.devfn)==8 { slot=PCI_SLOT((*dev).devfn); } else { loop { if (*hose).index==1 && PCI_SLOT((*(*dev).bus).self_.devfn)==8 { slot=PCI_SLOT((*dev).devfn); break; } pin=pci_swizzle_interrupt_pin(dev,pin as u8) as c_int; dev=(*dev).bus.self_; slot=PCI_SLOT((*dev).devfn); if (*(*dev).bus).self_.is_null() { break; } } } *pinp=pin as u8; slot }

unsafe fn dp264_init_pci(){common_init_pci();SMC669_Init(0);locate_and_init_vga(core::ptr::null_mut());}
unsafe fn monet_init_pci(){common_init_pci();SMC669_Init(1);es1888_init();locate_and_init_vga(core::ptr::null_mut());}
unsafe fn clipper_init_pci(){common_init_pci();locate_and_init_vga(core::ptr::null_mut());}
unsafe fn webbrick_init_arch(){tsunami_init_arch();(*hose_head).sg_isa.align_entry=4;(*hose_head).sg_pci.align_entry=4;}

static mut DP264_MV: alpha_machine_vector = alpha_machine_vector {
    vector_name: "DP264", DO_EV6_MMU, DO_DEFAULT_RTC, DO_TSUNAMI_IO,
    machine_check: Some(tsunami_machine_check), max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE, min_mem_address: DEFAULT_MEM_BASE,
    pci_dac_offset: TSUNAMI_DAC_OFFSET, nr_irqs: 64, device_interrupt: Some(dp264_device_interrupt),
    init_arch: Some(tsunami_init_arch), init_irq: Some(dp264_init_irq), init_rtc: Some(common_init_rtc),
    init_pci: Some(dp264_init_pci), kill_arch: Some(tsunami_kill_arch), pci_map_irq: Some(dp264_map_irq), pci_swizzle: Some(common_swizzle),
};
static mut MONET_MV: alpha_machine_vector = alpha_machine_vector {
    vector_name: "Monet", DO_EV6_MMU, DO_DEFAULT_RTC, DO_TSUNAMI_IO,
    machine_check: Some(tsunami_machine_check), max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE, min_mem_address: DEFAULT_MEM_BASE, pci_dac_offset: TSUNAMI_DAC_OFFSET,
    nr_irqs: 64, device_interrupt: Some(dp264_device_interrupt), init_arch: Some(tsunami_init_arch), init_irq: Some(dp264_init_irq), init_rtc: Some(common_init_rtc), init_pci: Some(monet_init_pci), kill_arch: Some(tsunami_kill_arch), pci_map_irq: Some(monet_map_irq), pci_swizzle: Some(monet_swizzle),
};
static mut WEBBRICK_MV: alpha_machine_vector = alpha_machine_vector {
    vector_name: "Webbrick", DO_EV6_MMU, DO_DEFAULT_RTC, DO_TSUNAMI_IO,
    machine_check: Some(tsunami_machine_check), max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS, min_io_address: DEFAULT_IO_BASE, min_mem_address: DEFAULT_MEM_BASE, pci_dac_offset: TSUNAMI_DAC_OFFSET,
    nr_irqs: 64, device_interrupt: Some(dp264_device_interrupt), init_arch: Some(webbrick_init_arch), init_irq: Some(dp264_init_irq), init_rtc: Some(common_init_rtc), init_pci: Some(common_init_pci), kill_arch: Some(tsunami_kill_arch), pci_map_irq: Some(webbrick_map_irq), pci_swizzle: Some(common_swizzle),
};
static mut CLIPPER_MV: alpha_machine_vector = alpha_machine_vector {
    vector_name: "Clipper", DO_EV6_MMU, DO_DEFAULT_RTC, DO_TSUNAMI_IO,
    machine_check: Some(tsunami_machine_check), max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS, min_io_address: DEFAULT_IO_BASE, min_mem_address: DEFAULT_MEM_BASE, pci_dac_offset: TSUNAMI_DAC_OFFSET,
    nr_irqs: 64, device_interrupt: Some(dp264_device_interrupt), init_arch: Some(tsunami_init_arch), init_irq: Some(clipper_init_irq), init_rtc: Some(common_init_rtc), init_pci: Some(clipper_init_pci), kill_arch: Some(tsunami_kill_arch), pci_map_irq: Some(clipper_map_irq), pci_swizzle: Some(common_swizzle),
};
static mut SHARK_MV: alpha_machine_vector = alpha_machine_vector {
    vector_name: "Shark", DO_EV6_MMU, DO_DEFAULT_RTC, DO_TSUNAMI_IO,
    machine_check: Some(tsunami_machine_check), max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS, min_io_address: DEFAULT_IO_BASE, min_mem_address: DEFAULT_MEM_BASE, pci_dac_offset: TSUNAMI_DAC_OFFSET,
    nr_irqs: 64, device_interrupt: Some(dp264_device_interrupt), init_arch: Some(tsunami_init_arch), init_irq: Some(clipper_init_irq), init_rtc: Some(common_init_rtc), init_pci: Some(common_init_pci), kill_arch: Some(tsunami_kill_arch), pci_map_irq: Some(clipper_map_irq), pci_swizzle: Some(common_swizzle),
};
// Sharks use Clipper's interrupt-routing functions.  Webbrick, Monet and Clipper
// are compiled unconditionally with DP264; setup_arch selects the active vector.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
