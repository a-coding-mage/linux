// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <linux/slab.h>

pub static mut slab_state: slab_state = 0;

pub unsafe fn slab_is_available() -> bool {
    unsafe { slab_state >= UP }
}
