// SPDX-License-Identifier: GPL-2.0
/* cpu.c: Dinky routines to look for the kind of Sparc cpu
 *        we are on.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 */

// Kernel and architecture dependencies supplied by the surrounding tree.

#[repr(C)]
pub struct CpuInfo { pub psr_vers: i32, pub name: *const u8, pub pmu_name: *const u8 }
#[repr(C)]
pub struct FpuInfo { pub fp_vers: i32, pub name: *const u8 }
#[repr(C)]
pub struct ManufacturerInfo { pub psr_impl: i32, pub cpu_info: [CpuInfo; 8], pub fpu_info: [FpuInfo; 8] }

const fn cpu(ver: i32, name: Option<&'static [u8]>) -> CpuInfo {
    CpuInfo { psr_vers: ver, name: match name { Some(s) => s.as_ptr(), None => core::ptr::null() }, pmu_name: core::ptr::null() }
}
const fn cpu_pmu(ver: i32, name: &'static [u8], pmu: &'static [u8]) -> CpuInfo {
    CpuInfo { psr_vers: ver, name: name.as_ptr(), pmu_name: pmu.as_ptr() }
}
const fn fpu(ver: i32, name: Option<&'static [u8]>) -> FpuInfo {
    FpuInfo { fp_vers: ver, name: match name { Some(s) => s.as_ptr(), None => core::ptr::null() } }
}
const ZC: CpuInfo = cpu(-1, None);
const ZF: FpuInfo = fpu(-1, None);

