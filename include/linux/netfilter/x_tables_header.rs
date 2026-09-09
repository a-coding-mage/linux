/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/netfilter/x_tables.h. Kernel-provided types and
// functions referenced here are intentionally left as external dependencies.

#[macro_export]
macro_rules! NF_INVF {
    ($ptr:expr, $flag:expr, $boolean:expr) => {
        (($boolean) ^ ((($ptr).invflags & ($flag)) != 0))
    };
}

#[repr(C)]
pub union xt_action_param_match_target {
    pub r#match: *const xt_match,
    pub target: *const xt_target,
}
#[repr(C)]
pub union xt_action_param_info {
    pub matchinfo: *const core::ffi::c_void,
    pub targinfo: *const core::ffi::c_void,
}
#[repr(C)]
pub struct xt_action_param {
    pub match_target: xt_action_param_match_target,
    pub info: xt_action_param_info,
    pub state: *const nf_hook_state,
    pub thoff: u32,
    pub fragoff: u16,
    pub hotdrop: bool,
}

#[inline]
pub unsafe fn xt_net(par: *const xt_action_param) -> *mut net { (*(*par).state).net }
#[inline]
pub unsafe fn xt_in(par: *const xt_action_param) -> *mut net_device { (*(*par).state).in_dev }
#[inline]
pub unsafe fn xt_out(par: *const xt_action_param) -> *mut net_device { (*(*par).state).out }
#[inline]
pub unsafe fn xt_hooknum(par: *const xt_action_param) -> u32 { (*(*par).state).hook }
#[inline]
pub unsafe fn xt_family(par: *const xt_action_param) -> u8 { (*(*par).state).pf }

#[repr(C)]
pub struct xt_mtchk_param {
    pub net: *mut net, pub table: *const core::ffi::c_char,
    pub entryinfo: *const core::ffi::c_void, pub r#match: *const xt_match,
    pub matchinfo: *mut core::ffi::c_void, pub hook_mask: u32,
    pub family: u8, pub nft_compat: bool,
}
#[repr(C)]
pub struct xt_mtdtor_param {
    pub net: *mut net, pub r#match: *const xt_match,
    pub matchinfo: *mut core::ffi::c_void, pub family: u8,
}
#[repr(C)]
pub struct xt_tgchk_param {
    pub net: *mut net, pub table: *const core::ffi::c_char,
    pub entryinfo: *const core::ffi::c_void, pub target: *const xt_target,
    pub targinfo: *mut core::ffi::c_void, pub hook_mask: u32,
    pub family: u8, pub nft_compat: bool,
}
#[repr(C)]
pub struct xt_tgdtor_param {
    pub net: *mut net, pub target: *const xt_target,
    pub targinfo: *mut core::ffi::c_void, pub family: u8,
}

#[repr(C)]
pub struct xt_match {
    pub list: list_head,
    pub name: [core::ffi::c_char; XT_EXTENSION_MAXNAMELEN],
    pub revision: u8,
    pub r#match: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub check_hooks: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_mtdtor_param)>,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    pub compat_from_user: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void)>,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    pub compat_to_user: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> i32>,
    pub me: *mut module, pub table: *const core::ffi::c_char,
    pub matchsize: u32, pub usersize: u32,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)] pub compatsize: u32,
    pub hooks: u32, pub proto: u16, pub family: u16,
}
#[repr(C)]
pub struct xt_target {
    pub list: list_head,
    pub name: [core::ffi::c_char; XT_EXTENSION_MAXNAMELEN], pub revision: u8,
    pub target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> u32>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> i32>,
    pub check_hooks: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_tgdtor_param)>,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    pub compat_from_user: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void)>,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    pub compat_to_user: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> i32>,
    pub me: *mut module, pub table: *const core::ffi::c_char,
    pub targetsize: u32, pub usersize: u32,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)] pub compatsize: u32,
    pub hooks: u32, pub proto: u16, pub family: u16,
}
#[repr(C)]
pub struct xt_table {
    pub list: list_head, pub valid_hooks: u32, pub private: *mut xt_table_info,
    pub ops: *mut nf_hook_ops, pub me: *mut module, pub af: u8, pub priority: i32,
    pub name: [core::ffi::c_char; XT_TABLE_MAXNAMELEN],
}
#[repr(C)]
pub struct xt_table_info {
    pub size: u32, pub number: u32, pub initial_entries: u32,
    pub hook_entry: [u32; NF_INET_NUMHOOKS], pub underflow: [u32; NF_INET_NUMHOOKS],
    pub stacksize: u32, pub jumpstack: *mut *mut *mut core::ffi::c_void,
    pub entries: [u8; 0],
}

