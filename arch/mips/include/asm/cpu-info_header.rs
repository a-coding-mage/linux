/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 Waldorf GMBH
 * Copyright (C) 1995, 1996, 1997, 1998, 1999, 2001, 2002, 2003 Ralf Baechle
 * Copyright (C) 1996 Paul M. Antoine
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2004  Maciej W. Rozycki
 */

// C dependencies: linux/cache.h, linux/types.h, and asm/mipsregs.h.

#[repr(C)]
pub struct cache_desc {
    pub waysize: ::kernel::ffi::c_uint,
    pub sets: ::kernel::ffi::c_ushort,
    pub ways: u8,
    pub linesz: u8,
    pub waybit: u8,
    pub flags: u8,
}

#[repr(C)]
pub struct guest_info {
    pub ases: ::kernel::ffi::c_ulong,
    pub ases_dyn: ::kernel::ffi::c_ulong,
    pub options: u64,
    pub options_dyn: u64,
    pub tlbsize: ::kernel::ffi::c_int,
    pub conf: u8,
    pub kscratch_mask: u8,
}

pub const MIPS_CACHE_NOT_PRESENT: u32 = 0x00000001;
pub const MIPS_CACHE_VTAG: u32 = 0x00000002;
pub const MIPS_CACHE_ALIASES: u32 = 0x00000004;
pub const MIPS_CACHE_IC_F_DC: u32 = 0x00000008;
pub const MIPS_IC_SNOOPS_REMOTE: u32 = 0x00000010;
pub const MIPS_CACHE_PINDEX: u32 = 0x00000020;

#[repr(C)]
pub struct cpuinfo_mips {
    pub asid_cache: u64,
    // CONFIG_MIPS_ASID_BITS_VARIABLE
    #[cfg(CONFIG_MIPS_ASID_BITS_VARIABLE)]
    pub asid_mask: ::kernel::ffi::c_ulong,
    pub ases: ::kernel::ffi::c_ulong,
    pub options: u64,
    pub udelay_val: ::kernel::ffi::c_uint,
    pub processor_id: ::kernel::ffi::c_uint,
    pub fpu_id: ::kernel::ffi::c_uint,
    pub fpu_csr31: ::kernel::ffi::c_uint,
    pub fpu_msk31: ::kernel::ffi::c_uint,
    pub msa_id: ::kernel::ffi::c_uint,
    pub cputype: ::kernel::ffi::c_uint,
    pub isa_level: ::kernel::ffi::c_int,
    pub tlbsize: ::kernel::ffi::c_int,
    pub tlbsizevtlb: ::kernel::ffi::c_int,
    pub tlbsizeftlbsets: ::kernel::ffi::c_int,
    pub tlbsizeftlbways: ::kernel::ffi::c_int,
    pub icache: cache_desc,
    pub dcache: cache_desc,
    pub vcache: cache_desc,
    pub scache: cache_desc,
    pub tcache: cache_desc,
    pub srsets: ::kernel::ffi::c_int,
    pub package: ::kernel::ffi::c_int,
    pub globalnumber: ::kernel::ffi::c_uint,
    pub vmbits: ::kernel::ffi::c_int,
    pub data: *mut ::kernel::ffi::c_void,
    pub watch_reg_count: ::kernel::ffi::c_uint,
    pub watch_reg_use_cnt: ::kernel::ffi::c_uint,
    pub watch_reg_masks: [u16; 4],
    pub kscratch_mask: ::kernel::ffi::c_uint,
    pub writecombine: ::kernel::ffi::c_uint,
    pub htw_seq: ::kernel::ffi::c_uint,
    pub guest: guest_info,
    pub gtoffset_mask: ::kernel::ffi::c_uint,
    pub guestid_mask: ::kernel::ffi::c_uint,
    pub guestid_cache: ::kernel::ffi::c_uint,
    // CONFIG_CPU_LOONGSON3_CPUCFG_EMULATION
    #[cfg(CONFIG_CPU_LOONGSON3_CPUCFG_EMULATION)]
    pub loongson3_cpucfg_data: [u32; 3],
}

