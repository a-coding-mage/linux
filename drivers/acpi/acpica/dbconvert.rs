// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*!
 * Module Name: dbconvert - debugger miscellaneous conversion routines
 */

// Dependencies are supplied by the surrounding ACPI implementation.

const DB_DEFAULT_PKG_ELEMENTS: u32 = 33;

pub unsafe extern "C" fn acpi_db_hex_char_to_value(
    hex_char: i32,
    return_value: *mut u8,
) -> acpi_status {
    let value: u8;
    if !isxdigit(hex_char) {
        return AE_BAD_HEX_CONSTANT;
    }
    if hex_char <= 0x39 {
        value = (hex_char - 0x30) as u8;
    } else {
        value = (toupper(hex_char) - 0x37) as u8;
    }
    *return_value = value;
    AE_OK
}

unsafe fn acpi_db_hex_byte_to_binary(hex_byte: *mut i8, return_value: *mut u8) -> acpi_status {
    let mut local0 = 0u8;
    let mut local1 = 0u8;
    let mut status = acpi_db_hex_char_to_value(*hex_byte.add(0) as i32, &mut local0);
    if ACPI_FAILURE(status) {
        return status;
    }
    status = acpi_db_hex_char_to_value(*hex_byte.add(1) as i32, &mut local1);
    if ACPI_FAILURE(status) {
        return status;
    }
    *return_value = (local0 << 4) | local1;
    AE_OK
}

unsafe fn acpi_db_convert_to_buffer(string: *mut i8, object: *mut acpi_object) -> acpi_status {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut length: u32 = 0;
    let mut status: acpi_status;
    acpi_ut_remove_whitespace(&mut string);
    while *string.add(i as usize) != 0 {
        i += 2;
        length += 1;
        while *string.add(i as usize) != 0
            && (*string.add(i as usize) == ',' as i8 || *string.add(i as usize) == ' ' as i8)
        {
            i += 1;
        }
    }
    let buffer = ACPI_ALLOCATE(length);
    if buffer.is_null() {
        return AE_NO_MEMORY;
    }
    i = 0;
    while *string.add(i as usize) != 0 {
        status = acpi_db_hex_byte_to_binary(string.add(i as usize), buffer.add(j as usize));
        if ACPI_FAILURE(status) {
            ACPI_FREE(buffer);
            return status;
        }
        j += 1;
        i += 2;
        while *string.add(i as usize) != 0
            && (*string.add(i as usize) == ',' as i8 || *string.add(i as usize) == ' ' as i8)
        {
            i += 1;
        }
    }
    (*object).type_ = ACPI_TYPE_BUFFER;
    (*object).buffer.pointer = buffer;
    (*object).buffer.length = length;
    AE_OK
}

pub unsafe extern "C" fn acpi_db_convert_to_package(
    string: *mut i8,
    object: *mut acpi_object,
) -> acpi_status {
    let mut this = string;
    let mut next: *mut i8 = core::ptr::null_mut();
    let mut i: u32 = 0;
    let mut type_: acpi_object_type = 0;
    let elements = ACPI_ALLOCATE_ZEROED(
        DB_DEFAULT_PKG_ELEMENTS * core::mem::size_of::<acpi_object>() as u32,
    ) as *mut acpi_object;
    if elements.is_null() {
        return AE_NO_MEMORY;
    }
    while i < DB_DEFAULT_PKG_ELEMENTS - 1 {
        this = acpi_db_get_next_token(this, &mut next, &mut type_);
        if this.is_null() {
            break;
        }
        let status = acpi_db_convert_to_object(type_, this, elements.add(i as usize));
        if ACPI_FAILURE(status) {
            acpi_db_delete_objects(i + 1, elements);
            ACPI_FREE(elements);
            return status;
        }
        i += 1;
        this = next;
    }
    (*object).type_ = ACPI_TYPE_PACKAGE;
    (*object).package.count = i;
    (*object).package.elements = elements;
    AE_OK
}

