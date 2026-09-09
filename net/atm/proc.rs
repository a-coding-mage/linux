// SPDX-License-Identifier: GPL-2.0
/* net/atm/proc.c - ATM /proc interface */

// Kernel dependencies supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static mut vcc_hash: [hlist_head; VCC_HTABLE_SIZE];
    static mut vcc_sklist_lock: rwlock_t;
    static mut atm_devs: list_head;
    static mut atm_proc_root: *mut proc_dir_entry;
    static init_net: net;

    fn noop_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn seq_printf(seq: *mut seq_file, fmt: *const c_char, ...);
    fn seq_puts(seq: *mut seq_file, s: *const c_char);
    fn seq_putc(seq: *mut seq_file, c: c_int);
    fn read_lock(lock: *mut rwlock_t);
    fn read_unlock(lock: *mut rwlock_t);
    fn hlist_empty(head: *const hlist_head) -> bool;
    fn __sk_head(head: *mut hlist_head) -> *mut sock;
    fn sk_head(head: *mut hlist_head) -> *mut sock;
    fn sk_next(sk: *mut sock) -> *mut sock;
    fn file_inode(file: *mut file) -> *mut inode;
    fn pde_data(inode: *mut inode) -> *mut c_void;
    fn sk_atm(vcc: *mut atm_vcc) -> *mut sock;
    fn atm_sk(sk: *mut sock) -> *mut atm_vcc;
    fn sk_wmem_alloc_get(sk: *mut sock) -> c_int;
    fn sk_rmem_alloc_get(sk: *mut sock) -> c_int;
    fn refcount_read(rc: *const refcount_t) -> c_int;
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn get_zeroed_page(flags: c_ulong) -> c_ulong;
    fn free_page(addr: c_ulong);
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn kasprintf(flags: c_ulong, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn proc_create_data(name: *const c_char, mode: c_int, parent: *mut proc_dir_entry,
                        ops: *const proc_ops, data: *mut c_void) -> *mut proc_dir_entry;
    fn remove_proc_entry(name: *const c_char, parent: *mut proc_dir_entry);
    fn proc_net_mkdir(net: *const net, name: *const c_char,
                      parent: *mut proc_dir_entry) -> *mut proc_dir_entry;
    fn proc_create_seq(name: *const c_char, mode: c_int, parent: *mut proc_dir_entry,
                       ops: *const seq_operations) -> *mut proc_dir_entry;
    fn proc_create_seq_private(name: *const c_char, mode: c_int,
                               parent: *mut proc_dir_entry, ops: *const seq_operations,
                               state_size: usize, data: *mut c_void) -> *mut proc_dir_entry;
    fn remove_proc_subtree(name: *const c_char, parent: *mut proc_dir_entry);
}

const VCC_HTABLE_SIZE: usize = 0; // supplied by the kernel headers
const ESI_LEN: c_int = 0; // supplied by the kernel headers
const SEQ_START_TOKEN: *mut sock = 1 as *mut sock;
const PF_ATMPVC: usize = 0; // supplied by the kernel headers
const GFP_KERNEL: c_ulong = 0;
const AF_ATMPVC: c_int = 0;

