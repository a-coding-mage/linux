// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ufs/cylinder.c
 *
 * Copyright (C) 1998
 * Daniel Pirkl <daniel.pirkl@email.cz>
 * Charles University, Faculty of Mathematics and Physics
 *
 *  ext2 - inode (block) bitmap caching inspired
 */

// Dependencies supplied by the surrounding kernel/UFS translation.

unsafe fn ufs_read_cylinder(
    sb: *mut super_block,
    cgno: u32,
    bitmap_nr: u32,
) -> bool {
    let sbi = UFS_SB(sb);
    let uspi: *mut ufs_sb_private_info;
    let ucpi: *mut ufs_cg_private_info;
    let ucg: *mut ufs_cylinder_group;
    let mut i: u32;
    let mut j: u32;

    UFSD!("ENTER, cgno %u, bitmap_nr %u\n", cgno, bitmap_nr);
    uspi = (*sbi).s_uspi;
    ucpi = (*sbi).s_ucpi[bitmap_nr as usize];
    ucg = (*(*sbi).s_ucg[cgno as usize]).b_data as *mut ufs_cylinder_group;

    (*UCPI_UBH(ucpi)).fragment = ufs_cgcmin(cgno);
    (*UCPI_UBH(ucpi)).count = (*uspi).s_cgsize >> (*sb).s_blocksize_bits;
    /* We have already the first fragment of cylinder group block in buffer */
    (*UCPI_UBH(ucpi)).bh[0] = (*sbi).s_ucg[cgno as usize];
    i = 1;
    while i < (*UCPI_UBH(ucpi)).count {
        (*UCPI_UBH(ucpi)).bh[i as usize] =
            sb_bread(sb, (*UCPI_UBH(ucpi)).fragment + i);
        if (*UCPI_UBH(ucpi)).bh[i as usize].is_null() {
            break;
        }
        i += 1;
    }
    if i < (*UCPI_UBH(ucpi)).count {
        j = 1;
        while j < i {
            brelse((*UCPI_UBH(ucpi)).bh[j as usize]);
            j += 1;
        }
        (*sbi).s_cgno[bitmap_nr as usize] = UFS_CGNO_EMPTY;
        ufs_error(sb, "ufs_read_cylinder", "can't read cylinder group block %u", cgno);
        return false;
    }
    (*sbi).s_cgno[bitmap_nr as usize] = cgno;

    (*ucpi).c_cgx = fs32_to_cpu(sb, (*ucg).cg_cgx);
    (*ucpi).c_ncyl = fs16_to_cpu(sb, (*ucg).cg_ncyl);
    (*ucpi).c_niblk = fs16_to_cpu(sb, (*ucg).cg_niblk);
    (*ucpi).c_ndblk = fs32_to_cpu(sb, (*ucg).cg_ndblk);
    (*ucpi).c_rotor = fs32_to_cpu(sb, (*ucg).cg_rotor);
    (*ucpi).c_frotor = fs32_to_cpu(sb, (*ucg).cg_frotor);
    (*ucpi).c_irotor = fs32_to_cpu(sb, (*ucg).cg_irotor);
    (*ucpi).c_btotoff = fs32_to_cpu(sb, (*ucg).cg_btotoff);
    (*ucpi).c_boff = fs32_to_cpu(sb, (*ucg).cg_boff);
    (*ucpi).c_iusedoff = fs32_to_cpu(sb, (*ucg).cg_iusedoff);
    (*ucpi).c_freeoff = fs32_to_cpu(sb, (*ucg).cg_freeoff);
    (*ucpi).c_nextfreeoff = fs32_to_cpu(sb, (*ucg).cg_nextfreeoff);
    (*ucpi).c_clustersumoff = fs32_to_cpu(sb, (*ucg).cg_u.cg_44.cg_clustersumoff);
    (*ucpi).c_clusteroff = fs32_to_cpu(sb, (*ucg).cg_u.cg_44.cg_clusteroff);
    (*ucpi).c_nclusterblks = fs32_to_cpu(sb, (*ucg).cg_u.cg_44.cg_nclusterblks);
    UFSD!("EXIT\n");
    true
}

