// SPDX-License-Identifier: GPL-2.0
// Rust translation of linux/irqdomain.h. C dependencies are supplied externally.

pub const IRQ_DOMAIN_IRQ_SPEC_PARAMS: usize = 16;

#[repr(C)]
pub struct irq_fwspec { pub fwnode: *mut fwnode_handle, pub param_count: core::ffi::c_int, pub param: [u32; IRQ_DOMAIN_IRQ_SPEC_PARAMS] }
#[repr(C)]
pub struct irq_fwspec_info { pub flags: usize, pub affinity: *const cpumask }
pub const IRQ_FWSPEC_INFO_AFFINITY_VALID: u32 = 1 << 0;

#[repr(C)]
pub struct irq_domain_ops {
    pub match_: Option<unsafe extern "C" fn(*mut irq_domain, *mut device_node, irq_domain_bus_token) -> core::ffi::c_int>,
    pub select: Option<unsafe extern "C" fn(*mut irq_domain, *mut irq_fwspec, irq_domain_bus_token) -> core::ffi::c_int>,
    pub map: Option<unsafe extern "C" fn(*mut irq_domain, u32, irq_hw_number_t) -> core::ffi::c_int>,
    pub unmap: Option<unsafe extern "C" fn(*mut irq_domain, u32)>,
    pub xlate: Option<unsafe extern "C" fn(*mut irq_domain, *mut device_node, *const u32, u32, *mut usize, *mut u32) -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub alloc: Option<unsafe extern "C" fn(*mut irq_domain, u32, u32, *mut core::ffi::c_void) -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub free: Option<unsafe extern "C" fn(*mut irq_domain, u32, u32)>,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub activate: Option<unsafe extern "C" fn(*mut irq_domain, *mut irq_data, bool) -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub deactivate: Option<unsafe extern "C" fn(*mut irq_domain, *mut irq_data)>,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub translate: Option<unsafe extern "C" fn(*mut irq_domain, *mut irq_fwspec, *mut usize, *mut u32) -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub get_fwspec_info: Option<unsafe extern "C" fn(*mut irq_fwspec, *mut irq_fwspec_info) -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_GENERIC_IRQ_DEBUGFS")]
    pub debug_show: Option<unsafe extern "C" fn(*mut seq_file, *mut irq_domain, *mut irq_data, core::ffi::c_int)>,
}
extern "C" { pub static irq_generic_chip_ops: irq_domain_ops; }

#[repr(C)]
pub struct irq_domain {
    pub link: list_head, pub name: *const core::ffi::c_char, pub ops: *const irq_domain_ops,
    pub host_data: *mut core::ffi::c_void, pub flags: u32, pub mapcount: u32, pub mutex: mutex,
    pub root: *mut irq_domain, pub fwnode: *mut fwnode_handle, pub bus_token: irq_domain_bus_token,
    pub gc: *mut irq_domain_chip_generic, pub dev: *mut device, pub pm_dev: *mut device,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")] pub parent: *mut irq_domain,
    #[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")] pub msi_parent_ops: *const msi_parent_ops,
    pub exit: Option<unsafe extern "C" fn(*mut irq_domain)>, pub hwirq_max: irq_hw_number_t,
    pub revmap_size: u32, pub revmap_tree: radix_tree_root,
    pub revmap: [*mut irq_data; 0],
}

pub const IRQ_DOMAIN_FLAG_HIERARCHY: u32 = 1 << 0;
pub const IRQ_DOMAIN_NAME_ALLOCATED: u32 = 1 << 1;
pub const IRQ_DOMAIN_FLAG_IPI_PER_CPU: u32 = 1 << 2;
pub const IRQ_DOMAIN_FLAG_IPI_SINGLE: u32 = 1 << 3;
pub const IRQ_DOMAIN_FLAG_MSI: u32 = 1 << 4;
pub const IRQ_DOMAIN_FLAG_ISOLATED_MSI: u32 = 1 << 5;
pub const IRQ_DOMAIN_FLAG_NO_MAP: u32 = 1 << 6;
pub const IRQ_DOMAIN_FLAG_MSI_PARENT: u32 = 1 << 8;
pub const IRQ_DOMAIN_FLAG_MSI_DEVICE: u32 = 1 << 9;
pub const IRQ_DOMAIN_FLAG_DESTROY_GC: u32 = 1 << 10;
pub const IRQ_DOMAIN_FLAG_MSI_IMMUTABLE: u32 = 1 << 11;
pub const IRQ_DOMAIN_FLAG_FWNODE_PARENT: u32 = 1 << 12;
pub const IRQ_DOMAIN_FLAG_NONCORE: u32 = 1 << 16;

#[repr(C)] pub struct irq_domain_info {
    pub fwnode: *mut fwnode_handle, pub domain_flags: u32, pub size: u32, pub hwirq_max: irq_hw_number_t,
    pub direct_max: core::ffi::c_int, pub hwirq_base: u32, pub virq_base: u32, pub bus_token: irq_domain_bus_token,
    pub name_suffix: *const core::ffi::c_char, pub ops: *const irq_domain_ops, pub host_data: *mut core::ffi::c_void,
    pub dev: *mut device, #[cfg(feature="CONFIG_IRQ_DOMAIN_HIERARCHY")] pub parent: *mut irq_domain,
    pub dgc_info: *mut irq_domain_chip_generic_info, pub init: Option<unsafe extern "C" fn(*mut irq_domain)->core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut irq_domain)>,
}

