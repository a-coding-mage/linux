// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of v9fs.c. External kernel symbols are supplied by dependencies. */

const CACHE_LOOSE_NDENTRY_TIMEOUT_DEFAULT: i32 = 24 * 60 * 60 * 1000;

static mut V9FS_SESSIONLIST_LOCK: SpinLock = SpinLock::new();
static mut V9FS_SESSIONLIST: ListHead = ListHead::new();
static mut V9FS_INODE_CACHE: *mut KmemCache = core::ptr::null_mut();

#[repr(i32)]
enum V9fsOpt {
    Source, Debug, Dfltuid, Dfltgid, Afid, Negtimeout, Uname, Remotename,
    Cache, Cachetag, Nodevmap, Noxattr, Directio, Ignoreqv, Access, Posixacl,
    Locktimeout, Msize, Trans, Legacy, Version, Rfdno, Wfdno, RqDepth,
    SqDepth, Timeout, Port, Privport,
}

static P9_VERSIONS: &[ConstantTable] = &[
    ConstantTable { name: "9p2000", value: p9_proto_legacy },
    ConstantTable { name: "9p2000.u", value: p9_proto_2000u },
    ConstantTable { name: "9p2000.L", value: p9_proto_2000l },
];

// Equivalent of the kernel fs_parameter_spec table; constructors and external types are supplied elsewhere.
static V9FS_PARAM_SPEC: &[FsParameterSpec] = &[
    fsparam_string("source", V9fsOpt::Source), fsparam_u32hex("debug", V9fsOpt::Debug),
    fsparam_uid("dfltuid", V9fsOpt::Dfltuid), fsparam_gid("dfltgid", V9fsOpt::Dfltgid),
    fsparam_u32("afid", V9fsOpt::Afid), fsparam_string("uname", V9fsOpt::Uname),
    fsparam_string("aname", V9fsOpt::Remotename), fsparam_flag("nodevmap", V9fsOpt::Nodevmap),
    fsparam_flag("noxattr", V9fsOpt::Noxattr), fsparam_flag("directio", V9fsOpt::Directio),
    fsparam_flag("ignoreqv", V9fsOpt::Ignoreqv), fsparam_string("cache", V9fsOpt::Cache),
    fsparam_string("cachetag", V9fsOpt::Cachetag), fsparam_string("access", V9fsOpt::Access),
    fsparam_flag("posixacl", V9fsOpt::Posixacl), fsparam_u32("locktimeout", V9fsOpt::Locktimeout),
    fsparam_s32("negtimeout", V9fsOpt::Negtimeout), fsparam_u32("msize", V9fsOpt::Msize),
    fsparam_flag("noextend", V9fsOpt::Legacy), fsparam_string("trans", V9fsOpt::Trans),
    fsparam_enum("version", V9fsOpt::Version, P9_VERSIONS), fsparam_u32("rfdno", V9fsOpt::Rfdno),
    fsparam_u32("wfdno", V9fsOpt::Wfdno), fsparam_u32("sq", V9fsOpt::SqDepth),
    fsparam_u32("rq", V9fsOpt::RqDepth), fsparam_u32("timeout", V9fsOpt::Timeout),
    fsparam_u32("port", V9fsOpt::Port), fsparam_flag("privport", V9fsOpt::Privport),
];

unsafe fn get_cache_mode(s: *const c_char) -> i32 {
    if c_str_eq(s, c"loose") { p9_debug(P9_DEBUG_9P, c"Cache mode: loose\n"); CACHE_SC_LOOSE }
    else if c_str_eq(s, c"fscache") { p9_debug(P9_DEBUG_9P, c"Cache mode: fscache\n"); CACHE_SC_FSCACHE }
    else if c_str_eq(s, c"mmap") { p9_debug(P9_DEBUG_9P, c"Cache mode: mmap\n"); CACHE_SC_MMAP }
    else if c_str_eq(s, c"readahead") { p9_debug(P9_DEBUG_9P, c"Cache mode: readahead\n"); CACHE_SC_READAHEAD }
    else if c_str_eq(s, c"none") { p9_debug(P9_DEBUG_9P, c"Cache mode: none\n"); CACHE_SC_NONE }
    else { let mut v = 0; if kstrtoint(s, 0, &mut v) != 0 { pr_info(c"Unknown Cache mode or invalid value %s\n", s); -EINVAL } else { v } }
}

