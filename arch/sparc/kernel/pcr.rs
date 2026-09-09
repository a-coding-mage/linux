// SPDX-License-Identifier: GPL-2.0-only
/* pcr.c: Generic sparc64 performance counter infrastructure.
 *
 * Copyright (C) 2009 David S. Miller (davem@davemloft.net)
 */

/* The declarations below are supplied by the surrounding kernel sources. */
use core::arch::asm;

#[repr(C)]
pub struct pt_regs { _private: [u8; 0] }

#[repr(C)]
pub struct pcr_ops {
    pub read_pcr: Option<unsafe extern "C" fn(usize) -> u64>,
    pub write_pcr: Option<unsafe extern "C" fn(usize, u64)>,
    pub read_pic: Option<unsafe extern "C" fn(usize) -> u64>,
    pub write_pic: Option<unsafe extern "C" fn(usize, u64)>,
    pub nmi_picl_value: Option<unsafe extern "C" fn(u32) -> u64>,
    pub pcr_nmi_enable: u64,
    pub pcr_nmi_disable: u64,
}

unsafe extern "C" {
    static mut pcr_ops: *const pcr_ops;
    static tlb_type: i32;
    static sun4v_chip_type: i32;
    fn clear_softint(value: u64);
    fn set_irq_regs(regs: *mut pt_regs) -> *mut pt_regs;
    fn irq_enter();
    fn irq_exit();
    fn irq_work_run();
    fn set_softint(value: u64);
    fn local_cpu_data() -> CpuData;
    fn sun4v_niagara2_setperf(group: u64, value: u64) -> usize;
    fn sun4v_vt_get_perfreg(reg_num: usize, value: *mut usize) -> usize;
    fn sun4v_vt_set_perfreg(reg_num: usize, value: u64) -> usize;
    fn sun4v_t5_get_perfreg(reg_num: usize, value: *mut usize) -> usize;
    fn sun4v_t5_set_perfreg(reg_num: usize, value: u64) -> usize;
    fn sun4v_m7_get_perfreg(reg_num: usize, value: *mut usize) -> usize;
    fn sun4v_m7_set_perfreg(reg_num: usize, value: u64) -> usize;
    fn sun4v_hvapi_register(group: usize, major: usize, minor: *mut usize) -> usize;
    fn sun4v_hvapi_unregister(group: usize);
}

#[repr(C)]
struct CpuData { clock_tick: u32 }

unsafe extern "C" { fn warn_on_once(condition: bool); }

#[no_mangle]
pub unsafe extern "C" fn deferred_pcr_work_irq(_irq: i32, regs: *mut pt_regs) {
    clear_softint(1u64 << PIL_DEFERRED_PCR_WORK);
    let old_regs = set_irq_regs(regs);
    irq_enter();
    irq_work_run();
    irq_exit();
    set_irq_regs(old_regs);
}

pub unsafe extern "C" fn arch_irq_work_raise() {
    set_softint(1u64 << PIL_DEFERRED_PCR_WORK);
}

static mut DIRECT_PCR_OPS: pcr_ops = pcr_ops {
    read_pcr: Some(direct_pcr_read), write_pcr: Some(direct_pcr_write),
    read_pic: Some(direct_pic_read), write_pic: Some(direct_pic_write),
    nmi_picl_value: Some(direct_picl_value),
    pcr_nmi_enable: PCR_PIC_PRIV | PCR_STRACE | PCR_UTRACE,
    pcr_nmi_disable: PCR_PIC_PRIV,
};

unsafe extern "C" fn direct_pcr_read(reg_num: usize) -> u64 {
    warn_on_once(reg_num != 0);
    let mut val: u64;
    asm!("rd %pcr, {0}", out(reg) val);
    val
}

unsafe extern "C" fn direct_pcr_write(reg_num: usize, val: u64) {
    warn_on_once(reg_num != 0);
    asm!("wr {0}, 0x0, %pcr", in(reg) val);
}

