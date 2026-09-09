// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Internal to external object translation utilities.

// C dependencies are supplied by the surrounding ACPI translation unit.

unsafe fn acpi_ut_copy_isimple_to_esimple(
    internal_object: *mut acpi_operand_object,
    external_object: *mut acpi_object,
    data_space: *mut u8,
    buffer_space_used: *mut acpi_size,
) -> acpi_status {
    *buffer_space_used = 0;
    if internal_object.is_null() { return AE_OK; }
    core::ptr::write_bytes(external_object as *mut u8, 0, core::mem::size_of::<acpi_object>());
    (*external_object).type_ = (*internal_object).common.type_;
    match (*internal_object).common.type_ {
        ACPI_TYPE_STRING => {
            (*external_object).string.pointer = data_space as *mut i8;
            (*external_object).string.length = (*internal_object).string.length;
            *buffer_space_used = acpi_round_up_to_native_word((*internal_object).string.length as acpi_size + 1);
            core::ptr::copy_nonoverlapping((*internal_object).string.pointer as *const u8, data_space, (*internal_object).string.length as usize + 1);
        }
        ACPI_TYPE_BUFFER => {
            (*external_object).buffer.pointer = data_space;
            (*external_object).buffer.length = (*internal_object).buffer.length;
            *buffer_space_used = acpi_round_up_to_native_word((*internal_object).string.length as acpi_size);
            core::ptr::copy_nonoverlapping((*internal_object).buffer.pointer, data_space, (*internal_object).buffer.length as usize);
        }
        ACPI_TYPE_INTEGER => (*external_object).integer.value = (*internal_object).integer.value,
        ACPI_TYPE_LOCAL_REFERENCE => match (*internal_object).reference.class_ {
            ACPI_REFCLASS_NAME => {
                (*external_object).reference.handle = (*internal_object).reference.node;
                (*external_object).reference.actual_type = acpi_ns_get_type((*internal_object).reference.node);
            }
            _ => return AE_TYPE,
        },
        ACPI_TYPE_PROCESSOR => {
            (*external_object).processor.proc_id = (*internal_object).processor.proc_id;
            (*external_object).processor.pblk_address = (*internal_object).processor.address;
            (*external_object).processor.pblk_length = (*internal_object).processor.length;
        }
        ACPI_TYPE_POWER => {
            (*external_object).power_resource.system_level = (*internal_object).power_resource.system_level;
            (*external_object).power_resource.resource_order = (*internal_object).power_resource.resource_order;
        }
        _ => return AE_SUPPORT,
    }
    AE_OK
}

unsafe fn acpi_ut_copy_ielement_to_eelement(
    object_type: u8, source_object: *mut acpi_operand_object,
    state: *mut acpi_generic_state, context: *mut core::ffi::c_void,
) -> acpi_status {
    let info = context as *mut acpi_pkg_info;
    let target = (*state).pkg.dest_object as *mut acpi_object;
    let target = &mut (*target).package.elements[(*state).pkg.index as usize] as *mut acpi_object;
    let mut object_space: acpi_size = 0;
    match object_type {
        ACPI_COPY_TYPE_SIMPLE => {
            let status = acpi_ut_copy_isimple_to_esimple(source_object, target, (*info).free_space, &mut object_space);
            if ACPI_FAILURE(status) { return status; }
        }
        ACPI_COPY_TYPE_PACKAGE => {
            (*target).type_ = ACPI_TYPE_PACKAGE;
            (*target).package.count = (*source_object).package.count;
            (*target).package.elements = (*info).free_space as *mut acpi_object;
            (*state).pkg.this_target_obj = target as *mut _;
            object_space = acpi_round_up_to_native_word((*target).package.count as acpi_size * core::mem::size_of::<acpi_object>() as acpi_size);
        }
        _ => return AE_BAD_PARAMETER,
    }
    (*info).free_space = (*info).free_space.add(object_space as usize);
    (*info).length += object_space;
    AE_OK
}

unsafe fn acpi_ut_copy_ipackage_to_epackage(internal_object: *mut acpi_operand_object, buffer: *mut u8, space_used: *mut acpi_size) -> acpi_status {
    let external_object = buffer as *mut acpi_object;
    let mut info = acpi_pkg_info { length: acpi_round_up_to_native_word(core::mem::size_of::<acpi_object>() as acpi_size), free_space: buffer.add(acpi_round_up_to_native_word(core::mem::size_of::<acpi_object>() as acpi_size) as usize), object_space: 0, num_packages: 1 };
    (*external_object).type_ = (*internal_object).common.type_;
    (*external_object).package.count = (*internal_object).package.count;
    (*external_object).package.elements = info.free_space as *mut acpi_object;
    info.length += (*external_object).package.count as acpi_size * acpi_round_up_to_native_word(core::mem::size_of::<acpi_object>() as acpi_size);
    info.free_space = info.free_space.add(((*external_object).package.count as acpi_size * acpi_round_up_to_native_word(core::mem::size_of::<acpi_object>() as acpi_size)) as usize);
    let status = acpi_ut_walk_package_tree(internal_object, external_object as *mut _, acpi_ut_copy_ielement_to_eelement, &mut info as *mut _ as *mut _);
    *space_used = info.length;
    status
}

