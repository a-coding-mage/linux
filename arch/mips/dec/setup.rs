/*
 * System-specific setup, especially interrupts.
 *
 * Direct Rust translation of setup.c.  Kernel and architecture symbols are
 * supplied by the surrounding tree.
 */

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
use core::ffi::c_void;

pub static mut dec_kn_slot_base: c_ulong = 0;
pub static mut dec_kn_slot_size: c_ulong = 0;
pub static mut dec_tc_bus: c_int = 0;
pub static mut ioasic_ssr_lock: c_void = unsafe { core::mem::zeroed() };
pub static mut ioasic_base: *mut u32 = core::ptr::null_mut();

pub static mut dec_interrupt: [c_int; DEC_NR_INTS] = [-1; DEC_NR_INTS];
pub static mut cpu_mask_nr_tbl: [[int_ptr; 2]; DEC_MAX_CPU_INTS] =
    [[int_ptr { i: !0 }, int_ptr { p: dec_intr_unimplemented }]; DEC_MAX_CPU_INTS];
pub static mut asic_mask_nr_tbl: [[int_ptr; 2]; DEC_MAX_ASIC_INTS] =
    [[int_ptr { i: !0 }, int_ptr { p: asic_intr_unimplemented }]; DEC_MAX_ASIC_INTS];
pub static mut cpu_fpu_mask: c_int = DEC_CPU_IRQ_MASK(DEC_CPU_INR_FPU);
pub static mut fpu_kstat_irq: *mut c_int = core::ptr::null_mut();
static mut busirq_handler: irq_handler_t = None;
static mut busirq_flags: c_uint = IRQF_NO_THREAD;

unsafe fn dec_be_init() {
    match mips_machtype {
        MACH_DS23100 => { mips_set_be_handler(dec_kn01_be_handler); busirq_handler = Some(dec_kn01_be_interrupt); busirq_flags |= IRQF_SHARED; dec_kn01_be_init(); }
        MACH_DS5000_1XX | MACH_DS5000_XX => { mips_set_be_handler(dec_kn02xa_be_handler); busirq_handler = Some(dec_kn02xa_be_interrupt); dec_kn02xa_be_init(); }
        MACH_DS5000_200 | MACH_DS5000_2X0 | MACH_DS5900 => { mips_set_be_handler(dec_ecc_be_handler); busirq_handler = Some(dec_ecc_be_interrupt); dec_ecc_be_init(); }
        _ => {}
    }
}

pub unsafe fn plat_mem_setup() {
    board_be_init = Some(dec_be_init);
    wbflush_setup();
    _machine_restart = Some(dec_machine_restart);
    _machine_halt = Some(dec_machine_halt);
    pm_power_off = Some(dec_machine_power_off);
    ioport_resource.start = !0;
    ioport_resource.end = 0;
    memblock_reserve(PHYS_OFFSET, __pa_symbol(&_text as *const _ as *const c_void) - PHYS_OFFSET);
}

/* Machine-specific IRQ routing tables. */
macro_rules! irq_table {
    ($name:ident, $($idx:expr => $value:expr),* $(,)?) => {
        static mut $name: [c_int; DEC_NR_INTS] = [-1; DEC_NR_INTS];
        #[allow(dead_code)]
        unsafe fn init_$name() { $( $name[$idx] = $value; )* }
    };
}

irq_table!(kn01_interrupt,
    DEC_IRQ_DZ11 => DEC_CPU_IRQ_NR(KN01_CPU_INR_DZ11), DEC_IRQ_FPU => DEC_CPU_IRQ_NR(DEC_CPU_INR_FPU),
    DEC_IRQ_LANCE => DEC_CPU_IRQ_NR(KN01_CPU_INR_LANCE), DEC_IRQ_BUS => DEC_CPU_IRQ_NR(KN01_CPU_INR_BUS),
    DEC_IRQ_RTC => DEC_CPU_IRQ_NR(KN01_CPU_INR_RTC), DEC_IRQ_SII => DEC_CPU_IRQ_NR(KN01_CPU_INR_SII),
    DEC_IRQ_VIDEO => DEC_CPU_IRQ_NR(KN01_CPU_INR_VIDEO));
