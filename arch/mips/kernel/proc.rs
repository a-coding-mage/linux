// SPDX-License-Identifier: GPL-2.0
/* Translated from proc.c. Kernel includes and configuration symbols are external dependencies. */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct proc_cpuinfo_notifier_args { pub m: *mut seq_file, pub n: c_ulong }
#[repr(C)] pub struct cpuinfo { pub processor_id: u32, pub fpu_id: u32, pub options: u64, pub udelay_val: u32, pub tlbsize: i32, pub watch_reg_count: i32, pub watch_reg_masks: *const u32, pub srsets: i32, pub kscratch_mask: u8, pub package: i32 }
extern "C" {
    pub static mut vced_count: u32;
    pub static mut vcei_count: u32;
    static mut proc_cpuinfo_chain: c_void;
    static mut cpu_data: [cpuinfo; 0];
    static __cpu_name: [*const c_char; 0];
    static nr_cpu_ids: c_ulong;
    fn raw_notifier_chain_register(h: *mut c_void, nb: *mut notifier_block) -> c_int;
    fn raw_notifier_call_chain(h: *mut c_void, val: c_ulong, v: *mut c_void) -> c_int;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn seq_putc(m: *mut seq_file, c: c_int);
    fn get_system_type() -> *const c_char;
    fn mips_get_machine_name() -> *const c_char;
    fn str_yes_no(v: bool) -> *const c_char;
    fn read_c0_config3() -> u32;
    fn hweight8(v: u8) -> u32;
    fn cpu_core(c: *const cpuinfo) -> i32;
    fn cpu_vpe_id(c: *const cpuinfo) -> i32;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...);
}

pub const MIPS_CPU_FPU: u64 = 1; // supplied by asm/cpu.h

#[no_mangle]
pub unsafe extern "C" fn register_proc_cpuinfo_notifier(nb: *mut notifier_block) -> c_int {
    raw_notifier_chain_register(&mut proc_cpuinfo_chain, nb)
}

#[no_mangle]
pub unsafe extern "C" fn proc_cpuinfo_notifier_call_chain(val: c_ulong, v: *mut c_void) -> c_int {
    raw_notifier_call_chain(&mut proc_cpuinfo_chain, val, v)
}

/* CPU feature predicates are supplied by asm/cpu-features.h. */
macro_rules! feature { ($name:ident) => { extern "C" { static $name: bool; } }; }
feature!(cpu_wait); feature!(cpu_has_counter); feature!(cpu_has_divec); feature!(cpu_has_watch);
feature!(cpu_has_mips_1); feature!(cpu_has_mips_2); feature!(cpu_has_mips_3); feature!(cpu_has_mips_4); feature!(cpu_has_mips_5);
feature!(cpu_has_mips32r1); feature!(cpu_has_mips32r2); feature!(cpu_has_mips32r5); feature!(cpu_has_mips32r6);
feature!(cpu_has_mips64r1); feature!(cpu_has_mips64r2); feature!(cpu_has_mips64r5); feature!(cpu_has_mips64r6);
feature!(cpu_has_mips16); feature!(cpu_has_mips16e2); feature!(cpu_has_mdmx); feature!(cpu_has_mips3d); feature!(cpu_has_smartmips);
feature!(cpu_has_dsp); feature!(cpu_has_dsp2); feature!(cpu_has_dsp3); feature!(cpu_has_mipsmt); feature!(cpu_has_mmips); feature!(cpu_has_vz); feature!(cpu_has_msa); feature!(cpu_has_eva); feature!(cpu_has_htw); feature!(cpu_has_xpa); feature!(cpu_has_loongson_mmi); feature!(cpu_has_loongson_cam); feature!(cpu_has_loongson_ext); feature!(cpu_has_loongson_ext2);
feature!(cpu_has_tlb); feature!(cpu_has_ftlb); feature!(cpu_has_tlbinv); feature!(cpu_has_segments); feature!(cpu_has_rixiex); feature!(cpu_has_ldpte); feature!(cpu_has_maar); feature!(cpu_has_rw_llb); feature!(cpu_has_4kex); feature!(cpu_has_3k_cache); feature!(cpu_has_4k_cache); feature!(cpu_has_octeon_cache); feature!(raw_cpu_has_fpu); feature!(cpu_has_32fpr); feature!(cpu_has_cache_cdex_p); feature!(cpu_has_cache_cdex_s); feature!(cpu_has_prefetch); feature!(cpu_has_mcheck); feature!(cpu_has_ejtag); feature!(cpu_has_llsc); feature!(cpu_has_guestctl0ext); feature!(cpu_has_guestctl1); feature!(cpu_has_guestctl2); feature!(cpu_has_guestid); feature!(cpu_has_drg); feature!(cpu_has_rixi); feature!(cpu_has_lpa); feature!(cpu_has_mvh); feature!(cpu_has_vtag_icache); feature!(cpu_has_dc_aliases); feature!(cpu_has_ic_fills_f_dc); feature!(cpu_has_pindexed_dcache); feature!(cpu_has_userlocal); feature!(cpu_has_nofpuex); feature!(cpu_has_vint); feature!(cpu_has_veic); feature!(cpu_has_inclusive_pcaches); feature!(cpu_has_perf_cntr_intr_bit); feature!(cpu_has_ufr); feature!(cpu_has_fre); feature!(cpu_has_cdmm); feature!(cpu_has_small_pages); feature!(cpu_has_nan_legacy); feature!(cpu_has_nan_2008); feature!(cpu_has_ebase_wg); feature!(cpu_has_badinstr); feature!(cpu_has_badinstrp); feature!(cpu_has_contextconfig); feature!(cpu_has_perf); feature!(cpu_has_mac2008_only); feature!(cpu_has_ftlbparex); feature!(cpu_has_gsexcex); feature!(cpu_has_shared_ftlb_ram); feature!(cpu_has_shared_ftlb_entries); feature!(cpu_has_mipsmt_pertccounters); feature!(cpu_has_mmid); feature!(cpu_has_mm_sysad); feature!(cpu_has_mm_full);

