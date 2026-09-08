/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of security/tomoyo/common.h. Kernel includes and build
 * attributes are dependencies supplied by the surrounding translation unit. */

pub const TOMOYO_HASH_BITS: u32 = 8;
pub const TOMOYO_MAX_HASH: u32 = 1u32 << TOMOYO_HASH_BITS;
pub const TOMOYO_SOCK_MAX: u32 = 6;
pub const TOMOYO_EXEC_TMPSIZE: usize = 4096;
pub const TOMOYO_GC_IN_PROGRESS: i8 = -1;
pub const TOMOYO_MAX_PROFILES: usize = 256;
pub const TOMOYO_MAX_ACL_GROUPS: usize = 256;
pub const TOMOYO_RETRY_REQUEST: i32 = 1;
pub const TOMOYO_MAX_IO_READ_QUEUE: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tomoyo_conditions_index { TOMOYO_TASK_UID, TOMOYO_TASK_EUID, TOMOYO_TASK_SUID, TOMOYO_TASK_FSUID, TOMOYO_TASK_GID, TOMOYO_TASK_EGID, TOMOYO_TASK_SGID, TOMOYO_TASK_FSGID, TOMOYO_TASK_PID, TOMOYO_TASK_PPID, TOMOYO_EXEC_ARGC, TOMOYO_EXEC_ENVC, TOMOYO_TYPE_IS_SOCKET, TOMOYO_TYPE_IS_SYMLINK, TOMOYO_TYPE_IS_FILE, TOMOYO_TYPE_IS_BLOCK_DEV, TOMOYO_TYPE_IS_DIRECTORY, TOMOYO_TYPE_IS_CHAR_DEV, TOMOYO_TYPE_IS_FIFO, TOMOYO_MODE_SETUID, TOMOYO_MODE_SETGID, TOMOYO_MODE_STICKY, TOMOYO_MODE_OWNER_READ, TOMOYO_MODE_OWNER_WRITE, TOMOYO_MODE_OWNER_EXECUTE, TOMOYO_MODE_GROUP_READ, TOMOYO_MODE_GROUP_WRITE, TOMOYO_MODE_GROUP_EXECUTE, TOMOYO_MODE_OTHERS_READ, TOMOYO_MODE_OTHERS_WRITE, TOMOYO_MODE_OTHERS_EXECUTE, TOMOYO_EXEC_REALPATH, TOMOYO_SYMLINK_TARGET, TOMOYO_PATH1_UID, TOMOYO_PATH1_GID, TOMOYO_PATH1_INO, TOMOYO_PATH1_MAJOR, TOMOYO_PATH1_MINOR, TOMOYO_PATH1_PERM, TOMOYO_PATH1_TYPE, TOMOYO_PATH1_DEV_MAJOR, TOMOYO_PATH1_DEV_MINOR, TOMOYO_PATH2_UID, TOMOYO_PATH2_GID, TOMOYO_PATH2_INO, TOMOYO_PATH2_MAJOR, TOMOYO_PATH2_MINOR, TOMOYO_PATH2_PERM, TOMOYO_PATH2_TYPE, TOMOYO_PATH2_DEV_MAJOR, TOMOYO_PATH2_DEV_MINOR, TOMOYO_PATH1_PARENT_UID, TOMOYO_PATH1_PARENT_GID, TOMOYO_PATH1_PARENT_INO, TOMOYO_PATH1_PARENT_PERM, TOMOYO_PATH2_PARENT_UID, TOMOYO_PATH2_PARENT_GID, TOMOYO_PATH2_PARENT_INO, TOMOYO_PATH2_PARENT_PERM, TOMOYO_MAX_CONDITION_KEYWORD, TOMOYO_NUMBER_UNION, TOMOYO_NAME_UNION, TOMOYO_ARGV_ENTRY, TOMOYO_ENVP_ENTRY }

