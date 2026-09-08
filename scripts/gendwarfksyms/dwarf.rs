// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Google LLC
 */

// Dependencies supplied by the surrounding project are intentionally not
// redefined here.

const KABI_PREFIX: &[u8] = b"__kabi_";
const KABI_PREFIX_LEN: usize = KABI_PREFIX.len() - 1;
const KABI_RESERVED_PREFIX: &[u8] = b"reserved";
const KABI_RESERVED_PREFIX_LEN: usize = KABI_RESERVED_PREFIX.len() - 1;
const KABI_RENAMED_PREFIX: &[u8] = b"renamed";
const KABI_RENAMED_PREFIX_LEN: usize = KABI_RENAMED_PREFIX.len() - 1;
const KABI_IGNORED_PREFIX: &[u8] = b"ignored";
const KABI_IGNORED_PREFIX_LEN: usize = KABI_IGNORED_PREFIX.len() - 1;

#[inline]
unsafe fn is_kabi_prefix(name: *const libc::c_char) -> bool {
    !name.is_null() && libc::strncmp(name, KABI_PREFIX.as_ptr() as *const libc::c_char, KABI_PREFIX_LEN) == 0
}

#[repr(C)]
enum KabiStatus { KabiNormal = 1, KabiReserved, KabiIgnored }

static mut DO_LINEBREAK: bool = false;
static mut INDENTATION_LEVEL: libc::c_int = 0;

unsafe fn process_linebreak(cache: *mut die, n: libc::c_int) {
    INDENTATION_LEVEL += n;
    DO_LINEBREAK = true;
    die_map_add_linebreak(cache, n);
}

unsafe fn get_flag_attr(die: *mut Dwarf_Die, id: libc::c_uint, value: *mut bool) -> bool {
    let mut da = Dwarf_Attribute::default();
    dwarf_attr(die, id, &mut da) != 0 && dwarf_formflag(&mut da, value) == 0
}
unsafe fn get_udata_attr(die: *mut Dwarf_Die, id: libc::c_uint, value: *mut Dwarf_Word) -> bool {
    let mut da = Dwarf_Attribute::default();
    dwarf_attr(die, id, &mut da) != 0 && dwarf_formudata(&mut da, value) == 0
}
unsafe fn get_ref_die_attr(die: *mut Dwarf_Die, id: libc::c_uint, value: *mut Dwarf_Die) -> bool {
    let mut da = Dwarf_Attribute::default();
    dwarf_attr(die, id, &mut da) != 0 && !dwarf_formref_die(&mut da, value).is_null()
}
unsafe fn get_name_attr(die: *mut Dwarf_Die) -> *const libc::c_char {
    let mut da = Dwarf_Attribute::default();
    if dwarf_attr(die, DW_AT_name, &mut da) != 0 { dwarf_formstring(&mut da) } else { core::ptr::null() }
}
unsafe fn get_linkage_name_attr(die: *mut Dwarf_Die) -> *const libc::c_char {
    let mut da = Dwarf_Attribute::default();
    if dwarf_attr(die, DW_AT_linkage_name, &mut da) != 0 { dwarf_formstring(&mut da) } else { core::ptr::null() }
}
unsafe fn get_symbol_name(die: *mut Dwarf_Die) -> *const libc::c_char {
    let mut name = get_linkage_name_attr(die);
    if name.is_null() { name = get_name_attr(die); }
    name
}

unsafe fn match_export_symbol(state: *mut state, die: *mut Dwarf_Die) -> bool {
    let mut source = die;
    let mut origin = Dwarf_Die::default();
    if get_ref_die_attr(die, DW_AT_abstract_origin, &mut origin) { source = &mut origin; }
    (*state).sym = symbol_get(get_symbol_name(die));
    if (*state).sym.is_null() && source != die { (*state).sym = symbol_get(get_symbol_name(source)); }
    (*state).die = *source;
    !(*state).sym.is_null()
}

static mut SRCFILE_CACHE: cache = cache::default();

