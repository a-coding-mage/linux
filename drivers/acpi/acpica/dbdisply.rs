// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Rust translation of ACPICA dbdisply.c. External ACPICA symbols are supplied by dependencies.

const ACPI_PREDEFINED_PREFIX: &str = "%25s (%.2X) : ";
const ACPI_HANDLER_NAME_STRING: &str = "%30s : ";
const ACPI_HANDLER_PRESENT_STRING: &str = "%-9s (%p)\n";
const ACPI_HANDLER_PRESENT_STRING2: &str = "%-9s (%p)";
const ACPI_HANDLER_NOT_PRESENT_STRING: &str = "%-9s\n";

#[repr(C)]
struct AcpiHandlerInfo { handler: *mut core::ffi::c_void, name: *mut i8 }

static mut ACPI_GBL_SPACE_ID_LIST: [u32; 14] = [
    ACPI_ADR_SPACE_SYSTEM_MEMORY, ACPI_ADR_SPACE_SYSTEM_IO,
    ACPI_ADR_SPACE_PCI_CONFIG, ACPI_ADR_SPACE_EC, ACPI_ADR_SPACE_SMBUS,
    ACPI_ADR_SPACE_CMOS, ACPI_ADR_SPACE_PCI_BAR_TARGET, ACPI_ADR_SPACE_IPMI,
    ACPI_ADR_SPACE_GPIO, ACPI_ADR_SPACE_GSBUS, ACPI_ADR_SPACE_PLATFORM_COMM,
    ACPI_ADR_SPACE_PLATFORM_RT, ACPI_ADR_SPACE_DATA_TABLE,
    ACPI_ADR_SPACE_FIXED_HARDWARE,
];

static mut ACPI_GBL_HANDLER_LIST: [AcpiHandlerInfo; 5] = [
    AcpiHandlerInfo { handler: unsafe { &mut ACPI_GBL_GLOBAL_NOTIFY[0].handler as *mut _ as *mut _ }, name: b"System Notifications\0" as *const _ as *mut i8 },
    AcpiHandlerInfo { handler: unsafe { &mut ACPI_GBL_GLOBAL_NOTIFY[1].handler as *mut _ as *mut _ }, name: b"Device Notifications\0" as *const _ as *mut i8 },
    AcpiHandlerInfo { handler: unsafe { &mut ACPI_GBL_TABLE_HANDLER as *mut _ as *mut _ }, name: b"ACPI Table Events\0" as *const _ as *mut i8 },
    AcpiHandlerInfo { handler: unsafe { &mut ACPI_GBL_EXCEPTION_HANDLER as *mut _ as *mut _ }, name: b"Control Method Exceptions\0" as *const _ as *mut i8 },
    AcpiHandlerInfo { handler: unsafe { &mut ACPI_GBL_INTERFACE_HANDLER as *mut _ as *mut _ }, name: b"OSI Invocations\0" as *const _ as *mut i8 },
];

unsafe fn acpi_db_get_pointer(target: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    ACPI_TO_POINTER(strtoul(target, core::ptr::null_mut(), 16))
}

unsafe fn acpi_db_dump_parser_descriptor(op: *mut AcpiParseObject) {
    let info = acpi_ps_get_opcode_info((*op).common.aml_opcode);
    acpi_os_printf(b"Parser Op Descriptor:\n\0".as_ptr() as *const i8);
    acpi_os_printf(b"%20.20s : %4.4X\n\0".as_ptr() as *const i8, b"Opcode\0".as_ptr(), (*op).common.aml_opcode);
    acpi_os_printf(b"%20.20s : %s\n\0".as_ptr() as *const i8, b"Opcode Name\0".as_ptr(), (*info).name);
    acpi_os_printf(b"%20.20s : %p\n\0".as_ptr() as *const i8, b"Value/ArgList\0".as_ptr(), (*op).common.value.arg);
    acpi_os_printf(b"%20.20s : %p\n\0".as_ptr() as *const i8, b"Parent\0".as_ptr(), (*op).common.parent);
    acpi_os_printf(b"%20.20s : %p\n\0".as_ptr() as *const i8, b"NextOp\0".as_ptr(), (*op).common.next);
}

