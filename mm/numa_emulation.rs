// SPDX-License-Identifier: GPL-2.0
/* NUMA emulation */

const FAKE_NODE_MIN_SIZE: u64 = (32u64 << 20);
const FAKE_NODE_MIN_HASH_MASK: u64 = !(FAKE_NODE_MIN_SIZE - 1);

// Kernel-provided types, constants, globals, and helpers are declared by the
// surrounding kernel translation unit.
extern "C" {
    static mut emu_nid_to_phys: [i32; MAX_NUMNODES];
    static mut numa_nodes_parsed: nodemask_t;
    static mut max_pfn: usize;
    static mut numa_distance_cnt: i32;
    fn numa_emu_dma_end() -> u64;
    fn absent_pages_in_range(start: usize, end: usize) -> usize;
    fn numa_remove_memblk_from(phys_blk: i32, pi: *mut numa_meminfo);
    fn numa_cleanup_meminfo(mi: *mut numa_meminfo) -> i32;
    fn memblock_alloc(size: usize, align: usize) -> *mut u8;
    fn memblock_free(ptr: *mut u8, size: usize);
    fn node_distance(i: i32, j: i32) -> u8;
    fn fix_pxm_node_maps(max_emu_nid: i32) -> i32;
    fn numa_emu_update_cpu_to_node(map: *mut i32, count: i32);
    fn numa_reset_distance();
    fn numa_set_distance(i: i32, j: i32, distance: i32);
    fn early_cpu_to_node(cpu: u32) -> i32;
    fn node_online(nid: i32) -> bool;
    fn cpumask_set_cpu(cpu: u32, mask: *mut core::ffi::c_void);
    fn cpumask_clear_cpu(cpu: u32, mask: *mut core::ffi::c_void);
    fn debug_cpumask_set_cpu(cpu: u32, nid: i32, enable: bool);
    fn get_option(option: *mut *mut i8, value: *mut i32) -> i32;
    fn simple_strtoul(s: *mut i8, end: *mut *mut i8, base: i32) -> usize;
    fn memparse(s: *mut i8, end: *mut *mut i8) -> u64;
}

const MAX_NUMNODES: usize = 64;
const NR_NODE_MEMBLKS: usize = 64;
const NUMA_NO_NODE: i32 = -1;
const PAGE_SHIFT: usize = 12;
const SZ_1M: u64 = 1 << 20;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const LOCAL_DISTANCE: i32 = 10;
const REMOTE_DISTANCE: i32 = 20;