#[repr(C)] struct file;
#[repr(C)] struct inode;
#[repr(C)] struct proc_dir_entry;
#[repr(C)] struct net;
#[repr(C)] struct hlist_head;
#[repr(C)] struct list_head;
#[repr(C)] struct rwlock_t;
#[repr(C)] struct atomic_t;
#[repr(C)] struct refcount_t;
#[repr(C)] struct seq_file { private: *mut c_void, file: *mut file }
#[repr(C)] struct sock { sk_family: c_int, sk_err: c_int, sk_sndbuf: c_int, sk_rcvbuf: c_int, sk_refcnt: refcount_t }
#[repr(C)] struct k_atm_aal_stats { tx: atomic_t, tx_err: atomic_t, rx: atomic_t, rx_err: atomic_t, rx_drop: atomic_t }
#[repr(C)] struct atm_dev_stats { aal0: k_atm_aal_stats, aal5: k_atm_aal_stats }
#[repr(C)] struct atm_dev_ops { proc_read: Option<unsafe extern "C" fn(*mut atm_dev, *mut loff_t, *mut c_char) -> c_int> }
#[repr(C)] struct atm_dev { number: c_int, type_: *const c_char, esi: [u8; 6], stats: atm_dev_stats, refcnt: refcount_t, dev_list: list_head, ops: *mut atm_dev_ops, proc_name: *mut c_char, proc_entry: *mut proc_dir_entry }
#[repr(C)] struct atm_vcc { dev: *mut atm_dev, vpi: c_int, vci: c_int, qos: qos, flags: c_ulong }
#[repr(C)] struct qos { aal: usize, rxtp: traffic_parameters, txtp: traffic_parameters }
#[repr(C)] struct traffic_parameters { min_pcr: c_int, traffic_class: usize }
#[repr(C)] struct proc_ops { proc_read: Option<unsafe extern "C" fn(*mut file, *mut c_char, usize, *mut loff_t) -> isize>, proc_lseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t> }
#[repr(C)] struct seq_operations { start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut c_void>, next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut loff_t) -> *mut c_void>, stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>, show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int> }
type loff_t = i64;

static ATM_DEV_PROC_OPS: proc_ops = proc_ops { proc_read: Some(proc_dev_atm_read), proc_lseek: Some(noop_llseek) };

unsafe extern "C" fn add_stats(seq: *mut seq_file, aal: *const c_char, stats: *const k_atm_aal_stats) {
    seq_printf(seq, b"%s ( %d %d %d %d %d )\0".as_ptr() as _, aal, atomic_read(&(*stats).tx), atomic_read(&(*stats).tx_err), atomic_read(&(*stats).rx), atomic_read(&(*stats).rx_err), atomic_read(&(*stats).rx_drop));
}

unsafe extern "C" fn atm_dev_info(seq: *mut seq_file, dev: *const atm_dev) {
    seq_printf(seq, b"%3d %-8s\0".as_ptr() as _, (*dev).number, (*dev).type_);
    for i in 0..ESI_LEN as usize { seq_printf(seq, b"%02x\0".as_ptr() as _, (*dev).esi[i]); }
    seq_puts(seq, b"  \0".as_ptr() as _); add_stats(seq, b"0\0".as_ptr() as _, &(*dev).stats.aal0); seq_puts(seq, b"  \0".as_ptr() as _); add_stats(seq, b"5\0".as_ptr() as _, &(*dev).stats.aal5); seq_printf(seq, b"\t[%d]\0".as_ptr() as _, refcount_read(&(*dev).refcnt)); seq_putc(seq, b'\n' as c_int);
}

#[repr(C)] struct vcc_state { bucket: c_int, sk: *mut sock }

unsafe extern "C" fn compare_family(sk: *mut sock, family: c_int) -> c_int { (!family == 0 || (*sk).sk_family == family) as c_int }

unsafe extern "C" fn __vcc_walk(sockp: *mut *mut sock, family: c_int, bucket: *mut c_int, mut l: loff_t) -> c_int {
    let mut sk = *sockp;
    if sk == SEQ_START_TOKEN { while *bucket < VCC_HTABLE_SIZE as c_int { let head = &mut vcc_hash[*bucket as usize]; sk = if hlist_empty(head) { core::ptr::null_mut() } else { __sk_head(head) }; if !sk.is_null() { break; } *bucket += 1; } l -= 1; }
    loop { while !sk.is_null() { l -= compare_family(sk, family) as i64; if l < 0 { *sockp = sk; return 1; } sk = sk_next(sk); } *bucket += 1; if *bucket >= VCC_HTABLE_SIZE as c_int { *sockp = SEQ_START_TOKEN; return 0; } sk = sk_head(&mut vcc_hash[*bucket as usize]); }
}

