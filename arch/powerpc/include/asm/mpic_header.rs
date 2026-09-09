/* SPDX-License-Identifier: GPL-2.0 */

/* Original declarations are guarded by __KERNEL__. */

pub const MPIC_GREG_BASE: u32 = 0x01000;
pub const MPIC_GREG_FEATURE_0: u32 = 0x00000;
pub const MPIC_GREG_FEATURE_LAST_SRC_MASK: u32 = 0x07ff0000;
pub const MPIC_GREG_FEATURE_LAST_SRC_SHIFT: u32 = 16;
pub const MPIC_GREG_FEATURE_LAST_CPU_MASK: u32 = 0x00001f00;
pub const MPIC_GREG_FEATURE_LAST_CPU_SHIFT: u32 = 8;
pub const MPIC_GREG_FEATURE_VERSION_MASK: u32 = 0xff;
pub const MPIC_GREG_FEATURE_1: u32 = 0x00010;
pub const MPIC_GREG_GLOBAL_CONF_0: u32 = 0x00020;
pub const MPIC_GREG_GCONF_RESET: u32 = 0x80000000;
/* On FSL MPIC implementations the Mode field is two bits wide:
 * 0b00 pass through, 0b01 mixed mode, 0b10 reserved,
 * 0b11 external proxy / coreint. */
pub const MPIC_GREG_GCONF_COREINT: u32 = 0x60000000;
pub const MPIC_GREG_GCONF_8259_PTHROU_DIS: u32 = 0x20000000;
pub const MPIC_GREG_GCONF_NO_BIAS: u32 = 0x10000000;
pub const MPIC_GREG_GCONF_BASE_MASK: u32 = 0x000fffff;
pub const MPIC_GREG_GCONF_MCK: u32 = 0x08000000;
pub const MPIC_GREG_GLOBAL_CONF_1: u32 = 0x00030;
pub const MPIC_GREG_VENDOR_0: u32 = 0x00040;
pub const MPIC_GREG_VENDOR_1: u32 = 0x00050;
pub const MPIC_GREG_VENDOR_2: u32 = 0x00060;
pub const MPIC_GREG_VENDOR_3: u32 = 0x00070;
pub const MPIC_GREG_VENDOR_ID: u32 = 0x00080;
pub const MPIC_GREG_VENDOR_ID_STEPPING_MASK: u32 = 0x00ff0000;
pub const MPIC_GREG_VENDOR_ID_STEPPING_SHIFT: u32 = 16;
pub const MPIC_GREG_VENDOR_ID_DEVICE_ID_MASK: u32 = 0x0000ff00;
pub const MPIC_GREG_VENDOR_ID_DEVICE_ID_SHIFT: u32 = 8;
pub const MPIC_GREG_VENDOR_ID_VENDOR_ID_MASK: u32 = 0x000000ff;
pub const MPIC_GREG_PROCESSOR_INIT: u32 = 0x00090;
pub const MPIC_GREG_IPI_VECTOR_PRI_0: u32 = 0x000a0;
pub const MPIC_GREG_IPI_VECTOR_PRI_1: u32 = 0x000b0;
pub const MPIC_GREG_IPI_VECTOR_PRI_2: u32 = 0x000c0;
pub const MPIC_GREG_IPI_VECTOR_PRI_3: u32 = 0x000d0;
pub const MPIC_GREG_IPI_STRIDE: u32 = 0x10;
pub const MPIC_GREG_SPURIOUS: u32 = 0x000e0;
pub const MPIC_GREG_TIMER_FREQ: u32 = 0x000f0;

pub const MPIC_TIMER_BASE: u32 = 0x01100;
pub const MPIC_TIMER_STRIDE: u32 = 0x40;
pub const MPIC_TIMER_GROUP_STRIDE: u32 = 0x1000;
pub const MPIC_TIMER_CURRENT_CNT: u32 = 0x00000;
pub const MPIC_TIMER_BASE_CNT: u32 = 0x00010;
pub const MPIC_TIMER_VECTOR_PRI: u32 = 0x00020;
pub const MPIC_TIMER_DESTINATION: u32 = 0x00030;