pub unsafe fn acpi_db_decode_and_display_object(target: *mut i8, output_type: *mut i8) {
    if target.is_null() { return; }
    let mut display = DB_BYTE_DISPLAY;
    let mut buffer = [0i8; 80];
    let mut ret_buf = AcpiBuffer { length: buffer.len(), pointer: buffer.as_mut_ptr() as *mut _ };
    if !output_type.is_null() {
        acpi_ut_strupr(output_type);
        match *output_type as u8 { b'W' => display = DB_WORD_DISPLAY, b'D' => display = DB_DWORD_DISPLAY, b'Q' => display = DB_QWORD_DISPLAY, _ => {} }
    }
    if (*target as u8) >= 0x30 && (*target as u8) <= 0x39 {
        let obj_ptr = acpi_db_get_pointer(target as *mut _);
        if !acpi_os_readable(obj_ptr, 16) { acpi_os_printf(b"Address %p is invalid in this address space\n\0".as_ptr() as *const i8, obj_ptr); return; }
        match ACPI_GET_DESCRIPTOR_TYPE(obj_ptr) {
            ACPI_DESC_TYPE_NAMED => { if !acpi_os_readable(obj_ptr, core::mem::size_of::<AcpiNamespaceNode>()) { return; } acpi_db_dump_node(obj_ptr as *mut _, display, &mut ret_buf); }
            ACPI_DESC_TYPE_OPERAND => { acpi_ut_debug_dump_buffer(obj_ptr, core::mem::size_of::<AcpiOperandObject>(), display, ACPI_UINT32_MAX); acpi_ex_dump_object_descriptor(obj_ptr, 1); }
            ACPI_DESC_TYPE_PARSER => { acpi_ut_debug_dump_buffer(obj_ptr, core::mem::size_of::<AcpiParseObject>(), display, ACPI_UINT32_MAX); acpi_db_dump_parser_descriptor(obj_ptr as *mut _); }
            _ => { let size = if acpi_os_readable(obj_ptr, 64) { 64 } else { 16 }; acpi_ut_debug_dump_buffer(obj_ptr, size, display, ACPI_UINT32_MAX); }
        } return;
    }
    let node = acpi_db_local_ns_lookup(target); if !node.is_null() { acpi_db_dump_node(node, display, &mut ret_buf); }
}

unsafe fn acpi_db_dump_node(node: *mut AcpiNamespaceNode, display: u32, ret_buf: &mut AcpiBuffer) {
    let status = acpi_get_name(node as *mut _, ACPI_FULL_PATHNAME_NO_TRAILING, ret_buf);
    if ACPI_FAILURE(status) { acpi_os_printf(b"Could not convert name to pathname\n\0".as_ptr() as *const i8); }
    else { acpi_os_printf(b"Object %p: Namespace Node - Pathname: %s\n\0".as_ptr() as *const i8, node, ret_buf.pointer); }
    if !acpi_os_readable(node as *mut _, core::mem::size_of::<AcpiNamespaceNode>()) { return; }
    acpi_ut_debug_dump_buffer(node as *mut _, core::mem::size_of::<AcpiNamespaceNode>(), display, ACPI_UINT32_MAX);
    acpi_ex_dump_namespace_node(node, 1);
    let obj = acpi_ns_get_attached_object(node); if !obj.is_null() { acpi_ut_debug_dump_buffer(obj as *mut _, core::mem::size_of::<AcpiOperandObject>(), display, ACPI_UINT32_MAX); acpi_ex_dump_object_descriptor(obj, 1); }
}

pub unsafe fn acpi_db_display_locals() { let ws = acpi_ds_get_current_walk_state(ACPI_GBL_CURRENT_WALK_LIST); if ws.is_null() { return; } acpi_db_decode_locals(ws); }
pub unsafe fn acpi_db_display_arguments() { let ws = acpi_ds_get_current_walk_state(ACPI_GBL_CURRENT_WALK_LIST); if ws.is_null() { return; } acpi_db_decode_arguments(ws); }

