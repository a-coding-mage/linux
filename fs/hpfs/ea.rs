// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/ea.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  handling extended attributes
 */

// Dependency declarations supplied by hpfs_fn.h remain external to this translation unit.

pub unsafe fn hpfs_ea_ext_remove(s: *mut super_block, a: secno, ano: ::std::os::raw::c_int, len: ::std::os::raw::c_uint) {
    let mut pos: ::std::os::raw::c_uint = 0;
    while pos < len {
        let mut ex = [0i8; 4 + 255 + 1 + 8];
        let ea = ex.as_mut_ptr() as *mut extended_attribute;
        if pos + 4 > len {
            hpfs_error(s, c"EAs don't end correctly, %s %08x, len %08x", if ano != 0 { c"anode" } else { c"sectors" }, a, len);
            return;
        }
        if hpfs_ea_read(s, a, ano, pos, 4, ex.as_mut_ptr()) != 0 { return; }
        if ea_indirect(ea) != 0 {
            if ea_valuelen(ea) != 8 { hpfs_error(s, c"ea_indirect(ea) set while ea->valuelen!=8, %s %08x, pos %08x", if ano != 0 { c"anode" } else { c"sectors" }, a, pos); return; }
            if hpfs_ea_read(s, a, ano, pos + 4, (*ea).namelen as _ + 9, ex.as_mut_ptr().add(4)) != 0 { return; }
            hpfs_ea_remove(s, ea_sec(ea), ea_in_anode(ea), ea_len(ea));
        }
        pos += (*ea).namelen as ::std::os::raw::c_uint + ea_valuelen(ea) as ::std::os::raw::c_uint + 5;
    }
    if ano == 0 { hpfs_free_sectors(s, a, (len + 511) >> 9); }
    else {
        let mut bh: *mut buffer_head = ::std::ptr::null_mut();
        let anode = hpfs_map_anode(s, a, &mut bh);
        if !anode.is_null() { hpfs_remove_btree(s, GET_BTREE_PTR!(&mut (*anode).btree)); brelse(bh); hpfs_free_sectors(s, a, 1); }
    }
}