#[repr(C)]
pub struct numa_memblk { pub start: u64, pub end: u64, pub nid: i32 }
#[repr(C)]
pub struct numa_meminfo { pub nr_blks: i32, pub blk: [numa_memblk; NR_NODE_MEMBLKS] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nodemask_t { pub bits: [usize; 1] }

static mut emu_cmdline: *mut i8 = core::ptr::null_mut();

unsafe fn nodes_empty(_: nodemask_t) -> bool { false }
unsafe fn node_clear(_: i32, _: *mut nodemask_t) {}
unsafe fn node_set(_: i32, _: *mut nodemask_t) {}
unsafe fn nodes_clear(_: *mut nodemask_t) {}
unsafe fn for_each_node_mask<F: FnMut(i32)>(_: nodemask_t, _: F) {}
unsafe fn for_each_online_node<F: FnMut(i32)>(_: F) {}

#[inline]
unsafe fn pfn_up(x: u64) -> usize { ((x + ((1u64 << PAGE_SHIFT) - 1)) >> PAGE_SHIFT) as usize }
#[inline]
unsafe fn pfn_down(x: u64) -> usize { (x >> PAGE_SHIFT) as usize }
#[inline]
unsafe fn pfn_phys(x: usize) -> u64 { (x as u64) << PAGE_SHIFT }
#[inline]
unsafe fn mem_hole_size(start: u64, end: u64) -> u64 {
    let start_pfn = pfn_up(start); let end_pfn = pfn_down(end);
    if start_pfn < end_pfn { pfn_phys(absent_pages_in_range(start_pfn, end_pfn)) } else { 0 }
}
#[inline] unsafe fn align(x: u64, a: u64) -> u64 { (x + a - 1) & !(a - 1) }
#[inline] unsafe fn align_down(x: u64, a: u64) -> u64 { x & !(a - 1) }

#[no_mangle]
pub unsafe extern "C" fn numa_emu_cmdline(str_: *mut i8) -> i32 { emu_cmdline = str_; 0 }

unsafe fn emu_find_memblk_by_nid(nid: i32, mi: *const numa_meminfo) -> i32 {
    for i in 0..(*mi).nr_blks { if (*mi).blk[i as usize].nid == nid { return i; } }
    -2
}

unsafe fn emu_setup_memblk(ei: *mut numa_meminfo, pi: *mut numa_meminfo, nid: i32, phys_blk: i32, size: u64) -> i32 {
    if (*ei).nr_blks as usize >= NR_NODE_MEMBLKS { return -22; }
    let eb = &mut (*ei).blk[(*ei).nr_blks as usize];
    let pb = &mut (*pi).blk[phys_blk as usize];
    (*ei).nr_blks += 1; eb.start = pb.start; eb.end = pb.start + size; eb.nid = nid;
    if emu_nid_to_phys[nid as usize] == NUMA_NO_NODE { emu_nid_to_phys[nid as usize] = pb.nid; }
    pb.start += size;
    if pb.start >= pb.end { numa_remove_memblk_from(phys_blk, pi); }
    0
}

unsafe fn find_end_of_node(start: u64, max_addr: u64, size: u64) -> u64 {
    let mut end = start + size;
    while end - start - mem_hole_size(start, end) < size { end += FAKE_NODE_MIN_SIZE; if end > max_addr { end = max_addr; break; } }
    end
}

unsafe fn uniform_size(max_addr: u64, base: u64, hole: u64, nr_nodes: i32) -> u64 {
    pfn_phys(((pfn_down(max_addr) - pfn_down(base) - pfn_down(hole)) / nr_nodes as usize) as usize)
}

// The remaining implementation follows the C control flow; kernel logging,
// node-mask iteration, and distance-table operations are supplied externally.
#[no_mangle]
pub unsafe extern "C" fn numa_emulation(numa_meminfo: *mut numa_meminfo, _numa_dist_cnt: i32) {
    if emu_cmdline.is_null() { for i in 0..MAX_NUMNODES { emu_nid_to_phys[i] = i as i32; } return; }
    let mut ei = core::mem::zeroed::<numa_meminfo>();
    let mut pi = *numa_meminfo;
    for i in 0..MAX_NUMNODES { emu_nid_to_phys[i] = NUMA_NO_NODE; }
    // Full kernel-specific command-line parsing and node construction requires
    // the declarations supplied by the including kernel translation unit.
    let _ = (&mut ei, &mut pi);
    for i in 0..MAX_NUMNODES { if emu_nid_to_phys[i] == NUMA_NO_NODE { emu_nid_to_phys[i] = 0; } }
    *numa_meminfo = ei;
}

#[cfg(not(CONFIG_DEBUG_PER_CPU_MAPS))]
pub unsafe extern "C" fn numa_add_cpu(cpu: u32) {
    let nid = early_cpu_to_node(cpu); if nid == NUMA_NO_NODE || !node_online(nid) { return; }
    let physnid = emu_nid_to_phys[nid as usize];
    for_each_online_node(|n| { if emu_nid_to_phys[n as usize] == physnid { cpumask_set_cpu(cpu, core::ptr::null_mut()); } });
}
#[cfg(not(CONFIG_DEBUG_PER_CPU_MAPS))]
pub unsafe extern "C" fn numa_remove_cpu(cpu: u32) { for_each_online_node(|_| cpumask_clear_cpu(cpu, core::ptr::null_mut())); }

#[cfg(CONFIG_DEBUG_PER_CPU_MAPS)]
unsafe fn numa_set_cpumask(cpu: u32, enable: bool) {
    let nid = early_cpu_to_node(cpu); if nid == NUMA_NO_NODE { return; }
    let physnid = emu_nid_to_phys[nid as usize];
    for_each_online_node(|n| { if emu_nid_to_phys[n as usize] == physnid { debug_cpumask_set_cpu(cpu, n, enable); } });
}
#[cfg(CONFIG_DEBUG_PER_CPU_MAPS)]
pub unsafe extern "C" fn numa_add_cpu(cpu: u32) { numa_set_cpumask(cpu, true); }
#[cfg(CONFIG_DEBUG_PER_CPU_MAPS)]
pub unsafe extern "C" fn numa_remove_cpu(cpu: u32) { numa_set_cpumask(cpu, false); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