#[repr(C)] pub enum tomoyo_path_stat_index { TOMOYO_PATH1, TOMOYO_PATH1_PARENT, TOMOYO_PATH2, TOMOYO_PATH2_PARENT, TOMOYO_MAX_PATH_STAT }
#[repr(C)] pub enum tomoyo_mode_index { TOMOYO_CONFIG_DISABLED, TOMOYO_CONFIG_LEARNING, TOMOYO_CONFIG_PERMISSIVE, TOMOYO_CONFIG_ENFORCING, TOMOYO_CONFIG_MAX_MODE, TOMOYO_CONFIG_WANT_REJECT_LOG = 64, TOMOYO_CONFIG_WANT_GRANT_LOG = 128, TOMOYO_CONFIG_USE_DEFAULT = 255 }
#[repr(C)] pub enum tomoyo_policy_id { TOMOYO_ID_GROUP, TOMOYO_ID_ADDRESS_GROUP, TOMOYO_ID_PATH_GROUP, TOMOYO_ID_NUMBER_GROUP, TOMOYO_ID_TRANSITION_CONTROL, TOMOYO_ID_AGGREGATOR, TOMOYO_ID_MANAGER, TOMOYO_ID_CONDITION, TOMOYO_ID_NAME, TOMOYO_ID_ACL, TOMOYO_ID_DOMAIN, TOMOYO_MAX_POLICY }
#[repr(C)] pub enum tomoyo_domain_info_flags_index { TOMOYO_DIF_QUOTA_WARNED, TOMOYO_DIF_TRANSITION_FAILED, TOMOYO_MAX_DOMAIN_INFO_FLAGS }
#[repr(C)] pub enum tomoyo_grant_log { TOMOYO_GRANTLOG_AUTO, TOMOYO_GRANTLOG_NO, TOMOYO_GRANTLOG_YES }
#[repr(C)] pub enum tomoyo_group_id { TOMOYO_PATH_GROUP, TOMOYO_NUMBER_GROUP, TOMOYO_ADDRESS_GROUP, TOMOYO_MAX_GROUP }
#[repr(C)] pub enum tomoyo_value_type { TOMOYO_VALUE_TYPE_INVALID, TOMOYO_VALUE_TYPE_DECIMAL, TOMOYO_VALUE_TYPE_OCTAL, TOMOYO_VALUE_TYPE_HEXADECIMAL }
#[repr(C)] pub enum tomoyo_transition_type { TOMOYO_TRANSITION_CONTROL_NO_RESET, TOMOYO_TRANSITION_CONTROL_RESET, TOMOYO_TRANSITION_CONTROL_NO_INITIALIZE, TOMOYO_TRANSITION_CONTROL_INITIALIZE, TOMOYO_TRANSITION_CONTROL_NO_KEEP, TOMOYO_TRANSITION_CONTROL_KEEP, TOMOYO_MAX_TRANSITION_TYPE }
#[repr(C)] pub enum tomoyo_acl_entry_type_index { TOMOYO_TYPE_PATH_ACL, TOMOYO_TYPE_PATH2_ACL, TOMOYO_TYPE_PATH_NUMBER_ACL, TOMOYO_TYPE_MKDEV_ACL, TOMOYO_TYPE_MOUNT_ACL, TOMOYO_TYPE_INET_ACL, TOMOYO_TYPE_UNIX_ACL, TOMOYO_TYPE_ENV_ACL, TOMOYO_TYPE_MANUAL_TASK_ACL }
#[repr(C)] pub enum tomoyo_path_acl_index { TOMOYO_TYPE_EXECUTE, TOMOYO_TYPE_READ, TOMOYO_TYPE_WRITE, TOMOYO_TYPE_APPEND, TOMOYO_TYPE_UNLINK, TOMOYO_TYPE_GETATTR, TOMOYO_TYPE_RMDIR, TOMOYO_TYPE_TRUNCATE, TOMOYO_TYPE_SYMLINK, TOMOYO_TYPE_CHROOT, TOMOYO_TYPE_UMOUNT, TOMOYO_MAX_PATH_OPERATION }
#[repr(C)] pub enum tomoyo_memory_stat_type { TOMOYO_MEMORY_POLICY, TOMOYO_MEMORY_AUDIT, TOMOYO_MEMORY_QUERY, TOMOYO_MAX_MEMORY_STAT }
#[repr(C)] pub enum tomoyo_mkdev_acl_index { TOMOYO_TYPE_MKBLOCK, TOMOYO_TYPE_MKCHAR, TOMOYO_MAX_MKDEV_OPERATION }
#[repr(C)] pub enum tomoyo_network_acl_index { TOMOYO_NETWORK_BIND, TOMOYO_NETWORK_LISTEN, TOMOYO_NETWORK_CONNECT, TOMOYO_NETWORK_SEND, TOMOYO_MAX_NETWORK_OPERATION }
#[repr(C)] pub enum tomoyo_path2_acl_index { TOMOYO_TYPE_LINK, TOMOYO_TYPE_RENAME, TOMOYO_TYPE_PIVOT_ROOT, TOMOYO_MAX_PATH2_OPERATION }
#[repr(C)] pub enum tomoyo_path_number_acl_index { TOMOYO_TYPE_CREATE, TOMOYO_TYPE_MKDIR, TOMOYO_TYPE_MKFIFO, TOMOYO_TYPE_MKSOCK, TOMOYO_TYPE_IOCTL, TOMOYO_TYPE_CHMOD, TOMOYO_TYPE_CHOWN, TOMOYO_TYPE_CHGRP, TOMOYO_MAX_PATH_NUMBER_OPERATION }
#[repr(C)] pub enum tomoyo_securityfs_interface_index { TOMOYO_DOMAINPOLICY, TOMOYO_EXCEPTIONPOLICY, TOMOYO_PROCESS_STATUS, TOMOYO_STAT, TOMOYO_AUDIT, TOMOYO_VERSION, TOMOYO_PROFILE, TOMOYO_QUERY, TOMOYO_MANAGER }
#[repr(C)] pub enum tomoyo_special_mount { TOMOYO_MOUNT_BIND, TOMOYO_MOUNT_MOVE, TOMOYO_MOUNT_REMOUNT, TOMOYO_MOUNT_MAKE_UNBINDABLE, TOMOYO_MOUNT_MAKE_PRIVATE, TOMOYO_MOUNT_MAKE_SLAVE, TOMOYO_MOUNT_MAKE_SHARED, TOMOYO_MAX_SPECIAL_MOUNT }
#[repr(C)] pub enum tomoyo_mac_index { TOMOYO_MAC_FILE_EXECUTE, TOMOYO_MAC_FILE_OPEN, TOMOYO_MAC_FILE_CREATE, TOMOYO_MAC_FILE_UNLINK, TOMOYO_MAC_FILE_GETATTR, TOMOYO_MAC_FILE_MKDIR, TOMOYO_MAC_FILE_RMDIR, TOMOYO_MAC_FILE_MKFIFO, TOMOYO_MAC_FILE_MKSOCK, TOMOYO_MAC_FILE_TRUNCATE, TOMOYO_MAC_FILE_SYMLINK, TOMOYO_MAC_FILE_MKBLOCK, TOMOYO_MAC_FILE_MKCHAR, TOMOYO_MAC_FILE_LINK, TOMOYO_MAC_FILE_RENAME, TOMOYO_MAC_FILE_CHMOD, TOMOYO_MAC_FILE_CHOWN, TOMOYO_MAC_FILE_CHGRP, TOMOYO_MAC_FILE_IOCTL, TOMOYO_MAC_FILE_CHROOT, TOMOYO_MAC_FILE_MOUNT, TOMOYO_MAC_FILE_UMOUNT, TOMOYO_MAC_FILE_PIVOT_ROOT, TOMOYO_MAC_NETWORK_INET_STREAM_BIND, TOMOYO_MAC_NETWORK_INET_STREAM_LISTEN, TOMOYO_MAC_NETWORK_INET_STREAM_CONNECT, TOMOYO_MAC_NETWORK_INET_DGRAM_BIND, TOMOYO_MAC_NETWORK_INET_DGRAM_SEND, TOMOYO_MAC_NETWORK_INET_RAW_BIND, TOMOYO_MAC_NETWORK_INET_RAW_SEND, TOMOYO_MAC_NETWORK_UNIX_STREAM_BIND, TOMOYO_MAC_NETWORK_UNIX_STREAM_LISTEN, TOMOYO_MAC_NETWORK_UNIX_STREAM_CONNECT, TOMOYO_MAC_NETWORK_UNIX_DGRAM_BIND, TOMOYO_MAC_NETWORK_UNIX_DGRAM_SEND, TOMOYO_MAC_NETWORK_UNIX_SEQPACKET_BIND, TOMOYO_MAC_NETWORK_UNIX_SEQPACKET_LISTEN, TOMOYO_MAC_NETWORK_UNIX_SEQPACKET_CONNECT, TOMOYO_MAC_ENVIRON, TOMOYO_MAX_MAC_INDEX }
#[repr(C)] pub enum tomoyo_mac_category_index { TOMOYO_MAC_CATEGORY_FILE, TOMOYO_MAC_CATEGORY_NETWORK, TOMOYO_MAC_CATEGORY_MISC, TOMOYO_MAX_MAC_CATEGORY_INDEX }
#[repr(C)] pub enum tomoyo_policy_stat_type { TOMOYO_STAT_POLICY_UPDATES, TOMOYO_STAT_POLICY_LEARNING, TOMOYO_STAT_POLICY_PERMISSIVE, TOMOYO_STAT_POLICY_ENFORCING, TOMOYO_MAX_POLICY_STAT }
#[repr(C)] pub enum tomoyo_pref_index { TOMOYO_PREF_MAX_AUDIT_LOG, TOMOYO_PREF_MAX_LEARNING_ENTRY, TOMOYO_MAX_PREF }

