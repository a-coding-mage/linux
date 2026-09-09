/* SPDX-License-Identifier: GPL-2.0 */
// Translated from <uapi/linux/irqnr.h> dependencies are supplied externally.

#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}

/// C declaration marked `__pure`.
unsafe extern "C" {
    pub fn irq_get_nr_irqs() -> ::core::ffi::c_uint;
    pub fn irq_set_nr_irqs(nr: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn irq_to_desc(irq: ::core::ffi::c_uint) -> *mut irq_desc;
    pub fn irq_get_next_irq(offset: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
}

#[macro_export]
macro_rules! for_each_irq_desc {
    ($irq:ident, $desc:ident, $body:block) => {{
        let mut __nr_irqs__: ::core::ffi::c_uint = unsafe { $crate::irq_get_nr_irqs() };
        while __nr_irqs__ != 0 {
            $irq = 0;
            $desc = unsafe { $crate::irq_to_desc($irq) };
            while $irq < __nr_irqs__ {
                if !$desc.is_null() $body
                $irq = $irq.wrapping_add(1);
                $desc = unsafe { $crate::irq_to_desc($irq) };
            }
            __nr_irqs__ = 0;
        }
    }};
}

#[macro_export]
macro_rules! for_each_irq_desc_reverse {
    ($irq:ident, $desc:ident, $body:block) => {{
        $irq = unsafe { $crate::irq_get_nr_irqs() }.wrapping_sub(1);
        $desc = unsafe { $crate::irq_to_desc($irq) };
        // The original C loop uses an unsigned irq and therefore its
        // `irq >= 0` condition is always true, preserving unsigned wraparound.
        while $irq >= 0 {
            if !$desc.is_null() $body
            $irq = $irq.wrapping_sub(1);
            $desc = unsafe { $crate::irq_to_desc($irq) };
        }
    }};
}

#[macro_export]
macro_rules! for_each_active_irq {
    ($irq:ident, $body:block) => {{
        let mut __nr_irqs__: ::core::ffi::c_uint = unsafe { $crate::irq_get_nr_irqs() };
        while __nr_irqs__ != 0 {
            $irq = unsafe { $crate::irq_get_next_irq(0) };
            while $irq < __nr_irqs__ {
                $body
                $irq = unsafe { $crate::irq_get_next_irq($irq.wrapping_add(1)) };
            }
            __nr_irqs__ = 0;
        }
    }};
}

#[macro_export]
macro_rules! for_each_irq_nr {
    ($irq:ident, $body:block) => {{
        let mut __nr_irqs__: ::core::ffi::c_uint = unsafe { $crate::irq_get_nr_irqs() };
        while __nr_irqs__ != 0 {
            $irq = 0;
            while $irq < __nr_irqs__ {
                $body
                $irq = $irq.wrapping_add(1);
            }
            __nr_irqs__ = 0;
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