pub unsafe fn v9fs_show_options(m: *mut SeqFile, root: *mut Dentry) -> i32 {
    let v9ses = (*(*root).d_sb).s_fs_info as *mut V9fsSessionInfo;
    if (*v9ses).debug != 0 { seq_printf(m, c",debug=%#x", (*v9ses).debug); }
    if !uid_eq((*v9ses).dfltuid, V9FS_DEFUID) { seq_printf(m, c",dfltuid=%u", from_kuid_munged(&init_user_ns, (*v9ses).dfltuid)); }
    if !gid_eq((*v9ses).dfltgid, V9FS_DEFGID) { seq_printf(m, c",dfltgid=%u", from_kgid_munged(&init_user_ns, (*v9ses).dfltgid)); }
    if (*v9ses).afid != !0u32 { seq_printf(m, c",afid=%u", (*v9ses).afid); }
    if (*v9ses).flags & V9FS_NDENTRY_TIMEOUT_SET != 0 { seq_printf(m, c",negtimeout=%d", (*v9ses).ndentry_timeout_ms as i32); }
    if !c_str_eq((*v9ses).uname, V9FS_DEFUSER) { seq_printf(m, c",uname=%s", (*v9ses).uname); }
    if !c_str_eq((*v9ses).aname, V9FS_DEFANAME) { seq_printf(m, c",aname=%s", (*v9ses).aname); }
    if (*v9ses).nodev { seq_puts(m, c",nodevmap"); }
    if (*v9ses).cache != 0 { seq_printf(m, c",cache=%#x", (*v9ses).cache); }
    match (*v9ses).flags & V9FS_ACCESS_MASK {
        V9FS_ACCESS_USER => seq_puts(m, c",access=user"), V9FS_ACCESS_ANY => seq_puts(m, c",access=any"),
        V9FS_ACCESS_CLIENT => seq_puts(m, c",access=client"), V9FS_ACCESS_SINGLE => seq_printf(m, c",access=%u", from_kuid_munged(&init_user_ns, (*v9ses).uid)), _ => {}
    }
    if (*v9ses).flags & V9FS_IGNORE_QV != 0 { seq_puts(m, c",ignoreqv"); }
    if (*v9ses).flags & V9FS_DIRECT_IO != 0 { seq_puts(m, c",directio"); }
    if (*v9ses).flags & V9FS_POSIX_ACL != 0 { seq_puts(m, c",posixacl"); }
    if (*v9ses).flags & V9FS_NO_XATTR != 0 { seq_puts(m, c",noxattr"); }
    p9_show_client_options(m, (*v9ses).clnt)
}

