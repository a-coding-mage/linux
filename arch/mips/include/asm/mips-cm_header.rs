/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of asm/mips-cm.h. */

// The following declarations are supplied by the surrounding kernel bindings.
extern "C" {
    pub static mut mips_gcr_base: *mut core::ffi::c_void;
    pub static mut mips_cm_l2sync_base: *mut core::ffi::c_void;
    pub fn mips_cm_phys_base() -> phys_addr_t;
    pub fn mips_cm_l2sync_phys_base() -> phys_addr_t;
    pub static mut mips_cm_is64: i32;
    pub static mut mips_cm_is_l2_hci_broken: bool;
}

pub type phys_addr_t = u64;

#[inline]
pub unsafe fn mips_cm_error_report() {}

#[inline]
pub unsafe fn mips_cm_probe() -> i32 { -19 /* -ENODEV */ }

#[inline]
pub unsafe fn mips_cm_present() -> bool { mips_gcr_base != core::ptr::null_mut() }

#[inline]
pub unsafe fn mips_cm_update_property() {}

#[inline]
pub unsafe fn mips_cm_has_l2sync() -> bool {
    mips_cm_l2sync_base != core::ptr::null_mut()
}

pub const MIPS_CM_GCB_OFS: usize = 0x0000;
pub const MIPS_CM_CLCB_OFS: usize = 0x2000;
pub const MIPS_CM_COCB_OFS: usize = 0x4000;
pub const MIPS_CM_GDB_OFS: usize = 0x6000;
pub const MIPS_CM_GCR_SIZE: usize = 0x8000;
pub const MIPS_CM_L2SYNC_SIZE: usize = 0x1000;

// C accessor macros expand through CPS_ACCESSOR_* supplied by mips-cps.h.
macro_rules! GCR_ACCESSOR_RO { ($sz:tt, $off:expr, $name:ident) => {
    CPS_ACCESSOR_RO!(gcr, $sz, MIPS_CM_GCB_OFS + $off, $name);
    CPS_ACCESSOR_RO!(gcr, $sz, MIPS_CM_COCB_OFS + $off, redir_$name);
} }
macro_rules! GCR_ACCESSOR_RW { ($sz:tt, $off:expr, $name:ident) => {
    CPS_ACCESSOR_RW!(gcr, $sz, MIPS_CM_GCB_OFS + $off, $name);
    CPS_ACCESSOR_RW!(gcr, $sz, MIPS_CM_COCB_OFS + $off, redir_$name);
} }
macro_rules! GCR_CX_ACCESSOR_RO { ($sz:tt, $off:expr, $name:ident) => {
    CPS_ACCESSOR_RO!(gcr, $sz, MIPS_CM_CLCB_OFS + $off, cl_$name);
    CPS_ACCESSOR_RO!(gcr, $sz, MIPS_CM_COCB_OFS + $off, co_$name);
} }
macro_rules! GCR_CX_ACCESSOR_RW { ($sz:tt, $off:expr, $name:ident) => {
    CPS_ACCESSOR_RW!(gcr, $sz, MIPS_CM_CLCB_OFS + $off, cl_$name);
    CPS_ACCESSOR_RW!(gcr, $sz, MIPS_CM_COCB_OFS + $off, co_$name);
} }

pub const fn bit(n: u32) -> u64 { 1u64 << n }
pub const fn genmask(h: u32, l: u32) -> u64 { ((1u64 << (h-l+1)) - 1) << l }

GCR_ACCESSOR_RO!(64, 0x000, config);
pub const CM_GCR_CONFIG_CLUSTER_COH_CAPABLE: u64 = bit(43);
pub const CM_GCR_CONFIG_CLUSTER_ID: u64 = genmask(39,32);
pub const CM_GCR_CONFIG_NUM_CLUSTERS: u64 = genmask(29,23);
pub const CM_GCR_CONFIG_NUMIOCU: u64 = genmask(15,8);
pub const CM_GCR_CONFIG_PCORES: u64 = genmask(7,0);
GCR_ACCESSOR_RW!(64, 0x008, base);
pub const CM_GCR_BASE_GCRBASE: u64 = genmask(47,15);
pub const CM_GCR_BASE_CMDEFTGT: u64 = genmask(1,0);
pub const CM_GCR_BASE_CMDEFTGT_MEM: u64 = 0;
pub const CM_GCR_BASE_CMDEFTGT_RESERVED: u64 = 1;
pub const CM_GCR_BASE_CMDEFTGT_IOCU0: u64 = 2;
pub const CM_GCR_BASE_CMDEFTGT_IOCU1: u64 = 3;
GCR_ACCESSOR_RW!(32, 0x020, access);
pub const CM_GCR_ACCESS_ACCESSEN: u64 = genmask(7,0);
GCR_ACCESSOR_RO!(32, 0x030, rev);
pub const CM_GCR_REV_MAJOR: u64 = genmask(15,8);
pub const CM_GCR_REV_MINOR: u64 = genmask(7,0);
pub const fn CM_ENCODE_REV(major: u64, minor: u64) -> u64 { ((major << 8) & CM_GCR_REV_MAJOR) | (minor & CM_GCR_REV_MINOR) }
pub const CM_REV_CM2: u64 = CM_ENCODE_REV(6,0);
pub const CM_REV_CM2_5: u64 = CM_ENCODE_REV(7,0);
pub const CM_REV_CM3: u64 = CM_ENCODE_REV(8,0);
pub const CM_REV_CM3_5: u64 = CM_ENCODE_REV(9,0);