pub unsafe extern "C" fn acpi_db_convert_to_object(
    type_: acpi_object_type,
    string: *mut i8,
    object: *mut acpi_object,
) -> acpi_status {
    let mut status = AE_OK;
    match type_ {
        ACPI_TYPE_STRING => {
            (*object).type_ = ACPI_TYPE_STRING;
            (*object).string.pointer = string;
            (*object).string.length = strlen(string) as u32;
        }
        ACPI_TYPE_BUFFER => status = acpi_db_convert_to_buffer(string, object),
        ACPI_TYPE_PACKAGE => status = acpi_db_convert_to_package(string, object),
        _ => {
            (*object).type_ = ACPI_TYPE_INTEGER;
            status = acpi_ut_strtoul64(string, &mut (*object).integer.value);
        }
    }
    status
}

pub unsafe extern "C" fn acpi_db_encode_pld_buffer(
    pld_info: *mut acpi_pld_info,
) -> *mut u8 {
    let buffer = ACPI_ALLOCATE_ZEROED(ACPI_PLD_BUFFER_SIZE) as *mut u32;
    if buffer.is_null() {
        return core::ptr::null_mut();
    }
    let mut dword: u32 = 0;
    ACPI_PLD_SET_REVISION(&mut dword, (*pld_info).revision);
    ACPI_PLD_SET_IGNORE_COLOR(&mut dword, (*pld_info).ignore_color);
    ACPI_PLD_SET_RED(&mut dword, (*pld_info).red);
    ACPI_PLD_SET_GREEN(&mut dword, (*pld_info).green);
    ACPI_PLD_SET_BLUE(&mut dword, (*pld_info).blue);
    ACPI_MOVE_32_TO_32(buffer.add(0), &mut dword);
    dword = 0;
    ACPI_PLD_SET_WIDTH(&mut dword, (*pld_info).width);
    ACPI_PLD_SET_HEIGHT(&mut dword, (*pld_info).height);
    ACPI_MOVE_32_TO_32(buffer.add(1), &mut dword);
    dword = 0;
    ACPI_PLD_SET_USER_VISIBLE(&mut dword, (*pld_info).user_visible);
    ACPI_PLD_SET_DOCK(&mut dword, (*pld_info).dock);
    ACPI_PLD_SET_LID(&mut dword, (*pld_info).lid);
    ACPI_PLD_SET_PANEL(&mut dword, (*pld_info).panel);
    ACPI_PLD_SET_VERTICAL(&mut dword, (*pld_info).vertical_position);
    ACPI_PLD_SET_HORIZONTAL(&mut dword, (*pld_info).horizontal_position);
    ACPI_PLD_SET_SHAPE(&mut dword, (*pld_info).shape);
    ACPI_PLD_SET_ORIENTATION(&mut dword, (*pld_info).group_orientation);
    ACPI_PLD_SET_TOKEN(&mut dword, (*pld_info).group_token);
    ACPI_PLD_SET_POSITION(&mut dword, (*pld_info).group_position);
    ACPI_PLD_SET_BAY(&mut dword, (*pld_info).bay);
    ACPI_MOVE_32_TO_32(buffer.add(2), &mut dword);
    dword = 0;
    ACPI_PLD_SET_EJECTABLE(&mut dword, (*pld_info).ejectable);
    ACPI_PLD_SET_OSPM_EJECT(&mut dword, (*pld_info).ospm_eject_required);
    ACPI_PLD_SET_CABINET(&mut dword, (*pld_info).cabinet_number);
    ACPI_PLD_SET_CARD_CAGE(&mut dword, (*pld_info).card_cage_number);
    ACPI_PLD_SET_REFERENCE(&mut dword, (*pld_info).reference);
    ACPI_PLD_SET_ROTATION(&mut dword, (*pld_info).rotation);
    ACPI_PLD_SET_ORDER(&mut dword, (*pld_info).order);
    ACPI_MOVE_32_TO_32(buffer.add(3), &mut dword);
    if (*pld_info).revision >= 2 {
        dword = 0;
        ACPI_PLD_SET_VERT_OFFSET(&mut dword, (*pld_info).vertical_offset);
        ACPI_PLD_SET_HORIZ_OFFSET(&mut dword, (*pld_info).horizontal_offset);
        ACPI_MOVE_32_TO_32(buffer.add(4), &mut dword);
    }
    buffer as *mut u8
}