unsafe fn is_definition_private(die: *mut Dwarf_Die) -> bool {
    let mut filenum: Dwarf_Word = 0;
    if !get_udata_attr(die, DW_AT_decl_file, &mut filenum) { return false; }
    let mut res = cache_get(&mut SRCFILE_CACHE, filenum);
    if res >= 0 { return res != 0; }
    let mut cudie = Dwarf_Die::default();
    if dwarf_cu_die((*die).cu, &mut cudie, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()) == 0 { error(cstr!("dwarf_cu_die failed: '%s'"), dwarf_errmsg(-1)); }
    let mut files = core::ptr::null_mut();
    if dwarf_getsrcfiles(&mut cudie, &mut files, core::ptr::null_mut()) != 0 { error(cstr!("dwarf_getsrcfiles failed: '%s'"), dwarf_errmsg(-1)); }
    let mut s = dwarf_filesrc(files, filenum, core::ptr::null_mut(), core::ptr::null_mut());
    if s.is_null() { error(cstr!("dwarf_filesrc failed: '%s'"), dwarf_errmsg(-1)); }
    s = libc::strrchr(s, b'.' as libc::c_int);
    res = if !s.is_null() && libc::strcmp(s, cstr!(".c")) == 0 { 1 } else { 0 };
    cache_set(&mut SRCFILE_CACHE, filenum, res);
    res != 0
}

unsafe fn is_kabi_definition(cache: *mut die, die: *mut Dwarf_Die) -> bool {
    let mut value = false;
    if get_flag_attr(die, DW_AT_declaration, &mut value) && value { return false; }
    if kabi_is_declonly((*cache).fqn) { return false; }
    !is_definition_private(die)
}

unsafe fn process(cache: *mut die, mut s: *const libc::c_char) {
    if s.is_null() { s = cstr!("<null>"); }
    if dump_dies && DO_LINEBREAK {
        libc::fputs(cstr!("\n"), libc::stderr);
        for _ in 0..INDENTATION_LEVEL { libc::fputs(cstr!("  "), libc::stderr); }
        DO_LINEBREAK = false;
    }
    if dump_dies { libc::fputs(s, libc::stderr); }
    if !cache.is_null() { die_debug_r(cstr!("cache %p string '%s'"), cache, s); }
    die_map_add_string(cache, s);
}

const MAX_FMT_BUFFER_SIZE: usize = 128;
unsafe fn process_fmt(cache: *mut die, fmt: *const libc::c_char, mut args: ...) {
    let mut buf = [0i8; MAX_FMT_BUFFER_SIZE];
    if vsnprintf(buf.as_mut_ptr(), buf.len(), fmt, args.as_va_list()) >= buf.len() { error(cstr!("vsnprintf overflow: increase MAX_FMT_BUFFER_SIZE")); }
    process(cache, buf.as_ptr());
}

unsafe fn update_fqn(cache: *mut die, die: *mut Dwarf_Die) {
    if (*cache).fqn.is_null() {
        let mut fqn: *mut die = core::ptr::null_mut();
        if __die_map_get((*die).addr as usize, DIE_FQN, &mut fqn) == 0 && !(*fqn).fqn.is_null() && *(*fqn).fqn != 0 { (*cache).fqn = xstrdup((*fqn).fqn); } else { (*cache).fqn = cstr!(""); }
    }
}
unsafe fn process_fqn(cache: *mut die, die: *mut Dwarf_Die) { update_fqn(cache, die); if *(*cache).fqn != 0 { process(cache, cstr!(" ")); } process(cache, (*cache).fqn); }

macro_rules! process_udata_attribute { ($name:ident, $attr:ident) => { unsafe fn $name(cache: *mut die, die: *mut Dwarf_Die) { let mut value = 0; if get_udata_attr(die, $attr, &mut value) { process_fmt(cache, cstr!(concat!(" ", stringify!($attr), "(%llu)")), value); } } }; }
process_udata_attribute!(process_accessibility_attr, DW_AT_accessibility);
process_udata_attribute!(process_alignment_attr, DW_AT_alignment);
process_udata_attribute!(process_bit_size_attr, DW_AT_bit_size);
process_udata_attribute!(process_encoding_attr, DW_AT_encoding);
process_udata_attribute!(process_data_bit_offset_attr, DW_AT_data_bit_offset);
process_udata_attribute!(process_data_member_location_attr, DW_AT_data_member_location);
process_udata_attribute!(process_discr_value_attr, DW_AT_discr_value);

unsafe fn process_byte_size_attr(cache: *mut die, die: *mut Dwarf_Die) { let mut value=0; let mut override_value=0; if get_udata_attr(die,DW_AT_byte_size,&mut value) { if stable && kabi_get_byte_size((*cache).fqn,&mut override_value) { value=override_value as _; } process_fmt(cache,cstr!(" byte_size(%llu)"),value); } }

