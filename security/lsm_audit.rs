// SPDX-License-Identifier: GPL-2.0-only
/*
 * common LSM auditing functions
 *
 * Based on code written for SELinux by :
 *			Stephen Smalley
 * 			James Morris <jmorris@redhat.com>
 * Author : Etienne Basset, <etienne.basset@ensta.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

type u8 = c_uchar;
type __be16 = u16;
type __be32 = u32;
type pid_t = c_int;

const EINVAL: c_int = 22;
const IPPROTO_TCP: u8 = 6;
const IPPROTO_UDP: u8 = 17;
const IPPROTO_SCTP: u8 = 132;
const IP_OFFSET: u16 = 0x1fff;
const AF_UNIX: c_int = 1;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const GFP_ATOMIC: c_uint = 0x20;
const __GFP_NOWARN: c_uint = 0x200;
const AUDIT_AVC: c_int = 1400;

#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qstr {
    pub name: *const c_char,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    pub d_lock: spinlock_t,
    pub d_name: qstr,
    pub d_inode: *mut inode,
}

#[repr(C)]
pub struct super_block {
    pub s_id: *const c_char,
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
    pub i_ino: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct path {
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct file {
    pub f_path: path,
}

#[repr(C)]
pub struct lsm_ioctlop_audit {
    pub path: path,
    pub cmd: u16,
}

#[repr(C)]
pub struct task_struct {
    pub comm: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct iphdr {
    pub _ihl_version: u8,
    pub _tos: u8,
    pub _tot_len: __be16,
    pub _id: __be16,
    pub frag_off: __be16,
    pub _ttl: u8,
    pub protocol: u8,
    pub _check: __be16,
    pub saddr: __be32,
    pub daddr: __be32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub _priority_version: u8,
    pub _flow_lbl: [u8; 3],
    pub _payload_len: __be16,
    pub nexthdr: u8,
    pub _hop_limit: u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct tcphdr {
    pub source: __be16,
    pub dest: __be16,
}

#[repr(C)]
pub struct udphdr {
    pub source: __be16,
    pub dest: __be16,
}

#[repr(C)]
pub struct sctphdr {
    pub source: __be16,
    pub dest: __be16,
}

#[repr(C)]
pub struct sock {
    pub sk_family: c_int,
    pub sk_v6_rcv_saddr: in6_addr,
    pub sk_v6_daddr: in6_addr,
}

#[repr(C)]
pub struct inet_sock {
    pub inet_rcv_saddr: __be32,
    pub inet_sport: __be16,
    pub inet_daddr: __be32,
    pub inet_dport: __be16,
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_path: [c_char; 108],
}

#[repr(C)]
pub struct unix_address {
    pub len: c_int,
    pub name: *mut sockaddr_un,
}

#[repr(C)]
pub struct unix_sock {
    pub addr: *mut unix_address,
    pub path: path,
}

#[repr(C)]
pub struct net_device {
    pub name: *const c_char,
}

#[repr(C)]
pub struct lsm_network_v4 {
    pub saddr: __be32,
    pub daddr: __be32,
}

#[repr(C)]
pub struct lsm_network_v6 {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct lsm_network_audit {
    pub sk: *const sock,
    pub family: c_int,
    pub sport: __be16,
    pub dport: __be16,
    pub v4info: lsm_network_v4,
    pub v6info: lsm_network_v6,
    pub netif: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_key_audit {
    pub key: c_uint,
    pub key_desc: *const c_char,
}

#[repr(C)]
pub struct lsm_ibpkey_audit {
    pub subnet_prefix: u64,
    pub pkey: c_uint,
}

#[repr(C)]
pub struct lsm_ibendport_audit {
    pub dev_name: *const c_char,
    pub port: c_uint,
}

#[repr(C)]
pub union common_audit_data_u {
    pub ipc_id: c_int,
    pub cap: c_int,
    pub path: path,
    pub file: *mut file,
    pub op: *mut lsm_ioctlop_audit,
    pub dentry: *mut dentry,
    pub inode: *mut inode,
    pub tsk: *mut task_struct,
    pub net: *mut lsm_network_audit,
    pub key_struct: lsm_key_audit,
    pub kmod_name: *const c_char,
    pub ibpkey: *mut lsm_ibpkey_audit,
    pub ibendport: *mut lsm_ibendport_audit,
    pub reason: usize,
    pub anonclass: *const c_char,
    pub nlmsg_type: u16,
}

#[repr(C)]
pub struct common_audit_data {
    pub type_: c_int,
    pub u: common_audit_data_u,
}

const LSM_AUDIT_DATA_NONE: c_int = 0;
const LSM_AUDIT_DATA_IPC: c_int = 1;
const LSM_AUDIT_DATA_CAP: c_int = 2;
const LSM_AUDIT_DATA_PATH: c_int = 3;
const LSM_AUDIT_DATA_FILE: c_int = 4;
const LSM_AUDIT_DATA_IOCTL_OP: c_int = 5;
const LSM_AUDIT_DATA_DENTRY: c_int = 6;
const LSM_AUDIT_DATA_INODE: c_int = 7;
const LSM_AUDIT_DATA_TASK: c_int = 8;
const LSM_AUDIT_DATA_NET: c_int = 9;
const LSM_AUDIT_DATA_KEY: c_int = 10;
const LSM_AUDIT_DATA_KMOD: c_int = 11;
const LSM_AUDIT_DATA_IBPKEY: c_int = 12;
const LSM_AUDIT_DATA_IBENDPORT: c_int = 13;
const LSM_AUDIT_DATA_LOCKDOWN: c_int = 14;
const LSM_AUDIT_DATA_ANONINODE: c_int = 15;
const LSM_AUDIT_DATA_NLMSGTYPE: c_int = 16;

unsafe extern "C" {
    static init_net: net;
    static current: *mut task_struct;
    static lockdown_reasons: *const *const c_char;

    fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr;
    fn tcp_hdr(skb: *mut sk_buff) -> *mut tcphdr;
    fn udp_hdr(skb: *mut sk_buff) -> *mut udphdr;
    fn sctp_hdr(skb: *mut sk_buff) -> *mut sctphdr;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn skb_network_offset(skb: *mut sk_buff) -> c_int;
    fn ipv6_skip_exthdr(
        skb: *mut sk_buff,
        start: c_int,
        nexthdrp: *mut u8,
        frag_offp: *mut __be16,
    ) -> c_int;
    fn skb_header_pointer(
        skb: *mut sk_buff,
        offset: c_int,
        len: c_int,
        buffer: *mut c_void,
    ) -> *mut c_void;
    fn ipv6_addr_any(addr: *const in6_addr) -> c_int;
    fn ntohs(netshort: __be16) -> u16;

    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_d_path(ab: *mut audit_buffer, prefix: *const c_char, path: *const path);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, string: *const c_char);
    fn audit_log_n_hex(ab: *mut audit_buffer, buf: *const c_void, len: c_int);
    fn audit_log_start(ctx: *mut c_void, gfp_mask: c_uint, type_: c_int) -> *mut audit_buffer;
    fn audit_context() -> *mut c_void;
    fn audit_log_end(ab: *mut audit_buffer);

    fn d_backing_inode(dentry: *mut dentry) -> *mut inode;
    fn file_inode(file: *mut file) -> *mut inode;
    fn spin_lock(lock: *const spinlock_t);
    fn spin_unlock(lock: *const spinlock_t);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn d_find_alias_rcu(inode: *mut inode) -> *mut dentry;
    fn task_tgid_nr(tsk: *mut task_struct) -> pid_t;
    fn get_task_comm(buf: *mut c_char, tsk: *mut task_struct) -> *mut c_char;
    fn inet_sk(sk: *const sock) -> *const inet_sock;
    fn unix_sk(sk: *const sock) -> *const unix_sock;
    fn smp_load_acquire(p: *const *mut unix_address) -> *mut unix_address;
    fn dev_get_by_index(net: *const net, ifindex: c_int) -> *mut net_device;
    fn dev_put(dev: *mut net_device);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/**
 * ipv4_skb_to_auditdata : fill auditdata from skb
 * @skb : the skb
 * @ad : the audit data to fill
 * @proto : the layer 4 protocol
 *
 * return  0 on success
 */
