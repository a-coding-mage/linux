/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies are supplied by the surrounding translation unit.
// __init annotations are intentionally omitted for the EFI stub.

#[cfg(not(any()))]
const _EFI_ALLOC_ALIGN_DEFAULT: usize = EFI_PAGE_SIZE;

pub const EFI_ALLOCATE_ANY_PAGES: u32 = 0;
pub const EFI_ALLOCATE_MAX_ADDRESS: u32 = 1;
pub const EFI_ALLOCATE_ADDRESS: u32 = 2;
pub const EFI_MAX_ALLOCATE_TYPE: u32 = 3;

pub const EFI_LOCATE_ALL_HANDLES: u32 = 0;
pub const EFI_LOCATE_BY_REGISTER_NOTIFY: u32 = 1;
pub const EFI_LOCATE_BY_PROTOCOL: u32 = 2;

pub const EFI_USEC_PER_SEC: u32 = 1_000_000;
pub const EFI_100NSEC_PER_USEC: u64 = 10;
pub const EFI_MMAP_NR_SLACK_SLOTS: u32 = 32;

pub const EFI_EVT_TIMER: u32 = 0x8000_0000;
pub const EFI_EVT_RUNTIME: u32 = 0x4000_0000;
pub const EFI_EVT_NOTIFY_WAIT: u32 = 0x0000_0100;
pub const EFI_EVT_NOTIFY_SIGNAL: u32 = 0x0000_0200;

pub struct edid_info;
pub struct screen_info;
pub struct sysfb_display_info;

extern "C" {
    pub static mut efi_no5lvl: bool;
    pub static mut efi_nochunk: bool;
    pub static mut efi_nokaslr: bool;
    pub static mut efi_loglevel: i32;
    pub static mut efi_mem_encrypt: i32;
    pub static mut efi_novamap: bool;
    pub static efi_system_table: *const efi_system_table_t;
    pub static efi_dxe_table: *const efi_dxe_services_table_t;

    pub fn efi_pe_entry(
        handle: efi_handle_t,
        sys_table_arg: *mut efi_system_table_t,
    ) -> efi_status_t;
}

pub type efi_dxe_services_table_t = efi_dxe_services_table;
pub type efi_device_path_protocol_t = efi_generic_dev_path;
pub type efi_event_t = *mut core::ffi::c_void;
pub type efi_event_notify_t = unsafe extern "C" fn(efi_event_t, *mut core::ffi::c_void);

#[repr(C)]
pub union efi_device_path_to_text_protocol {
    pub functions: efi_device_path_to_text_protocol_functions,
    pub mixed_mode: efi_device_path_to_text_protocol_mixed_mode,
}

#[repr(C)]
pub struct efi_device_path_to_text_protocol_functions {
    pub convert_device_node_to_text: unsafe extern "C" fn(
        *const efi_device_path_protocol_t,
        bool,
        bool,
    ) -> *mut efi_char16_t,
    pub convert_device_path_to_text: unsafe extern "C" fn(
        *const efi_device_path_protocol_t,
        bool,
        bool,
    ) -> *mut efi_char16_t,
}

#[repr(C)]
pub struct efi_device_path_to_text_protocol_mixed_mode {
    pub convert_device_node_to_text: u32,
    pub convert_device_path_to_text: u32,
}

pub type efi_device_path_to_text_protocol_t = efi_device_path_to_text_protocol;

#[repr(C)]
pub union efi_device_path_from_text_protocol {
    pub functions: efi_device_path_from_text_protocol_functions,
    pub mixed_mode: efi_device_path_from_text_protocol_mixed_mode,
}

#[repr(C)]
pub struct efi_device_path_from_text_protocol_functions {
    pub convert_text_to_device_node: unsafe extern "C" fn(
        *const efi_char16_t,
    ) -> *mut efi_device_path_protocol_t,
    pub convert_text_to_device_path: unsafe extern "C" fn(
        *const efi_char16_t,
    ) -> *mut efi_device_path_protocol_t,
}

#[repr(C)]
pub struct efi_device_path_from_text_protocol_mixed_mode {
    pub convert_text_to_device_node: u32,
    pub convert_text_to_device_path: u32,
}

