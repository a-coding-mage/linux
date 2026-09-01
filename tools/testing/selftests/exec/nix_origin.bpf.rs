// SPDX-License-Identifier: GPL-2.0
/*
 * nix_origin.bpf.c - $ORIGIN-relative PT_INTERP resolution
 *
 * A binfmt_misc_ops handler that makes relocatable (Nix-style) ELF
 * binaries work: if PT_INTERP starts with "$ORIGIN/", the loader is
 * resolved relative to the directory of the binary being executed and
 * selected via bpf_binprm_set_interp(). The match program reads the
 * program headers itself, so anything else never commits to this
 * handler and passes through untouched.
 *
 * Activate with:
 *   bpftool struct_ops register nix_origin.bpf.o /sys/fs/bpf
 *   echo ':nix-origin:B::::nix_origin:' > /proc/sys/fs/binfmt_misc/register
 */

// Dependencies supplied by the BPF/vmlinux environment in the original C:
// vmlinux.h, bpf/bpf_helpers.h, and bpf/bpf_tracing.h.

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

const PATH_MAX: usize = 4096;
const EI_CLASS: usize = 4;
const ELFCLASSXX: u8 = 2; /* ELFCLASS64; flip to 1 for 32-bit */
const PT_INTERP: u32 = 3;
const MAX_PHDRS: i32 = 64;

const ORIGIN: &[u8; 7] = b"$ORIGIN";
const ORIGIN_LEN: usize = ORIGIN.len();

const ENOENT: i32 = 2;
const ENOEXEC: i32 = 8;
const ENAMETOOLONG: i32 = 36;

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct file {
    pub f_path: path,
}

#[repr(C)]
pub struct path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct linux_binprm {
    pub buf: [u8; 256],
    pub file: *mut file,
}

#[repr(C)]
pub struct elf64_hdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub struct elf64_phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

extern "C" {
    fn bpf_dynptr_from_file(
        file: *mut file,
        flags: __u32,
        ptr__uninit: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_dynptr_file_discard(dynptr: *mut bpf_dynptr) -> i32;
    fn bpf_path_d_path(path: *const path, buf: *mut u8, buf__sz: usize) -> i32;
    fn bpf_binprm_set_interp(bprm: *mut linux_binprm, path: *const u8, path__sz: usize) -> i32;

    fn bpf_dynptr_read(
        dst: *mut core::ffi::c_void,
        len: usize,
        src: *mut bpf_dynptr,
        offset: u64,
        flags: u64,
    ) -> i32;
    fn bpf_strncmp(s1: *const u8, sz: usize, s2: *const u8) -> i32;
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i32;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i32;
}

const BPF_ANY: u64 = 0;

#[repr(C)]
pub struct scratch {
    pub interp: [u8; PATH_MAX], /* PT_INTERP as embedded in the binary */
    pub path: [u8; PATH_MAX],   /* d_path of the binary, becomes the result */
}

/* Keyed by pid: execs run concurrently and the programs can sleep. */
#[repr(C)]
pub struct scratch_map_def {
    _private: [u8; 0],
}

// Original C uses BPF map definition macros:
// struct { __uint(type, BPF_MAP_TYPE_HASH); __uint(max_entries, 512);
//          __type(key, __u64); __type(value, struct scratch); } scratch_map SEC(".maps");
#[no_mangle]
#[link_section = ".maps"]
pub static mut scratch_map: scratch_map_def = scratch_map_def { _private: [] };

static zero_scratch: scratch = scratch {
    interp: [0; PATH_MAX],
    path: [0; PATH_MAX],
};

/* An ELF64 binary per the prefetched header? */
unsafe fn is_elf64(bprm: *mut linux_binprm) -> bool {
    (*bprm).buf[0] == 0x7f
        && (*bprm).buf[1] == b'E'
        && (*bprm).buf[2] == b'L'
        && (*bprm).buf[3] == b'F'
        && (*bprm).buf[EI_CLASS] == ELFCLASSXX
}

/* Locate PT_INTERP; false if the file has none or looks malformed. */
unsafe fn find_pt_interp(dp: *mut bpf_dynptr, phdr: *mut elf64_phdr) -> bool {
    let mut ehdr: elf64_hdr = core::mem::zeroed();
    let mut found = false;
    let mut i: i32;

    if bpf_dynptr_read(
        &mut ehdr as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of::<elf64_hdr>(),
        dp,
        0,
        0,
    ) != 0
    {
        return false;
    }
    if ehdr.e_phentsize as usize != core::mem::size_of::<elf64_phdr>() {
        return false;
    }

    i = 0;
    while i < ehdr.e_phnum as i32 {
        if i >= MAX_PHDRS {
            break;
        }
        if bpf_dynptr_read(
            phdr as *mut core::ffi::c_void,
            core::mem::size_of::<elf64_phdr>(),
            dp,
            ehdr.e_phoff
                .wrapping_add((i as u64).wrapping_mul(core::mem::size_of::<elf64_phdr>() as u64)),
            0,
        ) != 0
        {
            return false;
        }
        if (*phdr).p_type == PT_INTERP {
            found = true;
            break;
        }
        i += 1;
    }
    found
}

/*
 * An ELF64 binary whose PT_INTERP starts with "$ORIGIN/" is ours. The
 * match can sleep and read the file, so the decision is made here and
 * regular binaries never commit to this handler: later binfmt_misc
 * entries and binfmt_elf see them as if we did not exist.
 */
#[no_mangle]
#[link_section = "struct_ops.s/match"]
pub unsafe extern "C" fn nix_origin_match(bprm: *mut linux_binprm) -> bool {
    let mut prefix: [u8; ORIGIN_LEN + 1] = [0; ORIGIN_LEN + 1];
    let mut phdr: elf64_phdr = core::mem::zeroed();
    let mut dp: bpf_dynptr = core::mem::zeroed();
    let mut ours = false;

    if !is_elf64(bprm) {
        return false;
    }

    /* The dynptr must be discarded on every path once requested. */
    if bpf_dynptr_from_file((*bprm).file, 0, &mut dp) != 0 {
        bpf_dynptr_file_discard(&mut dp);
        return ours;
    }
    if find_pt_interp(&mut dp, &mut phdr)
        && phdr.p_filesz > (ORIGIN_LEN + 1) as u64
        && bpf_dynptr_read(
            prefix.as_mut_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&prefix),
            &mut dp,
            phdr.p_offset,
            0,
        ) == 0
    {
        ours = bpf_strncmp(
            prefix.as_ptr(),
            core::mem::size_of_val(&prefix),
            b"$ORIGIN/\0".as_ptr(),
        ) == 0;
    }
    bpf_dynptr_file_discard(&mut dp);
    ours
}

