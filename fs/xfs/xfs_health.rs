// SPDX-License-Identifier: GPL-2.0+
/* Direct translation of xfs_health.c; included C headers provide external types and symbols. */

#[repr(C)]
pub struct ioctl_sick_map { pub sick_mask: u32, pub ioctl_mask: u32 }

unsafe fn xfs_health_unmount_group(xg: *mut xfs_group, warn: *mut bool) {
    let mut sick = 0u32; let mut checked = 0u32;
    xfs_group_measure_sickness(xg, &mut sick, &mut checked);
    if sick != 0 { trace_xfs_group_unfixed_corruption(xg, sick); *warn = true; }
}

pub unsafe fn xfs_health_unmount(mp: *mut xfs_mount) {
    let mut pag: *mut xfs_perag = core::ptr::null_mut();
    let mut rtg: *mut xfs_rtgroup = core::ptr::null_mut();
    let mut sick = 0u32; let mut checked = 0u32; let mut warn = false;
    if xfs_is_shutdown(mp) { return; }
    while { pag = xfs_perag_next(mp, pag); !pag.is_null() } { xfs_health_unmount_group(pag_group(pag), &mut warn); }
    while { rtg = xfs_rtgroup_next(mp, rtg); !rtg.is_null() } { xfs_health_unmount_group(rtg_group(rtg), &mut warn); }
    xfs_fs_measure_sickness(mp, &mut sick, &mut checked);
    if sick & !XFS_SICK_FS_COUNTERS != 0 { trace_xfs_fs_unfixed_corruption(mp, sick); warn = true; }
    if warn { xfs_warn(mp, "Uncorrected metadata errors detected; please run xfs_repair."); if sick & XFS_SICK_FS_COUNTERS != 0 { xfs_fs_mark_healthy(mp, XFS_SICK_FS_COUNTERS); } }
}

pub unsafe fn xfs_fs_mark_sick(mp: *mut xfs_mount, mask: u32) {
    ASSERT(!(mask & !XFS_SICK_FS_ALL)); trace_xfs_fs_mark_sick(mp, mask);
    spin_lock(&mut (*mp).m_sb_lock); let old_mask = (*mp).m_fs_sick; (*mp).m_fs_sick |= mask; spin_unlock(&mut (*mp).m_sb_lock);
    fserror_report_metadata((*mp).m_super, -EFSCORRUPTED, GFP_NOFS); if mask != 0 { xfs_healthmon_report_fs(mp, XFS_HEALTHMON_SICK, old_mask, mask); }
}
pub unsafe fn xfs_fs_mark_corrupt(mp: *mut xfs_mount, mask: u32) {
    ASSERT(!(mask & !XFS_SICK_FS_ALL)); trace_xfs_fs_mark_corrupt(mp, mask);
    spin_lock(&mut (*mp).m_sb_lock); let old_mask = (*mp).m_fs_sick; (*mp).m_fs_sick |= mask; (*mp).m_fs_checked |= mask; spin_unlock(&mut (*mp).m_sb_lock);
    fserror_report_metadata((*mp).m_super, -EFSCORRUPTED, GFP_NOFS); if mask != 0 { xfs_healthmon_report_fs(mp, XFS_HEALTHMON_CORRUPT, old_mask, mask); }
}
pub unsafe fn xfs_fs_mark_healthy(mp: *mut xfs_mount, mask: u32) {
    ASSERT(!(mask & !XFS_SICK_FS_ALL)); trace_xfs_fs_mark_healthy(mp, mask);
    spin_lock(&mut (*mp).m_sb_lock); let old_mask = (*mp).m_fs_sick; (*mp).m_fs_sick &= !mask; if (*mp).m_fs_sick & XFS_SICK_FS_PRIMARY == 0 { (*mp).m_fs_sick &= !XFS_SICK_FS_SECONDARY; } (*mp).m_fs_checked |= mask; spin_unlock(&mut (*mp).m_sb_lock);
    if mask != 0 { xfs_healthmon_report_fs(mp, XFS_HEALTHMON_HEALTHY, old_mask, mask); }
}
pub unsafe fn xfs_fs_measure_sickness(mp: *mut xfs_mount, sick: *mut u32, checked: *mut u32) { spin_lock(&mut (*mp).m_sb_lock); *sick=(*mp).m_fs_sick; *checked=(*mp).m_fs_checked; spin_unlock(&mut (*mp).m_sb_lock); }

