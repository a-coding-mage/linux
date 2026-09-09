/* Intel GTT (Graphics Translation Table) routines.  Kernel dependencies are
 * intentionally left external; this is a source-level translation. */

#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_void;

type dma_addr_t = u64;
type phys_addr_t = u64;
type resource_size_t = u64;
type u8 = u8; type u16 = u16; type u32 = u32; type u64 = u64;
type off_t = isize;

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub device: u16, pub devfn: u8, _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct sg_table { pub sgl: *mut scatterlist, pub nents: i32, pub orig_nents: i32 }
#[repr(C)] pub struct resource { pub start: resource_size_t, pub end: resource_size_t, pub name: *const u8, pub flags: u64 }
#[repr(C)] pub struct agp_memory { pub pages: *mut *mut page, pub page_count: usize, pub num_scratch_pages: usize, pub type_: i32, pub physical: phys_addr_t, pub is_flushed: bool, pub sg_list: *mut scatterlist, pub num_sg: i32, pub key: i32 }
#[repr(C)] pub struct agp_bridge_data { pub driver: *const agp_bridge_driver, pub dev_private_data: *mut c_void, pub dev: *mut pci_dev, pub current_size: *mut c_void, pub gart_bus_addr: dma_addr_t }
#[repr(C)] pub struct agp_bridge_driver { _private: [u8; 0] }

extern "C" {
    static mut agp_bridge: *mut agp_bridge_data;
    fn pci_resource_start(d: *mut pci_dev, bar: i32) -> phys_addr_t;
    fn pci_resource_len(d: *mut pci_dev, bar: i32) -> resource_size_t;
    fn pci_bus_address(d: *mut pci_dev, bar: i32) -> resource_size_t;
    fn pci_read_config_word(d: *mut pci_dev, off: u32, v: *mut u16);
    fn pci_read_config_dword(d: *mut pci_dev, off: u32, v: *mut u32);
    fn pci_write_config_word(d: *mut pci_dev, off: u32, v: u16);
    fn pci_write_config_dword(d: *mut pci_dev, off: u32, v: u32);
    fn pci_dev_get(d: *mut pci_dev) -> *mut pci_dev; fn pci_dev_put(d: *mut pci_dev);
    fn pci_get_device(vendor: u16, device: u16, from: *mut pci_dev) -> *mut pci_dev;
    fn ioremap(addr: phys_addr_t, size: usize) -> *mut u8; fn ioremap_wc(addr: phys_addr_t, size: usize) -> *mut u8; fn iounmap(p: *mut u8);
    fn readl(p: *mut u8) -> u32; fn readb(p: *mut u8) -> u8; fn writel(v: u32, p: *mut u8); fn writel_relaxed(v: u32, p: *mut u8);
    fn alloc_page(flags: u32) -> *mut page; fn __free_page(p: *mut page); fn alloc_gatt_pages(order: i32) -> *mut i8; fn free_gatt_pages(p: *mut i8, order: i32);
    fn page_to_phys(p: *mut page) -> phys_addr_t; fn virt_to_phys(p: *mut i8) -> phys_addr_t;
    fn set_pages_uc(p: *mut page, n: usize) -> i32; fn set_pages_wb(p: *mut page, n: usize) -> i32;
    fn dma_map_page(d: *mut device, p: *mut page, o: usize, n: usize, dir: i32) -> dma_addr_t; fn dma_unmap_page(d: *mut device, a: dma_addr_t, n: usize, dir: i32);
    fn dma_mapping_error(d: *mut device, a: dma_addr_t) -> bool; fn global_cache_flush(); fn wmb(); fn udelay(x: u32); fn wbinvd_on_all_cpus();
    fn device_iommu_mapped(d: *mut device) -> bool; fn intel_gmch_remove();
}