macro_rules! define_match { ($name:ident, $tag:ident) => { unsafe fn $name(die: *mut Dwarf_Die) -> bool { dwarf_tag(die) == $tag } }; }
define_match!(match_enumerator_type, DW_TAG_enumeration_type);
define_match!(match_formal_parameter_type, DW_TAG_formal_parameter);
define_match!(match_member_type, DW_TAG_member);
define_match!(match_subrange_type, DW_TAG_subrange_type);
pub unsafe fn match_all(_die: *mut Dwarf_Die) -> bool { true }

pub unsafe fn process_die_container(state: *mut state, cache: *mut die, die: *mut Dwarf_Die, func: die_callback_t, matcher: die_match_callback_t) -> libc::c_int {
    let mut current = Dwarf_Die::default();
    if !state.is_null() { (*state).first_list_item=true; }
    let mut res=checkp(dwarf_child(die,&mut current));
    while res==0 { if matcher(&mut current) { res=checkp(func(state,cache,&mut current)); if res!=0 { break; } } res=checkp(dwarf_siblingof(&mut current,&mut current)); }
    if !state.is_null() { (*state).first_list_item=false; } if res!=0 { res } else { 0 }
}

unsafe fn process_type_attr(state:*mut state, cache:*mut die, die:*mut Dwarf_Die) { let mut ty=Dwarf_Die::default(); if get_ref_die_attr(die,DW_AT_type,&mut ty) { check(process_type(state,cache,&mut ty)); } else { process(cache,cstr!("base_type void")); } }
unsafe fn process_list_comma(state:*mut state, cache:*mut die) { if (*state).first_list_item { (*state).first_list_item=false; } else { process(cache,cstr!(" ,")); process_linebreak(cache,0); } }
unsafe fn process_list_type(state:*mut state,cache:*mut die,die:*mut Dwarf_Die,ty:*const libc::c_char) { let mut name=get_name_attr(die); if stable { if is_kabi_prefix(name) { name=core::ptr::null(); } (*state).kabi.orig_name=core::ptr::null(); } process_list_comma(state,cache); process(cache,ty); process_type_attr(state,cache,die); if stable && !(*state).kabi.orig_name.is_null() { name=(*state).kabi.orig_name; } if !name.is_null() { process(cache,cstr!(" ")); process(cache,name); } process_accessibility_attr(cache,die); process_bit_size_attr(cache,die); process_data_bit_offset_attr(cache,die); process_data_member_location_attr(cache,die); }
unsafe fn process_formal_parameter_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process_list_type(s,c,d,cstr!("formal_parameter "));}
unsafe fn process_member_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process_list_type(s,c,d,cstr!("member "));}
unsafe fn process_container_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die,ty:*const libc::c_char){process(c,ty);process_fqn(c,d);process(c,cstr!(" {"));process_linebreak(c,1);process_type_attr(s,c,d);process_linebreak(c,-1);process(c,cstr!("}"));process_byte_size_attr(c,d);process_alignment_attr(c,d);}
macro_rules! container { ($n:ident,$s:literal) => { unsafe fn $n(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process_container_type(s,c,d,cstr!($s));} }; }
container!(process_atomic_type,"atomic_type"); container!(process_const_type,"const_type"); container!(process_immutable_type,"immutable_type"); container!(process_packed_type,"packed_type"); container!(process_pointer_type,"pointer_type"); container!(process_reference_type,"reference_type"); container!(process_restrict_type,"restrict_type"); container!(process_rvalue_reference_type,"rvalue_reference_type"); container!(process_shared_type,"shared_type"); container!(process_template_type_parameter_type,"template_type_parameter_type"); container!(process_volatile_type,"volatile_type"); container!(process_typedef_type,"typedef_type");