pub unsafe fn ufs_put_cylinder(sb: *mut super_block, bitmap_nr: u32) {
    let sbi = UFS_SB(sb);
    let uspi = (*sbi).s_uspi;
    UFSD!("ENTER, bitmap_nr %u\n", bitmap_nr);
    if (*sbi).s_cgno[bitmap_nr as usize] == UFS_CGNO_EMPTY { UFSD!("EXIT\n"); return; }
    let ucpi = (*sbi).s_ucpi[bitmap_nr as usize];
    let ucg = ubh_get_ucg(UCPI_UBH(ucpi));
    if (*uspi).s_ncg > UFS_MAX_GROUP_LOADED && bitmap_nr >= (*sbi).s_cg_loaded {
        ufs_panic(sb, "ufs_put_cylinder", "internal error"); return;
    }
    (*ucg).cg_rotor = cpu_to_fs32(sb, (*ucpi).c_rotor);
    (*ucg).cg_frotor = cpu_to_fs32(sb, (*ucpi).c_frotor);
    (*ucg).cg_irotor = cpu_to_fs32(sb, (*ucpi).c_irotor);
    ubh_mark_buffer_dirty(UCPI_UBH(ucpi));
    let mut i = 1;
    while i < (*UCPI_UBH(ucpi)).count { brelse((*UCPI_UBH(ucpi)).bh[i as usize]); i += 1; }
    (*sbi).s_cgno[bitmap_nr as usize] = UFS_CGNO_EMPTY;
    UFSD!("EXIT\n");
}

pub unsafe fn ufs_load_cylinder(sb: *mut super_block, cgno: u32) -> *mut ufs_cg_private_info {
    let sbi = UFS_SB(sb);
    let uspi = (*sbi).s_uspi;
    UFSD!("ENTER, cgno %u\n", cgno);
    if cgno >= (*uspi).s_ncg { ufs_panic(sb, "ufs_load_cylinder", "internal error, high number of cg"); return core::ptr::null_mut(); }
    if (*sbi).s_cgno[0] == cgno { UFSD!("EXIT\n"); return (*sbi).s_ucpi[0]; }
    if (*uspi).s_ncg <= UFS_MAX_GROUP_LOADED {
        if (*sbi).s_cgno[cgno as usize] == UFS_CGNO_EMPTY {
            if !ufs_read_cylinder(sb, cgno, cgno) { UFSD!("EXIT (FAILED)\n"); return core::ptr::null_mut(); }
        } else if (*sbi).s_cgno[cgno as usize] != cgno { ufs_panic(sb, "ufs_load_cylinder", "internal error, wrong number of cg in cache"); return core::ptr::null_mut(); }
        return (*sbi).s_ucpi[cgno as usize];
    }
    let mut i = 0;
    while i < (*sbi).s_cg_loaded && (*sbi).s_cgno[i as usize] != cgno { i += 1; }
    let ucpi;
    if i < (*sbi).s_cg_loaded {
        let cg = (*sbi).s_cgno[i as usize]; ucpi = (*sbi).s_ucpi[i as usize];
        let mut j = i; while j > 0 { (*sbi).s_cgno[j as usize] = (*sbi).s_cgno[(j-1) as usize]; (*sbi).s_ucpi[j as usize] = (*sbi).s_ucpi[(j-1) as usize]; j -= 1; }
        (*sbi).s_cgno[0] = cg; (*sbi).s_ucpi[0] = ucpi;
    } else {
        if (*sbi).s_cg_loaded < UFS_MAX_GROUP_LOADED { (*sbi).s_cg_loaded += 1; } else { ufs_put_cylinder(sb, UFS_MAX_GROUP_LOADED - 1); }
        let last = (*sbi).s_cg_loaded - 1; ucpi = (*sbi).s_ucpi[last as usize];
        let mut j = last; while j > 0 { (*sbi).s_cgno[j as usize] = (*sbi).s_cgno[(j-1) as usize]; (*sbi).s_ucpi[j as usize] = (*sbi).s_ucpi[(j-1) as usize]; j -= 1; }
        (*sbi).s_ucpi[0] = ucpi;
        if !ufs_read_cylinder(sb, cgno, 0) { UFSD!("EXIT (FAILED)\n"); return core::ptr::null_mut(); }
    }
    UFSD!("EXIT\n"); (*sbi).s_ucpi[0]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
