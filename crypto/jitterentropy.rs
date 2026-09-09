/* Rust translation of jitterentropy.c. */

pub type __u64 = u64;
pub type __s64 = i64;
pub type __u32 = u32;
pub type u8 = u8;

pub const DATA_SIZE_BITS: usize = 256;
pub const JENT_MEMORY_ACCESSLOOPS: u32 = 128;
pub const JENT_APT_WINDOW_SIZE: u32 = 512;
pub const JENT_APT_LSB: u32 = 16;
pub const JENT_APT_WORD_MASK: u32 = JENT_APT_LSB - 1;
pub const JENT_DISABLE_MEMORY_ACCESS: u32 = 1 << 2;
pub const JENT_ENOTIME: i32 = 1;
pub const JENT_ECOARSETIME: i32 = 2;
pub const JENT_ENOMONOTONIC: i32 = 3;
pub const JENT_EVARVAR: i32 = 5;
pub const JENT_ESTUCK: i32 = 8;
pub const JENT_EHEALTH: i32 = 9;
pub const JENT_ERCT: i32 = 10;
pub const JENT_EHASH: i32 = 11;
pub const JENT_EMEM: i32 = 12;
pub const JENT_RCT_FAILURE: u32 = 1;
pub const JENT_APT_FAILURE: u32 = 2;
pub const JENT_PERMANENT_FAILURE_SHIFT: u32 = 16;
pub const JENT_RCT_FAILURE_PERMANENT: u32 = JENT_RCT_FAILURE << JENT_PERMANENT_FAILURE_SHIFT;
pub const JENT_APT_FAILURE_PERMANENT: u32 = JENT_APT_FAILURE << JENT_PERMANENT_FAILURE_SHIFT;
pub const JENT_ENTROPY_SAFETY_FACTOR: u32 = 64;

#[repr(C)]
pub struct sha3_ctx { _private: [u8; 0] }

#[repr(C)]
pub struct rand_data {
    pub hash_state: *mut sha3_ctx,
    pub prev_time: __u64,
    pub last_delta: __u64,
    pub last_delta2: __s64,
    pub flags: u32,
    pub osr: u32,
    pub mem: *mut u8,
    pub memlocation: u32,
    pub memblocks: u32,
    pub memblocksize: u32,
    pub memaccessloops: u32,
    pub rct_count: u32,
    pub apt_cutoff: u32,
    pub apt_cutoff_permanent: u32,
    pub apt_observations: u32,
    pub apt_count: u32,
    pub apt_base: u32,
    pub health_failure: u32,
    pub apt_base_set: u32,
}

extern "C" {
    pub static fips_enabled: bool;
    fn jent_get_nstime(time: *mut __u64);
    fn jent_hash_time(ctx: *mut sha3_ctx, time: __u64, addtl: *mut u8, len: usize, loops: u32, stuck: i32);
    fn jent_read_random_block(ctx: *mut sha3_ctx, data: *mut u8, len: u32);
    fn jent_zalloc(size: usize) -> *mut rand_data;
    fn jent_zfree(ptr: *mut rand_data);
    fn jent_kvzalloc(size: usize) -> *mut u8;
    fn jent_kvzfree(ptr: *mut u8, size: usize);
}

static JENT_APT_CUTOFF_LOOKUP: [u32; 15] = [325,422,459,477,488,494,499,502,505,507,508,509,510,511,512];
static JENT_APT_CUTOFF_PERMANENT_LOOKUP: [u32; 15] = [355,447,479,494,502,507,510,512,512,512,512,512,512,512,512];