unsafe extern "C" fn vcc_walk(seq: *mut seq_file, l: loff_t) -> *mut c_void { let state = (*seq).private as *mut vcc_state; let family = pde_data(file_inode((*seq).file)) as usize as c_int; if __vcc_walk(&mut (*state).sk, family, &mut (*state).bucket, l) != 0 { state as *mut c_void } else { core::ptr::null_mut() } }
unsafe extern "C" fn vcc_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void { let state = (*seq).private as *mut vcc_state; read_lock(&mut vcc_sklist_lock); (*state).sk = SEQ_START_TOKEN; if *pos != 0 { vcc_walk(seq, *pos) } else { SEQ_START_TOKEN as *mut c_void } }
unsafe extern "C" fn vcc_seq_stop(_: *mut seq_file, _: *mut c_void) { read_unlock(&mut vcc_sklist_lock); }
unsafe extern "C" fn vcc_seq_next(seq: *mut seq_file, _: *mut c_void, pos: *mut loff_t) -> *mut c_void { let v = vcc_walk(seq, 1); *pos += 1; v }

unsafe extern "C" fn pvc_info(seq: *mut seq_file, vcc: *mut atm_vcc) { let class_name = [b"off\0",b"UBR\0",b"CBR\0",b"VBR\0",b"ABR\0"]; let aal_name = [b"---\0",b"1\0",b"2\0",b"3/4\0",b"???\0",b"5\0",b"???\0",b"???\0",b"???\0",b"???\0",b"???\0",b"???\0",b"???\0",b"0\0",b"???\0",b"???\0"]; seq_printf(seq, b"%3d %3d %5d %-3s %7d %-5s %7d %-6s\n\0".as_ptr() as _, (*(*vcc).dev).number, (*vcc).vpi, (*vcc).vci, if (*vcc).qos.aal >= aal_name.len() { b"err\0".as_ptr() } else { aal_name[(*vcc).qos.aal].as_ptr() }, (*vcc).qos.rxtp.min_pcr, class_name[(*vcc).qos.rxtp.traffic_class].as_ptr(), (*vcc).qos.txtp.min_pcr, class_name[(*vcc).qos.txtp.traffic_class].as_ptr()); }

unsafe extern "C" fn vcc_info(seq: *mut seq_file, vcc: *mut atm_vcc) { let sk = sk_atm(vcc); seq_printf(seq, b"%pK \0".as_ptr() as _, vcc); if (*vcc).dev.is_null() { seq_printf(seq, b"Unassigned    \0".as_ptr() as _); } else { seq_printf(seq, b"%3d %3d %5d \0".as_ptr() as _, (*(*vcc).dev).number, (*vcc).vpi, (*vcc).vci); } if (*sk).sk_family == AF_ATMPVC { seq_printf(seq, b"PVC\0".as_ptr() as _); } else { seq_printf(seq, b"%3d\0".as_ptr() as _, (*sk).sk_family); } seq_printf(seq, b" %04lx  %5d %7d/%7d %7d/%7d [%d]\n\0".as_ptr() as _, (*vcc).flags, (*sk).sk_err, sk_wmem_alloc_get(sk), (*sk).sk_sndbuf, sk_rmem_alloc_get(sk), (*sk).sk_rcvbuf, refcount_read(&(*sk).sk_refcnt)); }

unsafe extern "C" fn atm_dev_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int { if v == &mut atm_devs as *mut _ as *mut c_void { seq_puts(seq, b"Itf Type    ESI/\"MAC\"addr AAL(TX,err,RX,err,drop) ...               [refcnt]\n\0".as_ptr() as _); } else { atm_dev_info(seq, (v as *mut u8).offset(-(core::mem::offset_of!(atm_dev, dev_list) as isize)) as *mut atm_dev); } 0 }
unsafe extern "C" fn pvc_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int { if v == SEQ_START_TOKEN as *mut c_void { seq_puts(seq, b"Itf VPI VCI   AAL RX(PCR,Class) TX(PCR,Class)\n\0".as_ptr() as _); } else { let state = (*seq).private as *mut vcc_state; pvc_info(seq, atm_sk((*state).sk)); } 0 }
unsafe extern "C" fn vcc_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int { if v == SEQ_START_TOKEN as *mut c_void { seq_printf(seq, if core::mem::size_of::<*mut c_void>() == 4 { b"%-8s%s\0" } else { b"%-16s%s\0" }.as_ptr() as _, b"Address \0".as_ptr(), b"Itf VPI VCI   Fam Flags Reply Send buffer     Recv buffer      [refcnt]\n\0".as_ptr()); } else { let state = (*seq).private as *mut vcc_state; vcc_info(seq, atm_sk((*state).sk)); } 0 }

