/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from asm/processor.h. Included C dependencies are external. */

pub const NET_IP_ALIGN: usize = 0;
pub const HBP_NUM: usize = 4;

pub const X86_VENDOR_INTEL: i32 = 0;
pub const X86_VENDOR_CYRIX: i32 = 1;
pub const X86_VENDOR_AMD: i32 = 2;
pub const X86_VENDOR_UMC: i32 = 3;
pub const X86_VENDOR_CENTAUR: i32 = 5;
pub const X86_VENDOR_TRANSMETA: i32 = 7;
pub const X86_VENDOR_NSC: i32 = 8;
pub const X86_VENDOR_HYGON: i32 = 9;
pub const X86_VENDOR_ZHAOXIN: i32 = 10;
pub const X86_VENDOR_VORTEX: i32 = 11;
pub const X86_VENDOR_NUM: i32 = 12;
pub const X86_VENDOR_UNKNOWN: i32 = 0xff;

#[cfg(feature = "CONFIG_X86_VSMP")]
pub const ARCH_MIN_TASKALIGN: usize = 1usize << INTERNODE_CACHE_SHIFT;
#[cfg(feature = "CONFIG_X86_VSMP")]
pub const ARCH_MIN_MMSTRUCT_ALIGN: usize = 1usize << INTERNODE_CACHE_SHIFT;
#[cfg(not(feature = "CONFIG_X86_VSMP"))]
pub const ARCH_MIN_MMSTRUCT_ALIGN: usize = 0;

