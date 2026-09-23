// SPDX-License-Identifier: GPL-2.0
//! Lifetime-bound access to the existing elfutils DWARF reader.
//!
//! All FFI is confined here. A session owns its descriptor, callbacks and
//! library handles; DIEs and strings cannot outlive that session. C aggregate
//! layouts and function signatures follow elfutils libdw/libdwfl public ABI.

use crate::gendwarfksyms_header::{error, Result};
use std::collections::HashSet;
use std::ffi::{c_char, c_int, c_long, c_uint, c_void, CStr, CString};
use std::fs::File;
use std::marker::PhantomData;
use std::os::fd::{AsRawFd, IntoRawFd};
use std::path::Path;
use std::ptr::{self, NonNull};

macro_rules! opaque {
    ($($name:ident),*) => { $(#[repr(C)] struct $name { _private: [u8; 0] })* };
}
opaque!(Dwfl, DwflModule, Dwarf, DwarfCu, Abbrev, Files, Elf);

#[repr(C)]
#[derive(Clone, Copy)]
struct RawDie {
    addr: *mut c_void,
    cu: *mut DwarfCu,
    abbrev: *mut Abbrev,
    padding: c_long,
}

impl RawDie {
    const EMPTY: Self = Self {
        addr: ptr::null_mut(),
        cu: ptr::null_mut(),
        abbrev: ptr::null_mut(),
        padding: 0,
    };
}

#[repr(C)]
struct Attribute {
    code: c_uint,
    form: c_uint,
    value: *mut u8,
    cu: *mut DwarfCu,
}

type FindElf = unsafe extern "C" fn(
    *mut DwflModule,
    *mut *mut c_void,
    *const c_char,
    u64,
    *mut *mut c_char,
    *mut *mut Elf,
) -> c_int;
type FindDebug = unsafe extern "C" fn(
    *mut DwflModule,
    *mut *mut c_void,
    *const c_char,
    u64,
    *const c_char,
    *const c_char,
    u32,
    *mut *mut c_char,
) -> c_int;
// The section header is borrowed by elfutils; the Rust caller never inspects it.
type SectionAddress = unsafe extern "C" fn(
    *mut DwflModule,
    *mut *mut c_void,
    *const c_char,
    u64,
    *const c_char,
    u32,
    *const c_void,
    *mut u64,
) -> c_int;
type ModuleCallback = unsafe extern "C" fn(
    *mut DwflModule,
    *mut *mut c_void,
    *const c_char,
    u64,
    *mut c_void,
) -> c_int;
type RemovedCallback =
    unsafe extern "C" fn(*mut DwflModule, *mut c_void, *const c_char, u64, *mut c_void) -> c_int;

#[repr(C)]
struct Callbacks {
    find_elf: Option<FindElf>,
    find_debug: Option<FindDebug>,
    section_address: Option<SectionAddress>,
    debug_path: *mut *mut c_char,
}

// Library selection belongs to Kbuild (or the standalone test's link flags).
unsafe extern "C" {
    fn dwfl_begin(callbacks: *const Callbacks) -> *mut Dwfl;
    fn dwfl_end(session: *mut Dwfl);
    fn dwfl_report_offline(
        session: *mut Dwfl,
        name: *const c_char,
        path: *const c_char,
        fd: c_int,
    ) -> *mut DwflModule;
    fn dwfl_report_end(
        session: *mut Dwfl,
        removed: Option<RemovedCallback>,
        arg: *mut c_void,
    ) -> c_int;
    fn dwfl_getmodules(
        session: *mut Dwfl,
        callback: ModuleCallback,
        arg: *mut c_void,
        offset: isize,
    ) -> isize;
    fn dwfl_standard_find_debuginfo(
        module: *mut DwflModule,
        userdata: *mut *mut c_void,
        name: *const c_char,
        base: u64,
        file: *const c_char,
        debuglink: *const c_char,
        crc: u32,
        result: *mut *mut c_char,
    ) -> c_int;
    fn dwfl_offline_section_address(
        module: *mut DwflModule,
        userdata: *mut *mut c_void,
        name: *const c_char,
        base: u64,
        section: *const c_char,
        index: u32,
        header: *const c_void,
        address: *mut u64,
    ) -> c_int;
    fn dwfl_module_getdwarf(module: *mut DwflModule, bias: *mut u64) -> *mut Dwarf;
    fn dwarf_errmsg(error: c_int) -> *const c_char;
    fn dwarf_get_units(
        dwarf: *mut Dwarf,
        cu: *mut DwarfCu,
        next: *mut *mut DwarfCu,
        version: *mut u16,
        kind: *mut u8,
        die: *mut RawDie,
        subdie: *mut RawDie,
    ) -> c_int;
    fn dwarf_tag(die: *mut RawDie) -> c_int;
    fn dwarf_child(die: *mut RawDie, result: *mut RawDie) -> c_int;
    fn dwarf_siblingof(die: *mut RawDie, result: *mut RawDie) -> c_int;
    fn dwarf_attr(die: *mut RawDie, name: c_uint, result: *mut Attribute) -> *mut Attribute;
    fn dwarf_formstring(attribute: *mut Attribute) -> *const c_char;
    fn dwarf_formudata(attribute: *mut Attribute, result: *mut u64) -> c_int;
    fn dwarf_formflag(attribute: *mut Attribute, result: *mut bool) -> c_int;
    fn dwarf_formref_die(attribute: *mut Attribute, result: *mut RawDie) -> *mut RawDie;
    fn dwarf_cu_getdwarf(cu: *mut DwarfCu) -> *mut Dwarf;
    fn dwarf_die_addr_die(
        dwarf: *mut Dwarf,
        address: *mut c_void,
        result: *mut RawDie,
    ) -> *mut RawDie;
    fn dwarf_cu_die(
        cu: *mut DwarfCu,
        result: *mut RawDie,
        version: *mut u16,
        abbrev_offset: *mut u64,
        address_size: *mut u8,
        offset_size: *mut u8,
        signature: *mut u64,
        type_offset: *mut u64,
    ) -> *mut RawDie;
    fn dwarf_getsrcfiles(cu: *mut RawDie, files: *mut *mut Files, count: *mut usize) -> c_int;
    fn dwarf_filesrc(
        files: *mut Files,
        index: usize,
        mtime: *mut u64,
        size: *mut u64,
    ) -> *const c_char;
}

fn reason() -> Vec<u8> {
    // SAFETY: -1 requests an always-valid elfutils error string. Copy it before
    // another library call can change the thread's diagnostic state.
    unsafe { CStr::from_ptr(dwarf_errmsg(-1)).to_bytes().to_vec() }
}

pub(crate) struct Session {
    handle: NonNull<Dwfl>,
    // elfutils may retain these addresses, so both allocations live until Drop.
    _callbacks: Box<Callbacks>,
    path: CString,
}

impl Session {
    pub(crate) fn open(file: File, path: &Path) -> Result<Self> {
        let path = CString::new(path.as_os_str().as_encoded_bytes())
            .map_err(|_| error("main", &[b"input path contains a NUL byte"]))?;
        let callbacks = Box::new(Callbacks {
            find_elf: None,
            find_debug: Some(dwfl_standard_find_debuginfo),
            section_address: Some(dwfl_offline_section_address),
            debug_path: ptr::null_mut(),
        });
        // SAFETY: callbacks have the public ABI layout and remain allocated.
        let handle = NonNull::new(unsafe { dwfl_begin(&*callbacks) }).ok_or_else(|| {
            error(
                "main",
                &[
                    b"dwfl_begin failed for '",
                    path.to_bytes(),
                    b"': ",
                    &reason(),
                ],
            )
        })?;
        let session = Self {
            handle,
            _callbacks: callbacks,
            path,
        };
        // SAFETY: file is open and owned here. The library consumes it only on
        // success; on failure File closes it, and Session drops the DWFL state.
        let module = unsafe {
            dwfl_report_offline(
                session.handle.as_ptr(),
                session.path.as_ptr(),
                session.path.as_ptr(),
                file.as_raw_fd(),
            )
        };
        if module.is_null() {
            return Err(error(
                "main",
                &[
                    b"dwfl_report_offline failed for '",
                    session.path.to_bytes(),
                    b"': ",
                    &reason(),
                ],
            ));
        }
        let _transferred_fd = file.into_raw_fd();
        // SAFETY: reporting has finished; no callback/user data is requested.
        if unsafe { dwfl_report_end(session.handle.as_ptr(), None, ptr::null_mut()) } != 0 {
            return Err(error(
                "main",
                &[
                    b"dwfl_report_end failed for '",
                    session.path.to_bytes(),
                    b"'",
                ],
            ));
        }
        Ok(session)
    }

    pub(crate) fn modules(&self) -> Result<Vec<Module<'_>>> {
        type Record = (*mut DwflModule, *const c_char);
        unsafe extern "C" fn collect(
            module: *mut DwflModule,
            _: *mut *mut c_void,
            name: *const c_char,
            _: u64,
            arg: *mut c_void,
        ) -> c_int {
            if module.is_null() || name.is_null() {
                return 1;
            }
            // SAFETY: dwfl_getmodules calls synchronously with the unique Vec
            // passed below; no Rust borrow survives a callback invocation.
            unsafe { &mut *arg.cast::<Vec<Record>>() }.push((module, name));
            0
        }
        let mut records: Vec<Record> = Vec::new();
        // SAFETY: handle is live, and callback argument is valid throughout.
        let result = unsafe {
            dwfl_getmodules(
                self.handle.as_ptr(),
                collect,
                ptr::from_mut(&mut records).cast(),
                0,
            )
        };
        if result != 0 {
            return Err(error(
                "main",
                &[b"dwfl_getmodules failed for '", self.path.to_bytes(), b"'"],
            ));
        }
        Ok(records
            .into_iter()
            .map(|(raw, name)| Module {
                raw,
                name,
                owner: self,
            })
            .collect())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // SAFETY: this is the sole owning handle; all borrowed objects have
        // expired before Rust can drop the session and its consumed descriptor.
        unsafe { dwfl_end(self.handle.as_ptr()) };
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Module<'a> {
    raw: *mut DwflModule,
    name: *const c_char,
    owner: &'a Session,
}

impl<'a> Module<'a> {
    pub(crate) fn name(&self) -> &[u8] {
        // SAFETY: the checked callback name remains owned by this session.
        unsafe { CStr::from_ptr(self.name).to_bytes() }
    }

    pub(crate) fn units(self) -> Result<Vec<Die<'a>>> {
        let mut bias = 0;
        // SAFETY: module belongs to owner; bias is valid writable storage.
        let dwarf = unsafe { dwfl_module_getdwarf(self.raw, &mut bias) };
        let failed = || {
            error(
                "process_module",
                &[b"dwarf_get_units failed: no debugging information?"],
            )
        };
        if dwarf.is_null() {
            return Err(failed());
        }
        let mut cu = ptr::null_mut();
        let mut units = Vec::new();
        let mut seen = HashSet::new();
        loop {
            let mut raw = RawDie::EMPTY;
            // SAFETY: all descriptors come from elfutils and all optional
            // outputs are null exactly as permitted by dwarf_get_units.
            let status = unsafe {
                dwarf_get_units(
                    dwarf,
                    cu,
                    &mut cu,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    &mut raw,
                    ptr::null_mut(),
                )
            };
            if status < 0 {
                return Err(failed());
            }
            if status == 1 {
                break;
            }
            if raw.addr.is_null() || raw.cu.is_null() || !seen.insert(raw.addr as usize) {
                return Err(error("process_module", &[b"invalid or cyclic DWARF unit"]));
            }
            units.push(Die {
                raw,
                owner: PhantomData::<&'a Session>,
            });
            if cu.is_null() {
                break;
            }
        }
        // The owner borrow is deliberately retained by the returned DIEs.
        let _owner = self.owner;
        Ok(units)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Die<'a> {
    raw: RawDie,
    owner: PhantomData<&'a Session>,
}

impl<'a> Die<'a> {
    pub(crate) fn addr(self) -> usize {
        self.raw.addr as usize
    }

    pub(crate) fn tag(mut self) -> i32 {
        // SAFETY: raw is a valid, lifetime-bound library DIE; elfutils may fill
        // its abbreviation cache, which is why a local mutable copy is used.
        unsafe { dwarf_tag(&mut self.raw) }
    }

    fn attribute(mut self, id: u32) -> Option<Attribute> {
        let mut attr = Attribute {
            code: 0,
            form: 0,
            value: ptr::null_mut(),
            cu: ptr::null_mut(),
        };
        // SAFETY: output has the documented C attribute layout.
        (!unsafe { dwarf_attr(&mut self.raw, id, &mut attr) }.is_null()).then_some(attr)
    }

    pub(crate) fn string(self, id: u32) -> Option<&'a [u8]> {
        let mut attr = self.attribute(id)?;
        // SAFETY: successful strings are borrowed from the session's DWARF.
        let pointer = unsafe { dwarf_formstring(&mut attr) };
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(pointer).to_bytes() })
        }
    }

    pub(crate) fn udata(self, id: u32) -> Option<u64> {
        let mut attr = self.attribute(id)?;
        let mut value = 0;
        // SAFETY: the attribute and output storage are valid for this call.
        (unsafe { dwarf_formudata(&mut attr, &mut value) } == 0).then_some(value)
    }

    pub(crate) fn flag(self, id: u32) -> Option<bool> {
        let mut attr = self.attribute(id)?;
        let mut value = false;
        // SAFETY: C bool and Rust bool share the platform C ABI.
        (unsafe { dwarf_formflag(&mut attr, &mut value) } == 0).then_some(value)
    }

    pub(crate) fn reference(self, id: u32) -> Option<Self> {
        let mut attr = self.attribute(id)?;
        let mut raw = RawDie::EMPTY;
        // SAFETY: referenced DIEs share the owning session's lifetime.
        let found = unsafe { dwarf_formref_die(&mut attr, &mut raw) };
        if found.is_null() || raw.addr.is_null() || raw.cu.is_null() {
            None
        } else {
            Some(Self {
                raw,
                owner: PhantomData,
            })
        }
    }

    pub(crate) fn children(mut self) -> Result<Vec<Self>> {
        let mut raw = RawDie::EMPTY;
        // SAFETY: both input and output DIE storage are valid.
        let mut status = unsafe { dwarf_child(&mut self.raw, &mut raw) };
        let mut result = Vec::new();
        let mut seen = HashSet::new();
        let mut call = "dwarf_child(die, &current)";
        loop {
            if status < 0 {
                return Err(error(
                    "process_die_container",
                    &[format!("`{call}` failed: {status}").as_bytes()],
                ));
            }
            if status != 0 {
                break;
            }
            if raw.addr.is_null() || raw.cu.is_null() || !seen.insert(raw.addr as usize) {
                return Err(error(
                    "process_die_container",
                    &[b"invalid or cyclic DWARF sibling"],
                ));
            }
            result.push(Self {
                raw,
                owner: PhantomData,
            });
            let mut next = RawDie::EMPTY;
            // SAFETY: raw came from a successful child/sibling operation.
            status = unsafe { dwarf_siblingof(&mut raw, &mut next) };
            raw = next;
            call = "dwarf_siblingof(&current, &current)";
        }
        Ok(result)
    }

    pub(crate) fn from_address(self, address: usize) -> Result<Self> {
        let mut raw = RawDie::EMPTY;
        // SAFETY: elfutils validates an opaque cached address against its own
        // DWARF mappings before returning a reconstructed DIE. It is never
        // dereferenced by Rust. The CU and DWARF remain owned by our session.
        let found = unsafe {
            dwarf_die_addr_die(
                dwarf_cu_getdwarf(self.raw.cu),
                address as *mut c_void,
                &mut raw,
            )
        };
        if found.is_null() || raw.addr.is_null() || raw.cu.is_null() {
            Err(error("process_cached", &[b"dwarf_die_addr_die failed"]))
        } else {
            Ok(Self {
                raw,
                owner: PhantomData,
            })
        }
    }

    pub(crate) fn source_file(self, index: u64) -> Result<&'a [u8]> {
        let failed = |call: &[u8]| {
            error(
                "is_definition_private",
                &[call, b" failed: '", &reason(), b"'"],
            )
        };
        let index: usize = index
            .try_into()
            .map_err(|_| error("is_definition_private", &[b"file index overflow"]))?;
        let mut cu = RawDie::EMPTY;
        // SAFETY: all optional CU header fields may be omitted.
        if unsafe {
            dwarf_cu_die(
                self.raw.cu,
                &mut cu,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
        .is_null()
        {
            return Err(failed(b"dwarf_cu_die"));
        }
        let mut files = ptr::null_mut();
        // SAFETY: the returned file table belongs to this compilation unit.
        if unsafe { dwarf_getsrcfiles(&mut cu, &mut files, ptr::null_mut()) } != 0
            || files.is_null()
        {
            return Err(failed(b"dwarf_getsrcfiles"));
        }
        // SAFETY: elfutils checks the index and returns null if out of bounds.
        let path = unsafe { dwarf_filesrc(files, index, ptr::null_mut(), ptr::null_mut()) };
        if path.is_null() {
            return Err(failed(b"dwarf_filesrc"));
        }
        Ok(unsafe { CStr::from_ptr(path).to_bytes() })
    }
}

