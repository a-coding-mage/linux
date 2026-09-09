// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies and build-time macros are supplied by the surrounding
// Rust kernel environment.

extern "C" {
    static mut fpuemustats: PerCpu<MipsFpuEmulatorStats>;
    static mut mips_debugfs_dir: *mut Dentry;
    fn local_read(v: *mut Local) -> c_ulong;
    fn debugfs_create_dir(name: *const c_char, parent: *mut Dentry) -> *mut Dentry;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut Dentry,
                           data: *mut c_void, fops: *const FileOperations) -> *mut Dentry;
    fn strscpy(dst: *mut c_char, src: *const c_char, len: usize) -> isize;
}

#[repr(C)]
pub struct Dentry;
#[repr(C)]
pub struct SeqFile;
#[repr(C)]
pub struct Local;
#[repr(C)]
pub struct FileOperations;
#[repr(C)]
pub struct PerCpu<T>(T);
#[repr(C)]
pub struct MipsFpuEmulatorStats {
    pub emulated: Local, pub loads: Local, pub stores: Local, pub branches: Local,
    pub cp1ops: Local, pub cp1xops: Local, pub errors: Local,
    pub ieee754_inexact: Local, pub ieee754_underflow: Local, pub ieee754_overflow: Local,
    pub ieee754_zerodiv: Local, pub ieee754_invalidop: Local, pub ds_emul: Local,
    pub abs_s: Local, pub abs_d: Local, pub add_s: Local, pub add_d: Local,
    pub bc1eqz: Local, pub bc1nez: Local, pub ceil_w_s: Local, pub ceil_w_d: Local,
    pub ceil_l_s: Local, pub ceil_l_d: Local, pub class_s: Local, pub class_d: Local,
    pub cmp_af_s: Local, pub cmp_af_d: Local, pub cmp_eq_s: Local, pub cmp_eq_d: Local,
    pub cmp_le_s: Local, pub cmp_le_d: Local, pub cmp_lt_s: Local, pub cmp_lt_d: Local,
    pub cmp_ne_s: Local, pub cmp_ne_d: Local, pub cmp_or_s: Local, pub cmp_or_d: Local,
    pub cmp_ueq_s: Local, pub cmp_ueq_d: Local, pub cmp_ule_s: Local, pub cmp_ule_d: Local,
    pub cmp_ult_s: Local, pub cmp_ult_d: Local, pub cmp_un_s: Local, pub cmp_un_d: Local,
    pub cmp_une_s: Local, pub cmp_une_d: Local, pub cmp_saf_s: Local, pub cmp_saf_d: Local,
    pub cmp_seq_s: Local, pub cmp_seq_d: Local, pub cmp_sle_s: Local, pub cmp_sle_d: Local,
    pub cmp_slt_s: Local, pub cmp_slt_d: Local, pub cmp_sne_s: Local, pub cmp_sne_d: Local,
    pub cmp_sor_s: Local, pub cmp_sor_d: Local, pub cmp_sueq_s: Local, pub cmp_sueq_d: Local,
    pub cmp_sule_s: Local, pub cmp_sule_d: Local, pub cmp_sult_s: Local, pub cmp_sult_d: Local,
    pub cmp_sun_s: Local, pub cmp_sun_d: Local, pub cmp_sune_s: Local, pub cmp_sune_d: Local,
    pub cvt_d_l: Local, pub cvt_d_s: Local, pub cvt_d_w: Local, pub cvt_l_s: Local,
    pub cvt_l_d: Local, pub cvt_s_d: Local, pub cvt_s_l: Local, pub cvt_s_w: Local,
    pub cvt_w_s: Local, pub cvt_w_d: Local, pub div_s: Local, pub div_d: Local,
    pub floor_w_s: Local, pub floor_w_d: Local, pub floor_l_s: Local, pub floor_l_d: Local,
    pub maddf_s: Local, pub maddf_d: Local, pub max_s: Local, pub max_d: Local,
    pub maxa_s: Local, pub maxa_d: Local, pub min_s: Local, pub min_d: Local,
    pub mina_s: Local, pub mina_d: Local, pub mov_s: Local, pub mov_d: Local,
    pub msubf_s: Local, pub msubf_d: Local, pub mul_s: Local, pub mul_d: Local,
    pub neg_s: Local, pub neg_d: Local, pub recip_s: Local, pub recip_d: Local,
    pub rint_s: Local, pub rint_d: Local, pub round_w_s: Local, pub round_w_d: Local,
    pub round_l_s: Local, pub round_l_d: Local, pub rsqrt_s: Local, pub rsqrt_d: Local,
    pub sel_s: Local, pub sel_d: Local, pub seleqz_s: Local, pub seleqz_d: Local,
    pub selnez_s: Local, pub selnez_d: Local, pub sqrt_s: Local, pub sqrt_d: Local,
    pub sub_s: Local, pub sub_d: Local, pub trunc_w_s: Local, pub trunc_w_d: Local,
    pub trunc_l_s: Local, pub trunc_l_d: Local,
}

