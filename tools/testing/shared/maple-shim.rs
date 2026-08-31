// SPDX-License-Identifier: GPL-2.0-or-later

/* Very simple shim around the maple tree. */

// C dependencies:
// #include "maple-shared.h"
// #include <linux/slab.h>
// #include "../../../lib/maple_tree.c"

use core::ffi::c_void;

unsafe extern "C" {
    static mut maple_node_cache: *mut c_void;

    fn kmem_cache_free(cachep: *mut c_void, objp: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn maple_rcu_cb(head: *mut rcu_head) {
    let node: *mut maple_node = container_of!(head, maple_node, rcu);

    unsafe {
        kmem_cache_free(maple_node_cache, node.cast::<c_void>());
    }
}