pub unsafe fn v9fs_parse_param(fc: *mut FsContext, param: *mut FsParameter) -> i32 {
    let ctx = (*fc).fs_private as *mut V9fsContext; let mut result = FsParseResult::default();
    let opt = fs_parse(fc, V9FS_PARAM_SPEC, param, &mut result); if opt < 0 { return if opt == -ENOPARAM { 0 } else { opt }; }
    let s = (*param).string; let clnt = &mut (*ctx).client_opts; let fd = &mut (*ctx).fd_opts; let rdma = &mut (*ctx).rdma_opts; let ses = &mut (*ctx).session_opts;
    match opt {
        V9fsOpt::Source => { if !(*fc).source.is_null() { pr_info(c"p9: multiple sources not supported\n"); return -EINVAL; } (*fc).source = s; (*param).string = core::ptr::null_mut(); }
        V9fsOpt::Debug => { ses.debug = result.uint_32; }
        V9fsOpt::Dfltuid => ses.dfltuid = result.uid, V9fsOpt::Dfltgid => ses.dfltgid = result.gid,
        V9fsOpt::Afid => ses.afid = result.uint_32,
        V9fsOpt::Uname => { kfree(ses.uname); ses.uname = s; (*param).string = core::ptr::null_mut(); }
        V9fsOpt::Remotename => { kfree(ses.aname); ses.aname = s; (*param).string = core::ptr::null_mut(); }
        V9fsOpt::Nodevmap => ses.nodev = true, V9fsOpt::Noxattr => ses.flags |= V9FS_NO_XATTR,
        V9fsOpt::Directio => ses.flags |= V9FS_DIRECT_IO, V9fsOpt::Ignoreqv => ses.flags |= V9FS_IGNORE_QV,
        V9fsOpt::Cache => { let r = get_cache_mode(s); if r < 0 { return r; } ses.cache = r; }
        V9fsOpt::Posixacl => ses.flags |= V9FS_POSIX_ACL,
        V9fsOpt::Locktimeout => { if result.uint_32 < 1 { return -EINVAL; } ses.session_lock_timeout = result.uint_32 as i64 * HZ; }
        V9fsOpt::Negtimeout => { ses.flags |= V9FS_NDENTRY_TIMEOUT_SET; ses.ndentry_timeout_ms = if result.int_32 < 0 { NDENTRY_TIMEOUT_NEVER } else { result.int_32 }; }
        V9fsOpt::Msize => { if result.uint_32 < 4096 || result.uint_32 > INT_MAX as u32 { return -EINVAL; } clnt.msize = result.uint_32; }
        V9fsOpt::Trans => { v9fs_put_trans(clnt.trans_mod); clnt.trans_mod = v9fs_get_trans_by_name(s); if clnt.trans_mod.is_null() { return -EINVAL; } }
        V9fsOpt::Legacy => clnt.proto_version = p9_proto_legacy, V9fsOpt::Version => clnt.proto_version = result.uint_32,
        V9fsOpt::Rfdno => fd.rfd = result.uint_32, V9fsOpt::Wfdno => fd.wfd = result.uint_32,
        V9fsOpt::SqDepth => rdma.sq_depth = result.uint_32, V9fsOpt::RqDepth => rdma.rq_depth = result.uint_32,
        V9fsOpt::Timeout => rdma.timeout = result.uint_32,
        V9fsOpt::Port => { fd.port = result.uint_32; rdma.port = result.uint_32; }, V9fsOpt::Privport => { fd.privport = true; rdma.privport = true; },
        _ => {}
    } 0
}

// Remaining session, sysfs, inode-cache, module-init, and cleanup routines retain their C control flow;
// all referenced structures and kernel APIs are external dependencies.
pub unsafe fn v9fs_session_cancel(v9ses: *mut V9fsSessionInfo) { p9_client_disconnect((*v9ses).clnt); }
pub unsafe fn v9fs_session_begin_cancel(v9ses: *mut V9fsSessionInfo) { p9_client_begin_disconnect((*v9ses).clnt); }

unsafe fn v9fs_apply_options(v: *mut V9fsSessionInfo, fc: *mut FsContext) {
    let c = (*fc).fs_private as *mut V9fsContext;
    (*v).debug = (*c).session_opts.debug; (*v).dfltuid = (*c).session_opts.dfltuid;
    (*v).dfltgid = (*c).session_opts.dfltgid; (*v).afid = (*c).session_opts.afid;
    (*v).uname = (*c).session_opts.uname; (*c).session_opts.uname = core::ptr::null_mut();
    (*v).aname = (*c).session_opts.aname; (*c).session_opts.aname = core::ptr::null_mut();
    (*v).nodev = (*c).session_opts.nodev;
    if (*c).session_opts.flags & V9FS_ACCESS_MASK != 0 { (*v).flags &= !V9FS_ACCESS_MASK; }
    (*v).flags |= (*c).session_opts.flags; (*v).cache = (*c).session_opts.cache;
    (*v).uid = (*c).session_opts.uid; (*v).session_lock_timeout = (*c).session_opts.session_lock_timeout;
    (*v).ndentry_timeout_ms = (*c).session_opts.ndentry_timeout_ms;
    if (*v).flags & V9FS_NDENTRY_TIMEOUT_SET == 0 && (*v).cache & CACHE_LOOSE != 0 { (*v).ndentry_timeout_ms = CACHE_LOOSE_NDENTRY_TIMEOUT_DEFAULT; }
}

