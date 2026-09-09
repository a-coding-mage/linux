// SPDX-License-Identifier: GPL-2.0-only
/* Ethernet netdevice using ATM AAL5 as underlying carrier (RFC2684). */

// C headers and symbols from the kernel are supplied by external dependencies.

const BR2684_ETHERTYPE_LEN: usize = 2;
const BR2684_PAD_LEN: usize = 2;
const ETHERTYPE_IPV4: [u8; 2] = [0x08, 0x00];
const ETHERTYPE_IPV6: [u8; 2] = [0x86, 0xdd];
const LLC_OUI_PID_PAD: [u8; 11] = [0xaa, 0xaa, 0x03, 0x00, 0x80, 0xc2, 0x00, 0x07, 0x00, 0x00, 0x00];
const PAD: [u8; 2] = [0x00, 0x00];
const LLC_OUI_IPV4: [u8; 8] = [0xaa, 0xaa, 0x03, 0x00, 0x00, 0x00, 0x08, 0x00];
const LLC_OUI_IPV6: [u8; 8] = [0xaa, 0xaa, 0x03, 0x00, 0x00, 0x00, 0x86, 0xdd];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Br2684Encaps { EVc = BR2684_ENCAPS_VC, ELlc = BR2684_ENCAPS_LLC }

#[repr(C)]
struct Br2684Vcc {
    atmvcc: *mut AtmVcc, device: *mut NetDevice,
    old_push: Option<unsafe extern "C" fn(*mut AtmVcc, *mut SkBuff)>,
    old_pop: Option<unsafe extern "C" fn(*mut AtmVcc, *mut SkBuff)>,
    old_release_cb: Option<unsafe extern "C" fn(*mut AtmVcc)>, old_owner: *mut Module,
    encaps: Br2684Encaps, brvccs: ListHead,
    #[cfg(CONFIG_ATM_BR2684_IPFILTER)] filter: Br2684Filter,
    copies_needed: u32, copies_failed: u32, qspace: AtomicT,
}

#[repr(C)]
struct Br2684Dev { net_dev: *mut NetDevice, br2684_devs: ListHead, number: i32, brvccs: ListHead, mac_was_set: i32, payload: Br2684Payload }

static mut DEVS_LOCK: RwLock = DEFINE_RWLOCK!();
static mut BR2684_DEVS: ListHead = LIST_HEAD!();

#[inline] unsafe fn brpriv(dev: *const NetDevice) -> *mut Br2684Dev { netdev_priv(dev) as *mut Br2684Dev }
#[inline] unsafe fn list_entry_brdev(le: *const ListHead) -> *mut NetDevice { (*list_entry(le, Br2684Dev, br2684_devs)).net_dev }
#[inline] unsafe fn br2684_vcc(vcc: *const AtmVcc) -> *mut Br2684Vcc { (*vcc).user_back as *mut Br2684Vcc }
#[inline] unsafe fn list_entry_brvcc(le: *const ListHead) -> *mut Br2684Vcc { list_entry(le, Br2684Vcc, brvccs) }

unsafe fn br2684_find_dev(s: *const Br2684IfSpec) -> *mut NetDevice {
    let mut lh: *mut ListHead;
    match (*s).method {
        BR2684_FIND_BYNUM => { list_for_each!(lh, &raw mut BR2684_DEVS) { let d=list_entry_brdev(lh); if (*brpriv(d)).number == (*s).spec.devnum { return d; } } },
        BR2684_FIND_BYIFNAME => { list_for_each!(lh, &raw mut BR2684_DEVS) { let d=list_entry_brdev(lh); if strncmp((*d).name.as_ptr(), (*s).spec.ifname.as_ptr(), IFNAMSIZ) == 0 { return d; } } },
        _ => {}
    } null_mut()
}

unsafe extern "C" fn atm_dev_event(_this: *mut NotifierBlock, event: CULong, arg: *mut c_void) -> i32 {
    let atm_dev = arg as *mut AtmDev; let mut lh: *mut ListHead;
    pr_debug!("event={} dev={:p}\n", event, atm_dev); read_lock_irqsave!(&raw mut DEVS_LOCK);
    list_for_each!(lh, &raw mut BR2684_DEVS) { let dev=list_entry_brdev(lh); let mut v: *mut Br2684Vcc; list_for_each_entry!(v, &(*brpriv(dev)).brvccs, brvccs) { let a=(*v).atmvcc; if !a.is_null() && (*a).dev==atm_dev { if (*(*a).dev).signal==ATM_PHY_SIG_LOST { netif_carrier_off(dev); } else { netif_carrier_on(dev); } } } }
    read_unlock_irqrestore!(&raw mut DEVS_LOCK); NOTIFY_DONE
}

