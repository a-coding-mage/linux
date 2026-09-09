/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from powerpc/include/asm/eeh.h. */

/* Kernel-only header; included dependencies and configuration symbols are supplied externally. */

#[cfg(feature = "CONFIG_EEH")]
pub const EEH_ENABLED: i32 = 0x01;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_FORCE_DISABLED: i32 = 0x02;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PROBE_MODE_DEV: i32 = 0x04;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PROBE_MODE_DEVTREE: i32 = 0x08;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_ENABLE_IO_FOR_LOG: i32 = 0x20;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_EARLY_DUMP_LOG: i32 = 0x40;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_RST_HOLD_TIME: i32 = 250;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_RST_SETTLE_TIME: i32 = 1800;

#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_INVALID: i32 = 1 << 0;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_PHB: i32 = 1 << 1;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_DEVICE: i32 = 1 << 2;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_BUS: i32 = 1 << 3;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_VF: i32 = 1 << 4;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_ISOLATED: i32 = 1 << 0;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_RECOVERING: i32 = 1 << 1;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_CFG_BLOCKED: i32 = 1 << 2;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_RESET: i32 = 1 << 3;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_KEEP: i32 = 1 << 8;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_CFG_RESTRICTED: i32 = 1 << 9;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_REMOVED: i32 = 1 << 10;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_PE_PRI_BUS: i32 = 1 << 11;

#[repr(C)]
pub struct eeh_pe {
    pub type_: i32,
    pub state: i32,
    pub addr: i32,
    pub phb: *mut pci_controller,
    pub bus: *mut pci_bus,
    pub check_count: i32,
    pub freeze_count: i32,
    pub tstamp: time64_t,
    pub false_positives: i32,
    pub pass_dev_cnt: atomic_t,
    pub parent: *mut eeh_pe,
    pub data: *mut core::ffi::c_void,
    pub child_list: list_head,
    pub child: list_head,
    pub edevs: list_head,
    #[cfg(feature = "CONFIG_STACKTRACE")]
    pub stack_trace: [c_ulong; 64],
    #[cfg(feature = "CONFIG_STACKTRACE")]
    pub trace_entries: i32,
}

#[cfg(feature = "CONFIG_EEH")]
#[inline]
pub unsafe fn eeh_pe_passed(pe: *mut eeh_pe) -> bool {
    !pe.is_null() && atomic_read(&(*pe).pass_dev_cnt) != 0
}

pub const EEH_DEV_BRIDGE: i32 = 1 << 0;
pub const EEH_DEV_ROOT_PORT: i32 = 1 << 1;
pub const EEH_DEV_DS_PORT: i32 = 1 << 2;
pub const EEH_DEV_IRQ_DISABLED: i32 = 1 << 3;
pub const EEH_DEV_DISCONNECTED: i32 = 1 << 4;
pub const EEH_DEV_NO_HANDLER: i32 = 1 << 8;
pub const EEH_DEV_SYSFS: i32 = 1 << 9;
pub const EEH_DEV_REMOVED: i32 = 1 << 10;

#[repr(C)]
pub struct eeh_dev {
    pub mode: i32,
    pub bdfn: i32,
    pub controller: *mut pci_controller,
    pub pe_config_addr: i32,
    pub config_space: [u32; 16],
    pub pcix_cap: i32,
    pub pcie_cap: i32,
    pub aer_cap: i32,
    pub af_cap: i32,
    pub pe: *mut eeh_pe,
    pub entry: list_head,
    pub rmv_entry: list_head,
    pub pdn: *mut pci_dn,
    pub pdev: *mut pci_dev,
    pub in_error: bool,
    pub physfn: *mut pci_dev,
    pub vf_index: i32,
}

#[cfg(feature = "CONFIG_EEH")]
pub const EEH_NEXT_ERR_NONE: i32 = 0;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_NEXT_ERR_INF: i32 = 1;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_NEXT_ERR_FROZEN_PE: i32 = 2;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_NEXT_ERR_FENCED_PHB: i32 = 3;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_NEXT_ERR_DEAD_PHB: i32 = 4;
#[cfg(feature = "CONFIG_EEH")]
pub const EEH_NEXT_ERR_DEAD_IOC: i32 = 5;

