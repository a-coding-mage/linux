// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: nseval - Object evaluation, includes control method execution
 */

// Dependencies are supplied by the surrounding ACPICA translation unit.

pub type AcpiStatus = u32;
pub type AcpiObjectType = u8;
pub type AcpiHandle = *mut core::ffi::c_void;

pub const AE_OK: AcpiStatus = 0;
pub const AE_BAD_PARAMETER: AcpiStatus = 1;
pub const AE_NO_MEMORY: AcpiStatus = 2;
pub const AE_TYPE: AcpiStatus = 3;
pub const AE_NULL_OBJECT: AcpiStatus = 4;
pub const AE_CTRL_RETURN_VALUE: AcpiStatus = 0x1001;
pub const ACPI_TYPE_ANY: AcpiObjectType = 0;
pub const ACPI_TYPE_DEVICE: AcpiObjectType = 6;
pub const ACPI_TYPE_EVENT: AcpiObjectType = 7;
pub const ACPI_TYPE_METHOD: AcpiObjectType = 8;
pub const ACPI_TYPE_MUTEX: AcpiObjectType = 9;
pub const ACPI_TYPE_REGION: AcpiObjectType = 10;
pub const ACPI_TYPE_THERMAL: AcpiObjectType = 14;
pub const ACPI_TYPE_LOCAL_SCOPE: AcpiObjectType = 0x10;
pub const ACPI_TYPE_LOCAL_METHOD_ALIAS: AcpiObjectType = 0x0e;
pub const ACPI_METHOD_NUM_ARGS: u32 = 7;
pub const ACPI_IGNORE_RETURN_VALUE: u32 = 1 << 0;
pub const ACPI_NS_NO_UPSEARCH: u32 = 1 << 0;

#[repr(C)]
pub struct AcpiNamespaceNode {
    pub object: *mut core::ffi::c_void,
    pub flags: u8,
    pub name: AcpiName,
    pub type_: AcpiObjectType,
}

#[repr(C)]
pub struct AcpiName {
    pub ascii: *const core::ffi::c_char,
}

#[repr(C)]
pub struct AcpiEvaluateInfo {
    pub prefix_node: *mut AcpiNamespaceNode,
    pub relative_pathname: *const core::ffi::c_char,
    pub node: *mut AcpiNamespaceNode,
    pub parameters: *mut *mut AcpiOperandObject,
    pub return_object: *mut AcpiOperandObject,
    pub flags: u32,
    pub node_flags: u8,
    pub obj_desc: *mut AcpiOperandObject,
    pub predefined: *mut core::ffi::c_void,
    pub full_pathname: *mut core::ffi::c_char,
    pub param_count: u32,
}

#[repr(C)]
pub struct AcpiOperandObject {
    pub method: AcpiMethodObject,
}

#[repr(C)]
pub struct AcpiMethodObject {
    pub aml_start: *const u8,
    pub aml_length: u32,
}

