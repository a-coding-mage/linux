/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the corresponding Linux headers are intentionally
// left as external dependencies.

pub const CORENAME_MAX_SIZE: usize = 128;

/* Interpreter selection staged by a bpf binfmt_misc handler. */
#[repr(C)]
pub struct binfmt_misc_bpf {
    /* interpreters the matched entry bound, selectable by name */
    pub bpf_interps: *const list_head,
    pub bpf_interp: *const c_char, /* interpreter selected by a bpf handler */
    pub bpf_interp_file: *mut file, /* the bound interpreter it selected */
    pub bpf_interp_arg: *const c_char, /* interpreter argument from a bpf handler */
    pub bpf_flags: u64, /* enum bpf_binprm_flags from a bpf handler */
}

/*
 * This structure is used to hold the arguments that are used when loading binaries.
 */
#[repr(C)]
pub struct linux_binprm {
    // CONFIG_MMU selects the vma/vma_pages/argmin layout; otherwise the page
    // array layout below is used.
    #[cfg(CONFIG_MMU)]
    pub vma: *mut vm_area_struct,
    #[cfg(CONFIG_MMU)]
    pub vma_pages: c_ulong,
    #[cfg(CONFIG_MMU)]
    pub argmin: c_ulong, /* rlimit marker for copy_strings() */
    #[cfg(not(CONFIG_MMU))]
    pub page: [*mut page; 32],
    pub mm: *mut mm_struct,
    pub old_mm: *mut mm_struct, /* replaced address space, freed by setup_new_exec() */
    /* user_ns published to task->exec_state at execve, narrowed by would_dump(). */
    pub user_ns: *mut user_namespace,
    pub p: c_ulong, /* current top of mem */
    /* C bitfields: have_execfd, execfd_creds, secureexec, point_of_no_return,
       comm_from_dentry, and is_check occupy one unsigned int. */
    pub flags: c_uint,
    pub executable: *mut file, /* Executable to pass to the interpreter */
    pub interpreter: *mut file,
    pub loader: *mut file,
    pub file: *mut file,
    pub cred: *mut cred, /* new credentials */
    pub unsafe_: c_int, /* how unsafe this exec is (mask of LSM_UNSAFE_*) */
    pub per_clear: c_uint, /* bits to clear in current->personality */
    pub argc: c_int,
    pub envc: c_int,
    pub filename: *const c_char, /* Name of binary as seen by procps */
    pub interp: *const c_char, /* Name of the binary really executed. Most of
                                  the time same as filename, but could be
                                  different for binfmt_{misc,script} */
    pub fdpath: *const c_char, /* generated filename for execveat */
    pub bpf: binfmt_misc_bpf, /* bpf handler interpreter selection */
    pub interp_flags: c_uint,
    pub execfd: c_int, /* File descriptor of the executable */
    pub exec: c_ulong,
    pub rlim_stack: rlimit, /* Saved RLIMIT_STACK used during exec. */
    pub buf: [c_char; BINPRM_BUF_SIZE as usize],
}

pub const BINPRM_FLAGS_ENFORCE_NONDUMP_BIT: u32 = 0;
pub const BINPRM_FLAGS_ENFORCE_NONDUMP: u32 = 1 << BINPRM_FLAGS_ENFORCE_NONDUMP_BIT;
/* filename of the binary will be inaccessible after exec */
pub const BINPRM_FLAGS_PATH_INACCESSIBLE_BIT: u32 = 2;
pub const BINPRM_FLAGS_PATH_INACCESSIBLE: u32 = 1 << BINPRM_FLAGS_PATH_INACCESSIBLE_BIT;
/* preserve argv0 for the interpreter */
pub const BINPRM_FLAGS_PRESERVE_ARGV0_BIT: u32 = 3;
pub const BINPRM_FLAGS_PRESERVE_ARGV0: u32 = 1 << BINPRM_FLAGS_PRESERVE_ARGV0_BIT;
/* binfmt_misc dispatched to the interpreter transparently */
pub const BINPRM_FLAGS_TRANSPARENT_INTERP_BIT: u32 = 4;
pub const BINPRM_FLAGS_TRANSPARENT_INTERP: u32 = 1 << BINPRM_FLAGS_TRANSPARENT_INTERP_BIT;

