/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/binfmt_misc.h. The C include and header guard are
// intentionally omitted; referenced kernel types are supplied by dependencies.

pub const BINFMT_MISC_OPS_NAME_MAX: usize = 16;

/* Longest name a 'B' entry can bind an interpreter under. */
pub const BINFMT_MISC_INTERP_NAME_MAX: usize = 32;

/* Most interpreters one entry can bind. */
pub const BINFMT_MISC_INTERP_MAX: usize = 100;

/**
 * struct binfmt_misc_interp - an interpreter an entry was registered with
 * @list: link in the entry's list, in registration order
 * @file: the file, opened at registration and never resolved again
 * @ucounts: the UCOUNT_BINFMT_MISC_INTERPRETERS charge the binding took
 * @path: the path it was registered under, used as the name the interpreter
 *        runs under; stored after @name in the same allocation
 * @name: the name the load program selects it by; empty for the fixed
 *        interpreter of a static 'F' entry
 *
 * Owned by the entry and living exactly as long as it does. The list head
 * is handed to the handler's load program for the duration of one exec,
 * which picks one with bpf_binprm_select_interp().
 */
#[repr(C)]
pub struct binfmt_misc_interp {
    pub list: list_head,
    pub file: *mut file,
    pub ucounts: *mut ucounts,
    pub path: *const core::ffi::c_char,
    pub name: [core::ffi::c_char; 0],
}

pub unsafe extern "C" fn binfmt_misc_find_interp(
    interps: *const list_head,
    name: *const core::ffi::c_char,
) -> *const binfmt_misc_interp;

/**
 * enum bpf_binprm_flags - per-exec invocation flags a load program can request
 * @BPF_BINPRM_PRESERVE_ARGV0: keep the caller's argv[0] (like the 'P' flag)
 * @BPF_BINPRM_CREDENTIALS: compute credentials from the binary; implies execfd
 *                          (like the 'C' flag)
 * @BPF_BINPRM_EXECFD: pass the binary via AT_EXECFD (like the 'O' flag)
 * @BPF_BINPRM_TRANSPARENT: leave argv untouched, the interpreter takes the
 *                          binary from AT_EXECFD (like the 'T' flag); implies
 *                          execfd, excludes preserve-argv0
 * @BPF_BINPRM_LOADER: substitute the interpreter for the binary's PT_INTERP
 *                     and run the binary as a native exec (like the 'L'
 *                     flag); excludes every other flag
 *
 * Set from a load program with bpf_binprm_set_flags(). Unlike a static entry,
 * a bpf handler chooses these per exec rather than once at registration.
 */
#[repr(u64)]
pub enum bpf_binprm_flags {
    BPF_BINPRM_PRESERVE_ARGV0 = 1u64 << 0,
    BPF_BINPRM_CREDENTIALS = 1u64 << 1,
    BPF_BINPRM_EXECFD = 1u64 << 2,
    BPF_BINPRM_TRANSPARENT = 1u64 << 3,
    BPF_BINPRM_LOADER = 1u64 << 4,
}

/**
 * struct binfmt_misc_ops - bpf-backed binary type handler
 * @match: decide whether the handler applies to @bprm; consulted from the
 *         entry lookup walk like static magic and extension matching, in
 *         registration order with first-match-wins semantics; sleepable,
 *         so it can read the binary to decide, but the verifier rejects
 *         the interpreter selection kfuncs in it
 * @load:  select an interpreter for the matched @bprm via
 *         bpf_binprm_set_interp(), or one the entry bound via
 *         bpf_binprm_select_interp(), and return zero; a match is
 *         committed, so a failure fails the exec instead of falling
 *         through to later entries; -ENOEXEC does not fail the exec but
 *         moves on to the remaining binary formats
 * @name: name that 'B' entries reference the handler by
 */
#[repr(C)]
pub struct binfmt_misc_ops {
    pub r#match: Option<unsafe extern "C" fn(*mut linux_binprm) -> bool>,
    pub load: Option<unsafe extern "C" fn(*mut linux_binprm) -> core::ffi::c_int>,
    pub name: [core::ffi::c_char; BINFMT_MISC_OPS_NAME_MAX],
}

// Under CONFIG_BINFMT_MISC_BPF these are external declarations.
#[cfg(CONFIG_BINFMT_MISC_BPF)]
extern "C" {
    pub fn binfmt_misc_get_ops(
        user_ns: *mut user_namespace,
        name: *const core::ffi::c_char,
    ) -> *const binfmt_misc_ops;
    pub fn binfmt_misc_put_ops(ops: *const binfmt_misc_ops);
    pub fn bpf_prog_is_binfmt_misc_ops(prog: *const bpf_prog) -> bool;
}

// Without CONFIG_BINFMT_MISC_BPF, the C header provides these inline stubs.
#[cfg(not(CONFIG_BINFMT_MISC_BPF))]
pub unsafe fn binfmt_misc_get_ops(
    _user_ns: *mut user_namespace,
    _name: *const core::ffi::c_char,
) -> *const binfmt_misc_ops {
    core::ptr::null()
}

#[cfg(not(CONFIG_BINFMT_MISC_BPF))]
pub unsafe fn binfmt_misc_put_ops(_ops: *const binfmt_misc_ops) {}

#[cfg(not(CONFIG_BINFMT_MISC_BPF))]
pub unsafe fn bpf_prog_is_binfmt_misc_ops(_prog: *const bpf_prog) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
