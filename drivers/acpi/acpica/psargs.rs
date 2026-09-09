// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/* Module Name: psargs - Parse AML opcode arguments */

// External ACPI types, constants, macros, and functions are supplied by the
// surrounding translation unit and are intentionally not redefined here.

unsafe fn acpi_ps_get_next_package_length(parser_state: *mut acpi_parse_state) -> u32 {
    let aml = (*parser_state).aml;
    let mut package_length: u32 = 0;
    let mut byte_count: u32;
    let mut byte_zero_mask: u8 = 0x3f;
    let remaining = ((*parser_state).aml_end.offset_from(aml)) as u32;
    if remaining == 0 { return 0; }
    byte_count = (*aml >> 6) as u32;
    if byte_count >= remaining { (*parser_state).aml = (*parser_state).aml_end; return 0; }
    (*parser_state).aml = (*parser_state).aml.add(byte_count as usize + 1);
    while byte_count != 0 {
        package_length |= (*aml.add(byte_count as usize) as u32) << ((byte_count << 3) - 4);
        byte_zero_mask = 0x0f;
        byte_count -= 1;
    }
    package_length | ((*aml & byte_zero_mask) as u32)
}

pub unsafe fn acpi_ps_get_next_package_end(parser_state: *mut acpi_parse_state) -> *mut u8 {
    let start = (*parser_state).aml;
    start.add(acpi_ps_get_next_package_length(parser_state) as usize)
}

pub unsafe fn acpi_ps_get_next_namestring(parser_state: *mut acpi_parse_state) -> *mut i8 {
    let mut start = (*parser_state).aml;
    let mut end = start;
    while end < (*parser_state).aml_end && (ACPI_IS_ROOT_PREFIX(*end) || ACPI_IS_PARENT_PREFIX(*end)) { end = end.add(1); }
    if end >= (*parser_state).aml_end { (*parser_state).aml = (*parser_state).aml_end; return core::ptr::null_mut(); }
    match *end {
        0 => { if end == start { start = core::ptr::null_mut(); } end = end.add(1); }
        AML_DUAL_NAME_PREFIX => { end = end.add(1 + 2 * ACPI_NAMESEG_SIZE as usize); }
        AML_MULTI_NAME_PREFIX => {
            if end.add(1) >= (*parser_state).aml_end { (*parser_state).aml = (*parser_state).aml_end; return core::ptr::null_mut(); }
            end = end.add(2 + (*end.add(1) as usize) * ACPI_NAMESEG_SIZE as usize);
        }
        _ => { end = end.add(ACPI_NAMESEG_SIZE as usize); }
    }
    if end > (*parser_state).aml_end { (*parser_state).aml = (*parser_state).aml_end; return core::ptr::null_mut(); }
    (*parser_state).aml = end;
    start as *mut i8
}

pub unsafe fn acpi_ps_get_next_namepath(walk_state: *mut acpi_walk_state, parser_state: *mut acpi_parse_state, arg: *mut acpi_parse_object, possible_method_call: u8) -> acpi_status {
    let start = (*parser_state).aml;
    let path = acpi_ps_get_next_namestring(parser_state);
    acpi_ps_init_op(arg, AML_INT_NAMEPATH_OP);
    if path.is_null() { (*arg).common.value.name = path; return AE_OK; }
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut status = acpi_ns_lookup((*walk_state).scope_info, path, ACPI_TYPE_ANY, ACPI_IMODE_EXECUTE, ACPI_NS_SEARCH_PARENT | ACPI_NS_DONT_OPEN_SCOPE, core::ptr::null_mut(), &mut node);
    if ACPI_SUCCESS(status) && possible_method_call != 0 && (*node).type_ == ACPI_TYPE_METHOD {
        if GET_CURRENT_ARG_TYPE((*walk_state).arg_types) == ARGP_SUPERNAME || GET_CURRENT_ARG_TYPE((*walk_state).arg_types) == ARGP_TARGET {
            (*walk_state).parser_state.aml = start;
            (*walk_state).arg_count = 1;
            acpi_ps_init_op(arg, AML_INT_METHODCALL_OP);
        }
        let method_desc = acpi_ns_get_attached_object(node);
        let name_op = acpi_ps_alloc_op(AML_INT_NAMEPATH_OP, start);
        if name_op.is_null() { return AE_NO_MEMORY; }
        acpi_ps_init_op(arg, AML_INT_METHODCALL_OP);
        (*name_op).common.value.name = path;
        (*name_op).common.node = node;
        acpi_ps_append_arg(arg, name_op);
        if method_desc.is_null() { return AE_AML_INTERNAL; }
        (*walk_state).arg_count = (*method_desc).method.param_count;
        return AE_OK;
    }
    if status == AE_NOT_FOUND {
        if ((*walk_state).parse_flags & ACPI_PARSE_MODE_MASK) != ACPI_PARSE_EXECUTE { status = AE_OK; }
        else if (*(*walk_state).op).common.aml_opcode == AML_CONDITIONAL_REF_OF_OP { status = AE_OK; }
        else if !(*arg).common.parent.is_null() && ((*(*arg).common.parent).common.aml_opcode == AML_PACKAGE_OP || (*(*arg).common.parent).common.aml_opcode == AML_VARIABLE_PACKAGE_OP) { status = AE_OK; }
    }
    (*arg).common.value.name = path;
    status
}