pub unsafe fn acpi_ut_copy_iobject_to_eobject(internal_object: *mut acpi_operand_object, ret_buffer: *mut acpi_buffer) -> acpi_status {
    if (*internal_object).common.type_ == ACPI_TYPE_PACKAGE {
        acpi_ut_copy_ipackage_to_epackage(internal_object, (*ret_buffer).pointer as *mut u8, &mut (*ret_buffer).length)
    } else {
        let status = acpi_ut_copy_isimple_to_esimple(internal_object, (*ret_buffer).pointer as *mut acpi_object, ((*ret_buffer).pointer as *mut u8).add(acpi_round_up_to_native_word(core::mem::size_of::<acpi_object>() as acpi_size) as usize), &mut (*ret_buffer).length);
        (*ret_buffer).length += core::mem::size_of::<acpi_object>() as acpi_size;
        status
    }
}

unsafe fn acpi_ut_copy_esimple_to_isimple(external_object: *mut acpi_object, ret_internal_object: *mut *mut acpi_operand_object) -> acpi_status {
    let typ = (*external_object).type_;
    if typ == ACPI_TYPE_ANY { *ret_internal_object = core::ptr::null_mut(); return AE_OK; }
    if typ != ACPI_TYPE_STRING && typ != ACPI_TYPE_BUFFER && typ != ACPI_TYPE_INTEGER && typ != ACPI_TYPE_LOCAL_REFERENCE { return AE_SUPPORT; }
    let internal = acpi_ut_create_internal_object(typ);
    if internal.is_null() { return AE_NO_MEMORY; }
    match typ {
        ACPI_TYPE_STRING => { (*internal).string.pointer = acpi_allocate_zeroed((*external_object).string.length as acpi_size + 1) as *mut i8; if (*internal).string.pointer.is_null() { acpi_ut_remove_reference(internal); return AE_NO_MEMORY; } core::ptr::copy_nonoverlapping((*external_object).string.pointer as *const u8, (*internal).string.pointer as *mut u8, (*external_object).string.length as usize); (*internal).string.length = (*external_object).string.length; }
        ACPI_TYPE_BUFFER => { (*internal).buffer.pointer = acpi_allocate_zeroed((*external_object).buffer.length as acpi_size) as *mut u8; if (*internal).buffer.pointer.is_null() { acpi_ut_remove_reference(internal); return AE_NO_MEMORY; } core::ptr::copy_nonoverlapping((*external_object).buffer.pointer, (*internal).buffer.pointer, (*external_object).buffer.length as usize); (*internal).buffer.length = (*external_object).buffer.length; (*internal).buffer.flags |= AOPOBJ_DATA_VALID; }
        ACPI_TYPE_INTEGER => (*internal).integer.value = (*external_object).integer.value,
        ACPI_TYPE_LOCAL_REFERENCE => { (*internal).reference.class_ = ACPI_REFCLASS_REFOF; (*internal).reference.object = (*external_object).reference.handle; }
        _ => {}
    }
    *ret_internal_object = internal; AE_OK
}

unsafe fn acpi_ut_copy_epackage_to_ipackage(external_object: *mut acpi_object, internal_object: *mut *mut acpi_operand_object) -> acpi_status {
    let package = acpi_ut_create_package_object((*external_object).package.count); if package.is_null() { return AE_NO_MEMORY; }
    for i in 0..(*external_object).package.count { let status = acpi_ut_copy_eobject_to_iobject(&mut (*external_object).package.elements[i as usize], &mut (*package).package.elements[i as usize]); if ACPI_FAILURE(status) { (*package).package.count = i; (*package).package.elements[i as usize] = core::ptr::null_mut(); acpi_ut_remove_reference(package); return status; } }
    (*package).package.flags |= AOPOBJ_DATA_VALID; *internal_object = package; AE_OK
}

pub unsafe fn acpi_ut_copy_eobject_to_iobject(external_object: *mut acpi_object, internal_object: *mut *mut acpi_operand_object) -> acpi_status {
    if (*external_object).type_ == ACPI_TYPE_PACKAGE { acpi_ut_copy_epackage_to_ipackage(external_object, internal_object) } else { acpi_ut_copy_esimple_to_isimple(external_object, internal_object) }
}