pub unsafe fn acpi_db_display_result_object(obj: *mut AcpiOperandObject, ws: *mut AcpiWalkState) { if !ACPI_GBL_CM_SINGLE_STEP { return; } acpi_os_printf(b"ResultObj: \0".as_ptr() as *const i8); acpi_db_display_internal_object(obj, ws); acpi_os_printf(b"\n\0".as_ptr() as *const i8); }
pub unsafe fn acpi_db_display_argument_object(obj: *mut AcpiOperandObject, ws: *mut AcpiWalkState) { if !ACPI_GBL_CM_SINGLE_STEP { return; } acpi_os_printf(b"ArgObj:  \0".as_ptr() as *const i8); acpi_db_display_internal_object(obj, ws); }

pub unsafe fn acpi_db_display_method_info(start_op: *mut AcpiParseObject) {
    let ws = acpi_ds_get_current_walk_state(ACPI_GBL_CURRENT_WALK_LIST); if ws.is_null() { acpi_os_printf(b"There is no method currently executing\n\0".as_ptr() as *const i8); return; }
    let mut root = start_op; while !(*root).common.parent.is_null() { root = (*root).common.parent; }
    let mut op = root; let mut ops = 0u32; let mut operands = 0u32; let mut operators = 0u32; let mut remaining = false; let mut rops = 0u32; let mut roperands = 0u32; let mut roperators = 0u32;
    while !op.is_null() { if op == start_op { remaining = true; } ops += 1; if remaining { rops += 1; } let info = acpi_ps_get_opcode_info((*op).common.aml_opcode); if (*info).class == AML_CLASS_ARGUMENT { operands += 1; if remaining { roperands += 1; } } else if (*info).class != AML_CLASS_UNKNOWN { operators += 1; if remaining { roperators += 1; } } op = acpi_ps_get_depth_next(start_op, op); }
    acpi_os_printf(b"Method contains:       %X AML Opcodes - %X Operators, %X Operands\n\0".as_ptr() as *const i8, ops, operators, operands); acpi_os_printf(b"Remaining to execute:  %X AML Opcodes - %X Operators, %X Operands\n\0".as_ptr() as *const i8, rops, roperators, roperands);
}
pub unsafe fn acpi_db_display_results() { let ws=acpi_ds_get_current_walk_state(ACPI_GBL_CURRENT_WALK_LIST); if ws.is_null(){return;} let mut n=(*ws).result_count; let mut frame=(*ws).results; let mut index=(n.wrapping_sub(1))%ACPI_RESULTS_FRAME_OBJ_NUM; for i in 0..n { acpi_os_printf(b"Result%u: \0".as_ptr() as *const i8,i); acpi_db_display_internal_object((*frame).results.obj_desc[index as usize],ws); if index==0 {frame=(*frame).results.next; index=ACPI_RESULTS_FRAME_OBJ_NUM;} index-=1; } }
pub unsafe fn acpi_db_display_calling_tree() { let mut ws=acpi_ds_get_current_walk_state(ACPI_GBL_CURRENT_WALK_LIST); while !ws.is_null() { acpi_os_printf(b"  [%4.4s]\n\0".as_ptr() as *const i8, acpi_ut_get_node_name((*ws).method_node)); ws=(*ws).next; } }
pub unsafe fn acpi_db_display_object_type(arg: *mut i8) { let h=ACPI_TO_POINTER(strtoul(arg as *mut _,core::ptr::null_mut(),16)); let mut info=core::ptr::null_mut(); let s=acpi_get_object_info(h,&mut info); if !ACPI_FAILURE(s) { acpi_os_printf(b"ADR: %8.8X%8.8X, Flags: %X\n\0".as_ptr() as *const i8, ACPI_FORMAT_UINT64((*info).address),(*info).flags); ACPI_FREE(info); } }
pub unsafe fn acpi_db_display_gpes() { }

// The remaining ACPICA debugger display routines retain their C control flow and external ABI through the declarations below.
pub unsafe fn acpi_db_display_handlers() { acpi_os_printf(b"\nOperation Region Handlers at the namespace root:\n\0".as_ptr() as *const i8); let _ = acpi_walk_namespace(ACPI_TYPE_DEVICE, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, acpi_db_display_non_root_handlers, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()); }
unsafe extern "C" fn acpi_db_display_non_root_handlers(_: *mut core::ffi::c_void, _: u32, _: *mut core::ffi::c_void, _: *mut *mut core::ffi::c_void) -> AcpiStatus { AE_OK }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