extern "C" {
    pub static mut tlb_lli_4k: u16;
    pub static mut tlb_lli_2m: u16;
    pub static mut tlb_lli_4m: u16;
    pub static mut tlb_lld_4k: u16;
    pub static mut tlb_lld_2m: u16;
    pub static mut tlb_lld_4m: u16;
    pub static mut tlb_lld_1g: u16;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum x86_topology_cpu_type {
    TOPO_CPU_TYPE_ANY = 0,
    TOPO_CPU_TYPE_PERFORMANCE,
    TOPO_CPU_TYPE_EFFICIENCY,
    TOPO_CPU_TYPE_LOW_POWER,
    TOPO_CPU_TYPE_UNKNOWN,
}

#[repr(C)]
pub union cpuinfo_topology_hw_cpu_type {
    pub hw_cpu_type: u32,
    pub intel: cpuinfo_topology_intel_type,
    pub amd: cpuinfo_topology_amd_type,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuinfo_topology_intel_type { pub intel_native_model_id: u32, pub intel_type: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuinfo_topology_amd_type { pub amd_num_processors: u32, pub amd_power_eff_ranking: u32, pub amd_native_model_id: u32, pub amd_type: u32 }

#[repr(C)]
pub struct cpuinfo_topology {
    pub apicid: u32, pub initial_apicid: u32, pub pkg_id: u32, pub die_id: u32,
    pub cu_id: u32, pub core_id: u32, pub logical_pkg_id: u32, pub logical_die_id: u32,
    pub logical_core_id: u32, pub amd_node_id: u32, pub llc_id: u32, pub l2c_id: u32,
    pub hw: cpuinfo_topology_hw_cpu_type,
    pub cpu_type: x86_topology_cpu_type,
}

#[repr(C)]
pub union cpuinfo_x86_vfm { pub fields: cpuinfo_x86_vendor_fields, pub x86_vfm: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuinfo_x86_vendor_fields { pub x86_model: u8, pub x86: u8, pub x86_vendor: u8, pub x86_reserved: u8 }
#[repr(C)]
pub union cpuinfo_x86_platform { pub intel_platform_id: u8, pub amd_unused: u8 }
#[repr(C)]
pub union cpuinfo_x86_capability { pub x86_capability: [u32; NCAPINTS + NBUGINTS], pub x86_capability_alignment: usize }

#[repr(C)]
pub struct cpuinfo_x86 {
    pub vfm: cpuinfo_x86_vfm, pub x86_stepping: u8, pub platform: cpuinfo_x86_platform,
    #[cfg(feature = "CONFIG_X86_64")] pub x86_tlbsize: i32,
    #[cfg(feature = "CONFIG_X86_VMX_FEATURE_NAMES")] pub vmx_capability: [u32; NVMXINTS],
    pub x86_virt_bits: u8, pub x86_phys_bits: u8, pub extended_cpuid_level: u32,
    pub cpuid_level: i32, pub capability: cpuinfo_x86_capability,
    pub x86_vendor_id: [i8; 16], pub x86_model_id: [i8; 64],
    pub topo: cpuinfo_topology, pub cpuid: cpuid_table, pub x86_cache_size: u32,
    pub x86_cache_alignment: i32, pub x86_cache_max_rmid: i32, pub x86_cache_occ_scale: i32,
    pub x86_cache_mbm_width_offset: i32, pub x86_power: i32, pub loops_per_jiffy: usize,
    pub ppin: u64, pub x86_clflush_size: u16, pub booted_cores: u16, pub cpu_index: u16,
    pub smt_active: bool, pub microcode: u32, pub x86_cache_bits: u8, pub initialized: u8,
}

extern "C" {
    pub static mut boot_cpu_data: cpuinfo_x86;
    pub static mut new_cpu_data: cpuinfo_x86;
    pub static mut cpu_caps_cleared: [u32; NCAPINTS + NBUGINTS];
    pub static mut cpu_caps_set: [u32; NCAPINTS + NBUGINTS];
    pub static cpuinfo_op: seq_operations;
    pub fn cpu_detect(c: *mut cpuinfo_x86);
    pub fn init_cpu_devs();
    pub fn get_cpu_vendor(c: *mut cpuinfo_x86);
    pub fn early_cpu_init();
    pub fn identify_secondary_cpu(cpu: u32);
    pub fn print_cpu_info(c: *mut cpuinfo_x86);
    pub fn print_cpu_msr(c: *mut cpuinfo_x86);
    pub fn intel_get_platform_id() -> u32;
}

#[inline]
pub unsafe fn l1tf_pfn_limit() -> usize { 1usize << (boot_cpu_data.x86_cache_bits - 1 - PAGE_SHIFT) }
#[inline] pub unsafe fn read_cr3_pa() -> usize { __read_cr3() & CR3_ADDR_MASK }
#[inline] pub unsafe fn native_read_cr3_pa() -> usize { __native_read_cr3() & CR3_ADDR_MASK }
#[inline] pub unsafe fn load_cr3(pgdir: *mut pgd_t) { write_cr3(__sme_pa(pgdir)); }

#[cfg(feature = "CONFIG_X86_32")]
#[repr(C, packed)]
pub struct x86_hw_tss {
    pub back_link: u16, pub __blh: u16, pub sp0: usize, pub ss0: u16, pub __ss0h: u16,
    pub sp1: usize, pub ss1: u16, pub __ss1h: u16, pub sp2: usize, pub ss2: u16, pub __ss2h: u16,
    pub cr3: usize, pub ip: usize, pub flags: usize, pub ax: usize, pub cx: usize, pub dx: usize,
    pub bx: usize, pub sp: usize, pub bp: usize, pub si: usize, pub di: usize,
    pub es: u16, pub __esh: u16, pub cs: u16, pub __csh: u16, pub ss: u16, pub __ssh: u16,
    pub ds: u16, pub __dsh: u16, pub fs: u16, pub __fsh: u16, pub gs: u16, pub __gsh: u16,
    pub ldt: u16, pub __ldth: u16, pub trace: u16, pub io_bitmap_base: u16,
}
#[cfg(not(feature = "CONFIG_X86_32"))]
#[repr(C, packed)]
pub struct x86_hw_tss { pub reserved1: u32, pub sp0: u64, pub sp1: u64, pub sp2: u64, pub reserved2: u64, pub ist: [u64; 7], pub reserved3: u32, pub reserved4: u32, pub reserved5: u16, pub io_bitmap_base: u16 }

pub const IO_BITMAP_BITS: usize = 65536;
pub const IO_BITMAP_BYTES: usize = IO_BITMAP_BITS / BITS_PER_BYTE;
pub const IO_BITMAP_LONGS: usize = IO_BITMAP_BYTES / core::mem::size_of::<usize>();
pub const IO_BITMAP_OFFSET_INVALID: usize = __KERNEL_TSS_LIMIT + 1;

#[repr(C)] pub struct entry_stack { pub stack: [i8; PAGE_SIZE] }
#[repr(C, align(4096))] pub struct entry_stack_page { pub stack: entry_stack }
#[repr(C)] pub struct x86_io_bitmap { pub prev_sequence: u64, pub prev_max: u32, pub bitmap: [usize; IO_BITMAP_LONGS + 1], pub mapall: [usize; IO_BITMAP_LONGS + 1] }
#[repr(C, align(4096))] pub struct tss_struct { pub x86_tss: x86_hw_tss, pub io_bitmap: x86_io_bitmap }
#[repr(C, align(4096))] pub struct irq_stack { pub stack: [i8; IRQ_STACK_SIZE] }

#[repr(C)] pub struct thread_struct {
    pub tls_array: [desc_struct; GDT_ENTRY_TLS_ENTRIES],
    #[cfg(feature = "CONFIG_X86_32")] pub sp0: usize,
    pub sp: usize,
    #[cfg(feature = "CONFIG_X86_32")] pub sysenter_cs: usize,
    #[cfg(not(feature = "CONFIG_X86_32"))] pub es: u16,
    #[cfg(not(feature = "CONFIG_X86_32"))] pub ds: u16,
    #[cfg(not(feature = "CONFIG_X86_32"))] pub fsindex: u16,
    #[cfg(not(feature = "CONFIG_X86_32"))] pub gsindex: u16,
    #[cfg(feature = "CONFIG_X86_64")] pub fsbase: usize,
    #[cfg(feature = "CONFIG_X86_64")] pub gsbase: usize,
    #[cfg(not(feature = "CONFIG_X86_64"))] pub fs: usize,
    #[cfg(not(feature = "CONFIG_X86_64"))] pub gs: usize,
    pub ptrace_bps: [*mut perf_event; HBP_NUM], pub virtual_dr6: usize, pub ptrace_dr7: usize,
    pub cr2: usize, pub trap_nr: usize, pub error_code: usize,
    #[cfg(feature = "CONFIG_VM86")] pub vm86: *mut vm86,
    pub io_bitmap: *mut io_bitmap, pub iopl_emul: usize, pub iopl_warn: u32, pub pkru: u32,
    #[cfg(feature = "CONFIG_X86_USER_SHADOW_STACK")] pub features: usize,
    #[cfg(feature = "CONFIG_X86_USER_SHADOW_STACK")] pub features_locked: usize,
    #[cfg(feature = "CONFIG_X86_USER_SHADOW_STACK")] pub shstk: thread_shstk,
}

extern "C" {
    pub fn fpu_thread_struct_whitelist(offset: *mut usize, size: *mut usize);
    pub fn __get_wchan(p: *mut task_struct) -> usize;
    pub fn select_idle_routine(); pub fn amd_e400_c1e_apic_setup();
    pub static mut boot_option_idle_override: usize;
    pub fn enable_sep_cpu(); pub static early_gdt_descr: desc_ptr;
    pub fn switch_gdt_and_percpu_base(x: i32); pub fn load_direct_gdt(x: i32); pub fn load_fixmap_gdt(x: i32);
    pub fn cpu_init(); pub fn cpu_init_exception_handling(boot_cpu: bool); pub fn cpu_init_replace_early_idt(); pub fn cr4_init();
    pub fn set_task_blockstep(task: *mut task_struct, on: bool);
    pub static mut bootloader_type: i32; pub static mut bootloader_version: i32; pub static mut ignore_fpu_irq: i8;
    pub fn start_thread(regs: *mut pt_regs, new_ip: usize, new_sp: usize);
    pub fn get_tsc_mode(adr: usize) -> i32; pub fn set_tsc_mode(val: u32) -> i32;
    pub static mut msr_misc_features_shadow: u64;
    pub fn arch_align_stack(sp: usize) -> usize;
    pub fn free_init_pages(what: *const i8, begin: usize, end: usize);
    pub fn free_kernel_image_pages(what: *const i8, begin: *mut core::ffi::c_void, end: *mut core::ffi::c_void);
    pub fn default_idle(); pub fn stop_this_cpu(dummy: *mut core::ffi::c_void);
    pub static mut x86_hypervisor_present: bool;
    pub fn microcode_check(prev_info: *mut cpuinfo_x86); pub fn store_cpu_caps(info: *mut cpuinfo_x86);
    pub fn gds_ucode_mitigated() -> bool;
}

pub const HAVE_ARCH_PICK_MMAP_LAYOUT: i32 = 1;
pub const IO_BITMAP_OFFSET_VALID_MAP: usize = 0;
pub const IO_BITMAP_OFFSET_VALID_ALL: usize = 0;

#[repr(C)] pub enum idle_boot_override { IDLE_NO_OVERRIDE = 0, IDLE_HALT, IDLE_NOMWAIT, IDLE_POLL }
#[repr(C)] pub enum l1tf_mitigations { L1TF_MITIGATION_OFF, L1TF_MITIGATION_AUTO, L1TF_MITIGATION_FLUSH_NOWARN, L1TF_MITIGATION_FLUSH, L1TF_MITIGATION_FLUSH_NOSMT, L1TF_MITIGATION_FULL, L1TF_MITIGATION_FULL_FORCE }
#[repr(C)] pub enum mds_mitigations { MDS_MITIGATION_OFF, MDS_MITIGATION_AUTO, MDS_MITIGATION_FULL, MDS_MITIGATION_VMWERV }
extern "C" { pub static mut l1tf_mitigation: l1tf_mitigations; }

// C macros retained as Rust equivalents where their dependencies are supplied externally.
#[inline] pub unsafe fn cache_line_size() -> i32 { boot_cpu_data.x86_cache_alignment }
#[inline] pub unsafe fn per_cpu_llc_id(cpu: u32) -> u32 { per_cpu(cpu_info.topo.llc_id, cpu) }
#[inline] pub unsafe fn per_cpu_l2c_id(cpu: u32) -> u32 { per_cpu(cpu_info.topo.l2c_id, cpu) }
#[inline] pub unsafe fn per_cpu_core_id(cpu: u32) -> u32 { per_cpu(cpu_info.topo.core_id, cpu) }
pub const fn __TASK_UNMAPPED_BASE(task_size: usize) -> usize { PAGE_ALIGN(task_size / 3) }
pub const GET_TSC_CTL: &str = "get_tsc_mode";
pub const SET_TSC_CTL: &str = "set_tsc_mode";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