pub unsafe fn acpi_ps_get_next_simple_arg(parser_state: *mut acpi_parse_state, arg_type: u32, arg: *mut acpi_parse_object) {
    let aml = (*parser_state).aml;
    let remaining = (*parser_state).aml_end.offset_from(aml) as u32;
    let (opcode, length) = match arg_type {
        ARGP_BYTEDATA => { (*arg).common.value.integer = if remaining >= 1 { *aml as u64 } else { 0 }; (AML_BYTE_OP, if remaining >= 1 { 1 } else { 0 }) }
        ARGP_WORDDATA => { let n = remaining.min(2); (*arg).common.value.integer = 0; core::ptr::copy_nonoverlapping(aml, &mut (*arg).common.value.integer as *mut _ as *mut u8, n as usize); (AML_WORD_OP, n) }
        ARGP_DWORDDATA => { let n = remaining.min(4); (*arg).common.value.integer = 0; core::ptr::copy_nonoverlapping(aml, &mut (*arg).common.value.integer as *mut _ as *mut u8, n as usize); (AML_DWORD_OP, n) }
        ARGP_QWORDDATA => { let n = remaining.min(8); (*arg).common.value.integer = 0; core::ptr::copy_nonoverlapping(aml, &mut (*arg).common.value.integer as *mut _ as *mut u8, n as usize); (AML_QWORD_OP, n) }
        ARGP_CHARLIST => { (*arg).common.value.string = aml as *mut i8; let mut n=0; while n<remaining && *aml.add(n as usize)!=0 { n+=1; } if n<remaining { n+=1; } else if remaining>0 { *aml.add((remaining-1) as usize)=0; n=remaining; } (AML_STRING_OP,n) }
        ARGP_NAME | ARGP_NAMESTRING => { acpi_ps_init_op(arg, AML_INT_NAMEPATH_OP); (*arg).common.value.name = acpi_ps_get_next_namestring(parser_state); return; }
        _ => { return; }
    };
    acpi_ps_init_op(arg, opcode); (*parser_state).aml = (*parser_state).aml.add(length as usize);
}

unsafe fn acpi_ps_get_next_field(parser_state: *mut acpi_parse_state) -> *mut acpi_parse_object {
    let aml = (*parser_state).aml;
    if aml >= (*parser_state).aml_end { return core::ptr::null_mut(); }
    let (opcode, advance) = match ACPI_GET8(aml) { AML_FIELD_OFFSET_OP=>(AML_INT_RESERVEDFIELD_OP,1), AML_FIELD_ACCESS_OP=>(AML_INT_ACCESSFIELD_OP,1), AML_FIELD_CONNECTION_OP=>(AML_INT_CONNECTION_OP,1), AML_FIELD_EXT_ACCESS_OP=>(AML_INT_EXTACCESSFIELD_OP,1), _=>(AML_INT_NAMEDFIELD_OP,0) };
    (*parser_state).aml = (*parser_state).aml.add(advance);
    let field = acpi_ps_alloc_op(opcode, aml); if field.is_null() { return core::ptr::null_mut(); }
    match opcode {
        AML_INT_NAMEDFIELD_OP => { if (*parser_state).aml.add(ACPI_NAMESEG_SIZE as usize) > (*parser_state).aml_end { acpi_ps_free_op(field); return core::ptr::null_mut(); } let mut name=0u32; core::ptr::copy_nonoverlapping((*parser_state).aml, &mut name as *mut _ as *mut u8, 4); acpi_ps_set_name(field,name); (*parser_state).aml=(*parser_state).aml.add(4); (*field).common.value.size=acpi_ps_get_next_package_length(parser_state); }
        AML_INT_RESERVEDFIELD_OP => { (*field).common.value.size=acpi_ps_get_next_package_length(parser_state); }
        AML_INT_ACCESSFIELD_OP | AML_INT_EXTACCESSFIELD_OP => { if (*parser_state).aml.add(2)>(*parser_state).aml_end { acpi_ps_free_op(field); return core::ptr::null_mut(); } let a=ACPI_GET8((*parser_state).aml); let b=ACPI_GET8((*parser_state).aml.add(1)); (*parser_state).aml=(*parser_state).aml.add(2); (*field).common.value.integer=(a as u64)|((b as u64)<<8); if opcode==AML_INT_EXTACCESSFIELD_OP { if (*parser_state).aml>=(*parser_state).aml_end { acpi_ps_free_op(field); return core::ptr::null_mut(); } (*field).common.value.integer|=(ACPI_GET8((*parser_state).aml) as u64)<<16; (*parser_state).aml=(*parser_state).aml.add(1); } }
        AML_INT_CONNECTION_OP => { let arg=acpi_ps_alloc_op(AML_INT_NAMEPATH_OP,(*parser_state).aml); if arg.is_null(){acpi_ps_free_op(field);return core::ptr::null_mut();} (*arg).common.value.name=acpi_ps_get_next_namestring(parser_state); acpi_ps_append_arg(field,arg); }
        _ => {}
    } field
}

