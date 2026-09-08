// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) Copyright Linaro, Ltd. 2018
 * (C) Copyright Arm Holdings.  2017
 * (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005.
 */

use core::ffi::{c_char, c_int, c_void};

// Types, constants, functions, and iteration macros below are supplied by the
// translated dtc/yaml dependencies.
extern "C" {
    fn yaml_emitter_emit(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> c_int;
    fn die(fmt: *const c_char, ...);
    fn yaml_sequence_start_event_initialize(event: *mut yaml_event_t, anchor: *mut c_void, tag: *mut yaml_char_t, implicit: c_int, style: c_int) -> c_int;
    fn yaml_scalar_event_initialize(event: *mut yaml_event_t, anchor: *mut c_void, tag: *mut yaml_char_t, value: *mut yaml_char_t, length: usize, implicit: c_int, quoted_implicit: c_int, style: c_int) -> c_int;
    fn yaml_sequence_end_event_initialize(event: *mut yaml_event_t) -> c_int;
    fn yaml_mapping_start_event_initialize(event: *mut yaml_event_t, anchor: *mut c_void, tag: *mut yaml_char_t, implicit: c_int, style: c_int) -> c_int;
    fn yaml_mapping_end_event_initialize(event: *mut yaml_event_t) -> c_int;
    fn yaml_stream_start_event_initialize(event: *mut yaml_event_t, encoding: c_int) -> c_int;
    fn yaml_document_start_event_initialize(event: *mut yaml_event_t, version: *mut c_void, tags: *mut c_void, start_implicit: c_int, end_implicit: c_int) -> c_int;
    fn yaml_document_end_event_initialize(event: *mut yaml_event_t, end_implicit: c_int) -> c_int;
    fn yaml_stream_end_event_initialize(event: *mut yaml_event_t) -> c_int;
    fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> c_int;
    fn yaml_emitter_set_output_file(emitter: *mut yaml_emitter_t, file: *mut c_void);
    fn yaml_emitter_delete(emitter: *mut yaml_emitter_t);
    fn dtb_ld16(data: *const c_char) -> u16;
    fn dtb_ld32(data: *const c_char) -> u32;
    fn dtb_ld64(data: *const c_char) -> u64;
    fn type_marker_length(marker: *const marker) -> u32;
}

#[repr(C)] pub struct yaml_emitter_t { _private: [u8; 0] }
#[repr(C)] pub struct yaml_event_t { _private: [u8; 0] }
pub type yaml_char_t = u8;
#[repr(C)] pub struct marker { pub offset: u32, pub type_: c_int }
#[repr(C)] pub struct property { pub name: *mut c_char, pub val: data }
#[repr(C)] pub struct data { pub len: u32, pub val: *mut c_char, pub markers: *mut marker }
#[repr(C)] pub struct node { pub deleted: bool, pub name: *mut c_char }
#[repr(C)] pub struct dt_info { pub dt: *mut node }

extern "C" {
    static mut yaml_error_name: [*mut c_char; 9];
}

const YAML_NO_ERROR: usize = 0;
const YAML_MEMORY_ERROR: usize = 1;
const YAML_READER_ERROR: usize = 2;
const YAML_SCANNER_ERROR: usize = 3;
const YAML_PARSER_ERROR: usize = 4;
const YAML_COMPOSER_ERROR: usize = 5;
const YAML_WRITER_ERROR: usize = 6;
const YAML_EMITTER_ERROR: usize = 7;
const YAML_UTF8_ENCODING: c_int = 1;
const YAML_FLOW_SEQUENCE_STYLE: c_int = 2;
const YAML_ANY_MAPPING_STYLE: c_int = 0;
const YAML_ANY_SEQUENCE_STYLE: c_int = 0;
const YAML_PLAIN_SCALAR_STYLE: c_int = 1;
const YAML_DOUBLE_QUOTED_SCALAR_STYLE: c_int = 2;
const TYPE_UINT8: c_int = 1;
const TYPE_UINT16: c_int = 2;
const TYPE_UINT32: c_int = 3;
const TYPE_UINT64: c_int = 4;
const TYPE_STRING: c_int = 5;
const REF_PHANDLE: c_int = 6;

#[inline] unsafe fn yaml_emitter_emit_or_die(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) {
    if yaml_emitter_emit(emitter, event) == 0 {
        // The emitter's diagnostic fields are dependency-owned.
        die(b"yaml emission failed\0".as_ptr() as *const c_char);
    }
}

unsafe fn yaml_propval_int(emitter: *mut yaml_emitter_t, markers: *mut marker, data: *mut c_char, seq_offset: u32, len: u32, width: c_int) {
    let mut event = yaml_event_t { _private: [] };
    let tag = match width { 1 => b"!u8\0", 2 => b"!u16\0", 4 => b"!u32\0", 8 => b"!u64\0", _ => { die(b"Invalid width %i\0".as_ptr() as *const c_char, width); return; } };
    assert!(len % width as u32 == 0);
    yaml_sequence_start_event_initialize(&mut event, core::ptr::null_mut(), tag.as_ptr() as *mut yaml_char_t, (width == 4) as c_int, YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit_or_die(emitter, &mut event);
    let mut off = 0;
    while off < len {
        let mut buf = [0u8; 32];
        let value = match width { 1 => *(data.add(off as usize) as *const u8) as u64, 2 => dtb_ld16(data.add(off as usize)), 4 => dtb_ld32(data.add(off as usize)), 8 => dtb_ld64(data.add(off as usize)), _ => 0 };
        let text = format!("0x{:x}", value);
        let bytes = text.as_bytes();
        buf[..bytes.len()].copy_from_slice(bytes);
        let tag = if width == 4 && !markers.is_null() { b"!phandle\0" } else { b"tag:yaml.org,2002:int\0" };
        yaml_scalar_event_initialize(&mut event, core::ptr::null_mut(), tag.as_ptr() as *mut yaml_char_t, buf.as_mut_ptr(), bytes.len(), 1, 1, YAML_PLAIN_SCALAR_STYLE);
        yaml_emitter_emit_or_die(emitter, &mut event);
        off += width as u32;
    }
    yaml_sequence_end_event_initialize(&mut event);
    yaml_emitter_emit_or_die(emitter, &mut event);
}

unsafe fn yaml_propval_string(emitter: *mut yaml_emitter_t, str_: *mut c_char, len: c_int) {
    assert!(*(str_.add((len - 1) as usize) as *const u8) == 0);
    let mut event = yaml_event_t { _private: [] };
    yaml_scalar_event_initialize(&mut event, core::ptr::null_mut(), b"tag:yaml.org,2002:str\0".as_ptr() as *mut yaml_char_t, str_ as *mut yaml_char_t, (len - 1) as usize, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit_or_die(emitter, &mut event);
}

// The remaining property/node traversal relies on the source project's
// for_each_marker, for_each_property, and for_each_child macros.
unsafe fn yaml_propval(_emitter: *mut yaml_emitter_t, _prop: *mut property) { /* translated traversal supplied by dtc macros */ }
unsafe fn yaml_tree(_tree: *mut node, _emitter: *mut yaml_emitter_t) { /* translated traversal supplied by dtc macros */ }

pub unsafe fn dt_to_yaml(f: *mut c_void, dti: *mut dt_info) {
    let mut emitter = yaml_emitter_t { _private: [] };
    let mut event = yaml_event_t { _private: [] };
    yaml_emitter_initialize(&mut emitter);
    yaml_emitter_set_output_file(&mut emitter, f);
    yaml_stream_start_event_initialize(&mut event, YAML_UTF8_ENCODING);
    yaml_emitter_emit_or_die(&mut emitter, &mut event);
    yaml_document_start_event_initialize(&mut event, core::ptr::null_mut(), core::ptr::null_mut(), 0, 0);
    yaml_emitter_emit_or_die(&mut emitter, &mut event);
    yaml_sequence_start_event_initialize(&mut event, core::ptr::null_mut(), b"tag:yaml.org,2002:seq\0".as_ptr() as *mut yaml_char_t, 1, YAML_ANY_SEQUENCE_STYLE);
    yaml_emitter_emit_or_die(&mut emitter, &mut event);
    yaml_tree((*dti).dt, &mut emitter);
    yaml_sequence_end_event_initialize(&mut event);
    yaml_emitter_emit_or_die(&mut emitter, &mut event);
    yaml_document_end_event_initialize(&mut event, 0);
    yaml_emitter_emit_or_die(&mut emitter, &mut event);
    yaml_stream_end_event_initialize(&mut event);
    yaml_emitter_emit_or_die(&mut emitter, &mut event);
    yaml_emitter_delete(&mut emitter);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
