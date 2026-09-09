/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * ipmi_si.h
 *
 * Interface from the device-specific interfaces (OF, DMI, ACPI, PCI,
 * etc) to the base ipmi system interface code.
 */

/* C header dependencies are supplied by the surrounding translation unit. */

pub const SI_DEVICE_NAME: &str = "ipmi_si";

pub const DEFAULT_REGSPACING: u32 = 1;
pub const DEFAULT_REGSIZE: u32 = 1;

/* Numbers in this enumerator should be mapped to si_to_str[]. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum si_type {
    SI_TYPE_INVALID,
    SI_KCS,
    SI_SMIC,
    SI_BT,
    SI_TYPE_MAX,
}

/* Array is defined in the ipmi_si_intf.c. */
extern "C" {
    pub static si_to_str: *const *const core::ffi::c_char;
}

#[repr(C)]
pub struct ipmi_match_info {
    pub r#type: si_type,
}

extern "C" {
    pub static ipmi_kcs_si_info: ipmi_match_info;
    pub static ipmi_smic_si_info: ipmi_match_info;
    pub static ipmi_bt_si_info: ipmi_match_info;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ipmi_addr_space {
    IPMI_IO_ADDR_SPACE,
    IPMI_MEM_ADDR_SPACE,
}

/*
 * The structure for doing I/O in the state machine.  The state
 * machine doesn't have the actual I/O routines, they are done through
 * this interface.
 */
#[repr(C)]
pub struct si_sm_io {
    pub inputb: Option<unsafe extern "C" fn(io: *const si_sm_io, offset: u32) -> u8>,
    pub outputb: Option<unsafe extern "C" fn(io: *const si_sm_io, offset: u32, b: u8)>,

    /*
     * Generic info used by the actual handling routines, the
     * state machine shouldn't touch these.
     */
    pub addr: *mut core::ffi::c_void,
    pub regspacing: u32,
    pub regsize: u32,
    pub regshift: u32,
    pub addr_space: ipmi_addr_space,
    pub addr_data: core::ffi::c_ulong,
    pub addr_source: ipmi_addr_src, /* ACPI, PCI, SMBIOS, hardcode, etc. */
    pub addr_info: ipmi_smi_info_union,

    pub io_setup: Option<unsafe extern "C" fn(info: *mut si_sm_io) -> i32>,
    pub io_cleanup: Option<unsafe extern "C" fn(info: *mut si_sm_io)>,
    pub io_size: u32,

    pub irq: i32,
    pub irq_setup: Option<unsafe extern "C" fn(io: *mut si_sm_io) -> i32>,
    pub irq_handler_data: *mut core::ffi::c_void,
    pub irq_cleanup: Option<unsafe extern "C" fn(io: *mut si_sm_io)>,

    pub slave_addr: u8,
    pub si_info: *const ipmi_match_info,
    pub dev: *mut device,
}

extern "C" {
    pub fn ipmi_si_add_smi(io: *mut si_sm_io) -> i32;
    pub fn ipmi_si_irq_handler(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn ipmi_irq_start_cleanup(io: *mut si_sm_io);
    pub fn ipmi_std_irq_setup(io: *mut si_sm_io) -> i32;
    pub fn ipmi_irq_finish_setup(io: *mut si_sm_io);
    pub fn ipmi_si_remove_by_dev(dev: *mut device);
    pub fn ipmi_si_remove_by_data(
        addr_space: i32,
        si_type: si_type,
        addr: core::ffi::c_ulong,
    ) -> *mut device;
    pub fn ipmi_hardcode_init();
    pub fn ipmi_si_hardcode_exit();
    pub fn ipmi_si_hotmod_exit();
    pub fn ipmi_si_hardcode_match(addr_space: i32, addr: core::ffi::c_ulong) -> i32;
    pub fn ipmi_si_platform_init();
    pub fn ipmi_si_platform_shutdown();
    pub fn ipmi_remove_platform_device_by_name(name: *mut core::ffi::c_char);

    pub static mut ipmi_platform_driver: platform_driver;
}

/* Conditional declarations follow the corresponding C build-time options. */
#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    pub fn ipmi_si_pci_init();
    pub fn ipmi_si_pci_shutdown();
}
#[cfg(not(feature = "CONFIG_PCI"))]
pub unsafe fn ipmi_si_pci_init() {}
#[cfg(not(feature = "CONFIG_PCI"))]
pub unsafe fn ipmi_si_pci_shutdown() {}

#[cfg(feature = "CONFIG_IPMI_LS2K")]
extern "C" {
    pub fn ipmi_si_ls2k_init();
    pub fn ipmi_si_ls2k_shutdown();
}
#[cfg(not(feature = "CONFIG_IPMI_LS2K"))]
pub unsafe fn ipmi_si_ls2k_init() {}
#[cfg(not(feature = "CONFIG_IPMI_LS2K"))]
pub unsafe fn ipmi_si_ls2k_shutdown() {}

#[cfg(feature = "CONFIG_PARISC")]
extern "C" {
    pub fn ipmi_si_parisc_init();
    pub fn ipmi_si_parisc_shutdown();
}
#[cfg(not(feature = "CONFIG_PARISC"))]
pub unsafe fn ipmi_si_parisc_init() {}
#[cfg(not(feature = "CONFIG_PARISC"))]
pub unsafe fn ipmi_si_parisc_shutdown() {}

extern "C" {
    pub fn ipmi_si_port_setup(io: *mut si_sm_io) -> i32;
    pub fn ipmi_si_mem_setup(io: *mut si_sm_io) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
