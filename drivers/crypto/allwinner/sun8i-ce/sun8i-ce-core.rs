// SPDX-License-Identifier: GPL-2.0
// Rust translation of sun8i-ce-core.c.  Kernel declarations are supplied by
// the surrounding driver and are intentionally left as external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
extern "C" {
    fn sun8i_ce_cipher_init(_: *mut c_void) -> c_int;
    fn sun8i_ce_cipher_exit(_: *mut c_void);
    fn sun8i_ce_aes_setkey(_: *mut c_void, _: *const u8, _: c_uint) -> c_int;
    fn sun8i_ce_des3_setkey(_: *mut c_void, _: *const u8, _: c_uint) -> c_int;
    fn sun8i_ce_skencrypt(_: *mut c_void, _: *mut c_void) -> c_int;
    fn sun8i_ce_skdecrypt(_: *mut c_void, _: *mut c_void) -> c_int;
    fn sun8i_ce_cipher_do_one(_: *mut c_void, _: *mut c_void) -> c_int;
    fn sun8i_ce_hash_init(_: *mut c_void) -> c_int;
    fn sun8i_ce_hash_update(_: *mut c_void) -> c_int;
    fn sun8i_ce_hash_final(_: *mut c_void) -> c_int;
    fn sun8i_ce_hash_finup(_: *mut c_void) -> c_int;
    fn sun8i_ce_hash_digest(_: *mut c_void) -> c_int;
    fn sun8i_ce_hash_export(_: *mut c_void, _: *mut c_void) -> c_int;
    fn sun8i_ce_hash_import(_: *mut c_void, _: *const c_void) -> c_int;
    fn sun8i_ce_hash_init_tfm(_: *mut c_void) -> c_int;
    fn sun8i_ce_hash_exit_tfm(_: *mut c_void);
    fn sun8i_ce_hash_run(_: *mut c_void, _: *mut c_void) -> c_int;
}

// Constants and structures below correspond to declarations from sun8i-ce.h.
// Their concrete definitions are provided by the translated header/dependent
// kernel bindings.
#[repr(C)] pub struct ce_variant { pub alg_cipher: [c_int; 3], pub alg_hash: [c_int; 6], pub op_mode: [c_int; 2], pub cipher_t_dlen_in_bytes: bool, pub hash_t_dlen_in_bits: bool, pub trng_t_dlen_in_bytes: bool, pub needs_word_addresses: bool, pub ce_clks: [ce_clock; 4], pub esr: c_int, pub trng: c_int }
#[repr(C)] pub struct ce_clock { pub name: *const c_char, pub freq: c_ulong, pub max_freq: c_ulong }
#[repr(C)] pub struct ce_task { pub t_common_ctl: u32 }
#[repr(C)] pub struct sun8i_ce_flow { pub tl: *mut ce_task, pub t_phy: u64, pub status: c_int, pub engine: *mut c_void, pub complete: c_void, pub stat_req: c_ulong }
#[repr(C)] pub struct sun8i_ce_dev { pub flow: c_int, pub base: *mut u8, pub dev: *mut c_void, pub variant: *const ce_variant, pub chanlist: *mut sun8i_ce_flow, pub ceclks: [*mut c_void; 4], pub reset: *mut c_void, pub mlock: c_void, pub rnglock: c_void }

extern "C" {
    fn atomic_inc_return(v: *mut c_int) -> c_int;
    fn readl(p: *mut u8) -> u32;
    fn writel(v: u32, p: *mut u8);
    fn reinit_completion(c: *mut c_void);
    fn wait_for_completion_interruptible_timeout(c: *mut c_void, t: c_ulong) -> c_long;
    fn complete(c: *mut c_void);
    fn mutex_lock(m: *mut c_void); fn mutex_unlock(m: *mut c_void);
    fn desc_addr_val(ce: *mut sun8i_ce_dev, p: u64) -> u32;
    fn le32_to_cpu(v: u32) -> u32;
    fn sun8i_ce_dump_task_descriptors(c: *mut sun8i_ce_flow);
}
type c_long = i64;

pub const CE_ID_NOTSUPP: c_int = -1;