/*
 * The match is committed and already vetted the "$ORIGIN/" prefix, so
 * everything here reads the file again from scratch: -ENOEXEC only
 * covers a binary that changed under us and stopped being ours.
 */
#[no_mangle]
#[link_section = "struct_ops.s/load"]
pub unsafe extern "C" fn nix_origin_load(bprm: *mut linux_binprm) -> i32 {
    let mut isz: __u32;
    let mut sfx: __u32;
    let mut rsz: __u32;
    let mut slash: __u32;
    let mut phdr: elf64_phdr = core::mem::zeroed();
    let mut dp: bpf_dynptr = core::mem::zeroed();
    let mut sc: *mut scratch;
    let mut id: __u64 = 0;
    let mut ret: i32 = -ENOEXEC;
    let mut len: i32;
    let mut i: i32;

    if bpf_dynptr_from_file((*bprm).file, 0, &mut dp) != 0 {
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }

    if !find_pt_interp(&mut dp, &mut phdr) {
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }

    isz = phdr.p_filesz as __u32;
    if isz <= (ORIGIN_LEN + 1) as __u32 || isz as usize >= PATH_MAX {
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }
    /*
     * The range check above compiles to a test on a zero-extended copy of
     * the u64 p_filesz, so the verifier does not carry the bound to the
     * dynptr_read() length below ("unbounded memory access"). Mask isz to
     * the buffer size (a power of two) and force the masked value to be
     * materialized with a barrier so the read uses the bounded register.
     */
    isz &= (PATH_MAX - 1) as __u32;
    core::arch::asm!("", inout(reg) isz, options(nomem, nostack, preserves_flags));

    id = bpf_get_current_pid_tgid();
    if bpf_map_update_elem(
        &mut scratch_map as *mut _ as *mut core::ffi::c_void,
        &id as *const _ as *const core::ffi::c_void,
        &zero_scratch as *const _ as *const core::ffi::c_void,
        BPF_ANY,
    ) != 0
    {
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }
    sc = bpf_map_lookup_elem(
        &mut scratch_map as *mut _ as *mut core::ffi::c_void,
        &id as *const _ as *const core::ffi::c_void,
    ) as *mut scratch;
    if sc.is_null() {
        bpf_map_delete_elem(
            &mut scratch_map as *mut _ as *mut core::ffi::c_void,
            &id as *const _ as *const core::ffi::c_void,
        );
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }

    if bpf_dynptr_read(
        (*sc).interp.as_mut_ptr() as *mut core::ffi::c_void,
        isz as usize,
        &mut dp,
        phdr.p_offset,
        0,
    ) != 0
    {
        bpf_map_delete_elem(
            &mut scratch_map as *mut _ as *mut core::ffi::c_void,
            &id as *const _ as *const core::ffi::c_void,
        );
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }
    if (*sc).interp[isz as usize - 1] != b'\0' {
        bpf_map_delete_elem(
            &mut scratch_map as *mut _ as *mut core::ffi::c_void,
            &id as *const _ as *const core::ffi::c_void,
        );
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }

    /* Not "$ORIGIN/..." anymore? Then it is not ours anymore either. */
    if (*sc).interp[0] != b'$'
        || (*sc).interp[1] != b'O'
        || (*sc).interp[2] != b'R'
        || (*sc).interp[3] != b'I'
        || (*sc).interp[4] != b'G'
        || (*sc).interp[5] != b'I'
        || (*sc).interp[6] != b'N'
        || (*sc).interp[7] != b'/'
    {
        bpf_map_delete_elem(
            &mut scratch_map as *mut _ as *mut core::ffi::c_void,
            &id as *const _ as *const core::ffi::c_void,
        );
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }

    /*
     * From here on resolution failures fail the exec instead of falling
     * back to binfmt_elf, which would resolve the literal "$ORIGIN/..."
     * relative to the caller's cwd.
     */
    ret = -ENOENT;
    len = bpf_path_d_path(
        &(*(*bprm).file).f_path,
        (*sc).path.as_mut_ptr(),
        core::mem::size_of_val(&(*sc).path),
    );
    if len <= 0 || len as usize > PATH_MAX {
        bpf_map_delete_elem(
            &mut scratch_map as *mut _ as *mut core::ffi::c_void,
            &id as *const _ as *const core::ffi::c_void,
        );
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }
    /* Unreachable or unlinked ("... (deleted)") binaries can't resolve. */
    if (*sc).path[0] != b'/' {
        bpf_map_delete_elem(
            &mut scratch_map as *mut _ as *mut core::ffi::c_void,
            &id as *const _ as *const core::ffi::c_void,
        );
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }

    /* $ORIGIN = dirname of the binary. */
    slash = 0;
    i = 1;
    while i < len - 1 {
        if i as usize >= PATH_MAX {
            break;
        }
        if (*sc).path[i as usize] == b'/' {
            slash = i as __u32;
        }
        i += 1;
    }

    /* Splice the suffix (leading '/' and NUL included) onto the dir. */
    sfx = isz - ORIGIN_LEN as __u32;
    rsz = slash + sfx;
    if rsz as usize > PATH_MAX {
        ret = -ENAMETOOLONG;
        bpf_map_delete_elem(
            &mut scratch_map as *mut _ as *mut core::ffi::c_void,
            &id as *const _ as *const core::ffi::c_void,
        );
        bpf_dynptr_file_discard(&mut dp);
        return ret;
    }
    i = 0;
    while i < sfx as i32 {
        let s: __u32 = ORIGIN_LEN as __u32 + i as __u32;
        let d: __u32 = slash + i as __u32;

        if s as usize >= PATH_MAX || d as usize >= PATH_MAX {
            break;
        }
        (*sc).path[d as usize] = (*sc).interp[s as usize];
        i += 1;
    }

    ret = bpf_binprm_set_interp(bprm, (*sc).path.as_ptr(), rsz as usize);
    bpf_map_delete_elem(
        &mut scratch_map as *mut _ as *mut core::ffi::c_void,
        &id as *const _ as *const core::ffi::c_void,
    );
    bpf_dynptr_file_discard(&mut dp);
    ret
}

#[repr(C)]
pub struct binfmt_misc_ops {
    pub match_: Option<unsafe extern "C" fn(*mut linux_binprm) -> bool>,
    pub load: Option<unsafe extern "C" fn(*mut linux_binprm) -> i32>,
    pub name: *const u8,
}

unsafe impl Sync for binfmt_misc_ops {}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static nix_origin: binfmt_misc_ops = binfmt_misc_ops {
    match_: Some(nix_origin_match),
    load: Some(nix_origin_load),
    name: b"nix_origin\0".as_ptr(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