/* Kernel types are supplied by the including translation unit. */
extern "C" {
    pub static mut tomoyo_policy_loaded: bool;
    pub static mut tomoyo_enabled: i32;
}

#[repr(C)] pub struct tomoyo_acl_head { pub list: list_head, pub is_deleted: i8 }
#[repr(C)] pub struct tomoyo_shared_acl_head { pub list: list_head, pub users: atomic_t }
#[repr(C)] pub struct tomoyo_path_info { pub name: *const c_char, pub hash: u32, pub const_len: u16, pub is_dir: bool, pub is_patterned: bool }
#[repr(C)] pub struct tomoyo_name { pub head: tomoyo_shared_acl_head, pub entry: tomoyo_path_info }
#[repr(C)] pub struct tomoyo_name_union { pub filename: *const tomoyo_path_info, pub group: *mut tomoyo_group }
#[repr(C)] pub struct tomoyo_number_union { pub values: [c_ulong; 2], pub group: *mut tomoyo_group, pub value_type: [u8; 2] }
#[repr(C)] pub struct tomoyo_ipaddr_union { pub ip: [in6_addr; 2], pub group: *mut tomoyo_group, pub is_ipv6: bool }
#[repr(C)] pub struct tomoyo_group { pub head: tomoyo_shared_acl_head, pub group_name: *const tomoyo_path_info, pub member_list: list_head }
#[repr(C)] pub struct tomoyo_path_group { pub head: tomoyo_acl_head, pub member_name: *const tomoyo_path_info }
#[repr(C)] pub struct tomoyo_number_group { pub head: tomoyo_acl_head, pub number: tomoyo_number_union }
#[repr(C)] pub struct tomoyo_address_group { pub head: tomoyo_acl_head, pub address: tomoyo_ipaddr_union }
#[repr(C)] pub struct tomoyo_mini_stat { pub uid: kuid_t, pub gid: kgid_t, pub ino: u64, pub mode: umode_t, pub dev: dev_t, pub rdev: dev_t }
#[repr(C)] pub struct tomoyo_page_dump { pub page: *mut page, pub data: *mut c_char }
#[repr(C)] pub struct tomoyo_argv { pub index: c_ulong, pub value: *const tomoyo_path_info, pub is_not: bool }
#[repr(C)] pub struct tomoyo_envp { pub name: *const tomoyo_path_info, pub value: *const tomoyo_path_info, pub is_not: bool }
#[repr(C)] pub struct tomoyo_condition_element { pub left: u8, pub right: u8, pub equals: bool }
#[repr(C)] pub struct tomoyo_condition { pub head: tomoyo_shared_acl_head, pub size: u32, pub condc: u16, pub numbers_count: u16, pub names_count: u16, pub argc: u16, pub envc: u16, pub grant_log: u8, pub transit: *const tomoyo_path_info }
#[repr(C)] pub struct tomoyo_acl_info { pub list: list_head, pub cond: *mut tomoyo_condition, pub is_deleted: i8, pub type_: u8 }
#[repr(C)] pub struct tomoyo_acl_param { pub data: *mut c_char, pub list: *mut list_head, pub ns: *mut tomoyo_policy_namespace, pub is_delete: bool }
#[repr(C)] pub struct tomoyo_preference { pub learning_max_entry: c_uint, pub enforcing_verbose: bool, pub learning_verbose: bool, pub permissive_verbose: bool }
#[repr(C)] pub struct tomoyo_profile { pub comment: *const tomoyo_path_info, pub learning: *mut tomoyo_preference, pub permissive: *mut tomoyo_preference, pub enforcing: *mut tomoyo_preference, pub preference: tomoyo_preference, pub default_config: u8, pub config: [u8; 42], pub pref: [c_uint; 2] }
#[repr(C)] pub struct tomoyo_time { pub year: u16, pub month: u8, pub day: u8, pub hour: u8, pub min: u8, pub sec: u8 }
#[repr(C)] pub struct tomoyo_policy_namespace { pub profile_ptr: [*mut tomoyo_profile; 256], pub group_list: [list_head; 3], pub policy_list: [list_head; 12], pub acl_group: [list_head; 256], pub namespace_list: list_head, pub profile_version: c_uint, pub name: *const c_char }
#[repr(C)] pub struct tomoyo_task { pub domain_info: *mut tomoyo_domain_info, pub old_domain_info: *mut tomoyo_domain_info }