pub const MPIC_CPU_THISBASE: u32 = 0x00000;
pub const MPIC_CPU_BASE: u32 = 0x20000;
pub const MPIC_CPU_STRIDE: u32 = 0x01000;
pub const MPIC_CPU_IPI_DISPATCH_0: u32 = 0x00040;
pub const MPIC_CPU_IPI_DISPATCH_1: u32 = 0x00050;
pub const MPIC_CPU_IPI_DISPATCH_2: u32 = 0x00060;
pub const MPIC_CPU_IPI_DISPATCH_3: u32 = 0x00070;
pub const MPIC_CPU_IPI_DISPATCH_STRIDE: u32 = 0x00010;
pub const MPIC_CPU_CURRENT_TASK_PRI: u32 = 0x00080;
pub const MPIC_CPU_TASKPRI_MASK: u32 = 0x0000000f;
pub const MPIC_CPU_WHOAMI: u32 = 0x00090;
pub const MPIC_CPU_WHOAMI_MASK: u32 = 0x0000001f;
pub const MPIC_CPU_INTACK: u32 = 0x000a0;
pub const MPIC_CPU_EOI: u32 = 0x000b0;
pub const MPIC_CPU_MCACK: u32 = 0x000c0;

pub const MPIC_IRQ_BASE: u32 = 0x10000;
pub const MPIC_IRQ_STRIDE: u32 = 0x00020;
pub const MPIC_IRQ_VECTOR_PRI: u32 = 0x00000;
pub const MPIC_VECPRI_MASK: u32 = 0x80000000;
pub const MPIC_VECPRI_ACTIVITY: u32 = 0x40000000; /* Read Only */
pub const MPIC_VECPRI_PRIORITY_MASK: u32 = 0x000f0000;
pub const MPIC_VECPRI_PRIORITY_SHIFT: u32 = 16;
pub const MPIC_VECPRI_VECTOR_MASK: u32 = 0x000007ff;
pub const MPIC_VECPRI_POLARITY_POSITIVE: u32 = 0x00800000;
pub const MPIC_VECPRI_POLARITY_NEGATIVE: u32 = 0x00000000;
pub const MPIC_VECPRI_POLARITY_MASK: u32 = 0x00800000;
pub const MPIC_VECPRI_SENSE_LEVEL: u32 = 0x00400000;
pub const MPIC_VECPRI_SENSE_EDGE: u32 = 0x00000000;
pub const MPIC_VECPRI_SENSE_MASK: u32 = 0x00400000;
pub const MPIC_IRQ_DESTINATION: u32 = 0x00010;
pub const MPIC_FSL_BRR1: u32 = 0x00000;
pub const MPIC_FSL_BRR1_VER: u32 = 0x0000ffff;
pub const MPIC_MAX_IRQ_SOURCES: usize = 2048;
pub const MPIC_MAX_CPUS: usize = 32;
pub const MPIC_MAX_ISU: usize = 32;
pub const MPIC_MAX_ERR: usize = 32;
pub const MPIC_FSL_ERR_INT: u32 = 16;

pub const TSI108_GREG_BASE: u32 = 0x00000;
pub const TSI108_GREG_FEATURE_0: u32 = 0x00000;
pub const TSI108_GREG_GLOBAL_CONF_0: u32 = 0x00004;
pub const TSI108_GREG_VENDOR_ID: u32 = 0x0000c;
pub const TSI108_GREG_IPI_VECTOR_PRI_0: u32 = 0x00204;
pub const TSI108_GREG_IPI_STRIDE: u32 = 0x0c;
pub const TSI108_GREG_SPURIOUS: u32 = 0x00010;
pub const TSI108_GREG_TIMER_FREQ: u32 = 0x00014;
pub const TSI108_TIMER_BASE: u32 = 0x0030;
pub const TSI108_TIMER_STRIDE: u32 = 0x10;
pub const TSI108_TIMER_CURRENT_CNT: u32 = 0x00000;
pub const TSI108_TIMER_BASE_CNT: u32 = 0x00004;
pub const TSI108_TIMER_VECTOR_PRI: u32 = 0x00008;
pub const TSI108_TIMER_DESTINATION: u32 = 0x0000c;
pub const TSI108_CPU_BASE: u32 = 0x00300;
pub const TSI108_CPU_STRIDE: u32 = 0x00040;
pub const TSI108_CPU_IPI_DISPATCH_0: u32 = 0x00200;
pub const TSI108_CPU_IPI_DISPATCH_STRIDE: u32 = 0x00000;
pub const TSI108_CPU_CURRENT_TASK_PRI: u32 = 0x00000;
pub const TSI108_CPU_WHOAMI: u32 = 0xffffffff;
pub const TSI108_CPU_INTACK: u32 = 0x00004;
pub const TSI108_CPU_EOI: u32 = 0x00008;
pub const TSI108_CPU_MCACK: u32 = 0x00004;
pub const TSI108_IRQ_BASE: u32 = 0x00100;
pub const TSI108_IRQ_STRIDE: u32 = 0x00008;
pub const TSI108_IRQ_VECTOR_PRI: u32 = 0x00000;
pub const TSI108_VECPRI_VECTOR_MASK: u32 = 0x000000ff;
pub const TSI108_VECPRI_POLARITY_POSITIVE: u32 = 0x01000000;
pub const TSI108_VECPRI_POLARITY_NEGATIVE: u32 = 0x00000000;
pub const TSI108_VECPRI_SENSE_LEVEL: u32 = 0x02000000;
pub const TSI108_VECPRI_SENSE_EDGE: u32 = 0x00000000;
pub const TSI108_VECPRI_POLARITY_MASK: u32 = 0x01000000;
pub const TSI108_VECPRI_SENSE_MASK: u32 = 0x02000000;
pub const TSI108_IRQ_DESTINATION: u32 = 0x00004;