extern "C" {
    pub static mut cpu_data: [cpuinfo_mips; 0];
    pub fn cpu_probe();
    pub fn cpu_report();
    pub fn cpu_disable_mmid();
    pub static __cpu_name: [*const ::kernel::ffi::c_char; 0];
    pub fn register_proc_cpuinfo_notifier(nb: *mut notifier_block) -> ::kernel::ffi::c_int;
    pub fn proc_cpuinfo_notifier_call_chain(val: ::kernel::ffi::c_ulong, v: *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int;
}

pub struct seq_file;
pub struct notifier_block;

#[repr(C)]
pub struct proc_cpuinfo_notifier_args {
    pub m: *mut seq_file,
    pub n: ::kernel::ffi::c_ulong,
}

// The C notifier macro creates a static notifier_block and registers it.
#[macro_export]
macro_rules! proc_cpuinfo_notifier {
    ($fn:ident, $pri:expr) => {{
        static mut FN_NB: notifier_block = notifier_block;
        unsafe { register_proc_cpuinfo_notifier(&raw mut FN_NB) }
    }};
}

#[inline]
pub unsafe fn cpu_cluster(cpuinfo: *mut cpuinfo_mips) -> u32 {
    // CONFIG_CPU_MIPSR5/CONFIG_CPU_MIPSR6 build-time conditions are external.
    if !(IS_ENABLED(CONFIG_CPU_MIPSR5)) && !(IS_ENABLED(CONFIG_CPU_MIPSR6)) { return 0; }
    ((*cpuinfo).globalnumber & MIPS_GLOBALNUMBER_CLUSTER) >> MIPS_GLOBALNUMBER_CLUSTER_SHF
}

#[inline]
pub unsafe fn cpu_core(cpuinfo: *mut cpuinfo_mips) -> u32 {
    ((*cpuinfo).globalnumber & MIPS_GLOBALNUMBER_CORE) >> MIPS_GLOBALNUMBER_CORE_SHF
}

#[inline]
pub unsafe fn cpu_vpe_id(cpuinfo: *mut cpuinfo_mips) -> u32 {
    // CONFIG_MIPS_MT_SMP/CONFIG_CPU_MIPSR6 build-time conditions are external.
    if !(IS_ENABLED(CONFIG_MIPS_MT_SMP)) && !(IS_ENABLED(CONFIG_CPU_MIPSR6)) { return 0; }
    ((*cpuinfo).globalnumber & MIPS_GLOBALNUMBER_VP) >> MIPS_GLOBALNUMBER_VP_SHF
}

extern "C" {
    pub fn cpu_set_cluster(cpuinfo: *mut cpuinfo_mips, cluster: u32);
    pub fn cpu_set_core(cpuinfo: *mut cpuinfo_mips, core: u32);
    pub fn cpu_set_vpe_id(cpuinfo: *mut cpuinfo_mips, vpe: u32);
}

#[inline]
pub unsafe fn cpus_are_siblings(cpua: ::kernel::ffi::c_int, cpub: ::kernel::ffi::c_int) -> bool {
    let infoa = &cpu_data[cpua as usize];
    let infob = &cpu_data[cpub as usize];
    if infoa.package != infob.package { return false; }
    let gnuma = infoa.globalnumber & !MIPS_GLOBALNUMBER_VP;
    let gnumb = infob.globalnumber & !MIPS_GLOBALNUMBER_VP;
    if gnuma != gnumb { return false; }
    true
}

#[inline]
pub const fn cpu_asid_inc() -> ::kernel::ffi::c_ulong { 1 << CONFIG_MIPS_ASID_SHIFT }

#[inline]
pub unsafe fn cpu_asid_mask(cpuinfo: *mut cpuinfo_mips) -> ::kernel::ffi::c_ulong {
    #[cfg(CONFIG_MIPS_ASID_BITS_VARIABLE)]
    { return (*cpuinfo).asid_mask; }
    ((1 << CONFIG_MIPS_ASID_BITS) - 1) << CONFIG_MIPS_ASID_SHIFT
}

#[inline]
pub unsafe fn set_cpu_asid_mask(cpuinfo: *mut cpuinfo_mips, asid_mask: ::kernel::ffi::c_ulong) {
    #[cfg(CONFIG_MIPS_ASID_BITS_VARIABLE)]
    { (*cpuinfo).asid_mask = asid_mask; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