unsafe fn process_subrange_type(_s:*mut state,c:*mut die,d:*mut Dwarf_Die){let mut n=0;if get_udata_attr(d,DW_AT_count,&mut n){process_fmt(c,cstr!("[%llu]"),n)}else if get_udata_attr(d,DW_AT_upper_bound,&mut n){process_fmt(c,cstr!("[%llu]"),n+1)}else{process(c,cstr!("[]"));}}
unsafe fn process_array_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process(c,cstr!("array_type"));check(process_die_container(s,c,d,process_type,match_subrange_type));process(c,cstr!(" {"));process_linebreak(c,1);process_type_attr(s,c,d);process_linebreak(c,-1);process(c,cstr!("}"));}
unsafe fn process_subroutine_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process(c,cstr!("subroutine_type ("));process_linebreak(c,1);check(process_die_container(s,c,d,process_type,match_formal_parameter_type));process_linebreak(c,-1);process(c,cstr!(")"));process_linebreak(c,0);process(c,cstr!("-> "));process_type_attr(s,c,d);}
unsafe fn process_variant_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process_list_comma(s,c);process(c,cstr!("variant {"));process_linebreak(c,1);check(process_die_container(s,c,d,process_type,match_member_type));process_linebreak(c,-1);process(c,cstr!("}"));process_discr_value_attr(c,d);}
unsafe fn process_variant_part_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process_list_comma(s,c);process(c,cstr!("variant_part {"));process_linebreak(c,1);check(process_die_container(s,c,d,process_type,match_all));process_linebreak(c,-1);process(c,cstr!("}"));}

unsafe fn get_kabi_status(die:*mut Dwarf_Die,suffix:*mut *const libc::c_char)->KabiStatus{let mut n=get_name_attr(die);if !suffix.is_null(){*suffix=core::ptr::null();}if is_kabi_prefix(n){n=n.add(KABI_PREFIX_LEN);if libc::strncmp(n,KABI_RESERVED_PREFIX.as_ptr() as _,KABI_RESERVED_PREFIX_LEN)==0{return KabiStatus::KabiReserved;}if libc::strncmp(n,KABI_IGNORED_PREFIX.as_ptr() as _,KABI_IGNORED_PREFIX_LEN)==0{return KabiStatus::KabiIgnored;}if libc::strncmp(n,KABI_RENAMED_PREFIX.as_ptr() as _,KABI_RENAMED_PREFIX_LEN)==0{if !suffix.is_null(){*suffix=n.add(KABI_RENAMED_PREFIX_LEN);}return KabiStatus::KabiReserved;}}KabiStatus::KabiNormal}

unsafe fn check_struct_member_kabi_status(s:*mut state,_:*mut die,d:*mut Dwarf_Die)->libc::c_int{assert!(dwarf_tag(d)==DW_TAG_member);let r=get_kabi_status(d,&mut (*s).kabi.orig_name);if matches!(r,KabiStatus::KabiReserved)&&!get_ref_die_attr(d,DW_AT_type,&mut (*s).kabi.placeholder){error(cstr!("structure member missing a type?"));}r as _}
unsafe fn check_union_member_kabi_status(s:*mut state,_:*mut die,d:*mut Dwarf_Die)->libc::c_int{let mut ty=Dwarf_Die::default();assert!(dwarf_tag(d)==DW_TAG_member);if !get_ref_die_attr(d,DW_AT_type,&mut ty){error(cstr!("union member missing a type?"));}let r=get_kabi_status(d,&mut (*s).kabi.orig_name);if matches!(r,KabiStatus::KabiReserved){(*s).kabi.placeholder=ty;}if !matches!(r,KabiStatus::KabiNormal){return r as _;}if dwarf_tag(&mut ty)==DW_TAG_structure_type{let _=checkp(process_die_container(s,core::ptr::null_mut(),&mut ty,check_struct_member_kabi_status,match_member_type));}(*s).kabi.members+=1;if r as i32<=KabiStatus::KabiNormal as i32&&(*s).kabi.members<2{0}else{r as _}}

unsafe fn get_union_kabi_status(die:*mut Dwarf_Die,placeholder:*mut Dwarf_Die,orig:*mut *const libc::c_char)->KabiStatus{if !stable{return KabiStatus::KabiNormal;}let mut s=core::mem::zeroed::<state>();let r=checkp(process_die_container(&mut s,core::ptr::null_mut(),die,check_union_member_kabi_status,match_member_type));if r==KabiStatus::KabiReserved as _{if !placeholder.is_null(){*placeholder=s.kabi.placeholder;}if !orig.is_null(){*orig=s.kabi.orig_name;}}if r==KabiStatus::KabiIgnored as _{KabiStatus::KabiIgnored}else if r==KabiStatus::KabiReserved as _{KabiStatus::KabiReserved}else{KabiStatus::KabiNormal}}
unsafe fn is_kabi_ignored(d:*mut Dwarf_Die)->bool{if !stable{return false;}let mut t=Dwarf_Die::default();if !get_ref_die_attr(d,DW_AT_type,&mut t){error(cstr!("member missing a type?"));}dwarf_tag(&mut t)==DW_TAG_union_type&&checkp(get_union_kabi_status(&mut t,core::ptr::null_mut(),core::ptr::null_mut()))==KabiStatus::KabiIgnored as _}

