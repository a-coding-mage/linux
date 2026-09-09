// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/* AML Interpreter operand/object resolution */

// C dependencies supplied by the surrounding ACPICA translation.

unsafe fn acpi_ex_check_object_type(
    type_needed: acpi_object_type,
    this_type: acpi_object_type,
    object: *mut core::ffi::c_void,
) -> acpi_status {
    acpi_function_entry!();
    if type_needed == ACPI_TYPE_ANY { return AE_OK; }
    if type_needed == ACPI_TYPE_LOCAL_REFERENCE {
        if this_type == ACPI_TYPE_INTEGER
            && ((*((object as *mut union_acpi_operand_object))).common.flags & AOPOBJ_AML_CONSTANT) != 0
        { return AE_OK; }
    }
    if type_needed != this_type {
        acpi_error!(AE_INFO, "Needed type [%s], found [%s] %p",
            acpi_ut_get_type_name(type_needed), acpi_ut_get_type_name(this_type), object);
        return AE_AML_OPERAND_TYPE;
    }
    AE_OK
}

pub unsafe fn acpi_ex_resolve_operands(
    opcode: u16,
    mut stack_ptr: *mut *mut union_acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut obj_desc: *mut union_acpi_operand_object;
    let mut status = AE_OK;
    let mut object_type: u8;
    let mut arg_types: u32;
    let op_info: *const acpi_opcode_info;
    let mut this_arg_type: u32;
    let mut type_needed: acpi_object_type;
    let mut target_op: u16 = 0;
    let mut first_operand = true;

    acpi_function_trace_u32!(acpi_ex_resolve_operands, opcode);
    op_info = acpi_ps_get_opcode_info(opcode);
    if (*op_info).class == AML_CLASS_UNKNOWN { return AE_AML_BAD_OPCODE; }
    arg_types = (*op_info).runtime_args;
    if arg_types == ARGI_INVALID_OPCODE {
        acpi_error!(AE_INFO, "Unknown AML opcode 0x%X", opcode);
        return AE_AML_INTERNAL;
    }
    acpi_debug_print!(ACPI_DB_EXEC, "Opcode %X [%s] RequiredOperandTypes=%8.8X\n",
        opcode, (*op_info).name, arg_types);

    while GET_CURRENT_ARG_TYPE!(arg_types) != 0 {
        if !first_operand { stack_ptr = stack_ptr.offset(-1); }
        first_operand = false;
        if stack_ptr.is_null() || (*stack_ptr).is_null() {
            acpi_error!(AE_INFO, "Null stack entry at %p", stack_ptr);
            return AE_AML_INTERNAL;
        }
        obj_desc = *stack_ptr;
        match ACPI_GET_DESCRIPTOR_TYPE!(obj_desc) {
            ACPI_DESC_TYPE_NAMED => {
                object_type = (*(obj_desc as *mut acpi_namespace_node)).type_;
                if object_type == ACPI_TYPE_LOCAL_ALIAS {
                    obj_desc = acpi_ns_get_attached_object(obj_desc as *mut acpi_namespace_node);
                    *stack_ptr = obj_desc;
                    object_type = (*(obj_desc as *mut acpi_namespace_node)).type_;
                }
            }
            ACPI_DESC_TYPE_OPERAND => {
                object_type = (*obj_desc).common.type_;
                if !acpi_ut_valid_object_type(object_type) {
                    acpi_error!(AE_INFO, "Bad operand object type [0x%X]", object_type);
                    return AE_AML_OPERAND_TYPE;
                }
                if object_type == ACPI_TYPE_LOCAL_REFERENCE as u8 {
                    match (*obj_desc).reference.class_ {
                        ACPI_REFCLASS_DEBUG => { target_op = AML_DEBUG_OP; }
                        ACPI_REFCLASS_ARG | ACPI_REFCLASS_LOCAL | ACPI_REFCLASS_INDEX |
                        ACPI_REFCLASS_REFOF | ACPI_REFCLASS_TABLE | ACPI_REFCLASS_NAME => {
                            acpi_debug_print!(ACPI_DB_EXEC, "Operand is a Reference, Class [%s] %2.2X\n",
                                acpi_ut_get_reference_name(obj_desc), (*obj_desc).reference.class_);
                        }
                        _ => {
                            acpi_error!(AE_INFO, "Unknown Reference Class 0x%2.2X in %p",
                                (*obj_desc).reference.class_, obj_desc);
                            return AE_AML_OPERAND_TYPE;
                        }
                    }
                }
            }
            _ => {
                acpi_error!(AE_INFO, "Invalid descriptor %p [%s]", obj_desc,
                    acpi_ut_get_descriptor_name(obj_desc));
                return AE_AML_OPERAND_TYPE;
            }
        }
        this_arg_type = GET_CURRENT_ARG_TYPE!(arg_types);
        INCREMENT_ARG_LIST!(arg_types);
        match this_arg_type {
            ARGI_REF_OR_STRING => {
                if ACPI_GET_DESCRIPTOR_TYPE!(obj_desc) == ACPI_DESC_TYPE_OPERAND &&
                    (*obj_desc).common.type_ == ACPI_TYPE_STRING as u8 { continue; }
                // fall through to reference handling
                if ACPI_GET_DESCRIPTOR_TYPE!(obj_desc) == ACPI_DESC_TYPE_NAMED { continue; }
                status = acpi_ex_check_object_type(ACPI_TYPE_LOCAL_REFERENCE,
                    object_type as acpi_object_type, obj_desc as *mut _);
                if ACPI_FAILURE!(status) { return status; }
                continue;
            }
            ARGI_REFERENCE | ARGI_INTEGER_REF | ARGI_OBJECT_REF | ARGI_DEVICE_REF |
            ARGI_TARGETREF | ARGI_FIXED_TARGET | ARGI_SIMPLE_TARGET | ARGI_STORE_TARGET => {
                if ACPI_GET_DESCRIPTOR_TYPE!(obj_desc) == ACPI_DESC_TYPE_NAMED { continue; }
                status = acpi_ex_check_object_type(ACPI_TYPE_LOCAL_REFERENCE,
                    object_type as acpi_object_type, obj_desc as *mut _);
                if ACPI_FAILURE!(status) { return status; }
                continue;
            }
            ARGI_DATAREFOBJ if opcode == AML_STORE_OP && (*stack_ptr).common.type_ == ACPI_TYPE_LOCAL_REFERENCE as u8 &&
                (*stack_ptr).reference.class_ == ACPI_REFCLASS_INDEX => { continue; }
            _ => {}
        }
        status = acpi_ex_resolve_to_value(&mut stack_ptr, walk_state);
        if ACPI_FAILURE!(status) { return status; }
        obj_desc = *stack_ptr;
        match this_arg_type {
            ARGI_MUTEX => type_needed = ACPI_TYPE_MUTEX,
            ARGI_EVENT => type_needed = ACPI_TYPE_EVENT,
            ARGI_PACKAGE => type_needed = ACPI_TYPE_PACKAGE,
            ARGI_ANYTYPE => type_needed = ACPI_TYPE_ANY,
            ARGI_DDBHANDLE => type_needed = ACPI_TYPE_LOCAL_REFERENCE,
            ARGI_INTEGER => { status = acpi_ex_convert_to_integer(obj_desc, &mut stack_ptr, ACPI_IMPLICIT_CONVERSION); if ACPI_FAILURE!(status) { if status == AE_TYPE { return AE_AML_OPERAND_TYPE; } return status; } if obj_desc != *stack_ptr { acpi_ut_remove_reference(obj_desc); } continue; }
            ARGI_BUFFER => { status = acpi_ex_convert_to_buffer(obj_desc, &mut stack_ptr); if ACPI_FAILURE!(status) { if status == AE_TYPE { return AE_AML_OPERAND_TYPE; } return status; } if obj_desc != *stack_ptr { acpi_ut_remove_reference(obj_desc); } continue; }
            ARGI_STRING => { status = acpi_ex_convert_to_string(obj_desc, &mut stack_ptr, ACPI_IMPLICIT_CONVERT_HEX); if ACPI_FAILURE!(status) { if status == AE_TYPE { return AE_AML_OPERAND_TYPE; } return status; } if obj_desc != *stack_ptr { acpi_ut_remove_reference(obj_desc); } continue; }
            ARGI_COMPUTEDATA => match (*obj_desc).common.type_ { ACPI_TYPE_INTEGER | ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => continue, _ => return AE_AML_OPERAND_TYPE },
            ARGI_BUFFER_OR_STRING => match (*obj_desc).common.type_ { ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => continue, ACPI_TYPE_INTEGER => { status = acpi_ex_convert_to_buffer(obj_desc, &mut stack_ptr); if ACPI_FAILURE!(status) { return status; } if obj_desc != *stack_ptr { acpi_ut_remove_reference(obj_desc); } continue }, _ => return AE_AML_OPERAND_TYPE },
            ARGI_DATAOBJECT => match (*obj_desc).common.type_ { ACPI_TYPE_PACKAGE | ACPI_TYPE_STRING | ACPI_TYPE_BUFFER | ACPI_TYPE_LOCAL_REFERENCE => continue, _ => return AE_AML_OPERAND_TYPE },
            ARGI_COMPLEXOBJ => match (*obj_desc).common.type_ { ACPI_TYPE_PACKAGE | ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => continue, _ => return AE_AML_OPERAND_TYPE },
            ARGI_REGION_OR_BUFFER => match (*obj_desc).common.type_ { ACPI_TYPE_BUFFER | ACPI_TYPE_REGION => continue, _ => return AE_AML_OPERAND_TYPE },
            ARGI_DATAREFOBJ => match (*obj_desc).common.type_ { ACPI_TYPE_INTEGER | ACPI_TYPE_PACKAGE | ACPI_TYPE_STRING | ACPI_TYPE_BUFFER | ACPI_TYPE_BUFFER_FIELD | ACPI_TYPE_LOCAL_REFERENCE | ACPI_TYPE_LOCAL_REGION_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD | ACPI_TYPE_LOCAL_INDEX_FIELD | ACPI_TYPE_DDB_HANDLE => continue, _ if acpi_gbl_enable_interpreter_slack || target_op == AML_DEBUG_OP => continue, _ => return AE_AML_OPERAND_TYPE },
            _ => return AE_BAD_PARAMETER,
        }
        status = acpi_ex_check_object_type(type_needed, (*stack_ptr).common.type_ as acpi_object_type, *stack_ptr as *mut _);
        if ACPI_FAILURE!(status) { return status; }
    }
    acpi_dump_operands!((*walk_state).operands, acpi_ps_get_opcode_name(opcode), (*walk_state).num_operands);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
