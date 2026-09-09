// Translated from hostfs_user_exp.c.
//
// The Linux kernel module header and hostfs declarations are supplied by the
// surrounding build.  These EXPORT_SYMBOL_GPL invocations register symbols
// defined elsewhere; Rust has no direct file-local equivalent for that kernel
// registration macro, so their intent is preserved here.

// EXPORT_SYMBOL_GPL(stat_file);
// EXPORT_SYMBOL_GPL(access_file);
// EXPORT_SYMBOL_GPL(open_file);
// EXPORT_SYMBOL_GPL(open_dir);
// EXPORT_SYMBOL_GPL(seek_dir);
// EXPORT_SYMBOL_GPL(read_dir);
// EXPORT_SYMBOL_GPL(read_file);
// EXPORT_SYMBOL_GPL(write_file);
// EXPORT_SYMBOL_GPL(lseek_file);
// EXPORT_SYMBOL_GPL(fsync_file);
// EXPORT_SYMBOL_GPL(replace_file);
// EXPORT_SYMBOL_GPL(close_file);
// EXPORT_SYMBOL_GPL(close_dir);
// EXPORT_SYMBOL_GPL(file_create);
// EXPORT_SYMBOL_GPL(set_attr);
// EXPORT_SYMBOL_GPL(make_symlink);
// EXPORT_SYMBOL_GPL(unlink_file);
// EXPORT_SYMBOL_GPL(do_mkdir);
// EXPORT_SYMBOL_GPL(hostfs_do_rmdir);
// EXPORT_SYMBOL_GPL(do_mknod);
// EXPORT_SYMBOL_GPL(link_file);
// EXPORT_SYMBOL_GPL(hostfs_do_readlink);
// EXPORT_SYMBOL_GPL(rename_file);
// EXPORT_SYMBOL_GPL(rename2_file);
// EXPORT_SYMBOL_GPL(do_statfs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
