/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependencies are supplied by other translated units. */

/* enough to cover all DEFINE_PER_CPUs in modules */
/* CONFIG_MODULES selects the first value. */
pub const PERCPU_MODULE_RESERVE: usize = 8 << 10;

/* minimum unit size, also is the maximum supported allocation size */
pub const PCPU_MIN_UNIT_SIZE: usize = pcpu_pfn_align(32 << 10);

/* minimum allocation size and shift in bytes */
pub const PCPU_MIN_ALLOC_SHIFT: usize = 2;
pub const PCPU_MIN_ALLOC_SIZE: usize = 1 << PCPU_MIN_ALLOC_SHIFT;

/*
 * The PCPU_BITMAP_BLOCK_SIZE must be the same size as PAGE_SIZE as the
 * updating of hints is used to manage the nr_empty_pop_pages in both
 * the chunk and globally.
 */
pub const PCPU_BITMAP_BLOCK_SIZE: usize = PAGE_SIZE;
pub const PCPU_BITMAP_BLOCK_BITS: usize = PCPU_BITMAP_BLOCK_SIZE >> PCPU_MIN_ALLOC_SHIFT;

/* CONFIG_KMALLOC_PARTITION_CACHES, CONFIG_LOCKDEP, CONFIG_PAGE_SIZE_4KB
 * determine this build-time value. */
pub const PERCPU_DYNAMIC_SIZE_SHIFT: usize = 10;

/*
 * Percpu allocator can serve percpu allocations before slab is
 * initialized which allows slab to depend on the percpu allocator.
 * The following parameter decide how much resource to preallocate
 * for this.  Keep PERCPU_DYNAMIC_RESERVE equal to or larger than
 * PERCPU_DYNAMIC_EARLY_SIZE.
 */
pub const PERCPU_DYNAMIC_EARLY_SIZE: usize = 20 << PERCPU_DYNAMIC_SIZE_SHIFT;

/*
 * PERCPU_DYNAMIC_RESERVE indicates the amount of free area to piggy
 * back on the first chunk for dynamic percpu allocation if arch is
 * manually allocating and mapping it for faster access (as a part of
 * large page mapping for example).
 *
 * The following values give between one and two pages of free space
 * after typical minimal boot (2-way SMP, single disk and NIC) with
 * both defconfig and a distro config on x86_64 and 32.  More
 * intelligent way to determine this would be nice.
 */
/* BITS_PER_LONG > 32 selects 28; otherwise 20. */
pub const PERCPU_DYNAMIC_RESERVE: usize = 20 << PERCPU_DYNAMIC_SIZE_SHIFT;

extern "C" {
    pub static mut pcpu_base_addr: *mut core::ffi::c_void;
    pub static pcpu_unit_offsets: *const c_ulong;
}

#[repr(C)]
pub struct pcpu_group_info {
    pub nr_units: c_int, /* aligned # of units */
    pub base_offset: c_ulong, /* base address offset */
    pub cpu_map: *mut c_uint, /* unit->cpu map, empty entries contain NR_CPUS */
}

#[repr(C)]
pub struct pcpu_alloc_info {
    pub static_size: usize,
    pub reserved_size: usize,
    pub dyn_size: usize,
    pub unit_size: usize,
    pub atom_size: usize,
    pub alloc_size: usize,
    pub __ai_size: usize, /* internal, don't use */
    pub nr_groups: c_int, /* 0 if grouping unnecessary */
    pub groups: [pcpu_group_info; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pcpu_fc {
    PCPU_FC_AUTO,
    PCPU_FC_EMBED,
    PCPU_FC_PAGE,
    PCPU_FC_NR,
}

extern "C" {
    pub static pcpu_fc_names: [*const c_char; pcpu_fc::PCPU_FC_NR as usize];
    pub static mut pcpu_chosen_fc: pcpu_fc;

    pub fn pcpu_alloc_alloc_info(nr_groups: c_int, nr_units: c_int) -> *mut pcpu_alloc_info;
    pub fn pcpu_free_alloc_info(ai: *mut pcpu_alloc_info);
    pub fn pcpu_setup_first_chunk(ai: *const pcpu_alloc_info, base_addr: *mut core::ffi::c_void);
    pub fn pcpu_embed_first_chunk(
        reserved_size: usize,
        dyn_size: usize,
        atom_size: usize,
        cpu_distance_fn: pcpu_fc_cpu_distance_fn_t,
        cpu_to_nd_fn: pcpu_fc_cpu_to_node_fn_t,
    ) -> c_int;

    pub fn __is_kernel_percpu_address(addr: c_ulong, can_addr: *mut c_ulong) -> bool;
    pub fn is_kernel_percpu_address(addr: c_ulong) -> bool;

    pub fn pcpu_alloc_noprof(size: usize, align: usize, reserved: bool, gfp: gfp_t) -> *mut core::ffi::c_void;
    pub fn free_percpu(pdata: *mut core::ffi::c_void);
    pub fn per_cpu_ptr_to_phys(addr: *mut core::ffi::c_void) -> phys_addr_t;
    pub fn pcpu_nr_pages() -> c_ulong;
}

pub type pcpu_fc_cpu_to_node_fn_t = unsafe extern "C" fn(cpu: c_int) -> c_int;
pub type pcpu_fc_cpu_distance_fn_t = unsafe extern "C" fn(from: c_uint, to: c_uint) -> c_int;

/* CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK */
extern "C" {
    pub fn pcpu_populate_pte(addr: c_ulong);
    pub fn pcpu_page_first_chunk(reserved_size: usize, cpu_to_nd_fn: pcpu_fc_cpu_to_node_fn_t) -> c_int;
}

/* CONFIG_SMP and CONFIG_HAVE_SETUP_PER_CPU_AREA control this declaration. */
extern "C" {
    pub fn setup_per_cpu_areas();
}

#[macro_export]
macro_rules! __alloc_percpu_gfp {
    ($size:expr, $align:expr, $gfp:expr) => {
        alloc_hooks(unsafe { pcpu_alloc_noprof($size, $align, false, $gfp) })
    };
}
#[macro_export]
macro_rules! __alloc_percpu {
    ($size:expr, $align:expr) => {
        alloc_hooks(unsafe { pcpu_alloc_noprof($size, $align, false, GFP_KERNEL) })
    };
}
#[macro_export]
macro_rules! __alloc_reserved_percpu {
    ($size:expr, $align:expr) => {
        alloc_hooks(unsafe { pcpu_alloc_noprof($size, $align, true, GFP_KERNEL) })
    };
}

#[macro_export]
macro_rules! alloc_percpu_gfp {
    ($type:ty, $gfp:expr) => {
        __alloc_percpu_gfp!(core::mem::size_of::<$type>(), core::mem::align_of::<$type>(), $gfp)
    };
}
#[macro_export]
macro_rules! alloc_percpu {
    ($type:ty) => {
        __alloc_percpu!(core::mem::size_of::<$type>(), core::mem::align_of::<$type>())
    };
}
#[macro_export]
macro_rules! alloc_percpu_noprof {
    ($type:ty) => {
        unsafe { pcpu_alloc_noprof(core::mem::size_of::<$type>(), core::mem::align_of::<$type>(), false, GFP_KERNEL) }
    };
}

/* DEFINE_FREE(free_percpu, void __percpu *, free_percpu(_T)) */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
