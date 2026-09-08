// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005.
 */

// Dependencies supplied by the surrounding translation unit/project.

use std::ffi::CStr;

extern "C" {
    fn opendir(dirname: *const libc::c_char) -> *mut libc::DIR;
    fn readdir(dirp: *mut libc::DIR) -> *mut libc::dirent;
    fn closedir(dirp: *mut libc::DIR) -> libc::c_int;
    fn stat(path: *const libc::c_char, buf: *mut libc::stat) -> libc::c_int;
    fn fopen(path: *const libc::c_char, mode: *const libc::c_char) -> *mut libc::FILE;
    fn fclose(stream: *mut libc::FILE) -> libc::c_int;
    fn fprintf(stream: *mut libc::FILE, format: *const libc::c_char, ...) -> libc::c_int;
    fn strerror(errnum: libc::c_int) -> *mut libc::c_char;
    fn free(ptr: *mut libc::c_void);

    fn die(format: *const libc::c_char, ... ) -> !;
    fn build_node(
        name: *const libc::c_char,
        bus: *const libc::c_char,
        prop: *const libc::c_char,
    ) -> *mut node;
    fn streq(a: *const libc::c_char, b: *const libc::c_char) -> libc::c_int;
    fn join_path(dirname: *const libc::c_char, name: *const libc::c_char) -> *mut libc::c_char;
    fn build_property(
        name: *const libc::c_char,
        val: data,
        next: *mut libc::c_void,
    ) -> *mut property;
    fn data_copy_file(file: *mut libc::FILE, size: libc::off_t) -> data;
    fn add_property(tree: *mut node, prop: *mut property);
    fn name_node(tree: *mut node, name: *mut libc::c_char) -> *mut node;
    fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn add_child(tree: *mut node, child: *mut node);
    fn build_dt_info(
        flags: libc::c_uint,
        boot_cpuid_phys: *mut libc::c_void,
        tree: *mut node,
        boot_cpuid: libc::c_uint,
    ) -> *mut dt_info;
    fn guess_boot_cpuid(tree: *mut node) -> libc::c_uint;
}

#[repr(C)]
pub struct node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct property {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dt_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct data {
    _private: [u8; 0],
}

const DTSF_V1: libc::c_uint = 0x00000001;

unsafe fn read_fstree(dirname: *const libc::c_char) -> *mut node {
    let d: *mut libc::DIR;
    let mut de: *mut libc::dirent;
    let mut st: libc::stat = std::mem::zeroed();
    let tree: *mut node;

    d = opendir(dirname);
    if d.is_null() {
        die(
            b"Couldn't opendir() \"%s\": %s\n\0".as_ptr() as *const libc::c_char,
            dirname,
            strerror(*libc::__errno_location()),
        );
    }

    tree = build_node(std::ptr::null(), std::ptr::null(), std::ptr::null());

    loop {
        de = readdir(d);
        if de.is_null() {
            break;
        }

        let tmpname: *mut libc::c_char;
        let entry_name = (*de).d_name.as_ptr();

        if streq(entry_name, b".\0".as_ptr() as *const libc::c_char) != 0
            || streq(entry_name, b"..\0".as_ptr() as *const libc::c_char) != 0
        {
            continue;
        }

        tmpname = join_path(dirname, entry_name);

        if stat(tmpname, &mut st) < 0 {
            die(
                b"stat(%s): %s\n\0".as_ptr() as *const libc::c_char,
                tmpname,
                strerror(*libc::__errno_location()),
            );
        }

        if (st.st_mode & libc::S_IFMT) == libc::S_IFREG {
            let prop: *mut property;
            let pfile: *mut libc::FILE;

            pfile = fopen(tmpname, b"rb\0".as_ptr() as *const libc::c_char);
            if pfile.is_null() {
                fprintf(
                    libc::stderr,
                    b"WARNING: Cannot open %s: %s\n\0".as_ptr() as *const libc::c_char,
                    tmpname,
                    strerror(*libc::__errno_location()),
                );
            } else {
                prop = build_property(
                    entry_name,
                    data_copy_file(pfile, st.st_size),
                    std::ptr::null_mut(),
                );
                add_property(tree, prop);
                fclose(pfile);
            }
        } else if (st.st_mode & libc::S_IFMT) == libc::S_IFDIR {
            let mut newchild: *mut node;

            newchild = read_fstree(tmpname);
            newchild = name_node(newchild, xstrdup(entry_name));
            add_child(tree, newchild);
        }

        free(tmpname as *mut libc::c_void);
    }

    closedir(d);
    tree
}

#[no_mangle]
pub unsafe extern "C" fn dt_from_fs(dirname: *const libc::c_char) -> *mut dt_info {
    let mut tree: *mut node;

    tree = read_fstree(dirname);
    tree = name_node(tree, b"\0".as_ptr() as *mut libc::c_char);

    build_dt_info(DTSF_V1, std::ptr::null_mut(), tree, guess_boot_cpuid(tree))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