unsafe fn jent_apt_init(ec: *mut rand_data, osr: u32) {
    let n = JENT_APT_CUTOFF_LOOKUP.len();
    let i = if osr as usize >= n { n - 1 } else { osr as usize - 1 };
    (*ec).apt_cutoff = JENT_APT_CUTOFF_LOOKUP[i];
    (*ec).apt_cutoff_permanent = JENT_APT_CUTOFF_PERMANENT_LOOKUP[i];
}
unsafe fn jent_apt_reset(ec: *mut rand_data, delta_masked: u32) {
    (*ec).apt_count = 0; (*ec).apt_base = delta_masked; (*ec).apt_observations = 0;
}
unsafe fn jent_apt_insert(ec: *mut rand_data, delta_masked: u32) {
    if (*ec).apt_base_set == 0 { (*ec).apt_base = delta_masked; (*ec).apt_base_set = 1; return; }
    if delta_masked == (*ec).apt_base {
        (*ec).apt_count = (*ec).apt_count.wrapping_add(1);
        if (*ec).apt_count >= (*ec).apt_cutoff_permanent { (*ec).health_failure |= JENT_APT_FAILURE_PERMANENT; }
        else if (*ec).apt_count >= (*ec).apt_cutoff { (*ec).health_failure |= JENT_APT_FAILURE; }
    }
    (*ec).apt_observations = (*ec).apt_observations.wrapping_add(1);
    if (*ec).apt_observations >= JENT_APT_WINDOW_SIZE { jent_apt_reset(ec, delta_masked); }
}
unsafe fn jent_rct_insert(ec: *mut rand_data, stuck: i32) {
    if stuck != 0 {
        (*ec).rct_count = (*ec).rct_count.wrapping_add(1);
        if (*ec).rct_count >= 60 * (*ec).osr { (*ec).rct_count = u32::MAX; (*ec).health_failure |= JENT_RCT_FAILURE_PERMANENT; }
        else if (*ec).rct_count >= 30 * (*ec).osr { (*ec).rct_count = u32::MAX; (*ec).health_failure |= JENT_RCT_FAILURE; }
    } else { (*ec).rct_count = 0; }
}
#[inline] fn jent_delta(prev: __u64, next: __u64) -> __u64 { if prev < next { next - prev } else { u64::MAX.wrapping_sub(prev).wrapping_add(1).wrapping_add(next) } }
unsafe fn jent_stuck(ec: *mut rand_data, current_delta: __u64) -> i32 {
    let delta2 = jent_delta((*ec).last_delta, current_delta);
    let delta3 = jent_delta((*ec).last_delta2 as u64, delta2);
    (*ec).last_delta = current_delta; (*ec).last_delta2 = delta2 as i64;
    jent_apt_insert(ec, current_delta as u32);
    if current_delta == 0 || delta2 == 0 || delta3 == 0 { jent_rct_insert(ec, 1); 1 } else { jent_rct_insert(ec, 0); 0 }
}
unsafe fn jent_health_failure(ec: *mut rand_data) -> u32 { if !fips_enabled { 0 } else { (*ec).health_failure } }
unsafe fn jent_loop_shuffle(bits: u32, min: u32) -> __u64 {
    let mut time = 0; let mut shuffle = 0; let mask = (1u64 << bits) - 1;
    jent_get_nstime(&mut time);
    let mut i = 0; while ((DATA_SIZE_BITS as u32 + bits - 1) / bits) > i { shuffle ^= time & mask; time >>= bits; i += 1; }
    shuffle + (1u64 << min)
}
unsafe fn jent_condition_data(ec: *mut rand_data, time: __u64, stuck: i32) {
    let mut addtl = ( (*ec).rct_count, (*ec).apt_observations, (*ec).apt_count, (*ec).apt_base );
    jent_hash_time((*ec).hash_state, time, &mut addtl as *mut _ as *mut u8, core::mem::size_of_val(&addtl), 1 << 3, stuck);
}
unsafe fn jent_memaccess(ec: *mut rand_data, loop_cnt: __u64) {
    if ec.is_null() || (*ec).mem.is_null() { return; }
    let wrap = (*ec).memblocksize * (*ec).memblocks;
    let mut acc = jent_loop_shuffle(7, 0); if loop_cnt != 0 { acc = loop_cnt; }
    let mut i = 0; while i < (*ec).memaccessloops as u64 + acc { let p = (*ec).mem.add((*ec).memlocation as usize); *p = (*p).wrapping_add(1) & 0xff; (*ec).memlocation = ((*ec).memlocation + (*ec).memblocksize - 1) % wrap; i += 1; }
}
unsafe fn jent_measure_jitter(ec: *mut rand_data, ret: *mut __u64) -> i32 {
    jent_memaccess(ec, 0); let mut time = 0; jent_get_nstime(&mut time); let delta = jent_delta((*ec).prev_time, time); (*ec).prev_time = time; let stuck = jent_stuck(ec, delta); jent_condition_data(ec, delta, stuck); if !ret.is_null() { *ret = delta; } stuck
}
unsafe fn jent_gen_entropy(ec: *mut rand_data) { let mut k = 0; let safety = if fips_enabled { JENT_ENTROPY_SAFETY_FACTOR } else { 0 }; jent_measure_jitter(ec, core::ptr::null_mut()); while jent_health_failure(ec) == 0 { if jent_measure_jitter(ec, core::ptr::null_mut()) != 0 { continue; } k += 1; if k >= (DATA_SIZE_BITS as u32 + safety) * (*ec).osr { break; } } }

