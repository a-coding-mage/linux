/* SPDX-License-Identifier: GPL-2.0 */

/**
 * enum irqreturn - irqreturn type values
 * @IRQ_NONE:        interrupt was not from this device or was not handled
 * @IRQ_HANDLED:     interrupt was handled by this device
 * @IRQ_WAKE_THREAD: handler requests to wake the handler thread
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irqreturn {
    IRQ_NONE = 0 << 0,
    IRQ_HANDLED = 1 << 0,
    IRQ_WAKE_THREAD = 1 << 1,
}

pub type irqreturn_t = irqreturn;

macro_rules! IRQ_RETVAL {
    ($x:expr) => {
        if ($x) != 0 {
            irqreturn::IRQ_HANDLED
        } else {
            irqreturn::IRQ_NONE
        }
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
