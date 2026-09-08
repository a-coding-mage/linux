// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2006 David Gibson, IBM Corporation.
 */

unsafe fn fdt_blocks_misordered_(fdt: *const core::ffi::c_void, mem_rsv_size: i32, struct_size: i32) -> i32 {
    ((fdt_off_mem_rsvmap(fdt) < FDT_ALIGN(core::mem::size_of::<fdt_header>() as i32, 8))
        || (fdt_off_dt_struct(fdt) < fdt_off_mem_rsvmap(fdt) + mem_rsv_size)
        || (fdt_off_dt_strings(fdt) < fdt_off_dt_struct(fdt) + struct_size)
        || (fdt_totalsize(fdt) < fdt_off_dt_strings(fdt) + fdt_size_dt_strings(fdt))) as i32
}

unsafe fn fdt_downgrade_version(fdt: *mut core::ffi::c_void) {
    if !can_assume(LATEST) && fdt_version(fdt) > FDT_LAST_SUPPORTED_VERSION {
        fdt_set_version(fdt, FDT_LAST_SUPPORTED_VERSION);
    }
}

unsafe fn fdt_rw_probe_(fdt: *mut core::ffi::c_void) -> i32 {
    if can_assume(VALID_DTB) { return 0; }
    FDT_RO_PROBE!(fdt);
    if !can_assume(LATEST) && fdt_version(fdt) < 17 { return -FDT_ERR_BADVERSION; }
    if fdt_blocks_misordered_(fdt, core::mem::size_of::<fdt_reserve_entry>() as i32,
                              fdt_size_dt_struct(fdt)) != 0 { return -FDT_ERR_BADLAYOUT; }
    fdt_downgrade_version(fdt);
    0
}

#[inline]
unsafe fn fdt_data_size_(fdt: *mut core::ffi::c_void) -> u32 {
    (fdt_off_dt_strings(fdt) + fdt_size_dt_strings(fdt)) as u32
}

unsafe fn fdt_splice_(fdt: *mut core::ffi::c_void, splicepoint: *mut core::ffi::c_void,
                      oldlen: i32, newlen: i32) -> i32 {
    let p = splicepoint as *mut u8;
    let dsize = fdt_data_size_(fdt) as usize;
    let soff = p.offset_from(fdt as *mut u8) as usize;
    if oldlen < 0 || soff + oldlen as usize < soff || soff + oldlen as usize > dsize { return -FDT_ERR_BADOFFSET; }
    if (p as usize) < (fdt as usize) || dsize + newlen as usize < oldlen as usize { return -FDT_ERR_BADOFFSET; }
    if dsize - oldlen as usize + newlen as usize > fdt_totalsize(fdt) as usize { return -FDT_ERR_NOSPACE; }
    memmove(p.add(newlen as usize), p.add(oldlen as usize), dsize - soff - oldlen as usize);
    0
}

unsafe fn fdt_splice_mem_rsv_(fdt: *mut core::ffi::c_void, p: *mut fdt_reserve_entry, oldn: i32, newn: i32) -> i32 {
    let delta = (newn - oldn) * core::mem::size_of::<fdt_reserve_entry>() as i32;
    let err = fdt_splice_(fdt, p as *mut _, oldn * core::mem::size_of::<fdt_reserve_entry>() as i32,
                          newn * core::mem::size_of::<fdt_reserve_entry>() as i32);
    if err != 0 { return err; }
    fdt_set_off_dt_struct(fdt, fdt_off_dt_struct(fdt) + delta);
    fdt_set_off_dt_strings(fdt, fdt_off_dt_strings(fdt) + delta);
    0
}

unsafe fn fdt_splice_struct_(fdt: *mut core::ffi::c_void, p: *mut core::ffi::c_void, oldlen: i32, newlen: i32) -> i32 {
    let delta = newlen - oldlen;
    let err = fdt_splice_(fdt, p, oldlen, newlen);
    if err != 0 { return err; }
    fdt_set_size_dt_struct(fdt, fdt_size_dt_struct(fdt) + delta);
    fdt_set_off_dt_strings(fdt, fdt_off_dt_strings(fdt) + delta);
    0
}

unsafe fn fdt_del_last_string_(fdt: *mut core::ffi::c_void, s: *const i8) {
    fdt_set_size_dt_strings(fdt, fdt_size_dt_strings(fdt) - (strlen(s) as i32 + 1));
}

