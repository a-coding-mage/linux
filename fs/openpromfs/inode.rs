// SPDX-License-Identifier: GPL-2.0-only
/* inode.c: /proc/openprom handling routines
 *
 * Copyright (C) 1996-1999 Jakub Jelinek  (jakub@redhat.com)
 * Copyright (C) 1998      Eddie C. Dost  (ecd@skynet.be)
 */

// Linux and architecture-specific headers provide the types and functions
// referenced below.

static mut OP_MUTEX: DEFINE_MUTEX = DEFINE_MUTEX::new();

const OPENPROM_ROOT_INO: ino_t = 0;

#[repr(C)]
enum op_inode_type {
    op_inode_node,
    op_inode_prop,
}

#[repr(C)]
union op_inode_data {
    node: *mut device_node,
    prop: *mut property,
}

#[repr(C)]
struct op_inode_info {
    vfs_inode: inode,
    type_: op_inode_type,
    u: op_inode_data,
}

extern "C" {
    fn openprom_iget(sb: *mut super_block, ino: ino_t) -> *mut inode;
}

#[inline]
unsafe fn OP_I(inode: *mut inode) -> *mut op_inode_info {
    container_of!(inode, op_inode_info, vfs_inode)
}

unsafe fn is_string(p: *mut u8, len: c_int) -> c_int {
    let mut i = 0;
    while i < len {
        let val = *p.add(i as usize);
        if (i != 0 && val == 0) || (val >= b' ' && val <= b'~') {
            i += 1;
            continue;
        }
        return 0;
    }
    1
}

unsafe extern "C" fn property_show(f: *mut seq_file, _v: *mut c_void) -> c_int {
    let prop = (*f).private as *mut property;
    let mut pval: *mut u8;
    let mut len: c_int;

    len = (*prop).length;
    pval = (*prop).value;

    if is_string(pval, len) != 0 {
        while len > 0 {
            let n = strlen(pval as *const c_char) as c_int;
            seq_printf(f, c_str!("%s"), pval as *const c_char);
            // Skip over the NULL byte too.
            pval = pval.add((n + 1) as usize);
            len -= n + 1;
            if len > 0 {
                seq_printf(f, c_str!(" + "));
            }
        }
    } else if len & 3 != 0 {
        while len != 0 {
            len -= 1;
            if len != 0 {
                seq_printf(f, c_str!("%02x."), *pval);
            } else {
                seq_printf(f, c_str!("%02x"), *pval);
            }
            pval = pval.add(1);
        }
    } else {
        while len >= 4 {
            len -= 4;
            if len != 0 {
                seq_printf(f, c_str!("%08x."), *(pval as *mut c_uint));
            } else {
                seq_printf(f, c_str!("%08x"), *(pval as *mut c_uint));
            }
            pval = pval.add(4);
        }
    }
    seq_printf(f, c_str!("\n"));
    0
}

unsafe extern "C" fn property_start(_f: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    if *pos == 0 { pos as *mut c_void } else { core::ptr::null_mut() }
}

unsafe extern "C" fn property_next(_f: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    *pos += 1;
    core::ptr::null_mut()
}

unsafe extern "C" fn property_stop(_f: *mut seq_file, _v: *mut c_void) {}

static property_op: seq_operations = seq_operations {
    start: Some(property_start), next: Some(property_next),
    stop: Some(property_stop), show: Some(property_show),
};

unsafe extern "C" fn property_open(inode: *mut inode, file: *mut file) -> c_int {
    let oi = OP_I(inode);
    BUG_ON!((*oi).type_ != op_inode_type::op_inode_prop);
    let ret = seq_open(file, &property_op);
    if ret == 0 { (*( (*file).private_data as *mut seq_file)).private = (*oi).u.prop as *mut c_void; }
    ret
}

static openpromfs_prop_ops: file_operations = file_operations {
    open: Some(property_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(seq_release), ..file_operations::empty()
};

unsafe extern "C" fn openpromfs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let inode = file_inode!(file);
    let oi = OP_I(inode);
    let dp = (*oi).u.node;
    let mut child = (*dp).child;
    let mut prop = (*dp).properties;
    let mut i: c_int;
    mutex_lock!(&mut OP_MUTEX);
    if (*ctx).pos == 0 {
        if !dir_emit!(ctx, ".", 1, (*inode).i_ino, DT_DIR) { mutex_unlock!(&mut OP_MUTEX); return 0; }
        (*ctx).pos = 1;
    }
    if (*ctx).pos == 1 {
        let ino = if (*dp).parent.is_null() { OPENPROM_ROOT_INO } else { (*(*dp).parent).unique_id };
        if !dir_emit!(ctx, "..", 2, ino, DT_DIR) { mutex_unlock!(&mut OP_MUTEX); return 0; }
        (*ctx).pos = 2;
    }
    i = (*ctx).pos as c_int - 2;
    while i != 0 && !child.is_null() { child = (*child).sibling; i -= 1; }
    while !child.is_null() {
        let n = kbasename!((*child).full_name);
        if !dir_emit!(ctx, n, strlen(n), (*child).unique_id, DT_DIR) { break; }
        (*ctx).pos += 1; child = (*child).sibling;
    }
    while i != 0 && !prop.is_null() { prop = (*prop).next; i -= 1; }
    while !prop.is_null() {
        if !dir_emit!(ctx, (*prop).name, strlen((*prop).name), (*prop).unique_id, DT_REG) { break; }
        (*ctx).pos += 1; prop = (*prop).next;
    }
    mutex_unlock!(&mut OP_MUTEX); 0
}