unsafe fn process_structure_child(s:*mut state,c:*mut die,d:*mut Dwarf_Die)->libc::c_int{match dwarf_tag(d){DW_TAG_member=>if is_kabi_ignored(d){0}else{check(process_type(s,c,d))},DW_TAG_variant_part=>check(process_type(s,c,d)),DW_TAG_class_type|DW_TAG_enumeration_type|DW_TAG_structure_type|DW_TAG_template_type_parameter|DW_TAG_union_type|DW_TAG_subprogram=>0,_=>{error(cstr!("unexpected structure_type child: %x"),dwarf_tag(d));0}}}
unsafe fn process_structure(s:*mut state,c:*mut die,d:*mut Dwarf_Die,ty:*const libc::c_char){process(c,ty);process_fqn(c,d);process(c,cstr!(" {"));process_linebreak(c,1);let expand=(*s).expand.expand&&is_kabi_definition(c,d);if expand{(*s).expand.current_fqn=(*c).fqn;check(process_die_container(s,c,d,process_structure_child,match_all));}process_linebreak(c,-1);process(c,cstr!("}"));if expand{process_byte_size_attr(c,d);process_alignment_attr(c,d);}}
unsafe fn process_class_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process_structure(s,c,d,cstr!("class_type"));}
unsafe fn process_structure_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process_structure(s,c,d,cstr!("structure_type"));}
unsafe fn process_union_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){let mut p=Dwarf_Die::default();let r=checkp(get_union_kabi_status(d,&mut p,&mut (*s).kabi.orig_name));if r==KabiStatus::KabiReserved as _{check(process_type(s,c,&mut p));}if r>KabiStatus::KabiNormal as _{return;}process_structure(s,c,d,cstr!("union_type"));}
unsafe fn process_enumerator_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){let mut over=false;let mut ov=0;let mut value=0;if stable{update_fqn(c,d);if kabi_is_enumerator_ignored((*s).expand.current_fqn,(*c).fqn){return;}over=kabi_get_enumerator_value((*s).expand.current_fqn,(*c).fqn,&mut ov);value=ov;}process_list_comma(s,c);process(c,cstr!("enumerator"));process_fqn(c,d);if over||get_udata_attr(d,DW_AT_const_value,&mut value){process(c,cstr!(" = "));process_fmt(c,cstr!("%llu"),value);}}
unsafe fn process_enumeration_type(s:*mut state,c:*mut die,d:*mut Dwarf_Die){process_structure(s,c,d,cstr!("enumeration_type"));}
unsafe fn process_base_type(_s:*mut state,c:*mut die,d:*mut Dwarf_Die){process(c,cstr!("base_type"));process_fqn(c,d);process_byte_size_attr(c,d);process_encoding_attr(c,d);process_alignment_attr(c,d);}
unsafe fn process_unspecified_type(_s:*mut state,c:*mut die,_d:*mut Dwarf_Die){process(c,cstr!("unspecified_type"));}

unsafe fn process_cached(s:*mut state,c:*mut die,d:*mut Dwarf_Die){let mut df=(*c).fragments.next;while df!=&mut (*c).fragments as *mut _ as *mut die_fragment{match (*df).type_{FRAGMENT_STRING=>process(core::ptr::null_mut(),(*df).data.str_),FRAGMENT_LINEBREAK=>process_linebreak(core::ptr::null_mut(),(*df).data.linebreak),FRAGMENT_DIE=>{let mut child=Dwarf_Die::default();if dwarf_die_addr_die(dwarf_cu_getdwarf((*d).cu),(*df).data.addr as _,&mut child)==0{error(cstr!("dwarf_die_addr_die failed"));}check(process_type(s,core::ptr::null_mut(),&mut child));},_=>error(cstr!("empty die_fragment"))}df=(*df).list.next;}}
unsafe fn state_init(s:*mut state){(*s).expand.expand=true;(*s).expand.current_fqn=core::ptr::null();cache_init(&mut (*s).expansion_cache);}
unsafe fn expansion_state_restore(s:*mut expansion_state,saved:*mut expansion_state){(*s)=(*saved);}
unsafe fn expansion_state_save(s:*mut expansion_state,saved:*mut expansion_state){*saved=*s;}
unsafe fn is_expanded_type(tag:libc::c_int)->bool{tag==DW_TAG_class_type||tag==DW_TAG_structure_type||tag==DW_TAG_union_type||tag==DW_TAG_enumeration_type}

