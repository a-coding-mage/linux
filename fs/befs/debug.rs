// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/befs/debug.c
 *
 * Copyright (C) 2001 Will Dyson (will_dyson at pobox.com)
 *
 * With help from the ntfs-tng driver by Anton Altparmakov
 *
 * Copyright (C) 1999  Makoto Kato (m_kato@ga2.so-net.ne.jp)
 *
 * debug functions
 */

// External kernel and befs declarations are supplied by other translation units.

pub unsafe extern "C" fn befs_error(
    sb: *const super_block,
    fmt: *const core::ffi::c_char,
    mut args: ...,
) {
    let mut vaf: va_format;
    va_start(&mut args, fmt);
    vaf.fmt = fmt;
    vaf.va = &mut args;
    pr_err!("({}): %pV\n", (*sb).s_id, &vaf);
    va_end(&mut args);
}

pub unsafe extern "C" fn befs_warning(
    sb: *const super_block,
    fmt: *const core::ffi::c_char,
    mut args: ...,
) {
    let mut vaf: va_format;
    va_start(&mut args, fmt);
    vaf.fmt = fmt;
    vaf.va = &mut args;
    pr_warn!("({}): %pV\n", (*sb).s_id, &vaf);
    va_end(&mut args);
}

pub unsafe extern "C" fn befs_debug(
    sb: *const super_block,
    fmt: *const core::ffi::c_char,
    mut args: ...,
) {
    // CONFIG_BEFS_DEBUG
    let mut vaf: va_format;
    va_start(&mut args, fmt);
    vaf.fmt = fmt;
    vaf.va = &mut args;
    pr_debug!("({}): %pV\n", (*sb).s_id, &vaf);
    va_end(&mut args);
}

pub unsafe fn befs_dump_inode(sb: *const super_block, inode: *mut befs_inode) {
    // CONFIG_BEFS_DEBUG
    let mut tmp_run: befs_block_run;

    befs_debug(sb, "befs_inode information");
    befs_debug(sb, "  magic1 %08x", fs32_to_cpu(sb, (*inode).magic1));

    tmp_run = fsrun_to_cpu(sb, (*inode).inode_num);
    befs_debug!(sb, "  inode_num %u, %hu, %hu", tmp_run.allocation_group, tmp_run.start, tmp_run.len);

    befs_debug!(sb, "  uid %u", fs32_to_cpu(sb, (*inode).uid));
    befs_debug!(sb, "  gid %u", fs32_to_cpu(sb, (*inode).gid));
    befs_debug!(sb, "  mode %08x", fs32_to_cpu(sb, (*inode).mode));
    befs_debug!(sb, "  flags %08x", fs32_to_cpu(sb, (*inode).flags));
    befs_debug!(sb, "  create_time %llu", fs64_to_cpu(sb, (*inode).create_time));
    befs_debug!(sb, "  last_modified_time %llu", fs64_to_cpu(sb, (*inode).last_modified_time));

    tmp_run = fsrun_to_cpu(sb, (*inode).parent);
    befs_debug!(sb, "  parent [%u, %hu, %hu]", tmp_run.allocation_group, tmp_run.start, tmp_run.len);
    tmp_run = fsrun_to_cpu(sb, (*inode).attributes);
    befs_debug!(sb, "  attributes [%u, %hu, %hu]", tmp_run.allocation_group, tmp_run.start, tmp_run.len);
    befs_debug!(sb, "  type %08x", fs32_to_cpu(sb, (*inode).type_));
    befs_debug!(sb, "  inode_size %u", fs32_to_cpu(sb, (*inode).inode_size));

    if S_ISLNK(fs32_to_cpu(sb, (*inode).mode)) {
        befs_debug!(sb, "  Symbolic link [%s]", (*inode).data.symlink);
    } else {
        for i in 0..BEFS_NUM_DIRECT_BLOCKS {
            tmp_run = fsrun_to_cpu(sb, (*inode).data.datastream.direct[i as usize]);
            befs_debug!(sb, "  direct %d [%u, %hu, %hu]", i, tmp_run.allocation_group, tmp_run.start, tmp_run.len);
        }
        befs_debug!(sb, "  max_direct_range %llu", fs64_to_cpu(sb, (*inode).data.datastream.max_direct_range));
        tmp_run = fsrun_to_cpu(sb, (*inode).data.datastream.indirect);
        befs_debug!(sb, "  indirect [%u, %hu, %hu]", tmp_run.allocation_group, tmp_run.start, tmp_run.len);
        befs_debug!(sb, "  max_indirect_range %llu", fs64_to_cpu(sb, (*inode).data.datastream.max_indirect_range));
        tmp_run = fsrun_to_cpu(sb, (*inode).data.datastream.double_indirect);
        befs_debug!(sb, "  double indirect [%u, %hu, %hu]", tmp_run.allocation_group, tmp_run.start, tmp_run.len);
        befs_debug!(sb, "  max_double_indirect_range %llu", fs64_to_cpu(sb, (*inode).data.datastream.max_double_indirect_range));
        befs_debug!(sb, "  size %llu", fs64_to_cpu(sb, (*inode).data.datastream.size));
    }
}

