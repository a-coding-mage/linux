// SPDX-License-Identifier: GPL-2.0
/*
 *  Generic support for BUG()
 *
 *  This respects the following config options:
 *
 *  CONFIG_BUG - emit BUG traps.  Nothing happens without this.
 *  CONFIG_GENERIC_BUG - enable this code.
 *  CONFIG_GENERIC_BUG_RELATIVE_POINTERS - use 32-bit relative pointers for bug_addr and file
 *  CONFIG_DEBUG_BUGVERBOSE - emit full file+line information for each BUG
 *
 *  CONFIG_BUG and CONFIG_DEBUG_BUGVERBOSE are potentially user-settable
 *  (though they're generally always on).
 *
 *  CONFIG_GENERIC_BUG is set by each architecture using this code.
 *
 *  To use this, your architecture must:
 *
 *  1. Set up the config options:
 *     - Enable CONFIG_GENERIC_BUG if CONFIG_BUG
 *
 *  2. Implement BUG (and optionally BUG_ON, WARN, WARN_ON)
 *     - Define HAVE_ARCH_BUG
 *     - Implement BUG() to generate a faulting instruction
 *     - NOTE: struct bug_entry does not have "file" or "line" entries
 *       when CONFIG_DEBUG_BUGVERBOSE is not enabled, so you must generate
 *       the values accordingly.
 *
 *  3. Implement the trap
 *     - In the illegal instruction trap handler (typically), verify
 *       that the fault was in kernel mode, and call report_bug()
 *     - report_bug() will return whether it was a false alarm, a warning,
 *       or an actual bug.
 *     - You must implement the is_valid_bugaddr(bugaddr) callback which
 *       returns true if the eip is a real kernel address, and it points
 *       to the expected BUG trap instruction.
 *
 *    Jeremy Fitzhardinge <jeremy@goop.org> 2006
 */

// pr_fmt(fmt) is fmt: the printk formats below carry no prefix.

// Dependencies: linux/list.h, linux/module.h, linux/bug.h, linux/sched.h,
// linux/rculist.h, linux/ftrace.h, linux/context_tracking.h,
// kunit/test-bug.h.
//
// lib/Kconfig limits RUST_BUG to the x86-64 profile: relative bug-table
// pointers, HAVE_ARCH_BUG_FORMAT(_ARGS) and DEBUG_BUGVERBOSE. The arch
// __warn_args() va_list reconstruction stays native.

use core::mem::{offset_of, size_of};
use core::ptr::{addr_of, addr_of_mut, null, null_mut, read_volatile};
use kernel::bindings::{
    __warn, _printk, bug_entry, elf64_hdr, elf64_shdr, is_valid_bugaddr, list_head, module, pt_regs,
    rcu_read_lock, rcu_read_unlock, strcmp,
};
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// `struct module` embeds zero-sized lockdep keys; only pointers cross here.
#[allow(improper_ctypes)]
extern "C" {
    static mut __start___bug_table: [bug_entry; 0];
    static mut __stop___bug_table: [bug_entry; 0];

    fn lupos_bug_warn_rcu_enter() -> bool;
    fn lupos_bug_warn_rcu_exit(rcu: bool);
    fn lupos_bug_kunit_is_suppressed_warning(count: bool) -> bool;
    fn lupos_bug_disable_trace_on_warning();
    fn lupos_bug_warn_printf_args(fmt: *const c_char, regs: *mut pt_regs) -> bool;
    fn lupos_bug_list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn lupos_bug_list_del_rcu(entry: *mut list_head);
}

/// `enum bug_trap_type`; lib/bug_helpers.c asserts the values.
const BUG_TRAP_TYPE_NONE: c_uint = 0;
const BUG_TRAP_TYPE_WARN: c_uint = 1;
const BUG_TRAP_TYPE_BUG: c_uint = 2;

/// BUGFLAG_* from asm-generic/bug.h; lib/bug_helpers.c asserts them.
const BUGFLAG_WARNING: u16 = 1 << 0;
const BUGFLAG_ONCE: u16 = 1 << 1;
const BUGFLAG_DONE: u16 = 1 << 2;
const BUGFLAG_NO_CUT_HERE: u16 = 1 << 3;
const BUGFLAG_ARGS: u16 = 1 << 4;