#[repr(C)] pub struct tomoyo_request_info { pub obj: *mut tomoyo_obj_info, pub ee: *mut tomoyo_execve, pub domain: *mut tomoyo_domain_info, pub param: tomoyo_request_param, pub matched_acl: *mut tomoyo_acl_info, pub param_type: u8, pub granted: bool, pub retry: u8, pub profile: u8, pub mode: u8, pub type_: u8 }
#[repr(C)] pub union tomoyo_request_param { pub path: tomoyo_request_path, pub path2: tomoyo_request_path2, pub mkdev: tomoyo_request_mkdev, pub path_number: tomoyo_request_path_number, pub environ: tomoyo_request_environ, pub inet_network: tomoyo_request_inet, pub unix_network: tomoyo_request_unix, pub mount: tomoyo_request_mount, pub task: tomoyo_request_task }
#[repr(C)] pub struct tomoyo_request_path { pub filename: *const tomoyo_path_info, pub matched_path: *const tomoyo_path_info, pub operation: u8 }
#[repr(C)] pub struct tomoyo_request_path2 { pub filename1: *const tomoyo_path_info, pub filename2: *const tomoyo_path_info, pub operation: u8 }
#[repr(C)] pub struct tomoyo_request_mkdev { pub filename: *const tomoyo_path_info, pub mode: c_uint, pub major: c_uint, pub minor: c_uint, pub operation: u8 }
#[repr(C)] pub struct tomoyo_request_path_number { pub filename: *const tomoyo_path_info, pub number: c_ulong, pub operation: u8 }
#[repr(C)] pub struct tomoyo_request_environ { pub name: *const tomoyo_path_info }
#[repr(C)] pub struct tomoyo_request_inet { pub address: *const __be32, pub port: u16, pub protocol: u8, pub operation: u8, pub is_ipv6: bool }
#[repr(C)] pub struct tomoyo_request_unix { pub address: *const tomoyo_path_info, pub protocol: u8, pub operation: u8 }
#[repr(C)] pub struct tomoyo_request_mount { pub type_: *const tomoyo_path_info, pub dir: *const tomoyo_path_info, pub dev: *const tomoyo_path_info, pub flags: c_ulong, pub need_dev: c_int }
#[repr(C)] pub struct tomoyo_request_task { pub domainname: *const tomoyo_path_info }
#[repr(C)] pub struct tomoyo_obj_info { pub validate_done: bool, pub stat_valid: [bool; 4], pub path1: path, pub path2: path, pub stat: [tomoyo_mini_stat; 4], pub symlink_target: *mut tomoyo_path_info }
#[repr(C)] pub struct tomoyo_execve { pub r: tomoyo_request_info, pub obj: tomoyo_obj_info, pub bprm: *mut linux_binprm, pub transition: *const tomoyo_path_info, pub dump: tomoyo_page_dump, pub tmp: *mut c_char }
#[repr(C)] pub struct tomoyo_domain_info { pub list: list_head, pub acl_info_list: list_head, pub domainname: *const tomoyo_path_info, pub ns: *mut tomoyo_policy_namespace, pub group: [c_ulong; 4], pub profile: u8, pub is_deleted: bool, pub flags: [bool; 2], pub users: atomic_t }
#[repr(C)] pub struct tomoyo_task_acl { pub head: tomoyo_acl_info, pub domainname: *const tomoyo_path_info }
#[repr(C)] pub struct tomoyo_path_acl { pub head: tomoyo_acl_info, pub perm: u16, pub name: tomoyo_name_union }
#[repr(C)] pub struct tomoyo_path_number_acl { pub head: tomoyo_acl_info, pub perm: u8, pub name: tomoyo_name_union, pub number: tomoyo_number_union }
#[repr(C)] pub struct tomoyo_mkdev_acl { pub head: tomoyo_acl_info, pub perm: u8, pub name: tomoyo_name_union, pub mode: tomoyo_number_union, pub major: tomoyo_number_union, pub minor: tomoyo_number_union }
#[repr(C)] pub struct tomoyo_path2_acl { pub head: tomoyo_acl_info, pub perm: u8, pub name1: tomoyo_name_union, pub name2: tomoyo_name_union }
#[repr(C)] pub struct tomoyo_mount_acl { pub head: tomoyo_acl_info, pub dev_name: tomoyo_name_union, pub dir_name: tomoyo_name_union, pub fs_type: tomoyo_name_union, pub flags: tomoyo_number_union }
#[repr(C)] pub struct tomoyo_env_acl { pub head: tomoyo_acl_info, pub env: *const tomoyo_path_info }
#[repr(C)] pub struct tomoyo_inet_acl { pub head: tomoyo_acl_info, pub protocol: u8, pub perm: u8, pub address: tomoyo_ipaddr_union, pub port: tomoyo_number_union }
#[repr(C)] pub struct tomoyo_unix_acl { pub head: tomoyo_acl_info, pub protocol: u8, pub perm: u8, pub name: tomoyo_name_union }
#[repr(C)] pub struct tomoyo_transition_control { pub head: tomoyo_acl_head, pub type_: u8, pub is_last_name: bool, pub domainname: *const tomoyo_path_info, pub program: *const tomoyo_path_info }
#[repr(C)] pub struct tomoyo_aggregator { pub head: tomoyo_acl_head, pub original_name: *const tomoyo_path_info, pub aggregated_name: *const tomoyo_path_info }
#[repr(C)] pub struct tomoyo_manager { pub head: tomoyo_acl_head, pub manager: *const tomoyo_path_info }