unsafe fn put_features(m: *mut seq_file, label: *const c_char, names: &[(&'static bool, &'static [u8])]) {
    seq_puts(m, label);
    for (flag, name) in names { if **flag { seq_puts(m, name.as_ptr() as *const c_char); } }
    seq_putc(m, b'\n' as c_int);
}

unsafe extern "C" fn show_cpuinfo(m: *mut seq_file, v: *mut c_void) -> c_int {
    let n = (v as c_ulong).wrapping_sub(1) as usize;
    let c = &cpu_data[n];
    let version = c.processor_id;
    let fp_vers = c.fpu_id;
    let mut fmt = [0i8; 64];
    seq_printf(m, b"processor\t\t: %ld\n\0".as_ptr() as *const c_char, n as c_ulong);
    sprintf(fmt.as_mut_ptr(), b"cpu model\t\t: %%s V%%d.%%d%s\n\0".as_ptr() as *const c_char, if c.options & MIPS_CPU_FPU != 0 { b"  FPU V%d.%d\0".as_ptr() } else { b"\0".as_ptr() });
    seq_printf(m, fmt.as_ptr(), __cpu_name[n], (version >> 4) & 0xf, version & 0xf, (fp_vers >> 4) & 0xf, fp_vers & 0xf);
    seq_printf(m, b"BogoMIPS\t\t: %u.%02u\n\0".as_ptr() as *const c_char, c.udelay_val / (500000 / 100), (c.udelay_val / (5000 / 100)) % 100);
    seq_printf(m, b"tlb_entries\t\t: %d\n\0".as_ptr() as *const c_char, c.tlbsize);
    seq_printf(m, b"shadow register sets\t: %d\n\0".as_ptr() as *const c_char, c.srsets);
    seq_printf(m, b"kscratch registers\t: %d\n\0".as_ptr() as *const c_char, hweight8(c.kscratch_mask));
    seq_printf(m, b"package\t\t\t: %d\n\0".as_ptr() as *const c_char, c.package);
    seq_printf(m, b"core\t\t\t: %d\n\0".as_ptr() as *const c_char, cpu_core(c));
    sprintf(fmt.as_mut_ptr(), b"VCE%%c exceptions\t\t: %s\n\0".as_ptr() as *const c_char, if cpu_has_vce() { b"%u\0".as_ptr() } else { b"not available\0".as_ptr() });
    seq_printf(m, fmt.as_ptr(), b'D' as c_int, vced_count);
    seq_printf(m, fmt.as_ptr(), b'I' as c_int, vcei_count);
    let mut args = proc_cpuinfo_notifier_args { m, n: n as c_ulong };
    raw_notifier_call_chain(&mut proc_cpuinfo_chain, 0, &mut args as *mut _ as *mut c_void);
    seq_putc(m, b'\n' as c_int); 0
}

extern "C" { fn cpu_has_vce() -> bool; }

unsafe extern "C" fn c_start(_m: *mut seq_file, pos: *mut i64) -> *mut c_void { let i = *pos as c_ulong; if i < nr_cpu_ids { (i + 1) as *mut c_void } else { core::ptr::null_mut() } }
unsafe extern "C" fn c_next(m: *mut seq_file, _v: *mut c_void, pos: *mut i64) -> *mut c_void { *pos += 1; c_start(m, pos) }
unsafe extern "C" fn c_stop(_m: *mut seq_file, _v: *mut c_void) {}

#[repr(C)] pub struct seq_operations { pub start: unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut c_void, pub next: unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut i64) -> *mut c_void, pub stop: unsafe extern "C" fn(*mut seq_file, *mut c_void), pub show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int }
#[no_mangle] pub static cpuinfo_op: seq_operations = seq_operations { start: c_start, next: c_next, stop: c_stop, show: show_cpuinfo };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