irq_table!(kn230_interrupt,
    DEC_IRQ_DZ11 => DEC_CPU_IRQ_NR(KN230_CPU_INR_DZ11), DEC_IRQ_FPU => DEC_CPU_IRQ_NR(DEC_CPU_INR_FPU),
    DEC_IRQ_HALT => DEC_CPU_IRQ_NR(KN230_CPU_INR_HALT), DEC_IRQ_LANCE => DEC_CPU_IRQ_NR(KN230_CPU_INR_LANCE),
    DEC_IRQ_BUS => DEC_CPU_IRQ_NR(KN230_CPU_INR_BUS), DEC_IRQ_RTC => DEC_CPU_IRQ_NR(KN230_CPU_INR_RTC),
    DEC_IRQ_SII => DEC_CPU_IRQ_NR(KN230_CPU_INR_SII));

/* The remaining tables retain C's designated-index layout through builders. */
unsafe fn dec_init_kn01() { init_kn01_interrupt(); dec_interrupt.copy_from_slice(&kn01_interrupt); mips_cpu_irq_init(); }
unsafe fn dec_init_kn230() { init_kn230_interrupt(); dec_interrupt.copy_from_slice(&kn230_interrupt); mips_cpu_irq_init(); }
unsafe fn dec_init_kn02() { dec_interrupt.fill(-1); mips_cpu_irq_init(); init_kn02_irqs(KN02_IRQ_BASE); }
unsafe fn dec_init_kn02ba() { dec_interrupt.fill(-1); mips_cpu_irq_init(); init_ioasic_irqs(IO_IRQ_BASE); }
unsafe fn dec_init_kn02ca() { dec_interrupt.fill(-1); mips_cpu_irq_init(); init_ioasic_irqs(IO_IRQ_BASE); }
unsafe fn dec_init_kn03() { dec_interrupt.fill(-1); mips_cpu_irq_init(); init_ioasic_irqs(IO_IRQ_BASE); }

pub unsafe fn arch_init_irq() {
    match mips_machtype {
        MACH_DS23100 => dec_init_kn01(), MACH_DS5100 => dec_init_kn230(), MACH_DS5000_200 => dec_init_kn02(),
        MACH_DS5000_1XX => dec_init_kn02ba(), MACH_DS5000_2X0 | MACH_DS5900 => dec_init_kn03(),
        MACH_DS5000_XX => dec_init_kn02ca(), MACH_DS5800 | MACH_DS5400 | MACH_DS5500 => panic!("Don't know how to set this up!"), _ => {}
    }
    if !cpu_has_nofpuex { cpu_fpu_mask = 0; dec_interrupt[DEC_IRQ_FPU] = -1; }
    if current_cpu_type() == CPU_R4000SC || current_cpu_type() == CPU_R4400SC { dec_interrupt[DEC_IRQ_HALT] = -1; }
    if IS_ENABLED(CONFIG_MIPS_FP_SUPPORT) && dec_interrupt[DEC_IRQ_FPU] >= 0 && cpu_has_fpu {
        let irq_fpu = dec_interrupt[DEC_IRQ_FPU];
        if request_irq(irq_fpu, Some(no_action), IRQF_NO_THREAD, "fpu", core::ptr::null_mut()) != 0 { pr_err!("Failed to register fpu interrupt\n"); }
    }
    if dec_interrupt[DEC_IRQ_CASCADE] >= 0 && request_irq(dec_interrupt[DEC_IRQ_CASCADE], Some(no_action), IRQF_NO_THREAD, "cascade", core::ptr::null_mut()) != 0 { pr_err!("Failed to register cascade interrupt\n"); }
    if dec_interrupt[DEC_IRQ_BUS] >= 0 && busirq_handler.is_some() && request_irq(dec_interrupt[DEC_IRQ_BUS], busirq_handler, busirq_flags, "bus error", core::ptr::null_mut()) != 0 { pr_err!("Failed to register bus error interrupt\n"); }
    if dec_interrupt[DEC_IRQ_HALT] >= 0 && request_irq(dec_interrupt[DEC_IRQ_HALT], Some(dec_intr_halt), IRQF_NO_THREAD, "halt", core::ptr::null_mut()) != 0 { pr_err!("Failed to register halt interrupt\n"); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