/* Kernel-provided types and functions referenced above. */
extern "C" { pub fn strcmp(a: *const c_char, b: *const c_char) -> c_int; }

/* The remaining declarations retain the original C ABI and are intentionally
 * left as external dependencies. */
extern "C" {
    pub fn tomoyo_interface_init() -> c_int;
    pub fn tomoyo_address_matches_group(is_ipv6: bool, address: *const __be32, group: *const tomoyo_group) -> bool;
    pub fn tomoyo_compare_number_union(value: c_ulong, ptr: *const tomoyo_number_union) -> bool;
    pub fn tomoyo_correct_path(filename: *const c_char) -> bool;
    pub fn tomoyo_memory_ok(ptr: *mut c_void) -> bool;
    pub fn tomoyo_get_name(name: *const c_char) -> *const tomoyo_path_info;
    pub fn tomoyo_domain() -> *mut tomoyo_domain_info;
    pub fn tomoyo_update_stat(index: u8);
}

/* Direct equivalents of the simple inline helpers. */
#[inline] pub unsafe fn tomoyo_pathcmp(a: *const tomoyo_path_info, b: *const tomoyo_path_info) -> bool { (*a).hash != (*b).hash || strcmp((*a).name, (*b).name) != 0 }
#[inline] pub unsafe fn tomoyo_same_name_union(a: *const tomoyo_name_union, b: *const tomoyo_name_union) -> bool { (*a).filename == (*b).filename && (*a).group == (*b).group }
#[inline] pub unsafe fn tomoyo_same_number_union(a: *const tomoyo_number_union, b: *const tomoyo_number_union) -> bool { (*a).values == (*b).values && (*a).group == (*b).group && (*a).value_type == (*b).value_type }
#[inline] pub unsafe fn tomoyo_current_namespace() -> *mut tomoyo_policy_namespace { tomoyo_domain().as_ref().unwrap().ns }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
