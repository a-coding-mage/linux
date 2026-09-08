/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Security-Enhanced Linux (SELinux) security module
 *
 *  This file contains the SELinux security data structures for kernel objects.
 *
 *  Author(s):  Stephen Smalley, <stephen.smalley.work@gmail.com>
 *		Chris Vance, <cvance@nai.com>
 *		Wayne Salamon, <wsalamon@nai.com>
 *		James Morris <jmorris@redhat.com>
 *
 *  Copyright (C) 2001,2002 Networks Associates Technology, Inc.
 *  Copyright (C) 2003 Red Hat, Inc., James Morris <jmorris@redhat.com>
 *  Copyright (C) 2016 Mellanox Technologies
 */

use core::ffi::c_void;

pub const TSEC_AVDC_DIR_SIZE: usize = 1 << 2;

#[repr(C)]
pub struct avdc_entry {
    pub isid: u32, /* inode SID */
    pub avd: av_decision, /* av decision */
}

#[repr(C)]
pub struct cred_security_struct {
    pub osid: u32, /* SID prior to last execve */
    pub sid: u32, /* current SID */
    pub exec_sid: u32, /* exec SID */
    pub create_sid: u32, /* fscreate SID */
    pub keycreate_sid: u32, /* keycreate SID */
    pub sockcreate_sid: u32, /* fscreate SID */
}

#[repr(C)]
pub struct task_security_struct_avdcache {
    pub sid: u32, /* current SID for cached entries */
    pub seqno: u32, /* AVC sequence number */
    pub dir_spot: core::ffi::c_uint, /* dir cache index to check first */
    pub dir: [avdc_entry; TSEC_AVDC_DIR_SIZE], /* dir entries */
    pub permissive_neveraudit: bool, /* permissive and neveraudit */
}

#[repr(C)]
pub struct task_security_struct {
    pub avdcache: task_security_struct_avdcache,
}

pub unsafe fn task_avdcache_permnoaudit(tsec: *mut task_security_struct, sid: u32) -> bool {
    unsafe {
        (*tsec).avdcache.permissive_neveraudit
            && sid == (*tsec).avdcache.sid
            && (*tsec).avdcache.seqno == avc_policy_seqno()
    }
}

#[repr(C)]
pub enum label_initialized {
    LABEL_INVALID, /* invalid or not initialized */
    LABEL_INITIALIZED, /* initialized */
    LABEL_PENDING,
}