GCR_ACCESSOR_RW!(32, 0x038, err_control);
pub const CM_GCR_ERR_CONTROL_L2_ECC_EN: u64 = bit(1);
pub const CM_GCR_ERR_CONTROL_L2_ECC_SUPPORT: u64 = bit(0);
GCR_ACCESSOR_RW!(64, 0x040, error_mask);
GCR_ACCESSOR_RW!(64, 0x048, error_cause);
pub const CM_GCR_ERROR_CAUSE_ERRTYPE: u64 = genmask(31,27);
pub const CM3_GCR_ERROR_CAUSE_ERRTYPE: u64 = genmask(63,58);
pub const CM_GCR_ERROR_CAUSE_ERRINFO: u64 = genmask(26,0);
GCR_ACCESSOR_RW!(64, 0x050, error_addr);
GCR_ACCESSOR_RW!(64, 0x058, error_mult);
pub const CM_GCR_ERROR_MULT_ERR2ND: u64 = genmask(4,0);
GCR_ACCESSOR_RW!(64, 0x070, l2_only_sync_base);
pub const CM_GCR_L2_ONLY_SYNC_BASE_SYNCBASE: u64 = genmask(31,12);
pub const CM_GCR_L2_ONLY_SYNC_BASE_SYNCEN: u64 = bit(0);
GCR_ACCESSOR_RW!(64, 0x080, gic_base);
pub const CM_GCR_GIC_BASE_GICBASE: u64 = genmask(31,17);
pub const CM_GCR_GIC_BASE_GICEN: u64 = bit(0);
GCR_ACCESSOR_RW!(64, 0x088, cpc_base);
pub const CM_GCR_CPC_BASE_CPCBASE: u64 = genmask(31,15);
pub const CM_GCR_CPC_BASE_CPCEN: u64 = bit(0);

GCR_ACCESSOR_RW!(64, 0x090, reg0_base);
GCR_ACCESSOR_RW!(64, 0x0a0, reg1_base);
GCR_ACCESSOR_RW!(64, 0x0b0, reg2_base);
GCR_ACCESSOR_RW!(64, 0x0c0, reg3_base);
pub const CM_GCR_REGn_BASE_BASEADDR: u64 = genmask(31,16);
GCR_ACCESSOR_RW!(64, 0x098, reg0_mask);
GCR_ACCESSOR_RW!(64, 0x0a8, reg1_mask);
GCR_ACCESSOR_RW!(64, 0x0b8, reg2_mask);
GCR_ACCESSOR_RW!(64, 0x0c8, reg3_mask);
pub const CM_GCR_REGn_MASK_ADDRMASK: u64 = genmask(31,16);
pub const CM_GCR_REGn_MASK_CCAOVR: u64 = genmask(7,5);
pub const CM_GCR_REGn_MASK_CCAOVREN: u64 = bit(4);
pub const CM_GCR_REGn_MASK_DROPL2: u64 = bit(2);
pub const CM_GCR_REGn_MASK_CMTGT: u64 = genmask(1,0);
pub const CM_GCR_REGn_MASK_CMTGT_DISABLED: u64 = 0;
pub const CM_GCR_REGn_MASK_CMTGT_MEM: u64 = 1;
pub const CM_GCR_REGn_MASK_CMTGT_IOCU0: u64 = 2;
pub const CM_GCR_REGn_MASK_CMTGT_IOCU1: u64 = 3;