#[no_mangle]
pub unsafe extern "C" fn ipv4_skb_to_auditdata(
    skb: *mut sk_buff,
    ad: *mut common_audit_data,
    proto: *mut u8,
) -> c_int {
    let mut ret: c_int = 0;
    let ih: *mut iphdr;

    ih = ip_hdr(skb);
    (*(*ad).u.net).v4info.saddr = (*ih).saddr;
    (*(*ad).u.net).v4info.daddr = (*ih).daddr;

    if !proto.is_null() {
        *proto = (*ih).protocol;
    }
    /* non initial fragment */
    if (ntohs((*ih).frag_off) & IP_OFFSET) != 0 {
        return 0;
    }

    match (*ih).protocol {
        IPPROTO_TCP => {
            let th: *mut tcphdr = tcp_hdr(skb);

            (*(*ad).u.net).sport = (*th).source;
            (*(*ad).u.net).dport = (*th).dest;
        }
        IPPROTO_UDP => {
            let uh: *mut udphdr = udp_hdr(skb);

            (*(*ad).u.net).sport = (*uh).source;
            (*(*ad).u.net).dport = (*uh).dest;
        }
        IPPROTO_SCTP => {
            let sh: *mut sctphdr = sctp_hdr(skb);

            (*(*ad).u.net).sport = (*sh).source;
            (*(*ad).u.net).dport = (*sh).dest;
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

// Translated from: #if IS_ENABLED(CONFIG_IPV6)
/**
 * ipv6_skb_to_auditdata : fill auditdata from skb
 * @skb : the skb
 * @ad : the audit data to fill
 * @proto : the layer 4 protocol
 *
 * return  0 on success
 */
#[no_mangle]
pub unsafe extern "C" fn ipv6_skb_to_auditdata(
    skb: *mut sk_buff,
    ad: *mut common_audit_data,
    proto: *mut u8,
) -> c_int {
    let mut ret: c_int = 0;
    let ip6: *mut ipv6hdr;
    let mut nexthdr: u8;
    let mut frag_off: __be16 = 0;

    ip6 = ipv6_hdr(skb);
    (*(*ad).u.net).v6info.saddr = (*ip6).saddr;
    (*(*ad).u.net).v6info.daddr = (*ip6).daddr;
    /* IPv6 can have several extension header before the Transport header
     * skip them */
    let mut offset: c_int = skb_network_offset(skb);
    offset += size_of::<ipv6hdr>() as c_int;
    nexthdr = (*ip6).nexthdr;
    offset = ipv6_skip_exthdr(skb, offset, &mut nexthdr, &mut frag_off);
    if offset < 0 {
        return 0;
    }
    if !proto.is_null() {
        *proto = nexthdr;
    }
    match nexthdr {
        IPPROTO_TCP => {
            let mut _tcph: tcphdr = core::mem::zeroed();
            let th: *mut tcphdr = skb_header_pointer(
                skb,
                offset,
                size_of::<tcphdr>() as c_int,
                &mut _tcph as *mut _ as *mut c_void,
            ) as *mut tcphdr;
            if th.is_null() {
            } else {
                (*(*ad).u.net).sport = (*th).source;
                (*(*ad).u.net).dport = (*th).dest;
            }
        }
        IPPROTO_UDP => {
            let mut _udph: udphdr = core::mem::zeroed();
            let uh: *mut udphdr = skb_header_pointer(
                skb,
                offset,
                size_of::<udphdr>() as c_int,
                &mut _udph as *mut _ as *mut c_void,
            ) as *mut udphdr;
            if uh.is_null() {
            } else {
                (*(*ad).u.net).sport = (*uh).source;
                (*(*ad).u.net).dport = (*uh).dest;
            }
        }
        IPPROTO_SCTP => {
            let mut _sctph: sctphdr = core::mem::zeroed();
            let sh: *mut sctphdr = skb_header_pointer(
                skb,
                offset,
                size_of::<sctphdr>() as c_int,
                &mut _sctph as *mut _ as *mut c_void,
            ) as *mut sctphdr;
            if sh.is_null() {
            } else {
                (*(*ad).u.net).sport = (*sh).source;
                (*(*ad).u.net).dport = (*sh).dest;
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

unsafe fn print_ipv6_addr(
    ab: *mut audit_buffer,
    addr: *const in6_addr,
    port: __be16,
    name1: *const c_char,
    name2: *const c_char,
) {
    if ipv6_addr_any(addr) == 0 {
        audit_log_format(ab, c" %s=%pI6c".as_ptr(), name1, addr);
    }
    if port != 0 {
        audit_log_format(ab, c" %s=%d".as_ptr(), name2, ntohs(port) as c_int);
    }
}

unsafe fn print_ipv4_addr(
    ab: *mut audit_buffer,
    addr: __be32,
    port: __be16,
    name1: *const c_char,
    name2: *const c_char,
) {
    if addr != 0 {
        audit_log_format(ab, c" %s=%pI4".as_ptr(), name1, &addr as *const __be32);
    }
    if port != 0 {
        audit_log_format(ab, c" %s=%d".as_ptr(), name2, ntohs(port) as c_int);
    }
}

/**
 * audit_log_lsm_data - helper to log common LSM audit data
 * @ab : the audit buffer
 * @a : common audit data
 */
#[no_mangle]
pub unsafe extern "C" fn audit_log_lsm_data(ab: *mut audit_buffer, a: *const common_audit_data) {
    /*
     * To keep stack sizes in check force programmers to notice if they
     * start making this union too large!  See struct lsm_network_audit
     * as an example of how to deal with large data.
     */
    const _: [(); 1] = [(); (size_of::<common_audit_data_u>() <= size_of::<*mut c_void>() * 2) as usize];

    match (*a).type_ {
        LSM_AUDIT_DATA_NONE => {
            return;
        }
        LSM_AUDIT_DATA_IPC => {
            audit_log_format(ab, c" ipc_key=%d ".as_ptr(), (*a).u.ipc_id);
        }
        LSM_AUDIT_DATA_CAP => {
            audit_log_format(ab, c" capability=%d ".as_ptr(), (*a).u.cap);
        }
        LSM_AUDIT_DATA_PATH => {
            let inode: *mut inode;

            audit_log_d_path(ab, c" path=".as_ptr(), &(*a).u.path);

            inode = d_backing_inode((*a).u.path.dentry);
            if !inode.is_null() {
                audit_log_format(ab, c" dev=".as_ptr());
                audit_log_untrustedstring(ab, (*(*inode).i_sb).s_id);
                audit_log_format(ab, c" ino=%llu".as_ptr(), (*inode).i_ino);
            }
        }
        LSM_AUDIT_DATA_FILE => {
            let inode: *mut inode;

            audit_log_d_path(ab, c" path=".as_ptr(), &(*(*a).u.file).f_path);

            inode = file_inode((*a).u.file);
            if !inode.is_null() {
                audit_log_format(ab, c" dev=".as_ptr());
                audit_log_untrustedstring(ab, (*(*inode).i_sb).s_id);
                audit_log_format(ab, c" ino=%llu".as_ptr(), (*inode).i_ino);
            }
        }
        LSM_AUDIT_DATA_IOCTL_OP => {
            let inode: *mut inode;

            audit_log_d_path(ab, c" path=".as_ptr(), &(*(*a).u.op).path);

            inode = (*(*(*a).u.op).path.dentry).d_inode;
            if !inode.is_null() {
                audit_log_format(ab, c" dev=".as_ptr());
                audit_log_untrustedstring(ab, (*(*inode).i_sb).s_id);
                audit_log_format(ab, c" ino=%llu".as_ptr(), (*inode).i_ino);
            }

            audit_log_format(ab, c" ioctlcmd=0x%hx".as_ptr(), (*(*a).u.op).cmd as c_int);
        }
        LSM_AUDIT_DATA_DENTRY => {
            let inode: *mut inode;

            audit_log_format(ab, c" name=".as_ptr());
            spin_lock(&(*(*a).u.dentry).d_lock);
            audit_log_untrustedstring(ab, (*(*a).u.dentry).d_name.name);
            spin_unlock(&(*(*a).u.dentry).d_lock);

            inode = d_backing_inode((*a).u.dentry);
            if !inode.is_null() {
                audit_log_format(ab, c" dev=".as_ptr());
                audit_log_untrustedstring(ab, (*(*inode).i_sb).s_id);
                audit_log_format(ab, c" ino=%llu".as_ptr(), (*inode).i_ino);
            }
        }
        LSM_AUDIT_DATA_INODE => {
            let dentry: *mut dentry;
            let inode: *mut inode;

            rcu_read_lock();
            inode = (*a).u.inode;
            dentry = d_find_alias_rcu(inode);
            if !dentry.is_null() {
                audit_log_format(ab, c" name=".as_ptr());
                spin_lock(&(*dentry).d_lock);
                audit_log_untrustedstring(ab, (*dentry).d_name.name);
                spin_unlock(&(*dentry).d_lock);
            }
            audit_log_format(ab, c" dev=".as_ptr());
            audit_log_untrustedstring(ab, (*(*inode).i_sb).s_id);
            audit_log_format(ab, c" ino=%llu".as_ptr(), (*inode).i_ino);
            rcu_read_unlock();
        }
        LSM_AUDIT_DATA_TASK => {
            let tsk: *mut task_struct = (*a).u.tsk;
            if !tsk.is_null() {
                let pid: pid_t = task_tgid_nr(tsk);
                if pid != 0 {
                    let mut tskcomm: [c_char; size_of::<[c_char; 16]>()] = [0; size_of::<[c_char; 16]>()];
                    audit_log_format(ab, c" opid=%d ocomm=".as_ptr(), pid);
                    audit_log_untrustedstring(ab, get_task_comm(tskcomm.as_mut_ptr(), tsk));
                }
            }
        }
        LSM_AUDIT_DATA_NET => {
            if !(*(*a).u.net).sk.is_null() {
                let sk: *const sock = (*(*a).u.net).sk;
                let mut u: *const unix_sock;
                let mut addr: *mut unix_address;
                let mut len: c_int = 0;
                let mut p: *mut c_char = ptr::null_mut();

                match (*sk).sk_family {
                    AF_INET => {
                        let inet: *const inet_sock = inet_sk(sk);

                        print_ipv4_addr(
                            ab,
                            (*inet).inet_rcv_saddr,
                            (*inet).inet_sport,
                            c"laddr".as_ptr(),
                            c"lport".as_ptr(),
                        );
                        print_ipv4_addr(
                            ab,
                            (*inet).inet_daddr,
                            (*inet).inet_dport,
                            c"faddr".as_ptr(),
                            c"fport".as_ptr(),
                        );
                    }
                    // Translated from: #if IS_ENABLED(CONFIG_IPV6)
                    AF_INET6 => {
                        let inet: *const inet_sock = inet_sk(sk);

                        print_ipv6_addr(
                            ab,
                            &(*sk).sk_v6_rcv_saddr,
                            (*inet).inet_sport,
                            c"laddr".as_ptr(),
                            c"lport".as_ptr(),
                        );
                        print_ipv6_addr(
                            ab,
                            &(*sk).sk_v6_daddr,
                            (*inet).inet_dport,
                            c"faddr".as_ptr(),
                            c"fport".as_ptr(),
                        );
                    }
                    AF_UNIX => {
                        u = unix_sk(sk);
                        addr = smp_load_acquire(&(*u).addr);
                        if addr.is_null() {
                        } else if !(*u).path.dentry.is_null() {
                            audit_log_d_path(ab, c" path=".as_ptr(), &(*u).path);
                        } else {
                            len = (*addr).len - size_of::<i16>() as c_int;
                            p = &mut (*(*addr).name).sun_path[0];
                            audit_log_format(ab, c" path=".as_ptr());
                            if *p != 0 {
                                audit_log_untrustedstring(ab, p);
                            } else {
                                audit_log_n_hex(ab, p as *const c_void, len);
                            }
                        }
                    }
                    _ => {}
                }
            }

            match (*(*a).u.net).family {
                AF_INET => {
                    print_ipv4_addr(
                        ab,
                        (*(*a).u.net).v4info.saddr,
                        (*(*a).u.net).sport,
                        c"saddr".as_ptr(),
                        c"src".as_ptr(),
                    );
                    print_ipv4_addr(
                        ab,
                        (*(*a).u.net).v4info.daddr,
                        (*(*a).u.net).dport,
                        c"daddr".as_ptr(),
                        c"dest".as_ptr(),
                    );
                }
                AF_INET6 => {
                    print_ipv6_addr(
                        ab,
                        &(*(*a).u.net).v6info.saddr,
                        (*(*a).u.net).sport,
                        c"saddr".as_ptr(),
                        c"src".as_ptr(),
                    );
                    print_ipv6_addr(
                        ab,
                        &(*(*a).u.net).v6info.daddr,
                        (*(*a).u.net).dport,
                        c"daddr".as_ptr(),
                        c"dest".as_ptr(),
                    );
                }
                _ => {}
            }
            if (*(*a).u.net).netif > 0 {
                let dev: *mut net_device;

                /* NOTE: we always use init's namespace */
                dev = dev_get_by_index(&init_net, (*(*a).u.net).netif);
                if !dev.is_null() {
                    audit_log_format(ab, c" netif=%s".as_ptr(), (*dev).name);
                    dev_put(dev);
                }
            }
        }
        // Translated from: #ifdef CONFIG_KEYS
        LSM_AUDIT_DATA_KEY => {
            audit_log_format(ab, c" key_serial=%u".as_ptr(), (*a).u.key_struct.key);
            if !(*a).u.key_struct.key_desc.is_null() {
                audit_log_format(ab, c" key_desc=".as_ptr());
                audit_log_untrustedstring(ab, (*a).u.key_struct.key_desc);
            }
        }
        LSM_AUDIT_DATA_KMOD => {
            audit_log_format(ab, c" kmod=".as_ptr());
            audit_log_untrustedstring(ab, (*a).u.kmod_name);
        }
        LSM_AUDIT_DATA_IBPKEY => {
            let mut sbn_pfx: in6_addr = core::mem::zeroed();

            memset(
                &mut sbn_pfx.s6_addr as *mut _ as *mut c_void,
                0,
                size_of_val(&sbn_pfx.s6_addr),
            );
            memcpy(
                &mut sbn_pfx.s6_addr as *mut _ as *mut c_void,
                &(*(*a).u.ibpkey).subnet_prefix as *const _ as *const c_void,
                size_of_val(&(*(*a).u.ibpkey).subnet_prefix),
            );
            audit_log_format(
                ab,
                c" pkey=0x%x subnet_prefix=%pI6c".as_ptr(),
                (*(*a).u.ibpkey).pkey,
                &sbn_pfx as *const in6_addr,
            );
        }
        LSM_AUDIT_DATA_IBENDPORT => {
            audit_log_format(
                ab,
                c" device=%s port_num=%u".as_ptr(),
                (*(*a).u.ibendport).dev_name,
                (*(*a).u.ibendport).port,
            );
        }
        LSM_AUDIT_DATA_LOCKDOWN => {
            audit_log_format(
                ab,
                c" lockdown_reason=\"%s\"".as_ptr(),
                *lockdown_reasons.add((*a).u.reason),
            );
        }
        LSM_AUDIT_DATA_ANONINODE => {
            audit_log_format(ab, c" anonclass=%s".as_ptr(), (*a).u.anonclass);
        }
        LSM_AUDIT_DATA_NLMSGTYPE => {
            audit_log_format(ab, c" nl-msgtype=%hu".as_ptr(), (*a).u.nlmsg_type as c_int);
        }
        _ => {}
    } /* switch (a->type) */
}

/**
 * dump_common_audit_data - helper to dump common audit data
 * @ab : the audit buffer
 * @a : common audit data
 */
unsafe fn dump_common_audit_data(ab: *mut audit_buffer, a: *const common_audit_data) {
    let mut comm: [c_char; size_of::<[c_char; 16]>()] = [0; size_of::<[c_char; 16]>()];

    audit_log_format(ab, c" pid=%d comm=".as_ptr(), task_tgid_nr(current));
    audit_log_untrustedstring(ab, get_task_comm(comm.as_mut_ptr(), current));
    audit_log_lsm_data(ab, a);
}

/**
 * common_lsm_audit - generic LSM auditing function
 * @a:  auxiliary audit data
 * @pre_audit: lsm-specific pre-audit callback
 * @post_audit: lsm-specific post-audit callback
 *
 * setup the audit buffer for common security information
 * uses callback to print LSM specific information
 */
#[no_mangle]
pub unsafe extern "C" fn common_lsm_audit(
    a: *mut common_audit_data,
    pre_audit: Option<unsafe extern "C" fn(*mut audit_buffer, *mut c_void)>,
    post_audit: Option<unsafe extern "C" fn(*mut audit_buffer, *mut c_void)>,
) {
    let ab: *mut audit_buffer;

    if a.is_null() {
        return;
    }
    /* we use GFP_ATOMIC so we won't sleep */
    ab = audit_log_start(audit_context(), GFP_ATOMIC | __GFP_NOWARN, AUDIT_AVC);

    if ab.is_null() {
        return;
    }

    if let Some(pre_audit) = pre_audit {
        pre_audit(ab, a as *mut c_void);
    }

    dump_common_audit_data(ab, a);

    if let Some(post_audit) = post_audit {
        post_audit(ab, a as *mut c_void);
    }

    audit_log_end(ab);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
