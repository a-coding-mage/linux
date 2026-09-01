// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Compute "sympos", the position used by livepatch to disambiguate
 * duplicate symbol names in the patched object.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type size_t = usize;

#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Scn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Data {
    pub d_buf: *mut c_void,
}

#[repr(C)]
pub struct GElf_Ehdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GElf_Shdr {
    pub sh_name: size_t,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_entsize: u64,
}

#[repr(C)]
pub struct GElf_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

#[repr(C)]
pub struct klp_symid {
    pub id: u64,
    pub addr: u64,
}

#[repr(C)]
pub struct section {
    pub name: *const c_char,
    pub data: *mut Elf_Data,
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub sec: *mut section,
    pub bind: c_uint,
    pub idx: c_uint,
}

#[repr(C)]
pub struct reloc {
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct elf {
    pub name: *const c_char,
}

#[repr(C)]
pub struct vmlinux_sym {
    pub hash: hlist_node,
    pub name: *const c_char,
    pub addr: u64,
}

#[repr(C)]
pub struct vmlinux_symid {
    pub hash: hlist_node,
    pub id: u64,
    pub addr: u64,
}

#[repr(C)]
pub struct vmlinux_o_symid {
    pub hash: hlist_node,
    pub id: u64,
    pub sym_idx: c_uint,
}

#[repr(C)]
pub struct hlist_head {
    _private: [u8; 0],
}

#[repr(C)]
struct Vmlinux {
    elf: *mut Elf,
    syms: [hlist_head; 1 << 16],
    symids: [hlist_head; 1 << 16],
}

const O_RDONLY: c_int = 0;
const EV_CURRENT: c_uint = 1;
const EV_NONE: c_uint = 0;
const ELF_C_READ_MMAP: c_uint = 5;
const SHN_UNDEF: u16 = 0;
const SHN_LORESERVE: u16 = 0xff00;
const STT_SECTION: c_uint = 3;
const STT_FILE: c_uint = 4;
const SHT_SYMTAB: u32 = 2;
const SHF_ALLOC: u64 = 0x2;
const STB_LOCAL: c_uint = 0;
const ULONG_MAX: c_ulong = c_ulong::MAX;

unsafe extern "C" {
    static objname: *const c_char;
    static KLP_SYMID_SEC: *const c_char;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn str_ends_with(s: *const c_char, suffix: *const c_char) -> bool;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;

    fn elf_version(version: c_uint) -> c_uint;
    fn elf_begin(fd: c_int, cmd: c_uint, ref_: *mut Elf) -> *mut Elf;
    fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn elf_getshdrstrndx(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;
    fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn elf_strptr(elf: *mut Elf, section: size_t, offset: size_t) -> *const c_char;
    fn gelf_getsym(data: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Sym) -> *mut GElf_Sym;

    fn __bswap_if_needed(ehdr: *mut GElf_Ehdr, val: u64) -> u64;
    fn bswap_if_needed(elf: *mut elf, val: u64) -> u64;
    fn sec_size(sec: *mut section) -> c_ulong;
    fn find_reloc_by_dest(elf: *mut elf, sec: *mut section, offset: c_ulong) -> *mut reloc;
    fn str_hash(str_: *const c_char) -> u32;

    fn hash_add(table: *mut hlist_head, node: *mut hlist_node, key: u64);
    fn ERROR(fmt: *const c_char, ...);
    fn ERROR_GLIBC(fmt: *const c_char, ...);
    fn ERROR_ELF(fmt: *const c_char, ...);
}

/* static DEFINE_HASHTABLE(vmlinux_o_symids, 16); */
static mut VMLINUX_O_SYMIDS: [hlist_head; 1 << 16] = [hlist_head { _private: [] }; 1 << 16];

/*
 * The original linked kernel, found next to the orig vmlinux.o.  Read with raw
 * libelf rather than elf_open_read(): only the symbol table and the resolved
 * .klp.symid table are needed, not the (huge) instruction/reloc machinery.
 *
 * Both tables are built once by read_orig_vmlinux().  The Elf handle stays
 * open because the hashed names point into its mmapped string table.
 */
static mut VMLINUX: Vmlinux = Vmlinux {
    elf: ptr::null_mut(),
    syms: [hlist_head { _private: [] }; 1 << 16],
    symids: [hlist_head { _private: [] }; 1 << 16],
};

fn GELF_ST_TYPE(info: u8) -> c_uint {
    (info & 0xf) as c_uint
}

/*
 * Would the symbol be visible to the runtime's kallsyms-based symbol lookup?
 */
unsafe fn vmlinux_sym_in_kallsyms(elf: *mut Elf, sym: *mut GElf_Sym) -> bool {
    let type_ = GELF_ST_TYPE((*sym).st_info);
    let mut shdr: GElf_Shdr = core::mem::zeroed();
    let scn: *mut Elf_Scn;

    if (*sym).st_shndx == SHN_UNDEF || (*sym).st_shndx >= SHN_LORESERVE {
        return false;
    }

    if type_ == STT_SECTION || type_ == STT_FILE {
        return false;
    }

    scn = elf_getscn(elf, (*sym).st_shndx);
    if scn.is_null() || gelf_getshdr(scn, &mut shdr).is_null() {
        return false;
    }

    (shdr.sh_flags & SHF_ALLOC) != 0
}

unsafe extern "C" {
    fn elf_getscn(elf: *mut Elf, index: size_t) -> *mut Elf_Scn;
}

unsafe fn read_orig_vmlinux(filename: *const c_char) -> c_int {
    let mut shstrndx: size_t = 0;
    let mut nr_syms: size_t = 0;
    let mut nr_symids: size_t = 0;
    let mut strtab_idx: size_t = 0;
    let mut symtab_data: *mut Elf_Data = ptr::null_mut();
    let mut symid_data: *mut Elf_Data = ptr::null_mut();
    let mut symids: *mut klp_symid;
    let mut scn: *mut Elf_Scn = ptr::null_mut();
    let mut ehdr: GElf_Ehdr = core::mem::zeroed();
    let fd: c_int;

    fd = open(filename, O_RDONLY);
    if fd == -1 {
        ERROR_GLIBC(c"can't open '%s'".as_ptr(), filename);
        return -1;
    }

    if elf_version(EV_CURRENT) == EV_NONE {
        ERROR_ELF(c"elf_version".as_ptr());
        return -1;
    }

    VMLINUX.elf = elf_begin(fd, ELF_C_READ_MMAP, ptr::null_mut());
    if VMLINUX.elf.is_null() {
        ERROR_ELF(c"elf_begin".as_ptr());
        return -1;
    }

    if gelf_getehdr(VMLINUX.elf, &mut ehdr).is_null() {
        ERROR_ELF(c"gelf_getehdr".as_ptr());
        return -1;
    }

    if elf_getshdrstrndx(VMLINUX.elf, &mut shstrndx) != 0 {
        ERROR_ELF(c"elf_getshdrstrndx".as_ptr());
        return -1;
    }

    loop {
        scn = elf_nextscn(VMLINUX.elf, scn);
        if scn.is_null() {
            break;
        }

        let name: *const c_char;
        let mut shdr: GElf_Shdr = core::mem::zeroed();

        if gelf_getshdr(scn, &mut shdr).is_null() {
            ERROR_ELF(c"gelf_getshdr".as_ptr());
            return -1;
        }

        if shdr.sh_type == SHT_SYMTAB {
            symtab_data = elf_getdata(scn, ptr::null_mut());
            if symtab_data.is_null() {
                ERROR_ELF(c"elf_getdata".as_ptr());
                return -1;
            }
            nr_syms = (shdr.sh_size / shdr.sh_entsize) as size_t;
            strtab_idx = shdr.sh_link as size_t;
            continue;
        }

        name = elf_strptr(VMLINUX.elf, shstrndx, shdr.sh_name);
        if !name.is_null() && strcmp(name, KLP_SYMID_SEC) == 0 {
            if shdr.sh_size as usize % size_of::<klp_symid>() != 0 {
                ERROR(
                    c"%s: %s: struct klp_symid size mismatch".as_ptr(),
                    filename,
                    KLP_SYMID_SEC,
                );
                return -1;
            }
            symid_data = elf_getdata(scn, ptr::null_mut());
            if symid_data.is_null() {
                ERROR_ELF(c"elf_getdata".as_ptr());
                return -1;
            }
            nr_symids = shdr.sh_size as size_t / size_of::<klp_symid>();
        }
    }

    if symtab_data.is_null() {
        ERROR(c"%s: missing symbol table".as_ptr(), filename);
        return -1;
    }

    if symid_data.is_null() {
        ERROR(
            c"%s: missing %s section, kernel not built with CONFIG_KLP_BUILD?".as_ptr(),
            filename,
            KLP_SYMID_SEC,
        );
        return -1;
    }

    for i in 0..nr_syms {
        let vsym: *mut vmlinux_sym;
        let name: *const c_char;
        let mut s: GElf_Sym = core::mem::zeroed();

        if gelf_getsym(symtab_data, i as c_int, &mut s).is_null() {
            ERROR_ELF(c"gelf_getsym".as_ptr());
            return -1;
        }

        if !vmlinux_sym_in_kallsyms(VMLINUX.elf, &mut s) {
            continue;
        }

        name = elf_strptr(VMLINUX.elf, strtab_idx, s.st_name as size_t);
        if name.is_null() {
            continue;
        }

        vsym = calloc(1, size_of::<vmlinux_sym>()) as *mut vmlinux_sym;
        if vsym.is_null() {
            ERROR_GLIBC(c"calloc".as_ptr());
            return -1;
        }

        (*vsym).name = name;
        (*vsym).addr = s.st_value;
        hash_add(VMLINUX.syms.as_mut_ptr(), &mut (*vsym).hash, str_hash(name) as u64);
    }

    symids = (*symid_data).d_buf as *mut klp_symid;

    for i in 0..nr_symids {
        let vsymid: *mut vmlinux_symid;

        vsymid = calloc(1, size_of::<vmlinux_symid>()) as *mut vmlinux_symid;
        if vsymid.is_null() {
            ERROR_GLIBC(c"calloc".as_ptr());
            return -1;
        }

        (*vsymid).id = __bswap_if_needed(&mut ehdr, (*symids.add(i)).id);
        (*vsymid).addr = __bswap_if_needed(&mut ehdr, (*symids.add(i)).addr);
        hash_add(VMLINUX.symids.as_mut_ptr(), &mut (*vsymid).hash, (*vsymid).id);
    }

    /* the fd and Elf handle stay open, the hashed names live in the mmap */
    0
}

/*
 * Read the orig vmlinux.o's .klp.symid table, an array of entries whose 'addr'
 * fields have relocs to the symbols they describe.
 */
unsafe fn read_vmlinux_o_symids(vmlinux_o: *mut elf) -> c_int {
    let mut sec: *mut section;

    /* for_each_sec(vmlinux_o, sec) */
    sec = ptr::null_mut();
    while for_each_sec_next(vmlinux_o, &mut sec) {
        let nr: c_ulong;

        if strcmp((*sec).name, KLP_SYMID_SEC) != 0 {
            continue;
        }

        if sec_size(sec) as usize % size_of::<klp_symid>() != 0 {
            ERROR(
                c"%s: %s: struct klp_symid size mismatch".as_ptr(),
                (*vmlinux_o).name,
                KLP_SYMID_SEC,
            );
            return -1;
        }

        nr = sec_size(sec) / size_of::<klp_symid>() as c_ulong;

        for i in 0..nr {
            let offset = i * size_of::<klp_symid>() as c_ulong;
            let entry: *mut vmlinux_o_symid;
            let symid: *mut klp_symid;
            let reloc: *mut reloc;

            entry = calloc(1, size_of::<vmlinux_o_symid>()) as *mut vmlinux_o_symid;
            if entry.is_null() {
                ERROR_GLIBC(c"calloc".as_ptr());
                return -1;
            }

            symid = ((*(*sec).data).d_buf as *mut u8).add(offset as usize) as *mut klp_symid;
            (*entry).id = bswap_if_needed(vmlinux_o, (*symid).id);

            reloc = find_reloc_by_dest(
                vmlinux_o,
                sec,
                offset + core::mem::offset_of!(klp_symid, addr) as c_ulong,
            );
            if reloc.is_null() {
                ERROR(
                    c"%s: missing reloc for %s entry".as_ptr(),
                    (*vmlinux_o).name,
                    KLP_SYMID_SEC,
                );
                return -1;
            }
            (*entry).sym_idx = (*(*reloc).sym).idx;

            hash_add(VMLINUX_O_SYMIDS.as_mut_ptr(), &mut (*entry).hash, (*entry).sym_idx as u64);
        }
    }

    0
}

unsafe extern "C" {
    fn for_each_sec_next(elf: *mut elf, sec: *mut *mut section) -> bool;
    fn hash_for_each_possible_vmlinux_o_symids(
        table: *mut hlist_head,
        key: u64,
        cursor: *mut *mut vmlinux_o_symid,
    ) -> bool;
    fn hash_for_each_possible_vmlinux_symids(
        table: *mut hlist_head,
        key: u64,
        cursor: *mut *mut vmlinux_symid,
    ) -> bool;
    fn hash_for_each_possible_vmlinux_syms(
        table: *mut hlist_head,
        key: u64,
        cursor: *mut *mut vmlinux_sym,
    ) -> bool;
    fn for_each_sym_next(elf: *mut elf, sym: *mut *mut symbol) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn klp_sympos_init(orig: *mut elf) -> c_int {
    let filename: *mut c_char;
    let ret: c_int;

    if !str_ends_with(objname, c"vmlinux.o".as_ptr()) {
        return 0;
    }

    if read_vmlinux_o_symids(orig) != 0 {
        return -1;
    }

    filename = strndup(objname, strlen(objname) - 2);
    if filename.is_null() {
        ERROR_GLIBC(c"strndup".as_ptr());
        return -1;
    }

    ret = read_orig_vmlinux(filename);
    free(filename as *mut c_void);

    ret
}

/* Find the symbol's id in the orig vmlinux.o's .klp.symid table */
unsafe fn find_vmlinux_o_symid(sym: *mut symbol, id: *mut u64) -> c_int {
    let mut entry: *mut vmlinux_o_symid = ptr::null_mut();

    /* hash_for_each_possible(vmlinux_o_symids, entry, hash, sym->idx) */
    while hash_for_each_possible_vmlinux_o_symids(
        VMLINUX_O_SYMIDS.as_mut_ptr(),
        (*sym).idx as u64,
        &mut entry,
    ) {
        if (*entry).sym_idx == (*sym).idx {
            *id = (*entry).id;
            return 0;
        }
    }

    ERROR(
        c"no %s entry for symbol %s in orig vmlinux.o".as_ptr(),
        KLP_SYMID_SEC,
        (*sym).name,
    );
    -1
}

/* Find the symbol's final address in the orig vmlinux's .klp.symid table */
unsafe fn find_vmlinux_symid_addr(id: u64, addr: *mut u64) -> c_int {
    let mut symid: *mut vmlinux_symid = ptr::null_mut();

    /* hash_for_each_possible(vmlinux.symids, symid, hash, id) */
    while hash_for_each_possible_vmlinux_symids(VMLINUX.symids.as_mut_ptr(), id, &mut symid) {
        if (*symid).id == id {
            *addr = (*symid).addr;
            return 0;
        }
    }

    -1
}

/*
 * Find the sympos of a vmlinux-local symbol by ranking its final address
 * among the duplicately named symbols in the linked orig vmlinux, replicating
 * the order in which kallsyms_on_each_match_symbol() counts them.
 */
unsafe fn find_vmlinux_sympos(sym: *mut symbol) -> c_ulong {
    let mut nr_matches: c_ulong = 0;
    let mut sympos: c_ulong = 1;
    let key: u32 = str_hash((*sym).name);
    let mut vsym: *mut vmlinux_sym = ptr::null_mut();
    let mut found: bool = false;
    let mut id: u64 = 0;
    let mut addr: u64 = 0;

    /* hash_for_each_possible(vmlinux.syms, vsym, hash, key) */
    while hash_for_each_possible_vmlinux_syms(VMLINUX.syms.as_mut_ptr(), key as u64, &mut vsym) {
        if strcmp((*vsym).name, (*sym).name) == 0 {
            nr_matches += 1;
        }
    }

    if nr_matches == 0 {
        ERROR(c"can't find symbol %s in orig vmlinux".as_ptr(), (*sym).name);
        return ULONG_MAX;
    }

    /*
     * Unique symbols don't need disambiguating.  They also have no
     * .klp.symid entry, which is only emitted for names duplicated in
     * vmlinux.o, so the lookups below would fail.
     */
    if nr_matches == 1 {
        return 0;
    }

    if find_vmlinux_o_symid(sym, &mut id) != 0 {
        return ULONG_MAX;
    }

    if find_vmlinux_symid_addr(id, &mut addr) != 0 {
        ERROR(
            c"no %s entry for symbol %s in orig vmlinux".as_ptr(),
            KLP_SYMID_SEC,
            (*sym).name,
        );
        return ULONG_MAX;
    }

    /* hash_for_each_possible(vmlinux.syms, vsym, hash, key) */
    vsym = ptr::null_mut();
    while hash_for_each_possible_vmlinux_syms(VMLINUX.syms.as_mut_ptr(), key as u64, &mut vsym) {
        if strcmp((*vsym).name, (*sym).name) != 0 {
            continue;
        }

        if (*vsym).addr < addr {
            sympos += 1;
        } else if (*vsym).addr == addr {
            found = true;
        }
    }

    if !found {
        ERROR(
            c"%s address mismatch for symbol %s, stale orig vmlinux?".as_ptr(),
            KLP_SYMID_SEC,
            (*sym).name,
        );
        return ULONG_MAX;
    }

    sympos
}

unsafe fn is_init_sym(sym: *mut symbol) -> bool {
    strstarts((*(*sym).sec).name, c".init".as_ptr())
}

/*
 * "sympos" is used by livepatch to disambiguate duplicate symbol names.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn klp_find_sympos(elf: *mut elf, sym: *mut symbol) -> c_ulong {
    let mut sympos: c_ulong = 0;
    let mut nr_matches: c_ulong = 0;
    let mut has_dup: bool = false;
    let mut s: *mut symbol = ptr::null_mut();

    if is_init_sym(sym) {
        ERROR(
            c"%s: can't patch or reference init code/data".as_ptr(),
            (*sym).name,
        );
        return ULONG_MAX;
    }

    if (*sym).bind != STB_LOCAL {
        return 0;
    }

    /*
     * vmlinux: the final link reorders symbols relative to vmlinux.o,
     * so the position needs to be derived from the linked orig vmlinux via
     * the .klp.symid table.
     */
    if !VMLINUX.elf.is_null() {
        return find_vmlinux_sympos(sym);
    }

    /*
     * modules: the final .ko preserves symbol table order, so a
     * symtab-order count here matches the runtime count done by
     * module_kallsyms_on_each_symbol().
     */
    /* for_each_sym(elf, s) */
    while for_each_sym_next(elf, &mut s) {
        if strcmp((*s).name, (*sym).name) == 0 {
            nr_matches += 1;
            if s == sym {
                sympos = nr_matches;
            } else {
                has_dup = true;
            }
        }
    }

    if sympos == 0 {
        ERROR(c"can't find sympos for %s".as_ptr(), (*sym).name);
        return ULONG_MAX;
    }

    if has_dup { sympos } else { 0 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