unsafe fn fdt_splice_string_(fdt: *mut core::ffi::c_void, newlen: i32) -> i32 {
    let p = (fdt as *mut u8).add((fdt_off_dt_strings(fdt) + fdt_size_dt_strings(fdt)) as usize) as *mut _;
    let err = fdt_splice_(fdt, p, 0, newlen);
    if err != 0 { return err; }
    fdt_set_size_dt_strings(fdt, fdt_size_dt_strings(fdt) + newlen);
    0
}

unsafe fn fdt_find_add_string_(fdt: *mut core::ffi::c_void, s: *const i8, slen: i32, allocated: *mut i32) -> i32 {
    let strtab = (fdt as *mut u8).add(fdt_off_dt_strings(fdt) as usize) as *mut i8;
    if !can_assume(NO_ROLLBACK) { *allocated = 0; }
    let p = fdt_find_string_len_(strtab, fdt_size_dt_strings(fdt), s, slen);
    if !p.is_null() { return p.offset_from(strtab) as i32; }
    let new = strtab.add(fdt_size_dt_strings(fdt) as usize);
    let err = fdt_splice_string_(fdt, slen + 1);
    if err != 0 { return err; }
    if !can_assume(NO_ROLLBACK) { *allocated = 1; }
    memcpy(new as *mut _, s as *const _, slen as usize);
    *new.add(slen as usize) = 0;
    new.offset_from(strtab) as i32
}

pub unsafe fn fdt_add_mem_rsv(fdt: *mut core::ffi::c_void, address: u64, size: u64) -> i32 {
    FDT_RW_PROBE!(fdt);
    let re = fdt_mem_rsv_w_(fdt, fdt_num_mem_rsv(fdt));
    let err = fdt_splice_mem_rsv_(fdt, re, 0, 1);
    if err != 0 { return err; }
    (*re).address = cpu_to_fdt64(address); (*re).size = cpu_to_fdt64(size); 0
}

pub unsafe fn fdt_del_mem_rsv(fdt: *mut core::ffi::c_void, n: i32) -> i32 {
    let re = fdt_mem_rsv_w_(fdt, n);
    FDT_RW_PROBE!(fdt);
    if n >= fdt_num_mem_rsv(fdt) { return -FDT_ERR_NOTFOUND; }
    fdt_splice_mem_rsv_(fdt, re, 1, 0)
}

unsafe fn fdt_resize_property_(fdt: *mut core::ffi::c_void, nodeoffset: i32, name: *const i8, namelen: i32, len: i32, prop: *mut *mut fdt_property) -> i32 {
    let mut oldlen = 0; *prop = fdt_get_property_namelen_w(fdt, nodeoffset, name, namelen, &mut oldlen);
    if (*prop).is_null() { return oldlen; }
    let err = fdt_splice_struct_(fdt, (*prop).as_ref().unwrap().data.as_ptr() as *mut _, FDT_TAGALIGN(oldlen), FDT_TAGALIGN(len));
    if err != 0 { return err; } (**prop).len = cpu_to_fdt32(len as u32); 0
}

unsafe fn fdt_add_property_(fdt: *mut core::ffi::c_void, nodeoffset: i32, name: *const i8, namelen: i32, len: i32, prop: *mut *mut fdt_property) -> i32 {
    let nextoffset = fdt_check_node_offset_(fdt, nodeoffset); if nextoffset < 0 { return nextoffset; }
    let mut allocated = 0; let namestroff = fdt_find_add_string_(fdt, name, namelen, &mut allocated); if namestroff < 0 { return namestroff; }
    *prop = fdt_offset_ptr_w_(fdt, nextoffset); let proplen = core::mem::size_of::<fdt_property>() as i32 + FDT_TAGALIGN(len);
    let err = fdt_splice_struct_(fdt, *prop as *mut _, 0, proplen);
    if err != 0 { if !can_assume(NO_ROLLBACK) && allocated != 0 { fdt_del_last_string_(fdt, name); } return err; }
    (**prop).tag = cpu_to_fdt32(FDT_PROP); (**prop).nameoff = cpu_to_fdt32(namestroff as u32); (**prop).len = cpu_to_fdt32(len as u32); 0
}

pub unsafe fn fdt_set_name(fdt: *mut core::ffi::c_void, nodeoffset: i32, name: *const i8) -> i32 {
    FDT_RW_PROBE!(fdt); let mut oldlen = 0; let namep = fdt_get_name(fdt, nodeoffset, &mut oldlen) as *mut i8; if namep.is_null() { return oldlen; }
    let newlen = strlen(name) as i32; let err = fdt_splice_struct_(fdt, namep as *mut _, FDT_TAGALIGN(oldlen + 1), FDT_TAGALIGN(newlen + 1)); if err != 0 { return err; }
    memcpy(namep as *mut _, name as *const _, (newlen + 1) as usize); 0
}