extern "C" {
    pub fn xt_register_target(target: *mut xt_target) -> i32;
    pub fn xt_unregister_target(target: *mut xt_target);
    pub fn xt_register_targets(target: *mut xt_target, n: u32) -> i32;
    pub fn xt_unregister_targets(target: *mut xt_target, n: u32);
    pub fn xt_register_match(target: *mut xt_match) -> i32;
    pub fn xt_unregister_match(target: *mut xt_match);
    pub fn xt_register_matches(m: *mut xt_match, n: u32) -> i32;
    pub fn xt_unregister_matches(m: *mut xt_match, n: u32);
    pub fn xt_check_entry_offsets(base: *const core::ffi::c_void, elems: *const core::ffi::c_char, target_offset: u32, next_offset: u32) -> i32;
    pub fn xt_check_table_hooks(info: *const xt_table_info, valid_hooks: u32) -> i32;
    pub fn xt_alloc_entry_offsets(size: u32) -> *mut u32;
    pub fn xt_find_jump_offset(offsets: *const u32, target: u32, size: u32) -> bool;
    pub fn xt_check_proc_name(name: *const core::ffi::c_char, size: u32) -> i32;
    pub fn xt_check_hooks_match(par: *mut xt_mtchk_param) -> i32;
    pub fn xt_check_match(par: *mut xt_mtchk_param, size: u32, proto: u16, inv_proto: bool) -> i32;
    pub fn xt_check_hooks_target(par: *mut xt_tgchk_param) -> i32;
    pub fn xt_check_target(par: *mut xt_tgchk_param, size: u32, proto: u16, inv_proto: bool) -> i32;
    pub fn xt_copy_counters(arg: sockptr_t, len: u32, info: *mut xt_counters_info) -> *mut core::ffi::c_void;
    pub fn xt_match_to_user(m: *const xt_entry_match, u: *mut xt_entry_match) -> i32;
    pub fn xt_target_to_user(t: *const xt_entry_target, u: *mut xt_entry_target) -> i32;
    pub fn xt_data_to_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, usersize: i32, size: i32, aligned_size: i32) -> i32;
    pub fn xt_counters_alloc(counters: u32) -> *mut xt_counters;
    pub fn xt_alloc_table_info(size: u32) -> *mut xt_table_info;
    pub fn xt_free_table_info(info: *mut xt_table_info);
    pub fn xt_register_table(net: *mut net, table: *const xt_table, template_ops: *const nf_hook_ops, bootstrap: *mut xt_table_info, newinfo: *mut xt_table_info) -> *mut xt_table;
    pub fn xt_unregister_table_pre_exit(net: *mut net, af: u8, name: *const core::ffi::c_char);
    pub fn xt_unregister_table_exit(net: *mut net, af: u8, name: *const core::ffi::c_char) -> *mut xt_table;
    pub fn xt_replace_table(table: *mut xt_table, num_counters: u32, newinfo: *mut xt_table_info, error: *mut i32) -> *mut xt_table_info;
    pub fn xt_find_match(af: u8, name: *const core::ffi::c_char, revision: u8) -> *mut xt_match;
    pub fn xt_request_find_match(af: u8, name: *const core::ffi::c_char, revision: u8) -> *mut xt_match;
    pub fn xt_request_find_target(af: u8, name: *const core::ffi::c_char, revision: u8) -> *mut xt_target;
    pub fn xt_find_revision(af: u8, name: *const core::ffi::c_char, revision: u8, target: i32, err: *mut i32) -> i32;
    pub fn xt_find_table(net: *mut net, af: u8, name: *const core::ffi::c_char) -> *mut xt_table;
    pub fn xt_find_table_lock(net: *mut net, af: u8, name: *const core::ffi::c_char) -> *mut xt_table;
    pub fn xt_request_find_table_lock(net: *mut net, af: u8, name: *const core::ffi::c_char) -> *mut xt_table;
    pub fn xt_table_unlock(t: *mut xt_table);
    pub fn xt_proto_init(net: *mut net, af: u8) -> i32;
    pub fn xt_proto_fini(net: *mut net, af: u8);
}

#[repr(C)]
pub struct xt_percpu_counter_alloc_state { pub off: u32, pub mem: *const core::ffi::c_char }
extern "C" {
    pub fn xt_percpu_counter_alloc(state: *mut xt_percpu_counter_alloc_state, counter: *mut xt_counters) -> bool;
    pub fn xt_percpu_counter_free(cnt: *mut xt_counters);
    pub fn xt_hook_ops_alloc(t: *const xt_table, hookfn: nf_hookfn) -> *mut nf_hook_ops;
    pub fn xt_register_template(t: *const xt_table, table_init: Option<unsafe extern "C" fn(*mut net) -> i32>) -> i32;
    pub fn xt_unregister_template(t: *const xt_table);
}

#[inline] pub unsafe fn xt_get_this_cpu_counter(cnt: *mut xt_counters) -> *mut xt_counters { if nr_cpu_ids > 1 { this_cpu_ptr((*cnt).pcnt as *mut core::ffi::c_void) as *mut xt_counters } else { cnt } }
#[inline] pub unsafe fn xt_get_per_cpu_counter(cnt: *mut xt_counters, cpu: u32) -> *mut xt_counters { if nr_cpu_ids > 1 { per_cpu_ptr((*cnt).pcnt as *mut core::ffi::c_void, cpu) as *mut xt_counters } else { cnt } }

// CONFIG_NETFILTER_XTABLES_COMPAT declarations are preserved conditionally.
#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
extern "C" {
    pub fn xt_compat_lock(af: u8); pub fn xt_compat_unlock(af: u8);
    pub fn xt_compat_add_offset(af: u8, offset: u32, delta: i32) -> i32;
    pub fn xt_compat_flush_offsets(af: u8); pub fn xt_compat_init_offsets(af: u8, number: u32) -> i32;
    pub fn xt_compat_calc_jump(af: u8, offset: u32) -> i32;
    pub fn xt_compat_match_offset(m: *const xt_match) -> i32;
    pub fn xt_compat_target_offset(t: *const xt_target) -> i32;
    pub fn xt_compat_check_entry_offsets(base: *const core::ffi::c_void, elems: *const core::ffi::c_char, target_offset: u32, next_offset: u32) -> i32;
}

#[inline]
pub fn xt_compat_check() -> bool { true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