unsafe fn process_type(s:*mut state,parent:*mut die,d:*mut Dwarf_Die)->libc::c_int{let mut want=DIE_COMPLETE;let mut saved=core::mem::zeroed::<expansion_state>();expansion_state_save(&mut (*s).expand,&mut saved);let tag=dwarf_tag(d);if is_expanded_type(tag){if cache_was_expanded(&mut (*s).expansion_cache,(*d).addr){(*s).expand.expand=false;}if (*s).expand.expand{cache_mark_expanded(&mut (*s).expansion_cache,(*d).addr);}else{want=DIE_UNEXPANDED;}}let c=die_map_get(d,want);if (*c).state==want{process_cached(s,c,d);die_map_add_die(parent,c);expansion_state_restore(&mut (*s).expand,&mut saved);return 0;}match tag{DW_TAG_atomic_type=>process_atomic_type(s,c,d),DW_TAG_const_type=>process_const_type(s,c,d),DW_TAG_immutable_type=>process_immutable_type(s,c,d),DW_TAG_packed_type=>process_packed_type(s,c,d),DW_TAG_pointer_type=>process_pointer_type(s,c,d),DW_TAG_reference_type=>process_reference_type(s,c,d),DW_TAG_restrict_type=>process_restrict_type(s,c,d),DW_TAG_rvalue_reference_type=>process_rvalue_reference_type(s,c,d),DW_TAG_shared_type=>process_shared_type(s,c,d),DW_TAG_volatile_type=>process_volatile_type(s,c,d),DW_TAG_class_type=>process_class_type(s,c,d),DW_TAG_structure_type=>process_structure_type(s,c,d),DW_TAG_union_type=>process_union_type(s,c,d),DW_TAG_enumeration_type=>process_enumeration_type(s,c,d),DW_TAG_enumerator=>process_enumerator_type(s,c,d),DW_TAG_formal_parameter=>process_formal_parameter_type(s,c,d),DW_TAG_member=>process_member_type(s,c,d),DW_TAG_subrange_type=>process_subrange_type(s,c,d),DW_TAG_template_type_parameter=>process_template_type_parameter_type(s,c,d),DW_TAG_variant=>process_variant_type(s,c,d),DW_TAG_variant_part=>process_variant_part_type(s,c,d),DW_TAG_array_type=>process_array_type(s,c,d),DW_TAG_base_type=>process_base_type(s,c,d),DW_TAG_subroutine_type=>process_subroutine_type(s,c,d),DW_TAG_typedef=>process_typedef_type(s,c,d),DW_TAG_unspecified_type=>process_unspecified_type(s,c,d),_=>error(cstr!("unexpected type: %x"),tag)}(*c).tag=tag;(*c).state=want;die_map_add_die(parent,c);expansion_state_restore(&mut (*s).expand,&mut saved);0}

