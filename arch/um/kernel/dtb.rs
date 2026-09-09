// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_longlong, c_void};

// Declarations supplied by the kernel and UML architecture headers.
unsafe extern "C" {
    fn uml_load_file(name: *mut c_char, size: *mut c_longlong) -> *mut c_void;
    fn early_init_dt_scan(area: *mut c_void, address: usize) -> c_int;
    fn early_init_fdt_scan_reserved_mem();
    fn memblock_free(area: *mut c_void, size: c_longlong);
    fn unflatten_device_tree();
    fn __pa(area: *mut c_void) -> usize;
    fn pr_err(format: *const c_char, ...);
}

// `__initdata` is a linker/build-time annotation in the C source.
static mut dtb: *mut c_char = core::ptr::null_mut();

// `__init` is a linker/build-time annotation in the C source.
pub unsafe fn uml_dtb_init() {
    let mut size: c_longlong;
    let area: *mut c_void;

    area = uml_load_file(dtb, &mut size);
    if !area.is_null() {
        if early_init_dt_scan(area, __pa(area)) == 0 {
            // `pr_err` is a C variadic logging interface.
            pr_err(
                b"invalid DTB %s\n\0".as_ptr() as *const c_char,
                dtb,
            );
            memblock_free(area, size);
            return;
        }

        early_init_fdt_scan_reserved_mem();
    }

    unflatten_device_tree();
}

// `__init` is a linker/build-time annotation in the C source.
unsafe fn uml_dtb_setup(line: *mut c_char, add: *mut c_int) -> c_int {
    *add = 0;
    dtb = line;
    0
}

// Equivalent registration performed by the C `__uml_setup` macro:
// __uml_setup("dtb=", uml_dtb_setup,
// "dtb=<file>\n"
// "    Boot the kernel with the devicetree blob from the specified file.\n\n"
// );
const UML_DTB_SETUP_PREFIX: &[u8] = b"dtb=\0";
const UML_DTB_SETUP_HELP: &[u8] =
    b"dtb=<file>\n    Boot the kernel with the devicetree blob from the specified file.\n\n\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
