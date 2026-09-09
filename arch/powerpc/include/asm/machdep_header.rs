/* SPDX-License-Identifier: GPL-2.0-or-later */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* C header dependencies are supplied by the surrounding translation. */

#[repr(C)]
pub struct machdep_calls {
    pub name: *const core::ffi::c_char,
    pub compatible: *const core::ffi::c_char,
    pub compatibles: *const *const core::ffi::c_char,
    #[cfg(all(CONFIG_PPC64, CONFIG_PM))] pub iommu_restore: Option<unsafe extern "C" fn()>,
    #[cfg(all(CONFIG_PPC64, CONFIG_MEMORY_HOTPLUG))] pub memory_block_size: Option<unsafe extern "C" fn() -> usize>,
    pub dma_set_mask: Option<unsafe extern "C" fn(*mut device, u64)>,
    pub probe: Option<unsafe extern "C" fn() -> i32>,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub show_cpuinfo: Option<unsafe extern "C" fn(*mut seq_file)>,
    pub get_proc_freq: Option<unsafe extern "C" fn(u32) -> usize>,
    pub init_IRQ: Option<unsafe extern "C" fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> u32>,
    pub pcibios_fixup: Option<unsafe extern "C" fn()>,
    pub pci_irq_fixup: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub pcibios_root_bridge_prepare: Option<unsafe extern "C" fn(*mut pci_host_bridge) -> i32>,
    pub discover_phbs: Option<unsafe extern "C" fn()>,
    pub pci_setup_phb: Option<unsafe extern "C" fn(*mut pci_controller) -> i32>,
    pub restart: Option<unsafe extern "C" fn(*mut core::ffi::c_char) -> !>,
    pub halt: Option<unsafe extern "C" fn() -> !>,
    pub panic: Option<unsafe extern "C" fn(*mut core::ffi::c_char)>,
    pub time_init: Option<unsafe extern "C" fn() -> i64>,
    pub set_rtc_time: Option<unsafe extern "C" fn(*mut rtc_time) -> i32>,
    pub get_rtc_time: Option<unsafe extern "C" fn(*mut rtc_time)>,
    pub get_boot_time: Option<unsafe extern "C" fn() -> time64_t>,
    pub calibrate_decr: Option<unsafe extern "C" fn()>,
    pub progress: Option<unsafe extern "C" fn(*mut core::ffi::c_char, u16)>,
    pub log_error: Option<unsafe extern "C" fn(*mut core::ffi::c_char, u32, i32)>,
    pub nvram_read_val: Option<unsafe extern "C" fn(i32) -> u8>,
    pub nvram_write_val: Option<unsafe extern "C" fn(i32, u8)>,
    pub nvram_write: Option<unsafe extern "C" fn(*mut core::ffi::c_char, usize, *mut loff_t) -> isize>,
    pub nvram_read: Option<unsafe extern "C" fn(*mut core::ffi::c_char, usize, *mut loff_t) -> isize>,
    pub nvram_size: Option<unsafe extern "C" fn() -> isize>,
    pub nvram_sync: Option<unsafe extern "C" fn()>,
    pub system_reset_exception: Option<unsafe extern "C" fn(*mut pt_regs) -> i32>,
    pub machine_check_exception: Option<unsafe extern "C" fn(*mut pt_regs) -> i32>,
    pub handle_hmi_exception: Option<unsafe extern "C" fn(*mut pt_regs) -> i32>,
    pub hmi_exception_early: Option<unsafe extern "C" fn(*mut pt_regs) -> i32>,
    pub machine_check_early: Option<unsafe extern "C" fn(*mut pt_regs) -> i64>,
    pub mce_check_early_recovery: Option<unsafe extern "C" fn(*mut pt_regs) -> bool>,
    pub machine_check_log_err: Option<unsafe extern "C" fn()>,
    pub feature_call: Option<unsafe extern "C" fn(u32, ...) -> i64>,
    pub pci_get_legacy_ide_irq: Option<unsafe extern "C" fn(*mut pci_dev, i32) -> i32>,
    pub phys_mem_access_prot: Option<unsafe extern "C" fn(usize, usize, pgprot_t) -> pgprot_t>,
    pub power_save: Option<unsafe extern "C" fn()>,
    pub enable_pmcs: Option<unsafe extern "C" fn()>,
    pub set_dabr: Option<unsafe extern "C" fn(usize, usize) -> i32>,
    pub set_dawr: Option<unsafe extern "C" fn(i32, usize, usize) -> i32>,
    #[cfg(CONFIG_PPC32)] pub init: Option<unsafe extern "C" fn()>,
    #[cfg(CONFIG_PPC32)] pub pcibios_after_init: Option<unsafe extern "C" fn()>,
    pub pci_exclude_device: Option<unsafe extern "C" fn(*mut pci_controller, u8, u8) -> i32>,
    pub pcibios_fixup_resources: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub pcibios_fixup_bus: Option<unsafe extern "C" fn(*mut pci_bus)>,
    pub pcibios_fixup_phb: Option<unsafe extern "C" fn(*mut pci_controller)>,
    pub pcibios_bus_add_device: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub pcibios_default_alignment: Option<unsafe extern "C" fn() -> resource_size_t>,
    #[cfg(CONFIG_PCI_IOV)] pub pcibios_fixup_sriov: Option<unsafe extern "C" fn(*mut pci_dev)>,
    #[cfg(CONFIG_PCI_IOV)] pub pcibios_iov_resource_alignment: Option<unsafe extern "C" fn(*const pci_dev, i32) -> resource_size_t>,
    #[cfg(CONFIG_PCI_IOV)] pub pcibios_sriov_enable: Option<unsafe extern "C" fn(*mut pci_dev, u16) -> i32>,
    #[cfg(CONFIG_PCI_IOV)] pub pcibios_sriov_disable: Option<unsafe extern "C" fn(*mut pci_dev) -> i32>,
    pub machine_shutdown: Option<unsafe extern "C" fn()>,
    #[cfg(CONFIG_KEXEC_CORE)] pub kexec_cpu_down: Option<unsafe extern "C" fn(i32, i32)>,
    #[cfg(CONFIG_KEXEC_CORE)] pub machine_kexec: Option<unsafe extern "C" fn(*mut kimage)>,
    #[cfg(CONFIG_SUSPEND)] pub suspend_disable_irqs: Option<unsafe extern "C" fn()>,
    #[cfg(CONFIG_SUSPEND)] pub suspend_enable_irqs: Option<unsafe extern "C" fn()>,
    #[cfg(CONFIG_ARCH_CPU_PROBE_RELEASE)] pub cpu_probe: Option<unsafe extern "C" fn(*const core::ffi::c_char, usize) -> isize>,
    #[cfg(CONFIG_ARCH_CPU_PROBE_RELEASE)] pub cpu_release: Option<unsafe extern "C" fn(*const core::ffi::c_char, usize) -> isize>,
    pub get_random_seed: Option<unsafe extern "C" fn(*mut usize) -> i32>,
}

