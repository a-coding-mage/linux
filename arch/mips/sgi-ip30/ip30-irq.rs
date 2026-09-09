// SPDX-License-Identifier: GPL-2.0
/*
 * ip30-irq.rs: Highlevel interrupt handling for IP30 architecture.
 *
 * Translated from ip30-irq.c. Kernel declarations and constants supplied by
 * the surrounding Linux/MIPS sources are intentionally referenced externally.
 */

#[repr(C)]
pub struct heart_irq_data {
    pub irq_mask: *mut u64,
    pub cpu: i32,
}

// DECLARE_BITMAP(heart_irq_map, HEART_NUM_IRQS)
static mut heart_irq_map: [usize; 1] = [0; 1];
// DEFINE_PER_CPU(unsigned long, irq_enable_mask)
static mut irq_enable_mask: usize = 0;

extern "C" {
    fn find_first_zero_bit(addr: *const usize, nbits: usize) -> i32;
    fn test_and_set_bit(nr: i32, addr: *mut usize) -> bool;
    fn clear_bit(nr: u64, addr: *mut usize);
    fn set_bit(nr: u64, addr: *mut usize);
    fn smp_processor_id() -> i32;
    fn heart_read(addr: *const u64) -> u64;
    fn heart_write(value: u64, addr: *mut u64);
    fn scheduler_ipi();
    fn generic_smp_call_function_interrupt();
    fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: u64) -> i32;
    fn spurious_interrupt();
    fn panic(msg: *const u8) -> !;
    fn pr_alert(fmt: *const u8, ...);
    fn irq_data_get_irq_chip_data(d: *mut irq_data) -> *mut core::ffi::c_void;
    fn irqd_is_started(d: *mut irq_data) -> bool;
    fn cpumask_first_and(mask: *const cpumask, online: *const cpumask) -> i32;
    fn irq_data_update_effective_affinity(d: *mut irq_data, mask: *const cpumask);
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(p: *mut core::ffi::c_void);
    fn irq_domain_set_info(domain: *mut irq_domain, virq: u32, hwirq: i32,
                           chip: *mut irq_chip, chip_data: *mut heart_irq_data,
                           handler: unsafe extern "C" fn(), a: *mut core::ffi::c_void,
                           b: *mut core::ffi::c_void);
    fn irq_domain_get_irq_data(domain: *mut irq_domain, virq: u32) -> *mut irq_data;
    fn mips_cpu_irq_init();
    fn irq_domain_alloc_named_fwnode(name: *const u8) -> *mut fwnode_handle;
    fn irq_domain_create_linear(fn_: *mut fwnode_handle, size: u32,
                                ops: *const irq_domain_ops, host_data: *mut core::ffi::c_void)
                                -> *mut irq_domain;
    fn irq_set_default_domain(domain: *mut irq_domain);
    fn irq_set_percpu_devid(irq: u32);
    fn irq_set_chained_handler_and_data(irq: u32, handler: unsafe extern "C" fn(*mut irq_desc), data: *mut irq_domain);
    fn handle_level_irq();
}

#[repr(C)] pub struct irq_desc;
#[repr(C)] pub struct irq_domain;
#[repr(C)] pub struct irq_data { pub hwirq: u64, pub chip_data: *mut core::ffi::c_void }
#[repr(C)] pub struct irq_chip { pub name: *const u8, pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)> }
#[repr(C)] pub struct irq_domain_ops { pub alloc: Option<unsafe extern "C" fn(*mut irq_domain, u32, u32, *mut core::ffi::c_void) -> i32>, pub free: Option<unsafe extern "C" fn(*mut irq_domain, u32, u32)> }
#[repr(C)] pub struct irq_alloc_info;
#[repr(C)] pub struct cpumask;
#[repr(C)] pub struct fwnode_handle;

// HEART registers and constants are supplied by asm/sgi/heart.h.
extern "C" { static mut heart_regs: *mut heart_regs_t; static cpu_online_mask: *const cpumask; }
#[repr(C)] pub struct heart_regs_t { pub isr: u64, pub imr: [u64; 4], pub cause: u64, pub clear_isr: u64, pub mem_err_addr: u64 }