unsafe fn get_symbol_cache(_s:*mut state,d:*mut Dwarf_Die)->*mut die{let c=die_map_get(d,DIE_SYMBOL);if (*c).state!=DIE_INCOMPLETE{return core::ptr::null_mut();}(*c).tag=dwarf_tag(d);c}
unsafe fn process_symbol(s:*mut state,d:*mut Dwarf_Die,f:die_callback_t){symbol_set_die((*s).sym,d);let c=get_symbol_cache(s,d);if c.is_null(){return;}check(f(s,c,d));(*c).state=DIE_SYMBOL;}
unsafe fn process_subprogram(s:*mut state,d:*mut Dwarf_Die){process_symbol(s,d,process_subprogram_inner)}
unsafe fn process_subprogram_inner(s:*mut state,c:*mut die,d:*mut Dwarf_Die)->libc::c_int{process_subroutine_type(s,c,d);0}
unsafe fn process_variable(s:*mut state,d:*mut Dwarf_Die){process_symbol(s,d,process_variable_inner)}
unsafe fn process_variable_inner(s:*mut state,c:*mut die,d:*mut Dwarf_Die)->libc::c_int{process(c,cstr!("variable "));process_type_attr(s,c,d);0}
unsafe fn save_symbol_ptr(s:*mut state){let mut p=Dwarf_Die::default();let mut t=Dwarf_Die::default();if !get_ref_die_attr(&mut (*s).die,DW_AT_type,&mut p)||dwarf_tag(&mut p)!=DW_TAG_pointer_type{error(cstr!("%s must be a pointer type!"),get_symbol_name(&mut (*s).die));}if !get_ref_die_attr(&mut p,DW_AT_type,&mut t){error(cstr!("%s pointer missing a type attribute?"),get_symbol_name(&mut (*s).die));}if dwarf_tag(&mut t)==DW_TAG_subroutine_type{symbol_set_ptr((*s).sym,&mut t);}else{symbol_set_ptr((*s).sym,&mut p);}}
unsafe fn process_exported_symbols(_s:*mut state,c:*mut die,d:*mut Dwarf_Die)->libc::c_int{match dwarf_tag(d){DW_TAG_namespace|DW_TAG_class_type|DW_TAG_structure_type=>check(process_die_container(core::ptr::null_mut(),c,d,process_exported_symbols,match_all)),DW_TAG_subprogram|DW_TAG_variable=>{let mut s=core::mem::zeroed::<state>();if !match_export_symbol(&mut s,d){return 0;}state_init(&mut s);if is_symbol_ptr(get_symbol_name(&mut s.die)){save_symbol_ptr(&mut s);}else if dwarf_tag(d)==DW_TAG_subprogram{process_subprogram(&mut s,&mut s.die);}else{process_variable(&mut s,&mut s.die);}cache_free(&mut s.expansion_cache);0},_=>0}}
unsafe fn process_symbol_ptr(sym:*mut symbol,arg:*mut libc::c_void){if (*sym).state!=SYMBOL_UNPROCESSED||(*sym).ptr_die_addr==0{return;}let mut s=core::mem::zeroed::<state>();state_init(&mut s);s.sym=sym;if dwarf_die_addr_die(arg as *mut Dwarf,(*sym).ptr_die_addr as _,&mut s.die)==0{error(cstr!("dwarf_die_addr_die failed for symbol ptr: '%s'"),(*sym).name);}if dwarf_tag(&mut s.die)==DW_TAG_subroutine_type{process_subprogram(&mut s,&mut s.die);}else{process_variable(&mut s,&mut s.die);}cache_free(&mut s.expansion_cache);}
unsafe fn resolve_fqns(parent:*mut state,_:*mut die,d:*mut Dwarf_Die)->libc::c_int{let mut c=core::ptr::null_mut();if __die_map_get((*d).addr as usize,DIE_FQN,&mut c)!=0{return 0;}let tag=dwarf_tag(d);let use_prefix=tag==DW_TAG_namespace||tag==DW_TAG_class_type||tag==DW_TAG_structure_type;let name=get_name_attr(d);let mut s=core::mem::zeroed::<state>();s.expand.current_fqn=core::ptr::null();let mut fqn=cstr!("");let mut prefix=core::ptr::null_mut();if !parent.is_null()&&!(*parent).expand.current_fqn.is_null()&&(use_prefix||!name.is_null()){if asprintf(&mut prefix,cstr!("%s::%s"),(*parent).expand.current_fqn,if name.is_null(){cstr!("<anonymous>")}else{name})<0{error(cstr!("asprintf failed"));}if use_prefix{s.expand.current_fqn=prefix;}if !name.is_null(){fqn=prefix;prefix=core::ptr::null_mut();}}else if !name.is_null(){fqn=xstrdup(name);if use_prefix{s.expand.current_fqn=fqn;}}if *fqn!=0{c=die_map_get(d,DIE_FQN);(*c).fqn=fqn;(*c).state=DIE_FQN;}check(process_die_container(&mut s,core::ptr::null_mut(),d,resolve_fqns,match_all));if !prefix.is_null(){libc::free(prefix as _);}0}
pub unsafe fn process_cu(cudie:*mut Dwarf_Die){check(process_die_container(core::ptr::null_mut(),core::ptr::null_mut(),cudie,resolve_fqns,match_all));check(process_die_container(core::ptr::null_mut(),core::ptr::null_mut(),cudie,process_exported_symbols,match_all));symbol_for_each(process_symbol_ptr,dwarf_cu_getdwarf((*cudie).cu) as _);cache_free(&mut SRCFILE_CACHE);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
