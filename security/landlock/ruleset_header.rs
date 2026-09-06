/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock LSM - Ruleset management
 *
 * Copyright (c) 2016-2020 Mickael Salaun <mic@digikod.net>
 * Copyright (c) 2018-2020 ANSSI
 * Copyright (c) 2026 Cloudflare, Inc.
 */

/* Rust translation of landlock/ruleset.h. */
/* C includes translated as external dependencies:
 * linux/cleanup.h, linux/err.h, linux/mutex.h, linux/rbtree.h,
 * linux/refcount.h, access.h, limits.h, object.h.
 */

use core::ffi::{c_int, c_ulong};

/*
 * External types supplied by the translated dependency headers.
 */
pub type access_mask_t = u16;
pub type size_t = usize;

#[repr(C)]
pub struct rb_node {
	pub __private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
	pub __private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
	pub __private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
	pub __private: [u8; 0],
}

#[repr(C)]
pub struct access_masks {
	pub __private: [u8; 0],
}

#[repr(C)]
pub struct landlock_object {
	pub __private: [u8; 0],
}

/**
 * struct landlock_layer - Access rights for a given layer
 */
#[repr(C)]
pub struct landlock_layer {
	/**
	 * @level: Position of this layer in the layer stack.  Starts from 1.
	 */
	pub level: u8,
	/**
	 * @flags: Bitfield for special flags attached to this rule.
	 */
	pub flags: landlock_layer_flags,
	/**
	 * @access: Bitfield of allowed actions on the kernel object.  They are
	 * relative to the object type (e.g. %LANDLOCK_ACTION_FS_READ).
	 */
	pub access: access_mask_t,
}

#[repr(C)]
pub struct landlock_layer_flags {
	/**
	 * @flags.quiet: Suppresses denial logs for the object covered by
	 * this rule in this domain.  For filesystem rules, this inherits
	 * down the file hierarchy.
	 */
	pub _bitfield_1: u8,
}

impl landlock_layer_flags {
	pub const QUIET_MASK: u8 = 1 << 0;

	pub fn quiet(&self) -> u8 {
		self._bitfield_1 & Self::QUIET_MASK
	}

	pub fn set_quiet(&mut self, value: u8) {
		self._bitfield_1 =
			(self._bitfield_1 & !Self::QUIET_MASK) | ((value & 1) << 0);
	}
}

/**
 * union landlock_key - Key of a ruleset's red-black tree
 */
#[repr(C)]
pub union landlock_key {
	/**
	 * @object: Pointer to identify a kernel object (e.g. an inode).
	 */
	pub object: *mut landlock_object,
	/**
	 * @data: Raw data to identify an arbitrary 32-bit value
	 * (e.g. a TCP port).
	 */
	pub data: usize,
}

/**
 * enum landlock_key_type - Type of &union landlock_key
 */
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum landlock_key_type {
	/**
	 * @LANDLOCK_KEY_INODE: Type of &landlock_rules.root_inode's node keys.
	 */
	LANDLOCK_KEY_INODE = 1,
	/**
	 * @LANDLOCK_KEY_NET_PORT: Type of &landlock_rules.root_net_port's node
	 * keys.
	 */
	LANDLOCK_KEY_NET_PORT = 2,
}

/**
 * struct landlock_id - Unique rule identifier for a ruleset
 */
#[repr(C)]
pub struct landlock_id {
	/**
	 * @key: Identifies either a kernel object (e.g. an inode) or
	 * a raw value (e.g. a TCP port).
	 */
	pub key: landlock_key,
	/**
	 * @type: Type of a landlock_ruleset's root tree.
	 */
	pub type_: landlock_key_type,
}

/**
 * struct landlock_rule - Access rights tied to an object
 */
#[repr(C)]
pub struct landlock_rule {
	/**
	 * @node: Node in the ruleset's red-black tree.
	 */
	pub node: rb_node,
	/**
	 * @key: A union to identify either a kernel object (e.g. an inode) or
	 * a raw data value (e.g. a network socket port). This is used as a key
	 * for this ruleset element.  The pointer is set once and never
	 * modified.  It always points to an allocated object because each rule
	 * increments the refcount of its object.
	 */
	pub key: landlock_key,
	/**
	 * @num_layers: Number of entries in @layers.
	 */
	pub num_layers: u32,
	/**
	 * @layers: Stack of layers, from the latest to the newest, implemented
	 * as a flexible array member (FAM).
	 */
	pub layers: [landlock_layer; 0],
}