GCR_ACCESSOR_RO!(32, 0x0d0, gic_status);
pub const CM_GCR_GIC_STATUS_EX: u64 = bit(0);
GCR_ACCESSOR_RO!(32, 0x0f0, cpc_status);
pub const CM_GCR_CPC_STATUS_EX: u64 = bit(0);
GCR_ACCESSOR_RW!(32, 0x120, access_cm3);
GCR_ACCESSOR_RW!(32, 0x130, l2_config);
pub const CM_GCR_L2_CONFIG_BYPASS: u64 = bit(20);
pub const CM_GCR_L2_CONFIG_SET_SIZE: u64 = genmask(15,12);
pub const CM_GCR_L2_CONFIG_LINE_SIZE: u64 = genmask(11,8);
pub const CM_GCR_L2_CONFIG_ASSOC: u64 = genmask(7,0);
GCR_ACCESSOR_RO!(32, 0x150, sys_config2);
pub const CM_GCR_SYS_CONFIG2_MAXVPW: u64 = genmask(3,0);
GCR_ACCESSOR_RW!(64, 0x240, l2_ram_config);
pub const CM_GCR_L2_RAM_CONFIG_PRESENT: u64 = bit(31);
pub const CM_GCR_L2_RAM_CONFIG_HCI_DONE: u64 = bit(30);
pub const CM_GCR_L2_RAM_CONFIG_HCI_SUPPORTED: u64 = bit(29);
GCR_ACCESSOR_RW!(32, 0x300, l2_pft_control);
pub const CM_GCR_L2_PFT_CONTROL_PAGEMASK: u64 = genmask(31,12);
pub const CM_GCR_L2_PFT_CONTROL_PFTEN: u64 = bit(8);
pub const CM_GCR_L2_PFT_CONTROL_NPFT: u64 = genmask(7,0);
GCR_ACCESSOR_RW!(32, 0x308, l2_pft_control_b);
pub const CM_GCR_L2_PFT_CONTROL_B_CEN: u64 = bit(8);
pub const CM_GCR_L2_PFT_CONTROL_B_PORTID: u64 = genmask(7,0);
GCR_ACCESSOR_RW!(64, 0x600, l2_tag_addr);
GCR_ACCESSOR_RW!(64, 0x608, l2_tag_state);
GCR_ACCESSOR_RW!(64, 0x610, l2_data);
GCR_ACCESSOR_RW!(64, 0x618, l2_ecc);
GCR_ACCESSOR_RW!(32, 0x620, l2sm_cop);
pub const CM_GCR_L2SM_COP_PRESENT: u64 = bit(31);
pub const CM_GCR_L2SM_COP_RESULT: u64 = genmask(8,6);
pub const CM_GCR_L2SM_COP_RESULT_DONTCARE: u64 = 0;
pub const CM_GCR_L2SM_COP_RESULT_DONE_OK: u64 = 1;
pub const CM_GCR_L2SM_COP_RESULT_DONE_ERROR: u64 = 2;
pub const CM_GCR_L2SM_COP_RESULT_ABORT_OK: u64 = 3;
pub const CM_GCR_L2SM_COP_RESULT_ABORT_ERROR: u64 = 4;
pub const CM_GCR_L2SM_COP_RUNNING: u64 = bit(5);
pub const CM_GCR_L2SM_COP_TYPE: u64 = genmask(4,2);
pub const CM_GCR_L2SM_COP_TYPE_IDX_WBINV: u64 = 0;
pub const CM_GCR_L2SM_COP_TYPE_IDX_STORETAG: u64 = 1;
pub const CM_GCR_L2SM_COP_TYPE_IDX_STORETAGDATA: u64 = 2;
pub const CM_GCR_L2SM_COP_TYPE_HIT_INV: u64 = 4;
pub const CM_GCR_L2SM_COP_TYPE_HIT_WBINV: u64 = 5;
pub const CM_GCR_L2SM_COP_TYPE_HIT_WB: u64 = 6;
pub const CM_GCR_L2SM_COP_TYPE_FETCHLOCK: u64 = 7;
pub const CM_GCR_L2SM_COP_CMD: u64 = genmask(1,0);
pub const CM_GCR_L2SM_COP_CMD_START: u64 = 1;
pub const CM_GCR_L2SM_COP_CMD_ABORT: u64 = 3;
GCR_ACCESSOR_RW!(64, 0x628, l2sm_tag_addr_cop);
pub const CM_GCR_L2SM_TAG_ADDR_COP_NUM_LINES: u64 = genmask(63,48);
pub const CM_GCR_L2SM_TAG_ADDR_COP_START_TAG: u64 = genmask(47,6);
GCR_ACCESSOR_RW!(64, 0x680, bev_base);
GCR_CX_ACCESSOR_RW!(32, 0x000, reset_release);
GCR_CX_ACCESSOR_RW!(32, 0x008, coherence);
pub const CM_GCR_Cx_COHERENCE_COHDOMAINEN: u64 = genmask(7,0);
pub const CM3_GCR_Cx_COHERENCE_COHEN: u64 = bit(0);
GCR_CX_ACCESSOR_RO!(32, 0x010, config);
pub const CM_GCR_Cx_CONFIG_IOCUTYPE: u64 = genmask(11,10);
pub const CM_GCR_Cx_CONFIG_PVPE: u64 = genmask(9,0);
GCR_CX_ACCESSOR_RW!(32, 0x018, other);
pub const CM_GCR_Cx_OTHER_CORENUM: u64 = genmask(31,16);
pub const CM_GCR_Cx_OTHER_CLUSTER_EN: u64 = bit(31);
pub const CM_GCR_Cx_OTHER_GIC_EN: u64 = bit(30);
pub const CM_GCR_Cx_OTHER_BLOCK: u64 = genmask(25,24);
pub const CM_GCR_Cx_OTHER_BLOCK_LOCAL: u64 = 0;
pub const CM_GCR_Cx_OTHER_BLOCK_GLOBAL: u64 = 1;
pub const CM_GCR_Cx_OTHER_BLOCK_USER: u64 = 2;
pub const CM_GCR_Cx_OTHER_BLOCK_GLOBAL_HIGH: u64 = 3;
pub const CM_GCR_Cx_OTHER_CLUSTER: u64 = genmask(21,16);
pub const CM3_GCR_Cx_OTHER_CORE: u64 = genmask(13,8);
pub const CM_GCR_Cx_OTHER_CORE_CM: u64 = 32;
pub const CM3_GCR_Cx_OTHER_VP: u64 = genmask(2,0);
GCR_CX_ACCESSOR_RW!(32, 0x020, reset_base);
GCR_CX_ACCESSOR_RW!(64, 0x020, reset64_base);
pub const CM_GCR_Cx_RESET_BASE_BEVEXCBASE: u64 = genmask(31,12);
pub const CM_GCR_Cx_RESET64_BASE_BEVEXCBASE: u64 = genmask(47,12);
pub const CM_GCR_Cx_RESET_BASE_MODE: u64 = bit(1);
GCR_CX_ACCESSOR_RO!(32, 0x028, id);
pub const CM_GCR_Cx_ID_CLUSTER: u64 = genmask(15,8);
pub const CM_GCR_Cx_ID_CORE: u64 = genmask(7,0);
GCR_CX_ACCESSOR_RW!(32, 0x030, reset_ext_base);
pub const CM_GCR_Cx_RESET_EXT_BASE_EVARESET: u64 = bit(31);
pub const CM_GCR_Cx_RESET_EXT_BASE_UEB: u64 = bit(30);
pub const CM_GCR_Cx_RESET_EXT_BASE_BEVEXCMASK: u64 = genmask(27,20);
pub const CM_GCR_Cx_RESET_EXT_BASE_BEVEXCPA: u64 = genmask(7,1);
pub const CM_GCR_Cx_RESET_EXT_BASE_PRESENT: u64 = bit(0);

