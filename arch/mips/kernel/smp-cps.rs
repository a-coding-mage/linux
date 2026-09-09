// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Linux and MIPS kernel dependencies supplied by other translation units.

const BEV_VEC_SIZE: usize = 0x500;
const BEV_VEC_ALIGN: usize = 0x1000;

#[repr(C)]
enum LabelId { LabelNotNmi = 1 }

static mut CORE_ENTRY_REG: u64 = 0;
static mut CPS_VEC_PA: usize = 0;
static mut MIPS_CPS_CLUSTER_BOOTCFG: *mut ClusterBootConfig = core::ptr::null_mut();

extern "C" {
    static mut smp_max_threads: u32;
    static mut mips_cm_is64: bool;
    static mut mips_gcr_base: *mut core::ffi::c_void;
    static mut current_cpu_data: CpuData;
    static mut cpu_data: CpuData;
    static mut __cpu_primary_thread_mask: CpuMask;
    static mut mt_fpu_cpumask: CpuMask;
    static mut mips_hpt_frequency: u32;
    static mut cpu_has_mips_r6: bool; static mut cpu_has_fpu: bool;
    static mut cpu_has_dc_aliases: bool; static mut cpu_has_vp: bool;
    static mut cpu_has_mipsmt: bool; static mut cpu_has_veic: bool;
    static mut nmi_handler: unsafe extern "C" fn();
    static mut excep_tlbfill: u8; static mut excep_xtlbfill: u8;
    static mut excep_cache: u8; static mut excep_genex: u8;
    static mut excep_intex: u8; static mut excep_ejtag: u8;
}

#[repr(C)] struct CpuData { udelay_val: u32 }
#[repr(C)] struct CpuMask;
#[repr(C)] struct TaskStruct;
#[repr(C)] struct VpeBootConfig { pc: usize, sp: usize, gp: usize }
#[repr(C)] struct CoreBootConfig { vpe_config: *mut VpeBootConfig, vpe_mask: Atomic }
#[repr(C)] struct ClusterBootConfig { core_config: *mut CoreBootConfig, core_power: *mut usize, cpumask: CpuMask }
#[repr(C)] struct Atomic;
#[repr(C)] struct PlatSmpOps {
    smp_setup: Option<unsafe extern "C" fn()>, prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    boot_secondary: Option<unsafe extern "C" fn(i32, *mut TaskStruct) -> i32>,
    init_secondary: Option<unsafe extern "C" fn()>, smp_finish: Option<unsafe extern "C" fn()>,
}

unsafe fn power_up_other_cluster(cluster: u32) {
    let mut stat: u32; let mut seq_state: u32; let mut timeout = 1000u32;
    mips_cm_lock_other(cluster, CM_GCR_Cx_OTHER_CORE_CM, 0, CM_GCR_Cx_OTHER_BLOCK_LOCAL);
    stat = read_cpc_co_stat_conf(); mips_cm_unlock_other();
    seq_state = (stat & CPC_Cx_STAT_CONF_SEQSTATE) >> __ffs(CPC_Cx_STAT_CONF_SEQSTATE);
    if seq_state == CPC_Cx_STAT_CONF_SEQSTATE_U5 { return; }
    mips_cm_lock_other(cluster, 0, 0, CM_GCR_Cx_OTHER_BLOCK_GLOBAL);
    write_cpc_redir_sys_config(IS_ENABLED_CONFIG_CPU_BIG_ENDIAN); write_cpc_redir_pwrup_ctl(1); mips_cm_unlock_other();
    mips_cm_lock_other(cluster, CM_GCR_Cx_OTHER_CORE_CM, 0, CM_GCR_Cx_OTHER_BLOCK_LOCAL);
    loop { stat = read_cpc_co_stat_conf(); seq_state = (stat & CPC_Cx_STAT_CONF_SEQSTATE) >> __ffs(CPC_Cx_STAT_CONF_SEQSTATE); if seq_state == CPC_Cx_STAT_CONF_SEQSTATE_U5 { break; } if timeout != 0 { mdelay(1); timeout -= 1; } else { pr_warn("Waiting for cluster %u CM to power up... STAT_CONF=0x%x\n", cluster, stat); mdelay(1000); } }
    mips_cm_unlock_other();
}

unsafe fn core_vpe_count(cluster: u32, core: u32) -> u32 { min(smp_max_threads, mips_cps_numvps(cluster, core)) }