type c_char = i8; type c_void = core::ffi::c_void; type c_uint = u32; type c_ulong = usize;
type u64_ = u64;

macro_rules! for_each_online_cpu { ($cpu:ident, $body:block) => {{ /* supplied by kernel */ $body }} }
macro_rules! per_cpu { ($v:ident, $cpu:expr) => { core::ptr::addr_of_mut!($v) as *mut MipsFpuEmulatorStats } }
macro_rules! this_cpu_write { ($field:ident, $value:expr) => { unsafe { (*core::ptr::addr_of_mut!(fpuemustats.0)).$field = $value; } } }

unsafe fn fpuemu_stat_get(data: *mut c_void, val: *mut u64_) -> i32 {
    let mut sum: c_ulong = 0;
    let mut cpu: i32 = 0;
    for_each_online_cpu!(cpu, {
        let ps = per_cpu!(fpuemustats, cpu);
        let pv = (ps as *mut u8).offset(data as isize) as *mut Local;
        sum = sum.wrapping_add(local_read(pv));
    });
    *val = sum as u64_;
    0
}

// Used to obtain names for a debugfs instruction counter, given field name
// in fpuemustats structure. For example, for input "cmp_sueq_d", the output
// would be "cmp.sueq.d". This is needed since dots are not allowed to be
// used in structure field names, and are, on the other hand, desired to be
// used in debugfs item names to be clearly associated to corresponding
// MIPS FPU instructions.
unsafe fn adjust_instruction_counter_name(out_name: *mut c_char, in_name: *const c_char, len: usize) {
    strscpy(out_name, in_name, len);
    let mut i = 0usize;
    while *in_name.add(i) != 0 {
        if *out_name.add(i) == b'_' as c_char { *out_name.add(i) = b'.' as c_char; }
        i += 1;
    }
}

unsafe fn fpuemustats_clear_show(_s: *mut SeqFile, _unused: *mut c_void) -> i32 {
    macro_rules! clear { ($($f:ident),* $(,)?) => { $(this_cpu_write!($f, unsafe { core::mem::zeroed() });)* } }
    clear!(emulated, loads, stores, branches, cp1ops, cp1xops, errors, ieee754_inexact,
        ieee754_underflow, ieee754_overflow, ieee754_zerodiv, ieee754_invalidop, ds_emul,
        abs_s, abs_d, add_s, add_d, bc1eqz, bc1nez, ceil_w_s, ceil_w_d, ceil_l_s, ceil_l_d,
        class_s, class_d, cmp_af_s, cmp_af_d, cmp_eq_s, cmp_eq_d, cmp_le_s, cmp_le_d,
        cmp_lt_s, cmp_lt_d, cmp_ne_s, cmp_ne_d, cmp_or_s, cmp_or_d, cmp_ueq_s, cmp_ueq_d,
        cmp_ule_s, cmp_ule_d, cmp_ult_s, cmp_ult_d, cmp_un_s, cmp_un_d, cmp_une_s, cmp_une_d,
        cmp_saf_s, cmp_saf_d, cmp_seq_s, cmp_seq_d, cmp_sle_s, cmp_sle_d, cmp_slt_s, cmp_slt_d,
        cmp_sne_s, cmp_sne_d, cmp_sor_s, cmp_sor_d, cmp_sueq_s, cmp_sueq_d, cmp_sule_s, cmp_sule_d,
        cmp_sult_s, cmp_sult_d, cmp_sun_s, cmp_sun_d, cmp_sune_s, cmp_sune_d,
        cvt_d_l, cvt_d_s, cvt_d_w, cvt_l_s, cvt_l_d, cvt_s_d, cvt_s_l, cvt_s_w, cvt_w_s, cvt_w_d,
        div_s, div_d, floor_w_s, floor_w_d, floor_l_s, floor_l_d, maddf_s, maddf_d, max_s, max_d,
        maxa_s, maxa_d, min_s, min_d, mina_s, mina_d, mov_s, mov_d, msubf_s, msubf_d, mul_s, mul_d,
        neg_s, neg_d, recip_s, recip_d, rint_s, rint_d, round_w_s, round_w_d, round_l_s, round_l_d,
        rsqrt_s, rsqrt_d, sel_s, sel_d, seleqz_s, seleqz_d, selnez_s, selnez_d, sqrt_s, sqrt_d,
        sub_s, sub_d, trunc_w_s, trunc_w_d, trunc_l_s, trunc_l_d);
    0
}

