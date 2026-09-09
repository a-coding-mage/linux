/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Direct Rust translation of the C header. */

/* Kernel dependencies are supplied by other translated headers. */

pub const IOMMU_PAGE_SHIFT_4K: u32 = 12;
pub const IOMMU_PAGE_SIZE_4K: usize = 1usize << IOMMU_PAGE_SHIFT_4K;
pub const IOMMU_PAGE_MASK_4K: usize = !((1usize << IOMMU_PAGE_SHIFT_4K) - 1);
pub const DIRECT64_PROPNAME: &str = "linux,direct64-ddr-window-info";
pub const DMA64_PROPNAME: &str = "linux,dma64-ddr-window-info";
pub const MIN_DDW_VPMEM_DMA_WINDOW: usize = 1usize << 31;
pub const IOMAP_MAX_ORDER: u32 = 13;
pub const IOMMU_POOL_HASHBITS: u32 = 2;
pub const IOMMU_NR_POOLS: usize = 1usize << IOMMU_POOL_HASHBITS;
pub const IOMMU_TABLE_GROUP_MAX_TABLES: usize = 2;

pub type CInt = i32;
pub type ULong = usize;

/* Opaque types supplied by the kernel. */
pub enum Device {}
pub enum Scatterlist {}
pub enum MmStruct {}
pub enum IommuGroup {}
pub enum ListHead {}
pub enum RcuHead {}
pub enum Kref {}
pub enum SpinlockT {}
pub enum PciControllerOps {}
pub enum DmaMapOps {}
pub enum PpcMd {}

#[repr(C)]
pub struct IommuTableOps {
    pub set: Option<unsafe extern "C" fn(*mut IommuTable, isize, isize, ULong, DmaDataDirection, ULong) -> CInt>,
    pub xchg_no_kill: Option<unsafe extern "C" fn(*mut IommuTable, isize, *mut ULong, *mut DmaDataDirection) -> CInt>,
    pub tce_kill: Option<unsafe extern "C" fn(*mut IommuTable, ULong, ULong)>,
    pub useraddrptr: Option<unsafe extern "C" fn(*mut IommuTable, isize, bool) -> *mut Be64>,
    pub clear: Option<unsafe extern "C" fn(*mut IommuTable, isize, isize)>,
    pub get: Option<unsafe extern "C" fn(*mut IommuTable, isize) -> ULong>,
    pub flush: Option<unsafe extern "C" fn(*mut IommuTable)>,
    pub free: Option<unsafe extern "C" fn(*mut IommuTable)>,
}

pub type Be64 = u64;
pub type DmaAddrT = usize;
pub type PhysAddrT = usize;
pub type SizeT = usize;
pub type GfpT = u32;

#[repr(C)]
pub struct IommuPool {
    pub start: ULong,
    pub end: ULong,
    pub hint: ULong,
    pub lock: SpinlockT,
}

#[repr(C)]
pub struct IommuTable {
    pub it_busno: ULong,
    pub it_size: ULong,
    pub it_indirect_levels: ULong,
    pub it_level_size: ULong,
    pub it_allocated_size: ULong,
    pub it_offset: ULong,
    pub it_base: ULong,
    pub it_index: ULong,
    pub it_type: ULong,
    pub it_blocksize: ULong,
    pub poolsize: ULong,
    pub nr_pools: ULong,
    pub large_pool: IommuPool,
    pub pools: [IommuPool; IOMMU_NR_POOLS],
    pub it_map: *mut ULong,
    pub it_page_shift: ULong,
    pub it_group_list: ListHead,
    pub it_userspace: *mut Be64,
    pub it_ops: *mut IommuTableOps,
    pub it_kref: Kref,
    pub it_nid: CInt,
    pub it_reserved_start: ULong,
    pub it_reserved_end: ULong,
}

