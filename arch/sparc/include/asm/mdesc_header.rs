/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux::types, linux::cpumask, and asm::prom.

#[repr(C)]
pub struct mdesc_handle {
    _private: [u8; 0],
}

/* Machine description operations are to be surrounded by grab and
 * release calls.  The mdesc_handle returned from the grab is
 * the first argument to all of the operational calls that work
 * on mdescs.
 */
extern "C" {
    pub fn mdesc_grab() -> *mut mdesc_handle;
    pub fn mdesc_release(handle: *mut mdesc_handle);
}

pub const MDESC_NODE_NULL: u64 = !0u64;
pub const MDESC_MAX_STR_LEN: usize = 256;

extern "C" {
    pub fn mdesc_node_by_name(
        handle: *mut mdesc_handle,
        from_node: u64,
        name: *const core::ffi::c_char,
    ) -> u64;
}

#[macro_export]
macro_rules! mdesc_for_each_node_by_name {
    ($hdl:expr, $node:ident, $name:expr) => {
        loop {
            $node = unsafe {
                $crate::mdesc_node_by_name($hdl, $crate::MDESC_NODE_NULL, $name)
            };
            if $node == $crate::MDESC_NODE_NULL {
                break;
            }
            $node = unsafe { $crate::mdesc_node_by_name($hdl, $node, $name) };
        }
    };
}

/* Access to property values returned from mdesc_get_property() are
 * only valid inside of a mdesc_grab()/mdesc_release() sequence.
 * Once mdesc_release() is called, the memory backed up by these
 * pointers may reference freed up memory.
 *
 * Therefore callers must make copies of any property values
 * they need.
 *
 * These same rules apply to mdesc_node_name().
 */
extern "C" {
    pub fn mdesc_get_property(
        handle: *mut mdesc_handle,
        node: u64,
        name: *const core::ffi::c_char,
        lenp: *mut core::ffi::c_int,
    ) -> *const core::ffi::c_void;
    pub fn mdesc_node_name(
        hp: *mut mdesc_handle,
        node: u64,
    ) -> *const core::ffi::c_char;
}

/* MD arc iteration, the standard sequence is:
 *
 *	unsigned long arc;
 *	mdesc_for_each_arc(arc, handle, node, MDESC_ARC_TYPE_{FWD,BACK}) {
 *		unsigned long target = mdesc_arc_target(handle, arc);
 *		...
 *	}
 */

pub const MDESC_ARC_TYPE_FWD: &[u8] = b"fwd\0";
pub const MDESC_ARC_TYPE_BACK: &[u8] = b"back\0";

extern "C" {
    pub fn mdesc_next_arc(
        handle: *mut mdesc_handle,
        from: u64,
        arc_type: *const core::ffi::c_char,
    ) -> u64;
}

#[macro_export]
macro_rules! mdesc_for_each_arc {
    ($arc:ident, $hdl:expr, $node:expr, $type_:expr) => {
        loop {
            $arc = unsafe { $crate::mdesc_next_arc($hdl, $node, $type_) };
            if $arc == $crate::MDESC_NODE_NULL {
                break;
            }
            $arc = unsafe { $crate::mdesc_next_arc($hdl, $arc, $type_) };
        }
    };
}

extern "C" {
    pub fn mdesc_arc_target(hp: *mut mdesc_handle, arc: u64) -> u64;
    pub fn mdesc_update();
}

#[repr(C)]
pub struct mdesc_notifier_client {
    pub add: Option<unsafe extern "C" fn(
        handle: *mut mdesc_handle,
        node: u64,
        node_name: *const core::ffi::c_char,
    )>,
    pub remove: Option<unsafe extern "C" fn(
        handle: *mut mdesc_handle,
        node: u64,
        node_name: *const core::ffi::c_char,
    )>,
    pub node_name: *const core::ffi::c_char,
    pub next: *mut mdesc_notifier_client,
}

extern "C" {
    pub fn mdesc_register_notifier(client: *mut mdesc_notifier_client);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdev_port {
    pub id: u64,
    pub parent_cfg_hdl: u64,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds_port {
    pub id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union md_node_info {
    pub vdev_port: vdev_port,
    pub ds_port: ds_port,
}

extern "C" {
    pub fn mdesc_get_node(
        hp: *mut mdesc_handle,
        node_name: *const core::ffi::c_char,
        node_info: *mut md_node_info,
    ) -> u64;
    pub fn mdesc_get_node_info(
        hp: *mut mdesc_handle,
        node: u64,
        node_name: *const core::ffi::c_char,
        node_info: *mut md_node_info,
    ) -> core::ffi::c_int;
    pub fn mdesc_fill_in_cpu_data(mask: *mut cpumask_t);
    pub fn mdesc_populate_present_mask(mask: *mut cpumask_t);
    pub fn mdesc_get_page_sizes(mask: *mut cpumask_t, pgsz_mask: *mut usize);
    pub fn sun4v_mdesc_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