unsafe fn get_indirect_ea(s: *mut super_block, ano: ::std::os::raw::c_int, a: secno, size: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char {
    let ret = kmalloc((size + 1) as _, GFP_NOFS);
    if ret.is_null() { pr_err(c"out of memory for EA\n"); return ::std::ptr::null_mut(); }
    if hpfs_ea_read(s, a, ano, 0, size as _, ret) != 0 { kfree(ret); return ::std::ptr::null_mut(); }
    *ret.add(size as usize) = 0;
    ret
}

unsafe fn set_indirect_ea(s: *mut super_block, ano: ::std::os::raw::c_int, a: secno, data: *const ::std::os::raw::c_char, size: ::std::os::raw::c_int) { hpfs_ea_write(s, a, ano, 0, size as _, data); }

pub unsafe fn hpfs_read_ea(s: *mut super_block, fnode: *mut fnode, key: *mut ::std::os::raw::c_char, buf: *mut ::std::os::raw::c_char, size: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    let mut pos: ::std::os::raw::c_uint;
    let mut ano: ::std::os::raw::c_int;
    let mut len: ::std::os::raw::c_int;
    let mut a: secno;
    let mut ex = [0i8; 4 + 255 + 1 + 8];
    let mut ea: *mut extended_attribute;
    let ea_end = fnode_end_ea(fnode);
    ea = fnode_ea(fnode);
    while ea < ea_end { if strcmp((*ea).name.as_ptr(), key) == 0 { if ea_indirect(ea) != 0 { break; } if ea_valuelen(ea) >= size as _ { return -EINVAL; } memcpy(buf, ea_data(ea), ea_valuelen(ea) as _); *buf.add(ea_valuelen(ea) as usize)=0; return 0; } ea = next_ea(ea); }
    a = le32_to_cpu((*fnode).ea_secno); len = le32_to_cpu((*fnode).ea_size_l) as _; ano = fnode_in_anode(fnode); pos = 0;
    while pos < len as _ { ea = ex.as_mut_ptr() as *mut extended_attribute; if pos + 4 > len as _ { hpfs_error(s,c"EAs don't end correctly, %s %08x, len %08x",if ano!=0{c"anode"}else{c"sectors"},a,len); return -EIO; } if hpfs_ea_read(s,a,ano,pos,4,ex.as_mut_ptr())!=0{return -EIO;} if hpfs_ea_read(s,a,ano,pos+4,(*ea).namelen as _+1+(if ea_indirect(ea)!=0{8}else{0}),ex.as_mut_ptr().add(4))!=0{return -EIO;} if strcmp((*ea).name.as_ptr(),key)==0 { if ea_indirect(ea)!=0 { break; } if ea_valuelen(ea)>=size as _ {return -EINVAL;} if hpfs_ea_read(s,a,ano,pos+4+(*ea).namelen as _+1,ea_valuelen(ea),buf)!=0{return -EIO;} *buf.add(ea_valuelen(ea) as usize)=0; return 0; } pos += (*ea).namelen as u32 + ea_valuelen(ea) as u32 + 5; }
    if ea_indirect(ea)==0 { return -ENOENT; }
    if ea_len(ea)>=size as _ {return -EINVAL;} if hpfs_ea_read(s,ea_sec(ea),ea_in_anode(ea),0,ea_len(ea),buf)!=0{return -EIO;} *buf.add(ea_len(ea) as usize)=0; 0
}

pub unsafe fn hpfs_get_ea(s: *mut super_block, fnode: *mut fnode, key: *mut ::std::os::raw::c_char, size: *mut ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char {
    let mut pos: u32; let mut ano: ::std::os::raw::c_int; let mut len: ::std::os::raw::c_int; let mut a: secno; let mut ret: *mut ::std::os::raw::c_char; let mut ea: *mut extended_attribute; let end=fnode_end_ea(fnode);
    ea=fnode_ea(fnode); while ea<end { if strcmp((*ea).name.as_ptr(),key)==0 { if ea_indirect(ea)!=0 {*size=ea_len(ea);return get_indirect_ea(s,ea_in_anode(ea),ea_sec(ea),*size);} ret=kmalloc((ea_valuelen(ea)+1) as _,GFP_NOFS); if ret.is_null(){pr_err(c"out of memory for EA\n");return ::std::ptr::null_mut();} *size=ea_valuelen(ea) as _; memcpy(ret,ea_data(ea),ea_valuelen(ea) as _);*ret.add(*size as usize)=0;return ret;}ea=next_ea(ea);}
    a=le32_to_cpu((*fnode).ea_secno);len=le32_to_cpu((*fnode).ea_size_l) as _;ano=fnode_in_anode(fnode);pos=0; while pos<len as u32 {let mut ex=[0i8;4+255+1+8];ea=ex.as_mut_ptr() as *mut extended_attribute;if pos+4>len as u32{return ::std::ptr::null_mut();}if hpfs_ea_read(s,a,ano,pos,4,ex.as_mut_ptr())!=0{return ::std::ptr::null_mut();}if hpfs_ea_read(s,a,ano,pos+4,(*ea).namelen as _+1+(if ea_indirect(ea)!=0{8}else{0}),ex.as_mut_ptr().add(4))!=0{return ::std::ptr::null_mut();}if strcmp((*ea).name.as_ptr(),key)==0{if ea_indirect(ea)!=0{*size=ea_len(ea);return get_indirect_ea(s,ea_in_anode(ea),ea_sec(ea),*size);}ret=kmalloc((ea_valuelen(ea)+1) as _,GFP_NOFS);if ret.is_null(){pr_err(c"out of memory for EA\n");return ::std::ptr::null_mut();}*size=ea_valuelen(ea) as _;if hpfs_ea_read(s,a,ano,pos+4+(*ea).namelen as _+1,ea_valuelen(ea),ret)!=0{kfree(ret);return ::std::ptr::null_mut();}*ret.add(*size as usize)=0;return ret;}pos+=(*ea).namelen as u32+ea_valuelen(ea) as u32+5;}::std::ptr::null_mut()
}

/* Update or create extended attribute 'key' with value 'data'. */
pub unsafe fn hpfs_set_ea(inode: *mut inode, fnode: *mut fnode, key: *const ::std::os::raw::c_char, data: *const ::std::os::raw::c_char, size: ::std::os::raw::c_int) {
    let fno = (*inode).i_ino; let s = (*inode).i_sb; let mut pos: u32; let mut ano: ::std::os::raw::c_int; let mut len: ::std::os::raw::c_int; let mut a: secno; let mut h=[0u8;4]; let mut ea: *mut extended_attribute; let end=fnode_end_ea(fnode);
    ea=fnode_ea(fnode); while ea<end { if strcmp((*ea).name.as_ptr(),key)==0 { if ea_indirect(ea)!=0 {if ea_len(ea)==size as _{set_indirect_ea(s,ea_in_anode(ea),ea_sec(ea),data,size);}} else if ea_valuelen(ea)==size as _ {memcpy(ea_data(ea),data,size as _);} return;} ea=next_ea(ea); }
    a=le32_to_cpu((*fnode).ea_secno);len=le32_to_cpu((*fnode).ea_size_l) as _;ano=fnode_in_anode(fnode);pos=0;
    while pos<len as u32 {let mut ex=[0i8;4+255+1+8];ea=ex.as_mut_ptr() as *mut extended_attribute;if pos+4>len as u32{return;}if hpfs_ea_read(s,a,ano,pos,4,ex.as_mut_ptr())!=0{return;}if hpfs_ea_read(s,a,ano,pos+4,(*ea).namelen as _+1+(if ea_indirect(ea)!=0{8}else{0}),ex.as_mut_ptr().add(4))!=0{return;}if strcmp((*ea).name.as_ptr(),key)==0{if ea_indirect(ea)!=0{if ea_len(ea)==size as _{set_indirect_ea(s,ea_in_anode(ea),ea_sec(ea),data,size);}}else if ea_valuelen(ea)==size as _{hpfs_ea_write(s,a,ano,pos+4+(*ea).namelen as _+1,size as _,data);}return;}pos+=(*ea).namelen as u32+ea_valuelen(ea) as u32+5;}
    if le16_to_cpu((*fnode).ea_offs)==0 {(*fnode).ea_offs=cpu_to_le16(0xc4);}
    if le16_to_cpu((*fnode).ea_offs)<0xc4 || le16_to_cpu((*fnode).ea_offs)+le16_to_cpu((*fnode).acl_size_s)+le16_to_cpu((*fnode).ea_size_s)>0x200{return;}
    if (le16_to_cpu((*fnode).ea_size_s)!=0 || le32_to_cpu((*fnode).ea_size_l)==0) && le16_to_cpu((*fnode).ea_offs)+le16_to_cpu((*fnode).acl_size_s)+le16_to_cpu((*fnode).ea_size_s)+strlen(key) as u16+size as u16+5<=0x200 {ea=fnode_end_ea(fnode);*(ea as *mut i8)=0;(*ea).namelen=strlen(key) as _;(*ea).valuelen_lo=size as _;(*ea).valuelen_hi=(size>>8) as _;strcpy((*ea).name.as_mut_ptr(),key);memcpy(ea_data(ea),data,size as _);(*fnode).ea_size_s=cpu_to_le16(le16_to_cpu((*fnode).ea_size_s)+strlen(key) as u16+size as u16+5);hpfs_i(inode).i_ea_size+=5+strlen(key) as u64+size as u64;return;}
    if le16_to_cpu((*fnode).ea_size_s)!=0 && le32_to_cpu((*fnode).ea_size_l)==0 {let n=hpfs_alloc_sector(s,fno,1,0);if n==0{return;}let mut bh=::std::ptr::null_mut();let p=hpfs_get_sector(s,n,&mut bh);if p.is_null(){hpfs_free_sectors(s,n,1);return;}memcpy(p,fnode_ea(fnode),le16_to_cpu((*fnode).ea_size_s) as _);(*fnode).ea_size_l=cpu_to_le32(le16_to_cpu((*fnode).ea_size_s) as _);(*fnode).ea_size_s=cpu_to_le16(0);(*fnode).ea_secno=cpu_to_le32(n);(*fnode).flags&=!FNODE_anode;mark_buffer_dirty(bh);brelse(bh);}
    pos=le32_to_cpu((*fnode).ea_size_l)+5+strlen(key) as u32+size as u32;len=((le32_to_cpu((*fnode).ea_size_l)+511)>>9) as _;if pos>=30000{return;}
    while ((pos+511)>>9)>len as u32 {if len==0{let q=hpfs_alloc_sector(s,fno,1,0);if q==0{return;}(*fnode).ea_secno=cpu_to_le32(q);(*fnode).flags&=!FNODE_anode;len+=1;}else if !fnode_in_anode(fnode){if hpfs_alloc_if_possible(s,le32_to_cpu((*fnode).ea_secno)+len as u32)!=0{len+=1;}else{return;}}if fnode_in_anode(fnode)!=0{if hpfs_add_sector_to_btree(s,le32_to_cpu((*fnode).ea_secno),0,len as _)==-1{return;}len+=1;}}
    h[0]=0;h[1]=strlen(key) as _;h[2]=(size&0xff) as u8;h[3]=(size>>8) as u8;if hpfs_ea_write(s,le32_to_cpu((*fnode).ea_secno),fnode_in_anode(fnode),le32_to_cpu((*fnode).ea_size_l),4,h.as_ptr())!=0{return;}if hpfs_ea_write(s,le32_to_cpu((*fnode).ea_secno),fnode_in_anode(fnode),le32_to_cpu((*fnode).ea_size_l)+4,h[1] as _,key)!=0{return;}if hpfs_ea_write(s,le32_to_cpu((*fnode).ea_secno),fnode_in_anode(fnode),le32_to_cpu((*fnode).ea_size_l)+5+h[1] as u32,size as _,data)!=0{return;}(*fnode).ea_size_l=cpu_to_le32(pos);hpfs_i(inode).i_ea_size+=5+strlen(key) as u64+size as u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