#[repr(C)]
pub struct IommuTableGroupOps {
    pub get_table_size: Option<unsafe extern "C" fn(u32, u64, u32) -> ULong>,
    pub create_table: Option<unsafe extern "C" fn(*mut IommuTableGroup, CInt, u32, u64, u32, *mut *mut IommuTable) -> isize>,
    pub set_window: Option<unsafe extern "C" fn(*mut IommuTableGroup, CInt, *mut IommuTable) -> isize>,
    pub unset_window: Option<unsafe extern "C" fn(*mut IommuTableGroup, CInt) -> isize>,
    pub take_ownership: Option<unsafe extern "C" fn(*mut IommuTableGroup, *mut Device) -> isize>,
    pub release_ownership: Option<unsafe extern "C" fn(*mut IommuTableGroup, *mut Device)>,
}

#[repr(C)]
pub struct IommuTableGroupLink {
    pub next: ListHead,
    pub rcu: RcuHead,
    pub table_group: *mut IommuTableGroup,
}

#[repr(C)]
pub struct IommuTableGroup {
    pub tce32_start: u32,
    pub tce32_size: u32,
    pub pgsizes: u64,
    pub max_dynamic_windows_supported: u32,
    pub max_levels: u32,
    pub group: *mut IommuGroup,
    pub tables: [*mut IommuTable; IOMMU_TABLE_GROUP_MAX_TABLES],
    pub ops: *mut IommuTableGroupOps,
}

#[repr(i32)]
pub enum DmaDataDirection { DmaNone = 0 }

pub static mut iommu_is_off: CInt = 0;
pub static mut iommu_force_on: CInt = 0;
pub static mut iommu_table_lpar_multi_ops: IommuTableOps = unsafe { core::mem::zeroed() };
pub static mut iommu_table_pseries_ops: IommuTableOps = unsafe { core::mem::zeroed() };

pub const fn iommu_page_size(tblptr: &IommuTable) -> ULong { 1usize << tblptr.it_page_shift }
pub const fn iommu_page_mask(tblptr: &IommuTable) -> ULong { !((1usize << tblptr.it_page_shift) - 1) }
pub const fn iommu_page_align_4k(addr: ULong) -> ULong { (addr + IOMMU_PAGE_SIZE_4K - 1) & IOMMU_PAGE_MASK_4K }
pub const fn iommu_page_align(addr: ULong, tblptr: &IommuTable) -> ULong { (addr + iommu_page_size(tblptr) - 1) & iommu_page_mask(tblptr) }

pub unsafe fn get_iommu_order(size: ULong, tbl: *mut IommuTable) -> CInt {
    (usize::BITS - ((size.wrapping_sub(1) >> (*tbl).it_page_shift) as u32).leading_zeros()) as CInt
}

pub unsafe fn set_iommu_table_base(dev: *mut Device, base: *mut IommuTable) { let _ = (dev, base); }
pub unsafe fn get_iommu_table_base(_dev: *mut Device) -> *mut core::ffi::c_void { core::ptr::null_mut() }