#[repr(C)]
pub struct inode_security_struct {
    pub inode: *mut inode, /* back pointer to inode object */
    pub list: list_head, /* list of inode_security_struct */
    pub task_sid: u32, /* SID of creating task */
    pub sid: u32, /* SID of this object */
    pub sclass: u16, /* security class of this object */
    pub initialized: core::ffi::c_uchar, /* initialization flag */
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct file_security_struct {
    pub sid: u32, /* SID of open file description */
    pub fown_sid: u32, /* SID of file owner (for SIGIO) */
    pub isid: u32, /* SID of inode at the time of file open */
    pub pseqno: u32, /* Policy seqno at the time of file open */
}

#[repr(C)]
pub struct backing_file_security_struct {
    pub uf_sid: u32, /* associated user file fsec->sid */
}

#[repr(C)]
pub struct superblock_security_struct {
    pub sid: u32, /* SID of file system superblock */
    pub def_sid: u32, /* default SID for labeling */
    pub mntpoint_sid: u32, /* SECURITY_FS_USE_MNTPOINT context for files */
    pub creator_sid: u32, /* SID of privileged process */
    pub behavior: core::ffi::c_ushort, /* labeling behavior */
    pub flags: core::ffi::c_ushort, /* which mount options were specified */
    pub lock: mutex,
    pub isec_head: list_head,
    pub isec_lock: spinlock_t,
}

#[repr(C)]
pub struct msg_security_struct {
    pub sid: u32, /* SID of message */
}

#[repr(C)]
pub struct ipc_security_struct {
    pub sclass: u16, /* security class of this object */
    pub sid: u32, /* SID of IPC resource */
}

#[repr(C)]
pub struct netif_security_struct {
    pub ns: *const net, /* network namespace */
    pub ifindex: core::ffi::c_int, /* device index */
    pub sid: u32, /* SID for this interface */
}

#[repr(C)]
pub union netnode_security_struct_addr {
    pub ipv4: __be32, /* IPv4 node address */
    pub ipv6: in6_addr, /* IPv6 node address */
}

#[repr(C)]
pub struct netnode_security_struct {
    pub addr: netnode_security_struct_addr,
    pub sid: u32, /* SID for this node */
    pub family: u16, /* address family */
}

#[repr(C)]
pub struct netport_security_struct {
    pub sid: u32, /* SID for this node */
    pub port: u16, /* port number */
    pub protocol: u8, /* transport protocol */
}

/* CONFIG_NETLABEL conditional field definitions from C are preserved below. */
#[repr(C)]
pub enum sk_security_struct_nlbl_state {
    NLBL_UNSET = 0,
    NLBL_REQUIRE,
    NLBL_LABELED,
    NLBL_REQSKB,
    NLBL_CONNLABELED,
}

#[repr(C)]
pub enum sk_security_struct_sctp_assoc_state {
    SCTP_ASSOC_UNSET = 0,
    SCTP_ASSOC_SET,
}

#[repr(C)]
pub struct sk_security_struct {
    /*
     * Present only when CONFIG_NETLABEL is enabled in the C header:
     * nlbl_state: NetLabel state
     * nlbl_secattr: NetLabel sec attributes
     */
    #[cfg(CONFIG_NETLABEL)]
    pub nlbl_state: sk_security_struct_nlbl_state,
    #[cfg(CONFIG_NETLABEL)]
    pub nlbl_secattr: *mut netlbl_lsm_secattr,
    pub sid: u32, /* SID of this object */
    pub peer_sid: u32, /* SID of peer */
    pub sclass: u16, /* sock security class */
    pub sctp_assoc_state: sk_security_struct_sctp_assoc_state, /* SCTP association state */
}

#[repr(C)]
pub struct tun_security_struct {
    pub sid: u32, /* SID for the tun device sockets */
}

#[repr(C)]
pub struct key_security_struct {
    pub sid: u32, /* SID of key */
}

#[repr(C)]
pub struct ib_security_struct {
    pub sid: u32, /* SID of the queue pair or MAD agent */
}

#[repr(C)]
pub struct pkey_security_struct {
    pub subnet_prefix: u64, /* Port subnet prefix */
    pub pkey: u16, /* PKey number */
    pub sid: u32, /* SID of pkey */
}

#[repr(C)]
pub struct bpf_security_struct {
    pub sid: u32, /* SID of bpf obj creator */
    pub perms: u32, /* permissions for allowed bpf token commands */
    pub grantor_sid: u32, /* SID of token grantor */
}

#[repr(C)]
pub struct perf_event_security_struct {
    pub sid: u32, /* SID of perf_event obj creator */
}

extern "C" {
    pub static mut selinux_blob_sizes: lsm_blob_sizes;
    pub fn avc_policy_seqno() -> u32;
    pub fn current_cred() -> *const cred;
    pub fn backing_file_security(backing_file: *const file) -> *mut c_void;
}

pub unsafe fn selinux_cred(cred: *const cred) -> *mut cred_security_struct {
    unsafe {
        ((*cred).security as *mut u8).add(selinux_blob_sizes.lbs_cred as usize)
            as *mut cred_security_struct
    }
}

pub unsafe fn selinux_task(task: *const task_struct) -> *mut task_security_struct {
    unsafe {
        ((*task).security as *mut u8).add(selinux_blob_sizes.lbs_task as usize)
            as *mut task_security_struct
    }
}

pub unsafe fn selinux_file(file: *const file) -> *mut file_security_struct {
    unsafe {
        ((*file).f_security as *mut u8).add(selinux_blob_sizes.lbs_file as usize)
            as *mut file_security_struct
    }
}

pub unsafe fn selinux_backing_file(backing_file: *const file) -> *mut backing_file_security_struct {
    unsafe {
        let blob = backing_file_security(backing_file);
        (blob as *mut u8).add(selinux_blob_sizes.lbs_backing_file as usize)
            as *mut backing_file_security_struct
    }
}

pub unsafe fn selinux_inode(inode: *const inode) -> *mut inode_security_struct {
    unsafe {
        if unlikely((*inode).i_security.is_null()) {
            return core::ptr::null_mut();
        }
        ((*inode).i_security as *mut u8).add(selinux_blob_sizes.lbs_inode as usize)
            as *mut inode_security_struct
    }
}

pub unsafe fn selinux_msg_msg(msg_msg: *const msg_msg) -> *mut msg_security_struct {
    unsafe {
        ((*msg_msg).security as *mut u8).add(selinux_blob_sizes.lbs_msg_msg as usize)
            as *mut msg_security_struct
    }
}

pub unsafe fn selinux_ipc(ipc: *const kern_ipc_perm) -> *mut ipc_security_struct {
    unsafe {
        ((*ipc).security as *mut u8).add(selinux_blob_sizes.lbs_ipc as usize)
            as *mut ipc_security_struct
    }
}

/*
 * get the subjective security ID of the current task
 */
pub unsafe fn current_sid() -> u32 {
    unsafe {
        let crsec = selinux_cred(current_cred());

        (*crsec).sid
    }
}

pub unsafe fn selinux_superblock(superblock: *const super_block) -> *mut superblock_security_struct {
    unsafe {
        ((*superblock).s_security as *mut u8).add(selinux_blob_sizes.lbs_superblock as usize)
            as *mut superblock_security_struct
    }
}

/* CONFIG_KEYS conditional helper from C. */
#[cfg(CONFIG_KEYS)]
pub unsafe fn selinux_key(key: *const key) -> *mut key_security_struct {
    unsafe {
        ((*key).security as *mut u8).add(selinux_blob_sizes.lbs_key as usize)
            as *mut key_security_struct
    }
}

pub unsafe fn selinux_sock(sock: *const sock) -> *mut sk_security_struct {
    unsafe {
        ((*sock).sk_security as *mut u8).add(selinux_blob_sizes.lbs_sock as usize)
            as *mut sk_security_struct
    }
}

pub unsafe fn selinux_tun_dev(security: *mut c_void) -> *mut tun_security_struct {
    unsafe {
        (security as *mut u8).add(selinux_blob_sizes.lbs_tun_dev as usize)
            as *mut tun_security_struct
    }
}

pub unsafe fn selinux_ib(ib_sec: *mut c_void) -> *mut ib_security_struct {
    unsafe {
        (ib_sec as *mut u8).add(selinux_blob_sizes.lbs_ib as usize) as *mut ib_security_struct
    }
}

pub unsafe fn selinux_perf_event(perf_event: *mut c_void) -> *mut perf_event_security_struct {
    unsafe {
        (perf_event as *mut u8).add(selinux_blob_sizes.lbs_perf_event as usize)
            as *mut perf_event_security_struct
    }
}

/* CONFIG_BPF_SYSCALL conditional helpers from C. */
#[cfg(CONFIG_BPF_SYSCALL)]
pub unsafe fn selinux_bpf_map_security(map: *const bpf_map) -> *mut bpf_security_struct {
    unsafe {
        ((*map).security as *mut u8).add(selinux_blob_sizes.lbs_bpf_map as usize)
            as *mut bpf_security_struct
    }
}

#[cfg(CONFIG_BPF_SYSCALL)]
pub unsafe fn selinux_bpf_prog_security(prog: *const bpf_prog) -> *mut bpf_security_struct {
    unsafe {
        ((*(*prog).aux).security as *mut u8).add(selinux_blob_sizes.lbs_bpf_prog as usize)
            as *mut bpf_security_struct
    }
}

#[cfg(CONFIG_BPF_SYSCALL)]
pub unsafe fn selinux_bpf_token_security(token: *const bpf_token) -> *mut bpf_security_struct {
    unsafe {
        ((*token).security as *mut u8).add(selinux_blob_sizes.lbs_bpf_token as usize)
            as *mut bpf_security_struct
    }
}

/* CONFIG_BPF_SYSCALL conditional helpers from C. */
#[cfg(CONFIG_BPF_SYSCALL)]
pub unsafe fn selinux_bpf_map_security(map: *mut bpf_map) -> *mut bpf_security_struct {
    unsafe {
        ((*map).security as *mut u8).add(selinux_blob_sizes.lbs_bpf_map as usize)
            as *mut bpf_security_struct
    }
}

#[cfg(CONFIG_BPF_SYSCALL)]
pub unsafe fn selinux_bpf_prog_security(prog: *mut bpf_prog) -> *mut bpf_security_struct {
    unsafe {
        ((*(*prog).aux).security as *mut u8).add(selinux_blob_sizes.lbs_bpf_prog as usize)
            as *mut bpf_security_struct
    }
}

#[cfg(CONFIG_BPF_SYSCALL)]
pub unsafe fn selinux_bpf_token_security(token: *mut bpf_token) -> *mut bpf_security_struct {
    unsafe {
        ((*token).security as *mut u8).add(selinux_blob_sizes.lbs_bpf_token as usize)
            as *mut bpf_security_struct
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