unsafe extern "C" fn direct_pic_read(reg_num: usize) -> u64 {
    warn_on_once(reg_num != 0);
    let mut val: u64;
    asm!("rd %pic, {0}", out(reg) val);
    val
}

unsafe extern "C" fn direct_pic_write(reg_num: usize, val: u64) {
    warn_on_once(reg_num != 0);
    asm!("ba,pt %xcc, 99f\n nop\n .align 64\n99:wr {0}, 0x0, %pic\n rd %pic, %g0", in(reg) val);
}

unsafe extern "C" fn direct_picl_value(nmi_hz: u32) -> u64 {
    let delta = (*(&local_cpu_data() as *const CpuData)).clock_tick / nmi_hz;
    (0u64.wrapping_sub(delta as u64) & 0xffff_ffff) << 32
}

unsafe extern "C" fn n2_pcr_write(reg_num: usize, val: u64) {
    warn_on_once(reg_num != 0);
    if val & PCR_N2_HTRACE != 0 {
        let ret = sun4v_niagara2_setperf(HV_N2_PERF_SPARC_CTL, val);
        if ret != HV_EOK { direct_pcr_write(reg_num, val); }
    } else { direct_pcr_write(reg_num, val); }
}

unsafe extern "C" fn n2_picl_value(nmi_hz: u32) -> u64 {
    let delta = (*(&local_cpu_data() as *const CpuData)).clock_tick / (nmi_hz << 2);
    (0u64.wrapping_sub(delta as u64) & 0xffff_ffff) << 32
}

unsafe extern "C" fn n4_pcr_read(reg_num: usize) -> u64 { let mut val = 0usize; sun4v_vt_get_perfreg(reg_num, &mut val); val as u64 }
unsafe extern "C" fn n4_pcr_write(reg_num: usize, val: u64) { sun4v_vt_set_perfreg(reg_num, val); }
unsafe extern "C" fn n4_pic_read(reg_num: usize) -> u64 { let mut val: usize; asm!("ldxa [{0}] {2}, {1}", in(reg) reg_num * 0x8, out(reg) val, const ASI_PIC); val as u64 }
unsafe extern "C" fn n4_pic_write(reg_num: usize, val: u64) { asm!("stxa {0}, [{1}] {2}", in(reg) val, in(reg) reg_num * 0x8, const ASI_PIC); }
unsafe extern "C" fn n4_picl_value(nmi_hz: u32) -> u64 { let delta = (*(&local_cpu_data() as *const CpuData)).clock_tick / (nmi_hz << 2); 0u64.wrapping_sub(delta as u64) & 0xffff_ffff }
unsafe extern "C" fn n5_pcr_read(reg_num: usize) -> u64 { let mut val = 0usize; sun4v_t5_get_perfreg(reg_num, &mut val); val as u64 }
unsafe extern "C" fn n5_pcr_write(reg_num: usize, val: u64) { sun4v_t5_set_perfreg(reg_num, val); }
unsafe extern "C" fn m7_pcr_read(reg_num: usize) -> u64 { let mut val = 0usize; sun4v_m7_get_perfreg(reg_num, &mut val); val as u64 }
unsafe extern "C" fn m7_pcr_write(reg_num: usize, val: u64) { sun4v_m7_set_perfreg(reg_num, val); }