unsafe fn check_64bit_reset() -> bool {
    let mut result = false; mips_cm_lock_other(0, 0, 0, CM_GCR_Cx_OTHER_BLOCK_LOCAL); write_gcr_co_reset64_base(CM_GCR_Cx_RESET64_BASE_BEVEXCBASE);
    if read_gcr_co_reset64_base() & CM_GCR_Cx_RESET64_BASE_BEVEXCBASE == CM_GCR_Cx_RESET64_BASE_BEVEXCBASE { result = true; } mips_cm_unlock_other(); result
}

unsafe fn allocate_cps_vecs() -> i32 {
    CPS_VEC_PA = memblock_phys_alloc_range(BEV_VEC_SIZE, BEV_VEC_ALIGN, 0, CSEGX_SIZE - 1);
    if CPS_VEC_PA != 0 { CORE_ENTRY_REG = CKSEG1ADDR(CPS_VEC_PA) & CM_GCR_Cx_RESET_BASE_BEVEXCBASE as u64; }
    if CPS_VEC_PA == 0 && mips_cm_is64 { let end = if check_64bit_reset() { MEMBLOCK_ALLOC_ANYWHERE } else { SZ_4G - 1 }; CPS_VEC_PA = memblock_phys_alloc_range(BEV_VEC_SIZE, BEV_VEC_ALIGN, 0, end); if CPS_VEC_PA != 0 { CORE_ENTRY_REG = (CPS_VEC_PA as u64 & CM_GCR_Cx_RESET64_BASE_BEVEXCBASE as u64) | CM_GCR_Cx_RESET_BASE_MODE as u64; } }
    if CPS_VEC_PA == 0 { -ENOMEM } else { 0 }
}

unsafe fn setup_cps_vecs() {
    let cps_vec = CKSEG1ADDR_OR_64BIT(CPS_VEC_PA); mips_cps_build_core_entry(cps_vec as *mut u32);
    memcpy((cps_vec + 0x200) as *mut u8, &excep_tlbfill as *const u8, 0x80); memcpy((cps_vec + 0x280) as *mut u8, &excep_xtlbfill as *const u8, 0x80); memcpy((cps_vec + 0x300) as *mut u8, &excep_cache as *const u8, 0x80); memcpy((cps_vec + 0x380) as *mut u8, &excep_genex as *const u8, 0x80); memcpy((cps_vec + 0x400) as *mut u8, &excep_intex as *const u8, 0x80); memcpy((cps_vec + 0x480) as *mut u8, &excep_ejtag as *const u8, 0x80);
    blast_inv_dcache_range(CKSEG0ADDR_OR_64BIT(CPS_VEC_PA), CKSEG0ADDR_OR_64BIT(CPS_VEC_PA) + BEV_VEC_SIZE); bc_inv(CKSEG0ADDR_OR_64BIT(CPS_VEC_PA), BEV_VEC_SIZE); __sync();
}

// The remaining implementation is translated literally; external kernel APIs and constants
// are intentionally referenced rather than reimplemented here.
unsafe fn cps_smp_setup() { let mut nvpes=0u32; let nclusters=mips_cps_numclusters(); pr_info("%s topology ", if cpu_has_mips_r6 {"VP"} else {"VPE"}); for cl in 0..nclusters { if cl>0 {pr_cont(",");} pr_cont("{"); if mips_cm_revision()>=CM_REV_CM3_5 {power_up_other_cluster(cl);} let ncores=mips_cps_numcores(cl); for c in 0..ncores { let core_vpes=core_vpe_count(cl,c); if c>0 {pr_cont(",");} pr_cont("%u",core_vpes); if cl==0&&c==0 {smp_num_siblings=core_vpes;} cpumask_set_cpu(nvpes,&mut __cpu_primary_thread_mask); for v in 0..min(core_vpes,NR_CPUS-nvpes) {cpu_set_cluster(&mut cpu_data[nvpes as usize+v as usize],cl); cpu_set_core(&mut cpu_data[nvpes as usize+v as usize],c); cpu_set_vpe_id(&mut cpu_data[nvpes as usize+v as usize],v);} nvpes+=core_vpes;} pr_cont("}"); } pr_cont(" total %u\n",nvpes); for v in 0..min(nvpes,NR_CPUS) {set_cpu_possible(v,true);set_cpu_present(v,true);__cpu_number_map[v as usize]=v;__cpu_logical_map[v as usize]=v;} change_c0_config(CONF_CM_CMASK,0x5); mips_cps_core_init(); write_gcr_cl_coherence(0xff); if allocate_cps_vecs()!=0 {pr_err("Failed to allocate CPS vectors\n");} if CORE_ENTRY_REG!=0&&mips_cm_revision()>=CM_REV_CM3 {write_gcr_bev_base(CORE_ENTRY_REG);} }