pub const EEH_OPT_DISABLE: i32 = 0;
pub const EEH_OPT_ENABLE: i32 = 1;
pub const EEH_OPT_THAW_MMIO: i32 = 2;
pub const EEH_OPT_THAW_DMA: i32 = 3;
pub const EEH_OPT_FREEZE_PE: i32 = 4;
pub const EEH_STATE_UNAVAILABLE: i32 = 1 << 0;
pub const EEH_STATE_NOT_SUPPORT: i32 = 1 << 1;
pub const EEH_STATE_RESET_ACTIVE: i32 = 1 << 2;
pub const EEH_STATE_MMIO_ACTIVE: i32 = 1 << 3;
pub const EEH_STATE_DMA_ACTIVE: i32 = 1 << 4;
pub const EEH_STATE_MMIO_ENABLED: i32 = 1 << 5;
pub const EEH_STATE_DMA_ENABLED: i32 = 1 << 6;
pub const EEH_RESET_DEACTIVATE: i32 = 0;
pub const EEH_RESET_HOT: i32 = 1;
pub const EEH_RESET_FUNDAMENTAL: i32 = 3;
pub const EEH_LOG_TEMP: i32 = 1;
pub const EEH_LOG_PERM: i32 = 2;

#[repr(C)]
pub struct eeh_ops {
    pub name: *mut c_char,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev) -> *mut eeh_dev>,
    pub set_option: Option<unsafe extern "C" fn(*mut eeh_pe, i32) -> i32>,
    pub get_state: Option<unsafe extern "C" fn(*mut eeh_pe, *mut i32) -> i32>,
    pub reset: Option<unsafe extern "C" fn(*mut eeh_pe, i32) -> i32>,
    pub get_log: Option<unsafe extern "C" fn(*mut eeh_pe, i32, *mut c_char, c_ulong) -> i32>,
    pub configure_bridge: Option<unsafe extern "C" fn(*mut eeh_pe) -> i32>,
    pub err_inject: Option<unsafe extern "C" fn(*mut eeh_pe, i32, i32, c_ulong, c_ulong) -> i32>,
    pub read_config: Option<unsafe extern "C" fn(*mut eeh_dev, i32, i32, *mut u32) -> i32>,
    pub write_config: Option<unsafe extern "C" fn(*mut eeh_dev, i32, i32, u32) -> i32>,
    pub next_error: Option<unsafe extern "C" fn(*mut *mut eeh_pe) -> i32>,
    pub restore_config: Option<unsafe extern "C" fn(*mut eeh_dev) -> i32>,
    pub notify_resume: Option<unsafe extern "C" fn(*mut eeh_dev) -> i32>,
}

extern "C" {
    pub static mut eeh_subsystem_flags: i32;
    pub static mut eeh_max_freezes: u32;
    pub static mut eeh_debugfs_no_recover: bool;
    pub static mut eeh_ops: *mut eeh_ops;
    pub static mut confirm_error_lock: raw_spinlock_t;
}

#[cfg(feature = "CONFIG_EEH")]
#[inline] pub unsafe fn eeh_add_flag(flag: i32) { eeh_subsystem_flags |= flag; }
#[cfg(feature = "CONFIG_EEH")]
#[inline] pub unsafe fn eeh_clear_flag(flag: i32) { eeh_subsystem_flags &= !flag; }
#[cfg(feature = "CONFIG_EEH")]
#[inline] pub unsafe fn eeh_has_flag(flag: i32) -> bool { (eeh_subsystem_flags & flag) != 0 }
#[inline] pub unsafe fn eeh_enabled() -> bool {
    #[cfg(feature = "CONFIG_EEH")] { eeh_has_flag(EEH_ENABLED) && !eeh_has_flag(EEH_FORCE_DISABLED) }
    #[cfg(not(feature = "CONFIG_EEH"))] { false }
}
#[cfg(feature = "CONFIG_EEH")]
#[inline] pub unsafe fn eeh_state_active(state: i32) -> bool {
    (state & (EEH_STATE_MMIO_ACTIVE | EEH_STATE_DMA_ACTIVE)) == (EEH_STATE_MMIO_ACTIVE | EEH_STATE_DMA_ACTIVE)
}

