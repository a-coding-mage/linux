// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/* Module Name: dbobject - ACPI object decode and display */

/* Dependencies are supplied by the surrounding ACPI implementation. */

unsafe extern "C" {
    fn acpi_os_printf(format: *const i8, ...);
    fn acpi_ut_get_descriptor_name(obj: *mut acpi_operand_object) -> *const i8;
    fn acpi_ut_get_object_type_name(obj: *mut acpi_operand_object) -> *const i8;
    fn acpi_ut_get_node_name(node: *mut acpi_namespace_node) -> *const i8;
    fn acpi_ns_get_attached_object(node: *mut acpi_namespace_node) -> *mut acpi_operand_object;
    fn acpi_ut_get_reference_name(obj: *mut acpi_operand_object) -> *const i8;
}

unsafe extern "C" {
    static mut acpi_gbl_root_node: *mut acpi_namespace_node;
}

#[no_mangle]
pub unsafe extern "C" fn acpi_db_dump_method_info(
    status: acpi_status,
    walk_state: *mut acpi_walk_state,
) {
    let node = (*walk_state).method_node;
    if node == acpi_gbl_root_node { return; }
    if ACPI_CNTL_EXCEPTION(status) { return; }
    if !(*walk_state).deferred_node.is_null() {
        acpi_os_printf(b"Executing subtree for Buffer/Package/Region\0".as_ptr() as *const i8);
        return;
    }
    let thread = (*walk_state).thread;
    if thread.is_null() { return; }
    acpi_os_printf(b"\n\0".as_ptr() as *const i8);
    acpi_db_decode_locals(walk_state);
    acpi_os_printf(b"\n\0".as_ptr() as *const i8);
    acpi_db_decode_arguments(walk_state);
    acpi_os_printf(b"\n\0".as_ptr() as *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn acpi_db_decode_internal_object(obj_desc: *mut acpi_operand_object) {
    if obj_desc.is_null() {
        acpi_os_printf(b" Uninitialized\0".as_ptr() as *const i8); return;
    }
    if ACPI_GET_DESCRIPTOR_TYPE(obj_desc) != ACPI_DESC_TYPE_OPERAND {
        acpi_os_printf(b" %p [%s]\0".as_ptr() as *const i8, obj_desc,
                       acpi_ut_get_descriptor_name(obj_desc)); return;
    }
    acpi_os_printf(b" %s\0".as_ptr() as *const i8, acpi_ut_get_object_type_name(obj_desc));
    match (*obj_desc).common.type_ {
        ACPI_TYPE_INTEGER => acpi_os_printf(b" %8.8X%8.8X\0".as_ptr() as *const i8,
                                             ACPI_FORMAT_UINT64((*obj_desc).integer.value)),
        ACPI_TYPE_STRING => {
            acpi_os_printf(b"(%u) \"%.60s\0".as_ptr() as *const i8,
                           (*obj_desc).string.length, (*obj_desc).string.pointer);
            if (*obj_desc).string.length > 60 { acpi_os_printf(b"...\0".as_ptr() as *const i8); }
            else { acpi_os_printf(b"\"\0".as_ptr() as *const i8); }
        }
        ACPI_TYPE_BUFFER => {
            acpi_os_printf(b"(%u)\0".as_ptr() as *const i8, (*obj_desc).buffer.length);
            let mut i: u32 = 0;
            while i < 8 && i < (*obj_desc).buffer.length {
                acpi_os_printf(b" %2.2X\0".as_ptr() as *const i8, (*obj_desc).buffer.pointer.add(i as usize).read());
                i += 1;
            }
        }
        _ => acpi_os_printf(b" %p\0".as_ptr() as *const i8, obj_desc),
    }
}

unsafe fn acpi_db_decode_node(node: *mut acpi_namespace_node) {
    acpi_os_printf(b"<Node>          Name %4.4s\0".as_ptr() as *const i8, acpi_ut_get_node_name(node));
    if (*node).flags & ANOBJ_METHOD_ARG != 0 { acpi_os_printf(b" [Method Arg]\0".as_ptr() as *const i8); }
    if (*node).flags & ANOBJ_METHOD_LOCAL != 0 { acpi_os_printf(b" [Method Local]\0".as_ptr() as *const i8); }
    match (*node).type_ {
        ACPI_TYPE_DEVICE => acpi_os_printf(b" Device\0".as_ptr() as *const i8),
        ACPI_TYPE_THERMAL => acpi_os_printf(b" Thermal Zone\0".as_ptr() as *const i8),
        _ => acpi_db_decode_internal_object(acpi_ns_get_attached_object(node)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn acpi_db_display_internal_object(
    mut obj_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state,
) {
    acpi_os_printf(b"%p \0".as_ptr() as *const i8, obj_desc);
    if obj_desc.is_null() { acpi_os_printf(b"<Null Object>\n\0".as_ptr() as *const i8); return; }
    match ACPI_GET_DESCRIPTOR_TYPE(obj_desc) {
        ACPI_DESC_TYPE_PARSER => acpi_os_printf(b"<Parser> \0".as_ptr() as *const i8),
        ACPI_DESC_TYPE_NAMED => acpi_db_decode_node(obj_desc as *mut acpi_namespace_node),
        ACPI_DESC_TYPE_OPERAND => {
            let typ = (*obj_desc).common.type_;
            if typ > ACPI_TYPE_LOCAL_MAX { acpi_os_printf(b" Type %X [Invalid Type]\0".as_ptr() as *const i8, typ as u32); return; }
            if typ == ACPI_TYPE_LOCAL_REFERENCE {
                acpi_os_printf(b"[%s] \0".as_ptr() as *const i8, acpi_ut_get_reference_name(obj_desc));
                match (*obj_desc).reference.class {
                    ACPI_REFCLASS_LOCAL => { acpi_os_printf(b"%X \0".as_ptr() as *const i8, (*obj_desc).reference.value); if !walk_state.is_null() { obj_desc = (*walk_state).local_variables[(*obj_desc).reference.value as usize].object; acpi_os_printf(b"%p\0".as_ptr() as *const i8, obj_desc); acpi_db_decode_internal_object(obj_desc); } }
                    ACPI_REFCLASS_ARG => { acpi_os_printf(b"%X \0".as_ptr() as *const i8, (*obj_desc).reference.value); if !walk_state.is_null() { obj_desc = (*walk_state).arguments[(*obj_desc).reference.value as usize].object; acpi_os_printf(b"%p\0".as_ptr() as *const i8, obj_desc); acpi_db_decode_internal_object(obj_desc); } }
                    ACPI_REFCLASS_INDEX => match (*obj_desc).reference.target_type { ACPI_TYPE_BUFFER_FIELD => { acpi_os_printf(b"%p\0".as_ptr() as *const i8, (*obj_desc).reference.object); acpi_db_decode_internal_object((*obj_desc).reference.object); }, ACPI_TYPE_PACKAGE => { acpi_os_printf(b"%p\0".as_ptr() as *const i8, (*obj_desc).reference.where_); if (*obj_desc).reference.where_.is_null() { acpi_os_printf(b" Uninitialized WHERE pointer\0".as_ptr() as *const i8); } else { acpi_db_decode_internal_object(*(*obj_desc).reference.where_); } }, _ => acpi_os_printf(b"Unknown index target type\0".as_ptr() as *const i8) },
                    ACPI_REFCLASS_REFOF => { if (*obj_desc).reference.object.is_null() { acpi_os_printf(b"Uninitialized reference subobject pointer\0".as_ptr() as *const i8); } else { match ACPI_GET_DESCRIPTOR_TYPE((*obj_desc).reference.object) { ACPI_DESC_TYPE_NAMED => acpi_db_decode_node((*obj_desc).reference.object as *mut acpi_namespace_node), ACPI_DESC_TYPE_OPERAND => acpi_db_decode_internal_object((*obj_desc).reference.object), _ => {} } } }
                    ACPI_REFCLASS_NAME => acpi_db_decode_node((*obj_desc).reference.node),
                    ACPI_REFCLASS_DEBUG | ACPI_REFCLASS_TABLE => acpi_os_printf(b"\n\0".as_ptr() as *const i8),
                    _ => acpi_os_printf(b"%2.2X\n\0".as_ptr() as *const i8, (*obj_desc).reference.class),
                }
            } else { acpi_os_printf(b"<Obj>          \0".as_ptr() as *const i8); acpi_db_decode_internal_object(obj_desc); }
        }
        _ => acpi_os_printf(b"<Not a valid ACPI Object Descriptor> [%s]\0".as_ptr() as *const i8, acpi_ut_get_descriptor_name(obj_desc)),
    }
    acpi_os_printf(b"\n\0".as_ptr() as *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn acpi_db_decode_locals(walk_state: *mut acpi_walk_state) {
    let node = (*walk_state).method_node;
    if node == acpi_gbl_root_node { return; }
    if node.is_null() { acpi_os_printf(b"No method node (Executing subtree for buffer or opregion)\n\0".as_ptr() as *const i8); return; }
    if (*node).type_ != ACPI_TYPE_METHOD { acpi_os_printf(b"Executing subtree for Buffer/Package/Region\n\0".as_ptr() as *const i8); return; }
    let mut display = false;
    for i in 0..ACPI_METHOD_NUM_LOCALS { if !(*walk_state).local_variables[i as usize].object.is_null() { display = true; break; } }
    if display { acpi_os_printf(b"\nInitialized Local Variables for Method [%4.4s]:\n\0".as_ptr() as *const i8, acpi_ut_get_node_name(node)); for i in 0..ACPI_METHOD_NUM_LOCALS { let obj = (*walk_state).local_variables[i as usize].object; if !obj.is_null() { acpi_os_printf(b"  Local%X: \0".as_ptr() as *const i8, i); acpi_db_display_internal_object(obj, walk_state); } } }
    else { acpi_os_printf(b"No Local Variables are initialized for Method [%4.4s]\n\0".as_ptr() as *const i8, acpi_ut_get_node_name(node)); }
}

#[no_mangle]
pub unsafe extern "C" fn acpi_db_decode_arguments(walk_state: *mut acpi_walk_state) {
    let node = (*walk_state).method_node;
    if node == acpi_gbl_root_node { return; }
    if node.is_null() { acpi_os_printf(b"No method node (Executing subtree for buffer or opregion)\n\0".as_ptr() as *const i8); return; }
    if (*node).type_ != ACPI_TYPE_METHOD { acpi_os_printf(b"Executing subtree for Buffer/Package/Region\n\0".as_ptr() as *const i8); return; }
    let mut display = false;
    for i in 0..ACPI_METHOD_NUM_ARGS { if !(*walk_state).arguments[i as usize].object.is_null() { display = true; break; } }
    if display { acpi_os_printf(b"Initialized Arguments for Method [%4.4s]:  (%X arguments defined for method invocation)\n\0".as_ptr() as *const i8, acpi_ut_get_node_name(node), (*node).object.method.param_count); for i in 0..ACPI_METHOD_NUM_ARGS { let obj = (*walk_state).arguments[i as usize].object; if !obj.is_null() { acpi_os_printf(b"  Arg%u:   \0".as_ptr() as *const i8, i); acpi_db_display_internal_object(obj, walk_state); } } }
    else { acpi_os_printf(b"No Arguments are initialized for method [%4.4s]\n\0".as_ptr() as *const i8, acpi_ut_get_node_name(node)); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
