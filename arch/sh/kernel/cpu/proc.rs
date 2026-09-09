// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

// Declarations supplied by the corresponding kernel headers/build environment.
extern "C" {
    static cpu_data: *mut sh_cpuinfo;
    static nr_cpu_ids: loff_t;
    fn cpu_online(cpu: c_uint) -> bool;
    fn get_system_type() -> *const c_char;
    fn init_utsname() -> *mut utsname;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
}

type loff_t = i64;

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct utsname {
    pub machine: [c_char; 65],
}

#[repr(C)]
pub struct cache_info {
    pub flags: c_uint,
    pub ways: c_uint,
    pub sets: c_uint,
    pub linesz: c_uint,
}

#[repr(C)]
pub struct sh_cpuinfo {
    pub type_: c_uint,
    pub flags: c_ulong,
    pub cut_major: c_int,
    pub cut_minor: c_int,
    pub icache: cache_info,
    pub dcache: cache_info,
    pub scache: cache_info,
    pub phys_bits: c_uint,
    pub loops_per_jiffy: c_ulong,
}

extern "C" {
    static HZ: c_ulong;
}

// CPU_* constants and cache flags are supplied by asm/machvec.h and
// asm/processor.h. The table is represented as indexed entries.
static CPU_NAME: &[(c_uint, &str)] = &[
    (CPU_SH7201, "SH7201"), (CPU_SH7203, "SH7203"),
    (CPU_SH7263, "SH7263"), (CPU_SH7264, "SH7264"),
    (CPU_SH7269, "SH7269"), (CPU_SH7206, "SH7206"),
    (CPU_SH7619, "SH7619"), (CPU_SH7705, "SH7705"),
    (CPU_SH7706, "SH7706"), (CPU_SH7707, "SH7707"),
    (CPU_SH7708, "SH7708"), (CPU_SH7709, "SH7709"),
    (CPU_SH7710, "SH7710"), (CPU_SH7712, "SH7712"),
    (CPU_SH7720, "SH7720"), (CPU_SH7721, "SH7721"),
    (CPU_SH7729, "SH7729"), (CPU_SH7750, "SH7750"),
    (CPU_SH7750S, "SH7750S"), (CPU_SH7750R, "SH7750R"),
    (CPU_SH7751, "SH7751"), (CPU_SH7751R, "SH7751R"),
    (CPU_SH7760, "SH7760"), (CPU_SH4_202, "SH4-202"),
    (CPU_SH4_501, "SH4-501"), (CPU_SH7763, "SH7763"),
    (CPU_SH7770, "SH7770"), (CPU_SH7780, "SH7780"),
    (CPU_SH7781, "SH7781"), (CPU_SH7343, "SH7343"),
    (CPU_SH7785, "SH7785"), (CPU_SH7786, "SH7786"),
    (CPU_SH7757, "SH7757"), (CPU_SH7722, "SH7722"),
    (CPU_SHX3, "SH-X3"), (CPU_MXG, "MX-G"),
    (CPU_SH7723, "SH7723"), (CPU_SH7366, "SH7366"),
    (CPU_SH7724, "SH7724"), (CPU_SH7372, "SH7372"),
    (CPU_SH7734, "SH7734"), (CPU_J2, "J2"),
    (CPU_SH_NONE, "Unknown"),
];

#[no_mangle]
pub unsafe extern "C" fn get_cpu_subtype(c: *const sh_cpuinfo) -> *const c_char {
    let cpu_type = (*c).type_;
    for &(kind, name) in CPU_NAME {
        if kind == cpu_type {
            return name.as_ptr() as *const c_char;
        }
    }
    core::ptr::null()
}

// CONFIG_PROC_FS controls the following procfs implementation.
#[cfg(feature = "CONFIG_PROC_FS")]
mod proc_fs {
    use super::*;

    static CPU_FLAGS: &[Option<&str>] = &[
        Some("none"), Some("fpu"), Some("p2flush"), Some("mmuassoc"),
        Some("dsp"), Some("perfctr"), Some("ptea"), Some("llsc"),
        Some("l2"), Some("op32"), Some("pteaex"), None,
    ];

