/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// #include <linux/numa.h>
// #include <linux/pfn.h>
// #include <linux/cache.h>
// #include <linux/types.h>

unsafe extern "C" {
    pub static mut movable_node_enabled: bool;
}

#[inline]
pub unsafe fn movable_node_is_enabled() -> bool {
    unsafe { movable_node_enabled }
}
