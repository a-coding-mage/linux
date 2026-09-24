// SPDX-License-Identifier: GPL-2.0-only
//! Native owner of the UUID/GUID C ABI and original export licenses.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "uuid.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(guid_null, guid_null, "", "");
ffi_export::export_symbol!(uuid_null, uuid_null, "", "");
ffi_export::export_symbol!(generate_random_uuid, generate_random_uuid, "", "");
ffi_export::export_symbol!(generate_random_guid, generate_random_guid, "", "");
ffi_export::export_symbol!(guid_gen, guid_gen, "GPL", "");
ffi_export::export_symbol!(uuid_gen, uuid_gen, "GPL", "");
ffi_export::export_symbol!(uuid_is_valid, uuid_is_valid, "", "");
ffi_export::export_symbol!(guid_parse, guid_parse, "", "");
ffi_export::export_symbol!(uuid_parse, uuid_parse, "", "");
