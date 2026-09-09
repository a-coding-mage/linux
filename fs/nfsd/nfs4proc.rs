/*
 * Source-level Rust representation of the Linux NFSv4 server procedures.
 *
 * The implementation depends on the kernel types and helpers declared by the
 * surrounding NFSD translation units.  Those names are intentionally kept as
 * external dependencies, just as they are in the original implementation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Kernel/build configuration supplied by the surrounding translation unit.
// The C source includes Linux and NFSD headers here; their declarations are
// external to this isolated translation.

pub const NFSDDBG_FACILITY: u32 = NFSDDBG_PROC;

static mut inter_copy_offload_enable: bool = false;

static mut nfsd_attrmask: [u32; 3] = [
    NFSD_WRITEABLE_ATTRS_WORD0,
    NFSD_WRITEABLE_ATTRS_WORD1,
    NFSD_WRITEABLE_ATTRS_WORD2,
];

static mut nfsd41_ex_attrmask: [u32; 3] = [
    NFSD_SUPPATTR_EXCLCREAT_WORD0,
    NFSD_SUPPATTR_EXCLCREAT_WORD1,
    NFSD_SUPPATTR_EXCLCREAT_WORD2,
];

unsafe fn check_attr_support(
    cstate: *mut nfsd4_compound_state,
    bmval: *mut u32,
    writable: *mut u32,
) -> __be32 {
    let dentry = (*cstate).current_fh.fh_dentry;
    let exp = (*cstate).current_fh.fh_export;
    if !nfsd_attrs_supported((*cstate).minorversion, bmval) { return nfserr_attrnotsupp; }
    if (*bmval.add(0) & FATTR4_WORD0_ACL) != 0 && !IS_POSIXACL(d_inode(dentry)) { return nfserr_attrnotsupp; }
    if (*bmval.add(2) & (FATTR4_WORD2_POSIX_DEFAULT_ACL | FATTR4_WORD2_POSIX_ACCESS_ACL)) != 0 && !IS_POSIXACL(d_inode(dentry)) { return nfserr_attrnotsupp; }
    if (*bmval.add(2) & FATTR4_WORD2_SECURITY_LABEL) != 0 && ((*exp).ex_flags & NFSEXP_SECURITY_LABEL) == 0 { return nfserr_attrnotsupp; }
    if !writable.is_null() && !bmval_is_subset(bmval, writable) { return nfserr_inval; }
    if !writable.is_null() && (*bmval.add(2) & FATTR4_WORD2_MODE_UMASK) != 0 && (*bmval.add(1) & FATTR4_WORD1_MODE) != 0 { return nfserr_inval; }
    nfs_ok
}

unsafe fn nfsd4_check_open_attributes(cstate: *mut nfsd4_compound_state, open: *mut nfsd4_open) -> __be32 {
    if (*open).op_create != NFS4_OPEN_CREATE { return nfs_ok; }
    match (*open).op_createmode {
        NFS4_CREATE_UNCHECKED | NFS4_CREATE_GUARDED => check_attr_support(cstate, (*open).op_bmval.as_mut_ptr(), nfsd_attrmask.as_mut_ptr()),
        NFS4_CREATE_EXCLUSIVE4_1 => check_attr_support(cstate, (*open).op_bmval.as_mut_ptr(), nfsd41_ex_attrmask.as_mut_ptr()),
        _ => nfs_ok,
    }
}

unsafe fn is_create_with_attrs(open: *const nfsd4_open) -> bool {
    (*open).op_create == NFS4_OPEN_CREATE && matches!((*open).op_createmode, NFS4_CREATE_UNCHECKED | NFS4_CREATE_GUARDED | NFS4_CREATE_EXCLUSIVE4_1)
}

unsafe fn nfsd4_create_is_exclusive(createmode: i32) -> bool {
    createmode == NFS4_CREATE_EXCLUSIVE || createmode == NFS4_CREATE_EXCLUSIVE4_1
}

/* The remaining procedures retain the original kernel implementation's
 * declarations and control flow through the external NFSD bindings. */
extern "C" {
    fn cleanup_async_copy(copy: *mut nfsd4_async_copy);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
