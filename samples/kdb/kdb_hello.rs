/*
 * Created by: Jason Wessel <jason.wessel@windriver.com>
 *
 * Copyright (c) 2010 Wind River Systems, Inc.  All Rights Reserved.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// Dependencies supplied by the Linux kernel headers in the original source.

/*
 * All kdb shell command call backs receive argc and argv, where
 * argv[0] is the command the end user typed
 */
unsafe extern "C" {
    fn kdb_printf(format: *const core::ffi::c_char, ...);
    fn kdb_register(command: *mut kdbtab_t);
    fn kdb_unregister(command: *mut kdbtab_t);
}

// Supplied by <linux/kdb.h> in the original source.
type kdb_func_t = unsafe extern "C" fn(i32, *const *const core::ffi::c_char) -> i32;

#[repr(C)]
struct kdbtab_t {
    name: *const core::ffi::c_char,
    func: Option<kdb_func_t>,
    usage: *const core::ffi::c_char,
    help: *const core::ffi::c_char,
}

// Supplied by <linux/kdb.h> in the original source.
const KDB_ARGCOUNT: i32 = 1;

unsafe extern "C" fn kdb_hello_cmd(argc: i32, argv: *const *const core::ffi::c_char) -> i32 {
    if argc > 1 {
        return KDB_ARGCOUNT;
    }

    if argc != 0 {
        kdb_printf(b"Hello %s.\n\0".as_ptr() as *const core::ffi::c_char, *argv.add(1));
    } else {
        kdb_printf(b"Hello world!\n\0".as_ptr() as *const core::ffi::c_char);
    }

    0
}

static mut hello_cmd: kdbtab_t = kdbtab_t {
    name: b"hello\0".as_ptr() as *const core::ffi::c_char,
    func: Some(kdb_hello_cmd),
    usage: b"[string]\0".as_ptr() as *const core::ffi::c_char,
    help: b"Say Hello World or Hello [string]\0".as_ptr() as *const core::ffi::c_char,
};

unsafe extern "C" fn kdb_hello_cmd_init() -> i32 {
    /*
     * Registration of a dynamically added kdb command is done with
     * kdb_register().
     */
    kdb_register(&raw mut hello_cmd);
    0
}

unsafe extern "C" fn kdb_hello_cmd_exit() {
    kdb_unregister(&raw mut hello_cmd);
}

// module_init(kdb_hello_cmd_init);
// module_exit(kdb_hello_cmd_exit);

// MODULE_AUTHOR("WindRiver");
// MODULE_DESCRIPTION("KDB example to add a hello command");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