/* bprm_at_flags - the AT_FLAGS this invocation implies */
pub unsafe fn bprm_at_flags(bprm: *const linux_binprm) -> c_ulong {
    /* Transparency preserves the whole argv, argv[0] included. */
    if (*bprm).interp_flags & BINPRM_FLAGS_TRANSPARENT_INTERP != 0 { return AT_FLAGS_TRANSPARENT_INTERP as c_ulong; }
    if (*bprm).interp_flags & BINPRM_FLAGS_PRESERVE_ARGV0 != 0 { return AT_FLAGS_PRESERVE_ARGV0 as c_ulong; }
    0
}

#[repr(C)]
pub struct linux_binfmt {
    pub lh: list_head,
    pub module: *mut module,
    pub load_binary: Option<unsafe extern "C" fn(*mut linux_binprm) -> c_int>,
    #[cfg(CONFIG_COREDUMP)]
    pub core_dump: Option<unsafe extern "C" fn(*mut coredump_params) -> c_int>,
    #[cfg(CONFIG_COREDUMP)]
    pub min_coredump: c_ulong,
}

#[cfg(CONFIG_BINFMT_MISC)]
#[repr(C)]
pub struct binfmt_misc { pub entries: hlist_head, pub entries_lock: spinlock_t, pub enabled: bool }
#[cfg(CONFIG_BINFMT_MISC)]
extern "C" { pub static mut init_binfmt_misc: binfmt_misc; }

extern "C" {
    pub fn __register_binfmt(fmt: *mut linux_binfmt, insert: c_int);
    pub fn unregister_binfmt(fmt: *mut linux_binfmt);
    pub fn remove_arg_zero(bprm: *mut linux_binprm) -> c_int;
    pub fn begin_new_exec(bprm: *mut linux_binprm) -> c_int;
    pub fn setup_new_exec(bprm: *mut linux_binprm);
    pub fn finalize_exec(bprm: *mut linux_binprm);
    pub fn would_dump(bprm: *mut linux_binprm, file: *mut file);
    pub fn bprm_open_interpreter(bprm: *mut linux_binprm, path: *const c_char) -> *mut file;
    pub fn bprm_drop_loader(bprm: *mut linux_binprm);
    pub static mut suid_dumpable: c_int;
}

pub const EXSTACK_DEFAULT: c_int = 0;
pub const EXSTACK_DISABLE_X: c_int = 1;
pub const EXSTACK_ENABLE_X: c_int = 2;

extern "C" {
    pub fn setup_arg_pages(bprm: *mut linux_binprm, stack_top: c_ulong, executable_stack: c_int) -> c_int;
    pub fn transfer_args_to_stack(bprm: *mut linux_binprm, sp_location: *mut c_ulong) -> c_int;
    pub fn bprm_change_interp(interp: *const c_char, bprm: *mut linux_binprm) -> c_int;
    pub fn copy_string_kernel(arg: *const c_char, bprm: *mut linux_binprm) -> c_int;
    pub fn set_binfmt(new: *mut linux_binfmt);
    pub fn read_code(file: *mut file, addr: c_ulong, pos: loff_t, size: size_t) -> ssize_t;
    pub fn kernel_execve(filename: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
}

/* Registration of default binfmt handlers */
pub unsafe fn register_binfmt(fmt: *mut linux_binfmt) {
    __register_binfmt(fmt, 0);
}
/* Same as above, but adds a new binfmt at the top of the list */
pub unsafe fn insert_binfmt(fmt: *mut linux_binfmt) {
    __register_binfmt(fmt, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
