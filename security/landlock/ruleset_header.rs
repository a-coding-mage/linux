/* SPDX-License-Identifier: GPL-2.0-only */
/* Landlock LSM - Ruleset management */
/* C dependencies: linux/cleanup.h, linux/err.h, linux/mutex.h,
 * linux/rbtree.h, linux/refcount.h, access.h, limits.h, object.h. */

#[repr(C)]
pub struct landlock_layer {
	pub level: u8,
	pub flags: landlock_layer_flags,
	pub access: access_mask_t,
}

#[repr(C)]
pub struct landlock_layer_flags {
	pub quiet: u8,
}

#[repr(C)]
pub union landlock_key {
	pub object: *mut landlock_object,
	pub data: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum landlock_key_type {
	LANDLOCK_KEY_INODE = 1,
	LANDLOCK_KEY_NET_PORT = 2,
}

#[repr(C)]
pub struct landlock_id {
	pub key: landlock_key,
	pub type_: landlock_key_type,
}

#[repr(C)]
pub struct landlock_rule {
	pub node: rb_node,
	pub key: landlock_key,
	pub num_layers: u32,
	pub layers: [landlock_layer; 0],
}

#[repr(C)]
pub struct landlock_rules {
	pub root_inode: rb_root,
	/* Present when IS_ENABLED(CONFIG_INET). */
	pub root_net_port: rb_root,
	pub num_rules: u32,
}

#[repr(C)]
pub struct landlock_ruleset {
	pub rules: landlock_rules,
	pub lock: mutex,
	pub usage: refcount_t,
	/* Present when CONFIG_TRACEPOINTS is enabled. */
	pub version: u32,
	pub id: u64,
	pub quiet_masks: access_masks,
	pub handled_masks: access_masks,
}

unsafe extern "C" {
	pub fn landlock_create_ruleset(
		access_mask_fs: access_mask_t,
		access_mask_net: access_mask_t,
		scope_mask: access_mask_t,
	) -> *mut landlock_ruleset;
	pub fn landlock_put_ruleset(ruleset: *mut landlock_ruleset);
	pub fn landlock_insert_rule(
		ruleset: *mut landlock_ruleset,
		id: landlock_id,
		access: access_mask_t,
		flags: u32,
	) -> core::ffi::c_int;
	pub fn landlock_store_rule(
		rules: *mut landlock_rules,
		id: landlock_id,
		layers: *const [landlock_layer; 0],
		num_layers: usize,
	) -> core::ffi::c_int;
	pub fn landlock_free_rules(rules: *mut landlock_rules);
}

pub unsafe fn landlock_put_ruleset_free(ruleset: *mut landlock_ruleset) {
	if !ruleset.is_null() && !unsafe { IS_ERR_OR_NULL(ruleset.cast()) } {
		unsafe { landlock_put_ruleset(ruleset) };
	}
}

pub unsafe fn landlock_get_rule_root(
	rules: *mut landlock_rules,
	key_type: landlock_key_type,
) -> *mut rb_root {
	match key_type {
		landlock_key_type::LANDLOCK_KEY_INODE => unsafe { &mut (*rules).root_inode },
		landlock_key_type::LANDLOCK_KEY_NET_PORT => unsafe { &mut (*rules).root_net_port },
		_ => {
			unsafe { WARN_ON_ONCE(1) };
			unsafe { ERR_PTR(-(EINVAL as isize)) }
		}
	}
}

pub unsafe fn landlock_get_ruleset(ruleset: *mut landlock_ruleset) {
	if !ruleset.is_null() {
		unsafe { refcount_inc(&mut (*ruleset).usage) };
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