static openprom_operations: file_operations = file_operations {
    read: Some(generic_read_dir), iterate_shared: Some(openpromfs_readdir), llseek: Some(generic_file_llseek), ..file_operations::empty()
};

unsafe extern "C" fn openpromfs_lookup(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    let oi = OP_I(dir); BUG_ON!((*oi).type_ != op_inode_type::op_inode_node);
    let dp = (*oi).u.node; let name = (*dentry).d_name.name; let len = (*dentry).d_name.len;
    mutex_lock!(&mut OP_MUTEX); let mut child = (*dp).child;
    while !child.is_null() { let nn = kbasename!((*child).full_name); if len == strlen(nn) && !strncmp(nn, name, len) { let inode = openprom_iget((*dir).i_sb, (*child).unique_id); mutex_unlock!(&mut OP_MUTEX); return d_splice_alias!(inode, dentry); } child = (*child).sibling; }
    let mut prop = (*dp).properties;
    while !prop.is_null() { if len == strlen((*prop).name) && !strncmp((*prop).name, name, len) { let inode = openprom_iget((*dir).i_sb, (*prop).unique_id); mutex_unlock!(&mut OP_MUTEX); return d_splice_alias!(inode, dentry); } prop = (*prop).next; }
    mutex_unlock!(&mut OP_MUTEX); ERR_PTR!(-ENOENT)
}

static openprom_inode_operations: inode_operations = inode_operations { lookup: Some(openpromfs_lookup), ..inode_operations::empty() };

// The remaining filesystem callbacks retain the C implementation's pointer-level
// behavior and are declared here for linkage with the corresponding kernel APIs.
static mut op_inode_cachep: *mut kmem_cache = core::ptr::null_mut();
unsafe extern "C" fn openprom_alloc_inode(sb: *mut super_block) -> *mut inode { let oi = alloc_inode_sb!(sb, op_inode_cachep, GFP_KERNEL); if oi.is_null() { core::ptr::null_mut() } else { &mut (*oi).vfs_inode } }
unsafe extern "C" fn openprom_free_inode(inode: *mut inode) { kmem_cache_free!(op_inode_cachep, OP_I(inode)); }
unsafe extern "C" fn openpromfs_reconfigure(fc: *mut fs_context) -> c_int { sync_filesystem!((*fc).root.d_sb); (*fc).sb_flags |= SB_NOATIME; 0 }
unsafe extern "C" fn openprom_fill_super(s: *mut super_block, _fc: *mut fs_context) -> c_int { (*s).s_flags |= SB_NOATIME; (*s).s_blocksize = 1024; (*s).s_blocksize_bits = 10; (*s).s_magic = OPENPROM_SUPER_MAGIC; let root = openprom_iget(s, OPENPROM_ROOT_INO); if IS_ERR!(root) { return PTR_ERR!(root); } (*s).s_root = d_make_root!(root); if (*s).s_root.is_null() { return -ENOMEM; } 0 }
unsafe extern "C" fn openpromfs_get_tree(fc: *mut fs_context) -> c_int { get_tree_single!(fc, openprom_fill_super) }
unsafe extern "C" fn openpromfs_init_fs_context(fc: *mut fs_context) -> c_int { (*fc).ops = &openpromfs_context_ops; 0 }
unsafe extern "C" fn op_inode_init_once(data: *mut c_void) { inode_init_once!(&mut (*(data as *mut op_inode_info)).vfs_inode); }
unsafe extern "C" fn init_openprom_fs() -> c_int { op_inode_cachep = kmem_cache_create!(c_str!("op_inode_cache"), core::mem::size_of::<op_inode_info>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, op_inode_init_once); if op_inode_cachep.is_null() { return -ENOMEM; } register_filesystem!(&openprom_fs_type) }
unsafe extern "C" fn exit_openprom_fs() { unregister_filesystem!(&openprom_fs_type); rcu_barrier!(); kmem_cache_destroy!(op_inode_cachep); }

// C module registration and filesystem metadata:
// MODULE_ALIAS_FS("openpromfs");
// module_init(init_openprom_fs)
// module_exit(exit_openprom_fs)
// MODULE_DESCRIPTION("OpenPROM filesystem support");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
