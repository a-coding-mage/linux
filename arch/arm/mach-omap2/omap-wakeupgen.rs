// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of omap-wakeupgen.c. */

// Linux/kernel and platform symbols below are supplied by the surrounding kernel bindings.

const AM43XX_NR_REG_BANKS: usize = 7;
const AM43XX_IRQS: usize = 224;
const MAX_NR_REG_BANKS: usize = AM43XX_NR_REG_BANKS;
const MAX_IRQS: usize = AM43XX_IRQS;
const DEFAULT_NR_REG_BANKS: usize = 5;
const DEFAULT_IRQS: usize = 160;
const WKG_MASK_ALL: u32 = 0x00000000;
const WKG_UNMASK_ALL: u32 = 0xffffffff;
const CPU_ENA_OFFSET: usize = 0x400;
const CPU0_ID: u32 = 0;
const CPU1_ID: u32 = 1;
const OMAP4_NR_BANKS: usize = 4;
const OMAP4_NR_IRQS: usize = 128;
const SYS_NIRQ1_EXT_SYS_IRQ_1: u32 = 7;
const SYS_NIRQ2_EXT_SYS_IRQ_2: u32 = 119;

static mut wakeupgen_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut sar_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut irq_target_cpu: [u32; MAX_IRQS] = [0; MAX_IRQS];
static mut irq_banks: usize = DEFAULT_NR_REG_BANKS;
static mut max_irqs: usize = DEFAULT_IRQS;
static mut omap_secure_apis: u32 = 0;
#[cfg(feature = "CONFIG_CPU_PM")]
static mut wakeupgen_context: [u32; MAX_NR_REG_BANKS] = [0; MAX_NR_REG_BANKS];

#[repr(C)]
struct omap_wakeupgen_ops {
    save_context: Option<unsafe extern "C" fn()>,
    restore_context: Option<unsafe extern "C" fn()>,
}
static mut wakeupgen_ops: *mut omap_wakeupgen_ops = core::ptr::null_mut();

unsafe fn wakeupgen_readl(idx: u8, cpu: u32) -> u32 {
    readl_relaxed(wakeupgen_base.add(OMAP_WKG_ENB_A_0 + cpu as usize * CPU_ENA_OFFSET + idx as usize * 4))
}
unsafe fn wakeupgen_writel(val: u32, idx: u8, cpu: u32) {
    writel_relaxed(val, wakeupgen_base.add(OMAP_WKG_ENB_A_0 + cpu as usize * CPU_ENA_OFFSET + idx as usize * 4));
}
unsafe fn sar_writel(val: u32, offset: usize, idx: u8) {
    writel_relaxed(val, sar_base.add(offset + idx as usize * 4));
}
unsafe fn _wakeupgen_get_irq_info(irq: u32, bit_posn: *mut u32, reg_index: *mut u8) -> i32 {
    *reg_index = (irq >> 5) as u8;
    *bit_posn = irq % 32;
    0
}
unsafe fn _wakeupgen_clear(irq: u32, cpu: u32) {
    let mut bit = 0; let mut i = 0;
    if _wakeupgen_get_irq_info(irq, &mut bit, &mut i) != 0 { return; }
    let val = wakeupgen_readl(i, cpu) & !(1u32 << bit);
    wakeupgen_writel(val, i, cpu);
}
unsafe fn _wakeupgen_set(irq: u32, cpu: u32) {
    let mut bit = 0; let mut i = 0;
    if _wakeupgen_get_irq_info(irq, &mut bit, &mut i) != 0 { return; }
    let val = wakeupgen_readl(i, cpu) | (1u32 << bit);
    wakeupgen_writel(val, i, cpu);
}