pub const IRQCHIP_FWNODE_REAL: u32 = 0; pub const IRQCHIP_FWNODE_NAMED: u32 = 1; pub const IRQCHIP_FWNODE_NAMED_ID: u32 = 2;
extern "C" {
    pub fn of_phandle_args_to_fwspec(*mut device_node, *const u32, u32, *mut irq_fwspec);
    pub fn irq_domain_instantiate(*const irq_domain_info) -> *mut irq_domain;
    pub fn devm_irq_domain_instantiate(*mut device, *const irq_domain_info) -> *mut irq_domain;
    pub fn __irq_domain_alloc_fwnode(u32, core::ffi::c_int, *const core::ffi::c_char, *mut phys_addr_t, *mut fwnode_handle) -> *mut fwnode_handle;
    pub fn irq_domain_free_fwnode(*mut fwnode_handle);
    pub fn irq_domain_remove(*mut irq_domain);
    pub fn irq_domain_associate(*mut irq_domain, u32, irq_hw_number_t) -> core::ffi::c_int;
    pub fn irq_domain_associate_many(*mut irq_domain, u32, irq_hw_number_t, core::ffi::c_int);
    pub fn irq_create_mapping_affinity(*mut irq_domain, irq_hw_number_t, *const irq_affinity_desc) -> u32;
    pub fn irq_create_fwspec_mapping(*mut irq_fwspec) -> u32;
    pub fn irq_dispose_mapping(u32);
    pub fn __irq_resolve_mapping(*mut irq_domain, irq_hw_number_t, *mut u32) -> *mut irq_desc;
    pub fn irq_domain_xlate_onecell(*mut irq_domain,*mut device_node,*const u32,u32,*mut irq_hw_number_t,*mut u32)->core::ffi::c_int;
    pub fn irq_domain_xlate_twocell(*mut irq_domain,*mut device_node,*const u32,u32,*mut irq_hw_number_t,*mut u32)->core::ffi::c_int;
    pub fn irq_domain_xlate_onetwocell(*mut irq_domain,*mut device_node,*const u32,u32,*mut irq_hw_number_t,*mut u32)->core::ffi::c_int;
    pub fn irq_domain_xlate_twothreecell(*mut irq_domain,*mut device_node,*const u32,u32,*mut irq_hw_number_t,*mut u32)->core::ffi::c_int;
    pub fn irq_domain_translate_onecell(*mut irq_domain,*mut irq_fwspec,*mut usize,*mut u32)->core::ffi::c_int;
    pub fn irq_domain_translate_twocell(*mut irq_domain,*mut irq_fwspec,*mut usize,*mut u32)->core::ffi::c_int;
    pub fn irq_domain_translate_twothreecell(*mut irq_domain,*mut irq_fwspec,*mut usize,*mut u32)->core::ffi::c_int;
}

pub unsafe fn irq_domain_set_pm_device(d: *mut irq_domain, dev: *mut device) { if !d.is_null() { (*d).pm_dev = dev; } }
pub unsafe fn irq_create_mapping(d: *mut irq_domain, h: irq_hw_number_t) -> u32 { irq_create_mapping_affinity(d,h,core::ptr::null()) }
pub unsafe fn irq_resolve_mapping(d: *mut irq_domain,h: irq_hw_number_t)->*mut irq_desc { __irq_resolve_mapping(d,h,core::ptr::null_mut()) }
pub unsafe fn irq_find_mapping(d: *mut irq_domain,h: irq_hw_number_t)->u32 { let mut irq=0; if !__irq_resolve_mapping(d,h,&mut irq).is_null(){irq}else{0} }
pub unsafe fn irq_domain_is_hierarchy(d:*mut irq_domain)->bool{(*d).flags & IRQ_DOMAIN_FLAG_HIERARCHY != 0}
pub unsafe fn irq_domain_is_ipi(d:*mut irq_domain)->bool{(*d).flags & (IRQ_DOMAIN_FLAG_IPI_PER_CPU|IRQ_DOMAIN_FLAG_IPI_SINGLE)!=0}
pub unsafe fn irq_domain_is_ipi_per_cpu(d:*mut irq_domain)->bool{(*d).flags & IRQ_DOMAIN_FLAG_IPI_PER_CPU != 0}
pub unsafe fn irq_domain_is_ipi_single(d:*mut irq_domain)->bool{(*d).flags & IRQ_DOMAIN_FLAG_IPI_SINGLE != 0}
pub unsafe fn irq_domain_is_msi(d:*mut irq_domain)->bool{(*d).flags & IRQ_DOMAIN_FLAG_MSI != 0}
pub unsafe fn irq_domain_is_msi_parent(d:*mut irq_domain)->bool{(*d).flags & IRQ_DOMAIN_FLAG_MSI_PARENT != 0}
pub unsafe fn irq_domain_is_msi_device(d:*mut irq_domain)->bool{(*d).flags & IRQ_DOMAIN_FLAG_MSI_DEVICE != 0}
pub unsafe fn irq_domain_is_msi_immutable(d:*mut irq_domain)->bool{(*d).flags & IRQ_DOMAIN_FLAG_MSI_IMMUTABLE != 0}

// External types and declarations referenced by the original header are supplied by dependent headers.
extern "C" { pub static irq_domain_simple_ops: irq_domain_ops; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