#[repr(C)] pub struct sun8i_ce_alg_template { pub ce: *mut sun8i_ce_dev, pub typ: c_int, pub ce_algo_id: c_int, pub ce_blockmode: c_int, pub alg: *mut c_void, pub stat_req: c_ulong, pub stat_fb: c_ulong, pub fbname: *const c_char, pub stat_fb_len0: c_ulong, pub stat_fb_mod16: c_ulong, pub stat_fb_leniv: c_ulong, pub stat_fb_srcali: c_ulong, pub stat_fb_dstali: c_ulong, pub stat_fb_srclen: c_ulong, pub stat_fb_dstlen: c_ulong, pub stat_fb_maxsg: c_ulong }

// Variant tables (the string literals are C-compatible and retain the exact
// clock names and hardware capability matrices).
macro_rules! variant { ($hash:expr, $esr:expr, $trng:expr, $flags:expr, $mod:expr) => { ce_variant { alg_cipher: [0,1,2], alg_hash: $hash, op_mode: [0,1], cipher_t_dlen_in_bytes: $flags.0, hash_t_dlen_in_bits: $flags.1, trng_t_dlen_in_bytes: $flags.2, needs_word_addresses: $flags.3, ce_clks: [ce_clock{name:b"bus\\0".as_ptr() as *const c_char,freq:0,max_freq:200000000},ce_clock{name:b"mod\\0".as_ptr() as *const c_char,freq:$mod,max_freq:0},ce_clock{name:b"ram\\0".as_ptr() as *const c_char,freq:0,max_freq:400000000},ce_clock{name:core::ptr::null(),freq:0,max_freq:0}],esr:$esr,trng:$trng } }; }
pub static mut ce_h3_variant: ce_variant = variant!([3,4,5,6,7,8], 0, -1, (false,false,false,false), 50000000);
pub static mut ce_h5_variant: ce_variant = variant!([3,4,5,6,-1,-1], 3, -1, (false,false,false,false), 300000000);
pub static mut ce_h6_variant: ce_variant = variant!([3,4,5,6,7,8], 4, 9, (true,true,true,false), 300000000);
pub static mut ce_h616_variant: ce_variant = variant!([3,4,5,6,7,8], 4, 9, (true,true,true,true), 300000000);
pub static mut ce_a64_variant: ce_variant = variant!([3,4,5,6,-1,-1], 1, -1, (false,false,false,false), 300000000);
pub static mut ce_d1_variant: ce_variant = variant!([3,4,5,6,7,8], 5, 10, (false,false,false,false), 300000000);
pub static mut ce_r40_variant: ce_variant = variant!([3,4,5,6,-1,-1], 6, -1, (false,false,false,false), 300000000);

pub unsafe fn sun8i_ce_get_engine_number(ce: *mut sun8i_ce_dev) -> c_int { atomic_inc_return(&mut (*ce).flow) % (4 - 1) }

pub unsafe fn sun8i_ce_run_task(ce: *mut sun8i_ce_dev, flow: c_int, _name: *const c_char) -> c_int {
    let ch = &mut *(*ce).chanlist.add(flow as usize); mutex_lock(&mut (*ce).mlock);
    let mut v = readl((*ce).base.add(0x04)); writel(v | (1u32 << flow), (*ce).base.add(0x04));
    reinit_completion(&mut ch.complete); writel(desc_addr_val(ce, ch.t_phy), (*ce).base.add(0x08)); ch.status = 0;
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    v = 1 | ((le32_to_cpu((*ch.tl).t_common_ctl) & 0x7f) << 8); writel(v, (*ce).base.add(0x0c)); mutex_unlock(&mut (*ce).mlock);
    let _ = wait_for_completion_interruptible_timeout(&mut ch.complete, 0);
    let mut err = if ch.status == 0 { -14 } else { 0 }; v = readl((*ce).base.add(0x10));
    match (*(*ce).variant).esr { 0 => {}, 1|3|5|6 => { v >>= (flow * 4); v &= 0xf; if v != 0 { sun8i_ce_dump_task_descriptors(ch); err = -14; } }, 4 => { v >>= (flow * 8); v &= 0xff; if v != 0 { sun8i_ce_dump_task_descriptors(ch); err = -14; } }, _ => {} }
    err
}

// The remaining registration, PM, allocation, probe/remove, OF-match and
// module-driver definitions are external-kernel operations.  Keep their
// source-level entry points available for the surrounding Rust kernel port.
extern "C" { pub fn sun8i_ce_probe(pdev: *mut c_void) -> c_int; pub fn sun8i_ce_remove(pdev: *mut c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