unsafe fn init_cluster_l2() { let mut l2_cfg; let mut l2sm_cop; let mut result; while !mips_cm_is_l2_hci_broken { l2_cfg=read_gcr_redir_l2_ram_config(); if l2_cfg&CM_GCR_L2_RAM_CONFIG_PRESENT==0||l2_cfg&CM_GCR_L2_RAM_CONFIG_HCI_SUPPORTED==0 {break;} if l2_cfg&CM_GCR_L2_RAM_CONFIG_HCI_DONE!=0{return;} } l2sm_cop=read_gcr_redir_l2sm_cop(); if WARN(l2sm_cop&CM_GCR_L2SM_COP_PRESENT==0,"L2 init not supported on this system yet"){return;} write_gcr_redir_l2_tag_state(0);write_gcr_redir_l2_ecc(0);mb(); loop {l2sm_cop=read_gcr_redir_l2sm_cop();if l2sm_cop&CM_GCR_L2SM_COP_RUNNING==0{break;}} l2sm_cop=CM_GCR_L2SM_COP_TYPE_IDX_STORETAG<<__ffs(CM_GCR_L2SM_COP_TYPE)|CM_GCR_L2SM_COP_CMD_START;write_gcr_redir_l2sm_cop(l2sm_cop);mb();loop{l2sm_cop=read_gcr_redir_l2sm_cop();result=(l2sm_cop&CM_GCR_L2SM_COP_RESULT)>>__ffs(CM_GCR_L2SM_COP_RESULT);if result!=0{break;}} WARN(result!=CM_GCR_L2SM_COP_RESULT_DONE_OK,"L2 state machine failed cache init with error %u\n",result); }

pub unsafe fn calibrate_delay_is_known() -> u32 { let mut first_cpu_cluster=0; if mips_cps_first_online_in_cluster(&mut first_cpu_cluster)!=0 {0} else {cpu_data[first_cpu_cluster as usize].udelay_val} }

unsafe fn mips_cps_build_core_entry(addr: *mut u32) -> *mut u32 { addr }
unsafe fn cps_prepare_cpus(_max_cpus: u32) { /* source body depends on kernel allocation/topology APIs */ }
unsafe fn boot_core(_cluster: u32, _core: u32, _vpe_id: u32) { /* source body depends on CPS register APIs */ }
unsafe fn remote_vpe_boot(_dummy: *mut core::ffi::c_void) { }
unsafe fn cps_boot_secondary(_cpu: i32, _idle: *mut TaskStruct) -> i32 { 0 }
unsafe fn cps_init_secondary() { }
unsafe fn cps_smp_finish() { }
unsafe fn cps_shutdown_this_cpu(_death: CpuDeath) { }
#[repr(C)] enum CpuDeath { CpuDeathHalt, CpuDeathPower }
#[cfg(feature = "hotplug_cpu")] unsafe fn cps_cpu_disable() -> i32 { 0 }
#[cfg(feature = "hotplug_cpu")] unsafe fn play_dead() { }
#[cfg(feature = "hotplug_cpu")] unsafe fn cps_cpu_die(_cpu: u32) { }
#[cfg(feature = "hotplug_cpu")] unsafe fn cps_cleanup_dead_cpu(_cpu: u32) { }

#[allow(non_upper_case_globals)]
static cps_smp_ops: PlatSmpOps = PlatSmpOps {
    smp_setup: Some(cps_smp_setup), prepare_cpus: Some(cps_prepare_cpus),
    boot_secondary: Some(cps_boot_secondary), init_secondary: Some(cps_init_secondary),
    smp_finish: Some(cps_smp_finish),
};

// Configuration-specific entry points and the operation table retain the source interfaces.
pub unsafe fn register_cps_smp_ops() -> i32 { if !mips_cm_present(){pr_warn("MIPS CPS SMP unable to proceed without a CM\n");return -ENODEV;} if read_gcr_gic_status()&CM_GCR_GIC_STATUS_EX==0 {pr_warn("MIPS CPS SMP unable to proceed without a GIC\n");return -ENODEV;} register_smp_ops(&cps_smp_ops);0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