extern "C" {
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn read_gcr_rev() -> i32;
    fn read_gcr_sys_config2() -> u32;
    fn read_gcr_cl_config() -> u32;
    fn mips_cm_lock_other(cluster: u32, core: u32, vp: u32, block: u32);
    fn mips_cm_unlock_other();
}

#[inline]
pub unsafe fn mips_cm_l2sync() -> i32 {
    if !mips_cm_has_l2sync() { return -19; }
    writel(0, mips_cm_l2sync_base);
    0
}

#[inline]
pub unsafe fn mips_cm_revision() -> i32 {
    if !mips_cm_present() { 0 } else { read_gcr_rev() }
}

#[inline]
pub unsafe fn mips_cm_max_vp_width() -> u32 {
    if mips_cm_revision() as u64 >= CM_REV_CM3 { return ((read_gcr_sys_config2() as u64 & CM_GCR_SYS_CONFIG2_MAXVPW) >> 0) as u32; }
    if mips_cm_present() { return ((read_gcr_cl_config() as u64 & CM_GCR_Cx_CONFIG_PVPE) + 1) as u32; }
    1
}

#[inline]
pub unsafe fn mips_cm_lock_other_cpu(cpu: u32, block: u32) {
    // cpu_data, cpu_cluster, cpu_core and cpu_vpe_id are supplied by the kernel.
    mips_cm_lock_other(cpu_cluster(cpu), cpu_core(cpu), cpu_vpe_id(cpu), block);
}

extern "C" {
    fn cpu_cluster(cpu: u32) -> u32;
    fn cpu_core(cpu: u32) -> u32;
    fn cpu_vpe_id(cpu: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
