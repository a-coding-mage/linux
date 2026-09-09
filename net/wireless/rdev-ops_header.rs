// Direct Rust translation of rdev-ops.h.
// External kernel types, operations, tracing hooks, and constants are supplied by dependent modules.

// SPDX-License-Identifier: GPL-2.0
//
// * Portions of this file
// * Copyright(c) 2016-2017 Intel Deutschland GmbH
// * Copyright (C) 2018, 2021-2026 Intel Corporation
// 

unsafe fn int rdev_suspend(*mut cfg80211_registered_devicerdev,
			       *mut cfg80211_wowlanwowlan)
{
	int ret;
	trace_rdev_suspend(&(*rdev).wiphy, wowlan);
	ret = (*rdev).(*ops).suspend(&(*rdev).wiphy, wowlan);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_resume(*mut cfg80211_registered_devicerdev)
{
	int ret;
	trace_rdev_resume(&(*rdev).wiphy);
	ret = (*rdev).(*ops).resume(&(*rdev).wiphy);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn () rdev_set_wakeup(*mut cfg80211_registered_devicerdev,
				   bool enabled)
{
	trace_rdev_set_wakeup(&(*rdev).wiphy, enabled);
	(*rdev).(*ops).set_wakeup(&(*rdev).wiphy, enabled);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn *mut wireless_devrdev_add_virtual_intf(*mut cfg80211_registered_devicerdev, *mut core::ffi::c_charname,
		       u8 name_assign_type,
		       enum nl80211_iftype type,
		       *mut vif_paramsparams)
{
	*mut wireless_devret;
	trace_rdev_add_virtual_intf(&(*rdev).wiphy, name, type);
	ret = (*rdev).(*ops).add_virtual_intf(&(*rdev).wiphy, name, name_assign_type,
					  type, params);
	trace_rdev_return_wdev(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_del_virtual_intf(*mut cfg80211_registered_devicerdev,
		      *mut wireless_devwdev)
{
	int ret;
	trace_rdev_del_virtual_intf(&(*rdev).wiphy, wdev);
	ret = (*rdev).(*ops).del_virtual_intf(&(*rdev).wiphy, wdev);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_change_virtual_intf(*mut cfg80211_registered_devicerdev,
			 *mut net_devicedev, enum nl80211_iftype type,
			 *mut vif_paramsparams)
{
	int ret;
	trace_rdev_change_virtual_intf(&(*rdev).wiphy, dev, type);
	ret = (*rdev).(*ops).change_virtual_intf(&(*rdev).wiphy, dev, type, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_add_key(*mut cfg80211_registered_devicerdev,
			       *mut wireless_devwdev, int link_id,
			       u8 key_index, bool pairwise, *const u8mac_addr,
			       *mut key_paramsparams)
{
	int ret;
	trace_rdev_add_key(&(*rdev).wiphy, wdev, link_id, key_index, pairwise,
			   mac_addr, (*params).mode);
	ret = (*rdev).(*ops).add_key(&(*rdev).wiphy, wdev, link_id, key_index,
				  pairwise, mac_addr, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_get_key(*mut cfg80211_registered_devicerdev, *mut wireless_devwdev,
	     int link_id, u8 key_index, bool pairwise, *const u8mac_addr,
	     () *cookie,
	     () (*callback)(() *cookie, *mut key_params))
{
	int ret;
	trace_rdev_get_key(&(*rdev).wiphy, wdev, link_id, key_index, pairwise,
			   mac_addr);
	ret = (*rdev).(*ops).get_key(&(*rdev).wiphy, wdev, link_id, key_index,
				  pairwise, mac_addr, cookie, callback);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_del_key(*mut cfg80211_registered_devicerdev,
			       *mut wireless_devwdev, int link_id,
			       u8 key_index, bool pairwise, *const u8mac_addr)
{
	int ret;
	trace_rdev_del_key(&(*rdev).wiphy, wdev, link_id, key_index, pairwise,
			   mac_addr);
	ret = (*rdev).(*ops).del_key(&(*rdev).wiphy, wdev, link_id, key_index,
				  pairwise, mac_addr);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_default_key(*mut cfg80211_registered_devicerdev,
		     *mut net_devicenetdev, int link_id, u8 key_index,
		     bool unicast, bool multicast)
{
	int ret;
	trace_rdev_set_default_key(&(*rdev).wiphy, netdev, link_id, key_index,
				   unicast, multicast);
	ret = (*rdev).(*ops).set_default_key(&(*rdev).wiphy, netdev, link_id,
					  key_index, unicast, multicast);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_default_mgmt_key(*mut cfg80211_registered_devicerdev,
			  *mut wireless_devwdev, int link_id, u8 key_index)
{
	int ret;
	trace_rdev_set_default_mgmt_key(&(*rdev).wiphy, wdev, link_id,
					key_index);
	ret = (*rdev).(*ops).set_default_mgmt_key(&(*rdev).wiphy, wdev, link_id,
					       key_index);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_default_beacon_key(*mut cfg80211_registered_devicerdev,
			    *mut wireless_devwdev, int link_id,
			    u8 key_index)
{
	int ret;

	trace_rdev_set_default_beacon_key(&(*rdev).wiphy, wdev, link_id,
					  key_index);
	ret = (*rdev).(*ops).set_default_beacon_key(&(*rdev).wiphy, wdev, link_id,
						 key_index);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_start_ap(*mut cfg80211_registered_devicerdev,
				*mut net_devicedev,
				*mut cfg80211_ap_settingssettings)
{
	int ret;
	trace_rdev_start_ap(&(*rdev).wiphy, dev, settings);
	ret = (*rdev).(*ops).start_ap(&(*rdev).wiphy, dev, settings);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_change_beacon(*mut cfg80211_registered_devicerdev,
				     *mut net_devicedev,
				     *mut cfg80211_ap_updateinfo)
{
	int ret;
	trace_rdev_change_beacon(&(*rdev).wiphy, dev, info);
	ret = (*rdev).(*ops).change_beacon(&(*rdev).wiphy, dev, info);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_stop_ap(*mut cfg80211_registered_devicerdev,
			       *mut net_devicedev, u32 link_id)
{
	int ret;
	trace_rdev_stop_ap(&(*rdev).wiphy, dev, link_id);
	ret = (*rdev).(*ops).stop_ap(&(*rdev).wiphy, dev, link_id);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_add_station(*mut cfg80211_registered_devicerdev,
				   *mut wireless_devwdev, *mut u8mac,
				   *mut station_parametersparams)
{
	int ret;
	trace_rdev_add_station(&(*rdev).wiphy, wdev, mac, params);
	ret = (*rdev).(*ops).add_station(&(*rdev).wiphy, wdev, mac, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_del_station(*mut cfg80211_registered_devicerdev,
				   *mut wireless_devwdev,
				   *mut station_del_parametersparams)
{
	int ret;
	trace_rdev_del_station(&(*rdev).wiphy, wdev, params);
	ret = (*rdev).(*ops).del_station(&(*rdev).wiphy, wdev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_change_station(*mut cfg80211_registered_devicerdev,
				      *mut wireless_devwdev, *mut u8mac,
				      *mut station_parametersparams)
{
	int ret;
	trace_rdev_change_station(&(*rdev).wiphy, wdev, mac, params);
	ret = (*rdev).(*ops).change_station(&(*rdev).wiphy, wdev, mac, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_get_station(*mut cfg80211_registered_devicerdev,
				   *mut wireless_devwdev, *const u8mac,
				   *mut station_infosinfo)
{
	int ret;
	trace_rdev_get_station(&(*rdev).wiphy, wdev, mac);
	ret = (*rdev).(*ops).get_station(&(*rdev).wiphy, wdev, mac, sinfo);
	trace_rdev_return_int_station_info(&(*rdev).wiphy, ret, sinfo);
	return ret;
}

unsafe fn int rdev_dump_station(*mut cfg80211_registered_devicerdev,
				    *mut wireless_devwdev, int idx, *mut u8mac,
				    *mut station_infosinfo)
{
	int ret;
	trace_rdev_dump_station(&(*rdev).wiphy, wdev, idx, mac);
	ret = (*rdev).(*ops).dump_station(&(*rdev).wiphy, wdev, idx, mac, sinfo);
	trace_rdev_return_int_station_info(&(*rdev).wiphy, ret, sinfo);
	return ret;
}

unsafe fn int rdev_add_mpath(*mut cfg80211_registered_devicerdev,
				 *mut net_devicedev, *mut u8dst, *mut u8next_hop)
{
	int ret;
	trace_rdev_add_mpath(&(*rdev).wiphy, dev, dst, next_hop);
	ret = (*rdev).(*ops).add_mpath(&(*rdev).wiphy, dev, dst, next_hop);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_del_mpath(*mut cfg80211_registered_devicerdev,
				 *mut net_devicedev, *mut u8dst)
{
	int ret;
	trace_rdev_del_mpath(&(*rdev).wiphy, dev, dst);
	ret = (*rdev).(*ops).del_mpath(&(*rdev).wiphy, dev, dst);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_change_mpath(*mut cfg80211_registered_devicerdev,
				    *mut net_devicedev, *mut u8dst,
				    *mut u8next_hop)
{
	int ret;
	trace_rdev_change_mpath(&(*rdev).wiphy, dev, dst, next_hop);
	ret = (*rdev).(*ops).change_mpath(&(*rdev).wiphy, dev, dst, next_hop);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_get_mpath(*mut cfg80211_registered_devicerdev,
				 *mut net_devicedev, *mut u8dst, *mut u8next_hop,
				 *mut mpath_infopinfo)
{
	int ret;
	trace_rdev_get_mpath(&(*rdev).wiphy, dev, dst, next_hop);
	ret = (*rdev).(*ops).get_mpath(&(*rdev).wiphy, dev, dst, next_hop, pinfo);
	trace_rdev_return_int_mpath_info(&(*rdev).wiphy, ret, pinfo);
	return ret;

}

unsafe fn int rdev_get_mpp(*mut cfg80211_registered_devicerdev,
			       *mut net_devicedev, *mut u8dst, *mut u8mpp,
			       *mut mpath_infopinfo)
{
	int ret;

	trace_rdev_get_mpp(&(*rdev).wiphy, dev, dst, mpp);
	ret = (*rdev).(*ops).get_mpp(&(*rdev).wiphy, dev, dst, mpp, pinfo);
	trace_rdev_return_int_mpath_info(&(*rdev).wiphy, ret, pinfo);
	return ret;
}

unsafe fn int rdev_dump_mpath(*mut cfg80211_registered_devicerdev,
				  *mut net_devicedev, int idx, *mut u8dst,
				  *mut u8next_hop, *mut mpath_infopinfo)

{
	int ret;
	trace_rdev_dump_mpath(&(*rdev).wiphy, dev, idx, dst, next_hop);
	ret = (*rdev).(*ops).dump_mpath(&(*rdev).wiphy, dev, idx, dst, next_hop,
				    pinfo);
	trace_rdev_return_int_mpath_info(&(*rdev).wiphy, ret, pinfo);
	return ret;
}

unsafe fn int rdev_dump_mpp(*mut cfg80211_registered_devicerdev,
				*mut net_devicedev, int idx, *mut u8dst,
				*mut u8mpp, *mut mpath_infopinfo)

{
	int ret;

	trace_rdev_dump_mpp(&(*rdev).wiphy, dev, idx, dst, mpp);
	ret = (*rdev).(*ops).dump_mpp(&(*rdev).wiphy, dev, idx, dst, mpp, pinfo);
	trace_rdev_return_int_mpath_info(&(*rdev).wiphy, ret, pinfo);
	return ret;
}

unsafe fn int
rdev_get_mesh_config(*mut cfg80211_registered_devicerdev,
		     *mut net_devicedev, *mut mesh_configconf)
{
	int ret;
	trace_rdev_get_mesh_config(&(*rdev).wiphy, dev);
	ret = (*rdev).(*ops).get_mesh_config(&(*rdev).wiphy, dev, conf);
	trace_rdev_return_int_mesh_config(&(*rdev).wiphy, ret, conf);
	return ret;
}

unsafe fn int
rdev_update_mesh_config(*mut cfg80211_registered_devicerdev,
			*mut net_devicedev, u32 mask,
			const *mut mesh_confignconf)
{
	int ret;
	trace_rdev_update_mesh_config(&(*rdev).wiphy, dev, mask, nconf);
	ret = (*rdev).(*ops).update_mesh_config(&(*rdev).wiphy, dev, mask, nconf);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_join_mesh(*mut cfg80211_registered_devicerdev,
				 *mut net_devicedev,
				 const *mut mesh_configconf,
				 const *mut mesh_setupsetup)
{
	int ret;
	trace_rdev_join_mesh(&(*rdev).wiphy, dev, conf, setup);
	ret = (*rdev).(*ops).join_mesh(&(*rdev).wiphy, dev, conf, setup);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}


unsafe fn int rdev_leave_mesh(*mut cfg80211_registered_devicerdev,
				  *mut net_devicedev)
{
	int ret;
	trace_rdev_leave_mesh(&(*rdev).wiphy, dev);
	ret = (*rdev).(*ops).leave_mesh(&(*rdev).wiphy, dev);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_join_ocb(*mut cfg80211_registered_devicerdev,
				*mut net_devicedev,
				*mut ocb_setupsetup)
{
	int ret;
	trace_rdev_join_ocb(&(*rdev).wiphy, dev, setup);
	ret = (*rdev).(*ops).join_ocb(&(*rdev).wiphy, dev, setup);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_leave_ocb(*mut cfg80211_registered_devicerdev,
				 *mut net_devicedev)
{
	int ret;
	trace_rdev_leave_ocb(&(*rdev).wiphy, dev);
	ret = (*rdev).(*ops).leave_ocb(&(*rdev).wiphy, dev);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_change_bss(*mut cfg80211_registered_devicerdev,
				  *mut net_devicedev,
				  *mut bss_parametersparams)

{
	int ret;
	trace_rdev_change_bss(&(*rdev).wiphy, dev, params);
	ret = (*rdev).(*ops).change_bss(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn () rdev_inform_bss(*mut cfg80211_registered_devicerdev,
				   *mut cfg80211_bssbss,
				   const *mut cfg80211_bss_iesies,
				   () *drv_data)

{
	trace_rdev_inform_bss(&(*rdev).wiphy, bss);
	if ((*rdev).(*ops).inform_bss)
		(*rdev).(*ops).inform_bss(&(*rdev).wiphy, bss, ies, drv_data);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int rdev_set_txq_params(*mut cfg80211_registered_devicerdev,
				      *mut net_devicedev,
				      *mut ieee80211_txq_paramsparams)

{
	int ret;
	trace_rdev_set_txq_params(&(*rdev).wiphy, dev, params);
	ret = (*rdev).(*ops).set_txq_params(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_libertas_set_mesh_channel(*mut cfg80211_registered_devicerdev,
			       *mut net_devicedev,
			       *mut ieee80211_channelchan)
{
	int ret;
	trace_rdev_libertas_set_mesh_channel(&(*rdev).wiphy, dev, chan);
	ret = (*rdev).(*ops).libertas_set_mesh_channel(&(*rdev).wiphy, dev, chan);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_monitor_channel(*mut cfg80211_registered_devicerdev,
			 *mut net_devicedev,
			 *mut cfg80211_chan_defchandef)
{
	int ret;
	trace_rdev_set_monitor_channel(&(*rdev).wiphy, dev, chandef);
	ret = (*rdev).(*ops).set_monitor_channel(&(*rdev).wiphy, dev, chandef);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_scan(*mut cfg80211_registered_devicerdev,
			    *mut cfg80211_scan_request_intrequest)
{
	int ret;

	if (WARN_ON_ONCE(!(*request).req.n_ssids && (*request).req.ssids))
		return -EINVAL;

	trace_rdev_scan(&(*rdev).wiphy, request);
	ret = (*rdev).(*ops).scan(&(*rdev).wiphy, &(*request).req);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn () rdev_abort_scan(*mut cfg80211_registered_devicerdev,
				   *mut wireless_devwdev)
{
	trace_rdev_abort_scan(&(*rdev).wiphy, wdev);
	(*rdev).(*ops).abort_scan(&(*rdev).wiphy, wdev);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int rdev_auth(*mut cfg80211_registered_devicerdev,
			    *mut net_devicedev,
			    *mut cfg80211_auth_requestreq)
{
	int ret;
	trace_rdev_auth(&(*rdev).wiphy, dev, req);
	ret = (*rdev).(*ops).auth(&(*rdev).wiphy, dev, req);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_assoc(*mut cfg80211_registered_devicerdev,
			     *mut net_devicedev,
			     *mut cfg80211_assoc_requestreq)
{
	int ret;

	trace_rdev_assoc(&(*rdev).wiphy, dev, req);
	ret = (*rdev).(*ops).assoc(&(*rdev).wiphy, dev, req);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_deauth(*mut cfg80211_registered_devicerdev,
			      *mut net_devicedev,
			      *mut cfg80211_deauth_requestreq)
{
	int ret;
	trace_rdev_deauth(&(*rdev).wiphy, dev, req);
	ret = (*rdev).(*ops).deauth(&(*rdev).wiphy, dev, req);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_disassoc(*mut cfg80211_registered_devicerdev,
				*mut net_devicedev,
				*mut cfg80211_disassoc_requestreq)
{
	int ret;
	trace_rdev_disassoc(&(*rdev).wiphy, dev, req);
	ret = (*rdev).(*ops).disassoc(&(*rdev).wiphy, dev, req);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_connect(*mut cfg80211_registered_devicerdev,
			       *mut net_devicedev,
			       *mut cfg80211_connect_paramssme)
{
	int ret;
	trace_rdev_connect(&(*rdev).wiphy, dev, sme);
	ret = (*rdev).(*ops).connect(&(*rdev).wiphy, dev, sme);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_update_connect_params(*mut cfg80211_registered_devicerdev,
			   *mut net_devicedev,
			   *mut cfg80211_connect_paramssme, u32 changed)
{
	int ret;
	trace_rdev_update_connect_params(&(*rdev).wiphy, dev, sme, changed);
	ret = (*rdev).(*ops).update_connect_params(&(*rdev).wiphy, dev, sme, changed);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_disconnect(*mut cfg80211_registered_devicerdev,
				  *mut net_devicedev, u16 reason_code)
{
	int ret;
	trace_rdev_disconnect(&(*rdev).wiphy, dev, reason_code);
	ret = (*rdev).(*ops).disconnect(&(*rdev).wiphy, dev, reason_code);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_join_ibss(*mut cfg80211_registered_devicerdev,
				 *mut net_devicedev,
				 *mut cfg80211_ibss_paramsparams)
{
	int ret;
	trace_rdev_join_ibss(&(*rdev).wiphy, dev, params);
	ret = (*rdev).(*ops).join_ibss(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_leave_ibss(*mut cfg80211_registered_devicerdev,
				  *mut net_devicedev)
{
	int ret;
	trace_rdev_leave_ibss(&(*rdev).wiphy, dev);
	ret = (*rdev).(*ops).leave_ibss(&(*rdev).wiphy, dev);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_wiphy_params(*mut cfg80211_registered_devicerdev, int radio_idx,
		      u32 changed)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_set_wiphy_params(&(*rdev).wiphy, radio_idx, changed);
	if ((*rdev).(*ops).set_wiphy_params)
		ret = (*rdev).(*ops).set_wiphy_params(&(*rdev).wiphy, radio_idx,
						  changed);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_set_tx_power(*mut cfg80211_registered_devicerdev,
				    *mut wireless_devwdev, int radio_idx,
				    enum nl80211_tx_power_setting type,
				    int mbm)
{
	int ret;
	trace_rdev_set_tx_power(&(*rdev).wiphy, wdev, radio_idx, type, mbm);
	ret = (*rdev).(*ops).set_tx_power(&(*rdev).wiphy, wdev, radio_idx, type,
				      mbm);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_get_tx_power(*mut cfg80211_registered_devicerdev,
				    *mut wireless_devwdev, int radio_idx,
				    u32 link_id, int *dbm)
{
	int ret;
	trace_rdev_get_tx_power(&(*rdev).wiphy, wdev, radio_idx, link_id);
	ret = (*rdev).(*ops).get_tx_power(&(*rdev).wiphy, wdev, radio_idx, link_id,
				      dbm);
	trace_rdev_return_int_int(&(*rdev).wiphy, ret, *dbm);
	return ret;
}

unsafe fn int
rdev_set_multicast_to_unicast(*mut cfg80211_registered_devicerdev,
			      *mut net_devicedev,
			      const bool enabled)
{
	int ret;
	trace_rdev_set_multicast_to_unicast(&(*rdev).wiphy, dev, enabled);
	ret = (*rdev).(*ops).set_multicast_to_unicast(&(*rdev).wiphy, dev, enabled);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_get_txq_stats(*mut cfg80211_registered_devicerdev,
		   *mut wireless_devwdev,
		   *mut cfg80211_txq_statstxqstats)
{
	int ret;
	trace_rdev_get_txq_stats(&(*rdev).wiphy, wdev);
	ret = (*rdev).(*ops).get_txq_stats(&(*rdev).wiphy, wdev, txqstats);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn () rdev_rfkill_poll(*mut cfg80211_registered_devicerdev)
{
	trace_rdev_rfkill_poll(&(*rdev).wiphy);
	(*rdev).(*ops).rfkill_poll(&(*rdev).wiphy);
	trace_rdev_return_void(&(*rdev).wiphy);
}
// Conditional on CONFIG_NL80211_TESTMODE.
unsafe fn int rdev_testmode_cmd(*mut cfg80211_registered_devicerdev,
				    *mut wireless_devwdev,
				    () *data, int len)
{
	int ret;
	trace_rdev_testmode_cmd(&(*rdev).wiphy, wdev);
	ret = (*rdev).(*ops).testmode_cmd(&(*rdev).wiphy, wdev, data, len);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_testmode_dump(*mut cfg80211_registered_devicerdev,
				     *mut sk_buffskb,
				     *mut netlink_callbackcb, () *data,
				     int len)
{
	int ret;
	trace_rdev_testmode_dump(&(*rdev).wiphy);
	ret = (*rdev).(*ops).testmode_dump(&(*rdev).wiphy, skb, cb, data, len);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}


unsafe fn int
rdev_set_bitrate_mask(*mut cfg80211_registered_devicerdev,
		      *mut net_devicedev, u32 link_id,
		      *const u8peer,
		      const *mut cfg80211_bitrate_maskmask)
{
	int ret;
	trace_rdev_set_bitrate_mask(&(*rdev).wiphy, dev, link_id, peer, mask);
	ret = (*rdev).(*ops).set_bitrate_mask(&(*rdev).wiphy, dev, link_id,
					  peer, mask);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_dump_survey(*mut cfg80211_registered_devicerdev,
				   *mut net_devicenetdev, int idx,
				   *mut survey_infoinfo)
{
	int ret;
	trace_rdev_dump_survey(&(*rdev).wiphy, netdev, idx);
	ret = (*rdev).(*ops).dump_survey(&(*rdev).wiphy, netdev, idx, info);
	if (ret < 0)
		trace_rdev_return_int(&(*rdev).wiphy, ret);
	else
		trace_rdev_return_int_survey_info(&(*rdev).wiphy, ret, info);
	return ret;
}

unsafe fn int rdev_set_pmksa(*mut cfg80211_registered_devicerdev,
				 *mut net_devicenetdev,
				 *mut cfg80211_pmksapmksa)
{
	int ret;
	trace_rdev_set_pmksa(&(*rdev).wiphy, netdev, pmksa);
	ret = (*rdev).(*ops).set_pmksa(&(*rdev).wiphy, netdev, pmksa);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_del_pmksa(*mut cfg80211_registered_devicerdev,
				 *mut net_devicenetdev,
				 *mut cfg80211_pmksapmksa)
{
	int ret;
	trace_rdev_del_pmksa(&(*rdev).wiphy, netdev, pmksa);
	ret = (*rdev).(*ops).del_pmksa(&(*rdev).wiphy, netdev, pmksa);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_flush_pmksa(*mut cfg80211_registered_devicerdev,
				   *mut net_devicenetdev)
{
	int ret;
	trace_rdev_flush_pmksa(&(*rdev).wiphy, netdev);
	ret = (*rdev).(*ops).flush_pmksa(&(*rdev).wiphy, netdev);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_remain_on_channel(*mut cfg80211_registered_devicerdev,
		       *mut wireless_devwdev,
		       *mut ieee80211_channelchan,
		       u32 duration, u64 cookie, *const u8rx_addr)
{
	int ret;
	trace_rdev_remain_on_channel(&(*rdev).wiphy, wdev, chan, duration,
				     rx_addr);
	ret = (*rdev).(*ops).remain_on_channel(&(*rdev).wiphy, wdev, chan,
					   duration, cookie, rx_addr);
	trace_rdev_return_int_cookie(&(*rdev).wiphy, ret, cookie);
	return ret;
}

unsafe fn int
rdev_cancel_remain_on_channel(*mut cfg80211_registered_devicerdev,
			      *mut wireless_devwdev, u64 cookie)
{
	int ret;
	trace_rdev_cancel_remain_on_channel(&(*rdev).wiphy, wdev, cookie);
	ret = (*rdev).(*ops).cancel_remain_on_channel(&(*rdev).wiphy, wdev, cookie);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_mgmt_tx(*mut cfg80211_registered_devicerdev,
			       *mut wireless_devwdev,
			       *mut cfg80211_mgmt_tx_paramsparams,
			       u64 cookie)
{
	int ret;
	trace_rdev_mgmt_tx(&(*rdev).wiphy, wdev, params);
	ret = (*rdev).(*ops).mgmt_tx(&(*rdev).wiphy, wdev, params, cookie);
	trace_rdev_return_int_cookie(&(*rdev).wiphy, ret, cookie);
	return ret;
}

unsafe fn int rdev_tx_control_port(*mut cfg80211_registered_devicerdev,
				       *mut net_devicedev,
				       *const core::ffi::c_voidbuf, usize len,
				       *const u8dest, __be16 proto,
				       const bool noencrypt, int link,
				       u64 cookie)
{
	int ret;
	trace_rdev_tx_control_port(&(*rdev).wiphy, dev, buf, len,
				   dest, proto, noencrypt, link);
	ret = (*rdev).(*ops).tx_control_port(&(*rdev).wiphy, dev, buf, len,
					 dest, proto, noencrypt, link, cookie);
	if (cookie)
		trace_rdev_return_int_cookie(&(*rdev).wiphy, ret, cookie);
	else
		trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_mgmt_tx_cancel_wait(*mut cfg80211_registered_devicerdev,
			 *mut wireless_devwdev, u64 cookie)
{
	int ret;
	trace_rdev_mgmt_tx_cancel_wait(&(*rdev).wiphy, wdev, cookie);
	ret = (*rdev).(*ops).mgmt_tx_cancel_wait(&(*rdev).wiphy, wdev, cookie);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_set_power_mgmt(*mut cfg80211_registered_devicerdev,
				      *mut net_devicedev, bool enabled,
				      int timeout)
{
	int ret;
	trace_rdev_set_power_mgmt(&(*rdev).wiphy, dev, enabled, timeout);
	ret = (*rdev).(*ops).set_power_mgmt(&(*rdev).wiphy, dev, enabled, timeout);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_cqm_rssi_config(*mut cfg80211_registered_devicerdev,
			 *mut net_devicedev, s32 rssi_thold, u32 rssi_hyst)
{
	int ret;
	trace_rdev_set_cqm_rssi_config(&(*rdev).wiphy, dev, rssi_thold,
				       rssi_hyst);
	ret = (*rdev).(*ops).set_cqm_rssi_config(&(*rdev).wiphy, dev, rssi_thold,
				       rssi_hyst);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_cqm_rssi_range_config(*mut cfg80211_registered_devicerdev,
			       *mut net_devicedev, s32 low, s32 high)
{
	int ret;
	trace_rdev_set_cqm_rssi_range_config(&(*rdev).wiphy, dev, low, high);
	ret = (*rdev).(*ops).set_cqm_rssi_range_config(&(*rdev).wiphy, dev,
						   low, high);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_cqm_txe_config(*mut cfg80211_registered_devicerdev,
			*mut net_devicedev, u32 rate, u32 pkts, u32 intvl)
{
	int ret;
	trace_rdev_set_cqm_txe_config(&(*rdev).wiphy, dev, rate, pkts, intvl);
	ret = (*rdev).(*ops).set_cqm_txe_config(&(*rdev).wiphy, dev, rate, pkts,
					     intvl);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn ()
rdev_update_mgmt_frame_registrations(*mut cfg80211_registered_devicerdev,
				     *mut wireless_devwdev,
				     *mut mgmt_frame_regsupd)
{
	might_sleep();

	trace_rdev_update_mgmt_frame_registrations(&(*rdev).wiphy, wdev, upd);
	if ((*rdev).(*ops).update_mgmt_frame_registrations)
		(*rdev).(*ops).update_mgmt_frame_registrations(&(*rdev).wiphy, wdev,
							   upd);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int rdev_set_antenna(*mut cfg80211_registered_devicerdev,
				   int radio_idx, u32 tx_ant, u32 rx_ant)
{
	int ret;
	trace_rdev_set_antenna(&(*rdev).wiphy, radio_idx, tx_ant, rx_ant);
	ret = (*rdev).(*ops).set_antenna(&(*rdev).wiphy, -1, tx_ant, rx_ant);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_get_antenna(*mut cfg80211_registered_devicerdev,
				   int radio_idx, u32 *tx_ant, u32 *rx_ant)
{
	int ret;
	trace_rdev_get_antenna(&(*rdev).wiphy, radio_idx);
	ret = (*rdev).(*ops).get_antenna(&(*rdev).wiphy, radio_idx, tx_ant, rx_ant);
	if (ret)
		trace_rdev_return_int(&(*rdev).wiphy, ret);
	else
		trace_rdev_return_int_tx_rx(&(*rdev).wiphy, ret, *tx_ant,
					    *rx_ant);
	return ret;
}

unsafe fn int
rdev_sched_scan_start(*mut cfg80211_registered_devicerdev,
		      *mut net_devicedev,
		      *mut cfg80211_sched_scan_requestrequest)
{
	int ret;
	trace_rdev_sched_scan_start(&(*rdev).wiphy, dev, (*request).reqid);
	ret = (*rdev).(*ops).sched_scan_start(&(*rdev).wiphy, dev, request);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_sched_scan_stop(*mut cfg80211_registered_devicerdev,
				       *mut net_devicedev, u64 reqid)
{
	int ret;
	trace_rdev_sched_scan_stop(&(*rdev).wiphy, dev, reqid);
	ret = (*rdev).(*ops).sched_scan_stop(&(*rdev).wiphy, dev, reqid);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_set_rekey_data(*mut cfg80211_registered_devicerdev,
				      *mut net_devicedev,
				      *mut cfg80211_gtk_rekey_datadata)
{
	int ret;
	trace_rdev_set_rekey_data(&(*rdev).wiphy, dev);
	ret = (*rdev).(*ops).set_rekey_data(&(*rdev).wiphy, dev, data);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_tdls_mgmt(*mut cfg80211_registered_devicerdev,
				 *mut net_devicedev, *mut u8peer,
				 int link_id, u8 action_code,
				 u8 dialog_token, u16 status_code,
				 u32 peer_capability, bool initiator,
				 *const u8buf, usize len)
{
	int ret;
	trace_rdev_tdls_mgmt(&(*rdev).wiphy, dev, peer, link_id, action_code,
			     dialog_token, status_code, peer_capability,
			     initiator, buf, len);
	ret = (*rdev).(*ops).tdls_mgmt(&(*rdev).wiphy, dev, peer, link_id,
				   action_code, dialog_token, status_code,
				   peer_capability, initiator, buf, len);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_tdls_oper(*mut cfg80211_registered_devicerdev,
				 *mut net_devicedev, *mut u8peer,
				 enum nl80211_tdls_operation oper)
{
	int ret;
	trace_rdev_tdls_oper(&(*rdev).wiphy, dev, peer, oper);
	ret = (*rdev).(*ops).tdls_oper(&(*rdev).wiphy, dev, peer, oper);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_probe_peer(*mut cfg80211_registered_devicerdev,
				  *mut net_devicedev, *const u8peer,
				  u64 cookie)
{
	int ret;
	trace_rdev_probe_peer(&(*rdev).wiphy, dev, peer);
	ret = (*rdev).(*ops).probe_peer(&(*rdev).wiphy, dev, peer, cookie);
	trace_rdev_return_int_cookie(&(*rdev).wiphy, ret, cookie);
	return ret;
}

unsafe fn int rdev_set_noack_map(*mut cfg80211_registered_devicerdev,
				     *mut net_devicedev, u16 noack_map)
{
	int ret;
	trace_rdev_set_noack_map(&(*rdev).wiphy, dev, noack_map);
	ret = (*rdev).(*ops).set_noack_map(&(*rdev).wiphy, dev, noack_map);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_get_channel(*mut cfg80211_registered_devicerdev,
		 *mut wireless_devwdev,
		 u32 link_id,
		 *mut cfg80211_chan_defchandef)
{
	int ret;

	trace_rdev_get_channel(&(*rdev).wiphy, wdev, link_id);
	ret = (*rdev).(*ops).get_channel(&(*rdev).wiphy, wdev, link_id, chandef);
	trace_rdev_return_chandef(&(*rdev).wiphy, ret, chandef);

	return ret;
}

unsafe fn int rdev_start_p2p_device(*mut cfg80211_registered_devicerdev,
					*mut wireless_devwdev)
{
	int ret;

	trace_rdev_start_p2p_device(&(*rdev).wiphy, wdev);
	ret = (*rdev).(*ops).start_p2p_device(&(*rdev).wiphy, wdev);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn () rdev_stop_p2p_device(*mut cfg80211_registered_devicerdev,
					*mut wireless_devwdev)
{
	trace_rdev_stop_p2p_device(&(*rdev).wiphy, wdev);
	(*rdev).(*ops).stop_p2p_device(&(*rdev).wiphy, wdev);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int rdev_start_nan(*mut cfg80211_registered_devicerdev,
				 *mut wireless_devwdev,
				 *mut cfg80211_nan_confconf)
{
	int ret;

	trace_rdev_start_nan(&(*rdev).wiphy, wdev, conf);
	ret = (*rdev).(*ops).start_nan(&(*rdev).wiphy, wdev, conf);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn () rdev_stop_nan(*mut cfg80211_registered_devicerdev,
				 *mut wireless_devwdev)
{
	trace_rdev_stop_nan(&(*rdev).wiphy, wdev);
	(*rdev).(*ops).stop_nan(&(*rdev).wiphy, wdev);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int
rdev_add_nan_func(*mut cfg80211_registered_devicerdev,
		  *mut wireless_devwdev,
		  *mut cfg80211_nan_funcnan_func)
{
	int ret;

	trace_rdev_add_nan_func(&(*rdev).wiphy, wdev, nan_func);
	ret = (*rdev).(*ops).add_nan_func(&(*rdev).wiphy, wdev, nan_func);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn () rdev_del_nan_func(*mut cfg80211_registered_devicerdev,
				    *mut wireless_devwdev, u64 cookie)
{
	trace_rdev_del_nan_func(&(*rdev).wiphy, wdev, cookie);
	(*rdev).(*ops).del_nan_func(&(*rdev).wiphy, wdev, cookie);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int
rdev_nan_change_conf(*mut cfg80211_registered_devicerdev,
		     *mut wireless_devwdev,
		     *mut cfg80211_nan_confconf, u32 changes)
{
	int ret;

	trace_rdev_nan_change_conf(&(*rdev).wiphy, wdev, conf, changes);
	if ((*rdev).(*ops).nan_change_conf)
		ret = (*rdev).(*ops).nan_change_conf(&(*rdev).wiphy, wdev, conf,
						 changes);
	else
		ret = -EOPNOTSUPP;
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_nan_set_local_sched(*mut cfg80211_registered_devicerdev,
			 *mut wireless_devwdev,
			 *mut cfg80211_nan_local_schedsched)
{
	int ret;

	trace_rdev_nan_set_local_sched(&(*rdev).wiphy, wdev, sched);
	if ((*rdev).(*ops).nan_set_local_sched)
		ret = (*rdev).(*ops).nan_set_local_sched(&(*rdev).wiphy, wdev, sched);
	else
		ret = -EOPNOTSUPP;
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_nan_set_peer_sched(*mut cfg80211_registered_devicerdev,
			*mut wireless_devwdev,
			*mut cfg80211_nan_peer_schedsched)
{
	int ret;

	trace_rdev_nan_set_peer_sched(&(*rdev).wiphy, wdev, sched);
	if ((*rdev).(*ops).nan_set_peer_sched)
		ret = (*rdev).(*ops).nan_set_peer_sched(&(*rdev).wiphy, wdev, sched);
	else
		ret = -EOPNOTSUPP;
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_start_pd(*mut cfg80211_registered_devicerdev,
				*mut wireless_devwdev)
{
	int ret;

	trace_rdev_start_pd(&(*rdev).wiphy, wdev);
	ret = (*rdev).(*ops).start_pd(&(*rdev).wiphy, wdev);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn () rdev_stop_pd(*mut cfg80211_registered_devicerdev,
				*mut wireless_devwdev)
{
	trace_rdev_stop_pd(&(*rdev).wiphy, wdev);
	(*rdev).(*ops).stop_pd(&(*rdev).wiphy, wdev);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int rdev_set_mac_acl(*mut cfg80211_registered_devicerdev,
				   *mut net_devicedev,
				   *mut cfg80211_acl_dataparams)
{
	int ret;

	trace_rdev_set_mac_acl(&(*rdev).wiphy, dev, params);
	ret = (*rdev).(*ops).set_mac_acl(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_update_ft_ies(*mut cfg80211_registered_devicerdev,
				     *mut net_devicedev,
				     *mut cfg80211_update_ft_ies_paramsftie)
{
	int ret;

	trace_rdev_update_ft_ies(&(*rdev).wiphy, dev, ftie);
	ret = (*rdev).(*ops).update_ft_ies(&(*rdev).wiphy, dev, ftie);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_crit_proto_start(*mut cfg80211_registered_devicerdev,
					*mut wireless_devwdev,
					enum nl80211_crit_proto_id protocol,
					u16 duration)
{
	int ret;

	trace_rdev_crit_proto_start(&(*rdev).wiphy, wdev, protocol, duration);
	ret = (*rdev).(*ops).crit_proto_start(&(*rdev).wiphy, wdev,
					  protocol, duration);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn () rdev_crit_proto_stop(*mut cfg80211_registered_devicerdev,
				       *mut wireless_devwdev)
{
	trace_rdev_crit_proto_stop(&(*rdev).wiphy, wdev);
	(*rdev).(*ops).crit_proto_stop(&(*rdev).wiphy, wdev);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int rdev_channel_switch(*mut cfg80211_registered_devicerdev,
				      *mut net_devicedev,
				      *mut cfg80211_csa_settingsparams)
{
	int ret;

	trace_rdev_channel_switch(&(*rdev).wiphy, dev, params);
	ret = (*rdev).(*ops).channel_switch(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_set_qos_map(*mut cfg80211_registered_devicerdev,
				   *mut net_devicedev,
				   *mut cfg80211_qos_mapqos_map)
{
	int ret = -EOPNOTSUPP;

	if ((*rdev).(*ops).set_qos_map) {
		trace_rdev_set_qos_map(&(*rdev).wiphy, dev, qos_map);
		ret = (*rdev).(*ops).set_qos_map(&(*rdev).wiphy, dev, qos_map);
		trace_rdev_return_int(&(*rdev).wiphy, ret);
	}

	return ret;
}

unsafe fn int
rdev_set_ap_chanwidth(*mut cfg80211_registered_devicerdev,
		      *mut net_devicedev,
		      u32 link_id,
		      *mut cfg80211_chan_defchandef)
{
	int ret;

	trace_rdev_set_ap_chanwidth(&(*rdev).wiphy, dev, link_id, chandef);
	ret = (*rdev).(*ops).set_ap_chanwidth(&(*rdev).wiphy, dev, link_id, chandef);
	trace_rdev_return_int(&(*rdev).wiphy, ret);

	return ret;
}

unsafe fn int
rdev_add_tx_ts(*mut cfg80211_registered_devicerdev,
	       *mut net_devicedev, u8 tsid, *const u8peer,
	       u8 user_prio, u16 admitted_time)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_add_tx_ts(&(*rdev).wiphy, dev, tsid, peer,
			     user_prio, admitted_time);
	if ((*rdev).(*ops).add_tx_ts)
		ret = (*rdev).(*ops).add_tx_ts(&(*rdev).wiphy, dev, tsid, peer,
					   user_prio, admitted_time);
	trace_rdev_return_int(&(*rdev).wiphy, ret);

	return ret;
}

unsafe fn int
rdev_del_tx_ts(*mut cfg80211_registered_devicerdev,
	       *mut net_devicedev, u8 tsid, *const u8peer)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_del_tx_ts(&(*rdev).wiphy, dev, tsid, peer);
	if ((*rdev).(*ops).del_tx_ts)
		ret = (*rdev).(*ops).del_tx_ts(&(*rdev).wiphy, dev, tsid, peer);
	trace_rdev_return_int(&(*rdev).wiphy, ret);

	return ret;
}

unsafe fn int
rdev_tdls_channel_switch(*mut cfg80211_registered_devicerdev,
			 *mut net_devicedev, *const u8addr,
			 u8 oper_class, *mut cfg80211_chan_defchandef)
{
	int ret;

	trace_rdev_tdls_channel_switch(&(*rdev).wiphy, dev, addr, oper_class,
				       chandef);
	ret = (*rdev).(*ops).tdls_channel_switch(&(*rdev).wiphy, dev, addr,
					     oper_class, chandef);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn ()
rdev_tdls_cancel_channel_switch(*mut cfg80211_registered_devicerdev,
				*mut net_devicedev, *const u8addr)
{
	trace_rdev_tdls_cancel_channel_switch(&(*rdev).wiphy, dev, addr);
	(*rdev).(*ops).tdls_cancel_channel_switch(&(*rdev).wiphy, dev, addr);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int
rdev_start_radar_detection(*mut cfg80211_registered_devicerdev,
			   *mut net_devicedev,
			   *mut cfg80211_chan_defchandef,
			   u32 cac_time_ms, int link_id)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_start_radar_detection(&(*rdev).wiphy, dev, chandef,
					 cac_time_ms, link_id);
	if ((*rdev).(*ops).start_radar_detection)
		ret = (*rdev).(*ops).start_radar_detection(&(*rdev).wiphy, dev,
						       chandef, cac_time_ms,
						       link_id);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn ()
rdev_end_cac(*mut cfg80211_registered_devicerdev,
	     *mut net_devicedev, u32 link_id)
{
	trace_rdev_end_cac(&(*rdev).wiphy, dev, link_id);
	if ((*rdev).(*ops).end_cac)
		(*rdev).(*ops).end_cac(&(*rdev).wiphy, dev, link_id);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int
rdev_set_mcast_rate(*mut cfg80211_registered_devicerdev,
		    *mut net_devicedev,
		    int mcast_rate[NUM_NL80211_BANDS])
{
	int ret = -EOPNOTSUPP;

	trace_rdev_set_mcast_rate(&(*rdev).wiphy, dev, mcast_rate);
	if ((*rdev).(*ops).set_mcast_rate)
		ret = (*rdev).(*ops).set_mcast_rate(&(*rdev).wiphy, dev, mcast_rate);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_coalesce(*mut cfg80211_registered_devicerdev,
		  *mut cfg80211_coalescecoalesce)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_set_coalesce(&(*rdev).wiphy, coalesce);
	if ((*rdev).(*ops).set_coalesce)
		ret = (*rdev).(*ops).set_coalesce(&(*rdev).wiphy, coalesce);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_set_pmk(*mut cfg80211_registered_devicerdev,
			       *mut net_devicedev,
			       *mut cfg80211_pmk_confpmk_conf)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_set_pmk(&(*rdev).wiphy, dev, pmk_conf);
	if ((*rdev).(*ops).set_pmk)
		ret = (*rdev).(*ops).set_pmk(&(*rdev).wiphy, dev, pmk_conf);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_del_pmk(*mut cfg80211_registered_devicerdev,
			       *mut net_devicedev, *const u8aa)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_del_pmk(&(*rdev).wiphy, dev, aa);
	if ((*rdev).(*ops).del_pmk)
		ret = (*rdev).(*ops).del_pmk(&(*rdev).wiphy, dev, aa);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_external_auth(*mut cfg80211_registered_devicerdev,
		   *mut net_devicedev,
		   *mut cfg80211_external_auth_paramsparams)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_external_auth(&(*rdev).wiphy, dev, params);
	if ((*rdev).(*ops).external_auth)
		ret = (*rdev).(*ops).external_auth(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_get_ftm_responder_stats(*mut cfg80211_registered_devicerdev,
			     *mut net_devicedev,
			     *mut cfg80211_ftm_responder_statsftm_stats)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_get_ftm_responder_stats(&(*rdev).wiphy, dev, ftm_stats);
	if ((*rdev).(*ops).get_ftm_responder_stats)
		ret = (*rdev).(*ops).get_ftm_responder_stats(&(*rdev).wiphy, dev,
							ftm_stats);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_start_pmsr(*mut cfg80211_registered_devicerdev,
		*mut wireless_devwdev,
		*mut cfg80211_pmsr_requestrequest)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_start_pmsr(&(*rdev).wiphy, wdev, (*request).cookie);
	if ((*rdev).(*ops).start_pmsr)
		ret = (*rdev).(*ops).start_pmsr(&(*rdev).wiphy, wdev, request);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn ()
rdev_abort_pmsr(*mut cfg80211_registered_devicerdev,
		*mut wireless_devwdev,
		*mut cfg80211_pmsr_requestrequest)
{
	trace_rdev_abort_pmsr(&(*rdev).wiphy, wdev, (*request).cookie);
	if ((*rdev).(*ops).abort_pmsr)
		(*rdev).(*ops).abort_pmsr(&(*rdev).wiphy, wdev, request);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int rdev_update_owe_info(*mut cfg80211_registered_devicerdev,
				       *mut net_devicedev,
				       *mut cfg80211_update_owe_infooweinfo)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_update_owe_info(&(*rdev).wiphy, dev, oweinfo);
	if ((*rdev).(*ops).update_owe_info)
		ret = (*rdev).(*ops).update_owe_info(&(*rdev).wiphy, dev, oweinfo);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_probe_mesh_link(*mut cfg80211_registered_devicerdev,
		     *mut net_devicedev, *const u8dest,
		     *const core::ffi::c_voidbuf, usize len)
{
	int ret;

	trace_rdev_probe_mesh_link(&(*rdev).wiphy, dev, dest, buf, len);
	ret = (*rdev).(*ops).probe_mesh_link(&(*rdev).wiphy, dev, buf, len);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_set_tid_config(*mut cfg80211_registered_devicerdev,
				      *mut net_devicedev,
				      *mut cfg80211_tid_configtid_conf)
{
	int ret;

	trace_rdev_set_tid_config(&(*rdev).wiphy, dev, tid_conf);
	ret = (*rdev).(*ops).set_tid_config(&(*rdev).wiphy, dev, tid_conf);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_reset_tid_config(*mut cfg80211_registered_devicerdev,
					*mut net_devicedev, *const u8peer,
					u8 tids)
{
	int ret;

	trace_rdev_reset_tid_config(&(*rdev).wiphy, dev, peer, tids);
	ret = (*rdev).(*ops).reset_tid_config(&(*rdev).wiphy, dev, peer, tids);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int rdev_set_sar_specs(*mut cfg80211_registered_devicerdev,
				     *mut cfg80211_sar_specssar)
{
	int ret;

	trace_rdev_set_sar_specs(&(*rdev).wiphy, sar);
	ret = (*rdev).(*ops).set_sar_specs(&(*rdev).wiphy, sar);
	trace_rdev_return_int(&(*rdev).wiphy, ret);

	return ret;
}

unsafe fn int rdev_color_change(*mut cfg80211_registered_devicerdev,
				    *mut net_devicedev,
				    *mut cfg80211_color_change_settingsparams)
{
	int ret;

	trace_rdev_color_change(&(*rdev).wiphy, dev, params);
	ret = (*rdev).(*ops).color_change(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);

	return ret;
}

unsafe fn int
rdev_set_fils_aad(*mut cfg80211_registered_devicerdev,
		  *mut net_devicedev, *mut cfg80211_fils_aadfils_aad)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_set_fils_aad(&(*rdev).wiphy, dev, fils_aad);
	if ((*rdev).(*ops).set_fils_aad)
		ret = (*rdev).(*ops).set_fils_aad(&(*rdev).wiphy, dev, fils_aad);
	trace_rdev_return_int(&(*rdev).wiphy, ret);

	return ret;
}

unsafe fn int
rdev_set_radar_background(*mut cfg80211_registered_devicerdev,
			  *mut cfg80211_chan_defchandef)
{
	*mut wiphywiphy = &(*rdev).wiphy;
	int ret = -EOPNOTSUPP;

	trace_rdev_set_radar_background(wiphy, chandef);
	if ((*rdev).(*ops).set_radar_background)
		ret = (*rdev).(*ops).set_radar_background(wiphy, chandef);
	trace_rdev_return_int(wiphy, ret);

	return ret;
}

unsafe fn int
rdev_add_intf_link(*mut cfg80211_registered_devicerdev,
		   *mut wireless_devwdev,
		   u32 link_id)
{
	int ret = 0;

	trace_rdev_add_intf_link(&(*rdev).wiphy, wdev, link_id);
	if ((*rdev).(*ops).add_intf_link)
		ret = (*rdev).(*ops).add_intf_link(&(*rdev).wiphy, wdev, link_id);
	trace_rdev_return_int(&(*rdev).wiphy, ret);

	return ret;
}

unsafe fn ()
rdev_del_intf_link(*mut cfg80211_registered_devicerdev,
		   *mut wireless_devwdev,
		   u32 link_id)
{
	trace_rdev_del_intf_link(&(*rdev).wiphy, wdev, link_id);
	if ((*rdev).(*ops).del_intf_link)
		(*rdev).(*ops).del_intf_link(&(*rdev).wiphy, wdev, link_id);
	trace_rdev_return_void(&(*rdev).wiphy);
}

unsafe fn int
rdev_add_link_station(*mut cfg80211_registered_devicerdev,
		      *mut net_devicedev,
		      *mut link_station_parametersparams)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_add_link_station(&(*rdev).wiphy, dev, params);
	if ((*rdev).(*ops).add_link_station)
		ret = (*rdev).(*ops).add_link_station(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_mod_link_station(*mut cfg80211_registered_devicerdev,
		      *mut net_devicedev,
		      *mut link_station_parametersparams)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_mod_link_station(&(*rdev).wiphy, dev, params);
	if ((*rdev).(*ops).mod_link_station)
		ret = (*rdev).(*ops).mod_link_station(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_del_link_station(*mut cfg80211_registered_devicerdev,
		      *mut net_devicedev,
		      *mut link_station_del_parametersparams)
{
	int ret = -EOPNOTSUPP;

	trace_rdev_del_link_station(&(*rdev).wiphy, dev, params);
	if ((*rdev).(*ops).del_link_station)
		ret = (*rdev).(*ops).del_link_station(&(*rdev).wiphy, dev, params);
	trace_rdev_return_int(&(*rdev).wiphy, ret);
	return ret;
}

unsafe fn int
rdev_set_hw_timestamp(*mut cfg80211_registered_devicerdev,
		      *mut net_devicedev,
		      *mut cfg80211_set_hw_timestamphwts)
{
	*mut wiphywiphy = &(*rdev).wiphy;
	int ret = -EOPNOTSUPP;

	trace_rdev_set_hw_timestamp(wiphy, dev, hwts);
	if ((*rdev).(*ops).set_hw_timestamp)
		ret = (*rdev).(*ops).set_hw_timestamp(wiphy, dev, hwts);
	trace_rdev_return_int(wiphy, ret);

	return ret;
}

unsafe fn int
rdev_set_ttlm(*mut cfg80211_registered_devicerdev,
	      *mut net_devicedev,
	      *mut cfg80211_ttlm_paramsparams)
{
	*mut wiphywiphy = &(*rdev).wiphy;
	int ret = -EOPNOTSUPP;

	trace_rdev_set_ttlm(wiphy, dev, params);
	if ((*rdev).(*ops).set_ttlm)
		ret = (*rdev).(*ops).set_ttlm(wiphy, dev, params);
	trace_rdev_return_int(wiphy, ret);

	return ret;
}

unsafe fn u32
rdev_get_radio_mask(*mut cfg80211_registered_devicerdev,
		    *mut net_devicedev)
{
	*mut wiphywiphy = &(*rdev).wiphy;

	if (!(*rdev).(*ops).get_radio_mask)
		return 0;

	return (*rdev).(*ops).get_radio_mask(wiphy, dev);
}

unsafe fn int
rdev_assoc_ml_reconf(*mut cfg80211_registered_devicerdev,
		     *mut net_devicedev,
		     *mut cfg80211_ml_reconf_reqreq)
{
	*mut wiphywiphy = &(*rdev).wiphy;
	int ret = -EOPNOTSUPP;

	trace_rdev_assoc_ml_reconf(wiphy, dev, req);
	if ((*rdev).(*ops).assoc_ml_reconf)
		ret = (*rdev).(*ops).assoc_ml_reconf(wiphy, dev, req);
	trace_rdev_return_int(wiphy, ret);

	return ret;
}

unsafe fn int
rdev_set_epcs(*mut cfg80211_registered_devicerdev,
	      *mut net_devicedev, bool val)
{
	*mut wiphywiphy = &(*rdev).wiphy;
	int ret = -EOPNOTSUPP;

	trace_rdev_set_epcs(wiphy, dev, val);
	if ((*rdev).(*ops).set_epcs)
		ret = (*rdev).(*ops).set_epcs(wiphy, dev, val);
	trace_rdev_return_int(wiphy, ret);

	return ret;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
