/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2023 Isovalent */

// Translated from net/tcx.h.  The CONFIG_* branches below preserve the
// corresponding build-time configuration conditions.

pub struct tcx_entry {
    pub miniq: *mut mini_Qdisc,
    pub bundle: bpf_mprog_bundle,
    pub miniq_active: u32,
    pub rcu: rcu_head,
}

pub struct tcx_link {
    pub link: bpf_link,
    pub dev: *mut net_device,
}

#[inline]
pub unsafe fn tcx_set_ingress(skb: *mut sk_buff, ingress: bool) {
    // CONFIG_NET_XGRESS: skb->tc_at_ingress = ingress;
    #[cfg(CONFIG_NET_XGRESS)]
    {
        (*skb).tc_at_ingress = ingress;
    }
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_entry(entry: *mut bpf_mprog_entry) -> *mut tcx_entry {
    let bundle = (*entry).parent;
    // C container_of(bundle, struct tcx_entry, bundle).
    (bundle as *mut u8).sub(core::mem::offset_of!(tcx_entry, bundle)) as *mut tcx_entry
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_link(link: *const bpf_link) -> *mut tcx_link {
    // C container_of(link, struct tcx_link, link).
    (link as *const u8).sub(core::mem::offset_of!(tcx_link, link)) as *mut tcx_link
}

#[cfg(CONFIG_NET_XGRESS)]
extern "C" {
    pub fn tcx_inc();
    pub fn tcx_dec();
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_entry_sync() {
    // bpf_mprog_entry got a/b swapped; ensure no inflight users remain on old one.
    synchronize_rcu();
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_entry_update(dev: *mut net_device, entry: *mut bpf_mprog_entry, ingress: bool) {
    ASSERT_RTNL!();
    if ingress {
        rcu_assign_pointer!((*dev).tcx_ingress, entry);
    } else {
        rcu_assign_pointer!((*dev).tcx_egress, entry);
    }
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_entry_fetch(dev: *mut net_device, ingress: bool) -> *mut bpf_mprog_entry {
    ASSERT_RTNL!();
    if ingress {
        rcu_dereference_rtnl!((*dev).tcx_ingress)
    } else {
        rcu_dereference_rtnl!((*dev).tcx_egress)
    }
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_entry_create_noprof() -> *mut bpf_mprog_entry {
    let tcx = kzalloc_noprof!(core::mem::size_of::<tcx_entry>(), GFP_KERNEL);
    if !tcx.is_null() {
        bpf_mprog_bundle_init!(&mut (*tcx).bundle);
        &mut (*tcx).bundle.a
    } else {
        core::ptr::null_mut()
    }
}

// #define tcx_entry_create(...) alloc_hooks(tcx_entry_create_noprof(__VA_ARGS__))

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_entry_free(entry: *mut bpf_mprog_entry) {
    kfree_rcu!(tcx_entry(entry), rcu);
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_entry_fetch_or_create(
    dev: *mut net_device,
    ingress: bool,
    created: *mut bool,
) -> *mut bpf_mprog_entry {
    let mut entry = tcx_entry_fetch(dev, ingress);
    *created = false;
    if entry.is_null() {
        entry = tcx_entry_create_noprof();
        if entry.is_null() {
            return core::ptr::null_mut();
        }
        *created = true;
    }
    entry
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_skeys_inc(ingress: bool) {
    tcx_inc();
    if ingress { net_inc_ingress_queue(); } else { net_inc_egress_queue(); }
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_skeys_dec(ingress: bool) {
    if ingress { net_dec_ingress_queue(); } else { net_dec_egress_queue(); }
    tcx_dec();
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_miniq_inc(entry: *mut bpf_mprog_entry) {
    ASSERT_RTNL!();
    (*tcx_entry(entry)).miniq_active += 1;
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_miniq_dec(entry: *mut bpf_mprog_entry) {
    ASSERT_RTNL!();
    (*tcx_entry(entry)).miniq_active -= 1;
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_entry_is_active(entry: *mut bpf_mprog_entry) -> bool {
    ASSERT_RTNL!();
    bpf_mprog_total(entry) != 0 || (*tcx_entry(entry)).miniq_active != 0
}

#[cfg(CONFIG_NET_XGRESS)]
#[inline]
pub unsafe fn tcx_action_code(skb: *mut sk_buff, code: i32) -> tcx_action_base {
    match code {
        TCX_PASS => {
            (*skb).tc_index = qdisc_skb_cb!(skb).tc_classid;
            code
        }
        TCX_DROP | TCX_REDIRECT => code,
        TCX_NEXT | _ => TCX_NEXT,
    }
}

#[cfg(all(CONFIG_NET_XGRESS, CONFIG_BPF_SYSCALL))]
extern "C" {
    pub fn tcx_prog_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
    pub fn tcx_link_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
    pub fn tcx_prog_detach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
    pub fn tcx_uninstall(dev: *mut net_device, ingress: bool);
    pub fn tcx_prog_query(attr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
}

#[cfg(all(CONFIG_NET_XGRESS, CONFIG_BPF_SYSCALL))]
#[inline]
pub unsafe fn dev_tcx_uninstall(dev: *mut net_device) {
    ASSERT_RTNL!();
    tcx_uninstall(dev, true);
    tcx_uninstall(dev, false);
}

#[cfg(not(all(CONFIG_NET_XGRESS, CONFIG_BPF_SYSCALL)))]
#[inline]
pub unsafe fn tcx_prog_attach(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 { -EINVAL }

#[cfg(not(all(CONFIG_NET_XGRESS, CONFIG_BPF_SYSCALL)))]
#[inline]
pub unsafe fn tcx_link_attach(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 { -EINVAL }

#[cfg(not(all(CONFIG_NET_XGRESS, CONFIG_BPF_SYSCALL)))]
#[inline]
pub unsafe fn tcx_prog_detach(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 { -EINVAL }

#[cfg(not(all(CONFIG_NET_XGRESS, CONFIG_BPF_SYSCALL)))]
#[inline]
pub unsafe fn tcx_prog_query(_attr: *const bpf_attr, _uattr: *mut bpf_attr) -> i32 { -EINVAL }

#[cfg(not(all(CONFIG_NET_XGRESS, CONFIG_BPF_SYSCALL)))]
#[inline]
pub unsafe fn dev_tcx_uninstall(_dev: *mut net_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