pub unsafe fn xfs_agno_mark_sick(mp:*mut xfs_mount, agno:xfs_agnumber_t, mask:u32) { let pag=xfs_perag_get(mp,agno); if pag.is_null(){return;} xfs_ag_mark_sick(pag,mask); xfs_perag_put(pag); }
unsafe fn xfs_group_check_mask(xg:*mut xfs_group, mask:u32) { if (*xg).xg_type==XG_TYPE_AG { ASSERT(!(mask & !XFS_SICK_AG_ALL)); } else { ASSERT(!(mask & !XFS_SICK_RG_ALL)); } }
pub unsafe fn xfs_group_mark_sick(xg:*mut xfs_group, mask:u32) { xfs_group_check_mask(xg,mask); trace_xfs_group_mark_sick(xg,mask); spin_lock(&mut (*xg).xg_state_lock); let old=(*xg).xg_sick; (*xg).xg_sick|=mask; spin_unlock(&mut (*xg).xg_state_lock); fserror_report_metadata((*(*xg).xg_mount).m_super,-EFSCORRUPTED,GFP_NOFS); if mask!=0{xfs_healthmon_report_group(xg,XFS_HEALTHMON_SICK,old,mask);} }
pub unsafe fn xfs_group_mark_corrupt(xg:*mut xfs_group, mask:u32) { xfs_group_check_mask(xg,mask); trace_xfs_group_mark_corrupt(xg,mask); spin_lock(&mut (*xg).xg_state_lock); let old=(*xg).xg_sick; (*xg).xg_sick|=mask; (*xg).xg_checked|=mask; spin_unlock(&mut (*xg).xg_state_lock); fserror_report_metadata((*(*xg).xg_mount).m_super,-EFSCORRUPTED,GFP_NOFS); if mask!=0{xfs_healthmon_report_group(xg,XFS_HEALTHMON_CORRUPT,old,mask);} }
pub unsafe fn xfs_group_mark_healthy(xg:*mut xfs_group, mask:u32) { xfs_group_check_mask(xg,mask); trace_xfs_group_mark_healthy(xg,mask); spin_lock(&mut (*xg).xg_state_lock); let old=(*xg).xg_sick; (*xg).xg_sick&=!mask; if (*xg).xg_sick & XFS_SICK_AG_PRIMARY==0 {(*xg).xg_sick&=!XFS_SICK_AG_SECONDARY;} (*xg).xg_checked|=mask; spin_unlock(&mut (*xg).xg_state_lock); if mask!=0{xfs_healthmon_report_group(xg,XFS_HEALTHMON_HEALTHY,old,mask);} }
pub unsafe fn xfs_group_measure_sickness(xg:*mut xfs_group,sick:*mut u32,checked:*mut u32){spin_lock(&mut (*xg).xg_state_lock);*sick=(*xg).xg_sick;*checked=(*xg).xg_checked;spin_unlock(&mut (*xg).xg_state_lock);}
pub unsafe fn xfs_rgno_mark_sick(mp:*mut xfs_mount,rgno:xfs_rgnumber_t,mask:u32){let rtg=xfs_rtgroup_get(mp,rgno);if rtg.is_null(){return;}xfs_group_mark_sick(rtg_group(rtg),mask);xfs_rtgroup_put(rtg);}

