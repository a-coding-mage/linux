// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "test_task_pt_regs.skel.h"

use core::arch::asm;
use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of_val;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_task_pt_regs {
    pub bss: *mut test_task_pt_regs__bss,
    pub progs: test_task_pt_regs__progs,
    pub links: test_task_pt_regs__links,
}

#[repr(C)]
pub struct test_task_pt_regs__progs {
    pub handle_uprobe: *mut bpf_program,
}

#[repr(C)]
pub struct test_task_pt_regs__links {
    pub handle_uprobe: *mut bpf_link,
}

#[repr(C)]
pub struct test_task_pt_regs__bss {
    pub uprobe_res: i32,
    pub current_regs: pt_regs,
    pub ctx_regs: pt_regs,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn get_uprobe_offset(func: *const c_void) -> isize;
    fn test_task_pt_regs__open_and_load() -> *mut test_task_pt_regs;
    fn test_task_pt_regs__destroy(skel: *mut test_task_pt_regs);
    fn bpf_program__attach_uprobe(
        prog: *mut bpf_program,
        retprobe: bool,
        pid: c_int,
        binary_path: *const c_char,
        func_offset: isize,
    ) -> *mut bpf_link;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn ASSERT_GE(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: i32, expected: i32, name: *const c_char) -> bool;
    fn ASSERT_TRUE(condition: bool, name: *const c_char) -> bool;
}

/* uprobe attach point */
#[inline(never)]
unsafe extern "C" fn trigger_func() {
    unsafe {
        asm!("", options(nomem, nostack, preserves_flags));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_pt_regs() {
    let skel: *mut test_task_pt_regs;
    let uprobe_link: *mut bpf_link;
    let uprobe_offset: isize;
    let match_: bool;

    const UPROBE_OFFSET: &[u8] = b"uprobe_offset\0";
    const SKEL_OPEN: &[u8] = b"skel_open\0";
    const CHECK_BSS: &[u8] = b"check_bss\0";
    const PROC_SELF_EXE: &[u8] = b"/proc/self/exe\0";
    const ATTACH_UPROBE: &[u8] = b"attach_uprobe\0";
    const CHECK_UPROBE_RES: &[u8] = b"check_uprobe_res\0";
    const CHECK_REGS_MATCH: &[u8] = b"check_regs_match\0";

    unsafe {
        uprobe_offset = get_uprobe_offset(trigger_func as *const c_void);
        if !ASSERT_GE(uprobe_offset, 0, UPROBE_OFFSET.as_ptr() as *const c_char) {
            return;
        }

        skel = test_task_pt_regs__open_and_load();
        if !ASSERT_OK_PTR(skel as *const c_void, SKEL_OPEN.as_ptr() as *const c_char) {
            return;
        }
        if !ASSERT_OK_PTR((*skel).bss as *const c_void, CHECK_BSS.as_ptr() as *const c_char) {
            test_task_pt_regs__destroy(skel);
            return;
        }

        uprobe_link = bpf_program__attach_uprobe(
            (*skel).progs.handle_uprobe,
            false, /* retprobe */
            0,     /* self pid */
            PROC_SELF_EXE.as_ptr() as *const c_char,
            uprobe_offset,
        );
        if !ASSERT_OK_PTR(
            uprobe_link as *const c_void,
            ATTACH_UPROBE.as_ptr() as *const c_char,
        ) {
            test_task_pt_regs__destroy(skel);
            return;
        }
        (*skel).links.handle_uprobe = uprobe_link;

        /* trigger & validate uprobe */
        trigger_func();

        if !ASSERT_EQ(
            (*(*skel).bss).uprobe_res,
            1,
            CHECK_UPROBE_RES.as_ptr() as *const c_char,
        ) {
            test_task_pt_regs__destroy(skel);
            return;
        }

        match_ = memcmp(
            &(*(*skel).bss).current_regs as *const pt_regs as *const c_void,
            &(*(*skel).bss).ctx_regs as *const pt_regs as *const c_void,
            size_of_val(&(*(*skel).bss).current_regs),
        ) == 0;
        ASSERT_TRUE(match_, CHECK_REGS_MATCH.as_ptr() as *const c_char);

        test_task_pt_regs__destroy(skel);
    }
}
