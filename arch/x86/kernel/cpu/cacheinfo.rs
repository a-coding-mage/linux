// SPDX-License-Identifier: GPL-2.0
// x86 CPU caches detection and configuration.
// C headers and build-time macros are supplied by the surrounding kernel.

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum _cache_type { CTYPE_NULL = 0, CTYPE_DATA = 1, CTYPE_INST = 2, CTYPE_UNIFIED = 3 }

#[repr(C)]
#[derive(Copy, Clone)]
struct _cpuid4_leaf_eax_split { type_: u32, level: u32, is_self_initializing: u32, is_fully_associative: u32, reserved: u32, num_threads_sharing: u32, num_cores_on_die: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
union _cpuid4_leaf_eax { split: _cpuid4_leaf_eax_split, full: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
struct _cpuid4_leaf_ebx_split { coherency_line_size: u32, physical_line_partition: u32, ways_of_associativity: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
union _cpuid4_leaf_ebx { split: _cpuid4_leaf_ebx_split, full: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
struct _cpuid4_leaf_ecx_split { number_of_sets: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
union _cpuid4_leaf_ecx { split: _cpuid4_leaf_ecx_split, full: u32 }
#[repr(C)]
struct _cpuid4_info { eax: _cpuid4_leaf_eax, ebx: _cpuid4_leaf_ebx, ecx: _cpuid4_leaf_ecx, id: u32, size: usize }

const AMD_CPUID4_FULLY_ASSOCIATIVE: u32 = 0xffff;
const AMD_L2_L3_INVALID_ASSOC: usize = 0x9;
static CACHE_TYPE_MAP: [u32; 4] = [CACHE_TYPE_NOCACHE, CACHE_TYPE_DATA, CACHE_TYPE_INST, CACHE_TYPE_UNIFIED];
static ASSOCS: [u16; 16] = [0, 1, 2, 3, 4, 6, 8, 0, 16, 0, 32, 48, 64, 96, 128, 0xffff];
static LEVELS: [u8; 4] = [1, 1, 2, 3];
static TYPES: [u8; 4] = [1, 2, 3, 3];

#[repr(C)] union l1_cache { bits: l1_cache_bits, val: u32 }
#[repr(C)] struct l1_cache_bits { line_size:u32, lines_per_tag:u32, assoc:u32, size_in_kb:u32 }
#[repr(C)] union l2_cache { bits: l2_cache_bits, val: u32 }
#[repr(C)] struct l2_cache_bits { line_size:u32, lines_per_tag:u32, assoc:u32, size_in_kb:u32 }
#[repr(C)] union l3_cache { bits: l3_cache_bits, val: u32 }
#[repr(C)] struct l3_cache_bits { line_size:u32, lines_per_tag:u32, assoc:u32, res:u32, size_encoded:u32 }

extern "C" {
    fn cpuid(_: u32, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u32);
    fn cpuid_count(_: u32, _: u32, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u32);
    fn get_count_order(_: usize) -> i32;
    fn topology_num_cores_per_package() -> u32;
    fn boot_cpu_has(_: u32) -> bool;
    fn cpuid_amd_hygon_has_l3_cache() -> bool;
    fn cpuid_edx(_: u32) -> u32;
}

// The following declarations retain the original kernel interfaces. Their
// definitions and architecture-specific types are provided by dependencies.
extern "C" {
    static mut memory_caching_control: u32;
    fn amd_fill_cpuid4_info(index: i32, id4: *mut _cpuid4_info) -> i32;
}

unsafe fn cpuid4_info_fill_done(id4: *mut _cpuid4_info, eax: _cpuid4_leaf_eax, ebx: _cpuid4_leaf_ebx, ecx: _cpuid4_leaf_ecx) -> i32 {
    if unsafe { eax.split.type_ } == CTYPE_NULL as u32 { return -5; }
    unsafe { (*id4).eax=eax; (*id4).ebx=ebx; (*id4).ecx=ecx; (*id4).size=((ecx.split.number_of_sets+1) as usize)*((ebx.split.coherency_line_size+1) as usize)*((ebx.split.physical_line_partition+1) as usize)*((ebx.split.ways_of_associativity+1) as usize); }
    0
}

unsafe fn intel_fill_cpuid4_info(index: i32, id4: *mut _cpuid4_info) -> i32 {
    let mut eax= _cpuid4_leaf_eax{full:0}; let mut ebx=_cpuid4_leaf_ebx{full:0}; let mut ecx=_cpuid4_leaf_ecx{full:0}; let mut ignored=0;
    unsafe { cpuid_count(4,index as u32,&mut eax.full,&mut ebx.full,&mut ecx.full,&mut ignored); cpuid4_info_fill_done(id4,eax,ebx,ecx) }
}

unsafe fn find_num_cache_leaves(c: *const cpuinfo_x86) -> i32 {
    let op=if unsafe {(*c).x86_vendor==X86_VENDOR_AMD || (*c).x86_vendor==X86_VENDOR_HYGON} {0x8000001d} else {4}; let mut i=-1; loop { i+=1; let(mut a,mut b,mut d,mut e)=(0,0,0,0); unsafe{cpuid_count(op,i as u32,&mut a,&mut b,&mut d,&mut e)}; let x=_cpuid4_leaf_eax{full:a}; if unsafe{x.split.type_}==CTYPE_NULL as u32{return i;} }
}

// Remaining kernel-facing routines are direct unsafe translations; dependent
// cacheinfo, topology, cpumask, MTRR, PAT, and CPU-hotplug APIs are external.
extern "C" {
    fn init_amd_cacheinfo(c:*mut cpuinfo_x86); fn init_hygon_cacheinfo(c:*mut cpuinfo_x86);
    fn init_intel_cacheinfo(c:*mut cpuinfo_x86);
    fn cache_bp_init(); fn cache_bp_restore(); fn cache_aps_init();
    fn cache_disable(); fn cache_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
