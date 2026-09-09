// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Translation of acpi/acpica/exdump.c.  External ACPICA types, constants,
// macros, and routines are supplied by the surrounding translation unit.

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
use core::{ffi::{c_char, c_void}, ptr};

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
unsafe fn acpi_ex_out_string(title: *const c_char, value: *const c_char) {
    acpi_os_printf(cstr!("%20s : %s\n"), title, value);
}
#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
unsafe fn acpi_ex_out_pointer(title: *const c_char, value: *const c_void) {
    acpi_os_printf(cstr!("%20s : %p\n"), title, value);
}

// Object descriptor information tables. ACPI_EXD_OFFSET/NSOFFSET and the
// descriptor layout are defined by the ACPICA dependency.
#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
static mut ACPI_EX_DUMP_INTEGER: [acpi_exdump_info; 2] = [
    exd_init!(2), exd!(ACPI_EXD_UINT64, ACPI_EXD_OFFSET(integer.value), "Value")];
#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
static mut ACPI_EX_DUMP_STRING: [acpi_exdump_info; 4] = [
    exd_init!(4), exd!(ACPI_EXD_UINT32, ACPI_EXD_OFFSET(string.length), "Length"),
    exd!(ACPI_EXD_POINTER, ACPI_EXD_OFFSET(string.pointer), "Pointer"), exd!(ACPI_EXD_STRING, 0, 0)];
#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
static mut ACPI_EX_DUMP_BUFFER: [acpi_exdump_info; 5] = [
    exd_init!(5), exd!(ACPI_EXD_UINT32, ACPI_EXD_OFFSET(buffer.length), "Length"),
    exd!(ACPI_EXD_POINTER, ACPI_EXD_OFFSET(buffer.pointer), "Pointer"),
    exd!(ACPI_EXD_NODE, ACPI_EXD_OFFSET(buffer.node), "Parent Node"), exd!(ACPI_EXD_BUFFER, 0, 0)];
