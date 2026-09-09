/* SPDX-License-Identifier: MIT */

/*
 * Prototype for this hypercall is:
 *  int physdev_op(int cmd, void *args)
 * @cmd  == PHYSDEVOP_??? (physdev operation).
 * @args == Operation-specific extra arguments (NULL if none).
 */

pub const PHYSDEVOP_eoi: u32 = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_eoi { pub irq: u32 }

pub const PHYSDEVOP_pirq_eoi_gmfn_v1: u32 = 17;
pub const PHYSDEVOP_pirq_eoi_gmfn_v2: u32 = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_pirq_eoi_gmfn { pub gmfn: xen_ulong_t }

pub const PHYSDEVOP_irq_status_query: u32 = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_irq_status_query { pub irq: u32, pub flags: u32 }

pub const _XENIRQSTAT_needs_eoi: u32 = 0;
pub const XENIRQSTAT_needs_eoi: u32 = 1u32 << _XENIRQSTAT_needs_eoi;
pub const _XENIRQSTAT_shared: u32 = 1;
pub const XENIRQSTAT_shared: u32 = 1u32 << _XENIRQSTAT_shared;

pub const PHYSDEVOP_set_iopl: u32 = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_set_iopl { pub iopl: u32 }

pub const PHYSDEVOP_set_iobitmap: u32 = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_set_iobitmap { pub bitmap: *mut u8, pub nr_ports: u32 }

pub const PHYSDEVOP_apic_read: u32 = 8;
pub const PHYSDEVOP_apic_write: u32 = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_apic { pub apic_physbase: ::core::ffi::c_ulong, pub reg: u32, pub value: u32 }

pub const PHYSDEVOP_alloc_irq_vector: u32 = 10;
pub const PHYSDEVOP_free_irq_vector: u32 = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_irq { pub irq: u32, pub vector: u32 }

pub const MAP_PIRQ_TYPE_MSI: u32 = 0x0;
pub const MAP_PIRQ_TYPE_GSI: u32 = 0x1;
pub const MAP_PIRQ_TYPE_UNKNOWN: u32 = 0x2;
pub const MAP_PIRQ_TYPE_MSI_SEG: u32 = 0x3;
pub const MAP_PIRQ_TYPE_MULTI_MSI: u32 = 0x4;
pub const PHYSDEVOP_map_pirq: u32 = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_map_pirq {
    pub domid: domid_t, pub type_: i32, pub index: i32, pub pirq: i32,
    pub bus: i32, pub devfn: i32, pub entry_nr: i32, pub table_base: u64,
}

pub const PHYSDEVOP_unmap_pirq: u32 = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_unmap_pirq { pub domid: domid_t, pub pirq: i32 }

pub const PHYSDEVOP_manage_pci_add: u32 = 15;
pub const PHYSDEVOP_manage_pci_remove: u32 = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_manage_pci { pub bus: u8, pub devfn: u8 }

pub const PHYSDEVOP_restore_msi: u32 = 19;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_restore_msi { pub bus: u8, pub devfn: u8 }

pub const PHYSDEVOP_manage_pci_add_ext: u32 = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_manage_pci_ext {
    pub bus: u8, pub devfn: u8, pub is_extfn: u32, pub is_virtfn: u32,
    pub physfn: physdev_pci_fn,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_pci_fn { pub bus: u8, pub devfn: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub union physdev_op_u {
    pub irq_status_query: physdev_irq_status_query,
    pub set_iopl: physdev_set_iopl,
    pub set_iobitmap: physdev_set_iobitmap,
    pub apic_op: physdev_apic,
    pub irq_op: physdev_irq,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_op { pub cmd: u32, pub u: physdev_op_u }

pub const PHYSDEVOP_setup_gsi: u32 = 21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_setup_gsi { pub gsi: i32, pub triggering: u8, pub polarity: u8 }

pub const PHYSDEVOP_get_nr_pirqs: u32 = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_nr_pirqs { pub nr_pirqs: u32 }

pub const PHYSDEVOP_get_free_pirq: u32 = 23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_get_free_pirq { pub type_: i32, pub pirq: u32 }

pub const XEN_PCI_DEV_EXTFN: u32 = 0x1;
pub const XEN_PCI_DEV_VIRTFN: u32 = 0x2;
pub const XEN_PCI_DEV_PXM: u32 = 0x4;
pub const XEN_PCI_MMCFG_RESERVED: u32 = 0x1;
pub const PHYSDEVOP_pci_mmcfg_reserved: u32 = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_pci_mmcfg_reserved { pub address: u64, pub segment: u16, pub start_bus: u8, pub end_bus: u8, pub flags: u32 }

pub const PHYSDEVOP_pci_device_add: u32 = 25;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_pci_device_add {
    pub seg: u16, pub bus: u8, pub devfn: u8, pub flags: u32, pub physfn: physdev_pci_fn,
    pub optarr: [u32; 0],
}

pub const PHYSDEVOP_pci_device_remove: u32 = 26;
pub const PHYSDEVOP_restore_msi_ext: u32 = 27;
pub const PHYSDEVOP_prepare_msix: u32 = 30;
pub const PHYSDEVOP_release_msix: u32 = 31;
pub const PHYSDEVOP_pci_device_reset: u32 = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_pci_device { pub seg: u16, pub bus: u8, pub devfn: u8 }
pub const PCI_DEVICE_RESET_COLD: u32 = 0x0;
pub const PCI_DEVICE_RESET_WARM: u32 = 0x1;
pub const PCI_DEVICE_RESET_HOT: u32 = 0x2;
pub const PCI_DEVICE_RESET_FLR: u32 = 0x3;
pub const PCI_DEVICE_RESET_MASK: u32 = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_device_reset { pub dev: physdev_pci_device, pub flags: u32 }

pub const PHYSDEVOP_DBGP_RESET_PREPARE: u32 = 1;
pub const PHYSDEVOP_DBGP_RESET_DONE: u32 = 2;
pub const PHYSDEVOP_DBGP_BUS_UNKNOWN: u32 = 0;
pub const PHYSDEVOP_DBGP_BUS_PCI: u32 = 1;
pub const PHYSDEVOP_dbgp_op: u32 = 29;
#[repr(C)]
#[derive(Copy, Clone)]
pub union physdev_dbgp_op_u { pub pci: physdev_pci_device }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_dbgp_op { pub op: u8, pub bus: u8, pub u: physdev_dbgp_op_u }

pub const PHYSDEVOP_IRQ_UNMASK_NOTIFY: u32 = 4;
pub const PHYSDEVOP_IRQ_STATUS_QUERY: u32 = PHYSDEVOP_irq_status_query;
pub const PHYSDEVOP_SET_IOPL: u32 = PHYSDEVOP_set_iopl;
pub const PHYSDEVOP_SET_IOBITMAP: u32 = PHYSDEVOP_set_iobitmap;
pub const PHYSDEVOP_APIC_READ: u32 = PHYSDEVOP_apic_read;
pub const PHYSDEVOP_APIC_WRITE: u32 = PHYSDEVOP_apic_write;
pub const PHYSDEVOP_ASSIGN_VECTOR: u32 = PHYSDEVOP_alloc_irq_vector;
pub const PHYSDEVOP_FREE_VECTOR: u32 = PHYSDEVOP_free_irq_vector;
pub const PHYSDEVOP_IRQ_NEEDS_UNMASK_NOTIFY: u32 = XENIRQSTAT_needs_eoi;
pub const PHYSDEVOP_IRQ_SHARED: u32 = XENIRQSTAT_shared;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
