// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Aml Field I/O

// Includes and ACPI declarations are supplied by the surrounding translation.

unsafe fn acpi_ex_field_datum_io(obj_desc: *mut acpi_operand_object, off: u32, value: *mut u64, rw: u32) -> acpi_status;
unsafe fn acpi_ex_register_overflow(obj_desc: *mut acpi_operand_object, value: u64) -> u8 {
    if (*obj_desc).common_field.bit_length >= ACPI_INTEGER_BIT_SIZE { return FALSE; }
    if value >= (1u64 << (*obj_desc).common_field.bit_length) {
        ACPI_ERROR!((AE_INFO, "Index value 0x%8.8X%8.8X overflows field width 0x%X", ACPI_FORMAT_UINT64(value), (*obj_desc).common_field.bit_length));
        return TRUE;
    }
    FALSE
}

unsafe fn acpi_ex_setup_region(obj_desc: *mut acpi_operand_object, off: u32) -> acpi_status {
    let mut status = AE_OK;
    let rgn_desc = (*obj_desc).common_field.region_obj;
    ACPI_FUNCTION_TRACE_U32!(ex_setup_region, off);
    if (*rgn_desc).common.type_ != ACPI_TYPE_REGION {
        ACPI_ERROR!((AE_INFO, "Needed Region, found type 0x%X (%s)", (*rgn_desc).common.type_, acpi_ut_get_object_type_name(rgn_desc)));
        return AE_AML_OPERAND_TYPE;
    }
    let space_id = (*rgn_desc).region.space_id;
    if !acpi_is_valid_space_id(space_id) { ACPI_ERROR!((AE_INFO, "Invalid/unknown Address Space ID: 0x%2.2X", space_id)); return AE_AML_INVALID_SPACE_ID; }
    if (*rgn_desc).common.flags & AOPOBJ_DATA_VALID == 0 {
        status = acpi_ds_get_region_arguments(rgn_desc);
        if ACPI_FAILURE!(status) { return status; }
    }
    if space_id == ACPI_ADR_SPACE_SMBUS || space_id == ACPI_ADR_SPACE_GSBUS || space_id == ACPI_ADR_SPACE_IPMI { return AE_OK; }
    if (*rgn_desc).region.length < (*obj_desc).common_field.base_byte_offset + off + (*obj_desc).common_field.access_byte_width {
        if acpi_gbl_enable_interpreter_slack && ACPI_ROUND_UP!((*rgn_desc).region.length, (*obj_desc).common_field.access_byte_width) >= (*obj_desc).common_field.base_byte_offset + (*obj_desc).common_field.access_byte_width + off { return AE_OK; }
        if (*rgn_desc).region.length < (*obj_desc).common_field.access_byte_width { ACPI_ERROR!((AE_INFO, "Field access width too large for region")); }
        ACPI_ERROR!((AE_INFO, "Field Base+Offset+Width is beyond end of region"));
        return AE_AML_REGION_LIMIT;
    }
    AE_OK
}

pub unsafe fn acpi_ex_access_region(obj_desc: *mut acpi_operand_object, off: u32, value: *mut u64, function: u32) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ex_access_region);
    let status = acpi_ex_setup_region(obj_desc, off);
    if ACPI_FAILURE!(status) { return status; }
    let rgn_desc = (*obj_desc).common_field.region_obj;
    let region_offset = (*obj_desc).common_field.base_byte_offset + off;
    if function & ACPI_IO_MASK == ACPI_READ { ACPI_DEBUG_PRINT!((ACPI_DB_BFIELD, "[READ]")); } else { ACPI_DEBUG_PRINT!((ACPI_DB_BFIELD, "[WRITE]")); }
    let status = acpi_ev_address_space_dispatch(rgn_desc, obj_desc, function, region_offset, ACPI_MUL_8!((*obj_desc).common_field.access_byte_width), value);
    if ACPI_FAILURE!(status) && status == AE_NOT_IMPLEMENTED { ACPI_ERROR!((AE_INFO, "Region not implemented")); }
    else if ACPI_FAILURE!(status) && status == AE_NOT_EXIST { ACPI_ERROR!((AE_INFO, "Region has no handler")); }
    status
}

