// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfsplus/options.c
 *
 * Copyright (C) 2001
 * Brad Boyer (flar@allandria.com)
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 *
 * Option parsing
 */

// Linux kernel headers are represented by the corresponding external Rust
// declarations supplied by the surrounding translation unit.

const opt_creator: i32 = 0;
const opt_type: i32 = 1;
const opt_umask: i32 = 2;
const opt_uid: i32 = 3;
const opt_gid: i32 = 4;
const opt_part: i32 = 5;
const opt_session: i32 = 6;
const opt_nls: i32 = 7;
const opt_decompose: i32 = 8;
const opt_barrier: i32 = 9;
const opt_force: i32 = 10;

/*
 * The entries below are the direct Rust-side record of the C fsparam macros:
 * fsparam_string("creator", opt_creator), fsparam_string("type", opt_type),
 * fsparam_u32oct("umask", opt_umask), fsparam_u32("uid", opt_uid),
 * fsparam_u32("gid", opt_gid), fsparam_u32("part", opt_part),
 * fsparam_u32("session", opt_session), fsparam_string("nls", opt_nls),
 * fsparam_flag_no("decompose", opt_decompose),
 * fsparam_flag_no("barrier", opt_barrier), fsparam_flag("force", opt_force),
 * {}
 */
extern "C" {
    static hfs_param_spec: fs_parameter_spec;
}

/* Initialize an options object to reasonable defaults */
pub unsafe fn hfsplus_fill_defaults(opts: *mut hfsplus_sb_info) {
    if opts.is_null() {
        return;
    }

    (*opts).creator = HFSPLUS_DEF_CR_TYPE;
    (*opts).type_ = HFSPLUS_DEF_CR_TYPE;
    (*opts).umask = current_umask();
    (*opts).uid = current_uid();
    (*opts).gid = current_gid();
    (*opts).part = -1;
    (*opts).session = -1;
}

/* Parse options from mount. Returns nonzero errno on failure */
pub unsafe fn hfsplus_parse_param(
    fc: *mut fs_context,
    param: *mut fs_parameter,
) -> i32 {
    let sbi = (*fc).s_fs_info as *mut hfsplus_sb_info;
    let mut result: fs_parse_result = core::mem::zeroed();
    let opt: i32;

    /*
     * Only the force option is examined during remount, all others
     * are ignored.
     */
    if (*fc).purpose == FS_CONTEXT_FOR_RECONFIGURE
        && strncmp((*param).key, b"force\0".as_ptr() as *const i8, 5) != 0
    {
        return 0;
    }

    opt = fs_parse(fc, &hfs_param_spec, param, &mut result);
    if opt < 0 {
        return opt;
    }

    match opt {
        x if x == opt_creator => {
            if strlen((*param).string) != 4 {
                pr_err(b"creator requires a 4 character value\n\0");
                return -EINVAL;
            }
            core::ptr::copy_nonoverlapping(
                (*param).string as *const u8,
                &mut (*sbi).creator as *mut _ as *mut u8,
                4,
            );
        }
        x if x == opt_type => {
            if strlen((*param).string) != 4 {
                pr_err(b"type requires a 4 character value\n\0");
                return -EINVAL;
            }
            core::ptr::copy_nonoverlapping(
                (*param).string as *const u8,
                &mut (*sbi).type_ as *mut _ as *mut u8,
                4,
            );
        }
        x if x == opt_umask => (*sbi).umask = result.uint_32 as umode_t,
        x if x == opt_uid => {
            (*sbi).uid = result.uid;
            set_bit(HFSPLUS_SB_UID, &mut (*sbi).flags);
        }
        x if x == opt_gid => {
            (*sbi).gid = result.gid;
            set_bit(HFSPLUS_SB_GID, &mut (*sbi).flags);
        }
        x if x == opt_part => (*sbi).part = result.uint_32,
        x if x == opt_session => (*sbi).session = result.uint_32,
        x if x == opt_nls => {
            if !(*sbi).nls.is_null() {
                pr_err(b"unable to change nls mapping\n\0");
                return -EINVAL;
            }
            (*sbi).nls = load_nls((*param).string);
            if (*sbi).nls.is_null() {
                pr_err(b"unable to load nls mapping \"%s\"\n\0", (*param).string);
                return -EINVAL;
            }
        }
        x if x == opt_decompose => {
            if result.negated {
                set_bit(HFSPLUS_SB_NODECOMPOSE, &mut (*sbi).flags);
            } else {
                clear_bit(HFSPLUS_SB_NODECOMPOSE, &mut (*sbi).flags);
            }
        }
        x if x == opt_barrier => {
            if result.negated {
                set_bit(HFSPLUS_SB_NOBARRIER, &mut (*sbi).flags);
            } else {
                clear_bit(HFSPLUS_SB_NOBARRIER, &mut (*sbi).flags);
            }
        }
        x if x == opt_force => set_bit(HFSPLUS_SB_FORCE, &mut (*sbi).flags),
        _ => return -EINVAL,
    }

    0
}

pub unsafe fn hfsplus_show_options(seq: *mut seq_file, root: *mut dentry) -> i32 {
    let sbi = HFSPLUS_SB((*root).d_sb);

    if (*sbi).creator != HFSPLUS_DEF_CR_TYPE {
        seq_show_option_n(seq, b"creator\0".as_ptr() as *const i8, &(*sbi).creator as *const _ as *const i8, 4);
    }
    if (*sbi).type_ != HFSPLUS_DEF_CR_TYPE {
        seq_show_option_n(seq, b"type\0".as_ptr() as *const i8, &(*sbi).type_ as *const _ as *const i8, 4);
    }
    seq_printf(seq, b",umask=%o,uid=%u,gid=%u\0".as_ptr() as *const i8, (*sbi).umask,
        from_kuid_munged(&init_user_ns, (*sbi).uid),
        from_kgid_munged(&init_user_ns, (*sbi).gid));
    if (*sbi).part >= 0 { seq_printf(seq, b",part=%u\0".as_ptr() as *const i8, (*sbi).part); }
    if (*sbi).session >= 0 { seq_printf(seq, b",session=%u\0".as_ptr() as *const i8, (*sbi).session); }
    if !(*sbi).nls.is_null() { seq_printf(seq, b",nls=%s\0".as_ptr() as *const i8, (*sbi).nls.charset); }
    if test_bit(HFSPLUS_SB_NODECOMPOSE, &(*sbi).flags) { seq_puts(seq, b",nodecompose\0".as_ptr() as *const i8); }
    if test_bit(HFSPLUS_SB_NOBARRIER, &(*sbi).flags) { seq_puts(seq, b",nobarrier\0".as_ptr() as *const i8); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