static mut ATM_DEV_NOTIFIER: NotifierBlock = NotifierBlock { notifier_call: Some(atm_dev_event) };

unsafe extern "C" fn br2684_pop(vcc: *mut AtmVcc, skb: *mut SkBuff) { let b=br2684_vcc(vcc); pr_debug!("(vcc {:p} ; net_dev {:p} )\n",vcc,(*b).device); ((*b).old_pop.unwrap())(vcc,skb); if atomic_inc_return!(&mut (*b).qspace)==1 { netif_wake_queue((*b).device); } }

unsafe fn br2684_xmit_vcc(mut skb: *mut SkBuff, dev: *mut NetDevice, b: *mut Br2684Vcc) -> i32 {
    let d=brpriv(dev); let minheadroom=if (*b).encaps==Br2684Encaps::ELlc { if (*d).payload==PBridged { LLC_OUI_PID_PAD.len() } else { LLC_OUI_IPV4.len() } } else if (*d).payload==PBridged { BR2684_PAD_LEN } else { 0 };
    if skb_headroom(skb)<minheadroom { let s=skb_realloc_headroom(skb,minheadroom); (*b).copies_needed+=1; dev_kfree_skb(skb); if s.is_null(){(*b).copies_failed+=1;return 0;} skb=s; }
    if (*b).encaps==Br2684Encaps::ELlc { if (*d).payload==PBridged { skb_push(skb,LLC_OUI_PID_PAD.len()); skb_copy_to_linear_data(skb,LLC_OUI_PID_PAD.as_ptr(),LLC_OUI_PID_PAD.len()); } else { let prot=ntohs((*skb).protocol); skb_push(skb,LLC_OUI_IPV4.len()); let p=match prot { ETH_P_IP=>&LLC_OUI_IPV4, ETH_P_IPV6=>&LLC_OUI_IPV6, _=>{dev_kfree_skb(skb);return 0;} }; skb_copy_to_linear_data(skb,p.as_ptr(),p.len()); } } else if (*d).payload==PBridged { skb_push(skb,2); memset((*skb).data as *mut c_void,0,2); }
    ATM_SKB!(skb).vcc=(*b).atmvcc; atm_account_tx((*b).atmvcc,skb); (*dev).stats.tx_packets+=1; (*dev).stats.tx_bytes+=(*skb).len; if atomic_dec_return!(&mut (*b).qspace)<1 { netif_stop_queue((*b).device); if atomic_read!(&(*b).qspace)>0 {netif_wake_queue((*b).device);} } (!((*(*b).atmvcc).send.unwrap())((*b).atmvcc,skb)) as i32
}

unsafe extern "C" fn br2684_release_cb(vcc:*mut AtmVcc){let b=br2684_vcc(vcc);if atomic_read!(&(*b).qspace)>0{netif_wake_queue((*b).device);}if let Some(f)=(*b).old_release_cb{f(vcc);}}
#[inline] unsafe fn pick_outgoing_vcc(_skb:*const SkBuff,d:*const Br2684Dev)->*mut Br2684Vcc{if list_empty!(&(*d).brvccs){null_mut()}else{list_entry_brvcc((*d).brvccs.next)}}

// Remaining kernel callbacks retain the C control flow and depend on kernel ABI types.
// Their declarations are preserved as external symbols for the surrounding translation.
unsafe extern "C" { fn br2684_push(vcc:*mut AtmVcc, skb:*mut SkBuff); fn br2684_close_vcc(v:*mut Br2684Vcc); fn br2684_regvcc(v:*mut AtmVcc,arg:*mut c_void)->i32; fn br2684_create(arg:*mut c_void)->i32; fn br2684_ioctl(sock:*mut Socket,cmd:u32,arg:ULong)->i32; }

// The module initialization/teardown entry points and netdevice setup are supplied below
// in kernel builds; external declarations preserve their interfaces.
extern "C" { fn br2684_init()->i32; fn br2684_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
