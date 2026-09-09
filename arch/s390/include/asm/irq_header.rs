/* SPDX-License-Identifier: GPL-2.0 */

pub const EXT_INTERRUPT: u32 = 0;
pub const IO_INTERRUPT: u32 = 1;
pub const THIN_INTERRUPT: u32 = 2;

pub const NR_IRQS_BASE: u32 = 3;
pub const NR_IRQS: u32 = NR_IRQS_BASE;
pub const NR_IRQS_LEGACY: u32 = NR_IRQS_BASE;

/* External interruption codes */
pub const EXT_IRQ_INTERRUPT_KEY: u32 = 0x0040;
pub const EXT_IRQ_CLK_COMP: u32 = 0x1004;
pub const EXT_IRQ_CPU_TIMER: u32 = 0x1005;
pub const EXT_IRQ_WARNING_TRACK: u32 = 0x1007;
pub const EXT_IRQ_MALFUNC_ALERT: u32 = 0x1200;
pub const EXT_IRQ_EMERGENCY_SIG: u32 = 0x1201;
pub const EXT_IRQ_EXTERNAL_CALL: u32 = 0x1202;
pub const EXT_IRQ_TIMING_ALERT: u32 = 0x1406;
pub const EXT_IRQ_MEASURE_ALERT: u32 = 0x1407;
pub const EXT_IRQ_SERVICE_SIG: u32 = 0x2401;
pub const EXT_IRQ_CP_SERVICE: u32 = 0x2603;
pub const EXT_IRQ_IUCV: u32 = 0x4000;

/* Dependencies supplied by the surrounding kernel environment:
 * linux/hardirq.h, linux/percpu.h, linux/cache.h, linux/types.h,
 * and asm/ctlreg.h.
 */

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum interruption_class {
    IRQEXT_CLK,
    IRQEXT_EXC,
    IRQEXT_EMS,
    IRQEXT_TMR,
    IRQEXT_TLA,
    IRQEXT_PFL,
    IRQEXT_DSD,
    IRQEXT_VRT,
    IRQEXT_SCP,
    IRQEXT_IUC,
    IRQEXT_CMS,
    IRQEXT_CMC,
    IRQEXT_FTP,
    IRQEXT_WTI,
    IRQIO_CIO,
    IRQIO_DAS,
    IRQIO_C15,
    IRQIO_C70,
    IRQIO_TAP,
    IRQIO_VMR,
    IRQIO_CTC,
    IRQIO_ADM,
    IRQIO_CSC,
    IRQIO_VIR,
    IRQIO_QAI,
    IRQIO_APB,
    IRQIO_PCF,
    IRQIO_PCD,
    IRQIO_MSI,
    IRQIO_VAI,
    IRQIO_GAL,
    NMI_NMI,
    CPU_RST,
    NR_ARCH_IRQS,
}

#[repr(C)]
pub struct irq_stat {
    pub irqs: [core::ffi::c_uint; NR_ARCH_IRQS as usize],
}

/* DECLARE_PER_CPU_SHARED_ALIGNED(struct irq_stat, irq_stat); */
extern "C" {
    pub static mut irq_stat: irq_stat;
}

#[inline(always)]
pub unsafe fn inc_irq_stat(irq: interruption_class) {
    /* __this_cpu_inc(irq_stat.irqs[irq]); */
    let index = irq as usize;
    irq_stat.irqs[index] = irq_stat.irqs[index].wrapping_add(1);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_code_fields {
    pub subcode: u16,
    pub code: u16,
}

#[repr(C)]
pub union ext_code_union {
    pub fields: ext_code_fields,
    pub int_code: u32,
}

#[repr(C)]
pub struct ext_code {
    pub value: ext_code_union,
}

pub type ext_int_handler_t = unsafe extern "C" fn(ext_code, u32, core::ffi::c_ulong);

extern "C" {
    pub fn register_external_irq(code: u16, handler: ext_int_handler_t) -> core::ffi::c_int;
    pub fn unregister_external_irq(code: u16, handler: ext_int_handler_t) -> core::ffi::c_int;
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum irq_subclass {
    IRQ_SUBCLASS_MEASUREMENT_ALERT = 5,
    IRQ_SUBCLASS_SERVICE_SIGNAL = 9,
    IRQ_SUBCLASS_WARNING_TRACK = 33,
}

/* CR0_IRQ_SUBCLASS_MASK is composed from CR0_* constants supplied by asm/ctlreg.h:
 * (CR0_WARNING_TRACK | CR0_MALFUNCTION_ALERT_SUBMASK |
 *  CR0_EMERGENCY_SIGNAL_SUBMASK | CR0_EXTERNAL_CALL_SUBMASK |
 *  CR0_CLOCK_COMPARATOR_SUBMASK | CR0_CPU_TIMER_SUBMASK |
 *  CR0_SERVICE_SIGNAL_SUBMASK | CR0_INTERRUPT_KEY_SUBMASK |
 *  CR0_MEASUREMENT_ALERT_SUBMASK | CR0_ETR_SUBMASK | CR0_IUCV)
 */

extern "C" {
    pub fn irq_subclass_register(subclass: irq_subclass);
    pub fn irq_subclass_unregister(subclass: irq_subclass);
}

#[inline(always)]
pub const fn irq_canonicalize(irq: u32) -> u32 {
    irq
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