const PAGE_SHIFT: u32 = 12; const PAGE_SIZE: usize = 4096;
const I810_PTE_VALID: u32 = 1; const I810_PTE_LOCAL: u32 = 2; const I830_PTE_SYSTEM_CACHED: u32 = 8;
const AGP_DCACHE_MEMORY: u32 = 1; const AGP_USER_CACHED_MEMORY: u32 = 2; const AGP_USER_MEMORY: u32 = 3; const AGP_PHYS_MEMORY: u32 = 4;
const DMA_BIDIRECTIONAL: i32 = 0; const GFP_KERNEL: u32 = 0; const GFP_DMA32: u32 = 0; const __GFP_ZERO: u32 = 0;
const I810_MMADR_BAR: i32 = 0; const I810_PTE_BASE: u64 = 0; const I810_PGETBL_CTL: usize = 0; const I810_PGETBL_ENABLED: u32 = 1; const I810_DRAM_CTL: usize = 0; const I810_DRAM_ROW_0: u32 = 0; const I810_DRAM_ROW_0_SDRAM: u32 = 0;
const I830_GMCH_CTRL: u32 = 0; const I830_GMCH_ENABLED: u16 = 1; const I830_GMCH_GMS_MASK: u16 = 0; const I830_GMCH_MEM_MASK: u16 = 0; const I830_GMCH_MEM_64M: u16 = 0; const I830_RDRAM_CHANNEL_TYPE: usize = 0;
const I915_MMADR_BAR: i32 = 0; const I915_PTE_BAR: i32 = 0; const GFX_FLSH_CNTL: usize = 0;

#[repr(C)] struct IntelGttDriver { gen: u8, is_g33: bool, is_pineview: bool, is_ironlake: bool, has_pgtbl_enable: bool, dma_mask_size: u8, setup: Option<unsafe extern "C" fn()->i32>, cleanup: Option<unsafe extern "C" fn()>, write_entry: Option<unsafe extern "C" fn(dma_addr_t,u32,u32)>, read_entry: Option<unsafe extern "C" fn(u32,*mut bool,*mut bool)->dma_addr_t>, check_flags: Option<unsafe extern "C" fn(u32)->bool>, chipset_flush: Option<unsafe extern "C" fn()> }
#[repr(C)] struct IntelPrivate { driver: *const IntelGttDriver, pcidev: *mut pci_dev, bridge_dev: *mut pci_dev, registers: *mut u8, gtt_phys_addr: phys_addr_t, pgetbl_save: u32, gtt: *mut u8, clear_fake_agp: bool, num_dcache_entries: i32, i9xx_flush_page: *mut c_void, i81x_gtt_table: *mut i8, ifp_resource: resource, resource_valid: i32, scratch_page: *mut page, scratch_page_dma: phys_addr_t, refcount: i32, needs_dmar: bool, gma_bus_addr: phys_addr_t, stolen_size: resource_size_t, gtt_total_entries: u32, gtt_mappable_entries: u32 }
static mut intel_private: IntelPrivate = IntelPrivate { driver: core::ptr::null(), pcidev: core::ptr::null_mut(), bridge_dev: core::ptr::null_mut(), registers: core::ptr::null_mut(), gtt_phys_addr: 0, pgetbl_save: 0, gtt: core::ptr::null_mut(), clear_fake_agp: false, num_dcache_entries: 0, i9xx_flush_page: core::ptr::null_mut(), i81x_gtt_table: core::ptr::null_mut(), ifp_resource: resource { start: 0, end: 0, name: core::ptr::null(), flags: 0 }, resource_valid: 0, scratch_page: core::ptr::null_mut(), scratch_page_dma: 0, refcount: 0, needs_dmar: false, gma_bus_addr: 0, stolen_size: 0, gtt_total_entries: 0, gtt_mappable_entries: 0 };