unsafe fn xfs_inode_report_fserror(ip:*mut xfs_inode){if xfs_iflags_test(ip,XFS_INEW|XFS_IRECLAIM)||xfs_is_internal_inode(ip){fserror_report_metadata((*ip).i_mount.m_super,-EFSCORRUPTED,GFP_NOFS);}else{fserror_report_file_metadata(VFS_I(ip),-EFSCORRUPTED,GFP_NOFS);}}
pub unsafe fn xfs_inode_mark_sick(ip:*mut xfs_inode,mask:u32){ASSERT(!(mask&!XFS_SICK_INO_ALL));trace_xfs_inode_mark_sick(ip,mask);spin_lock(&mut (*ip).i_flags_lock);let old=(*ip).i_sick;(*ip).i_sick|=mask;spin_unlock(&mut (*ip).i_flags_lock);spin_lock(&mut (*VFS_I(ip)).i_lock);inode_state_clear(VFS_I(ip),I_DONTCACHE);spin_unlock(&mut (*VFS_I(ip)).i_lock);xfs_inode_report_fserror(ip);if mask!=0{xfs_healthmon_report_inode(ip,XFS_HEALTHMON_SICK,old,mask);}}
pub unsafe fn xfs_inode_mark_corrupt(ip:*mut xfs_inode,mask:u32){ASSERT(!(mask&!XFS_SICK_INO_ALL));trace_xfs_inode_mark_corrupt(ip,mask);spin_lock(&mut (*ip).i_flags_lock);let old=(*ip).i_sick;(*ip).i_sick|=mask;(*ip).i_checked|=mask;spin_unlock(&mut (*ip).i_flags_lock);spin_lock(&mut (*VFS_I(ip)).i_lock);inode_state_clear(VFS_I(ip),I_DONTCACHE);spin_unlock(&mut (*VFS_I(ip)).i_lock);xfs_inode_report_fserror(ip);if mask!=0{xfs_healthmon_report_inode(ip,XFS_HEALTHMON_CORRUPT,old,mask);}}
pub unsafe fn xfs_inode_mark_healthy(ip:*mut xfs_inode,mask:u32){ASSERT(!(mask&!XFS_SICK_INO_ALL));trace_xfs_inode_mark_healthy(ip,mask);spin_lock(&mut (*ip).i_flags_lock);let old=(*ip).i_sick;(*ip).i_sick&=!mask;if (*ip).i_sick&XFS_SICK_INO_PRIMARY==0{(*ip).i_sick&=!XFS_SICK_INO_SECONDARY;}(*ip).i_checked|=mask;spin_unlock(&mut (*ip).i_flags_lock);if mask!=0{xfs_healthmon_report_inode(ip,XFS_HEALTHMON_HEALTHY,old,mask);}}
pub unsafe fn xfs_inode_measure_sickness(ip:*mut xfs_inode,sick:*mut u32,checked:*mut u32){spin_lock(&mut (*ip).i_flags_lock);*sick=(*ip).i_sick;*checked=(*ip).i_checked;spin_unlock(&mut (*ip).i_flags_lock);}

unsafe fn map_mask(map:&[ioctl_sick_map], sick:u32)->u32{let mut out=0;for m in map{if sick&m.sick_mask!=0{out|=m.ioctl_mask;}}out}
static FS_MAP:&[ioctl_sick_map]=&[
 ioctl_sick_map{sick_mask:XFS_SICK_FS_COUNTERS,ioctl_mask:XFS_FSOP_GEOM_SICK_COUNTERS},ioctl_sick_map{sick_mask:XFS_SICK_FS_UQUOTA,ioctl_mask:XFS_FSOP_GEOM_SICK_UQUOTA},ioctl_sick_map{sick_mask:XFS_SICK_FS_GQUOTA,ioctl_mask:XFS_FSOP_GEOM_SICK_GQUOTA},ioctl_sick_map{sick_mask:XFS_SICK_FS_PQUOTA,ioctl_mask:XFS_FSOP_GEOM_SICK_PQUOTA},ioctl_sick_map{sick_mask:XFS_SICK_FS_QUOTACHECK,ioctl_mask:XFS_FSOP_GEOM_SICK_QUOTACHECK},ioctl_sick_map{sick_mask:XFS_SICK_FS_NLINKS,ioctl_mask:XFS_FSOP_GEOM_SICK_NLINKS},ioctl_sick_map{sick_mask:XFS_SICK_FS_METADIR,ioctl_mask:XFS_FSOP_GEOM_SICK_METADIR},ioctl_sick_map{sick_mask:XFS_SICK_FS_METAPATH,ioctl_mask:XFS_FSOP_GEOM_SICK_METAPATH}];
