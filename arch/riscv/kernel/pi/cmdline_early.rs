// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_void};

extern "C" {
    fn fdt_path_offset(fdt: *const c_void, path: *const c_char) -> i32;
    fn fdt_getprop(
        fdt: *const c_void,
        nodeoffset: i32,
        name: *const c_char,
        lenp: *mut i32,
    ) -> *const c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

// CONFIG_CMDLINE_FORCE, CONFIG_CMDLINE_EXTEND, CONFIG_CMDLINE_FALLBACK,
// CONFIG_CMDLINE, COMMAND_LINE_SIZE, SATP_MODE_39, and SATP_MODE_48 are
// supplied by the kernel build configuration and architecture headers.

static mut early_cmdline: [c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];

unsafe fn get_early_cmdline(dtb_pa: usize) -> *mut c_char {
    let mut fdt_cmdline: *const c_char;
    let mut fdt_cmdline_size: isize = 0;
    let mut chosen_node: i32;

    // !IS_ENABLED(CONFIG_CMDLINE_FORCE)
    if !cfg!(feature = "CONFIG_CMDLINE_FORCE") {
        chosen_node = fdt_path_offset(dtb_pa as *const c_void, b"/chosen\0".as_ptr() as *const c_char);
        if chosen_node >= 0 {
            fdt_cmdline = fdt_getprop(
                dtb_pa as *const c_void,
                chosen_node,
                b"bootargs\0".as_ptr() as *const c_char,
                core::ptr::null_mut(),
            );
            if !fdt_cmdline.is_null() {
                fdt_cmdline_size = strscpy(early_cmdline.as_mut_ptr(), fdt_cmdline, COMMAND_LINE_SIZE);
                if fdt_cmdline_size < 0 {
                    return early_cmdline.as_mut_ptr();
                }
            }
        }
    }

    if cfg!(feature = "CONFIG_CMDLINE_EXTEND")
        || cfg!(feature = "CONFIG_CMDLINE_FORCE")
        || fdt_cmdline_size == 0 // CONFIG_CMDLINE_FALLBACK
    {
        strscpy(
            early_cmdline.as_mut_ptr().add(fdt_cmdline_size as usize),
            CONFIG_CMDLINE.as_ptr() as *const c_char,
            COMMAND_LINE_SIZE - fdt_cmdline_size as usize,
        );
    }

    early_cmdline.as_mut_ptr()
}

unsafe fn match_noXlvl(cmdline: *mut c_char) -> u64 {
    if !strstr(cmdline, b"no4lvl\0".as_ptr() as *const c_char).is_null() {
        SATP_MODE_39
    } else if !strstr(cmdline, b"no5lvl\0".as_ptr() as *const c_char).is_null() {
        SATP_MODE_48
    } else {
        0
    }
}

pub unsafe fn set_satp_mode_from_cmdline(dtb_pa: usize) -> u64 {
    let cmdline = get_early_cmdline(dtb_pa);

    match_noXlvl(cmdline)
}

unsafe fn match_nokaslr(cmdline: *mut c_char) -> bool {
    !strstr(cmdline, b"nokaslr\0".as_ptr() as *const c_char).is_null()
}

pub unsafe fn set_nokaslr_from_cmdline(dtb_pa: usize) -> bool {
    let cmdline = get_early_cmdline(dtb_pa);

    match_nokaslr(cmdline)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
