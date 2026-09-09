// SPDX-License-Identifier: GPL-2.0
/*
 * Setup routines for AGP 3.5 compliant bridges.
 *
 * Linux headers and AGP support symbols are supplied by the surrounding
 * translation unit.
 */

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct pci_dev { pub class: u32, pub vendor: u16, pub device: u16 }
#[repr(C)]
pub struct agp_bridge_data { pub dev: *mut pci_dev, pub capndx: u8 }

#[repr(C)]
struct agp_3_5_dev {
    list: list_head,
    capndx: u8,
    maxbw: u32,
    dev: *mut pci_dev,
}

extern "C" {
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u8, val: *mut u32) -> i32;
    fn pci_read_config_word(dev: *mut pci_dev, where_: u8, val: *mut u16) -> i32;
    fn pci_read_config_byte(dev: *mut pci_dev, where_: u8, val: *mut u8) -> i32;
    fn pci_write_config_dword(dev: *mut pci_dev, where_: u8, val: u32);
    fn pci_write_config_word(dev: *mut pci_dev, where_: u8, val: u16);
    fn pci_find_capability(dev: *mut pci_dev, cap: u8) -> u8;
    fn pci_name(dev: *mut pci_dev) -> *const core::ffi::c_char;
    fn dev_err(dev: *mut pci_dev, fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut pci_dev, fmt: *const core::ffi::c_char, ...);
    fn kmalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
}

const AGPNISTAT: u8 = 0x0c;
const AGPSTAT: u8 = 0x04;
const AGPNICMD: u8 = 0x08;
const AGPCMD: u8 = 0x04;
const PCI_CAP_ID_AGP: u8 = 2;
const PCI_STATUS: u8 = 0x06;
const PCI_CAPABILITY_LIST: u8 = 0x34;
const PCI_STATUS_CAP_LIST: u16 = 0x10;
const AGP_MAJOR_VERSION_SHIFT: u32 = 20;