pub type eeh_edev_traverse_func = Option<unsafe extern "C" fn(*mut eeh_dev, *mut c_void)>;
pub type eeh_pe_traverse_func = Option<unsafe extern "C" fn(*mut eeh_pe, *mut c_void) -> *mut c_void>;

#[inline] pub unsafe fn eeh_dev_to_pdn(edev: *mut eeh_dev) -> *mut pci_dn { if edev.is_null() { core::ptr::null_mut() } else { (*edev).pdn } }
#[inline] pub unsafe fn eeh_dev_to_pci_dev(edev: *mut eeh_dev) -> *mut pci_dev { if edev.is_null() { core::ptr::null_mut() } else { (*edev).pdev } }
#[inline] pub unsafe fn eeh_dev_to_pe(edev: *mut eeh_dev) -> *mut eeh_pe { if edev.is_null() { core::ptr::null_mut() } else { (*edev).pe } }

extern "C" {
    pub fn eeh_set_pe_aux_size(size: i32);
    pub fn eeh_phb_pe_create(phb: *mut pci_controller) -> i32;
    pub fn eeh_wait_state(pe: *mut eeh_pe, max_wait: i32) -> i32;
    pub fn eeh_phb_pe_get(phb: *mut pci_controller) -> *mut eeh_pe;
    pub fn eeh_pe_next(pe: *mut eeh_pe, root: *mut eeh_pe) -> *mut eeh_pe;
    pub fn eeh_pe_get(phb: *mut pci_controller, pe_no: i32) -> *mut eeh_pe;
    pub fn eeh_pe_tree_insert(edev: *mut eeh_dev, parent: *mut eeh_pe) -> i32;
    pub fn eeh_pe_tree_remove(edev: *mut eeh_dev) -> i32;
    pub fn eeh_pe_update_time_stamp(pe: *mut eeh_pe);
    pub fn eeh_pe_traverse(root: *mut eeh_pe, f: eeh_pe_traverse_func, flag: *mut c_void) -> *mut c_void;
    pub fn eeh_pe_dev_traverse(root: *mut eeh_pe, f: eeh_edev_traverse_func, flag: *mut c_void);
    pub fn eeh_pe_restore_bars(pe: *mut eeh_pe);
    pub fn eeh_pe_loc_get(pe: *mut eeh_pe) -> *const c_char;
    pub fn eeh_pe_bus_get(pe: *mut eeh_pe) -> *mut pci_bus;
    pub fn eeh_pe_loc_get_bus(bus: *mut pci_bus) -> *const c_char;
    pub fn eeh_pe_bus_get_nolock(pe: *mut eeh_pe) -> *mut pci_bus;
    pub fn eeh_show_enabled();
    pub fn eeh_init(ops: *mut eeh_ops) -> i32;
    pub fn eeh_check_failure(token: *const c_void) -> i32;
    pub fn eeh_dev_check_failure(edev: *mut eeh_dev) -> i32;
    pub fn eeh_addr_cache_init();
    pub fn eeh_probe_device(pdev: *mut pci_dev);
    pub fn eeh_remove_device(pdev: *mut pci_dev);
    pub fn eeh_unfreeze_pe(pe: *mut eeh_pe) -> i32;
    pub fn eeh_pe_reset_and_recover(pe: *mut eeh_pe) -> i32;
    pub fn eeh_dev_open(pdev: *mut pci_dev) -> i32;
    pub fn eeh_dev_release(pdev: *mut pci_dev);
    pub fn eeh_pe_set_option(pe: *mut eeh_pe, option: i32) -> i32;
    pub fn eeh_pe_get_state(pe: *mut eeh_pe) -> i32;
    pub fn eeh_pe_reset(pe: *mut eeh_pe, option: i32, include_passed: bool) -> i32;
    pub fn eeh_pe_configure(pe: *mut eeh_pe) -> i32;
    pub fn eeh_pe_inject_err(pe: *mut eeh_pe, typ: i32, func: i32, addr: c_ulong, mask: c_ulong) -> i32;
    pub fn eeh_pe_inject_mmio_error(pdev: *mut pci_dev) -> i32;
}

#[inline]
pub unsafe fn EEH_POSSIBLE_ERROR<T: PartialEq + From<u8>>(val: T, typ: T) -> bool { val == typ && eeh_enabled() }
pub const fn EEH_IO_ERROR_VALUE(size: usize) -> u32 { !0u32 >> ((4 - size) * 8) }