    unsafe fn show_cpuflags(m: *mut seq_file, c: *const sh_cpuinfo) {
        seq_printf(m, c"cpu flags\t:".as_ptr(),);
        if (*c).flags == 0 {
            seq_printf(m, c" %s\n".as_ptr(), CPU_FLAGS[0].unwrap().as_ptr());
            return;
        }
        let mut i = 0usize;
        while CPU_FLAGS[i].is_some() {
            if ((*c).flags & (1u64 << i)) != 0 {
                seq_printf(m, c" %s".as_ptr(), CPU_FLAGS[i + 1].unwrap().as_ptr());
            }
            i += 1;
        }
        seq_printf(m, c"\n".as_ptr(),);
    }

    unsafe fn show_cacheinfo(m: *mut seq_file, type_: *const c_char, info: cache_info) {
        let cache_size = info.ways * info.sets * info.linesz;
        seq_printf(m, c"%s size\t: %2dKiB (%d-way)\n".as_ptr(), type_, cache_size >> 10, info.ways);
    }

    unsafe extern "C" fn show_cpuinfo(m: *mut seq_file, v: *mut c_void) -> c_int {
        let c = v as *mut sh_cpuinfo;
        let cpu = c.offset_from(cpu_data) as c_uint;
        if !cpu_online(cpu) { return 0; }
        if cpu == 0 { seq_printf(m, c"machine\t\t: %s\n".as_ptr(), get_system_type()); }
        else { seq_printf(m, c"\n".as_ptr(),); }
        seq_printf(m, c"processor\t: %d\n".as_ptr(), cpu);
        seq_printf(m, c"cpu family\t: %s\n".as_ptr(), (*init_utsname()).machine.as_ptr());
        seq_printf(m, c"cpu type\t: %s\n".as_ptr(), get_cpu_subtype(c));
        if (*c).cut_major == -1 { seq_printf(m, c"cut\t\t: unknown\n".as_ptr(),); }
        else if (*c).cut_minor == -1 { seq_printf(m, c"cut\t\t: %d.x\n".as_ptr(), (*c).cut_major); }
        else { seq_printf(m, c"cut\t\t: %d.%d\n".as_ptr(), (*c).cut_major, (*c).cut_minor); }
        show_cpuflags(m, c);
        seq_printf(m, c"cache type\t: ".as_ptr(),);
        if ((*c).icache.flags & SH_CACHE_COMBINED) != 0 {
            seq_printf(m, c"unified\n".as_ptr(),); show_cacheinfo(m, c"cache".as_ptr(), (*c).icache);
        } else {
            seq_printf(m, c"split (harvard)\n".as_ptr(),); show_cacheinfo(m, c"icache".as_ptr(), (*c).icache); show_cacheinfo(m, c"dcache".as_ptr(), (*c).dcache);
        }
        if ((*c).flags & CPU_HAS_L2_CACHE) != 0 { show_cacheinfo(m, c"scache".as_ptr(), (*c).scache); }
        seq_printf(m, c"address sizes\t: %u bits physical\n".as_ptr(), (*c).phys_bits);
        seq_printf(m, c"bogomips\t: %lu.%02lu\n".as_ptr(), (*c).loops_per_jiffy / (500000 / HZ), ((*c).loops_per_jiffy / (5000 / HZ)) % 100);
        0
    }

    unsafe extern "C" fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
        if *pos < nr_cpu_ids { cpu_data.offset(*pos as isize) as *mut c_void } else { core::ptr::null_mut() }
    }
    unsafe extern "C" fn c_next(m: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void { *pos += 1; c_start(m, pos) }
    unsafe extern "C" fn c_stop(_m: *mut seq_file, _v: *mut c_void) {}

    #[repr(C)]
    pub struct seq_operations {
        pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut c_void>,
        pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut loff_t) -> *mut c_void>,
        pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>,
        pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
    }

    #[no_mangle]
    pub static CPUINFO_OP: seq_operations = seq_operations {
        start: Some(c_start),
        next: Some(c_next),
        stop: Some(c_stop),
        show: Some(show_cpuinfo),
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
