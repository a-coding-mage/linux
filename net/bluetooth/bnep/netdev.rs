// SPDX-License-Identifier: GPL-2.0
/*
   BNEP implementation for Linux Bluetooth stack (BlueZ).
   Copyright (C) 2001-2002 Inventel Systemes
   Written 2001-2002 by
	Clément Moreau <clement.moreau@inventel.fr>
	David Libault  <david.libault@inventel.fr>

   Copyright (C) 2002 Maxim Krasnyansky <maxk@qualcomm.com>

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
   IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
   CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
   OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

   ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
   COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
   SOFTWARE IS DISCLAIMED.
*/

const BNEP_TX_QUEUE_LEN: usize = 20;

unsafe fn bnep_net_open(dev: *mut net_device) -> i32 {
	netif_start_queue(dev);
	0
}

unsafe fn bnep_net_close(dev: *mut net_device) -> i32 {
	netif_stop_queue(dev);
	0
}

unsafe fn bnep_net_set_mc_list(dev: *mut net_device) {
    // CONFIG_BT_BNEP_MC_FILTER controls this implementation block.
    #[cfg(CONFIG_BT_BNEP_MC_FILTER)]
    {
	let s = netdev_priv(dev) as *mut bnep_session;
	let sk = (*(*s).sock).sk;
	let mut size: i32;

	BT_DBG!("%s mc_count %d", (*dev).name, netdev_mc_count(dev));

	size = (core::mem::size_of::<bnep_set_filter_req>() +
		(BNEP_MAX_MULTICAST_FILTERS + 1) * ETH_ALEN * 2) as i32;
	let skb = alloc_skb(size, GFP_ATOMIC);
	if skb.is_null() {
		BT_ERR!("%s Multicast list allocation failed", (*dev).name);
		return;
	}

	let r = (*skb).data as *mut bnep_set_filter_req;
	__skb_put(skb, core::mem::size_of::<bnep_set_filter_req>());
	(*r).type_ = BNEP_CONTROL;
	(*r).ctrl = BNEP_FILTER_MULTI_ADDR_SET;

	if (*dev).flags & (IFF_PROMISC | IFF_ALLMULTI) != 0 {
		let start = [0x01u8; ETH_ALEN];
		__skb_put_data(skb, start.as_ptr(), ETH_ALEN);
		__skb_put_data(skb, (*dev).broadcast.as_ptr(), ETH_ALEN);
		(*r).len = htons((ETH_ALEN * 2) as u16);
	} else {
		let len = (*skb).len;
		if (*dev).flags & IFF_BROADCAST != 0 {
			__skb_put_data(skb, (*dev).broadcast.as_ptr(), ETH_ALEN);
			__skb_put_data(skb, (*dev).broadcast.as_ptr(), ETH_ALEN);
		}
		/* FIXME: We should group addresses here. */
		let mut i = 0;
		let mut ha = core::ptr::null_mut();
		netdev_for_each_mc_addr!(ha, dev, {
			if i == BNEP_MAX_MULTICAST_FILTERS { break; }
			__skb_put_data(skb, (*ha).addr.as_ptr(), ETH_ALEN);
			__skb_put_data(skb, (*ha).addr.as_ptr(), ETH_ALEN);
			i += 1;
		});
		(*r).len = htons(((*skb).len - len) as u16);
	}
	skb_queue_tail(&mut (*sk).sk_write_queue, skb);
	wake_up_interruptible(sk_sleep(sk));
    }
}

unsafe fn bnep_net_set_mac_addr(dev: *mut net_device, _arg: *mut core::ffi::c_void) -> i32 {
	BT_DBG!("%s", (*dev).name);
	0
}

unsafe fn bnep_net_timeout(dev: *mut net_device, _txqueue: u32) {
	BT_DBG!("net_timeout");
	netif_wake_queue(dev);
}

#[cfg(CONFIG_BT_BNEP_MC_FILTER)]
unsafe fn bnep_net_mc_filter(skb: *mut sk_buff, s: *mut bnep_session) -> i32 {
	let eh = (*skb).data as *mut ethhdr;
	if ((*eh).h_dest[0] & 1) != 0 &&
		!test_bit(bnep_mc_hash((*eh).h_dest.as_ptr()), &mut (*s).mc_filter as *mut _ as *mut ulong) {
		return 1;
	}
	0
}

#[cfg(CONFIG_BT_BNEP_PROTO_FILTER)]
unsafe fn bnep_net_eth_proto(skb: *mut sk_buff) -> u16 {
	let eh = (*skb).data as *mut ethhdr;
	let proto = ntohs((*eh).h_proto);
	if proto >= ETH_P_802_3_MIN { return proto; }
	if get_unaligned((*skb).data as *const u16) == htons(0xFFFF) { return ETH_P_802_3; }
	ETH_P_802_2
}

#[cfg(CONFIG_BT_BNEP_PROTO_FILTER)]
unsafe fn bnep_net_proto_filter(skb: *mut sk_buff, s: *mut bnep_session) -> i32 {
	let proto = bnep_net_eth_proto(skb);
	let f = (*s).proto_filter;
	let mut i = 0;
	while i < BNEP_MAX_PROTO_FILTERS && (*f.add(i)).end != 0 {
		if proto >= (*f.add(i)).start && proto <= (*f.add(i)).end { return 0; }
		i += 1;
	}
	BT_DBG!("BNEP: filtered skb %p, proto 0x%.4x", skb, proto);
	1
}

unsafe fn bnep_net_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
	let s = netdev_priv(dev) as *mut bnep_session;
	let sk = (*(*s).sock).sk;
	BT_DBG!("skb %p, dev %p", skb, dev);
	#[cfg(CONFIG_BT_BNEP_MC_FILTER)]
	if bnep_net_mc_filter(skb, s) != 0 { kfree_skb(skb); return NETDEV_TX_OK; }
	#[cfg(CONFIG_BT_BNEP_PROTO_FILTER)]
	if bnep_net_proto_filter(skb, s) != 0 { kfree_skb(skb); return NETDEV_TX_OK; }
	netif_trans_update(dev);
	skb_queue_tail(&mut (*sk).sk_write_queue, skb);
	wake_up_interruptible(sk_sleep(sk));
	if skb_queue_len(&(*sk).sk_write_queue) >= BNEP_TX_QUEUE_LEN { BT_DBG!("tx queue is full"); netif_stop_queue(dev); }
	NETDEV_TX_OK
}

static bnep_netdev_ops: net_device_ops = net_device_ops {
	.ndo_open: Some(bnep_net_open), .ndo_stop: Some(bnep_net_close),
	.ndo_start_xmit: Some(bnep_net_xmit), .ndo_validate_addr: Some(eth_validate_addr),
	.ndo_set_rx_mode: Some(bnep_net_set_mc_list), .ndo_set_mac_address: Some(bnep_net_set_mac_addr),
	.ndo_tx_timeout: Some(bnep_net_timeout),
};

unsafe fn bnep_net_setup(dev: *mut net_device) {
	eth_broadcast_addr((*dev).broadcast.as_mut_ptr());
	(*dev).addr_len = ETH_ALEN;
	ether_setup(dev);
	(*dev).min_mtu = 0;
	(*dev).max_mtu = ETH_MAX_MTU;
	(*dev).priv_flags &= !IFF_TX_SKB_SHARING;
	(*dev).netdev_ops = &bnep_netdev_ops;
	(*dev).watchdog_timeo = HZ * 2;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
