/* SPDX-License-Identifier: GPL-2.0 */
// C header dependencies are supplied by other translation units.

#[repr(C)]
pub enum SeverityLevel {
    MceNoSeverity,
    MceDeferredSeverity,
    MceKeepSeverity,
    MceSomeSeverity,
    MceAoSeverity,
    MceUcSeverity,
    MceArSeverity,
    McePanicSeverity,
}
pub const MCE_UCNA_SEVERITY: SeverityLevel = SeverityLevel::MceDeferredSeverity;
pub const INITIAL_CHECK_INTERVAL: usize = 5 * 60;

#[repr(C)]
pub struct MceEvtLlist {
    pub llnode: LlistNode,
    pub err: MceHwErr,
}

extern "C" {
    pub static mut x86_mce_decoder_chain: BlockingNotifierHead;
    pub static mut mce_banks_ce_disabled: MceBanksT;
    pub fn mce_gen_pool_process(unused: *mut WorkStruct);
    pub fn mce_gen_pool_empty() -> bool;
    pub fn mce_gen_pool_add(err: *mut MceHwErr) -> bool;
    pub fn mce_gen_pool_init() -> bool;
    pub fn mce_gen_pool_prepare_records() -> *mut LlistNode;
    pub fn mce_severity(a: *mut Mce, regs: *mut PtRegs, msg: *mut *mut i8, is_excp: bool) -> i32;
    pub fn mce_get_debugfs_dir() -> *mut Dentry;
}

// CONFIG_X86_MCE_INTEL declarations; otherwise these functions are empty and return false.
extern "C" {
    pub fn mce_intel_handle_storm(bank: i32, on: bool);
    pub fn cmci_disable_bank(bank: i32);
    pub fn intel_init_cmci();
    pub fn intel_init_lmce();
    pub fn intel_clear_lmce();
    pub fn intel_filter_mce(m: *mut Mce) -> bool;
    pub fn intel_mce_usable_address(m: *mut Mce) -> bool;
    pub fn mce_timer_kick(storm: bool);
}

// CONFIG_X86_MCE_THRESHOLD declarations; otherwise these functions are no-ops and return false/0.
extern "C" {
    pub fn cmci_storm_begin(bank: u32);
    pub fn cmci_storm_end(bank: u32);
    pub fn mce_track_storm(mce: *mut Mce);
    pub fn mce_inherit_storm(bank: u32);
    pub fn mce_get_storm_mode() -> bool;
    pub fn mce_set_storm_mode(storm: bool);
    pub fn mce_get_apei_thr_limit() -> u32;
}

#[repr(C)]
pub struct StormBank {
    pub history: u64,
    pub timestamp: u64,
    pub in_storm_mode: bool,
    pub poll_only: bool,
}
pub const NUM_HISTORY_BITS: usize = core::mem::size_of::<u64>() * 8;
pub const STORM_BEGIN_THRESHOLD: usize = 5;
pub const STORM_END_POLL_THRESHOLD: usize = 29;

#[repr(C)]
pub struct McaStormDesc {
    pub banks: [StormBank; MAX_NR_BANKS],
    pub stormy_bank_count: u8,
    pub poll_mode: bool,
}
extern "C" { pub static mut storm_desc: McaStormDesc; }

// CONFIG_ACPI_APEI declarations; fallback implementations return the C defaults.
extern "C" {
    pub fn apei_write_mce(m: *mut Mce) -> i32;
    pub fn apei_read_mce(m: *mut Mce, record_id: *mut u64) -> isize;
    pub fn apei_check_mce() -> i32;
    pub fn apei_clear_mce(record_id: u64) -> i32;
}

#[inline]
pub unsafe fn mce_cmp(m1: *mut Mce, m2: *mut Mce) -> bool {
    (*m1).bank != (*m2).bank || (*m1).status != (*m2).status ||
        (*m1).addr != (*m2).addr || (*m1).misc != (*m2).misc
}

extern "C" {
    pub static mut dev_attr_trigger: DeviceAttribute;
    pub fn mce_work_trigger();
    pub fn mce_register_injector_chain(nb: *mut NotifierBlock);
    pub fn mce_unregister_injector_chain(nb: *mut NotifierBlock);
}

#[repr(C)]
pub struct McaConfig {
    pub lmce_disabled: u64,
    pub disabled: u64,
    pub ser: u64,
    pub recovery: u64,
    pub bios_cmci_threshold: u64,
    pub initialized: u64,
    pub reserved: u64,
    pub dont_log_ce: bool,
    pub cmci_disabled: bool,
    pub ignore_ce: bool,
    pub print_all: bool,
    pub monarch_timeout: i32,
    pub panic_timeout: i32,
    pub rip_msr: u32,
    pub bootlog: i8,
}
extern "C" { pub static mut mca_cfg: McaConfig; pub static mut mce_num_banks: u32; }

#[repr(C)]
pub struct MceVendorFlags {
    pub overflow_recov: u64,
    pub succor: u64,
    pub smca: u64,
    pub zen_ifu_quirk: u64,
    pub amd_threshold: u64,
    pub p5: u64,
    pub winchip: u64,
    pub snb_ifu_quirk: u64,
    pub skx_repmov_quirk: u64,
    pub reserved_0: u64,
}
extern "C" { pub static mut mce_flags: MceVendorFlags; }

#[repr(C)]
pub struct MceBank {
    pub ctl: u64,
    pub init: u64,
    pub lsb_in_status: u64,
    pub reserved_1: u64,
}
extern "C" { pub static mut mce_banks_array: [MceBank; MAX_NR_BANKS]; }

#[repr(C)]
pub enum McaMsr { McaCtl, McaStatus, McaAddr, McaMisc }

extern "C" {
    pub fn filter_mce(m: *mut Mce) -> bool;
    pub fn mce_prep_record_common(m: *mut Mce);
    pub fn mce_prep_record_per_cpu(cpu: u32, m: *mut Mce);
    pub fn mce_threshold_create_device(cpu: u32);
    pub fn mce_threshold_remove_device(cpu: u32);
    pub fn mce_amd_handle_storm(bank: u32, on: bool);
    pub fn amd_filter_mce(m: *mut Mce) -> bool;
    pub fn amd_mce_usable_address(m: *mut Mce) -> bool;
    pub fn amd_clear_bank(m: *mut Mce);
    pub fn smca_bsp_init();
    pub fn intel_p5_mcheck_init(c: *mut CpuinfoX86);
    pub fn winchip_mcheck_init(c: *mut CpuinfoX86);
    pub fn pentium_machine_check(regs: *mut PtRegs);
    pub fn winchip_machine_check(regs: *mut PtRegs);
    pub fn enable_p5_mce();
    pub fn mce_rdmsrq(msr: u32) -> u64;
    pub fn mce_wrmsrq(msr: u32, v: u64);
    pub fn mca_msr_reg(bank: i32, reg: McaMsr) -> u32;
    pub static mut mc_poll_banks: Option<unsafe extern "C" fn()>;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