static mut N2_PCR_OPS: pcr_ops = pcr_ops { read_pcr: Some(direct_pcr_read), write_pcr: Some(n2_pcr_write), read_pic: Some(direct_pic_read), write_pic: Some(direct_pic_write), nmi_picl_value: Some(n2_picl_value), pcr_nmi_enable: PCR_PIC_PRIV | PCR_STRACE | PCR_UTRACE | PCR_N2_TOE_OV1 | (2 << PCR_N2_SL1_SHIFT) | (0xff << PCR_N2_MASK1_SHIFT), pcr_nmi_disable: PCR_PIC_PRIV };
static mut N4_PCR_OPS: pcr_ops = pcr_ops { read_pcr: Some(n4_pcr_read), write_pcr: Some(n4_pcr_write), read_pic: Some(n4_pic_read), write_pic: Some(n4_pic_write), nmi_picl_value: Some(n4_picl_value), pcr_nmi_enable: PCR_N4_PICNPT | PCR_N4_STRACE | PCR_N4_UTRACE | PCR_N4_TOE | (26 << PCR_N4_SL_SHIFT), pcr_nmi_disable: PCR_N4_PICNPT };
static mut N5_PCR_OPS: pcr_ops = pcr_ops { read_pcr: Some(n5_pcr_read), write_pcr: Some(n5_pcr_write), read_pic: Some(n4_pic_read), write_pic: Some(n4_pic_write), nmi_picl_value: Some(n4_picl_value), pcr_nmi_enable: PCR_N4_PICNPT | PCR_N4_STRACE | PCR_N4_UTRACE | PCR_N4_TOE | (26 << PCR_N4_SL_SHIFT), pcr_nmi_disable: PCR_N4_PICNPT };
static mut M7_PCR_OPS: pcr_ops = pcr_ops { read_pcr: Some(m7_pcr_read), write_pcr: Some(m7_pcr_write), read_pic: Some(n4_pic_read), write_pic: Some(n4_pic_write), nmi_picl_value: Some(n4_picl_value), pcr_nmi_enable: PCR_N4_PICNPT | PCR_N4_STRACE | PCR_N4_UTRACE | PCR_N4_TOE | (26 << PCR_N4_SL_SHIFT), pcr_nmi_disable: PCR_N4_PICNPT };

static mut perf_hsvc_group: usize = 0;
static mut perf_hsvc_major: usize = 0;
static mut perf_hsvc_minor: usize = 0;

unsafe extern "C" fn register_perf_hsvc() -> i32 {
    if tlb_type == hypervisor {
        perf_hsvc_group = match sun4v_chip_type {
            SUN4V_CHIP_NIAGARA1 => HV_GRP_NIAG_PERF, SUN4V_CHIP_NIAGARA2 => HV_GRP_N2_CPU,
            SUN4V_CHIP_NIAGARA3 => HV_GRP_KT_CPU, SUN4V_CHIP_NIAGARA4 => HV_GRP_VT_CPU,
            SUN4V_CHIP_NIAGARA5 => HV_GRP_T5_CPU, SUN4V_CHIP_SPARC_M7 => HV_GRP_M7_PERF,
            _ => return -ENODEV,
        } as usize;
        perf_hsvc_major = 1; perf_hsvc_minor = 0;
        if sun4v_hvapi_register(perf_hsvc_group, perf_hsvc_major, &mut perf_hsvc_minor) != 0 { return -ENODEV; }
    } 0
}
unsafe extern "C" fn unregister_perf_hsvc() { if tlb_type == hypervisor { sun4v_hvapi_unregister(perf_hsvc_group); } }

// Build-time constants and the remaining operation-table wiring are supplied by asm/pcr.h.
pub unsafe extern "C" fn pcr_arch_init() -> i32 {
    let mut err = register_perf_hsvc(); if err != 0 { return err; }
    if tlb_type == hypervisor { pcr_ops = match sun4v_chip_type { SUN4V_CHIP_NIAGARA1 | SUN4V_CHIP_NIAGARA2 | SUN4V_CHIP_NIAGARA3 => &N2_PCR_OPS, SUN4V_CHIP_NIAGARA4 => &N4_PCR_OPS, SUN4V_CHIP_NIAGARA5 => &N5_PCR_OPS, SUN4V_CHIP_SPARC_M7 => &M7_PCR_OPS, _ => { unregister_perf_hsvc(); return -ENODEV; } }; }
    else if tlb_type == cheetah || tlb_type == cheetah_plus { pcr_ops = &DIRECT_PCR_OPS; }
    else { unregister_perf_hsvc(); err = -ENODEV; return err; }
    nmi_init()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