pub unsafe fn fdt_setprop_placeholder_namelen(fdt: *mut core::ffi::c_void, nodeoffset: i32, name: *const i8, namelen: i32, len: i32, prop_data: *mut *mut core::ffi::c_void) -> i32 {
    FDT_RW_PROBE!(fdt); let mut prop = core::ptr::null_mut(); let mut err = fdt_resize_property_(fdt,nodeoffset,name,namelen,len,&mut prop);
    if err == -FDT_ERR_NOTFOUND { err = fdt_add_property_(fdt,nodeoffset,name,namelen,len,&mut prop); } if err != 0 { return err; } *prop_data = (*prop).data.as_mut_ptr() as *mut _; 0
}

pub unsafe fn fdt_setprop_namelen(fdt:*mut core::ffi::c_void,nodeoffset:i32,name:*const i8,namelen:i32,val:*const core::ffi::c_void,len:i32)->i32 { let mut p=core::ptr::null_mut(); let err=fdt_setprop_placeholder_namelen(fdt,nodeoffset,name,namelen,len,&mut p); if err!=0{return err;} if len!=0{memcpy(p,val,len as usize);} 0 }

pub unsafe fn fdt_appendprop(fdt:*mut core::ffi::c_void,nodeoffset:i32,name:*const i8,val:*const core::ffi::c_void,len:i32)->i32 { FDT_RW_PROBE!(fdt); let mut oldlen=0; let mut prop=fdt_get_property_w(fdt,nodeoffset,name,&mut oldlen); if !prop.is_null(){let newlen=len+oldlen;let err=fdt_splice_struct_(fdt,(*prop).data.as_mut_ptr() as *mut _,FDT_TAGALIGN(oldlen),FDT_TAGALIGN(newlen));if err!=0{return err;}(*prop).len=cpu_to_fdt32(newlen as u32);memcpy((*prop).data.as_mut_ptr().add(oldlen as usize) as *mut _,val,len as usize);}else{let err=fdt_add_property_(fdt,nodeoffset,name,strlen(name) as i32,len,&mut prop);if err!=0{return err;}memcpy((*prop).data.as_mut_ptr() as *mut _,val,len as usize);}0 }

pub unsafe fn fdt_delprop(fdt:*mut core::ffi::c_void,nodeoffset:i32,name:*const i8)->i32 { FDT_RW_PROBE!(fdt); let mut len=0; let prop=fdt_get_property_w(fdt,nodeoffset,name,&mut len);if prop.is_null(){return len;} fdt_splice_struct_(fdt,prop as *mut _,core::mem::size_of::<fdt_property>() as i32+FDT_TAGALIGN(len),0) }

pub unsafe fn fdt_add_subnode_namelen(fdt:*mut core::ffi::c_void,parentoffset:i32,name:*const i8,namelen:i32)->i32 { FDT_RW_PROBE!(fdt); let mut offset=fdt_subnode_offset_namelen(fdt,parentoffset,name,namelen);if offset>=0{return -FDT_ERR_EXISTS;}if offset!=-FDT_ERR_NOTFOUND{return offset;}let mut nextoffset=0;let mut tag=fdt_next_tag(fdt,parentoffset,&mut nextoffset);if !can_assume(LIBFDT_FLAWLESS)&&tag!=FDT_BEGIN_NODE{return -FDT_ERR_INTERNAL;}loop{offset=nextoffset;tag=fdt_next_tag(fdt,offset,&mut nextoffset);if tag!=FDT_PROP&&tag!=FDT_NOP{break;}}let nh=fdt_offset_ptr_w_(fdt,offset);let nodelen=core::mem::size_of::<fdt_node_header>() as i32+FDT_TAGALIGN(namelen+1)+FDT_TAGSIZE;let err=fdt_splice_struct_(fdt,nh as *mut _,0,nodelen);if err!=0{return err;}(*nh).tag=cpu_to_fdt32(FDT_BEGIN_NODE);memset((*nh).name.as_mut_ptr() as *mut _,0,FDT_TAGALIGN(namelen+1) as usize);memcpy((*nh).name.as_mut_ptr() as *mut _,name,namelen as usize);let endtag=(nh as *mut u8).add((nodelen-FDT_TAGSIZE) as usize) as *mut u32;*endtag=cpu_to_fdt32(FDT_END_NODE);offset }