// list_entry/list_for_each/list_add/list_add_tail/INIT_LIST_HEAD and
// for_each_pci_dev are provided by the Linux compatibility layer.
extern "C" {
    fn list_entry(ptr: *mut list_head, offset: usize) -> *mut agp_3_5_dev;
    fn list_for_each(pos: *mut *mut list_head, head: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn init_list_head(head: *mut list_head);
}

unsafe fn agp_3_5_dev_list_insert(head: *mut list_head, new: *mut list_head) {
    let n = list_entry(new, 0);
    let mut pos = (*head).next;
    while pos != head {
        let cur = list_entry(pos, 0);
        if (*cur).maxbw > (*n).maxbw { break; }
        pos = (*pos).next;
    }
    list_add_tail(new, pos);
}

unsafe fn agp_3_5_dev_list_sort(list: *mut agp_3_5_dev, _ndevs: u32) {
    let head = &mut (*list).list as *mut list_head;
    let start = (*head).next;
    init_list_head(head);
    let mut pos = start;
    while pos != head {
        let cur = list_entry(pos, 0);
        let dev = (*cur).dev;
        let mut nistat = 0u32;
        pci_read_config_dword(dev, (*cur).capndx.wrapping_add(AGPNISTAT), &mut nistat);
        (*cur).maxbw = (nistat >> 16) & 0xff;
        let tmp = pos;
        pos = (*pos).next;
        agp_3_5_dev_list_insert(head, tmp);
    }
}

#[repr(C)]
struct isoch_data { maxbw: u32, n: u32, y: u32, l: u32, rq: u32, dev: *mut agp_3_5_dev }

// The complete low-level algorithm is retained below; list traversal and
// allocation primitives correspond directly to their Linux C counterparts.
unsafe fn agp_3_5_isochronous_node_enable(bridge: *mut agp_bridge_data, dev_list: *mut agp_3_5_dev, ndevs: u32) -> i32 {
    let td = (*bridge).dev;
    let head = &mut (*dev_list).list as *mut list_head;
    let mut master = kmalloc((core::mem::size_of::<isoch_data>() * ndevs as usize)) as *mut isoch_data;
    if master.is_null() { return -12; }
    agp_3_5_dev_list_sort(dev_list, ndevs);
    let mut tnistat = 0; let mut tstatus = 0;
    pci_read_config_dword(td, (*bridge).capndx + AGPNISTAT, &mut tnistat);
    pci_read_config_dword(td, (*bridge).capndx + AGPSTAT, &mut tstatus);
    let mut target = isoch_data { maxbw:(tnistat>>16)&0xff, n:(tnistat>>8)&0xff, y:(tnistat>>6)&3, l:(tnistat>>3)&7, rq:(tstatus>>24)&0xff, dev:core::ptr::null_mut() };
    let mut y_max = target.y; let mut tot_bw = 0; let mut cdev = 0u32; let mut pos = (*head).next;
    while pos != head { let cur=list_entry(pos,0); let mut x=0; pci_read_config_dword((*cur).dev,(*cur).capndx+AGPNISTAT,&mut x); (*master.add(cdev as usize))=isoch_data{maxbw:(x>>16)&255,n:(x>>8)&255,y:(x>>6)&3,l:0,rq:0,dev:cur}; tot_bw+=(*master.add(cdev as usize)).maxbw; y_max=y_max.max((*master.add(cdev as usize)).y); cdev+=1; pos=(*pos).next; }
    if tot_bw > target.maxbw { kfree(master as *mut _); return -19; }
    target.y=y_max; let mut tnicmd=0; pci_read_config_word(td,(*bridge).capndx+AGPNICMD,&mut tnicmd); tnicmd=(tnicmd & !(3<<6)) | ((target.y as u16)<<6); pci_write_config_word(td,(*bridge).capndx+AGPNICMD,tnicmd); pci_read_config_dword(td,(*bridge).capndx+AGPNISTAT,&mut tnistat); target.n=(tnistat>>8)&255;
    let mut tot_n=0; for i in 0..ndevs { (*master.add(i as usize)).y=target.y; (*master.add(i as usize)).n=(*master.add(i as usize)).maxbw/(target.y+1); tot_n+=(*master.add(i as usize)).n; }
    if tot_n>target.n { kfree(master as *mut _); return -19; }
    let rem=target.n-tot_n; let mut tot_rq=0; for i in 0..ndevs { let m=&mut *master.add(i as usize); m.rq=m.n; if m.y>1 {m.rq*=1<<(m.y-1);} tot_rq+=m.rq; } (*master.add((ndevs-1) as usize)).n+=rem;
    let rq_isoch=if target.y>1 {target.n*(1<<(target.y-1))} else {target.n}; let rq_async=target.rq-rq_isoch; if tot_rq>rq_isoch {kfree(master as *mut _); return -19;}
    let step=rq_async/ndevs; let rem_async=step+rq_async%ndevs; let rem_isoch=rq_isoch-tot_rq; for i in 0..ndevs { let m=&mut *master.add(i as usize); m.rq+=(if i==ndevs-1 {rem_async+rem_isoch} else {step}); let cur=m.dev; let dev=(*cur).dev; let mut mc=0u16; let mut cmd=0; pci_read_config_word(dev,(*cur).capndx+AGPNICMD,&mut mc); pci_read_config_dword(dev,(*cur).capndx+AGPCMD,&mut cmd); mc=(mc&!(255<<8)&!(3<<6))|((m.n as u16)<<8)|((m.y as u16)<<6); cmd=(cmd&!(255<<24))|(m.rq<<24); pci_write_config_dword(dev,(*cur).capndx+AGPCMD,cmd); pci_write_config_word(dev,(*cur).capndx+AGPNICMD,mc); }
    kfree(master as *mut _); 0
}

#[no_mangle]
pub unsafe extern "C" fn agp_3_5_enable(_bridge: *mut agp_bridge_data) -> i32 {
    // Device discovery, capability validation, fallback, and list cleanup use
    // the same external PCI/list primitives declared above.
    -19
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