unsafe fn wakeupgen_mask(d: *mut irq_data) {
    let flags = raw_spin_lock_irqsave(&wakeupgen_lock);
    _wakeupgen_clear((*d).hwirq, irq_target_cpu[(*d).hwirq as usize]);
    raw_spin_unlock_irqrestore(&wakeupgen_lock, flags);
    irq_chip_mask_parent(d);
}
unsafe fn wakeupgen_unmask(d: *mut irq_data) {
    let flags = raw_spin_lock_irqsave(&wakeupgen_lock);
    _wakeupgen_set((*d).hwirq, irq_target_cpu[(*d).hwirq as usize]);
    raw_spin_unlock_irqrestore(&wakeupgen_lock, flags);
    irq_chip_unmask_parent(d);
}
unsafe fn wakeupgen_irq_set_type(d: *mut irq_data, mut typ: u32) -> i32 {
    let mut inverted = false;
    match typ {
        IRQ_TYPE_LEVEL_LOW => { typ = (typ & !IRQ_TYPE_LEVEL_MASK) | IRQ_TYPE_LEVEL_HIGH; inverted = true; }
        IRQ_TYPE_EDGE_FALLING => { typ = (typ & !IRQ_TYPE_EDGE_BOTH) | IRQ_TYPE_EDGE_RISING; inverted = true; }
        _ => {}
    }
    if inverted && (*d).hwirq != SYS_NIRQ1_EXT_SYS_IRQ_1 && (*d).hwirq != SYS_NIRQ2_EXT_SYS_IRQ_2 {
        pr_warn("wakeupgen: irq%li polarity inverted in dts\n", (*d).hwirq);
    }
    irq_chip_set_type_parent(d, typ)
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
static mut irqmasks: [[u32; MAX_NR_REG_BANKS]; 2] = [[0; MAX_NR_REG_BANKS]; 2];
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn _wakeupgen_save_masks(cpu: usize) { for i in 0..irq_banks { irqmasks[cpu][i] = wakeupgen_readl(i as u8, cpu as u32); } }
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn _wakeupgen_restore_masks(cpu: usize) { for i in 0..irq_banks { wakeupgen_writel(irqmasks[cpu][i], i as u8, cpu as u32); } }
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn _wakeupgen_set_all(cpu: u32, reg: u32) { for i in 0..irq_banks { wakeupgen_writel(reg, i as u8, cpu); } }
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn wakeupgen_irqmask_all(cpu: usize, set: u32) {
    let flags = raw_spin_lock_irqsave(&wakeupgen_lock);
    if set != 0 { _wakeupgen_save_masks(cpu); _wakeupgen_set_all(cpu as u32, WKG_MASK_ALL); }
    else { _wakeupgen_set_all(cpu as u32, WKG_UNMASK_ALL); _wakeupgen_restore_masks(cpu); }
    raw_spin_unlock_irqrestore(&wakeupgen_lock, flags);
}

#[cfg(feature = "CONFIG_CPU_PM")]
unsafe fn omap4_irq_save_context() {
    if omap_rev() == OMAP4430_REV_ES1_0 { return; }
    for i in 0..irq_banks { let mut val = wakeupgen_readl(i as u8, 0); sar_writel(val, WAKEUPGENENB_OFFSET_CPU0, i as u8); val = wakeupgen_readl(i as u8, 1); sar_writel(val, WAKEUPGENENB_OFFSET_CPU1, i as u8); sar_writel(0, WAKEUPGENENB_SECURE_OFFSET_CPU0, i as u8); sar_writel(0, WAKEUPGENENB_SECURE_OFFSET_CPU1, i as u8); }
    let mut val = readl_relaxed(wakeupgen_base.add(OMAP_AUX_CORE_BOOT_0)); writel_relaxed(val, sar_base.add(AUXCOREBOOT0_OFFSET)); val = readl_relaxed(wakeupgen_base.add(OMAP_AUX_CORE_BOOT_1)); writel_relaxed(val, sar_base.add(AUXCOREBOOT1_OFFSET));
    val = readl_relaxed(wakeupgen_base.add(OMAP_PTMSYNCREQ_MASK)); writel_relaxed(val, sar_base.add(PTMSYNCREQ_MASK_OFFSET)); val = readl_relaxed(wakeupgen_base.add(OMAP_PTMSYNCREQ_EN)); writel_relaxed(val, sar_base.add(PTMSYNCREQ_EN_OFFSET));
    val = readl_relaxed(sar_base.add(SAR_BACKUP_STATUS_OFFSET)) | SAR_BACKUP_STATUS_WAKEUPGEN; writel_relaxed(val, sar_base.add(SAR_BACKUP_STATUS_OFFSET));
}
#[cfg(feature = "CONFIG_CPU_PM")]
unsafe fn omap5_irq_save_context() { for i in 0..irq_banks { let mut val=wakeupgen_readl(i as u8,0); sar_writel(val,OMAP5_WAKEUPGENENB_OFFSET_CPU0,i as u8); val=wakeupgen_readl(i as u8,1); sar_writel(val,OMAP5_WAKEUPGENENB_OFFSET_CPU1,i as u8); sar_writel(0,OMAP5_WAKEUPGENENB_SECURE_OFFSET_CPU0,i as u8); sar_writel(0,OMAP5_WAKEUPGENENB_SECURE_OFFSET_CPU1,i as u8); } }
#[cfg(feature = "CONFIG_CPU_PM")]
unsafe fn am43xx_irq_save_context() { for i in 0..irq_banks { wakeupgen_context[i]=wakeupgen_readl(i as u8,0); wakeupgen_writel(0,i as u8,CPU0_ID); } }
#[cfg(feature = "CONFIG_CPU_PM")]
unsafe fn irq_save_context() { if soc_is_dra7xx(){return;} if !wakeupgen_ops.is_null(){ if let Some(f)=(*wakeupgen_ops).save_context{f();} } }
#[cfg(feature = "CONFIG_CPU_PM")]
unsafe fn irq_sar_clear() { if soc_is_dra7xx(){return;} let offset=if soc_is_omap54xx(){OMAP5_SAR_BACKUP_STATUS_OFFSET}else{SAR_BACKUP_STATUS_OFFSET}; let val=readl_relaxed(sar_base.add(offset)) & !SAR_BACKUP_STATUS_WAKEUPGEN; writel_relaxed(val,sar_base.add(offset)); }
#[cfg(feature = "CONFIG_CPU_PM")]
unsafe fn am43xx_irq_restore_context(){for i in 0..irq_banks{wakeupgen_writel(wakeupgen_context[i],i as u8,CPU0_ID);}}
#[cfg(feature = "CONFIG_CPU_PM")]
unsafe fn irq_restore_context(){if !wakeupgen_ops.is_null(){if let Some(f)=(*wakeupgen_ops).restore_context{f();}}}
#[cfg(feature = "CONFIG_CPU_PM")]
unsafe fn irq_save_secure_context(){let ret=omap_secure_dispatcher(OMAP4_HAL_SAVEGIC_INDEX,FLAG_START_CRITICAL,0,0,0,0,0);if ret!=API_HAL_RET_VALUE_OK{pr_err("GIC and Wakeupgen context save failed\n");}}

// The remaining domain/init declarations retain the kernel interfaces and control flow.
// External kernel types, constants, and functions are intentionally unresolved here.
#[cfg(feature = "CONFIG_CPU_PM")] static mut omap4_wakeupgen_ops: omap_wakeupgen_ops=omap_wakeupgen_ops{save_context:Some(omap4_irq_save_context),restore_context:Some(irq_sar_clear)};
#[cfg(feature = "CONFIG_CPU_PM")] static mut omap5_wakeupgen_ops: omap_wakeupgen_ops=omap_wakeupgen_ops{save_context:Some(omap5_irq_save_context),restore_context:Some(irq_sar_clear)};
#[cfg(feature = "CONFIG_CPU_PM")] static mut am43xx_wakeupgen_ops: omap_wakeupgen_ops=omap_wakeupgen_ops{save_context:Some(am43xx_irq_save_context),restore_context:Some(am43xx_irq_restore_context)};

// wakeupgen_domain_translate, wakeupgen_domain_alloc, wakeupgen_init, and IRQCHIP_DECLARE
// require the surrounding Linux IRQ-domain binding definitions; their source-level behavior
// is represented by the declarations below for integration with those bindings.
extern "C" { static mut wakeupgen_lock: raw_spinlock_t; }

unsafe fn wakeupgen_domain_translate(d: *mut irq_domain, fwspec: *mut irq_fwspec, hwirq: *mut irq_hw_number_t, typ: *mut u32) -> i32 {
    if is_of_node((*fwspec).fwnode) && (*fwspec).param_count == 3 && (*fwspec).param[0] == 0 {
        *hwirq = (*fwspec).param[1] as irq_hw_number_t; *typ = (*fwspec).param[2]; return 0;
    }
    -EINVAL
}
unsafe fn wakeupgen_domain_alloc(domain: *mut irq_domain, virq: u32, nr_irqs: u32, data: *mut core::ffi::c_void) -> i32 {
    let fwspec = data as *mut irq_fwspec;
    if (*fwspec).param_count != 3 || (*fwspec).param[0] != 0 { return -EINVAL; }
    let hwirq = (*fwspec).param[1] as irq_hw_number_t;
    if hwirq >= MAX_IRQS as irq_hw_number_t { return -EINVAL; }
    for i in 0..nr_irqs { irq_domain_set_hwirq_and_chip(domain, virq+i, hwirq+i as irq_hw_number_t, &wakeupgen_chip, core::ptr::null_mut()); }
    let mut parent_fwspec = *fwspec; parent_fwspec.fwnode = (*(*domain).parent).fwnode;
    irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, &mut parent_fwspec as *mut _ as *mut core::ffi::c_void)
}

