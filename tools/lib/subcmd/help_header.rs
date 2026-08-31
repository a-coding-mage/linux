/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct cmdnames {
    pub alloc: usize,
    pub cnt: usize,
    pub names: *mut *mut cmdname,
}

#[repr(C)]
pub struct cmdname {
    /* also used for similarity index in help.c */
    pub len: usize,
    pub name: [c_char; 0],
}

unsafe extern "C" {
    pub fn putchar(c: c_int) -> c_int;
}

#[inline]
pub unsafe fn mput_char(c: c_char, mut num: c_uint) {
    while num != 0 {
        num = num.wrapping_sub(1);
        unsafe {
            putchar(c as c_int);
        }
    }
}

unsafe extern "C" {
    pub fn load_command_list(
        prefix: *const c_char,
        main_cmds: *mut cmdnames,
        other_cmds: *mut cmdnames,
    );
    pub fn add_cmdname(cmds: *mut cmdnames, name: *const c_char, len: usize);
    pub fn clean_cmdnames(cmds: *mut cmdnames);
    pub fn cmdname_compare(a: *const c_void, b: *const c_void) -> c_int;
    pub fn uniq(cmds: *mut cmdnames);
    /* Here we require that excludes is a sorted list. */
    pub fn exclude_cmds(cmds: *mut cmdnames, excludes: *mut cmdnames);
    pub fn is_in_cmdlist(c: *mut cmdnames, s: *const c_char) -> c_int;
    pub fn list_commands(
        title: *const c_char,
        main_cmds: *mut cmdnames,
        other_cmds: *mut cmdnames,
    );
}
