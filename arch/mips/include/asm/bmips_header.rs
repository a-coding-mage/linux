/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2011 by Kevin Cernekee (cernekee@gmail.com)
 *
 * Definitions for BMIPS processors
 */

use core::ffi::c_void;

/* NOTE: the CBR register returns a PA, and it can be above 0xff00_0000 */
pub unsafe fn BMIPS_GET_CBR() -> *mut c_void {
    (CKSEG1 | ((read_c0_brcm_cbr() >> 18) << 18)) as *mut c_void
}

pub const BMIPS_RAC_CONFIG: u32 = 0x00000000;
pub const BMIPS_RAC_ADDRESS_RANGE: u32 = 0x00000004;
pub const BMIPS_RAC_CONFIG_1: u32 = 0x00000008;
pub const BMIPS_L2_CONFIG: u32 = 0x0000000c;
pub const BMIPS_LMB_CONTROL: u32 = 0x0000001c;
pub const BMIPS_SYSTEM_BASE: u32 = 0x00000020;
pub const BMIPS_PERF_GLOBAL_CONTROL: u32 = 0x00020000;
pub const BMIPS_PERF_CONTROL_0: u32 = 0x00020004;
pub const BMIPS_PERF_CONTROL_1: u32 = 0x00020008;
pub const BMIPS_PERF_COUNTER_0: u32 = 0x00020010;
pub const BMIPS_PERF_COUNTER_1: u32 = 0x00020014;
pub const BMIPS_PERF_COUNTER_2: u32 = 0x00020018;
pub const BMIPS_PERF_COUNTER_3: u32 = 0x0002001c;
pub const BMIPS_RELO_VECTOR_CONTROL_0: u32 = 0x00030000;
pub const BMIPS_RELO_VECTOR_CONTROL_1: u32 = 0x00038000;

pub const BMIPS_NMI_RESET_VEC: u32 = 0x80000000;
pub const BMIPS_WARM_RESTART_VEC: u32 = 0x80000380;

pub const ZSCM_REG_BASE: u32 = 0x97000000;

/* C: declarations supplied by the surrounding kernel headers. */
extern "C" {
    pub static bmips43xx_smp_ops: plat_smp_ops;
    pub static bmips5000_smp_ops: plat_smp_ops;

    pub fn current_cpu_type() -> i32;
    pub fn register_up_smp_ops() -> i32;
    pub fn register_smp_ops(ops: *const plat_smp_ops);

    pub static mut bmips_reset_nmi_vec: u8;
    pub static mut bmips_reset_nmi_vec_end: u8;
    pub static mut bmips_smp_movevec: u8;
    pub static mut bmips_smp_int_vec: u8;
    pub static mut bmips_smp_int_vec_end: u8;

    pub static mut bmips_cbr_addr: *mut c_void;
    pub static mut bmips_smp_enabled: i32;
    pub static mut bmips_cpu_offset: i32;
    pub static mut bmips_booted_mask: cpumask_t;
    pub static mut bmips_tp1_irqs: usize;

    pub fn bmips_ebase_setup();
    pub fn plat_wired_tlb_setup();
    pub fn bmips_cpu_setup();

    pub fn read_c0_brcm_cbr() -> usize;
    pub fn read_c0_ddatalo() -> usize;
    pub fn write_c0_ddatalo(data: usize);
    pub fn barrier();
    pub fn cache_op(op: i32, address: usize);
    pub fn __sync();
    pub fn _ssnop();
}

/* C: enabled when CONFIG_CPU_BMIPS and CONFIG_SMP are enabled. */
pub unsafe fn register_bmips_smp_ops() -> i32 {
    match current_cpu_type() {
        CPU_BMIPS32 | CPU_BMIPS3300 => register_up_smp_ops(),
        CPU_BMIPS4350 | CPU_BMIPS4380 => {
            register_smp_ops(&bmips43xx_smp_ops);
            0
        }
        CPU_BMIPS5000 => {
            register_smp_ops(&bmips5000_smp_ops);
            0
        }
        _ => -19, // -ENODEV
    }
}

pub unsafe fn bmips_read_zscm_reg(offset: u32) -> usize {
    let ret: usize;
    barrier();
    cache_op(Index_Load_Tag_S, (ZSCM_REG_BASE + offset) as usize);
    __sync();
    _ssnop();
    _ssnop();
    _ssnop();
    _ssnop();
    _ssnop();
    _ssnop();
    _ssnop();
    ret = read_c0_ddatalo();
    _ssnop();
    ret
}

pub unsafe fn bmips_write_zscm_reg(offset: u32, data: usize) {
    write_c0_ddatalo(data);
    _ssnop();
    _ssnop();
    _ssnop();
    cache_op(Index_Store_Tag_S, (ZSCM_REG_BASE + offset) as usize);
    _ssnop();
    _ssnop();
    _ssnop();
    barrier();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