/// bug_addr(): the CONFIG_GENERIC_BUG_RELATIVE_POINTERS displacement.
///
/// # Safety
/// `bug` points to a valid bug table entry.
unsafe fn bug_addr(bug: *const bug_entry) -> c_ulong {
    // SAFETY: bug is valid; the displacement is relative to its own field.
    unsafe {
        let disp = addr_of!((*bug).bug_addr_disp);
        (disp as c_ulong).wrapping_add((*disp) as c_ulong)
    }
}

/* Updates are protected by module mutex */
#[cfg(CONFIG_MODULES)]
static mut MODULE_BUG_LIST: list_head = list_head {
    next: addr_of_mut!(MODULE_BUG_LIST),
    prev: addr_of_mut!(MODULE_BUG_LIST),
};

/// # Safety
/// Called with module bug tables stable under RCU.
#[cfg(CONFIG_MODULES)]
unsafe fn module_find_bug(bugaddr: c_ulong) -> *mut bug_entry {
    // SAFETY: guard(rcu)(): the module list is walked under rcu_read_lock().
    unsafe {
        rcu_read_lock();
        let head = addr_of_mut!(MODULE_BUG_LIST);
        let mut pos = read_volatile(addr_of!((*head).next));
        while pos != head {
            let m = pos.byte_sub(offset_of!(module, bug_list)).cast::<module>();
            let mut bug = (*m).bug_table;
            let mut i: c_uint = 0;
            while i < (*m).num_bugs {
                if bugaddr == bug_addr(bug) {
                    rcu_read_unlock();
                    return bug;
                }
                i += 1;
                bug = bug.add(1);
            }
            pos = read_volatile(addr_of!((*pos).next));
        }
        rcu_read_unlock();
    }
    null_mut()
}

/// Records the module's __bug_table section and publishes it.
///
/// # Safety
/// `hdr` and `sechdrs` describe the loaded module image of `mod_`; called
/// under the module mutex.
#[cfg(CONFIG_MODULES)]
#[no_mangle]
pub unsafe extern "C" fn module_bug_finalize(hdr: *const elf64_hdr, sechdrs: *const elf64_shdr, mod_: *mut module) {
    // SAFETY: the section headers and strings belong to the module image.
    unsafe {
        (*mod_).bug_table = null_mut();
        (*mod_).num_bugs = 0;

        /* Find the __bug_table section, if present */
        let secstrings = hdr
            .cast::<c_char>()
            .add((*sechdrs.add(usize::from((*hdr).e_shstrndx))).sh_offset as usize);
        let mut i: c_uint = 1;
        while i < c_uint::from((*hdr).e_shnum) {
            let shdr = sechdrs.add(i as usize);
            if strcmp(secstrings.add((*shdr).sh_name as usize), c"__bug_table".as_ptr().cast()) == 0 {
                (*mod_).bug_table = (*shdr).sh_addr as *mut bug_entry;
                (*mod_).num_bugs = ((*shdr).sh_size / size_of::<bug_entry>() as u64) as c_uint;
                break;
            }
            i += 1;
        }

        /*
         * Strictly speaking this should have a spinlock to protect against
         * traversals, but since we only traverse on BUG()s, a spinlock
         * could potentially lead to deadlock and thus be counter-productive.
         * Thus, this uses RCU to safely manipulate the bug list, since BUG
         * must run in non-interruptive state.
         */
        lupos_bug_list_add_rcu(addr_of_mut!((*mod_).bug_list), addr_of_mut!(MODULE_BUG_LIST));
    }
}

/// # Safety
/// `mod_` was finalized by module_bug_finalize(); called under the module
/// mutex.
#[cfg(CONFIG_MODULES)]
#[no_mangle]
pub unsafe extern "C" fn module_bug_cleanup(mod_: *mut module) {
    // SAFETY: forwarded caller contract.
    unsafe { lupos_bug_list_del_rcu(addr_of_mut!((*mod_).bug_list)) };
}