unsafe fn acpi_ex_field_datum_io(obj_desc: *mut acpi_operand_object, off: u32, mut value: *mut u64, rw: u32) -> acpi_status {
    let mut local = 0u64;
    if rw == ACPI_READ { if value.is_null() { value = &mut local; } *value = 0; }
    let mut status;
    match (*obj_desc).common.type_ {
        ACPI_TYPE_BUFFER_FIELD => {
            if (*obj_desc).common.flags & AOPOBJ_DATA_VALID == 0 { status = acpi_ds_get_buffer_field_arguments(obj_desc); if ACPI_FAILURE!(status) { return status; } }
            let src = (*obj_desc).buffer_field.buffer_obj as *mut u8;
            let p = src.add((*obj_desc).buffer_field.base_byte_offset as usize + off as usize);
            if rw == ACPI_READ { core::ptr::copy_nonoverlapping(p, value as *mut u8, (*obj_desc).common_field.access_byte_width as usize); } else { core::ptr::copy_nonoverlapping(value as *const u8, p, (*obj_desc).common_field.access_byte_width as usize); }
            status = AE_OK;
        }
        ACPI_TYPE_LOCAL_BANK_FIELD => {
            if acpi_ex_register_overflow((*obj_desc).bank_field.bank_obj, (*obj_desc).bank_field.value as u64) != FALSE { return AE_AML_REGISTER_LIMIT; }
            status = acpi_ex_insert_into_field((*obj_desc).bank_field.bank_obj, &mut (*obj_desc).bank_field.value as *mut _ as *mut _, core::mem::size_of_val(&(*obj_desc).bank_field.value) as u32);
            if ACPI_FAILURE!(status) { return status; }
            status = acpi_ex_access_region(obj_desc, off, value, rw);
        }
        ACPI_TYPE_LOCAL_REGION_FIELD => { status = acpi_ex_access_region(obj_desc, off, value, rw); }
        ACPI_TYPE_LOCAL_INDEX_FIELD => {
            if acpi_ex_register_overflow((*obj_desc).index_field.index_obj, (*obj_desc).index_field.value as u64) != FALSE { return AE_AML_REGISTER_LIMIT; }
            let mut index = off + (*obj_desc).index_field.value;
            status = acpi_ex_insert_into_field((*obj_desc).index_field.index_obj, &mut index as *mut _ as *mut _, core::mem::size_of::<u32>() as u32);
            if ACPI_FAILURE!(status) { return status; }
            if rw == ACPI_READ { status = acpi_ex_extract_from_field((*obj_desc).index_field.data_obj, value as *mut _, core::mem::size_of::<u64>() as u32); } else { status = acpi_ex_insert_into_field((*obj_desc).index_field.data_obj, value as *mut _, core::mem::size_of::<u64>() as u32); }
        }
        _ => { ACPI_ERROR!((AE_INFO, "Wrong object type in field I/O %u", (*obj_desc).common.type_)); status = AE_AML_INTERNAL; }
    }
    status
}

pub unsafe fn acpi_ex_write_with_update_rule(obj_desc: *mut acpi_operand_object, mask: u64, field_value: u64, off: u32) -> acpi_status {
    let mut merged = field_value;
    if mask != ACPI_UINT64_MAX { match (*obj_desc).common_field.field_flags & AML_FIELD_UPDATE_RULE_MASK {
        AML_FIELD_UPDATE_PRESERVE => { let mut cur=0; if ((!mask << (ACPI_MUL_8!(8)-ACPI_MUL_8!((*obj_desc).common_field.access_byte_width))) != 0) { let s=acpi_ex_field_datum_io(obj_desc,off,&mut cur,ACPI_READ); if ACPI_FAILURE!(s){return s;} merged |= cur & !mask; } }
        AML_FIELD_UPDATE_WRITE_AS_ONES => merged |= !mask,
        AML_FIELD_UPDATE_WRITE_AS_ZEROS => merged &= mask,
        _ => return AE_AML_OPERAND_VALUE,
    }}
    acpi_ex_field_datum_io(obj_desc, off, &mut merged, ACPI_WRITE)
}