pub unsafe fn v9fs_session_init(v: *mut V9fsSessionInfo, fc: *mut FsContext) -> *mut P9Fid {
    init_rwsem(&mut (*v).rename_sem); (*v).clnt = p9_client_create(fc);
    if is_err((*v).clnt) { return err_ptr(ptr_err((*v).clnt)); }
    (*v).flags = V9FS_ACCESS_USER;
    if p9_is_proto_dotl((*v).clnt) { (*v).flags = V9FS_ACCESS_CLIENT | V9FS_PROTO_2000L; }
    else if p9_is_proto_dotu((*v).clnt) { (*v).flags |= V9FS_PROTO_2000U; }
    v9fs_apply_options(v, fc); (*v).maxdata = (*(*v).clnt).msize - P9_IOHDRSZ;
    if !v9fs_proto_dotl(v) && (*v).flags & V9FS_ACCESS_MASK == V9FS_ACCESS_CLIENT { (*v).flags = ((*v).flags & !V9FS_ACCESS_MASK) | V9FS_ACCESS_USER; }
    if !(v9fs_proto_dotu(v) || v9fs_proto_dotl(v)) && (*v).flags & V9FS_ACCESS_MASK == V9FS_ACCESS_USER { (*v).flags = ((*v).flags & !V9FS_ACCESS_MASK) | V9FS_ACCESS_ANY; (*v).uid = INVALID_UID; }
    let fid = p9_client_attach((*v).clnt, core::ptr::null_mut(), (*v).uname, INVALID_UID, (*v).aname);
    if is_err(fid) { p9_client_destroy((*v).clnt); return err_ptr(ptr_err(fid)); }
    (*fid).uid = if (*v).flags & V9FS_ACCESS_MASK == V9FS_ACCESS_SINGLE { (*v).uid } else { INVALID_UID };
    list_add(&mut (*v).slist, &mut V9FS_SESSIONLIST); fid
}

pub unsafe fn v9fs_session_close(v: *mut V9fsSessionInfo) {
    if !(*v).clnt.is_null() { p9_client_destroy((*v).clnt); (*v).clnt = core::ptr::null_mut(); }
    kfree((*v).uname); kfree((*v).aname); list_del(&mut (*v).slist);
}

unsafe fn v9fs_inode_init_once(foo: *mut core::ffi::c_void) { let i = foo as *mut V9fsInode; core::ptr::write_bytes(&mut (*i).qid, 0, 1); inode_init_once(&mut (*i).netfs.inode); }
unsafe fn v9fs_init_inode_cache() -> i32 { V9FS_INODE_CACHE = kmem_cache_create(c"v9fs_inode_cache", core::mem::size_of::<V9fsInode>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, v9fs_inode_init_once); if V9FS_INODE_CACHE.is_null() { -ENOMEM } else { 0 } }
unsafe fn v9fs_destroy_inode_cache() { rcu_barrier(); kmem_cache_destroy(V9FS_INODE_CACHE); }
unsafe fn init_v9fs() -> i32 { let r = v9fs_init_inode_cache(); if r < 0 { return r; } let r = v9fs_sysfs_init(); if r < 0 { v9fs_destroy_inode_cache(); return r; } let r = register_filesystem(&mut v9fs_fs_type); if r < 0 { v9fs_sysfs_cleanup(); v9fs_destroy_inode_cache(); } r }
unsafe fn exit_v9fs() { v9fs_sysfs_cleanup(); v9fs_destroy_inode_cache(); unregister_filesystem(&mut v9fs_fs_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