#[cfg(not(CONFIG_MODULES))]
unsafe fn module_find_bug(_bugaddr: c_ulong) -> *mut bug_entry {
    null_mut()
}

/// Returns the file and line recorded in a bug table entry.
///
/// # Safety
/// `bug` is a valid entry; `file` and `line` point to writable storage.
#[no_mangle]
pub unsafe extern "C" fn bug_get_file_line(bug: *mut bug_entry, file: *mut *const c_char, line: *mut c_uint) {
    // SAFETY: CONFIG_DEBUG_BUGVERBOSE with relative pointers.
    unsafe {
        let disp = addr_of!((*bug).file_disp);
        *file = disp.cast::<c_char>().wrapping_offset(*disp as isize);
        *line = c_uint::from((*bug).line);
    }
}

/// # Safety
/// `bug` is a valid entry.
unsafe fn bug_get_format(bug: *mut bug_entry) -> *const c_char {
    let mut format: *const c_char = null();

    // SAFETY: HAVE_ARCH_BUG_FORMAT with relative pointers.
    unsafe {
        /*
         * Allow an architecture to:
         *  - relative encode NULL (difficult vs KASLR);
         *  - use a literal 0 (there are no valid objects inside
         *    the __bug_table itself to refer to after all);
         *  - use an empty string.
         */
        if (*bug).format_disp != 0 {
            let disp = addr_of!((*bug).format_disp);
            format = disp.cast::<c_char>().wrapping_offset(*disp as isize);
        }
        if !format.is_null() && *format == 0 {
            format = null();
        }
    }
    format
}

/// Finds the bug table entry for a trapping address.
///
/// # Safety
/// The kernel and module bug tables are live.
#[no_mangle]
pub unsafe extern "C" fn find_bug(bugaddr: c_ulong) -> *mut bug_entry {
    // SAFETY: the linker-defined bounds delimit the built-in table.
    unsafe {
        let mut bug = addr_of_mut!(__start___bug_table).cast::<bug_entry>();
        let stop = addr_of_mut!(__stop___bug_table).cast::<bug_entry>();
        while bug < stop {
            if bugaddr == bug_addr(bug) {
                return bug;
            }
            bug = bug.add(1);
        }

        module_find_bug(bugaddr)
    }
}

/// # Safety
/// `fmt` is NULL or a bug-table format; `regs` NULL or the trap registers.
unsafe fn __warn_printf(fmt: *const c_char, regs: *mut pt_regs) {
    if fmt.is_null() {
        return;
    }

    // SAFETY: HAVE_ARCH_BUG_FORMAT_ARGS: the native leaf rebuilds the
    // arguments from the trap registers when it can.
    unsafe {
        if !regs.is_null() && lupos_bug_warn_printf_args(fmt, regs) {
            return;
        }

        _printk(c"\x014%s".as_ptr().cast(), fmt);
    }
}