pub unsafe extern "C" fn acpi_db_dump_pld_buffer(obj_desc: *mut acpi_object) {
    if (*obj_desc).type_ != ACPI_TYPE_PACKAGE {
        return;
    }
    let buffer_desc = (*obj_desc).package.elements;
    if (*buffer_desc).type_ != ACPI_TYPE_BUFFER {
        return;
    }
    let mut pld_info: *mut acpi_pld_info = core::ptr::null_mut();
    let status = acpi_decode_pld_buffer(
        (*buffer_desc).buffer.pointer,
        (*buffer_desc).buffer.length,
        &mut pld_info,
    );
    if ACPI_FAILURE(status) {
        return;
    }
    let new_buffer = acpi_db_encode_pld_buffer(pld_info);
    if !new_buffer.is_null() {
        if memcmp(new_buffer, (*buffer_desc).buffer.pointer, (*buffer_desc).buffer.length as usize) != 0 {
            acpi_os_printf(b"Converted _PLD buffer does not compare. New:\n\0".as_ptr());
            acpi_ut_dump_buffer(new_buffer, (*buffer_desc).buffer.length, DB_BYTE_DISPLAY, 0);
        }
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Revision\0".as_ptr(), (*pld_info).revision);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_IgnoreColor\0".as_ptr(), (*pld_info).ignore_color);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Red\0".as_ptr(), (*pld_info).red);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Green\0".as_ptr(), (*pld_info).green);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Blue\0".as_ptr(), (*pld_info).blue);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Width\0".as_ptr(), (*pld_info).width);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Height\0".as_ptr(), (*pld_info).height);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_UserVisible\0".as_ptr(), (*pld_info).user_visible);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Dock\0".as_ptr(), (*pld_info).dock);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Lid\0".as_ptr(), (*pld_info).lid);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Panel\0".as_ptr(), (*pld_info).panel);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_VerticalPosition\0".as_ptr(), (*pld_info).vertical_position);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_HorizontalPosition\0".as_ptr(), (*pld_info).horizontal_position);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Shape\0".as_ptr(), (*pld_info).shape);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_GroupOrientation\0".as_ptr(), (*pld_info).group_orientation);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_GroupToken\0".as_ptr(), (*pld_info).group_token);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_GroupPosition\0".as_ptr(), (*pld_info).group_position);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Bay\0".as_ptr(), (*pld_info).bay);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Ejectable\0".as_ptr(), (*pld_info).ejectable);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_EjectRequired\0".as_ptr(), (*pld_info).ospm_eject_required);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_CabinetNumber\0".as_ptr(), (*pld_info).cabinet_number);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_CardCageNumber\0".as_ptr(), (*pld_info).card_cage_number);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Reference\0".as_ptr(), (*pld_info).reference);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Rotation\0".as_ptr(), (*pld_info).rotation);
        acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_Order\0".as_ptr(), (*pld_info).order);
        if (*buffer_desc).buffer.length > 16 {
            acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_VerticalOffset\0".as_ptr(), (*pld_info).vertical_offset);
            acpi_os_printf(b"%20s : %-6X\n\0".as_ptr(), b"PLD_HorizontalOffset\0".as_ptr(), (*pld_info).horizontal_offset);
        }
        ACPI_FREE(new_buffer);
    }
    ACPI_FREE(pld_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