pub unsafe fn fdt_add_subnode(fdt:*mut core::ffi::c_void,parentoffset:i32,name:*const i8)->i32 { fdt_add_subnode_namelen(fdt,parentoffset,name,strlen(name) as i32) }

pub unsafe fn fdt_del_node(fdt:*mut core::ffi::c_void,nodeoffset:i32)->i32 { FDT_RW_PROBE!(fdt);let endoffset=fdt_node_end_offset_(fdt,nodeoffset);if endoffset<0{return endoffset;}fdt_splice_struct_(fdt,fdt_offset_ptr_w_(fdt,nodeoffset) as *mut _,endoffset-nodeoffset,0) }

unsafe fn fdt_packblocks_(old:*const i8,new:*mut i8,mem_rsv_size:i32,struct_size:i32,strings_size:i32) { let mem_rsv_off=FDT_ALIGN(core::mem::size_of::<fdt_header>() as i32,8);let struct_off=mem_rsv_off+mem_rsv_size;let strings_off=struct_off+struct_size;memmove(new.add(mem_rsv_off as usize) as *mut _,(old as *mut i8).add(fdt_off_mem_rsvmap(old) as usize) as *const _,mem_rsv_size as usize);fdt_set_off_mem_rsvmap(new as *mut _,mem_rsv_off);memmove(new.add(struct_off as usize) as *mut _,(old as *mut i8).add(fdt_off_dt_struct(old) as usize) as *const _,struct_size as usize);fdt_set_off_dt_struct(new as *mut _,struct_off);fdt_set_size_dt_struct(new as *mut _,struct_size);memmove(new.add(strings_off as usize) as *mut _,(old as *mut i8).add(fdt_off_dt_strings(old) as usize) as *const _,strings_size as usize);fdt_set_off_dt_strings(new as *mut _,strings_off);fdt_set_size_dt_strings(new as *mut _,fdt_size_dt_strings(old)); }

pub unsafe fn fdt_open_into(fdt:*const core::ffi::c_void,buf:*mut core::ffi::c_void,bufsize:i32)->i32 { FDT_RO_PROBE!(fdt);let mem_rsv_size=(fdt_num_mem_rsv(fdt)+1)*core::mem::size_of::<fdt_reserve_entry>() as i32;let struct_size;if can_assume(LATEST)||fdt_version(fdt)>=17{struct_size=fdt_size_dt_struct(fdt);}else if fdt_version(fdt)==16{let mut s=0;while fdt_next_tag(fdt,s,&mut s)!=FDT_END{}if s<0{return s;}struct_size=s;}else{return -FDT_ERR_BADVERSION;}if can_assume(LIBFDT_ORDER)||fdt_blocks_misordered_(fdt,mem_rsv_size,struct_size)==0{let err=fdt_move(fdt,buf,bufsize);if err!=0{return err;}fdt_set_version(buf,17);fdt_set_size_dt_struct(buf,struct_size);fdt_set_totalsize(buf,bufsize);return 0;}let newsize=FDT_ALIGN(core::mem::size_of::<fdt_header>() as i32,8)+mem_rsv_size+struct_size+fdt_size_dt_strings(fdt);if bufsize<newsize{return -FDT_ERR_NOSPACE;}let fdtstart=fdt as *const i8;let fdtend=fdtstart.add(fdt_totalsize(fdt) as usize);let mut tmp=buf as *mut i8;if tmp.add(newsize as usize)>fdtstart as *mut i8&&tmp<fdtend as *mut i8{tmp=fdtend as *mut i8;if tmp.add(newsize as usize)> (buf as *mut i8).add(bufsize as usize){return -FDT_ERR_NOSPACE;}}fdt_packblocks_(fdt,tmp,mem_rsv_size,struct_size,fdt_size_dt_strings(fdt));memmove(buf,tmp as *const _,newsize as usize);fdt_set_magic(buf,FDT_MAGIC);fdt_set_totalsize(buf,bufsize);fdt_set_version(buf,17);fdt_set_last_comp_version(buf,16);fdt_set_boot_cpuid_phys(buf,fdt_boot_cpuid_phys(fdt));0 }

pub unsafe fn fdt_pack(fdt:*mut core::ffi::c_void)->i32 { FDT_RW_PROBE!(fdt);let mem_rsv_size=(fdt_num_mem_rsv(fdt)+1)*core::mem::size_of::<fdt_reserve_entry>() as i32;fdt_packblocks_(fdt as *const _,fdt as *mut _,mem_rsv_size,fdt_size_dt_struct(fdt),fdt_size_dt_strings(fdt));fdt_set_totalsize(fdt,fdt_data_size_(fdt) as i32);0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