#[repr(usize)]
pub enum MpicRegIndex {
    MPIC_IDX_GREG_BASE = 0,
    MPIC_IDX_GREG_FEATURE_0,
    MPIC_IDX_GREG_GLOBAL_CONF_0,
    MPIC_IDX_GREG_VENDOR_ID,
    MPIC_IDX_GREG_IPI_VECTOR_PRI_0,
    MPIC_IDX_GREG_IPI_STRIDE,
    MPIC_IDX_GREG_SPURIOUS,
    MPIC_IDX_GREG_TIMER_FREQ,
    MPIC_IDX_TIMER_BASE,
    MPIC_IDX_TIMER_STRIDE,
    MPIC_IDX_TIMER_CURRENT_CNT,
    MPIC_IDX_TIMER_BASE_CNT,
    MPIC_IDX_TIMER_VECTOR_PRI,
    MPIC_IDX_TIMER_DESTINATION,
    MPIC_IDX_CPU_BASE,
    MPIC_IDX_CPU_STRIDE,
    MPIC_IDX_CPU_IPI_DISPATCH_0,
    MPIC_IDX_CPU_IPI_DISPATCH_STRIDE,
    MPIC_IDX_CPU_CURRENT_TASK_PRI,
    MPIC_IDX_CPU_WHOAMI,
    MPIC_IDX_CPU_INTACK,
    MPIC_IDX_CPU_EOI,
    MPIC_IDX_CPU_MCACK,
    MPIC_IDX_IRQ_BASE,
    MPIC_IDX_IRQ_STRIDE,
    MPIC_IDX_IRQ_VECTOR_PRI,
    MPIC_IDX_VECPRI_VECTOR_MASK,
    MPIC_IDX_VECPRI_POLARITY_POSITIVE,
    MPIC_IDX_VECPRI_POLARITY_NEGATIVE,
    MPIC_IDX_VECPRI_SENSE_LEVEL,
    MPIC_IDX_VECPRI_SENSE_EDGE,
    MPIC_IDX_VECPRI_POLARITY_MASK,
    MPIC_IDX_VECPRI_SENSE_MASK,
    MPIC_IDX_IRQ_DESTINATION,
    MPIC_IDX_END,
}

#[cfg(CONFIG_MPIC_U3_HT_IRQS)]
#[repr(C)]
pub struct MpicIrqFixup {
    pub base: *mut u8,
    pub applebase: *mut u8,
    pub data: u32,
    pub index: u32,
}