#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
static mut ACPI_EX_DUMP_PACKAGE: [acpi_exdump_info; 6] = [
    exd_init!(6), exd!(ACPI_EXD_NODE, ACPI_EXD_OFFSET(package.node), "Parent Node"),
    exd!(ACPI_EXD_UINT8, ACPI_EXD_OFFSET(package.flags), "Flags"),
    exd!(ACPI_EXD_UINT32, ACPI_EXD_OFFSET(package.count), "Element Count"),
    exd!(ACPI_EXD_POINTER, ACPI_EXD_OFFSET(package.elements), "Element List"), exd!(ACPI_EXD_PACKAGE, 0, 0)];

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
unsafe fn acpi_ex_dump_object(mut obj: *mut acpi_operand_object, mut info: *mut acpi_exdump_info) {
    if info.is_null() { acpi_os_printf(cstr!("ExDumpObject: Display not implemented for object type %s\n"), acpi_ut_get_object_type_name(obj)); return; }
    let mut count = (*info).offset;
    while count != 0 {
        if obj.is_null() { return; }
        let target = (obj as *mut u8).add((*info).offset as usize);
        let name = (*info).name;
        match (*info).opcode {
            ACPI_EXD_INIT => {},
            ACPI_EXD_TYPE => acpi_os_printf(cstr!("%20s : %2.2X [%s]\n"), cstr!("Type"), (*obj).common.type_, acpi_ut_get_object_type_name(obj)),
            ACPI_EXD_UINT8 => acpi_os_printf(cstr!("%20s : %2.2X\n"), name, *target),
            ACPI_EXD_UINT16 => acpi_os_printf(cstr!("%20s : %4.4X\n"), name, ACPI_GET16(target)),
            ACPI_EXD_UINT32 => acpi_os_printf(cstr!("%20s : %8.8X\n"), name, ACPI_GET32(target)),
            ACPI_EXD_UINT64 => acpi_os_printf(cstr!("%20s : %8.8X%8.8X\n"), cstr!("Value"), ACPI_FORMAT_UINT64(ACPI_GET64(target))),
            ACPI_EXD_POINTER | ACPI_EXD_ADDRESS => acpi_ex_out_pointer(name, *(target as *mut *const c_void)),
            ACPI_EXD_STRING => { acpi_ut_print_string((*obj).string.pointer, ACPI_UINT8_MAX); acpi_os_printf(cstr!("\n")); },
            ACPI_EXD_BUFFER => ACPI_DUMP_BUFFER((*obj).buffer.pointer, (*obj).buffer.length),
            ACPI_EXD_PACKAGE => { acpi_os_printf(cstr!("\nPackage Contents:\n")); acpi_ex_dump_package_obj(obj, 0, 0); },
            ACPI_EXD_FIELD => acpi_ex_dump_object(obj, ACPI_EX_DUMP_FIELD_COMMON.as_mut_ptr()),
            ACPI_EXD_REFERENCE => { acpi_ex_out_string(cstr!("Class Name"), acpi_ut_get_reference_name(obj)); acpi_ex_dump_reference_obj(obj); },
            ACPI_EXD_NODE => { let node = *(target as *mut *mut acpi_namespace_node); acpi_os_printf(cstr!("%20s : %p"), name, node); if !node.is_null() { acpi_os_printf(cstr!(" [%4.4s]"), (*node).name.ascii.as_ptr()); } acpi_os_printf(cstr!("\n")); },
            _ => { acpi_os_printf(cstr!("**** Invalid table opcode [%X] ****\n"), (*info).opcode); return; }
        }
        info = info.add(1); count -= 1;
    }
}

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
unsafe fn acpi_ex_dump_reference_obj(obj: *mut acpi_operand_object) {
    if (*obj).reference.class == ACPI_REFCLASS_NAME { acpi_os_printf(cstr!(" %p "), (*obj).reference.node); }
    else if !(*obj).reference.object.is_null() { acpi_os_printf(cstr!(" Target: %p\n"), (*obj).reference.object); }
}

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
unsafe fn acpi_ex_dump_package_obj(obj: *mut acpi_operand_object, level: u32, index: u32) {
    if level != 0 { for _ in 0..level { acpi_os_printf(cstr!(" ")); } acpi_os_printf(cstr!("[%.2d] "), index); }
    acpi_os_printf(cstr!("%p "), obj);
    if obj.is_null() { acpi_os_printf(cstr!("[Null Object]\n")); return; }
    match (*obj).common.type_ {
        ACPI_TYPE_INTEGER => acpi_os_printf(cstr!("[Integer] = %8.8X%8.8X\n"), ACPI_FORMAT_UINT64((*obj).integer.value)),
        ACPI_TYPE_STRING => { acpi_os_printf(cstr!("[String] Value: ")); acpi_ut_print_string((*obj).string.pointer, ACPI_UINT8_MAX); acpi_os_printf(cstr!("\n")); },
        ACPI_TYPE_BUFFER => { acpi_os_printf(cstr!("[Buffer] Length %.2X = \n"), (*obj).buffer.length); },
        ACPI_TYPE_PACKAGE => { acpi_os_printf(cstr!("[Package] Contains %u Elements:\n"), (*obj).package.count); for i in 0..(*obj).package.count { acpi_ex_dump_package_obj(*(*obj).package.elements.add(i as usize), level + 1, i); } },
        ACPI_TYPE_LOCAL_REFERENCE => { acpi_os_printf(cstr!("[Object Reference] Class [%s]"), acpi_ut_get_reference_name(obj)); acpi_ex_dump_reference_obj(obj); },
        _ => acpi_os_printf(cstr!("[%s] Type: %2.2X\n"), acpi_ut_get_type_name((*obj).common.type_), (*obj).common.type_)
    }
}

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
pub unsafe fn acpi_ex_dump_operand(obj: *mut acpi_operand_object, depth: u32) {
    if obj.is_null() { ACPI_DEBUG_PRINT!((ACPI_DB_EXEC, cstr!("Null Object Descriptor\n"))); return; }
    match (*obj).common.type_ {
        ACPI_TYPE_BUFFER => acpi_os_printf(cstr!("Buffer length %.2X @ %p\n"), (*obj).buffer.length, (*obj).buffer.pointer),
        ACPI_TYPE_INTEGER => acpi_os_printf(cstr!("Integer %8.8X%8.8X\n"), ACPI_FORMAT_UINT64((*obj).integer.value)),
        ACPI_TYPE_STRING => { acpi_os_printf(cstr!("String length %X @ %p "), (*obj).string.length, (*obj).string.pointer); acpi_ut_print_string((*obj).string.pointer, ACPI_UINT8_MAX); acpi_os_printf(cstr!("\n")); },
        ACPI_TYPE_PACKAGE => { acpi_os_printf(cstr!("Package [Len %X] ElementArray %p\n"), (*obj).package.count, (*obj).package.elements); if acpi_dbg_level > 1 { for i in 0..(*obj).package.count { acpi_ex_dump_operand(*(*obj).package.elements.add(i as usize), depth + 1); } } },
        _ => acpi_os_printf(cstr!("Unknown Type %X\n"), (*obj).common.type_)
    }
}

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
pub unsafe fn acpi_ex_dump_operands(mut operands: *mut *mut acpi_operand_object, opcode_name: *const c_char, mut num_operands: u32) {
    if opcode_name.is_null() { /* C assigns UNKNOWN; caller-owned pointer remains unchanged here. */ }
    if num_operands == 0 { num_operands = 1; }
    while num_operands != 0 { acpi_ex_dump_operand(*operands, 0); operands = operands.add(1); num_operands -= 1; }
}

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
pub unsafe fn acpi_ex_dump_namespace_node(node: *mut acpi_namespace_node, flags: u32) { if flags != 0 { acpi_ex_dump_object(node as *mut acpi_operand_object, ACPI_EX_DUMP_NODE.as_mut_ptr()); } }

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
pub unsafe fn acpi_ex_dump_object_descriptor(mut obj: *mut acpi_operand_object, flags: u32) {
    if obj.is_null() { return; }
    if (*obj).common.type_ <= ACPI_TYPE_LOCAL_MAX { acpi_ex_dump_object(obj, ACPI_EX_DUMP_COMMON.as_mut_ptr()); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