unsafe extern "C" fn i830_check_flags(f:u32)->bool { matches!(f,0|AGP_PHYS_MEMORY|AGP_USER_CACHED_MEMORY|AGP_USER_MEMORY) }
unsafe extern "C" fn i810_write_entry(addr:dma_addr_t, entry:u32, flags:u32) { let mut x=I810_PTE_VALID; if flags==AGP_DCACHE_MEMORY{x|=I810_PTE_LOCAL} else if flags==AGP_USER_CACHED_MEMORY{x|=I830_PTE_SYSTEM_CACHED}; writel_relaxed((addr as u32)|x, intel_private.gtt.add(entry as usize*4)); }
unsafe extern "C" fn i830_write_entry(addr:dma_addr_t,entry:u32,flags:u32){let mut x=I810_PTE_VALID;if flags==AGP_USER_CACHED_MEMORY{x|=I830_PTE_SYSTEM_CACHED};writel_relaxed(addr as u32|x,intel_private.gtt.add(entry as usize*4));}
unsafe extern "C" fn i965_write_entry(mut addr:dma_addr_t,entry:u32,flags:u32){let mut x=I810_PTE_VALID;if flags==AGP_USER_CACHED_MEMORY{x|=I830_PTE_SYSTEM_CACHED};addr|=addr>>28&0xf0;writel_relaxed(addr as u32|x,intel_private.gtt.add(entry as usize*4));}
unsafe extern "C" fn i810_read_entry(e:u32,p:*mut bool,l:*mut bool)->dma_addr_t{let v=readl(intel_private.gtt.add(e as usize*4));*p=v&I810_PTE_VALID!=0;*l=v&I810_PTE_LOCAL!=0;(v&!0xfff) as dma_addr_t}
unsafe extern "C" fn i830_read_entry(e:u32,p:*mut bool,l:*mut bool)->dma_addr_t{let v=readl(intel_private.gtt.add(e as usize*4));*p=v&I810_PTE_VALID!=0;*l=false;(v&!0xfff) as dma_addr_t}
unsafe extern "C" fn i965_read_entry(e:u32,p:*mut bool,l:*mut bool)->dma_addr_t{let v=readl(intel_private.gtt.add(e as usize*4)) as u64;*p=v&I810_PTE_VALID as u64!=0;*l=false;((v&0xf0)<<28)|(v&!0xfff)}

#[no_mangle] pub unsafe extern "C" fn intel_gmch_gtt_insert_page(addr:dma_addr_t,pg:u32,flags:u32){if let Some(f)=(*intel_private.driver).write_entry{f(addr,pg,flags)};readl(intel_private.gtt.add(pg as usize*4));if let Some(f)=(*intel_private.driver).chipset_flush{f()}}
#[no_mangle] pub unsafe extern "C" fn intel_gmch_gtt_read_entry(pg:u32,p:*mut bool,l:*mut bool)->dma_addr_t{(*intel_private.driver).read_entry.unwrap()(pg,p,l)}
#[no_mangle] pub unsafe extern "C" fn intel_gmch_gtt_clear_range(first:u32,n:u32){for i in first..first+n{(*intel_private.driver).write_entry.unwrap()(intel_private.scratch_page_dma,i,0)};wmb()}
#[no_mangle] pub unsafe extern "C" fn intel_gmch_gtt_flush(){if let Some(f)=(*intel_private.driver).chipset_flush{f()}}
#[no_mangle] pub unsafe extern "C" fn intel_gmch_gtt_get(total:*mut u64,base:*mut phys_addr_t,end:*mut resource_size_t){*total=(intel_private.gtt_total_entries as u64)<<PAGE_SHIFT;*base=intel_private.gma_bus_addr;*end=(intel_private.gtt_mappable_entries as u64)<<PAGE_SHIFT}
#[no_mangle] pub unsafe extern "C" fn intel_gmch_remove(){intel_private.refcount-=1;if intel_private.refcount!=0{return} if !intel_private.scratch_page.is_null(){__free_page(intel_private.scratch_page)};if !intel_private.pcidev.is_null(){pci_dev_put(intel_private.pcidev)};if !intel_private.bridge_dev.is_null(){pci_dev_put(intel_private.bridge_dev)};intel_private.driver=core::ptr::null()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