/// # Safety
/// `bug` is NULL or a valid entry; `regs` NULL or the trap registers.
unsafe fn __report_bug(mut bug: *mut bug_entry, bugaddr: c_ulong, regs: *mut pt_regs) -> c_uint {
    let mut file: *const c_char = null();
    let mut line: c_uint = 0;

    // SAFETY: the entry belongs to a live bug table; only BUGFLAG_DONE is
    // stored, as in the C unit.
    unsafe {
        if bug.is_null() {
            if is_valid_bugaddr(bugaddr) == 0 {
                return BUG_TRAP_TYPE_NONE;
            }

            bug = find_bug(bugaddr);
            if bug.is_null() {
                return BUG_TRAP_TYPE_NONE;
            }
        }

        bug_get_file_line(bug, &mut file, &mut line);
        let fmt = bug_get_format(bug);

        let flags = (*bug).flags;
        let warning = flags & BUGFLAG_WARNING != 0;
        let once = flags & BUGFLAG_ONCE != 0;
        let done = flags & BUGFLAG_DONE != 0;
        let no_cut = flags & BUGFLAG_NO_CUT_HERE != 0;
        let has_args = flags & BUGFLAG_ARGS != 0;

        /*
         * Before the once logic so suppressed warnings do not consume
         * the single-fire budget of WARN_ON_ONCE().
         */
        if warning && lupos_bug_kunit_is_suppressed_warning(true) {
            return BUG_TRAP_TYPE_WARN;
        }

        lupos_bug_disable_trace_on_warning();

        if warning && once {
            if done {
                return BUG_TRAP_TYPE_WARN;
            }

            /*
             * Since this is the only store, concurrency is not an issue.
             */
            (*bug).flags |= BUGFLAG_DONE;
        }

        /*
         * BUG() and WARN_ON() families don't print a custom debug message
         * before triggering the exception handler, so we must add the
         * "cut here" line now. WARN() issues its own "cut here" before the
         * extra debugging message it writes before triggering the handler.
         */
        if !no_cut {
            _printk(c"\x016------------[ cut here ]------------\n".as_ptr().cast());
            __warn_printf(fmt, if has_args { regs } else { null_mut() });
        }

        if warning {
            /* this is a WARN_ON rather than BUG/BUG_ON */
            __warn(file, line as c_int, bugaddr as *mut c_void, c_uint::from(flags >> 8), regs, null_mut());
            return BUG_TRAP_TYPE_WARN;
        }

        if !file.is_null() {
            _printk(c"\x012kernel BUG at %s:%u!\n".as_ptr().cast(), file, line);
        } else {
            _printk(
                c"\x012kernel BUG at %pB [verbose debug info unavailable]\n".as_ptr().cast(),
                bugaddr as *mut c_void,
            );
        }
    }

    BUG_TRAP_TYPE_BUG
}

/// Reports a trap on a known bug table entry.
///
/// # Safety
/// `bug` is a valid entry; `regs` NULL or the trap registers.
#[no_mangle]
pub unsafe extern "C" fn report_bug_entry(bug: *mut bug_entry, regs: *mut pt_regs) -> c_uint {
    // SAFETY: forwarded caller contract.
    unsafe {
        let rcu = lupos_bug_warn_rcu_enter();
        let ret = __report_bug(bug, bug_addr(bug), regs);
        lupos_bug_warn_rcu_exit(rcu);

        ret
    }
}

/// Reports a trap at `bugaddr`.
///
/// # Safety
/// `regs` is NULL or the trap registers.
#[no_mangle]
pub unsafe extern "C" fn report_bug(bugaddr: c_ulong, regs: *mut pt_regs) -> c_uint {
    // SAFETY: forwarded caller contract.
    unsafe {
        let rcu = lupos_bug_warn_rcu_enter();
        let ret = __report_bug(null_mut(), bugaddr, regs);
        lupos_bug_warn_rcu_exit(rcu);

        ret
    }
}

/// # Safety
/// `start..end` is a bug table.
unsafe fn clear_once_table(start: *mut bug_entry, end: *mut bug_entry) {
    let mut bug = start;

    // SAFETY: only entries inside the table are touched.
    unsafe {
        while bug < end {
            (*bug).flags &= !BUGFLAG_DONE;
            bug = bug.add(1);
        }
    }
}

/// Re-arms every WARN_ON_ONCE() of the kernel and its modules.
///
/// # Safety
/// The kernel and module bug tables are live.
#[no_mangle]
pub unsafe extern "C" fn generic_bug_clear_once() {
    // SAFETY: module tables are walked under rcu_read_lock().
    unsafe {
        #[cfg(CONFIG_MODULES)]
        {
            rcu_read_lock();
            let head = addr_of_mut!(MODULE_BUG_LIST);
            let mut pos = read_volatile(addr_of!((*head).next));
            while pos != head {
                let m = pos.byte_sub(offset_of!(module, bug_list)).cast::<module>();
                clear_once_table((*m).bug_table, (*m).bug_table.add((*m).num_bugs as usize));
                pos = read_volatile(addr_of!((*pos).next));
            }
            rcu_read_unlock();
        }

        clear_once_table(
            addr_of_mut!(__start___bug_table).cast::<bug_entry>(),
            addr_of_mut!(__stop___bug_table).cast::<bug_entry>(),
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