static MANUFACTURER_INFO: &[ManufacturerInfo] = &[
    ManufacturerInfo { psr_impl: 0, cpu_info: [cpu(0, Some(b"Fujitsu  MB86900/1A or LSI L64831 SparcKIT-40\0")),cpu(4, Some(b"Fujitsu  MB86904\0")),cpu(5, Some(b"Fujitsu TurboSparc MB86907\0")),ZC,ZC,ZC,ZC,ZC], fpu_info: [fpu(0,Some(b"Fujitsu MB86910 or Weitek WTL1164/5\0")),fpu(1,Some(b"Fujitsu MB86911 or Weitek WTL1164/5 or LSI L64831\0")),fpu(2,Some(b"LSI Logic L64802 or Texas Instruments ACT8847\0")),fpu(3,Some(b"Weitek WTL3170/2\0")),fpu(4,Some(b"Lsi Logic/Meiko L64804 or compatible\0")),ZF,ZF,ZF] },
    ManufacturerInfo { psr_impl: 1, cpu_info: [cpu(0,Some(b"LSI Logic Corporation - L64811\0")),cpu(1,Some(b"Cypress/ROSS CY7C601\0")),cpu(3,Some(b"Cypress/ROSS CY7C611\0")),cpu(0xf,Some(b"ROSS HyperSparc RT620\0")),cpu(0xe,Some(b"ROSS HyperSparc RT625 or RT626\0")),ZC,ZC,ZC], fpu_info: [fpu(0,Some(b"ROSS HyperSparc combined IU/FPU\0")),fpu(1,Some(b"Lsi Logic L64814\0")),fpu(2,Some(b"Texas Instruments TMS390-C602A\0")),fpu(3,Some(b"Cypress CY7C602 FPU\0")),ZF,ZF,ZF,ZF] },
    ManufacturerInfo { psr_impl: 2, cpu_info: [cpu(0,Some(b"Bipolar Integrated Technology - B5010\0")),ZC,ZC,ZC,ZC,ZC,ZC,ZC], fpu_info:[ZF;8] },
    ManufacturerInfo { psr_impl: 3, cpu_info: [cpu(0,Some(b"LSI Logic Corporation - unknown-type\0")),ZC,ZC,ZC,ZC,ZC,ZC,ZC], fpu_info:[ZF;8] },
    ManufacturerInfo { psr_impl: 4, cpu_info: [cpu(0,Some(b"Texas Instruments, Inc. - SuperSparc-(II)\0")),cpu(1,Some(b"Texas Instruments, Inc. - MicroSparc\0")),cpu(2,Some(b"Texas Instruments, Inc. - MicroSparc II\0")),cpu(3,Some(b"Texas Instruments, Inc. - SuperSparc 51\0")),cpu(4,Some(b"Texas Instruments, Inc. - SuperSparc 61\0")),cpu(5,Some(b"Texas Instruments, Inc. - unknown\0")),ZC,ZC], fpu_info:[fpu(0,Some(b"SuperSparc on-chip FPU\0")),fpu(4,Some(b"TI MicroSparc on chip FPU\0")),ZF,ZF,ZF,ZF,ZF,ZF] },
    ManufacturerInfo { psr_impl: 5, cpu_info:[cpu(0,Some(b"Matsushita - MN10501\0")),ZC,ZC,ZC,ZC,ZC,ZC,ZC], fpu_info:[fpu(0,Some(b"Matsushita MN10501\0")),ZF,ZF,ZF,ZF,ZF,ZF,ZF] },
    ManufacturerInfo { psr_impl: 6, cpu_info:[cpu(0,Some(b"Philips Corporation - unknown\0")),ZC,ZC,ZC,ZC,ZC,ZC,ZC], fpu_info:[ZF;8] },
    ManufacturerInfo { psr_impl: 7, cpu_info:[cpu(0,Some(b"Harvest VLSI Design Center, Inc. - unknown\0")),ZC,ZC,ZC,ZC,ZC,ZC,ZC], fpu_info:[ZF;8] },
    ManufacturerInfo { psr_impl: 8, cpu_info:[cpu(0,Some(b"Systems and Processes Engineering Corporation (SPEC)\0")),ZC,ZC,ZC,ZC,ZC,ZC,ZC], fpu_info:[ZF;8] },
    ManufacturerInfo { psr_impl: 9, cpu_info:[cpu(0,Some(b"Fujitsu or Weitek Power-UP\0")),cpu(1,Some(b"Fujitsu or Weitek Power-UP\0")),cpu(2,Some(b"Fujitsu or Weitek Power-UP\0")),cpu(3,Some(b"Fujitsu or Weitek Power-UP\0")),ZC,ZC,ZC,ZC], fpu_info:[fpu(3,Some(b"Fujitsu or Weitek on-chip FPU\0")),ZF,ZF,ZF,ZF,ZF,ZF,ZF] },
    ManufacturerInfo { psr_impl: 0x0b, cpu_info:[cpu(3,Some(b"LEON\0")),ZC,ZC,ZC,ZC,ZC,ZC,ZC], fpu_info:[fpu(2,Some(b"GRFPU\0")),fpu(3,Some(b"GRFPU-Lite\0")),ZF,ZF,ZF,ZF,ZF,ZF] },
    ManufacturerInfo { psr_impl: 0x17, cpu_info:[cpu_pmu(0x10,b"TI UltraSparc I   (SpitFire)\0",b"ultra12\0"),cpu_pmu(0x11,b"TI UltraSparc II  (BlackBird)\0",b"ultra12\0"),cpu_pmu(0x12,b"TI UltraSparc IIi (Sabre)\0",b"ultra12\0"),cpu_pmu(0x13,b"TI UltraSparc IIe (Hummingbird)\0",b"ultra12\0"),ZC,ZC,ZC,ZC], fpu_info:[fpu(0x10,Some(b"UltraSparc I integrated FPU\0")),fpu(0x11,Some(b"UltraSparc II integrated FPU\0")),fpu(0x12,Some(b"UltraSparc IIi integrated FPU\0")),fpu(0x13,Some(b"UltraSparc IIe integrated FPU\0")),ZF,ZF,ZF,ZF] },
    ManufacturerInfo { psr_impl: 0x22, cpu_info:[cpu_pmu(0x10,b"TI UltraSparc I   (SpitFire)\0",b"ultra12\0"),ZC,ZC,ZC,ZC,ZC,ZC,ZC], fpu_info:[fpu(0x10,Some(b"UltraSparc I integrated FPU\0")),ZF,ZF,ZF,ZF,ZF,ZF,ZF] },
    ManufacturerInfo { psr_impl: 0x3e, cpu_info:[cpu_pmu(0x14,b"TI UltraSparc III (Cheetah)\0",b"ultra3\0"),cpu_pmu(0x15,b"TI UltraSparc III+ (Cheetah+)\0",b"ultra3+\0"),cpu_pmu(0x16,b"TI UltraSparc IIIi (Jalapeno)\0",b"ultra3i\0"),cpu_pmu(0x18,b"TI UltraSparc IV (Jaguar)\0",b"ultra3+\0"),cpu_pmu(0x19,b"TI UltraSparc IV+ (Panther)\0",b"ultra4+\0"),cpu_pmu(0x22,b"TI UltraSparc IIIi+ (Serrano)\0",b"ultra3i\0"),ZC,ZC], fpu_info:[fpu(0x14,Some(b"UltraSparc III integrated FPU\0")),fpu(0x15,Some(b"UltraSparc III+ integrated FPU\0")),fpu(0x16,Some(b"UltraSparc IIIi integrated FPU\0")),fpu(0x18,Some(b"UltraSparc IV integrated FPU\0")),fpu(0x19,Some(b"UltraSparc IV+ integrated FPU\0")),fpu(0x22,Some(b"UltraSparc IIIi+ integrated FPU\0")),ZF,ZF] },
];

pub static mut NCPUS_PROBED: i32 = 0;
pub static mut FSR_STORAGE: u32 = 0;
static mut SPARC_CPU_TYPE: *const u8 = core::ptr::null();
static mut SPARC_FPU_TYPE: *const u8 = core::ptr::null();
pub static mut SPARC_PMU_TYPE: *const u8 = core::ptr::null();

