// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type ssize_t = isize;
type size_t = usize;

const PATH_MAX: usize = 4096;
const O_RDWR: c_int = 0o2;

#[repr(C)]
pub struct hlist_head {
    pub first: *mut c_void,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pv_ops_entry {
    pub targets: list_head,
    pub clean: bool,
}

#[repr(C)]
pub struct objtool_file {
    pub elf: *mut elf,
    pub insn_hash: [hlist_head; 1],
    pub retpoline_call_list: list_head,
    pub return_thunk_list: list_head,
    pub static_call_list: list_head,
    pub mcount_loc_list: list_head,
    pub endbr_list: list_head,
    pub call_list: list_head,
    pub ignore_unreachables: bool,
    pub hints: bool,
    pub pv_ops: *mut pv_ops_entry,
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub pv_target: list_head,
}

#[repr(C)]
pub struct objtool_opts {
    pub no_unreachable: bool,
    pub noinstr: bool,
}

unsafe extern "C" {
    static mut opts: objtool_opts;

    fn elf_open_read(filename: *const c_char, flags: c_int) -> *mut elf;
    fn init_signal_handler() -> c_int;
    fn exec_cmd_init(
        exec_name: *const c_char,
        prefix: *const c_char,
        exec_path: *const c_char,
        env_path: *const c_char,
    );
    fn pager_init(env: *const c_char);
    fn cmd_klp(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn objtool_run(argc: c_int, argv: *mut *const c_char) -> c_int;

    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
}

unsafe fn ERROR(msg: *const c_char) {
    unsafe extern "C" {
        fn ERROR(msg: *const c_char);
    }

    unsafe { ERROR(msg) };
}

unsafe fn hash_init(hash: *mut hlist_head) {
    unsafe {
        (*hash).first = ptr::null_mut();
    }
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

unsafe fn list_empty(head: *const list_head) -> bool {
    unsafe { (*head).next == head as *mut list_head }
}

unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    unsafe {
        let next = (*head).next;
        (*new).next = next;
        (*new).prev = head;
        (*next).prev = new;
        (*head).next = new;
    }
}

static mut file: objtool_file = objtool_file {
    elf: ptr::null_mut(),
    insn_hash: [hlist_head {
        first: ptr::null_mut(),
    }],
    retpoline_call_list: list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    },
    return_thunk_list: list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    },
    static_call_list: list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    },
    mcount_loc_list: list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    },
    endbr_list: list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    },
    call_list: list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    },
    ignore_unreachables: false,
    hints: false,
    pv_ops: ptr::null_mut(),
};

#[no_mangle]
pub unsafe extern "C" fn objtool_open_read(filename: *const c_char) -> *mut objtool_file {
    unsafe {
        if !file.elf.is_null() {
            ERROR(c"won't handle more than one file at a time".as_ptr());
            return ptr::null_mut();
        }

        file.elf = elf_open_read(filename, O_RDWR);
        if file.elf.is_null() {
            return ptr::null_mut();
        }

        hash_init(file.insn_hash.as_mut_ptr());
        INIT_LIST_HEAD(&mut file.retpoline_call_list);
        INIT_LIST_HEAD(&mut file.return_thunk_list);
        INIT_LIST_HEAD(&mut file.static_call_list);
        INIT_LIST_HEAD(&mut file.mcount_loc_list);
        INIT_LIST_HEAD(&mut file.endbr_list);
        INIT_LIST_HEAD(&mut file.call_list);
        file.ignore_unreachables = opts.no_unreachable;
        file.hints = false;

        &mut file
    }
}

#[no_mangle]
pub unsafe extern "C" fn objtool_pv_add(
    f: *mut objtool_file,
    idx: c_int,
    func: *mut symbol,
) -> c_int {
    unsafe {
        if !opts.noinstr {
            return 0;
        }

        if (*f).pv_ops.is_null() {
            ERROR(c"paravirt confusion".as_ptr());
            return -1;
        }

        /*
         * These functions will be patched into native code,
         * see paravirt_patch().
         */
        if strcmp((*func).name, c"_paravirt_nop".as_ptr()) == 0
            || strcmp((*func).name, c"_paravirt_ident_64".as_ptr()) == 0
        {
            return 0;
        }

        /* already added this function */
        if !list_empty(&(*func).pv_target) {
            return 0;
        }

        list_add(
            &mut (*func).pv_target,
            &mut (*(*f).pv_ops.add(idx as usize)).targets,
        );
        (*(*f).pv_ops.add(idx as usize)).clean = false;
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn top_level_dir(file_arg: *const c_char) -> *mut c_char {
    unsafe {
        let mut len: ssize_t;
        let self_len: ssize_t;
        let file_len: ssize_t;
        let mut self_buf = [0 as c_char; PATH_MAX];
        let str_ptr: *mut c_char;
        let mut i: c_int;

        len = readlink(
            c"/proc/self/exe".as_ptr(),
            self_buf.as_mut_ptr(),
            size_of::<[c_char; PATH_MAX]>() - 1,
        );
        if len <= 0 {
            return ptr::null_mut();
        }
        self_buf[len as usize] = b'\0' as c_char;

        i = 0;
        while i < 3 {
            let s = strrchr(self_buf.as_ptr(), b'/' as c_int);
            if s.is_null() {
                return ptr::null_mut();
            }
            *s = b'\0' as c_char;
            i += 1;
        }

        self_len = strlen(self_buf.as_ptr()) as ssize_t;
        file_len = strlen(file_arg) as ssize_t;

        str_ptr = malloc((self_len + file_len + 2) as size_t) as *mut c_char;
        if str_ptr.is_null() {
            return ptr::null_mut();
        }

        memcpy(
            str_ptr as *mut c_void,
            self_buf.as_ptr() as *const c_void,
            self_len as size_t,
        );
        *str_ptr.add(self_len as usize) = b'/' as c_char;
        strcpy(str_ptr.add(self_len as usize + 1), file_arg);

        str_ptr
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *const c_char) -> c_int {
    unsafe {
        let UNUSED: *const c_char = c"OBJTOOL_NOT_IMPLEMENTED".as_ptr();

        if init_signal_handler() != 0 {
            return -1;
        }

        /* libsubcmd init */
        exec_cmd_init(c"objtool".as_ptr(), UNUSED, UNUSED, UNUSED);
        pager_init(UNUSED);

        if argc > 1 && strcmp(*argv.add(1), c"klp".as_ptr()) == 0 {
            argc -= 1;
            argv = argv.add(1);
            return cmd_klp(argc, argv);
        }

        objtool_run(argc, argv)
    }
}