pub type efi_device_path_from_text_protocol_t = efi_device_path_from_text_protocol;

#[inline]
pub unsafe fn efi_set_u64_split(data: u64, lo: *mut u32, hi: *mut u32) {
    *lo = data as u32;
    *hi = (data >> 32) as u32;
}

#[inline]
pub unsafe fn efi_set_event_at(events: *mut efi_event_t, idx: usize, event: efi_event_t) {
    if efi_is_native() {
        *events.add(idx) = event;
    } else {
        *(events as *mut u32).add(idx) = event as usize as u32;
    }
}

#[macro_export]
macro_rules! efi_is_native { () => { true }; }

#[macro_export]
macro_rules! efi_get_handle_at {
    ($array:expr, $idx:expr) => {
        if $crate::efi_is_native!() {
            ($array)[$idx]
        } else {
            (($array as *const u32)[$idx] as usize) as efi_handle_t
        }
    };
}

#[macro_export]
macro_rules! efi_get_handle_num {
    ($size:expr) => {
        ($size) / if $crate::efi_is_native!() {
            core::mem::size_of::<efi_handle_t>()
        } else {
            core::mem::size_of::<u32>()
        }
    };
}

#[macro_export]
macro_rules! for_each_efi_handle {
    ($handle:ident, $array:expr, $num:expr, $body:block) => {{
        for __i in 0..($num) {
            $handle = $crate::efi_get_handle_at!($array, __i);
            $body
        }
    }};
}

#[macro_export]
macro_rules! efi_call_proto {
    ($inst:expr, $func:ident $(, $arg:expr)*) => {{
        let __inst = $inst;
        unsafe { (__inst.$func)(__inst $(, $arg)*) }
    }};
}

#[macro_export]
macro_rules! get_efi_var {
    ($name:expr, $vendor:expr $(, $arg:expr)*) => {
        unsafe { (efi_system_table.runtime.get_variable)(($name) as *mut efi_char16_t, ($vendor) as *mut efi_guid_t $(, $arg)*) }
    };
}

#[macro_export]
macro_rules! set_efi_var {
    ($name:expr, $vendor:expr $(, $arg:expr)*) => {
        unsafe { (efi_system_table.runtime.set_variable)(($name) as *mut efi_char16_t, ($vendor) as *mut efi_guid_t $(, $arg)*) }
    };
}

// These macros preserve the C EFI call and printk interfaces supplied by dependencies.
#[macro_export]
macro_rules! efi_bs_call { ($func:ident $(, $arg:expr)*) => { unsafe { (efi_system_table.boottime.$func)( $($arg),* ) } }; }
#[macro_export]
macro_rules! efi_rt_call { ($func:ident $(, $arg:expr)*) => { unsafe { (efi_system_table.runtime.$func)( $($arg),* ) } }; }
#[macro_export]
macro_rules! efi_dxe_call { ($func:ident $(, $arg:expr)*) => { unsafe { (efi_dxe_table.$func)( $($arg),* ) } }; }
#[macro_export]
macro_rules! efi_info { ($($arg:tt)*) => { efi_printk!(KERN_INFO, $($arg)*) }; }
#[macro_export]
macro_rules! efi_warn { ($($arg:tt)*) => { efi_printk!(KERN_WARNING, "WARNING: " $($arg)*) }; }
#[macro_export]
macro_rules! efi_err { ($($arg:tt)*) => { efi_printk!(KERN_ERR, "ERROR: " $($arg)*) }; }
#[macro_export]
macro_rules! efi_debug { ($($arg:tt)*) => { efi_printk!(KERN_DEBUG, "DEBUG: " $($arg)*) }; }

#[macro_export]
macro_rules! fdt_setprop_inplace_var {
    ($fdt:expr, $node_offset:expr, $name:expr, $var:expr) => {
        fdt_setprop_inplace!($fdt, $node_offset, $name, &$var, core::mem::size_of_val(&$var))
    };
}
#[macro_export]
macro_rules! fdt_setprop_var {
    ($fdt:expr, $node_offset:expr, $name:expr, $var:expr) => {
        fdt_setprop!($fdt, $node_offset, $name, &$var, core::mem::size_of_val(&$var))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