unsafe fn acpi_ut_copy_simple_object(source_desc: *mut acpi_operand_object, dest_desc: *mut acpi_operand_object) -> acpi_status {
    let reference_count = (*dest_desc).common.reference_count; let next_object = (*dest_desc).common.next_object;
    let copy_size = if acpi_get_descriptor_type(source_desc) == ACPI_DESC_TYPE_NAMED { core::mem::size_of::<acpi_namespace_node>() } else { core::mem::size_of::<acpi_operand_object>() };
    core::ptr::copy_nonoverlapping(source_desc as *const u8, dest_desc as *mut u8, copy_size);
    (*dest_desc).common.reference_count = reference_count; (*dest_desc).common.next_object = next_object; (*dest_desc).common.flags &= !AOPOBJ_STATIC_POINTER;
    match (*dest_desc).common.type_ {
        ACPI_TYPE_BUFFER if !(*source_desc).buffer.pointer.is_null() && (*source_desc).buffer.length != 0 => { (*dest_desc).buffer.pointer = acpi_allocate((*source_desc).buffer.length as acpi_size) as *mut u8; if (*dest_desc).buffer.pointer.is_null() { return AE_NO_MEMORY; } core::ptr::copy_nonoverlapping((*source_desc).buffer.pointer, (*dest_desc).buffer.pointer, (*source_desc).buffer.length as usize); }
        ACPI_TYPE_STRING if !(*source_desc).string.pointer.is_null() => { (*dest_desc).string.pointer = acpi_allocate((*source_desc).string.length as acpi_size + 1) as *mut i8; if (*dest_desc).string.pointer.is_null() { return AE_NO_MEMORY; } core::ptr::copy_nonoverlapping((*source_desc).string.pointer as *const u8, (*dest_desc).string.pointer as *mut u8, (*source_desc).string.length as usize + 1); }
        ACPI_TYPE_LOCAL_REFERENCE => { if (*source_desc).reference.class_ != ACPI_REFCLASS_TABLE && (*source_desc).reference.class_ != ACPI_REFCLASS_LOCAL && (*source_desc).reference.class_ != ACPI_REFCLASS_ARG && (*source_desc).reference.class_ != ACPI_REFCLASS_DEBUG { acpi_ut_add_reference((*source_desc).reference.object); } }
        ACPI_TYPE_REGION => { if !(*dest_desc).region.handler.is_null() { acpi_ut_add_reference((*dest_desc).region.handler); } }
        ACPI_TYPE_MUTEX => { let mut p = (*dest_desc).mutex.os_mutex; let s = acpi_os_create_mutex(&mut p); if ACPI_FAILURE(s) { return s; } (*dest_desc).mutex.os_mutex = p; }
        ACPI_TYPE_EVENT => { let mut p = (*dest_desc).event.os_semaphore; let s = acpi_os_create_semaphore(ACPI_NO_UNIT_LIMIT, 0, &mut p); if ACPI_FAILURE(s) { return s; } (*dest_desc).event.os_semaphore = p; }
        _ => {}
    } AE_OK
}

unsafe fn acpi_ut_copy_ielement_to_ielement(object_type: u8, source_object: *mut acpi_operand_object, state: *mut acpi_generic_state, _context: *mut core::ffi::c_void) -> acpi_status {
    let target = &mut (*state).pkg.dest_object.as_mut().unwrap().package.elements[(*state).pkg.index as usize];
    match object_type {
        ACPI_COPY_TYPE_SIMPLE => { if source_object.is_null() { *target = core::ptr::null_mut(); } else { let obj = acpi_ut_create_internal_object((*source_object).common.type_); if obj.is_null() { return AE_NO_MEMORY; } let s = acpi_ut_copy_simple_object(source_object, obj); if ACPI_FAILURE(s) { acpi_ut_remove_reference(obj); return s; } *target = obj; } }
        ACPI_COPY_TYPE_PACKAGE => { let obj = acpi_ut_create_package_object((*source_object).package.count); if obj.is_null() { return AE_NO_MEMORY; } (*obj).common.flags = (*source_object).common.flags; (*state).pkg.this_target_obj = obj; *target = obj; }
        _ => return AE_BAD_PARAMETER,
    } AE_OK
}

unsafe fn acpi_ut_copy_ipackage_to_ipackage(source_obj: *mut acpi_operand_object, dest_obj: *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status {
    (*dest_obj).common.type_ = (*source_obj).common.type_; (*dest_obj).common.flags = (*source_obj).common.flags; (*dest_obj).package.count = (*source_obj).package.count;
    (*dest_obj).package.elements = acpi_allocate_zeroed(((*source_obj).package.count as acpi_size + 1) * core::mem::size_of::<*mut core::ffi::c_void>() as acpi_size) as *mut *mut acpi_operand_object;
    if (*dest_obj).package.elements.is_null() { return AE_NO_MEMORY; }
    acpi_ut_walk_package_tree(source_obj, dest_obj, acpi_ut_copy_ielement_to_ielement, walk_state as *mut _)
}

pub unsafe fn acpi_ut_copy_iobject_to_iobject(source_desc: *mut acpi_operand_object, dest_desc: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status {
    *dest_desc = acpi_ut_create_internal_object((*source_desc).common.type_); if (*dest_desc).is_null() { return AE_NO_MEMORY; }
    let status = if (*source_desc).common.type_ == ACPI_TYPE_PACKAGE { acpi_ut_copy_ipackage_to_ipackage(source_desc, *dest_desc, walk_state) } else { acpi_ut_copy_simple_object(source_desc, *dest_desc) };
    if ACPI_FAILURE(status) { acpi_ut_remove_reference(*dest_desc); } status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
