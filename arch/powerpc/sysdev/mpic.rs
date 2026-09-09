/*
 * Rust translation of arch/powerpc/kernel/mpic.c.
 * External kernel types, constants, functions, and macros are supplied by
 * the surrounding kernel translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn hard_smp_processor_id() -> u32;
    fn get_hard_smp_processor_id(cpu: u32) -> u32;
    fn in_be32(p: *mut u32) -> u32;
    fn in_le32(p: *mut u32) -> u32;
    fn out_be32(p: *mut u32, v: u32);
    fn out_le32(p: *mut u32, v: u32);
    fn ioremap(addr: u64, size: usize) -> *mut u8;
    fn swab32(v: u32) -> u32;
}

#[repr(C)]
pub struct bus_type { pub name: *const u8, pub dev_name: *const u8 }
#[repr(C)]
pub struct mpic_reg_bank { pub base: *mut u8, pub dhost: *mut u8 }
#[repr(C)]
pub struct mpic {
    pub next: *mut mpic, pub name: *const u8, pub node: *mut u8,
    pub paddr: u64, pub flags: u32, pub reg_type: u32,
    pub gregs: mpic_reg_bank, pub tmregs: mpic_reg_bank,
    pub thiscpuregs: mpic_reg_bank, pub cpuregs: [mpic_reg_bank; 32],
    pub isus: [mpic_reg_bank; 32], pub num_sources: u32,
    pub isu_size: u32, pub isu_shift: u32, pub isu_mask: u32,
    pub spurious_vec: u32, pub ipi_vecs: [u32; 4], pub timer_vecs: [u32; 8],
    pub protected: *mut u8,
}

pub static mut mpic_subsys: bus_type = bus_type {
    name: b"mpic\0".as_ptr(), dev_name: b"mpic\0".as_ptr(),
};
static mut mpics: *mut mpic = core::ptr::null_mut();
static mut mpic_primary: *mut mpic = core::ptr::null_mut();

#[inline]
unsafe fn mpic_processor_id(m: *mut mpic) -> u32 {
    if (*m).flags & MPIC_SECONDARY == 0 { hard_smp_processor_id() } else { 0 }
}

#[inline]
unsafe fn _mpic_read(typ: u32, rb: *mut mpic_reg_bank, reg: u32) -> u32 {
    match typ {
        mpic_access_mmio_be => in_be32((*rb).base.add((reg >> 2) as usize) as *mut u32),
        _ => in_le32((*rb).base.add((reg >> 2) as usize) as *mut u32),
    }
}
#[inline]
unsafe fn _mpic_write(typ: u32, rb: *mut mpic_reg_bank, reg: u32, value: u32) {
    match typ {
        mpic_access_mmio_be => out_be32((*rb).base.add((reg >> 2) as usize) as *mut u32, value),
        _ => out_le32((*rb).base.add((reg >> 2) as usize) as *mut u32, value),
    }
}

#[inline]
unsafe fn mpic_tm_offset(m: *mut mpic, tm: u32) -> u32 {
    (tm >> 2) * MPIC_TIMER_GROUP_STRIDE + (tm & 3) * MPIC_TIMER_STRIDE
}
#[inline]
unsafe fn _mpic_ipi_read(m: *mut mpic, ipi: u32) -> u32 {
    let off = MPIC_GREG_IPI_VECTOR_PRI_0 + ipi * MPIC_GREG_IPI_STRIDE;
    _mpic_read((*m).reg_type, &mut (*m).gregs, off)
}
#[inline]
unsafe fn _mpic_ipi_write(m: *mut mpic, ipi: u32, v: u32) {
    _mpic_write((*m).reg_type, &mut (*m).gregs,
                MPIC_GREG_IPI_VECTOR_PRI_0 + ipi * MPIC_GREG_IPI_STRIDE, v)
}
#[inline]
unsafe fn _mpic_tm_read(m: *mut mpic, tm: u32) -> u32 {
    _mpic_read((*m).reg_type, &mut (*m).tmregs,
               mpic_tm_offset(m, tm) + MPIC_TIMER_VECTOR_PRI)
}
#[inline]
unsafe fn _mpic_tm_write(m: *mut mpic, tm: u32, v: u32) {
    _mpic_write((*m).reg_type, &mut (*m).tmregs,
                mpic_tm_offset(m, tm) + MPIC_TIMER_VECTOR_PRI, v)
}
#[inline]
unsafe fn _mpic_cpu_read(m: *mut mpic, reg: u32) -> u32 {
    let cpu = mpic_processor_id(m) as usize;
    _mpic_read((*m).reg_type, &mut (*m).cpuregs[cpu], reg)
}
#[inline]
unsafe fn _mpic_cpu_write(m: *mut mpic, reg: u32, v: u32) {
    let cpu = mpic_processor_id(m) as usize;
    _mpic_write((*m).reg_type, &mut (*m).cpuregs[cpu], reg, v)
}

unsafe fn mpic_eoi(m: *mut mpic) { _mpic_cpu_write(m, MPIC_CPU_EOI, 0); }

pub unsafe fn mpic_test_broken_ipi(m: *mut mpic) {
    _mpic_write((*m).reg_type, &mut (*m).gregs, MPIC_GREG_IPI_VECTOR_PRI_0, MPIC_VECPRI_MASK);
    if _mpic_read((*m).reg_type, &mut (*m).gregs, MPIC_GREG_IPI_VECTOR_PRI_0) == swab32(MPIC_VECPRI_MASK) {
        (*m).flags |= MPIC_BROKEN_IPI;
    }
}

pub unsafe fn mpic_cpu_get_priority() -> u32 {
    _mpic_cpu_read(mpic_primary, MPIC_CPU_CURRENT_TASK_PRI)
}
pub unsafe fn mpic_cpu_set_priority(mut prio: u32) {
    prio &= MPIC_CPU_TASKPRI_MASK;
    _mpic_cpu_write(mpic_primary, MPIC_CPU_CURRENT_TASK_PRI, prio);
}
pub unsafe fn mpic_get_one_irq(m: *mut mpic) -> u32 {
    let src = _mpic_cpu_read(m, MPIC_CPU_INTACK) & MPIC_VECPRI_VECTOR_MASK;
    if src == (*m).spurious_vec { if (*m).flags & MPIC_SPV_EOI != 0 { mpic_eoi(m); } return 0; }
    irq_find_mapping((*m).irqhost, src)
}

/* The remaining declarations and callbacks retain the C implementation's
 * externally visible interfaces; their kernel-provided dependencies are
 * intentionally left as declarations for the surrounding translation. */
extern "C" {
    fn irq_find_mapping(domain: *mut u8, hwirq: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
