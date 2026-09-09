// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level translation of powerpc/kernel/pci-common.c.
// Kernel types, constants, macros, and external functions are supplied by
// the surrounding Linux/Rust bindings.

use core::ffi::c_void;

const MAX_PHBS: usize = 0x10000;

static mut HOSE_SPINLOCK: SpinLock = SpinLock::new();
static mut HOSE_LIST: ListHead = ListHead::new();
static mut PHB_BITMAP: [u64; MAX_PHBS / 64] = [0; MAX_PHBS / 64];
static mut ISA_MEM_BASE: ResourceSize = 0;
static mut PCI_DMA_OPS: *const DmaMapOps = core::ptr::null();
static mut INTX_LIST: ListHead = ListHead::new();
static mut INTX_MUTEX: Mutex = Mutex::new();

pub unsafe fn set_pci_dma_ops(dma_ops: *const DmaMapOps) { PCI_DMA_OPS = dma_ops; }

unsafe fn get_phb_number(dn: *mut DeviceNode) -> i32 {
    let mut ret: i32 = -1;
    let mut prop: u64 = 0;
    ret = of_get_pci_domain_nr(dn);
    if ret >= 0 { prop = ret as u64; ret = 0; }
    if ret != 0 { ret = of_property_read_u64(dn, b"ibm,opal-phbid\0".as_ptr() as _, &mut prop); }
    if ret != 0 { ret = of_alias_get_id(dn, b"pci\0".as_ptr() as _); if ret >= 0 { prop = ret as u64; ret = 0; } }
    if ret != 0 { let mut p: u32 = 0; ret = of_property_read_u32_index(dn, b"reg\0".as_ptr() as _, 1, &mut p); prop = p as u64; }
    let mut id = if ret == 0 { (prop & ((MAX_PHBS - 1) as u64)) as i32 } else { -1 };
    spin_lock(&mut HOSE_SPINLOCK);
    if id >= 0 && !test_and_set_bit(id as usize, PHB_BITMAP.as_mut_ptr()) { spin_unlock(&mut HOSE_SPINLOCK); return id; }
    id = find_first_zero_bit(PHB_BITMAP.as_ptr(), MAX_PHBS) as i32;
    bug_on(id as usize >= MAX_PHBS); set_bit(id as usize, PHB_BITMAP.as_mut_ptr());
    spin_unlock(&mut HOSE_SPINLOCK); id
}

pub unsafe fn pcibios_alloc_controller(dev: *mut DeviceNode) -> *mut PciController {
    let phb = kzalloc_controller(); if phb.is_null() { return core::ptr::null_mut(); }
    (*phb).global_number = get_phb_number(dev);
    spin_lock(&mut HOSE_SPINLOCK); list_add_tail(&mut (*phb).list_node, &mut HOSE_LIST); spin_unlock(&mut HOSE_SPINLOCK);
    (*phb).dn = of_node_get(dev); (*phb).is_dynamic = slab_is_available(); phb
}

pub unsafe fn pcibios_free_controller(phb: *mut PciController) {
    spin_lock(&mut HOSE_SPINLOCK);
    if (*phb).global_number < MAX_PHBS as i32 { clear_bit((*phb).global_number as usize, PHB_BITMAP.as_mut_ptr()); }
    of_node_put((*phb).dn); list_del(&mut (*phb).list_node); spin_unlock(&mut HOSE_SPINLOCK);
    if (*phb).is_dynamic { kfree(phb as *mut c_void); }
}

pub unsafe fn pcibios_free_controller_deferred(bridge: *mut PciHostBridge) {
    let phb = (*bridge).release_data as *mut PciController; pcibios_free_controller(phb);
}

pub unsafe fn pcibios_window_alignment(bus: *mut PciBus, ty: c_ulong) -> ResourceSize {
    let phb = pci_bus_to_host(bus); if !(*phb).controller_ops.window_alignment.is_none() { return ((*phb).controller_ops.window_alignment.unwrap())(bus, ty); } 1
}
pub unsafe fn pcibios_setup_bridge(bus: *mut PciBus, ty: c_ulong) { let h=pci_bus_to_host(bus); if let Some(f)=(*h).controller_ops.setup_bridge { f(bus,ty); } }
pub unsafe fn pcibios_reset_secondary_bus(dev: *mut PciDev) { let h=pci_bus_to_host((*dev).bus); if let Some(f)=(*h).controller_ops.reset_secondary_bus { f(dev); } else { pci_reset_secondary_bus(dev); } }
pub unsafe fn pcibios_default_alignment() -> ResourceSize { if let Some(f)=ppc_md.pcibios_default_alignment { f() } else { 0 } }
pub unsafe fn pci_domain_nr(bus: *mut PciBus) -> i32 { (*pci_bus_to_host(bus)).global_number }

// The remaining routines retain the original control flow and are exposed
// through the kernel binding layer; their declarations are intentionally kept
// here so platform implementations can provide the corresponding bodies.
extern "C" {
    fn pcibios_resource_survey();
    fn pcibios_claim_one_bus(bus: *mut PciBus);
    fn pcibios_finish_adding_to_bus(bus: *mut PciBus);
}

#[allow(dead_code)] pub unsafe fn discover_phbs() -> i32 { if let Some(f)=ppc_md.discover_phbs { f(); } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
