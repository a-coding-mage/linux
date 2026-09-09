/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/arch/alpha/kernel/err_impl.h
 *
 * Contains declarations and macros to support Alpha error handling
 * implementations.
 */

// Dependency supplied by <asm/mce.h>.

#[repr(C)]
pub union el_timestamp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct el_subpacket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ev7_lf_subpackets {
    _private: [u8; 0],
}

#[repr(C)]
pub struct el_common {
    _private: [u8; 0],
}

#[repr(C)]
pub struct el_subpacket_annotation {
    pub next: *mut el_subpacket_annotation,
    pub class: u16,
    pub type_: u16,
    pub revision: u16,
    pub description: *mut core::ffi::c_char,
    pub annotation: *mut *mut core::ffi::c_char,
}

#[macro_export]
macro_rules! SUBPACKET_ANNOTATION {
    ($c:expr, $t:expr, $r:expr, $d:expr, $a:expr) => {
        $crate::el_subpacket_annotation {
            next: core::ptr::null_mut(),
            class: $c,
            type_: $t,
            revision: $r,
            description: $d,
            annotation: $a,
        }
    };
}

#[repr(C)]
pub struct el_subpacket_handler {
    pub next: *mut el_subpacket_handler,
    pub class: u16,
    pub handler: Option<unsafe extern "C" fn(*mut el_subpacket) -> *mut el_subpacket>,
}

#[macro_export]
macro_rules! SUBPACKET_HANDLER_INIT {
    ($c:expr, $h:expr) => {
        $crate::el_subpacket_handler {
            next: core::ptr::null_mut(),
            class: $c,
            handler: $h,
        }
    };
}

/* Field extraction and in-position mask helpers. The referenced field must
 * provide Rust constants named <field>__S and <field>__M. */
#[macro_export]
macro_rules! EXTRACT {
    ($u:expr, $f:ident) => {
        (($u >> $f##__S) & $f##__M)
    };
}

#[macro_export]
macro_rules! GEN_MASK {
    ($f:ident) => {
        (($f##__M as u64) << $f##__S)
    };
}

extern "C" {
    pub static mut err_print_prefix: *mut core::ffi::c_char;

    pub fn mchk_dump_mem(
        address: *mut core::ffi::c_void,
        size: usize,
        annotation: *mut *mut core::ffi::c_char,
    );
    pub fn mchk_dump_logout_frame(frame: *mut el_common);
    pub fn el_print_timestamp(timestamp: *mut el_timestamp);
    pub fn el_process_subpackets(subpacket: *mut el_subpacket, packet_count: i32);
    pub fn el_process_subpacket(subpacket: *mut el_subpacket) -> *mut el_subpacket;
    pub fn el_annotate_subpacket(subpacket: *mut el_subpacket);
    pub fn cdl_check_console_data_log();
    pub fn cdl_register_subpacket_annotation(annotation: *mut el_subpacket_annotation) -> i32;
    pub fn cdl_register_subpacket_handler(handler: *mut el_subpacket_handler) -> i32;

    pub fn ev7_collect_logout_frame_subpackets(
        subpacket: *mut el_subpacket,
        subpackets: *mut ev7_lf_subpackets,
    ) -> *mut ev7_lf_subpackets;
    pub fn ev7_register_error_handlers();
    pub fn ev7_machine_check(arg1: u64, arg2: u64);

    pub fn ev6_register_error_handlers();
    pub fn ev6_process_logout_frame(frame: *mut el_common, arg: i32) -> i32;
    pub fn ev6_machine_check(arg1: u64, arg2: u64);

    pub fn marvel_machine_check(arg1: u64, arg2: u64);
    pub fn marvel_register_error_handlers();

    pub fn titan_process_logout_frame(frame: *mut el_common, arg: i32) -> i32;
    pub fn titan_machine_check(arg1: u64, arg2: u64);
    pub fn titan_register_error_handlers();
    pub fn privateer_process_logout_frame(frame: *mut el_common, arg: i32) -> i32;
    pub fn privateer_machine_check(arg1: u64, arg2: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