static RT_MAP:&[ioctl_sick_map]=&[ioctl_sick_map{sick_mask:XFS_SICK_RG_BITMAP,ioctl_mask:XFS_FSOP_GEOM_SICK_RT_BITMAP},ioctl_sick_map{sick_mask:XFS_SICK_RG_SUMMARY,ioctl_mask:XFS_FSOP_GEOM_SICK_RT_SUMMARY}];
static AG_MAP:&[ioctl_sick_map]=&[ioctl_sick_map{sick_mask:XFS_SICK_AG_SB,ioctl_mask:XFS_AG_GEOM_SICK_SB},ioctl_sick_map{sick_mask:XFS_SICK_AG_AGF,ioctl_mask:XFS_AG_GEOM_SICK_AGF},ioctl_sick_map{sick_mask:XFS_SICK_AG_AGFL,ioctl_mask:XFS_AG_GEOM_SICK_AGFL},ioctl_sick_map{sick_mask:XFS_SICK_AG_AGI,ioctl_mask:XFS_AG_GEOM_SICK_AGI},ioctl_sick_map{sick_mask:XFS_SICK_AG_BNOBT,ioctl_mask:XFS_AG_GEOM_SICK_BNOBT},ioctl_sick_map{sick_mask:XFS_SICK_AG_CNTBT,ioctl_mask:XFS_AG_GEOM_SICK_CNTBT},ioctl_sick_map{sick_mask:XFS_SICK_AG_INOBT,ioctl_mask:XFS_AG_GEOM_SICK_INOBT},ioctl_sick_map{sick_mask:XFS_SICK_AG_FINOBT,ioctl_mask:XFS_AG_GEOM_SICK_FINOBT},ioctl_sick_map{sick_mask:XFS_SICK_AG_RMAPBT,ioctl_mask:XFS_AG_GEOM_SICK_RMAPBT},ioctl_sick_map{sick_mask:XFS_SICK_AG_REFCNTBT,ioctl_mask:XFS_AG_GEOM_SICK_REFCNTBT},ioctl_sick_map{sick_mask:XFS_SICK_AG_INODES,ioctl_mask:XFS_AG_GEOM_SICK_INODES}];
static RTGROUP_MAP:&[ioctl_sick_map]=&[ioctl_sick_map{sick_mask:XFS_SICK_RG_SUPER,ioctl_mask:XFS_RTGROUP_GEOM_SICK_SUPER},ioctl_sick_map{sick_mask:XFS_SICK_RG_BITMAP,ioctl_mask:XFS_RTGROUP_GEOM_SICK_BITMAP},ioctl_sick_map{sick_mask:XFS_SICK_RG_SUMMARY,ioctl_mask:XFS_RTGROUP_GEOM_SICK_SUMMARY},ioctl_sick_map{sick_mask:XFS_SICK_RG_RMAPBT,ioctl_mask:XFS_RTGROUP_GEOM_SICK_RMAPBT},ioctl_sick_map{sick_mask:XFS_SICK_RG_REFCNTBT,ioctl_mask:XFS_RTGROUP_GEOM_SICK_REFCNTBT}];
static INO_MAP:&[ioctl_sick_map]=&[ioctl_sick_map{sick_mask:XFS_SICK_INO_CORE,ioctl_mask:XFS_BS_SICK_INODE},ioctl_sick_map{sick_mask:XFS_SICK_INO_BMBTD,ioctl_mask:XFS_BS_SICK_BMBTD},ioctl_sick_map{sick_mask:XFS_SICK_INO_BMBTA,ioctl_mask:XFS_BS_SICK_BMBTA},ioctl_sick_map{sick_mask:XFS_SICK_INO_BMBTC,ioctl_mask:XFS_BS_SICK_BMBTC},ioctl_sick_map{sick_mask:XFS_SICK_INO_DIR,ioctl_mask:XFS_BS_SICK_DIR},ioctl_sick_map{sick_mask:XFS_SICK_INO_XATTR,ioctl_mask:XFS_BS_SICK_XATTR},ioctl_sick_map{sick_mask:XFS_SICK_INO_SYMLINK,ioctl_mask:XFS_BS_SICK_SYMLINK},ioctl_sick_map{sick_mask:XFS_SICK_INO_PARENT,ioctl_mask:XFS_BS_SICK_PARENT},ioctl_sick_map{sick_mask:XFS_SICK_INO_BMBTD_ZAPPED,ioctl_mask:XFS_BS_SICK_BMBTD},ioctl_sick_map{sick_mask:XFS_SICK_INO_BMBTA_ZAPPED,ioctl_mask:XFS_BS_SICK_BMBTA},ioctl_sick_map{sick_mask:XFS_SICK_INO_DIR_ZAPPED,ioctl_mask:XFS_BS_SICK_DIR},ioctl_sick_map{sick_mask:XFS_SICK_INO_SYMLINK_ZAPPED,ioctl_mask:XFS_BS_SICK_SYMLINK},ioctl_sick_map{sick_mask:XFS_SICK_INO_DIRTREE,ioctl_mask:XFS_BS_SICK_DIRTREE}];