/// DWARF wire constants used by the processor (from the public dwarf.h).
#[allow(non_upper_case_globals)]
pub(crate) mod constants {
    pub(crate) const DW_TAG_array_type: i32 = 0x01;
    pub(crate) const DW_TAG_class_type: i32 = 0x02;
    pub(crate) const DW_TAG_enumeration_type: i32 = 0x04;
    pub(crate) const DW_TAG_formal_parameter: i32 = 0x05;
    pub(crate) const DW_TAG_member: i32 = 0x0d;
    pub(crate) const DW_TAG_pointer_type: i32 = 0x0f;
    pub(crate) const DW_TAG_reference_type: i32 = 0x10;
    pub(crate) const DW_TAG_structure_type: i32 = 0x13;
    pub(crate) const DW_TAG_subroutine_type: i32 = 0x15;
    pub(crate) const DW_TAG_typedef: i32 = 0x16;
    pub(crate) const DW_TAG_union_type: i32 = 0x17;
    pub(crate) const DW_TAG_variant: i32 = 0x19;
    pub(crate) const DW_TAG_subrange_type: i32 = 0x21;
    pub(crate) const DW_TAG_base_type: i32 = 0x24;
    pub(crate) const DW_TAG_const_type: i32 = 0x26;
    pub(crate) const DW_TAG_enumerator: i32 = 0x28;
    pub(crate) const DW_TAG_packed_type: i32 = 0x2d;
    pub(crate) const DW_TAG_subprogram: i32 = 0x2e;
    pub(crate) const DW_TAG_template_type_parameter: i32 = 0x2f;
    pub(crate) const DW_TAG_variant_part: i32 = 0x33;
    pub(crate) const DW_TAG_variable: i32 = 0x34;
    pub(crate) const DW_TAG_volatile_type: i32 = 0x35;
    pub(crate) const DW_TAG_restrict_type: i32 = 0x37;
    pub(crate) const DW_TAG_namespace: i32 = 0x39;
    pub(crate) const DW_TAG_unspecified_type: i32 = 0x3b;
    pub(crate) const DW_TAG_shared_type: i32 = 0x40;
    pub(crate) const DW_TAG_rvalue_reference_type: i32 = 0x42;
    pub(crate) const DW_TAG_atomic_type: i32 = 0x47;
    pub(crate) const DW_TAG_immutable_type: i32 = 0x4b;
    pub(crate) const DW_AT_name: u32 = 0x03;
    pub(crate) const DW_AT_byte_size: u32 = 0x0b;
    pub(crate) const DW_AT_bit_size: u32 = 0x0d;
    pub(crate) const DW_AT_discr_value: u32 = 0x16;
    pub(crate) const DW_AT_const_value: u32 = 0x1c;
    pub(crate) const DW_AT_upper_bound: u32 = 0x2f;
    pub(crate) const DW_AT_abstract_origin: u32 = 0x31;
    pub(crate) const DW_AT_accessibility: u32 = 0x32;
    pub(crate) const DW_AT_count: u32 = 0x37;
    pub(crate) const DW_AT_data_member_location: u32 = 0x38;
    pub(crate) const DW_AT_decl_file: u32 = 0x3a;
    pub(crate) const DW_AT_declaration: u32 = 0x3c;
    pub(crate) const DW_AT_encoding: u32 = 0x3e;
    pub(crate) const DW_AT_type: u32 = 0x49;
    pub(crate) const DW_AT_data_bit_offset: u32 = 0x6b;
    pub(crate) const DW_AT_linkage_name: u32 = 0x6e;
    pub(crate) const DW_AT_alignment: u32 = 0x88;
}