#[cfg(not(feature = "CONFIG_EEH"))]
#[inline] pub unsafe fn eeh_check_failure(_token: *const c_void) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_EEH"))]
#[inline] pub unsafe fn eeh_phb_pe_create(_phb: *mut pci_controller) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_EEH"))]
#[inline] pub unsafe fn eeh_show_enabled() {}
#[cfg(not(feature = "CONFIG_EEH"))]
#[inline] pub unsafe fn eeh_addr_cache_init() {}
#[cfg(not(feature = "CONFIG_EEH"))]
#[inline] pub unsafe fn eeh_probe_device(_dev: *mut pci_dev) {}
#[cfg(not(feature = "CONFIG_EEH"))]
#[inline] pub unsafe fn eeh_remove_device(_dev: *mut pci_dev) {}
#[cfg(not(feature = "CONFIG_EEH"))]
#[inline] pub unsafe fn eeh_dev_check_failure(_dev: *mut eeh_dev) -> i32 { 0 }

#[cfg(feature = "CONFIG_PPC64")]
extern "C" {
    fn in_8(addr: *const c_void) -> u8;
    fn in_le16(addr: *const c_void) -> u16;
    fn in_le32(addr: *const c_void) -> u32;
    fn in_le64(addr: *const c_void) -> u64;
    fn in_be16(addr: *const c_void) -> u16;
    fn in_be32(addr: *const c_void) -> u32;
    fn in_be64(addr: *const c_void) -> u64;
}
#[cfg(feature = "CONFIG_PPC64")]
#[inline] pub unsafe fn eeh_readb(addr: *const c_void) -> u8 { let v = in_8(addr); if v == u8::MAX && eeh_enabled() { eeh_check_failure(addr); } v }
#[cfg(feature = "CONFIG_PPC64")]
#[inline] pub unsafe fn eeh_readw(addr: *const c_void) -> u16 { let v = in_le16(addr); if v == u16::MAX && eeh_enabled() { eeh_check_failure(addr); } v }
#[cfg(feature = "CONFIG_PPC64")]
#[inline] pub unsafe fn eeh_readl(addr: *const c_void) -> u32 { let v = in_le32(addr); if v == u32::MAX && eeh_enabled() { eeh_check_failure(addr); } v }
#[cfg(feature = "CONFIG_PPC64")]
#[inline] pub unsafe fn eeh_readq(addr: *const c_void) -> u64 { let v = in_le64(addr); if v == u64::MAX && eeh_enabled() { eeh_check_failure(addr); } v }
#[cfg(feature = "CONFIG_PPC64")]
#[inline] pub unsafe fn eeh_readw_be(addr: *const c_void) -> u16 { let v = in_be16(addr); if v == u16::MAX && eeh_enabled() { eeh_check_failure(addr); } v }
#[cfg(feature = "CONFIG_PPC64")]
#[inline] pub unsafe fn eeh_readl_be(addr: *const c_void) -> u32 { let v = in_be32(addr); if v == u32::MAX && eeh_enabled() { eeh_check_failure(addr); } v }
#[cfg(feature = "CONFIG_PPC64")]
#[inline] pub unsafe fn eeh_readq_be(addr: *const c_void) -> u64 { let v = in_be64(addr); if v == u64::MAX && eeh_enabled() { eeh_check_failure(addr); } v }

#[cfg(feature = "CONFIG_PPC_PSERIES")]
#[cfg(feature = "CONFIG_EEH")]
extern "C" { pub fn pseries_eeh_init_edev_recursive(pdn: *mut pci_dn); }

/* CONFIG_PPC64 MMIO wrappers and CONFIG_STACKTRACE fields retain their source conditions. */

#[repr(C)] pub struct pci_dev;
#[repr(C)] pub struct pci_bus;
#[repr(C)] pub struct pci_dn;
#[repr(C)] pub struct pci_controller;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct atomic_t;
#[repr(C)] pub struct raw_spinlock_t;
pub type time64_t = i64;
pub type c_ulong = usize;
pub type c_void = core::ffi::c_void;
pub type c_char = i8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