extern "C" {
    pub fn e500_idle(); pub fn power4_idle(); pub fn ppc6xx_idle();
    pub static mut ppc_md: machdep_calls;
    pub static mut machine_id: *mut machdep_calls;
}

pub type time64_t = i64;
extern "C" { pub fn __machine_is(md: *const machdep_calls) -> bool; }

#[macro_export] macro_rules! machine_is { ($name:ident) => {{ unsafe { $crate::__machine_is(core::ptr::addr_of!($crate::mach_$name)) } }}; }

#[inline] pub unsafe fn log_error(buf: *mut core::ffi::c_char, err_type: u32, fatal: i32) {
    if let Some(f) = ppc_md.log_error { f(buf, err_type, fatal); }
}

/* The initcall macros register generated functions in the kernel's initcall sections. */
// machine_*_initcall declarations retain that intent for the surrounding build.

/* External C types supplied by included headers. */
pub enum pt_regs {} pub enum pci_bus {} pub enum device {} pub enum device_node {}
pub enum iommu_table {} pub enum rtc_time {} pub enum file {} pub enum pci_dev {}
pub enum pci_controller {} pub enum kimage {} pub enum pci_host_bridge {} pub enum seq_file {}
pub type pgprot_t = usize; pub type resource_size_t = usize; pub type loff_t = i64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