unsafe fn debugfs_fpuemu() -> i32 {
    let base = debugfs_create_dir(b"fpuemustats\0".as_ptr() as *const c_char, mips_debugfs_dir);
    debugfs_create_file(b"fpuemustats_clear\0".as_ptr() as *const c_char, 0o444,
        mips_debugfs_dir, core::ptr::null_mut(), core::ptr::null());
    let stats = ["emulated", "loads", "stores", "branches", "cp1ops", "cp1xops", "errors",
        "ieee754_inexact", "ieee754_underflow", "ieee754_overflow", "ieee754_zerodiv",
        "ieee754_invalidop", "ds_emul"];
    for name in stats {
        let mut n = [0i8; 32];
        for (i, b) in name.bytes().enumerate() { n[i] = b as i8; }
        debugfs_create_file(n.as_ptr(), 0o444, base, core::ptr::null_mut(), core::ptr::null());
    }
    let inst = debugfs_create_dir(b"instructions\0".as_ptr() as *const c_char, base);
    let instructions = [
        "abs_s","abs_d","add_s","add_d","bc1eqz","bc1nez","ceil_w_s","ceil_w_d","ceil_l_s","ceil_l_d",
        "class_s","class_d","cmp_af_s","cmp_af_d","cmp_eq_s","cmp_eq_d","cmp_le_s","cmp_le_d","cmp_lt_s","cmp_lt_d",
        "cmp_ne_s","cmp_ne_d","cmp_or_s","cmp_or_d","cmp_ueq_s","cmp_ueq_d","cmp_ule_s","cmp_ule_d","cmp_ult_s","cmp_ult_d",
        "cmp_un_s","cmp_un_d","cmp_une_s","cmp_une_d","cmp_saf_s","cmp_saf_d","cmp_seq_s","cmp_seq_d","cmp_sle_s","cmp_sle_d",
        "cmp_slt_s","cmp_slt_d","cmp_sne_s","cmp_sne_d","cmp_sor_s","cmp_sor_d","cmp_sueq_s","cmp_sueq_d","cmp_sule_s","cmp_sule_d",
        "cmp_sult_s","cmp_sult_d","cmp_sun_s","cmp_sun_d","cmp_sune_s","cmp_sune_d","cvt_d_l","cvt_d_s","cvt_d_w","cvt_l_s",
        "cvt_l_d","cvt_s_d","cvt_s_l","cvt_s_w","cvt_w_s","cvt_w_d","div_s","div_d","floor_w_s","floor_w_d","floor_l_s","floor_l_d",
        "maddf_s","maddf_d","max_s","max_d","maxa_s","maxa_d","min_s","min_d","mina_s","mina_d","mov_s","mov_d","msubf_s","msubf_d",
        "mul_s","mul_d","neg_s","neg_d","recip_s","recip_d","rint_s","rint_d","round_w_s","round_w_d","round_l_s","round_l_d",
        "rsqrt_s","rsqrt_d","sel_s","sel_d","seleqz_s","seleqz_d","selnez_s","selnez_d","sqrt_s","sqrt_d","sub_s","sub_d",
        "trunc_w_s","trunc_w_d","trunc_l_s","trunc_l_d"];
    for name in instructions {
        let mut n = [0i8; 32];
        for (i, b) in name.bytes().enumerate() { n[i] = if b == b'_' { b'.' as i8 } else { b as i8 }; }
        debugfs_create_file(n.as_ptr(), 0o444, inst, core::ptr::null_mut(), core::ptr::null());
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