#[no_mangle]
pub unsafe extern "C" fn jent_read_entropy(ec: *mut rand_data, data: *mut u8, mut len: u32) -> i32 {
    if ec.is_null() { return -1; } let mut p = data;
    while len > 0 { jent_gen_entropy(ec); let h = jent_health_failure(ec); if h > JENT_PERMANENT_FAILURE_SHIFT { return -3; } else if h != 0 { if jent_entropy_init(0, 0, core::ptr::null_mut(), ec) != 0 { (*ec).health_failure &= JENT_RCT_FAILURE_PERMANENT | JENT_APT_FAILURE_PERMANENT; return -3; } return -2; } let n = (DATA_SIZE_BITS as u32 / 8).min(len); jent_read_random_block((*ec).hash_state, p, n); len -= n; p = p.add(n as usize); }
    0
}

pub const JENT_MEMORY_SIZE: usize = 0; // CONFIG_CRYPTO_JITTERENTROPY_MEMORY_BLOCKS * CONFIG_CRYPTO_JITTERENTROPY_MEMORY_BLOCKSIZE

#[no_mangle]
pub unsafe extern "C" fn jent_entropy_collector_alloc(osr: u32, flags: u32, hash_state: *mut sha3_ctx) -> *mut rand_data {
    let ec = jent_zalloc(core::mem::size_of::<rand_data>()); if ec.is_null() { return core::ptr::null_mut(); }
    if flags & JENT_DISABLE_MEMORY_ACCESS == 0 { (*ec).mem = jent_kvzalloc(JENT_MEMORY_SIZE); if (*ec).mem.is_null() { jent_zfree(ec); return core::ptr::null_mut(); } }
    (*ec).osr = if osr == 0 { 1 } else { osr }; (*ec).flags = flags; (*ec).hash_state = hash_state; jent_apt_init(ec, (*ec).osr); jent_gen_entropy(ec); ec
}
#[no_mangle]
pub unsafe extern "C" fn jent_entropy_collector_free(ec: *mut rand_data) { jent_kvzfree((*ec).mem, JENT_MEMORY_SIZE); (*ec).mem = core::ptr::null_mut(); jent_zfree(ec); }
#[no_mangle]
pub unsafe extern "C" fn jent_entropy_init(osr: u32, flags: u32, hash_state: *mut sha3_ctx, p_ec: *mut rand_data) -> i32 {
    let mut ec = p_ec; let mut ec_free = false; if ec.is_null() { ec = jent_entropy_collector_alloc(osr, flags, hash_state); if ec.is_null() { return JENT_EMEM; } ec_free = true; } else { jent_apt_reset(ec, 0); (*ec).apt_base_set = 0; (*ec).rct_count = 0; (*ec).health_failure &= !JENT_RCT_FAILURE; (*ec).health_failure &= !JENT_APT_FAILURE; }
    let mut time_backwards = 0; let mut ret = 0; for i in 0..1124 { let mut delta = 0; jent_measure_jitter(ec, &mut delta); let end = (*ec).prev_time; let start = end.wrapping_sub(delta); if start == 0 || end == 0 { ret = JENT_ENOTIME; break; } if delta == 0 { ret = JENT_ECOARSETIME; break; } if i >= 100 && !(end > start) { time_backwards += 1; } }
    if ret == 0 && time_backwards > 3 { ret = JENT_ENOMONOTONIC; } if ret == 0 { let h = jent_health_failure(ec); if h != 0 { ret = if h & JENT_RCT_FAILURE != 0 { JENT_ERCT } else { JENT_EHEALTH }; } } if ec_free { jent_entropy_collector_free(ec); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