extern "C" {
    fn acpi_ns_get_node(prefix: *mut AcpiNamespaceNode, path: *const core::ffi::c_char, flags: u32, node: *mut *mut AcpiNamespaceNode) -> AcpiStatus;
    fn acpi_ns_get_type(node: *mut AcpiNamespaceNode) -> AcpiObjectType;
    fn acpi_ns_get_attached_object(node: *mut AcpiNamespaceNode) -> *mut AcpiOperandObject;
    fn acpi_ut_match_predefined_method(name: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn acpi_ns_get_normalized_pathname(node: *mut AcpiNamespaceNode, flag: bool) -> *mut core::ffi::c_char;
    fn acpi_ut_get_type_name(type_: AcpiObjectType) -> *const core::ffi::c_char;
    fn acpi_ns_check_acpi_compliance(path: *mut core::ffi::c_char, node: *mut AcpiNamespaceNode, predefined: *mut core::ffi::c_void);
    fn acpi_ns_check_argument_count(path: *mut core::ffi::c_char, node: *mut AcpiNamespaceNode, count: u32, predefined: *mut core::ffi::c_void);
    fn acpi_ns_check_argument_types(info: *mut AcpiEvaluateInfo);
    fn acpi_ex_enter_interpreter();
    fn acpi_ex_exit_interpreter();
    fn acpi_ps_execute_method(info: *mut AcpiEvaluateInfo) -> AcpiStatus;
    fn acpi_ex_resolve_node_to_value(node: *mut *mut AcpiNamespaceNode, walk_state: *mut core::ffi::c_void) -> AcpiStatus;
    fn acpi_ns_check_return_value(node: *mut AcpiNamespaceNode, info: *mut AcpiEvaluateInfo, count: u32, status: AcpiStatus, ret: *mut *mut AcpiOperandObject);
    fn acpi_ut_remove_reference(object: *mut AcpiOperandObject);
    fn acpi_ut_get_object_type_name(object: *mut AcpiOperandObject) -> *const core::ffi::c_char;
    fn acpi_free(ptr: *mut core::ffi::c_void);
}

#[inline]
unsafe fn acpi_failure(status: AcpiStatus) -> bool { status != AE_OK && status != AE_CTRL_RETURN_VALUE }

/// Execute a control method or return the current value of an ACPI namespace object.
#[no_mangle]
pub unsafe extern "C" fn acpi_ns_evaluate(info: *mut AcpiEvaluateInfo) -> AcpiStatus {
    if info.is_null() { return AE_BAD_PARAMETER; }
    let info = &mut *info;
    let mut status: AcpiStatus;

    if info.node.is_null() {
        status = acpi_ns_get_node(info.prefix_node, info.relative_pathname, ACPI_NS_NO_UPSEARCH, &mut info.node);
        if acpi_failure(status) { return status; }
    }
    if acpi_ns_get_type(info.node) == ACPI_TYPE_LOCAL_METHOD_ALIAS {
        info.node = (*info.node).object as *mut AcpiNamespaceNode;
    }
    info.return_object = core::ptr::null_mut();
    info.node_flags = (*info.node).flags;
    info.obj_desc = acpi_ns_get_attached_object(info.node);
    info.predefined = acpi_ut_match_predefined_method((*info.node).name.ascii);
    info.full_pathname = acpi_ns_get_normalized_pathname(info.node, true);
    if info.full_pathname.is_null() { return AE_NO_MEMORY; }

    info.param_count = 0;
    if !info.parameters.is_null() {
        while !(*info.parameters.add(info.param_count as usize)).is_null() { info.param_count += 1; }
        if info.param_count > ACPI_METHOD_NUM_ARGS { info.param_count = ACPI_METHOD_NUM_ARGS; }
    }
    acpi_ns_check_acpi_compliance(info.full_pathname, info.node, info.predefined);
    acpi_ns_check_argument_count(info.full_pathname, info.node, info.param_count, info.predefined);
    acpi_ns_check_argument_types(info);

    match acpi_ns_get_type(info.node) {
        ACPI_TYPE_ANY | ACPI_TYPE_DEVICE | ACPI_TYPE_EVENT | ACPI_TYPE_MUTEX |
        ACPI_TYPE_REGION | ACPI_TYPE_THERMAL | ACPI_TYPE_LOCAL_SCOPE => {
            status = AE_TYPE;
        }
        ACPI_TYPE_METHOD => {
            if info.obj_desc.is_null() { status = AE_NULL_OBJECT; }
            else { acpi_ex_enter_interpreter(); status = acpi_ps_execute_method(info); acpi_ex_exit_interpreter(); }
        }
        _ => {
            acpi_ex_enter_interpreter();
            info.return_object = info.node as *mut AcpiOperandObject;
            status = acpi_ex_resolve_node_to_value(&mut info.return_object.cast::<AcpiNamespaceNode>(), core::ptr::null_mut());
            acpi_ex_exit_interpreter();
            if acpi_failure(status) { info.return_object = core::ptr::null_mut(); } else { status = AE_CTRL_RETURN_VALUE; }
        }
    }
    acpi_ns_check_return_value(info.node, info, info.param_count, status, &mut info.return_object);
    if status == AE_CTRL_RETURN_VALUE {
        if info.flags & ACPI_IGNORE_RETURN_VALUE != 0 { acpi_ut_remove_reference(info.return_object); info.return_object = core::ptr::null_mut(); }
        status = AE_OK;
    } else if acpi_failure(status) && !info.return_object.is_null() {
        acpi_ut_remove_reference(info.return_object); info.return_object = core::ptr::null_mut();
    }
    acpi_free(info.full_pathname as *mut core::ffi::c_void);
    info.full_pathname = core::ptr::null_mut();
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