/* Display super block structure for debug. */
pub unsafe fn befs_dump_super_block(sb: *const super_block, sup: *mut befs_super_block) {
    // CONFIG_BEFS_DEBUG
    let mut tmp_run: befs_block_run;
    befs_debug!(sb, "befs_super_block information");
    befs_debug!(sb, "  name %s", (*sup).name);
    befs_debug!(sb, "  magic1 %08x", fs32_to_cpu(sb, (*sup).magic1));
    befs_debug!(sb, "  fs_byte_order %08x", fs32_to_cpu(sb, (*sup).fs_byte_order));
    befs_debug!(sb, "  block_size %u", fs32_to_cpu(sb, (*sup).block_size));
    befs_debug!(sb, "  block_shift %u", fs32_to_cpu(sb, (*sup).block_shift));
    befs_debug!(sb, "  num_blocks %llu", fs64_to_cpu(sb, (*sup).num_blocks));
    befs_debug!(sb, "  used_blocks %llu", fs64_to_cpu(sb, (*sup).used_blocks));
    befs_debug!(sb, "  inode_size %u", fs32_to_cpu(sb, (*sup).inode_size));
    befs_debug!(sb, "  magic2 %08x", fs32_to_cpu(sb, (*sup).magic2));
    befs_debug!(sb, "  blocks_per_ag %u", fs32_to_cpu(sb, (*sup).blocks_per_ag));
    befs_debug!(sb, "  ag_shift %u", fs32_to_cpu(sb, (*sup).ag_shift));
    befs_debug!(sb, "  num_ags %u", fs32_to_cpu(sb, (*sup).num_ags));
    befs_debug!(sb, "  flags %08x", fs32_to_cpu(sb, (*sup).flags));
    tmp_run = fsrun_to_cpu(sb, (*sup).log_blocks);
    befs_debug!(sb, "  log_blocks %u, %hu, %hu", tmp_run.allocation_group, tmp_run.start, tmp_run.len);
    befs_debug!(sb, "  log_start %lld", fs64_to_cpu(sb, (*sup).log_start));
    befs_debug!(sb, "  log_end %lld", fs64_to_cpu(sb, (*sup).log_end));
    befs_debug!(sb, "  magic3 %08x", fs32_to_cpu(sb, (*sup).magic3));
    tmp_run = fsrun_to_cpu(sb, (*sup).root_dir);
    befs_debug!(sb, "  root_dir %u, %hu, %hu", tmp_run.allocation_group, tmp_run.start, tmp_run.len);
    tmp_run = fsrun_to_cpu(sb, (*sup).indices);
    befs_debug!(sb, "  indices %u, %hu, %hu", tmp_run.allocation_group, tmp_run.start, tmp_run.len);
}

pub unsafe fn befs_dump_index_entry(sb: *const super_block, super_: *mut befs_disk_btree_super) {
    // CONFIG_BEFS_DEBUG
    befs_debug!(sb, "Btree super structure");
    befs_debug!(sb, "  magic %08x", fs32_to_cpu(sb, (*super_).magic));
    befs_debug!(sb, "  node_size %u", fs32_to_cpu(sb, (*super_).node_size));
    befs_debug!(sb, "  max_depth %08x", fs32_to_cpu(sb, (*super_).max_depth));
    befs_debug!(sb, "  data_type %08x", fs32_to_cpu(sb, (*super_).data_type));
    befs_debug!(sb, "  root_node_pointer %016LX", fs64_to_cpu(sb, (*super_).root_node_ptr));
    befs_debug!(sb, "  free_node_pointer %016LX", fs64_to_cpu(sb, (*super_).free_node_ptr));
    befs_debug!(sb, "  maximum size %016LX", fs64_to_cpu(sb, (*super_).max_size));
}

pub unsafe fn befs_dump_index_node(sb: *const super_block, node: *mut befs_btree_nodehead) {
    // CONFIG_BEFS_DEBUG
    befs_debug!(sb, "Btree node structure");
    befs_debug!(sb, "  left %016LX", fs64_to_cpu(sb, (*node).left));
    befs_debug!(sb, "  right %016LX", fs64_to_cpu(sb, (*node).right));
    befs_debug!(sb, "  overflow %016LX", fs64_to_cpu(sb, (*node).overflow));
    befs_debug!(sb, "  all_key_count %hu", fs16_to_cpu(sb, (*node).all_key_count));
    befs_debug!(sb, "  all_key_length %hu", fs16_to_cpu(sb, (*node).all_key_length));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