pub unsafe fn acpi_ex_extract_from_field(obj_desc: *mut acpi_operand_object, buffer: *mut core::ffi::c_void, buffer_length: u32) -> acpi_status {
    let need=ACPI_ROUND_BITS_UP_TO_BYTES!((*obj_desc).common_field.bit_length); if buffer_length < need { return AE_BUFFER_OVERFLOW; }
    core::ptr::write_bytes(buffer as *mut u8,0,buffer_length as usize);
    let width=ACPI_MUL_8!((*obj_desc).common_field.access_byte_width); let mut raw=0u64; let s=acpi_ex_field_datum_io(obj_desc,0,&mut raw,ACPI_READ); if ACPI_FAILURE!(s){return s;}
    let mut merged=raw >> (*obj_desc).common_field.start_field_bit_offset; let mut off=(*obj_desc).common_field.access_byte_width; let count=ACPI_ROUND_UP_TO!((*obj_desc).common_field.bit_length + (*obj_desc).common_field.start_field_bit_offset,width);
    for i in 1..count { raw=0; let s=acpi_ex_field_datum_io(obj_desc,off,&mut raw,ACPI_READ); if ACPI_FAILURE!(s){return s;} if width-(*obj_desc).common_field.start_field_bit_offset < ACPI_INTEGER_BIT_SIZE { merged |= raw << (width-(*obj_desc).common_field.start_field_bit_offset); } if i==ACPI_ROUND_UP_TO!((*obj_desc).common_field.bit_length,width){break;} core::ptr::copy_nonoverlapping(&merged as *const u64 as *const u8, (buffer as *mut u8).add((off-(*obj_desc).common_field.access_byte_width) as usize), core::cmp::min((*obj_desc).common_field.access_byte_width,buffer_length-(off-(*obj_desc).common_field.access_byte_width)) as usize); off+=(*obj_desc).common_field.access_byte_width; merged=raw >> (*obj_desc).common_field.start_field_bit_offset; }
    core::ptr::copy_nonoverlapping(&merged as *const u64 as *const u8, (buffer as *mut u8).add((off-(*obj_desc).common_field.access_byte_width) as usize), core::cmp::min((*obj_desc).common_field.access_byte_width,buffer_length-(off-(*obj_desc).common_field.access_byte_width)) as usize); AE_OK
}

pub unsafe fn acpi_ex_insert_into_field(obj_desc: *mut acpi_operand_object, buffer: *mut core::ffi::c_void, buffer_length: u32) -> acpi_status {
    let required=ACPI_ROUND_BITS_UP_TO_BYTES!((*obj_desc).common_field.bit_length); let mut tmp=core::ptr::null_mut(); let mut buf=buffer; let mut len=buffer_length;
    if len<required { tmp=ACPI_ALLOCATE_ZEROED!(required) as *mut _; if tmp.is_null(){return AE_NO_MEMORY;} core::ptr::copy_nonoverlapping(buffer as *const u8,tmp as *mut u8,len as usize); buf=tmp;len=required; }
    let width=ACPI_MUL_8!((*obj_desc).common_field.access_byte_width); let wm=ACPI_MASK_BITS_ABOVE_64!(width); let mut mask=wm & ACPI_MASK_BITS_BELOW!((*obj_desc).common_field.start_field_bit_offset); let count=ACPI_ROUND_UP_TO!((*obj_desc).common_field.bit_length,width); let fcount=ACPI_ROUND_UP_TO!((*obj_desc).common_field.bit_length+(*obj_desc).common_field.start_field_bit_offset,width); let mut raw=0u64; core::ptr::copy_nonoverlapping(buf as *const u8,&mut raw as *mut u64 as *mut u8,core::cmp::min((*obj_desc).common_field.access_byte_width,len) as usize); let mut merged=raw << (*obj_desc).common_field.start_field_bit_offset; let mut off=0; let mut status=AE_OK;
    for i in 1..fcount { merged &= mask; status=acpi_ex_write_with_update_rule(obj_desc,mask,merged,off); if ACPI_FAILURE!(status){break;} off+=(*obj_desc).common_field.access_byte_width; merged=if width-(*obj_desc).common_field.start_field_bit_offset < ACPI_INTEGER_BIT_SIZE {raw >> (width-(*obj_desc).common_field.start_field_bit_offset)} else {0}; mask=wm; if i==count{break;} let bo=off as usize; raw=0; core::ptr::copy_nonoverlapping((buf as *const u8).add(bo),&mut raw as *mut u64 as *mut u8,core::cmp::min((*obj_desc).common_field.access_byte_width,len-off) as usize); merged|=raw<<(*obj_desc).common_field.start_field_bit_offset; }
    if tmp!=core::ptr::null_mut(){ACPI_FREE!(tmp);} status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
