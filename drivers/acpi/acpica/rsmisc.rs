// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Miscellaneous resource descriptors.

// Dependencies supplied by the surrounding ACPICA translation.

const INIT_RESOURCE_TYPE: fn(*const acpi_rsconvert_info) -> u32 = |i| unsafe { (*i).resource_offset };
const INIT_RESOURCE_LENGTH: fn(*const acpi_rsconvert_info) -> u32 = |i| unsafe { (*i).aml_offset };
const INIT_TABLE_LENGTH: fn(*const acpi_rsconvert_info) -> u8 = |i| unsafe { (*i).value as u8 };
const COMPARE_OPCODE: fn(*const acpi_rsconvert_info) -> u32 = |i| unsafe { (*i).resource_offset };
const COMPARE_TARGET: fn(*const acpi_rsconvert_info) -> u32 = |i| unsafe { (*i).aml_offset };
const COMPARE_VALUE: fn(*const acpi_rsconvert_info) -> u32 = |i| unsafe { (*i).value };

pub unsafe fn acpi_rs_convert_aml_to_resource(
    resource: *mut acpi_resource, aml: *mut aml_resource,
    mut info: *mut acpi_rsconvert_info,
) -> acpi_status {
    let aml_resource_length = acpi_ut_get_resource_length(aml);
    let mut flags_mode = false;
    let mut item_count: u16 = 0;
    let mut temp16: u16 = 0;
    if info.is_null() { return AE_BAD_PARAMETER; }
    let mut count = INIT_TABLE_LENGTH(info);
    while count != 0 {
        let mut target: *mut u8 = core::ptr::null_mut();
        let source = (aml as *mut u8).add((*info).aml_offset as usize) as *mut u8;
        let destination = (resource as *mut u8).add((*info).resource_offset as usize);
        match (*info).opcode {
            ACPI_RSC_INITGET => { core::ptr::write_bytes(resource as *mut u8, 0, (*info).aml_offset as usize); (*resource).type_ = (*info).resource_offset; (*resource).length = (*info).aml_offset; }
            ACPI_RSC_INITSET => {}
            ACPI_RSC_FLAGINIT => flags_mode = true,
            ACPI_RSC_1BITFLAG => *destination = (*source >> (*info).value) & 1,
            ACPI_RSC_2BITFLAG => *destination = (*source >> (*info).value) & 3,
            ACPI_RSC_3BITFLAG => *destination = (*source >> (*info).value) & 7,
            ACPI_RSC_6BITFLAG => *destination = (*source >> (*info).value) & 0x3f,
            ACPI_RSC_COUNT => { item_count = *source as u16; *destination = item_count as u8; (*resource).length += (*info).value * item_count.saturating_sub(1) as u32; }
            ACPI_RSC_COUNT16 => { item_count = aml_resource_length as u16; *(destination as *mut u16) = item_count; (*resource).length += (*info).value * item_count.saturating_sub(1) as u32; }
            ACPI_RSC_COUNT_GPIO_PIN => { target = (aml as *mut u8).add((*info).value as usize); item_count = *(target as *const u16) - *(source as *const u16); (*resource).length += item_count as u32; item_count /= 2; *(destination as *mut u16) = item_count; }
            ACPI_RSC_COUNT_GPIO_VEN => { item_count = *source as u16; *destination = item_count as u8; (*resource).length += (*info).value * item_count as u32; }
            ACPI_RSC_COUNT_GPIO_RES => { target = (aml as *mut u8).add((*info).value as usize + 2); if *(target as *const u16) != 0 { target = (aml as *mut u8).add((*info).value as usize); item_count = *(target as *const u16) - *(source as *const u16); } else { item_count = (*aml).large_header.resource_length + core::mem::size_of::<aml_resource_large_header>() as u16 - *(source as *const u16); } (*resource).length += item_count as u32; *(destination as *mut u16) = item_count; }
            ACPI_RSC_COUNT_SERIAL_VEN => { temp16 = *(source as *const u16); item_count = temp16 - (*info).value as u16; (*resource).length += item_count as u32; *(destination as *mut u16) = item_count; }
            ACPI_RSC_COUNT_SERIAL_RES => { temp16 = *(source as *const u16); item_count = (aml_resource_length + core::mem::size_of::<aml_resource_large_header>() as u32) as u16 - temp16 - (*info).value as u16; (*resource).length += item_count as u32; *(destination as *mut u16) = item_count; }
            ACPI_RSC_LENGTH => (*resource).length += (*info).value,
            ACPI_RSC_MOVE8 | ACPI_RSC_MOVE16 | ACPI_RSC_MOVE32 | ACPI_RSC_MOVE64 => { if (*info).value != 0 { item_count = (*info).value as u16; } acpi_rs_move_data(destination as *mut _, source as *mut _, item_count, (*info).opcode); }
            ACPI_RSC_MOVE_GPIO_PIN => { target = (resource as *mut u8).add((*resource).length as usize - item_count as usize * 2); *(destination as *mut *mut u16) = target as *mut u16; let s = (aml as *mut u8).add(*(source as *const u16) as usize); acpi_rs_move_data(target as *mut _, s as *mut _, item_count, (*info).opcode); }
            ACPI_RSC_MOVE_GPIO_RES | ACPI_RSC_MOVE_SERIAL_VEN => { target = (resource as *mut u8).add((*resource).length as usize - item_count as usize); *(destination as *mut *mut u8) = target; let s = if (*info).opcode == ACPI_RSC_MOVE_SERIAL_VEN { (aml as *mut u8).add((*info).value as usize) } else { (aml as *mut u8).add(*(source as *const u16) as usize) }; acpi_rs_move_data(target as *mut _, s as *mut _, item_count, (*info).opcode); }
            ACPI_RSC_MOVE_SERIAL_RES => { target = (resource as *mut u8).add((*resource).length as usize - item_count as usize); *(destination as *mut *mut u8) = target; temp16 = *(source as *const u16); let s = (aml as *mut u8).add((temp16 as u32 + (*info).value) as usize); acpi_rs_move_data(target as *mut _, s as *mut _, item_count, (*info).opcode); }
            ACPI_RSC_SET8 => core::ptr::write_bytes(destination, (*info).aml_offset as u8, (*info).value as usize),
            ACPI_RSC_DATA8 => { target = (resource as *mut u8).add((*info).value as usize); core::ptr::copy_nonoverlapping(source, destination, *(target as *const u16) as usize); }
            ACPI_RSC_ADDRESS => if !acpi_rs_get_address_common(resource, aml) { return AE_AML_INVALID_RESOURCE_TYPE; },
            ACPI_RSC_SOURCE => (*resource).length += acpi_rs_get_resource_source(aml_resource_length, (*info).value, destination, aml, core::ptr::null_mut()),
            ACPI_RSC_SOURCEX => { target = (resource as *mut u8).add((*info).aml_offset as usize + item_count as usize * 4); (*resource).length += acpi_rs_get_resource_source(aml_resource_length, ((item_count.saturating_sub(1) as u32 * 4) + (*info).value), destination, aml, target); }
            ACPI_RSC_BITMASK | ACPI_RSC_BITMASK16 => { temp16 = if (*info).opcode == ACPI_RSC_BITMASK16 { *(source as *const u16) } else { *source as u16 }; item_count = acpi_rs_decode_bitmask(temp16, destination) as u16; if item_count != 0 { (*resource).length += item_count as u32 - 1; } *((resource as *mut u8).add((*info).value as usize)) = item_count as u8; }
            ACPI_RSC_EXIT_NE => { if (*info).resource_offset == ACPI_RSC_COMPARE_AML_LENGTH && aml_resource_length != (*info).value { break; } if (*info).resource_offset == ACPI_RSC_COMPARE_VALUE && *source as u32 != (*info).value { break; } }
            _ => return AE_BAD_PARAMETER,
        }
        count -= 1; info = info.add(1);
    }
    if !flags_mode { (*resource).length = acpi_round_up_native_word((*resource).length); }
    AE_OK
}

