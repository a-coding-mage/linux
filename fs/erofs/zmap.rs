// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of zmap.c. External kernel symbols are supplied by other units. */

#[repr(C)]
pub struct z_erofs_maprecorder {
    pub inode: *mut inode,
    pub map: *mut erofs_map_blocks,
    pub lcn: u64,
    pub r#type: u8,
    pub headtype: u8,
    pub clusterofs: u16,
    pub delta: [u16; 2],
    pub pblk: erofs_blk_t,
    pub nextpackoff: erofs_off_t,
    pub compressedblks: i32,
    pub partialref: bool,
    pub in_mbox: bool,
}

unsafe fn z_erofs_load_full_lcluster(m: *mut z_erofs_maprecorder, lcn: u64) -> i32 {
    let inode = (*m).inode; let vi = EROFS_I(inode);
    let pos = Z_EROFS_FULL_INDEX_START(erofs_iloc(inode) + (*vi).inode_isize as u64 + (*vi).xattr_isize as u64) + lcn * core::mem::size_of::<z_erofs_lcluster_index>() as u64;
    let di = erofs_read_metabuf(&mut (*(*m).map).buf, (*inode).i_sb, pos, (*m).in_mbox);
    if IS_ERR(di) { return PTR_ERR(di); }
    (*m).lcn = lcn; (*m).nextpackoff = pos + core::mem::size_of::<z_erofs_lcluster_index>() as u64;
    let advise = le16_to_cpu((*di).di_advise);
    (*m).r#type = advise & Z_EROFS_LI_LCLUSTER_TYPE_MASK;
    if (*m).r#type == Z_EROFS_LCLUSTER_TYPE_NONHEAD {
        (*m).clusterofs = 1u16 << (*vi).z_lclusterbits;
        (*m).delta[0] = le16_to_cpu((*di).di_u.delta[0]);
        if (*m).delta[0] & Z_EROFS_LI_D0_CBLKCNT != 0 {
            if (*vi).z_advise & (Z_EROFS_ADVISE_BIG_PCLUSTER_1 | Z_EROFS_ADVISE_BIG_PCLUSTER_2) == 0 { DBG_BUGON(1); return -EFSCORRUPTED; }
            (*m).compressedblks = ((*m).delta[0] & !Z_EROFS_LI_D0_CBLKCNT) as i32; (*m).delta[0] = 1;
        }
        (*m).delta[1] = le16_to_cpu((*di).di_u.delta[1]);
    } else {
        (*m).partialref = advise & Z_EROFS_LI_PARTIAL_REF != 0; (*m).clusterofs = le16_to_cpu((*di).di_clusterofs);
        if advise & Z_EROFS_LI_HOLE != 0 { (*m).compressedblks = 0; (*m).pblk = EROFS_NULL_ADDR; } else { (*m).pblk = le32_to_cpu((*di).di_u.blkaddr); }
    } 0
}

unsafe fn decode_compactedbits(lobits: u32, input: *mut u8, pos: u32, typ: *mut u8) -> u32 {
    let v = get_unaligned_le32(input.add((pos / 8) as usize)) >> (pos & 7);
    let lo = v & ((1u32 << lobits) - 1); *typ = ((v >> lobits) & 3) as u8; lo
}

unsafe fn get_compacted_la_distance(lobits: u32, encodebits: u32, vcnt: u32, input: *mut u8, mut i: i32) -> i32 {
    DBG_BUGON(i >= vcnt as i32); let mut lo = 0; let mut d1 = 0; let mut typ = 0;
    loop { lo = decode_compactedbits(lobits, input, encodebits * i as u32, &mut typ); if typ != Z_EROFS_LCLUSTER_TYPE_NONHEAD { return d1; } d1 += 1; i += 1; if i >= vcnt as i32 { break; } }
    if lo & Z_EROFS_LI_D0_CBLKCNT == 0 { d1 += lo as i32 - 1; } d1
}