/**
 * struct landlock_rules - Red-black tree storage for Landlock rules
 *
 * This structure holds the rule trees shared by both rulesets and domains.
 */
#[repr(C)]
pub struct landlock_rules {
	/**
	 * @root_inode: Root of a red-black tree containing &struct
	 * landlock_rule nodes with inode object.  Immutable for domains.
	 */
	pub root_inode: rb_root,

	/* Present when IS_ENABLED(CONFIG_INET). */
	/**
	 * @root_net_port: Root of a red-black tree containing &struct
	 * landlock_rule nodes with network port.  Immutable for domains.
	 */
	pub root_net_port: rb_root,

	/**
	 * @num_rules: Number of non-overlapping (i.e. not for the same object)
	 * rules in this tree storage.
	 */
	pub num_rules: u32,
}

/**
 * struct landlock_ruleset - Landlock ruleset
 *
 * This data structure must contain unique entries, be updatable, and quick to
 * match an object.
 */
#[repr(C)]
pub struct landlock_ruleset {
	/**
	 * @rules: Red-black tree storage for rules.
	 */
	pub rules: landlock_rules,
	/**
	 * @lock: Protects against concurrent modifications of @rules, if @usage
	 * is greater than zero.
	 */
	pub lock: mutex,
	/**
	 * @usage: Number of file descriptors referencing this ruleset.
	 */
	pub usage: refcount_t,

	/* Present when CONFIG_TRACEPOINTS is enabled. */
	/**
	 * @version: Counter incremented on each successful
	 * landlock_add_rule(2), including when it only extends an existing
	 * rule's access rights.  Used by tracepoints to correlate a domain with
	 * the exact ruleset state it was created from.  Protected by @lock.
	 */
	pub version: u32,
	/**
	 * @id: Unique identifier for this ruleset, used for tracing.
	 */
	pub id: u64,

	/**
	 * @quiet_masks: Stores the quiet flags for an unmerged ruleset.  For a
	 * merged domain, this is stored in each layer's struct
	 * landlock_hierarchy instead.
	 */
	pub quiet_masks: access_masks,
	/**
	 * @handled_masks: Contains the subset of filesystem and network actions
	 * that are handled by this ruleset.
	 */
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
	) -> c_int;

	pub fn landlock_store_rule(
		rules: *mut landlock_rules,
		id: landlock_id,
		layers: *const [landlock_layer; 0],
		num_layers: size_t,
	) -> c_int;

	pub fn landlock_free_rules(rules: *mut landlock_rules);

	fn IS_ERR_OR_NULL(ptr: *const core::ffi::c_void) -> bool;
	fn ERR_PTR(error: c_long) -> *mut rb_root;
	fn WARN_ON_ONCE(condition: c_int) -> c_int;
	fn refcount_inc(r: *mut refcount_t);
}

pub type c_long = isize;

pub const EINVAL: c_int = 22;

/*
 * DEFINE_FREE(landlock_put_ruleset, struct landlock_ruleset *,
 *             if (!IS_ERR_OR_NULL(_T)) landlock_put_ruleset(_T))
 */
pub unsafe fn landlock_put_ruleset_free(ruleset: *mut landlock_ruleset) {
	if !unsafe { IS_ERR_OR_NULL(ruleset.cast()) } {
		unsafe { landlock_put_ruleset(ruleset) };
	}
}

/**
 * landlock_get_rule_root - Get the root of a rule tree by key type
 *
 * @rules: The rules storage to look up.
 * @key_type: The type of key to select the tree for.
 *
 * Return: A pointer to the rb_root, or ERR_PTR(-EINVAL) on unknown type.
 */
pub unsafe fn landlock_get_rule_root(
	rules: *mut landlock_rules,
	key_type: landlock_key_type,
) -> *mut rb_root {
	match key_type {
		landlock_key_type::LANDLOCK_KEY_INODE => unsafe { &mut (*rules).root_inode },

		/* Present when IS_ENABLED(CONFIG_INET). */
		landlock_key_type::LANDLOCK_KEY_NET_PORT => unsafe { &mut (*rules).root_net_port },
		_ => {
			unsafe { WARN_ON_ONCE(1) };
			unsafe { ERR_PTR(-(EINVAL as c_long)) }
		}
	}
}

pub unsafe fn landlock_get_ruleset(ruleset: *mut landlock_ruleset) {
	if !ruleset.is_null() {
		unsafe { refcount_inc(unsafe { &mut (*ruleset).usage }) };
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