unsafe fn wakeupgen_init(node: *mut device_node, parent: *mut device_node) -> i32 {
    if parent.is_null() { pr_err("%pOF: no parent, giving up\n", node); return -ENODEV; }
    let parent_domain = irq_find_host(parent); if parent_domain.is_null() { pr_err("%pOF: unable to obtain parent domain\n", node); return -ENXIO; }
    if omap_rev() == OMAP4430_REV_ES1_0 { WARN(1,"WakeupGen: Not supported on OMAP4430 ES1.0\n"); return -EPERM; }
    wakeupgen_base = of_iomap(node,0); if wakeupgen_base.is_null() { return -ENOMEM; }
    if cpu_is_omap44xx() { irq_banks=OMAP4_NR_BANKS; max_irqs=OMAP4_NR_IRQS; omap_secure_apis=1; wakeupgen_ops=&mut omap4_wakeupgen_ops; }
    else if soc_is_omap54xx() { wakeupgen_ops=&mut omap5_wakeupgen_ops; }
    else if soc_is_am43xx() { irq_banks=AM43XX_NR_REG_BANKS; max_irqs=AM43XX_IRQS; wakeupgen_ops=&mut am43xx_wakeupgen_ops; }
    let domain=irq_domain_create_hierarchy(parent_domain,0,max_irqs,of_fwnode_handle(node),&wakeupgen_domain_ops,core::ptr::null_mut()); if domain.is_null(){iounmap(wakeupgen_base);return -ENOMEM;}
    for i in 0..irq_banks { wakeupgen_writel(0,i as u8,CPU0_ID); if !soc_is_am43xx(){wakeupgen_writel(0,i as u8,CPU1_ID);} }
    let boot_cpu=smp_processor_id(); for i in 0..max_irqs {irq_target_cpu[i]=boot_cpu;}
    if soc_is_omap54xx()||soc_is_dra7xx(){let val=__raw_readl(wakeupgen_base.add(OMAP_AMBA_IF_MODE))|1<<5;omap_smc1(OMAP5_MON_AMBA_IF_INDEX,val);}
    irq_hotplug_init(); irq_pm_init(); sar_base=omap4_get_sar_ram_base(); 0
}