static ATM_DEV_SEQ_OPS: seq_operations = seq_operations { start: None, next: None, stop: None, show: Some(atm_dev_seq_show) };
static PVC_SEQ_OPS: seq_operations = seq_operations { start: Some(vcc_seq_start), next: Some(vcc_seq_next), stop: Some(vcc_seq_stop), show: Some(pvc_seq_show) };
static VCC_SEQ_OPS: seq_operations = seq_operations { start: Some(vcc_seq_start), next: Some(vcc_seq_next), stop: Some(vcc_seq_stop), show: Some(vcc_seq_show) };

unsafe extern "C" fn proc_dev_atm_read(file: *mut file, buf: *mut c_char, count: usize, pos: *mut loff_t) -> isize { if count == 0 { return 0; } let page = get_zeroed_page(GFP_KERNEL); if page == 0 { return -12; } let dev = pde_data(file_inode(file)) as *mut atm_dev; let mut length = if (*(*dev).ops).proc_read.is_none() { -22 } else { ((*(*dev).ops).proc_read.unwrap())(dev, pos, page as *mut c_char) }; if length as usize > count { length = -22; } if length >= 0 { if copy_to_user(buf as *mut c_void, page as *const c_void, length as usize) != 0 { length = -14; } *pos += 1; } free_page(page); length as isize }

#[no_mangle] pub unsafe extern "C" fn atm_proc_dev_register(dev: *mut atm_dev) -> c_int { if (*(*dev).ops).proc_read.is_none() { return 0; } (*dev).proc_name = kasprintf(GFP_KERNEL, b"%s:%d\0".as_ptr() as _, (*dev).type_, (*dev).number); if (*dev).proc_name.is_null() { return -12; } (*dev).proc_entry = proc_create_data((*dev).proc_name, 0, atm_proc_root, &ATM_DEV_PROC_OPS, dev as *mut c_void); if (*dev).proc_entry.is_null() { kfree((*dev).proc_name as *mut c_void); return -12; } 0 }
#[no_mangle] pub unsafe extern "C" fn atm_proc_dev_deregister(dev: *mut atm_dev) { if (*(*dev).ops).proc_read.is_none() { return; } remove_proc_entry((*dev).proc_name, atm_proc_root); kfree((*dev).proc_name as *mut c_void); }
#[no_mangle] pub unsafe extern "C" fn atm_proc_init() -> c_int { atm_proc_root = proc_net_mkdir(&init_net, b"atm\0".as_ptr() as _, init_net.proc_net); if atm_proc_root.is_null() { return -12; } proc_create_seq(b"devices\0".as_ptr() as _, 0o444, atm_proc_root, &ATM_DEV_SEQ_OPS); proc_create_seq_private(b"pvc\0".as_ptr() as _, 0o444, atm_proc_root, &PVC_SEQ_OPS, core::mem::size_of::<vcc_state>(), PF_ATMPVC as *mut c_void); proc_create_seq_private(b"vc\0".as_ptr() as _, 0o444, atm_proc_root, &VCC_SEQ_OPS, core::mem::size_of::<vcc_state>(), core::ptr::null_mut()); 0 }
#[no_mangle] pub unsafe extern "C" fn atm_proc_exit() { remove_proc_subtree(b"atm\0".as_ptr() as _, init_net.proc_net); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
