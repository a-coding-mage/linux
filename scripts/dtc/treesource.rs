// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005.
 */

// Dependencies supplied by the surrounding translation unit.

extern "C" {
    static mut yyin: *mut FILE;
    fn yyparse() -> c_int;
    static mut yylloc: YYLTYPE;
}

static mut parser_output: *mut dt_info = std::ptr::null_mut();
static mut treesource_error: bool = false;

unsafe fn delim_start(t: markertype) -> *const c_char { match t { TYPE_UINT8=>b"[\0".as_ptr() as *const c_char, TYPE_UINT16=>b"/bits/ 16 <\0".as_ptr() as *const c_char, TYPE_UINT32=>b"<\0".as_ptr() as *const c_char, TYPE_UINT64=>b"/bits/ 64 <\0".as_ptr() as *const c_char, _=>b"\0".as_ptr() as *const c_char } }

pub unsafe fn dt_from_source(fname: *const c_char) -> *mut dt_info {
    parser_output = std::ptr::null_mut();
    treesource_error = false;
    srcfile_push(fname);
    yyin = (*current_srcfile).f;
    yylloc.file = current_srcfile;
    if yyparse() != 0 { die(b"Unable to parse input tree\n\0".as_ptr() as *const c_char); }
    if treesource_error { die(b"Syntax error parsing input tree\n\0".as_ptr() as *const c_char); }
    parser_output
}

unsafe fn write_prefix(f: *mut FILE, level: c_int) {
    for _ in 0..level { fputc(b'\t' as c_int, f); }
}

unsafe fn isstring(c: c_char) -> bool {
    isprint(c as c_uchar) != 0 || c == 0 || libc::strchr(b"\x07\x08\t\n\x0b\x0c\r\0".as_ptr() as *const c_char, c as c_int) != std::ptr::null_mut()
}

unsafe fn write_propval_string(f: *mut FILE, s: *const c_char, len: usize) {
    let end = s.add(len.wrapping_sub(1));
    if len == 0 { return; }
    assert!(*end == 0);
    fprintf(f, b"\"\0".as_ptr() as *const c_char);
    let mut p = s;
    while p < end {
        let c = *p; p = p.add(1);
        let esc = match c { 7=>b"\\a\0",8=>b"\\b\0",9=>b"\\t\0",10=>b"\\n\0",11=>b"\\v\0",12=>b"\\f\0",13=>b"\\r\0",92=>b"\\\\\0",34=>b"\\\"\0",0=>b"\\0\0", _=>std::ptr::null() };
        if !esc.is_null() { fprintf(f, esc.as_ptr() as *const c_char); }
        else if isprint(c as c_uchar) != 0 { fprintf(f, b"%c\0".as_ptr() as *const c_char, c as c_int); }
        else { fprintf(f, b"\\x%02x\0".as_ptr() as *const c_char, c as c_uchar as c_uint); }
    }
    fprintf(f, b"\"\0".as_ptr() as *const c_char);
}

unsafe fn write_propval_int(f: *mut FILE, p: *const c_char, len: usize, width: usize) {
    assert!(len % width == 0);
    let mut q = p;
    let end = p.add(len);
    while q < end {
        match width {
            1 => fprintf(f, b"%02x\0".as_ptr() as *const c_char, *(q as *const u8) as c_uint),
            2 => fprintf(f, b"0x%02x\0".as_ptr() as *const c_char, dtb_ld16(q) as c_uint),
            4 => fprintf(f, b"0x%02x\0".as_ptr() as *const c_char, dtb_ld32(q) as c_uint),
            8 => fprintf(f, b"0x%02x\0".as_ptr() as *const c_char, dtb_ld64(q) as c_ulonglong),
            _ => (),
        }
        q = q.add(width);
        if q < end { fputc(b' ' as c_int, f); }
    }
}

unsafe fn add_marker(mi: *mut *mut marker, typ: markertype, offset: c_uint, ref_: *mut c_char) -> *mut *mut marker {
    while !(*mi).is_null() && (**mi).offset < offset { mi = &mut (**mi).next; }
    if !(*mi).is_null() && (**mi).offset == offset && is_type_marker((**mi).typ) {
        if is_type_marker(typ) { return mi; }
        mi = &mut (**mi).next;
    }
    if !(*mi).is_null() && (**mi).offset == offset && typ == (**mi).typ { return mi; }
    let nm = xmalloc(std::mem::size_of::<marker>()) as *mut marker;
    (*nm).typ = typ; (*nm).offset = offset; (*nm).ref_ = ref_; (*nm).next = *mi; *mi = nm; &mut (*nm).next
}

pub unsafe fn property_add_marker(prop: *mut property, typ: markertype, offset: c_uint, ref_: *mut c_char) { add_marker(&mut (*prop).val.markers, typ, offset, ref_); }

unsafe fn add_string_markers(prop: *mut property, offset: c_uint, len: c_int) {
    let mut l = libc::strlen((*prop).val.val.add(offset as usize)) as c_int + 1;
    let mut mi = &mut (*prop).val.markers as *mut *mut marker;
    while l < len { mi = add_marker(mi, TYPE_STRING, offset + l as c_uint, std::ptr::null_mut()); l += libc::strlen((*prop).val.val.add((offset as c_int + l) as usize)) as c_int + 1; }
}