unsafe fn set_cpu_and_fpu(psr_impl: i32, psr_vers: i32, fpu_vers: i32) {
    SPARC_CPU_TYPE = core::ptr::null(); SPARC_FPU_TYPE = core::ptr::null(); SPARC_PMU_TYPE = core::ptr::null();
    let manuf = MANUFACTURER_INFO.iter().find(|m| m.psr_impl == psr_impl);
    if let Some(m) = manuf {
        for c in &m.cpu_info { if c.psr_vers == -1 { break; } if c.psr_vers == psr_vers { SPARC_CPU_TYPE=c.name; SPARC_PMU_TYPE=c.pmu_name; SPARC_FPU_TYPE=b"No FPU\0".as_ptr(); break; } }
        for f in &m.fpu_info { if f.fp_vers == -1 { break; } if f.fp_vers == fpu_vers { SPARC_FPU_TYPE=f.name; break; } }
    }
    if SPARC_CPU_TYPE.is_null() { SPARC_CPU_TYPE=b"Unknown CPU\0".as_ptr(); }
    if SPARC_FPU_TYPE.is_null() { SPARC_FPU_TYPE=b"Unknown FPU\0".as_ptr(); }
    if SPARC_PMU_TYPE.is_null() { SPARC_PMU_TYPE=b"Unknown PMU\0".as_ptr(); }
}

// The following declarations mirror the external kernel interfaces used below.
#[repr(C)] pub struct SeqFile { _private: [u8; 0] }
#[repr(C)] pub struct SeqOperations { pub start: unsafe extern "C" fn(*mut SeqFile,*mut i64)->*mut core::ffi::c_void, pub next: unsafe extern "C" fn(*mut SeqFile,*mut core::ffi::c_void,*mut i64)->*mut core::ffi::c_void, pub stop: unsafe extern "C" fn(*mut SeqFile,*mut core::ffi::c_void), pub show: unsafe extern "C" fn(*mut SeqFile,*mut core::ffi::c_void)->i32 }
unsafe extern "C" { fn num_online_cpus() -> i32; fn mmu_info(_: *mut SeqFile); fn cpucap_info(_: *mut SeqFile); fn smp_bogo(_: *mut SeqFile); fn smp_info(_: *mut SeqFile); }

unsafe extern "C" fn c_start(_: *mut SeqFile, pos: *mut i64) -> *mut core::ffi::c_void {
    if *pos == 0 { c_start as *mut core::ffi::c_void } else { core::ptr::null_mut() }
}
unsafe extern "C" fn c_next(m: *mut SeqFile, _: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void { *pos += 1; c_start(m,pos) }
unsafe extern "C" fn c_stop(_: *mut SeqFile, _: *mut core::ffi::c_void) {}

#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn show_cpuinfo(m: *mut SeqFile, _: *mut core::ffi::c_void) -> i32 {
    // seq_printf formatting and the PROM/CPU data accesses are provided by the kernel.
    let _ = m; 0
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn show_cpuinfo(m: *mut SeqFile, _: *mut core::ffi::c_void) -> i32 {
    let _ = m; cpucap_info(m); mmu_info(m); 0
}

#[no_mangle]
pub static CPUINFO_OP: SeqOperations = SeqOperations { start: c_start, next: c_next, stop: c_stop, show: show_cpuinfo };

#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn cpu_type_probe() -> i32 {
    unsafe extern "C" { fn get_psr() -> u32; fn put_psr(_: u32); fn get_fsr() -> u32; }
    let psr = get_psr();
    let psr_impl = (psr >> 28) & 0xf; let psr_vers = (psr >> 24) & 0xf;
    put_psr(psr | (1 << 12));
    let fpu_vers = (get_fsr() >> 17) & 7;
    put_psr(psr); set_cpu_and_fpu(psr_impl as i32, psr_vers as i32, fpu_vers as i32); 0
}

#[cfg(target_pointer_width = "64")]
unsafe fn sun4v_cpu_probe() {
    unsafe extern "C" { static sun4v_chip_type: i32; }
    let (cpu,fpu,pmu) = match sun4v_chip_type {
        0 => (b"UltraSparc T1 (Niagara)\0",b"UltraSparc T1 integrated FPU\0",b"niagara\0"),
        1 => (b"UltraSparc T2 (Niagara2)\0",b"UltraSparc T2 integrated FPU\0",b"niagara2\0"),
        2 => (b"UltraSparc T3 (Niagara3)\0",b"UltraSparc T3 integrated FPU\0",b"niagara3\0"),
        3 => (b"UltraSparc T4 (Niagara4)\0",b"UltraSparc T4 integrated FPU\0",b"niagara4\0"),
        4 => (b"UltraSparc T5 (Niagara5)\0",b"UltraSparc T5 integrated FPU\0",b"niagara5\0"),
        _ => (b"Unknown SUN4V CPU\0",b"Unknown SUN4V FPU\0",b"Unknown SUN4V PMU\0"),
    }; SPARC_CPU_TYPE=cpu.as_ptr(); SPARC_FPU_TYPE=fpu.as_ptr(); SPARC_PMU_TYPE=pmu.as_ptr();
}

#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn cpu_type_probe() -> i32 {
    unsafe extern "C" { static tlb_type: i32; }
    if tlb_type == 1 { sun4v_cpu_probe(); } else { set_cpu_and_fpu(0,0,0); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
