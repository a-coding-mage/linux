/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are referenced here.

pub const ARCH_GENERIC_PCI_MMAP_RESOURCE: i32 = 1;
#[inline]
pub const fn arch_can_pci_mmap_wc() -> i32 { 1 }

pub const PCIBIOS_MIN_IO: u64 = 0x1000;
pub const PCIBIOS_MIN_MEM: u64 = 0x10000000;

#[repr(C)]
pub struct pci_dev;
#[repr(C)]
pub struct pci_bus;

unsafe extern "C" {
    pub fn pci_iomap(dev: *mut pci_dev, bar: i32, max: usize) -> *mut core::ffi::c_void;
    pub fn pci_iounmap(dev: *mut pci_dev, addr: *mut core::ffi::c_void);
    pub fn pci_domain_nr(bus: *mut pci_bus) -> i32;
    pub fn pci_proc_domain(bus: *mut pci_bus) -> i32;
}

pub const ZPCI_BUS_NR: i32 = 0;
pub const ZPCI_NR_DMA_SPACES: usize = 1;
pub const ZPCI_NR_DEVICES: usize = CONFIG_PCI_NR_FUNCTIONS;
pub const ZPCI_DOMAIN_BITMAP_SIZE: usize = 1 << 16;

pub const ZPCI_FC_FN_ENABLED: u32 = 0x80;
pub const ZPCI_FC_ERROR: u32 = 0x40;
pub const ZPCI_FC_BLOCKED: u32 = 0x20;
pub const ZPCI_FC_DMA_ENABLED: u32 = 0x10;
pub const ZPCI_FMB_DMA_COUNTER_VALID: u32 = 1 << 23;

#[repr(C)]
pub struct zpci_fmb_fmt0 { pub dma_rbytes: u64, pub dma_wbytes: u64 }
#[repr(C)]
pub struct zpci_fmb_fmt1 {
    pub rx_bytes: u64, pub rx_packets: u64, pub tx_bytes: u64, pub tx_packets: u64,
}
#[repr(C)]
pub struct zpci_fmb_fmt2 { pub consumed_work_units: u64, pub max_work_units: u64 }
#[repr(C)]
pub struct zpci_fmb_fmt3 { pub tx_bytes: u64 }

#[repr(C, packed(1), align(128))]
pub struct zpci_fmb {
    pub format: u8,
    pub fmt_ind: u32,
    pub samples: u32,
    pub last_update: u64,
    pub ld_ops: u64,
    pub st_ops: u64,
    pub stb_ops: u64,
    pub rpcit_ops: u64,
    pub fmt: zpci_fmb_fmt,
}

#[repr(C)]
pub union zpci_fmb_fmt {
    pub fmt0: zpci_fmb_fmt0,
    pub fmt1: zpci_fmb_fmt1,
    pub fmt2: zpci_fmb_fmt2,
    pub fmt3: zpci_fmb_fmt3,
}

#[repr(C)]
pub enum zpci_state {
    ZPCI_FN_STATE_STANDBY = 0,
    ZPCI_FN_STATE_CONFIGURED = 1,
    ZPCI_FN_STATE_RESERVED = 2,
}

#[repr(C)]
pub struct zpci_bar_struct {
    pub res: *mut resource,
    pub mio_wb: *mut core::ffi::c_void,
    pub mio_wt: *mut core::ffi::c_void,
    pub val: u32,
    pub map_idx: u16,
    pub size: u8,
}

#[repr(C)]
pub struct kvm_zdev;

pub const ZPCI_FUNCTIONS_PER_BUS: usize = 256;
#[repr(C)]
pub struct zpci_bus {
    pub kref: kref,
    pub bus: *mut pci_bus,
    pub function: [*mut zpci_dev; ZPCI_FUNCTIONS_PER_BUS],
    pub resources: list_head,
    pub bus_next: list_head,
    pub bus_resource: resource,
    pub msi_parent_domain: *mut irq_domain,
    pub topo: i32,
    pub domain_nr: i32,
    pub multifunction: u8,
    pub topo_is_tid: u8,
    pub max_bus_speed: pci_bus_speed,
}

#[repr(C, packed(1))]
pub struct zpci_ccdf_err {
    pub reserved1: u32,
    pub fh: u32,
    pub fid: u32,
    pub ett: u32,
    pub mvn: u32,
    pub dmaas: u32,
    pub reserved2: u32,
    pub q: u32,
    pub rw: u32,
    pub faddr: u64,
    pub reserved3: u32,
    pub reserved4: u16,
    pub pec: u16,
}

pub const ZPCI_ERR_PENDING_MAX: usize = 4;
#[repr(C)]
pub struct zpci_ccdf_pending {
    pub mediated_recovery: bool,
    pub count: u8,
    pub head: u8,
    pub tail: u8,
    pub err: [zpci_ccdf_err; ZPCI_ERR_PENDING_MAX],
}

#[repr(C)]
pub struct zpci_dev {
    pub zbus: *mut zpci_bus,
    pub entry: list_head,
    pub iommu_list: list_head,
    pub kref: kref,
    pub rcu: rcu_head,
    pub hotplug_slot: hotplug_slot,
    pub state_lock: mutex,
    pub state: zpci_state,
    pub fid: u32, pub fh: u32, pub gisa: u32,
    pub vfn: u16, pub pchid: u16, pub maxstbl: u16, pub rid: u16, pub tid: u16,
    pub pfgid: u8, pub pft: u8, pub port: u8, pub fidparm: u8, pub dtsm: u8,
    pub rid_available: u8, pub has_hp_slot: u8, pub has_resources: u8, pub is_physfn: u8,
    pub util_str_avail: u8, pub tid_avail: u8, pub rtr_avail: u8,
    pub devfn: u32,
    pub pfip: [u8; CLP_PFIP_NR_SEGMENTS], pub uid: u32,
    pub util_str: [u8; CLP_UTIL_STR_LEN],
    pub msi_addr: u64, pub max_msi: u32, pub msi_first_bit: u32, pub msi_nr_irqs: u32,
    pub aibv: *mut airq_iv, pub aisb: usize,
    pub dma_table: *mut usize, pub tlb_refresh: i32,
    pub iommu_dev: iommu_device,
    pub res_name: [u8; 16], pub mio_capable: bool,
    pub bars: [zpci_bar_struct; PCI_STD_NUM_BARS],
    pub start_dma: u64, pub end_dma: u64, pub dma_mask: u64,
    pub fmb_lock: mutex, pub fmb: *mut zpci_fmb, pub fmb_update: u16, pub fmb_length: u16,
    pub version: u8, pub max_bus_speed: pci_bus_speed,
    pub debugfs_dev: *mut dentry,
    pub s390_domain: *mut iommu_domain, pub kzdev: *mut kvm_zdev, pub kzdev_lock: mutex,
    pub pending_errs: zpci_ccdf_pending, pub pending_errs_lock: mutex, pub dom_lock: spinlock_t,
}

#[inline]
pub unsafe fn zdev_enabled(zdev: *mut zpci_dev) -> bool {
    ((*zdev).fh & (1usize << 31) as u32) != 0
}

unsafe extern "C" {
    pub static zpci_attr_group: attribute_group;
    pub static pfip_attr_group: attribute_group;
    pub static zpci_ident_attr_group: attribute_group;
    pub static zpci_slot_attr_group: attribute_group;
}

// ARCH_PCI_DEV_GROUPS expands to references to the three attribute groups above.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