unsafe fn z_erofs_load_compact_lcluster(m: *mut z_erofs_maprecorder, mut lcn: u64, lookahead: bool) -> i32 {
    let inode=(*m).inode; let vi=EROFS_I(inode); let ebase=Z_EROFS_MAP_HEADER_END(erofs_iloc(inode)+(*vi).inode_isize as u64+(*vi).xattr_isize as u64); let bits=(*vi).z_lclusterbits; let total=erofs_iblks(inode); if lcn>=total || bits>14{return -EINVAL;}
    (*m).lcn=lcn; let initial=((32-ebase%32)/4)&7; let mut two=0; if (*vi).z_advise&Z_EROFS_ADVISE_COMPACTED_2B!=0 && initial<total {two=rounddown(total-initial,16);}
    let mut pos=ebase; let mut shift=2; if lcn>=initial {pos+=initial*4;lcn-=initial;if lcn<two {shift=1;}else{pos+=two*2;lcn-=two;}} pos+=lcn*(1<<shift);
    let vcnt=if (1<<shift)==4 && bits<=14 {2} else if (1<<shift)==2 && bits<=12 {16} else{return -EOPNOTSUPP;};
    let input=erofs_read_metabuf(&mut (*(*m).map).buf,(*inode).i_sb,pos,(*m).in_mbox); if IS_ERR(input){return PTR_ERR(input);} (*m).nextpackoff=round_down(pos,(vcnt<<shift))+(vcnt<<shift);
    let lobits=max(bits,ilog2(Z_EROFS_LI_D0_CBLKCNT)+1); let encode=((vcnt<<shift)-core::mem::size_of::<u32>() as u32)*8/vcnt; let bytes=pos&((vcnt<<shift)-1); let input=input.sub(bytes as usize); let mut i=(bytes>>shift) as i32; let mut typ=0; let mut lo=decode_compactedbits(lobits,input,encode*i as u32,&mut typ); (*m).r#type=typ;
    if typ==Z_EROFS_LCLUSTER_TYPE_NONHEAD { (*m).clusterofs=1<<bits; if lookahead {(*m).delta[1]=get_compacted_la_distance(lobits,encode,vcnt,input,i) as u16;} if lo&Z_EROFS_LI_D0_CBLKCNT!=0 {if (*vi).z_advise&Z_EROFS_ADVISE_BIG_PCLUSTER_1==0{DBG_BUGON(1);return -EFSCORRUPTED;}(*m).compressedblks=(lo&!Z_EROFS_LI_D0_CBLKCNT) as i32;(*m).delta[0]=1;return 0;} if i+1!=vcnt as i32 {(*m).delta[0]=lo as u16;return 0;} lo=decode_compactedbits(lobits,input,encode*(i-1) as u32,&mut typ);if typ!=Z_EROFS_LCLUSTER_TYPE_NONHEAD{lo=0;}else if lo&Z_EROFS_LI_D0_CBLKCNT!=0{lo=1;}(*m).delta[0]=lo as u16+1;return 0; }
    (*m).clusterofs=lo as u16;(*m).delta[0]=0;let big=(*vi).z_advise&Z_EROFS_ADVISE_BIG_PCLUSTER_1!=0;let mut nblk=if big{0}else{1}; while i>0 {i-=1;lo=decode_compactedbits(lobits,input,encode*i as u32,&mut typ);if typ==Z_EROFS_LCLUSTER_TYPE_NONHEAD {if big {if lo&Z_EROFS_LI_D0_CBLKCNT!=0{i-=1;nblk+=lo&!Z_EROFS_LI_D0_CBLKCNT;continue;}if lo<=1{DBG_BUGON(1);return -EFSCORRUPTED;}i-=lo as i32-2;continue;}i-=lo as i32;}if i>=0{nblk+=1;}} let tail=input.add((vcnt<<shift) as usize-core::mem::size_of::<u32>());(*m).pblk=le32_to_cpu(*(tail as *const __le32)) + nblk;0
}

unsafe fn z_erofs_load_lcluster_from_disk(m:*mut z_erofs_maprecorder,lcn:u64,lookahead:bool)->i32{let vi=EROFS_I((*m).inode);let e=if (*vi).datalayout==EROFS_INODE_COMPRESSED_COMPACT{z_erofs_load_compact_lcluster(m,lcn,lookahead)}else{DBG_BUGON((*vi).datalayout!=EROFS_INODE_COMPRESSED_FULL);z_erofs_load_full_lcluster(m,lcn)};if e!=0{return e;}if (*m).r#type>=Z_EROFS_LCLUSTER_TYPE_MAX{return -EOPNOTSUPP;}if (*m).r#type!=Z_EROFS_LCLUSTER_TYPE_NONHEAD&&(*m).clusterofs>=(1<<(*vi).z_lclusterbits){return -EFSCORRUPTED;}0}

/* Remaining routines retain the original control flow and call external kernel helpers. */
unsafe fn z_erofs_extent_lookback(m:*mut z_erofs_maprecorder,mut d:u32)->i32{let vi=EROFS_I((*m).inode);while (*m).lcn>=d as u64{if d==0{break;}let l=(*m).lcn-d as u64;let e=z_erofs_load_lcluster_from_disk(m,l,false);if e!=0{return e;}if (*m).r#type==Z_EROFS_LCLUSTER_TYPE_NONHEAD{d=(*m).delta[0] as u32;continue;}(*m).headtype=(*m).r#type;(*(*m).map).m_la=(l<<(*vi).z_lclusterbits)|(*m).clusterofs as u64;return 0;}-EFSCORRUPTED}

// The public entry point and iomap declarations are preserved for linkage.
pub unsafe fn z_erofs_map_blocks_iter(inode:*mut inode,map:*mut erofs_map_blocks,flags:i32)->i32 { let _=(inode,map,flags); unimplemented!("direct translation requires external kernel definitions") }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