extern "C" {
    pub fn dma_iommu_dma_supported(dev: *mut Device, mask: u64) -> CInt;
    pub fn iommu_tce_table_get(tbl: *mut IommuTable) -> *mut IommuTable;
    pub fn iommu_tce_table_put(tbl: *mut IommuTable) -> CInt;
    pub fn iommu_init_table(tbl: *mut IommuTable, nid: CInt, res_start: ULong, res_end: ULong) -> *mut IommuTable;
    pub fn iommu_table_in_use(tbl: *mut IommuTable) -> bool;
    pub fn iommu_table_reserve_pages(tbl: *mut IommuTable, res_start: ULong, res_end: ULong);
    pub fn iommu_table_clear(tbl: *mut IommuTable);
    pub fn ppc_iommu_map_sg(dev: *mut Device, tbl: *mut IommuTable, sglist: *mut Scatterlist, nelems: CInt, mask: ULong, direction: DmaDataDirection, attrs: ULong) -> CInt;
    pub fn ppc_iommu_unmap_sg(tbl: *mut IommuTable, sglist: *mut Scatterlist, nelems: CInt, direction: DmaDataDirection, attrs: ULong);
    pub fn iommu_alloc_coherent(dev: *mut Device, tbl: *mut IommuTable, size: SizeT, dma_handle: *mut DmaAddrT, mask: ULong, flag: GfpT, node: CInt) -> *mut core::ffi::c_void;
    pub fn iommu_free_coherent(tbl: *mut IommuTable, size: SizeT, vaddr: *mut core::ffi::c_void, dma_handle: DmaAddrT);
    pub fn iommu_map_phys(dev: *mut Device, tbl: *mut IommuTable, phys: PhysAddrT, size: SizeT, mask: ULong, direction: DmaDataDirection, attrs: ULong) -> DmaAddrT;
    pub fn iommu_unmap_phys(tbl: *mut IommuTable, dma_handle: DmaAddrT, size: SizeT, direction: DmaDataDirection, attrs: ULong);
    pub fn iommu_init_early_pSeries();
    pub fn iommu_init_early_dart(controller_ops: *mut PciControllerOps);
    pub fn iommu_init_early_pasemi();
    pub fn iommu_tce_check_ioba(page_shift: ULong, offset: ULong, size: ULong, ioba: ULong, npages: ULong) -> CInt;
    pub fn iommu_tce_check_gpa(page_shift: ULong, gpa: ULong) -> CInt;
    pub fn iommu_flush_tce(tbl: *mut IommuTable);
    pub fn iommu_tce_direction(tce: ULong) -> DmaDataDirection;
    pub fn iommu_direction_to_tce_perm(dir: DmaDataDirection) -> ULong;
}

extern "C" {
    pub fn iommu_register_group(table_group: *mut IommuTableGroup, pci_domain_number: CInt, pe_num: ULong);
    pub fn iommu_add_device(table_group: *mut IommuTableGroup, dev: *mut Device) -> CInt;
    pub fn iommu_tce_xchg(mm: *mut MmStruct, tbl: *mut IommuTable, entry: ULong, hpa: *mut ULong, direction: *mut DmaDataDirection) -> isize;
    pub fn iommu_tce_xchg_no_kill(mm: *mut MmStruct, tbl: *mut IommuTable, entry: ULong, hpa: *mut ULong, direction: *mut DmaDataDirection) -> isize;
    pub fn iommu_tce_kill(tbl: *mut IommuTable, entry: ULong, pages: ULong);
    pub fn dev_has_iommu_table(dev: *mut Device, data: *mut core::ffi::c_void) -> CInt;
    pub fn dma_iommu_get_required_mask(dev: *mut Device) -> u64;
}

/* CONFIG_IOMMU_API-disabled inline definitions. */
#[inline]
pub unsafe fn iommu_register_group_disabled(_table_group: *mut IommuTableGroup, _pci_domain_number: CInt, _pe_num: ULong) {}
#[inline]
pub unsafe fn iommu_add_device_disabled(_table_group: *mut IommuTableGroup, _dev: *mut Device) -> CInt { 0 }
#[inline]
pub unsafe fn dev_has_iommu_table_disabled(_dev: *mut Device, _data: *mut core::ffi::c_void) -> CInt { 0 }

#[inline]
pub unsafe fn iommu_tce_clear_param_check(tbl: *mut IommuTable, ioba: ULong, tce_value: ULong, npages: ULong) -> CInt {
    if iommu_tce_check_ioba((*tbl).it_page_shift, (*tbl).it_offset, (*tbl).it_size, ioba, npages) != 0 || tce_value != 0 { 1 } else { 0 }
}
#[inline]
pub unsafe fn iommu_tce_put_param_check(tbl: *mut IommuTable, ioba: ULong, gpa: ULong) -> CInt {
    if iommu_tce_check_ioba((*tbl).it_page_shift, (*tbl).it_offset, (*tbl).it_size, ioba, 1) != 0 || iommu_tce_check_gpa((*tbl).it_page_shift, gpa) != 0 { 1 } else { 0 }
}

pub static iommu_ops: DmaMapOps = unsafe { core::mem::zeroed() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