unsafe fn heart_alloc_int() -> i32 {
    loop {
        let bit = find_first_zero_bit(heart_irq_map.as_ptr(), HEART_NUM_IRQS as usize);
        if bit >= HEART_NUM_IRQS { return -ENOSPC; }
        if !test_and_set_bit(bit, heart_irq_map.as_mut_ptr()) { return bit; }
    }
}

unsafe extern "C" fn ip30_error_irq(_desc: *mut irq_desc) {
    let cpu = smp_processor_id();
    let pending = heart_read(&(*heart_regs).isr);
    let mask = heart_read(&(*heart_regs).imr[cpu as usize]);
    let cause = heart_read(&(*heart_regs).cause);
    let error_irqs = pending & HEART_L4_INT_MASK & mask;
    if error_irqs == 0 { return; }
    heart_write(mask & !pending, &mut (*heart_regs).imr[cpu as usize]);
    heart_write(HEART_L4_INT_MASK, &mut (*heart_regs).clear_isr);
    if cause != 0 {
        pr_alert(b"IP30: CPU%d: HEART ATTACK! ISR = 0x%.16llx, IMR = 0x%.16llx, CAUSE = 0x%.16llx\n\0".as_ptr(), cpu, pending, mask, cause);
        if cause & HC_COR_MEM_ERR != 0 {
            let err_reg = heart_read(&(*heart_regs).mem_err_addr);
            pr_alert(b"  HEART_MEMERR_ADDR = 0x%.16llx\n\0".as_ptr(), err_reg);
        }
        let mut i = HEART_ERR_MASK_END;
        while i >= HEART_ERR_MASK_START { if (pending >> i) & 1 != 0 { pr_alert(b"  HEART Error IRQ #%d\n\0".as_ptr(), i); } i -= 1; }
        panic(b"IP30: Fatal Error !\n\0".as_ptr());
    }
    heart_write(mask, &mut (*heart_regs).imr[cpu as usize]);
}

unsafe extern "C" fn ip30_normal_irq(desc: *mut irq_desc) {
    let cpu = smp_processor_id();
    let mut pend = heart_read(&(*heart_regs).isr);
    let mask = heart_read(&(*heart_regs).imr[cpu as usize]) & (HEART_L0_INT_MASK | HEART_L1_INT_MASK | HEART_L2_INT_MASK);
    pend &= mask;
    if pend == 0 { return; }
    #[cfg(CONFIG_SMP)] {
        if pend & (1u64 << HEART_L2_INT_RESCHED_CPU_0) != 0 { heart_write(1u64 << HEART_L2_INT_RESCHED_CPU_0, &mut (*heart_regs).clear_isr); scheduler_ipi(); return; }
        if pend & (1u64 << HEART_L2_INT_RESCHED_CPU_1) != 0 { heart_write(1u64 << HEART_L2_INT_RESCHED_CPU_1, &mut (*heart_regs).clear_isr); scheduler_ipi(); return; }
        if pend & (1u64 << HEART_L2_INT_CALL_CPU_0) != 0 { heart_write(1u64 << HEART_L2_INT_CALL_CPU_0, &mut (*heart_regs).clear_isr); generic_smp_call_function_interrupt(); return; }
        if pend & (1u64 << HEART_L2_INT_CALL_CPU_1) != 0 { heart_write(1u64 << HEART_L2_INT_CALL_CPU_1, &mut (*heart_regs).clear_isr); generic_smp_call_function_interrupt(); return; }
    }
    let _ = generic_handle_domain_irq(core::ptr::null_mut(), pend.trailing_zeros() as u64);
    let _ = desc;
}

