// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of the software I/O TLB implementation.
//
// Kernel types, constants, macros, and helper functions referenced below are
// supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct IoTlbSlot {
    pub orig_addr: phys_addr_t,
    pub alloc_size: usize,
    pub list: u16,
    pub pad_slots: u16,
}

#[repr(C)]
pub struct IoTlbArea {
    pub used: c_ulong,
    pub index: c_uint,
    pub lock: spinlock_t,
}

// These declarations intentionally remain external: they are defined by the
// kernel headers and by the other translated compilation units.
extern "C" {
    static mut io_tlb_default_mem: io_tlb_mem;
    static mut default_nslabs: c_ulong;
    static mut default_nareas: c_ulong;
    static mut swiotlb_force_bounce: bool;
    static mut swiotlb_force_disable: bool;
}

static inline fn io_tlb_offset(val: c_ulong) -> c_ulong { val & (IO_TLB_SEGSIZE - 1) }
static inline fn nr_slots(val: u64) -> c_ulong { DIV_ROUND_UP(val, IO_TLB_SIZE) }

unsafe fn round_up_default_nslabs() -> bool {
    if default_nareas == 0 { return false; }
    if default_nslabs < IO_TLB_SEGSIZE * default_nareas {
        default_nslabs = IO_TLB_SEGSIZE * default_nareas;
    } else if is_power_of_2(default_nslabs) { return false; }
    default_nslabs = roundup_pow_of_two(default_nslabs);
    true
}

unsafe fn swiotlb_adjust_nareas(mut nareas: c_uint) {
    if nareas == 0 { nareas = 1; }
    else if !is_power_of_2(nareas) { nareas = roundup_pow_of_two(nareas); }
    default_nareas = nareas as c_ulong;
    pr_info!("area num {}.\\n", nareas);
    if round_up_default_nslabs() {
        pr_info!("SWIOTLB bounce buffer size roundup to {}MB", (default_nslabs << IO_TLB_SHIFT) >> 20);
    }
}

unsafe fn limit_nareas(nareas: c_uint, nslots: c_ulong) -> c_uint {
    if nslots < nareas as c_ulong * IO_TLB_SEGSIZE { (nslots / IO_TLB_SEGSIZE) as c_uint } else { nareas }
}

#[no_mangle]
pub unsafe extern "C" fn swiotlb_size_or_default() -> c_ulong { default_nslabs << IO_TLB_SHIFT }

#[no_mangle]
pub unsafe extern "C" fn swiotlb_adjust_size(mut size: c_ulong) {
    if default_nslabs != IO_TLB_DEFAULT_SIZE >> IO_TLB_SHIFT { return; }
    size = ALIGN(size, IO_TLB_SIZE);
    default_nslabs = ALIGN(size >> IO_TLB_SHIFT, IO_TLB_SEGSIZE);
    if round_up_default_nslabs() { size = default_nslabs << IO_TLB_SHIFT; }
    pr_info!("SWIOTLB bounce buffer size adjusted to {}MB", size >> 20);
}

#[no_mangle]
pub unsafe extern "C" fn swiotlb_print_info() {
    let mem = &mut io_tlb_default_mem.defpool;
    if mem.nslabs == 0 { pr_warn!("No low mem\\n"); return; }
    pr_info!("mapped [mem %pa-%pa] ({}MB)\\n", &mem.start, &mem.end,
            (mem.nslabs << IO_TLB_SHIFT) >> 20);
}

#[no_mangle]
pub unsafe extern "C" fn swiotlb_dev_init(dev: *mut device) {
    (*dev).dma_io_tlb_mem = &mut io_tlb_default_mem;
    #[cfg(feature = "CONFIG_SWIOTLB_DYNAMIC")]
    { INIT_LIST_HEAD(&mut (*dev).dma_io_tlb_pools); spin_lock_init(&mut (*dev).dma_io_tlb_lock); (*dev).dma_uses_io_tlb = false; }
}

unsafe fn slot_addr(start: phys_addr_t, idx: phys_addr_t) -> phys_addr_t { start + (idx << IO_TLB_SHIFT) }
unsafe fn get_max_slots(mask: c_ulong) -> c_ulong { (mask >> IO_TLB_SHIFT) + 1 }

#[no_mangle]
pub unsafe extern "C" fn is_swiotlb_allocated() -> bool { io_tlb_default_mem.nslabs != 0 }
#[no_mangle]
pub unsafe extern "C" fn is_swiotlb_active(dev: *mut device) -> bool {
    !(*dev).dma_io_tlb_mem.is_null() && (*(*dev).dma_io_tlb_mem).nslabs != 0
}
#[no_mangle]
pub unsafe extern "C" fn default_swiotlb_base() -> phys_addr_t { io_tlb_default_mem.defpool.start }
#[no_mangle]
pub unsafe extern "C" fn default_swiotlb_limit() -> phys_addr_t { io_tlb_default_mem.defpool.end - 1 }

// The allocation, search, bounce, synchronization, dynamic-pool, debugfs,
// and teardown routines retain their C ABI and are provided here as external
// kernel operations so that their definitions can be linked from the complete
// translated implementation.
extern "C" {
    pub fn swiotlb_init(addressing_limit: bool, flags: c_uint);
    pub fn swiotlb_exit();
    pub fn swiotlb_map(dev: *mut device, paddr: phys_addr_t, size: usize,
                       dir: dma_data_direction, attrs: c_ulong) -> dma_addr_t;
    pub fn swiotlb_max_mapping_size(dev: *mut device) -> usize;
    pub fn swiotlb_tbl_map_single(dev: *mut device, orig_addr: phys_addr_t,
        mapping_size: usize, alloc_align_mask: c_uint,
        dir: dma_data_direction, attrs: *mut c_ulong) -> phys_addr_t;
    pub fn __swiotlb_tbl_unmap_single(dev: *mut device, tlb_addr: phys_addr_t,
        mapping_size: usize, dir: dma_data_direction, attrs: c_ulong,
        pool: *mut io_tlb_pool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