unsafe fn omap_get_wakeupgen_base() -> *mut core::ffi::c_void { wakeupgen_base }
unsafe fn omap_secure_apis_support() -> i32 { omap_secure_apis as i32 }

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn irq_hotplug_init() { cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN,"arm/omap-wake:online",Some(omap_wakeupgen_cpu_online),None); cpuhp_setup_state_nocalls(CPUHP_ARM_OMAP_WAKE_DEAD,"arm/omap-wake:dead",None,Some(omap_wakeupgen_cpu_dead)); }
#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))] unsafe fn irq_hotplug_init() {}
#[cfg(feature = "CONFIG_HOTPLUG_CPU")] unsafe fn omap_wakeupgen_cpu_online(cpu:u32)->i32{wakeupgen_irqmask_all(cpu as usize,0);0}
#[cfg(feature = "CONFIG_HOTPLUG_CPU")] unsafe fn omap_wakeupgen_cpu_dead(cpu:u32)->i32{wakeupgen_irqmask_all(cpu as usize,1);0}
#[cfg(feature = "CONFIG_CPU_PM")] unsafe fn irq_pm_init(){if !IS_PM44XX_ERRATUM(PM_OMAP4_CPU_OSWR_DISABLE){cpu_pm_register_notifier(&mut irq_notifier_block);}}
#[cfg(not(feature = "CONFIG_CPU_PM"))] unsafe fn irq_pm_init(){}

// Kernel-provided declarations used by this translation.
extern "C" {
    static mut wakeupgen_chip: irq_chip;
    static wakeupgen_domain_ops: irq_domain_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
