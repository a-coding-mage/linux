/*
 * Rust translation of asm/setup.h.
 * C preprocessor configuration branches are represented with cfg! feature
 * checks; symbols from the included UAPI headers remain external dependencies.
 */

pub const CL_SIZE: usize = COMMAND_LINE_SIZE;

unsafe extern "C" {
    pub static mut m68k_machtype: ::core::ffi::c_ulong;
    pub static mut m68k_cputype: ::core::ffi::c_ulong;
    pub static mut m68k_fputype: ::core::ffi::c_ulong;
    pub static mut m68k_mmutype: ::core::ffi::c_ulong;
    #[cfg(feature = "CONFIG_VME")]
    pub static mut vme_brdtype: ::core::ffi::c_ulong;
    pub static mut m68k_is040or060: ::core::ffi::c_int;
    pub static mut m68k_num_memory: ::core::ffi::c_int;
    pub static mut m68k_realnum_memory: ::core::ffi::c_int;
    pub static mut m68k_memory: [m68k_mem_info; NUM_MEMINFO];
}

#[repr(C)]
pub struct m68k_mem_info {
    pub addr: ::core::ffi::c_ulong,
    pub size: ::core::ffi::c_ulong,
}

pub const NUM_MEMINFO: usize = 4;

// The following macros preserve the original CONFIG_* conditional intent.
macro_rules! MACH_IS_AMIGA { () => {{
    if !cfg!(feature = "CONFIG_AMIGA") { 0 }
    else if cfg!(any(feature = "CONFIG_ATARI", feature = "CONFIG_MAC", feature = "CONFIG_APOLLO", feature = "CONFIG_MVME16x", feature = "CONFIG_BVME6000", feature = "CONFIG_HP300", feature = "CONFIG_Q40", feature = "CONFIG_SUN3X", feature = "CONFIG_MVME147", feature = "CONFIG_VIRT") { unsafe { (m68k_machtype == MACH_AMIGA) as _ } }
    else { 1 }
}}; }
macro_rules! MACH_IS_ATARI { () => {{ if !cfg!(feature="CONFIG_ATARI") {0} else if cfg!(any(feature="CONFIG_AMIGA",feature="CONFIG_MAC",feature="CONFIG_APOLLO",feature="CONFIG_MVME16x",feature="CONFIG_BVME6000",feature="CONFIG_HP300",feature="CONFIG_Q40",feature="CONFIG_SUN3X",feature="CONFIG_MVME147",feature="CONFIG_VIRT")) { unsafe {(m68k_machtype == MACH_ATARI) as _} } else {1} }}; }
macro_rules! MACH_IS_MAC { () => {{ if !cfg!(feature="CONFIG_MAC") {0} else if cfg!(any(feature="CONFIG_AMIGA",feature="CONFIG_ATARI",feature="CONFIG_APOLLO",feature="CONFIG_MVME16x",feature="CONFIG_BVME6000",feature="CONFIG_HP300",feature="CONFIG_Q40",feature="CONFIG_SUN3X",feature="CONFIG_MVME147",feature="CONFIG_VIRT")) { unsafe {(m68k_machtype == MACH_MAC) as _} } else {1} }}; }

macro_rules! MACH_IS_SUN3 { () => { if cfg!(feature="CONFIG_SUN3") { 1 } else { 0 } }; }
macro_rules! MACH_IS_APOLLO { () => { if cfg!(feature="CONFIG_APOLLO") { unsafe {(m68k_machtype == MACH_APOLLO) as _} } else { 0 } }; }
macro_rules! MACH_IS_MVME147 { () => { if cfg!(feature="CONFIG_MVME147") { unsafe {(m68k_machtype == MACH_MVME147) as _} } else { 0 } }; }
macro_rules! MACH_IS_MVME16x { () => { if cfg!(feature="CONFIG_MVME16x") { unsafe {(m68k_machtype == MACH_MVME16x) as _} } else { 0 } }; }
macro_rules! MACH_IS_BVME6000 { () => { if cfg!(feature="CONFIG_BVME6000") { unsafe {(m68k_machtype == MACH_BVME6000) as _} } else { 0 } }; }
macro_rules! MACH_IS_HP300 { () => { if cfg!(feature="CONFIG_HP300") { unsafe {(m68k_machtype == MACH_HP300) as _} } else { 0 } }; }
macro_rules! MACH_IS_Q40 { () => { if cfg!(feature="CONFIG_Q40") { unsafe {(m68k_machtype == MACH_Q40) as _} } else { 0 } }; }
macro_rules! MACH_IS_SUN3X { () => { if cfg!(feature="CONFIG_SUN3X") { unsafe {(m68k_machtype == MACH_SUN3X) as _} } else { 0 } }; }
macro_rules! MACH_IS_VIRT { () => { if cfg!(feature="CONFIG_VIRT") { unsafe {(m68k_machtype == MACH_VIRT) as _} } else { 0 } }; }

macro_rules! MACH_TYPE { () => { unsafe { m68k_machtype } }; }

macro_rules! CPU_IS_020 { () => { if cfg!(feature="CONFIG_M68020") { unsafe { m68k_cputype & CPU_68020 } } else { 0 } }; }
macro_rules! CPU_IS_030 { () => { if cfg!(feature="CONFIG_M68030") { unsafe { m68k_cputype & CPU_68030 } } else { 0 } }; }
macro_rules! CPU_IS_040 { () => { if cfg!(feature="CONFIG_M68040") { unsafe { m68k_cputype & CPU_68040 } } else { 0 } }; }
macro_rules! CPU_IS_060 { () => { if cfg!(feature="CONFIG_M68060") { unsafe { m68k_cputype & CPU_68060 } } else { 0 } }; }
macro_rules! MMU_IS_851 { () => { if cfg!(feature="CONFIG_M68020") { unsafe { m68k_mmutype & MMU_68851 } } else { 0 } }; }
macro_rules! MMU_IS_030 { () => { if cfg!(feature="CONFIG_M68030") { unsafe { m68k_mmutype & MMU_68030 } } else { 0 } }; }
macro_rules! MMU_IS_040 { () => { if cfg!(feature="CONFIG_M68040") { unsafe { m68k_mmutype & MMU_68040 } } else { 0 } }; }
macro_rules! CPU_IS_020_OR_030 { () => { if cfg!(any(feature="CONFIG_M68020", feature="CONFIG_M68030")) { if cfg!(any(feature="CONFIG_M68040", feature="CONFIG_M68060")) { unsafe { (!m68k_is040or060) as _ } } else { 1 } } else { 0 } }; }
macro_rules! CPU_IS_040_OR_060 { () => { if cfg!(any(feature="CONFIG_M68040", feature="CONFIG_M68060")) { if cfg!(any(feature="CONFIG_M68020", feature="CONFIG_M68030")) { unsafe { m68k_is040or060 } } else { 1 } } else { 0 } }; }
macro_rules! CPU_IS_COLDFIRE { () => { if cfg!(feature="CONFIG_COLDFIRE") { 1 } else { 0 } }; }
macro_rules! MMU_IS_COLDFIRE { () => { if cfg!(feature="CONFIG_COLDFIRE") { 1 } else { 0 } }; }
macro_rules! CPU_TYPE { () => { unsafe { m68k_cputype } }; }
macro_rules! FPU_IS_EMU { () => { if cfg!(feature="CONFIG_M68KFPU_EMU") { if cfg!(feature="CONFIG_M68KFPU_EMU_ONLY") { 1 } else { unsafe { (m68k_fputype == 0) as _ } } } else { 0 } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
