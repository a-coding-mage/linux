/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/arch/alpha/kernel/pci_impl.h
 *
 * Declarations and inline definitions for interfacing with PCI initialization.
 */

// C forward declarations; the corresponding Rust types are supplied by dependencies.
pub struct pci_dev;
pub struct pci_controller;

pub const EISA_DEFAULT_IO_BASE: usize = 0x9000; // start above 8th slot
pub const DEFAULT_IO_BASE: usize = 0x8000; // start at 8th slot

pub const XL_DEFAULT_MEM_BASE: usize = (16 + 2) * 1024 * 1024;
pub const APECS_AND_LCA_DEFAULT_MEM_BASE: usize = (16 + 2) * 1024 * 1024;
pub const MCPCIA_DEFAULT_MEM_BASE: usize = (32 + 2) * 1024 * 1024;
pub const T2_DEFAULT_MEM_BASE: usize = (16 + 1) * 1024 * 1024;
pub const DEFAULT_MEM_BASE: usize = (128 + 16) * 1024 * 1024;
pub const CIA_DEFAULT_MEM_BASE: usize = (32 + 2) * 1024 * 1024;
pub const IRONGATE_DEFAULT_MEM_BASE: usize = (256 * 8 - 16) * 1024 * 1024;
pub const DEFAULT_AGP_APER_SIZE: usize = 64 * 1024 * 1024;

/* The table-based IRQ mapping operation for single-bus Alphas. */
#[macro_export]
macro_rules! common_table_lookup {
    ($slot:expr, $min_idsel:expr, $max_idsel:expr, $pin:expr,
     $irqs_per_slot:expr, $irq_tab:expr) => {{
        let mut _ctl_: libc::c_long = -1;
        if $slot >= $min_idsel && $slot <= $max_idsel && $pin < $irqs_per_slot {
            _ctl_ = $irq_tab[$slot - $min_idsel][$pin];
        }
        _ctl_
    }};
}

/* A PCI IOMMU allocation arena. */
#[repr(C)]
pub struct pci_iommu_arena {
    pub lock: spinlock_t,
    pub hose: *mut pci_controller,
    pub ptes: *mut libc::c_ulong,
    pub dma_base: dma_addr_t,
    pub size: libc::c_uint,
    pub next_entry: libc::c_uint,
    pub align_entry: libc::c_uint,
}

pub const IOMMU_INVALID_PTE: libc::c_ulong = 0x2; // 32:63 bits MBZ
pub const IOMMU_RESERVED_PTE: libc::c_ulong = 0xface;

/* Build-time CONFIG_ALPHA_SRM/CONFIG_ALPHA_CIA and CONFIG_ALPHA_GENERIC
 * conditions determine whether SRM setup restoration is required. */

#[cfg(any(feature = "alpha_generic", feature = "need_srm_save_restore"))]
extern "C" {
    pub fn pci_restore_srm_config();
}

#[cfg(not(any(feature = "alpha_generic", feature = "need_srm_save_restore")))]
#[inline(always)]
pub unsafe fn pci_restore_srm_config() {}

pub static mut hose_head: *mut pci_controller;
pub static mut hose_tail: *mut *mut pci_controller;
pub static mut pci_isa_hose: *mut pci_controller;

pub static mut alpha_agpgart_size: libc::c_ulong;

extern "C" {
    pub fn common_init_pci();
    pub fn pci_common_swizzle();
    pub fn alloc_pci_controller() -> *mut pci_controller;
    pub fn alloc_resource() -> *mut resource;

    pub fn iommu_arena_new_node(
        node: libc::c_int,
        hose: *mut pci_controller,
        dma_base: dma_addr_t,
        size: libc::c_ulong,
        align: libc::c_ulong,
    ) -> *mut pci_iommu_arena;
    pub fn iommu_arena_new(
        hose: *mut pci_controller,
        dma_base: dma_addr_t,
        size: libc::c_ulong,
        align: libc::c_ulong,
    ) -> *mut pci_iommu_arena;

    pub static pci_io_names: *const *const libc::c_char;
    pub static pci_mem_names: *const *const libc::c_char;
    pub static pci_hae0_name: *const libc::c_char;

    pub fn size_for_memory(max: libc::c_ulong) -> libc::c_ulong;

    pub fn iommu_reserve(arena: *mut pci_iommu_arena, start: libc::c_long, n: libc::c_long) -> libc::c_int;
    pub fn iommu_release(arena: *mut pci_iommu_arena, start: libc::c_long, n: libc::c_long) -> libc::c_int;
    pub fn iommu_bind(arena: *mut pci_iommu_arena, start: libc::c_long, n: libc::c_long, pages: *mut *mut page) -> libc::c_int;
    pub fn iommu_unbind(arena: *mut pci_iommu_arena, start: libc::c_long, n: libc::c_long) -> libc::c_int;
}

pub use pci_common_swizzle as common_swizzle;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