pub unsafe fn xfs_healthmon_fs_mask(mask:u32)->u32{map_mask(FS_MAP,mask)} pub unsafe fn xfs_healthmon_perag_mask(mask:u32)->u32{map_mask(AG_MAP,mask)} pub unsafe fn xfs_healthmon_rtgroup_mask(mask:u32)->u32{map_mask(RTGROUP_MAP,mask)} pub unsafe fn xfs_healthmon_inode_mask(mask:u32)->u32{map_mask(INO_MAP,mask)}
pub unsafe fn xfs_fsop_geom_health(mp:*mut xfs_mount,geo:*mut xfs_fsop_geom){(*geo).sick=0;(*geo).checked=0;let(mut s,mut c)=(0,0);xfs_fs_measure_sickness(mp,&mut s,&mut c);for m in FS_MAP{if c&m.sick_mask!=0{(*geo).checked|=m.ioctl_mask}if s&m.sick_mask!=0{(*geo).sick|=m.ioctl_mask}}let mut rtg=core::ptr::null_mut();while{rtg=xfs_rtgroup_next(mp,rtg);!rtg.is_null()}{xfs_group_measure_sickness(rtg_group(rtg),&mut s,&mut c);for m in RT_MAP{if c&m.sick_mask!=0{(*geo).checked|=m.ioctl_mask}if s&m.sick_mask!=0{(*geo).sick|=m.ioctl_mask}}}}
pub unsafe fn xfs_ag_geom_health(pag:*mut xfs_perag,geo:*mut xfs_ag_geometry){(*geo).ag_sick=0;(*geo).ag_checked=0;let(mut s,mut c)=(0,0);xfs_group_measure_sickness(pag_group(pag),&mut s,&mut c);for m in AG_MAP{if c&m.sick_mask!=0{(*geo).ag_checked|=m.ioctl_mask}if s&m.sick_mask!=0{(*geo).ag_sick|=m.ioctl_mask}}}
pub unsafe fn xfs_rtgroup_geom_health(rtg:*mut xfs_rtgroup,geo:*mut xfs_rtgroup_geometry){(*geo).rg_sick=0;(*geo).rg_checked=0;let(mut s,mut c)=(0,0);xfs_group_measure_sickness(rtg_group(rtg),&mut s,&mut c);for m in RTGROUP_MAP{if c&m.sick_mask!=0{(*geo).rg_checked|=m.ioctl_mask}if s&m.sick_mask!=0{(*geo).rg_sick|=m.ioctl_mask}}}
pub unsafe fn xfs_bulkstat_health(ip:*mut xfs_inode,bs:*mut xfs_bulkstat){(*bs).bs_sick=0;(*bs).bs_checked=0;let(mut s,mut c)=(0,0);xfs_inode_measure_sickness(ip,&mut s,&mut c);for m in INO_MAP{if c&m.sick_mask!=0{(*bs).bs_checked|=m.ioctl_mask}if s&m.sick_mask!=0{(*bs).bs_sick|=m.ioctl_mask}}}
pub unsafe fn xfs_bmap_mark_sick(ip:*mut xfs_inode,whichfork:i32){let mask=match whichfork{XFS_DATA_FORK=>XFS_SICK_INO_BMBTD,XFS_ATTR_FORK=>XFS_SICK_INO_BMBTA,XFS_COW_FORK=>XFS_SICK_INO_BMBTC,_=>{ASSERT(false);return}};xfs_inode_mark_sick(ip,mask);}
pub unsafe fn xfs_btree_mark_sick(cur:*mut xfs_btree_cur){if xfs_btree_is_bmap((*cur).bc_ops){xfs_bmap_mark_sick((*cur).bc_ino.ip,(*cur).bc_ino.whichfork);}else if (*(*cur).bc_ops).type_!=XFS_BTREE_TYPE_MEM{ASSERT(!(*cur).bc_group.is_null());ASSERT((*(*cur).bc_ops).sick_mask!=0);xfs_group_mark_sick((*cur).bc_group,(*(*cur).bc_ops).sick_mask);}}
pub unsafe fn xfs_dirattr_mark_sick(ip:*mut xfs_inode,whichfork:i32){let mask=match whichfork{XFS_DATA_FORK=>XFS_SICK_INO_DIR,XFS_ATTR_FORK=>XFS_SICK_INO_XATTR,_=>{ASSERT(false);return}};xfs_inode_mark_sick(ip,mask);}
pub unsafe fn xfs_da_mark_sick(args:*mut xfs_da_args){xfs_dirattr_mark_sick((*args).dp,(*args).whichfork);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
