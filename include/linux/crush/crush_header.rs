/* SPDX-License-Identifier: GPL-2.0 */

/* CRUSH is a pseudo-random data distribution algorithm for structured storage clusters. */

pub const CRUSH_MAGIC: u64 = 0x0001_0000;
pub const CRUSH_MAX_DEPTH: i32 = 10;
pub const CRUSH_MAX_RULESET: i32 = 1 << 8;
pub const CRUSH_MAX_RULES: i32 = CRUSH_MAX_RULESET;
pub const CRUSH_MAX_DEVICE_WEIGHT: u32 = 100u32 * 0x10000u32;
pub const CRUSH_MAX_BUCKET_WEIGHT: u32 = 65535u32 * 0x10000u32;
pub const CRUSH_ITEM_UNDEF: i32 = 0x7ffffffe;
pub const CRUSH_ITEM_NONE: i32 = 0x7fffffff;

#[repr(C)]
pub struct crush_rule_step { pub op: u32, pub arg1: i32, pub arg2: i32 }

pub const CRUSH_RULE_NOOP: i32 = 0;
pub const CRUSH_RULE_TAKE: i32 = 1;
pub const CRUSH_RULE_CHOOSE_FIRSTN: i32 = 2;
pub const CRUSH_RULE_CHOOSE_INDEP: i32 = 3;
pub const CRUSH_RULE_EMIT: i32 = 4;
pub const CRUSH_RULE_CHOOSELEAF_FIRSTN: i32 = 6;
pub const CRUSH_RULE_CHOOSELEAF_INDEP: i32 = 7;
pub const CRUSH_RULE_SET_CHOOSE_TRIES: i32 = 8;
pub const CRUSH_RULE_SET_CHOOSELEAF_TRIES: i32 = 9;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_TRIES: i32 = 10;
pub const CRUSH_RULE_SET_CHOOSE_LOCAL_FALLBACK_TRIES: i32 = 11;
pub const CRUSH_RULE_SET_CHOOSELEAF_VARY_R: i32 = 12;
pub const CRUSH_RULE_SET_CHOOSELEAF_STABLE: i32 = 13;

pub const CRUSH_CHOOSE_N: i32 = 0;
#[inline] pub const fn CRUSH_CHOOSE_N_MINUS(x: i32) -> i32 { -x }

#[repr(C)]
pub struct crush_rule_mask { pub ruleset: u8, pub type_: u8, pub min_size: u8, pub max_size: u8 }
#[repr(C)]
pub struct crush_rule { pub len: u32, pub mask: crush_rule_mask, pub steps: [crush_rule_step; 0] }
#[inline] pub const fn crush_rule_size(len: usize) -> usize { core::mem::size_of::<crush_rule>() + len * core::mem::size_of::<crush_rule_step>() }

pub const CRUSH_BUCKET_UNIFORM: i32 = 1;
pub const CRUSH_BUCKET_LIST: i32 = 2;
pub const CRUSH_BUCKET_TREE: i32 = 3;
pub const CRUSH_BUCKET_STRAW: i32 = 4;
pub const CRUSH_BUCKET_STRAW2: i32 = 5;
pub const CRUSH_LEGACY_ALLOWED_BUCKET_ALGS: u32 = (1 << CRUSH_BUCKET_UNIFORM) | (1 << CRUSH_BUCKET_LIST) | (1 << CRUSH_BUCKET_STRAW);

unsafe extern "C" { pub fn crush_bucket_alg_name(alg: i32) -> *const core::ffi::c_char; }

#[repr(C)]
pub struct crush_bucket { pub id: i32, pub type_: u16, pub alg: u8, pub hash: u8, pub weight: u32, pub size: u32, pub items: *mut i32 }
#[repr(C)]
pub struct crush_weight_set { pub weights: *mut u32, pub size: u32 }
#[repr(C)]
pub struct crush_choose_arg { pub ids: *mut i32, pub ids_size: u32, pub weight_set: *mut crush_weight_set, pub weight_set_size: u32 }
#[repr(C)]
pub struct crush_choose_arg_map { pub args: *mut crush_choose_arg, pub size: u32 }

#[repr(C)] pub struct crush_bucket_uniform { pub h: crush_bucket, pub item_weight: u32 }
#[repr(C)] pub struct crush_bucket_list { pub h: crush_bucket, pub item_weights: *mut u32, pub sum_weights: *mut u32 }
#[repr(C)] pub struct crush_bucket_tree { pub h: crush_bucket, pub num_nodes: u8, pub node_weights: *mut u32 }
#[repr(C)] pub struct crush_bucket_straw { pub h: crush_bucket, pub item_weights: *mut u32, pub straws: *mut u32 }
#[repr(C)] pub struct crush_bucket_straw2 { pub h: crush_bucket, pub item_weights: *mut u32 }

#[repr(C)]
pub struct crush_map {
    pub buckets: *mut *mut crush_bucket,
    pub rules: *mut *mut crush_rule,
    pub max_buckets: i32,
    pub max_rules: u32,
    pub max_devices: i32,
    pub choose_local_tries: u32,
    pub choose_local_fallback_tries: u32,
    pub choose_total_tries: u32,
    pub chooseleaf_descend_once: u32,
    pub chooseleaf_vary_r: u8,
    pub chooseleaf_stable: u8,
    pub working_size: usize,
    pub straw_calc_version: u8,
    pub allowed_bucket_algs: u32,
    pub choose_tries: *mut u32,
}

unsafe extern "C" {
    pub fn crush_get_bucket_item_weight(b: *const crush_bucket, pos: i32) -> i32;
    pub fn crush_destroy_bucket_uniform(b: *mut crush_bucket_uniform);
    pub fn crush_destroy_bucket_list(b: *mut crush_bucket_list);
    pub fn crush_destroy_bucket_tree(b: *mut crush_bucket_tree);
    pub fn crush_destroy_bucket_straw(b: *mut crush_bucket_straw);
    pub fn crush_destroy_bucket_straw2(b: *mut crush_bucket_straw2);
    pub fn crush_destroy_bucket(b: *mut crush_bucket);
    pub fn crush_destroy_rule(r: *mut crush_rule);
    pub fn crush_destroy(map: *mut crush_map);
}

#[inline]
pub const fn crush_calc_tree_node(i: i32) -> i32 { ((i + 1) << 1) - 1 }

#[repr(C)] pub struct crush_work_bucket { pub perm_x: u32, pub perm_n: u32, pub perm: *mut u32 }
#[repr(C)] pub struct crush_work { pub work: *mut *mut crush_work_bucket }

/* Kernel-only declarations are intentionally preserved as conditional dependency references. */
#[cfg(feature = "__KERNEL__")]
unsafe extern "C" {
    pub fn clear_crush_names(root: *mut core::ffi::c_void);
    pub fn clear_choose_args(c: *mut crush_map);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