unsafe extern "C" fn ip30_ack_heart_irq(d: *mut irq_data) { heart_write(1u64 << (*d).hwirq, &mut (*heart_regs).clear_isr); }
unsafe extern "C" fn ip30_mask_heart_irq(d: *mut irq_data) { let hd = irq_data_get_irq_chip_data(d) as *mut heart_irq_data; clear_bit((*d).hwirq, &mut irq_enable_mask); heart_write(irq_enable_mask as u64, &mut (*heart_regs).imr[(*hd).cpu as usize]); }
unsafe extern "C" fn ip30_mask_and_ack_heart_irq(d: *mut irq_data) { ip30_mask_heart_irq(d); ip30_ack_heart_irq(d); }
unsafe extern "C" fn ip30_unmask_heart_irq(d: *mut irq_data) { let hd = irq_data_get_irq_chip_data(d) as *mut heart_irq_data; set_bit((*d).hwirq, &mut irq_enable_mask); heart_write(irq_enable_mask as u64, &mut (*heart_regs).imr[(*hd).cpu as usize]); }
unsafe extern "C" fn ip30_set_heart_irq_affinity(d: *mut irq_data, mask: *const cpumask, _force: bool) -> i32 { let hd = irq_data_get_irq_chip_data(d) as *mut heart_irq_data; if hd.is_null() { return -EINVAL; } if irqd_is_started(d) { ip30_mask_and_ack_heart_irq(d); } (*hd).cpu = cpumask_first_and(mask, cpu_online_mask); if irqd_is_started(d) { ip30_unmask_heart_irq(d); } irq_data_update_effective_affinity(d, mask); 0 }
static mut heart_irq_chip: irq_chip = irq_chip { name: b"HEART\0".as_ptr(), irq_ack: Some(ip30_ack_heart_irq) };
unsafe extern "C" fn heart_domain_alloc(_domain: *mut irq_domain, _virq: u32, nr_irqs: u32, arg: *mut core::ffi::c_void) -> i32 { if nr_irqs > 1 || arg.is_null() { return -EINVAL; } let hd = kzalloc_obj::<heart_irq_data>(); if hd.is_null() { return -12; } let hwirq = heart_alloc_int(); if hwirq < 0 { kfree(hd as *mut core::ffi::c_void); return -11; } 0 }
unsafe extern "C" fn heart_domain_free(_domain: *mut irq_domain, _virq: u32, nr_irqs: u32) { if nr_irqs <= 1 {} }
static heart_domain_ops: irq_domain_ops = irq_domain_ops { alloc: Some(heart_domain_alloc), free: Some(heart_domain_free) };

pub unsafe extern "C" fn ip30_install_ipi() { let cpu = smp_processor_id(); set_bit(HEART_L2_INT_RESCHED_CPU_0 + cpu as u64, &mut irq_enable_mask); set_bit(HEART_L2_INT_CALL_CPU_0 + cpu as u64, &mut irq_enable_mask); heart_write(irq_enable_mask as u64, &mut (*heart_regs).imr[cpu as usize]); }

pub unsafe extern "C" fn arch_init_irq() {
    mips_cpu_irq_init();
    for cpu in 0..4 { heart_write(HEART_CLR_ALL_MASK, &mut (*heart_regs).imr[cpu]); }
    heart_write(HEART_ACK_ALL_MASK, &mut (*heart_regs).clear_isr);
    irq_enable_mask |= HEART_CPU0_ERR_MASK as usize; heart_write(irq_enable_mask as u64, &mut (*heart_regs).imr[0]);
    irq_enable_mask |= HEART_CPU1_ERR_MASK as usize; heart_write(irq_enable_mask as u64, &mut (*heart_regs).imr[1]);
    for i in HEART_L4_INT_XWID_ERR_9..=HEART_L4_INT_HEART_EXCP { set_bit(i as u64, heart_irq_map.as_mut_ptr()); }
    let _ = cpu_online_mask;
}

// External constants from the architecture headers.
extern "C" { static HEART_NUM_IRQS: i32; static ENOSPC: i32; static EINVAL: i32; static HC_COR_MEM_ERR: u64; static HEART_L4_INT_MASK: u64; static HEART_L0_INT_MASK: u64; static HEART_L1_INT_MASK: u64; static HEART_L2_INT_MASK: u64; static HEART_CLR_ALL_MASK: u64; static HEART_ACK_ALL_MASK: u64; static HEART_CPU0_ERR_MASK: u64; static HEART_CPU1_ERR_MASK: u64; static HEART_ERR_MASK_END: u64; static HEART_ERR_MASK_START: u64; static HEART_L2_INT_RESCHED_CPU_0: u64; static HEART_L2_INT_RESCHED_CPU_1: u64; static HEART_L2_INT_CALL_CPU_0: u64; static HEART_L2_INT_CALL_CPU_1: u64; static HEART_L4_INT_XWID_ERR_9: u64; static HEART_L4_INT_HEART_EXCP: u64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
