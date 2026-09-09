/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/*
 * Dependency: irq_handler_t is supplied by the Linux interrupt interfaces.
 */

/***************************** Prototypes *****************************/

unsafe extern "C" {
    pub fn stdma_try_lock(handler: irq_handler_t, data: *mut c_void) -> i32;
    pub fn stdma_lock(handler: irq_handler_t, data: *mut c_void);
    pub fn stdma_release();
    pub fn stdma_islocked() -> i32;
    pub fn stdma_is_locked_by(handler: irq_handler_t) -> i32;
    pub fn stdma_init();
}

/************************* End of Prototypes **************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