pub unsafe fn acpi_rs_convert_resource_to_aml(resource: *mut acpi_resource, aml: *mut aml_resource, mut info: *mut acpi_rsconvert_info) -> acpi_status {
    if info.is_null() { return AE_BAD_PARAMETER; }
    let mut count = INIT_TABLE_LENGTH(info); let mut aml_length: u32 = 0; let mut item_count: u16 = 0; let mut temp16: u16;
    while count != 0 {
        let mut source = (resource as *mut u8).add((*info).resource_offset as usize); let mut destination = (aml as *mut u8).add((*info).aml_offset as usize);
        match (*info).opcode {
            ACPI_RSC_INITSET => { core::ptr::write_bytes(aml as *mut u8, 0, (*info).aml_offset as usize); aml_length = (*info).aml_offset; acpi_rs_set_resource_header((*info).resource_offset, aml_length, aml); }
            ACPI_RSC_INITGET => {}
            ACPI_RSC_FLAGINIT => *destination = 0,
            ACPI_RSC_1BITFLAG => *destination |= (*source & 1) << (*info).value,
            ACPI_RSC_2BITFLAG => *destination |= (*source & 3) << (*info).value,
            ACPI_RSC_3BITFLAG => *destination |= (*source & 7) << (*info).value,
            ACPI_RSC_6BITFLAG => *destination |= (*source & 0x3f) << (*info).value,
            ACPI_RSC_COUNT => { item_count = *source as u16; *destination = item_count as u8; aml_length += (*info).value * item_count.saturating_sub(1) as u32; }
            ACPI_RSC_COUNT16 => { item_count = *(source as *const u16); aml_length += item_count as u32; acpi_rs_set_resource_length(aml_length, aml); }
            ACPI_RSC_COUNT_GPIO_PIN => { item_count = *(source as *const u16); *(destination as *mut u16) = aml_length as u16; aml_length += item_count as u32 * 2; *( (aml as *mut u8).add((*info).value as usize) as *mut u16) = aml_length as u16; acpi_rs_set_resource_length(aml_length, aml); }
            ACPI_RSC_COUNT_GPIO_VEN => { item_count = *(source as *const u16); *(destination as *mut u16) = item_count; aml_length += (*info).value * item_count as u32; acpi_rs_set_resource_length(aml_length, aml); }
            ACPI_RSC_COUNT_GPIO_RES => { item_count = *(source as *const u16); *(destination as *mut u16) = aml_length as u16; aml_length += item_count as u32; *((aml as *mut u8).add((*info).value as usize) as *mut u16) = aml_length as u16; acpi_rs_set_resource_length(aml_length, aml); }
            ACPI_RSC_COUNT_SERIAL_VEN => { item_count = *(source as *const u16); *(destination as *mut u16) = item_count + (*info).value as u16; aml_length += item_count as u32; acpi_rs_set_resource_length(aml_length, aml); }
            ACPI_RSC_COUNT_SERIAL_RES => { item_count = *(source as *const u16); aml_length += item_count as u32; acpi_rs_set_resource_length(aml_length, aml); }
            ACPI_RSC_LENGTH => acpi_rs_set_resource_length((*info).value, aml),
            ACPI_RSC_MOVE8 | ACPI_RSC_MOVE16 | ACPI_RSC_MOVE32 | ACPI_RSC_MOVE64 => { if (*info).value != 0 { item_count = (*info).value as u16; } acpi_rs_move_data(destination as *mut _, source as *mut _, item_count, (*info).opcode); }
            ACPI_RSC_MOVE_GPIO_PIN | ACPI_RSC_MOVE_GPIO_RES => { destination = (aml as *mut u8).add(*(destination as *const u16) as usize); source = if (*info).opcode == ACPI_RSC_MOVE_GPIO_PIN { *(source as *const *mut u16) as *mut u8 } else { *(source as *const *mut u8) }; acpi_rs_move_data(destination as *mut _, source as *mut _, item_count, (*info).opcode); }
            ACPI_RSC_MOVE_SERIAL_VEN | ACPI_RSC_MOVE_SERIAL_RES => { destination = (aml as *mut u8).add(aml_length as usize - item_count as usize); source = *(source as *const *mut u8); acpi_rs_move_data(destination as *mut _, source as *mut _, item_count, (*info).opcode); }
            ACPI_RSC_ADDRESS => acpi_rs_set_address_common(aml, resource),
            ACPI_RSC_SOURCEX => { aml_length = acpi_rs_set_resource_source(aml, aml_length, source as *mut _); acpi_rs_set_resource_length(aml_length, aml); }
            ACPI_RSC_SOURCE => { aml_length = acpi_rs_set_resource_source(aml, (*info).value, source as *mut _); acpi_rs_set_resource_length(aml_length, aml); }
            ACPI_RSC_BITMASK => *destination = acpi_rs_encode_bitmask(source as *mut _, *((resource as *mut u8).add((*info).value as usize))),
            ACPI_RSC_BITMASK16 => { temp16 = acpi_rs_encode_bitmask(source as *mut _, *((resource as *mut u8).add((*info).value as usize))) as u16; *(destination as *mut u16) = temp16; }
            ACPI_RSC_EXIT_LE => if item_count <= (*info).value as u16 { break },
            ACPI_RSC_EXIT_NE => if *((resource as *mut u8).add((*info).aml_offset as usize)) as u32 != (*info).value { break },
            ACPI_RSC_EXIT_EQ => if *((resource as *mut u8).add((*info).aml_offset as usize)) as u32 == (*info).value { break },
            _ => return AE_BAD_PARAMETER,
        }
        count -= 1; info = info.add(1);
    }
    AE_OK
}

// The source's #if 0 block contains historical resource validations and is intentionally inactive.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