#[repr(C)]
pub enum MpicRegType { MpicAccessMmioLe, MpicAccessMmioBe, #[cfg(CONFIG_PPC_DCR)] MpicAccessDcr }

#[repr(C)]
pub struct MpicRegBank {
    pub base: *mut u32,
    #[cfg(CONFIG_PPC_DCR)] pub dhost: dcr_host_t,
}

#[repr(C)]
pub struct MpicIrqSave {
    pub vecprio: u32,
    pub dest: u32,
    #[cfg(CONFIG_MPIC_U3_HT_IRQS)] pub fixup_data: u32,
}

#[repr(C)]
pub struct Mpic {
    pub node: *mut device_node,
    pub irqhost: *mut irq_domain,
    pub hc_irq: irq_chip,
    #[cfg(CONFIG_MPIC_U3_HT_IRQS)] pub hc_ht_irq: irq_chip,
    #[cfg(CONFIG_SMP)] pub hc_ipi: irq_chip,
    pub hc_tm: irq_chip,
    pub hc_err: irq_chip,
    pub name: *const u8,
    pub flags: u32,
    pub isu_size: u32,
    pub isu_shift: u32,
    pub isu_mask: u32,
    pub num_sources: u32,
    pub ipi_vecs: [u32; 4],
    pub timer_vecs: [u32; 8],
    pub err_int_vecs: [u32; MPIC_MAX_ERR],
    pub spurious_vec: u32,
    #[cfg(CONFIG_MPIC_U3_HT_IRQS)] pub fixups: *mut MpicIrqFixup,
    #[cfg(CONFIG_MPIC_U3_HT_IRQS)] pub fixup_lock: raw_spinlock_t,
    pub reg_type: MpicRegType,
    pub paddr: phys_addr_t,
    pub thiscpuregs: MpicRegBank,
    pub gregs: MpicRegBank,
    pub tmregs: MpicRegBank,
    pub cpuregs: [MpicRegBank; MPIC_MAX_CPUS],
    pub isus: [MpicRegBank; MPIC_MAX_ISU],
    pub err_regs: *mut u32,
    pub protected: *mut c_ulong,
    #[cfg(CONFIG_MPIC_WEIRD)] pub hw_set: *mut u32,
    #[cfg(CONFIG_PCI_MSI)] pub msi_bitmap: msi_bitmap,
    #[cfg(CONFIG_MPIC_BROKEN_REGREAD)] pub isu_reg0_shadow: [u32; MPIC_MAX_IRQ_SOURCES],
    pub next: *mut Mpic,
    #[cfg(CONFIG_PM)] pub save_data: *mut MpicIrqSave,
}

pub const MPIC_SECONDARY: u32 = 0x00000001;
pub const MPIC_BIG_ENDIAN: u32 = 0x00000002;
pub const MPIC_U3_HT_IRQS: u32 = 0x00000004;
pub const MPIC_BROKEN_IPI: u32 = 0x00000008;
pub const MPIC_SPV_EOI: u32 = 0x00000020;
pub const MPIC_NO_PTHROU_DIS: u32 = 0x00000040;
pub const MPIC_USES_DCR: u32 = 0x00000080;
pub const MPIC_LARGE_VECTORS: u32 = 0x00000100;
pub const MPIC_ENABLE_MCK: u32 = 0x00000200;
pub const MPIC_NO_BIAS: u32 = 0x00000400;
pub const MPIC_SINGLE_DEST_CPU: u32 = 0x00001000;
pub const MPIC_ENABLE_COREINT: u32 = 0x00002000;
pub const MPIC_NO_RESET: u32 = 0x00004000;
pub const MPIC_FSL: u32 = 0x00008000;
pub const MPIC_FSL_HAS_EIMR: u32 = 0x00010000;
pub const MPIC_REGSET_MASK: u32 = 0xf0000000;
#[inline] pub const fn MPIC_REGSET(val: u32) -> u32 { (val & 0xf) << 28 }
#[inline] pub const fn MPIC_GET_REGSET(flags: u32) -> u32 { (flags >> 28) & 0xf }
pub const MPIC_REGSET_STANDARD: u32 = MPIC_REGSET(0);
pub const MPIC_REGSET_TSI108: u32 = MPIC_REGSET(1);

#[cfg(CONFIG_MPIC)]
extern "C" { pub fn fsl_mpic_primary_get_version() -> u32; }
#[cfg(not(CONFIG_MPIC))]
#[inline] pub fn fsl_mpic_primary_get_version() -> u32 { 0 }

extern "C" {
    pub fn mpic_alloc(node: *mut device_node, phys_addr: phys_addr_t, flags: u32,
                      isu_size: u32, irq_count: u32, name: *const u8) -> *mut Mpic;
    pub fn mpic_assign_isu(mpic: *mut Mpic, isu_num: u32, phys_addr: phys_addr_t);
    pub fn mpic_init(mpic: *mut Mpic);
    pub fn mpic_irq_set_priority(irq: u32, pri: u32);
    pub fn mpic_setup_this_cpu();
    pub fn mpic_teardown_this_cpu(secondary: i32);
    pub fn mpic_cpu_get_priority() -> i32;
    pub fn mpic_cpu_set_priority(prio: i32);
    pub fn mpic_request_ipis();
    pub fn smp_mpic_message_pass(target: i32, msg: i32);
    pub fn mpic_unmask_irq(d: *mut irq_data);
    pub fn mpic_mask_irq(d: *mut irq_data);
    pub fn mpic_end_irq(d: *mut irq_data);
    pub fn mpic_get_one_irq(mpic: *mut Mpic) -> u32;
    pub fn mpic_get_irq() -> u32;
    pub fn mpic_get_coreint_irq() -> u32;
    pub fn mpic_get_mcirq() -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
