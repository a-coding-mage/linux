/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_NUMA_MEMBLKS */

/* #define NR_NODE_MEMBLKS (MAX_NUMNODES * 2) */
pub const NR_NODE_MEMBLKS: usize = MAX_NUMNODES * 2;

extern "C" {
    pub fn numa_set_distance(from: ::kernel::ffi::c_int,
                              to: ::kernel::ffi::c_int,
                              distance: ::kernel::ffi::c_int);
    pub fn numa_reset_distance();
}

#[repr(C)]
pub struct numa_memblk {
    pub start: u64,
    pub end: u64,
    pub nid: ::kernel::ffi::c_int,
}

#[repr(C)]
pub struct numa_meminfo {
    pub nr_blks: ::kernel::ffi::c_int,
    pub blk: [numa_memblk; NR_NODE_MEMBLKS],
}

extern "C" {
    pub fn numa_add_memblk(nodeid: ::kernel::ffi::c_int, start: u64, end: u64) -> ::kernel::ffi::c_int;
    pub fn numa_add_reserved_memblk(nid: ::kernel::ffi::c_int, start: u64, end: u64) -> ::kernel::ffi::c_int;
    pub fn numa_remove_memblk_from(idx: ::kernel::ffi::c_int, mi: *mut numa_meminfo);
    pub fn numa_cleanup_meminfo(mi: *mut numa_meminfo) -> ::kernel::ffi::c_int;
    pub fn numa_memblks_init(init_func: Option<unsafe extern "C" fn() -> ::kernel::ffi::c_int>,
                             memblock_force_top_down: bool) -> ::kernel::ffi::c_int;
    pub static mut numa_distance_cnt: ::kernel::ffi::c_int;
}

/* CONFIG_NUMA_EMU */
#[cfg(CONFIG_NUMA_EMU)]
extern "C" {
    pub static mut emu_nid_to_phys: [::kernel::ffi::c_int; MAX_NUMNODES];
    pub fn numa_emu_cmdline(str_: *mut ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn numa_emu_update_cpu_to_node(emu_nid_to_phys: *mut ::kernel::ffi::c_int,
                                       nr_emu_nids: ::kernel::ffi::c_uint);
    pub fn numa_emu_dma_end() -> u64;
    pub fn numa_emulation(numa_meminfo: *mut numa_meminfo,
                          numa_dist_cnt: ::kernel::ffi::c_int);
}

#[cfg(not(CONFIG_NUMA_EMU))]
#[inline]
pub unsafe fn numa_emulation(_numa_meminfo: *mut numa_meminfo,
                             _numa_dist_cnt: ::kernel::ffi::c_int) {
}

#[cfg(not(CONFIG_NUMA_EMU))]
#[inline]
pub unsafe fn numa_emu_cmdline(_str: *mut ::kernel::ffi::c_char) -> ::kernel::ffi::c_int {
    -EINVAL
}

/* CONFIG_NUMA_KEEP_MEMINFO */
#[cfg(CONFIG_NUMA_KEEP_MEMINFO)]
extern "C" {
    pub fn phys_to_target_node(start: u64) -> ::kernel::ffi::c_int;
    pub fn memory_add_physaddr_to_nid(start: u64) -> ::kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
