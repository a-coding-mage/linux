/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: this header uses kernel types and the kernel `unlikely`
// helper supplied by other files.

/* Call a function once. Similar to DO_ONCE(), but does not use jump label
 * patching via static keys.
 */
#[macro_export]
macro_rules! DO_ONCE_LITE {
    ($func:path $(, $args:expr)*) => {{
        $crate::DO_ONCE_LITE_IF!(true, $func $(, $args)*)
    }};
}

#[macro_export]
macro_rules! __ONCE_LITE_IF {
    ($condition:expr) => {{
        #[link_section = ".data..once"]
        static mut __ALREADY_DONE: bool = false;
        let __ret_cond: bool = ($condition) as bool;
        let mut __ret_once: bool = false;

        if $crate::unlikely(__ret_cond) && $crate::unlikely(unsafe { !__ALREADY_DONE }) {
            unsafe {
                __ALREADY_DONE = true;
            }
            __ret_once = true;
        }
        $crate::unlikely(__ret_once)
    }};
}

#[macro_export]
macro_rules! DO_ONCE_LITE_IF {
    ($condition:expr, $func:path $(, $args:expr)*) => {{
        let __ret_do_once: bool = ($condition) as bool;

        if $crate::__ONCE_LITE_IF!(__ret_do_once) {
            $func($($args),*);
        }

        $crate::unlikely(__ret_do_once)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
