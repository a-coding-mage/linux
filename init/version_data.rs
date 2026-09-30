// SPDX-License-Identifier: GPL-2.0-only
//! Shared constant initialization for both UTS_VERSION build stages.

use super::bindings;
use core::ptr;
use kernel::ffi::c_char;

const fn c_array<const N: usize>(source: &[u8]) -> [c_char; N] {
    // C permits exactly N non-NUL bytes in an N-byte array initializer.
    assert!(!source.is_empty() && source[source.len() - 1] == 0 && source.len() - 1 <= N);
    let mut result = [0; N];
    let mut index = 0;
    while index < source.len() && index < N {
        result[index] = source[index] as c_char;
        index += 1;
    }
    result
}

/// The caller supplies the eventual static address, never a temporary copy.
pub(super) const unsafe fn namespace(this: *mut bindings::uts_namespace) -> bindings::uts_namespace {
    // SAFETY: canonical namespace fields are integer/nullable pointer records;
    // zero initializes all fields omitted by the original C initializer. Each
    // raw self pointer identifies a list head within the caller's static.
    unsafe {
        let mut value: bindings::uts_namespace = core::mem::zeroed();
        value.name.sysname = c_array(bindings::RUST_VERSION_SYSNAME);
        value.name.nodename = c_array(bindings::RUST_VERSION_NODENAME);
        value.name.release = c_array(bindings::RUST_VERSION_RELEASE);
        value.name.version = c_array(bindings::RUST_VERSION_VERSION);
        value.name.machine = c_array(bindings::RUST_VERSION_MACHINE);
        value.name.domainname = c_array(bindings::RUST_VERSION_DOMAINNAME);
        value.user_ns = ptr::addr_of_mut!(bindings::init_user_ns);
        value.ns.ns_type = bindings::RUST_VERSION_NS_TYPE;
        value.ns.inum = bindings::RUST_VERSION_NS_INUM;
        #[cfg(CONFIG_UTS_NS)]
        { value.ns.ops = ptr::addr_of!(bindings::utsns_operations); }
        value.ns.__bindgen_anon_1.__ns_ref.refs.counter = 1;
        let tree = &mut value.ns.__bindgen_anon_2.rust_ns_tree;
        tree.ns_id = bindings::RUST_VERSION_NS_ID as u64;
        tree.__ns_ref_active.counter = 1;
        let live_tree = ptr::addr_of_mut!((*this).ns.__bindgen_anon_2.rust_ns_tree);
        let unified = ptr::addr_of_mut!((*live_tree).ns_unified_node.ns_list_entry);
        tree.ns_unified_node.ns_list_entry = bindings::list_head { next: unified, prev: unified };
        let per_type = ptr::addr_of_mut!((*live_tree).ns_tree_node.ns_list_entry);
        tree.ns_tree_node.ns_list_entry = bindings::list_head { next: per_type, prev: per_type };
        let owner = ptr::addr_of_mut!((*live_tree).ns_owner_node.ns_list_entry);
        tree.ns_owner_node.ns_list_entry = bindings::list_head { next: owner, prev: owner };
        let owned = ptr::addr_of_mut!((*live_tree).ns_owner_root.ns_list_head);
        tree.ns_owner_root.ns_list_head = bindings::list_head { next: owned, prev: owned };
        value
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