unsafe fn acpi_ps_free_field_list(mut cur: *mut acpi_parse_object) { while !cur.is_null() { let next=(*cur).common.next; let arg=acpi_ps_get_arg(cur,0); if !arg.is_null(){acpi_ps_free_op(arg);} acpi_ps_free_op(cur); cur=next; } }

pub unsafe fn acpi_ps_get_next_arg(walk_state:*mut acpi_walk_state, parser_state:*mut acpi_parse_state, arg_type:u32, return_arg:*mut *mut acpi_parse_object)->acpi_status {
    let mut arg=core::ptr::null_mut(); let mut prev=core::ptr::null_mut(); let mut status=AE_OK;
    match arg_type {
        ARGP_BYTEDATA|ARGP_WORDDATA|ARGP_DWORDDATA|ARGP_CHARLIST|ARGP_NAME|ARGP_NAMESTRING => { arg=acpi_ps_alloc_op(AML_BYTE_OP,(*parser_state).aml); if arg.is_null(){return AE_NO_MEMORY;} acpi_ps_get_next_simple_arg(parser_state,arg_type,arg); }
        ARGP_PKGLENGTH => { (*parser_state).pkg_end=acpi_ps_get_next_package_end(parser_state); if (*parser_state).pkg_end>(*parser_state).aml_end || (*parser_state).pkg_end<(*parser_state).aml{return AE_AML_PACKAGE_LIMIT;} }
        ARGP_FIELDLIST => { while (*parser_state).aml<(*parser_state).pkg_end { let field=acpi_ps_get_next_field(parser_state); if field.is_null(){if !arg.is_null(){acpi_ps_free_field_list(arg);}return AE_NO_MEMORY;} if !prev.is_null(){(*prev).common.next=field;}else{arg=field;}prev=field;} (*parser_state).aml=(*parser_state).pkg_end; }
        ARGP_BYTELIST => { if (*parser_state).aml<(*parser_state).pkg_end {arg=acpi_ps_alloc_op(AML_INT_BYTELIST_OP,(*parser_state).aml);if arg.is_null(){return AE_NO_MEMORY;}(*arg).common.value.size=(*parser_state).pkg_end.offset_from((*parser_state).aml) as u32;(*arg).named.data=(*parser_state).aml;(*parser_state).aml=(*parser_state).pkg_end;} }
        ARGP_SIMPLENAME|ARGP_NAME_OR_REF|ARGP_TARGET|ARGP_SUPERNAME => { arg=acpi_ps_alloc_op(AML_INT_NAMEPATH_OP,(*parser_state).aml);if arg.is_null(){return AE_NO_MEMORY;}status=acpi_ps_get_next_namepath(walk_state,parser_state,arg,if arg_type==ARGP_TARGET||arg_type==ARGP_SUPERNAME{ACPI_POSSIBLE_METHOD_CALL}else{ACPI_NOT_METHOD_CALL});if ACPI_FAILURE(status){acpi_ps_free_op(arg);arg=core::ptr::null_mut();} }
        ARGP_DATAOBJ|ARGP_TERMARG|ARGP_DATAOBJLIST|ARGP_TERMLIST|ARGP_OBJLIST => {(*walk_state).arg_count=if arg_type>=ARGP_DATAOBJLIST{ACPI_VAR_ARGS}else{1};}
        _=>{status=AE_AML_OPERAND_TYPE;}
    } *return_arg=arg; status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