pub unsafe fn add_phandle_marker(dti: *mut dt_info, prop: *mut property, offset: c_uint) {
    if (*prop).val.len < offset + 4 { if quiet < 1 { fprintf(stderr, b"Warning: property %s too short to contain a phandle at offset %u\n\0".as_ptr() as *const c_char, (*prop).name, offset); } return; }
    let phandle = dtb_ld32((*prop).val.val.add(offset as usize));
    let refn = get_node_by_phandle((*dti).dt, phandle);
    if refn.is_null() { if quiet < 1 { fprintf(stderr, b"Warning: node referenced by phandle 0x%x in property %s not found\n\0".as_ptr() as *const c_char, phandle, (*prop).name); } return; }
    let ref_ = if !(*refn).labels.is_null() { (*(*refn).labels).label } else { (*refn).fullpath };
    add_marker(&mut (*prop).val.markers, REF_PHANDLE, offset, ref_);
}

unsafe fn guess_value_type(prop: *mut property, offset: c_uint, len: c_int) -> markertype {
    let p = (*prop).val.val.add(offset as usize); let mut nnotstring=0; let mut nnul=0;
    for i in 0..len { if !isstring(*p.add(i as usize)) { nnotstring+=1; } if *p.add(i as usize)==0 { nnul+=1; } }
    if *p.add((len-1) as usize)==0 && nnotstring==0 && nnul <= len-nnul { if nnul>1 { add_string_markers(prop, offset, len); } TYPE_STRING } else if len % std::mem::size_of::<cell_t>() as c_int == 0 { TYPE_UINT32 } else { TYPE_UINT8 }
}

unsafe fn guess_type_markers(prop: *mut property) {
    let mut m = &mut (*prop).val.markers as *mut *mut marker; let mut offset=0;
    while !(*m).is_null() { if is_type_marker((**m).typ) { return; } if (**m).offset > offset { m=add_marker(m, guess_value_type(prop, offset, (**m).offset-offset as c_uint), offset, std::ptr::null_mut()); offset=(**m).offset; } if (**m).typ==REF_PHANDLE { m=add_marker(m, TYPE_UINT32, offset, std::ptr::null_mut()); offset+=4; } m=&mut (**m).next; }
    if offset < (*prop).val.len { add_marker(m, guess_value_type(prop, offset, (*prop).val.len-offset as usize as c_int), offset, std::ptr::null_mut()); }
}

unsafe fn write_propval(f: *mut FILE, prop: *mut property) {
    let len=(*prop).val.len; if len==0 { fprintf(f,b";\n\0".as_ptr() as *const c_char); return; }
    fprintf(f,b" =\0".as_ptr() as *const c_char); guess_type_markers(prop);
    let mut m=(*prop).val.markers; let mut emit=TYPE_NONE;
    while !m.is_null() { let chunk=if !(*m).next.is_null(){(*m).next.offset-(*m).offset}else{len as c_uint-(*m).offset}; let p=(*prop).val.val.add((*m).offset as usize);
        if is_type_marker((*m).typ) { emit=(*m).typ; fprintf(f,b" %s\0".as_ptr() as *const c_char,delim_start(emit)); } else if (*m).typ==LABEL { fprintf(f,b" %s:\0".as_ptr() as *const c_char,(*m).ref_); }
        if emit!=TYPE_NONE && chunk!=0 { match emit { TYPE_UINT16=>write_propval_int(f,p,chunk as usize,2), TYPE_UINT32=>write_propval_int(f,p,chunk as usize,4), TYPE_UINT64=>write_propval_int(f,p,chunk as usize,8), TYPE_STRING=>write_propval_string(f,p,chunk as usize), _=>write_propval_int(f,p,chunk as usize,1) } }
        if !(*m).next.is_null() { m=(*m).next; } else { break; }
    }
    fprintf(f,b";\n\0".as_ptr() as *const c_char);
}

unsafe fn write_tree_source_node(f: *mut FILE, tree: *mut node, level: c_int) {
    write_prefix(f,level); let mut l=(*tree).labels; while !l.is_null(){fprintf(f,b"%s: \0".as_ptr() as *const c_char,(*l).label);l=(*l).next;}
    if !(*tree).name.is_null() && *(*tree).name!=0 {fprintf(f,b"%s {\n\0".as_ptr() as *const c_char,(*tree).name);} else {fprintf(f,b"/ {\n\0".as_ptr() as *const c_char);}
    let mut p=(*tree).proplist; while !p.is_null(){write_prefix(f,level+1);let mut pl=(*p).labels;while !pl.is_null(){fprintf(f,b"%s: \0".as_ptr() as *const c_char,(*pl).label);pl=(*pl).next;}fprintf(f,b"%s\0".as_ptr() as *const c_char,(*p).name);write_propval(f,p);p=(*p).next;}
    let mut c=(*tree).children;while !c.is_null(){fprintf(f,b"\n\0".as_ptr() as *const c_char);write_tree_source_node(f,c,level+1);c=(*c).next_sibling;}
    write_prefix(f,level);fprintf(f,b"};\n\0".as_ptr() as *const c_char);
}

// The remaining source-level emission routines retain the same external data model.
pub unsafe fn dt_to_source(f: *mut FILE, dti: *mut dt_info) {
    fprintf(f, b"/dts-v1/;\n\0".as_ptr() as *const c_char);
    if (*dti).dtsflags & DTSF_PLUGIN != 0 { fprintf(f, b"/plugin/;\n\0".as_ptr() as *const c_char); }
    fprintf(f, b"\n\0".as_ptr() as *const c_char);
    let mut re=(*dti).reservelist; while !re.is_null() { let mut l=(*re).labels; while !l.is_null() { fprintf(f,b"%s: \0".as_ptr() as *const c_char,(*l).label); l=(*l).next; } fprintf(f,b"/memreserve/\t0x%016llx 0x%016llx;\n\0".as_ptr() as *const c_char,(*re).address as c_ulonglong,(*re).size as c_ulonglong); re=(*re).next; }
    write_tree_source_node(f, (*dti).dt, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
