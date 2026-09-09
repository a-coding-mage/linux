// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of parse.c; external kernel symbols are supplied by the surrounding crate. */

static EMPTY_NON_INHERITANCE: [u8; 3] = [WLAN_EID_EXTENSION, 1, WLAN_EID_EXT_NON_INHERITANCE];

#[repr(C)]
struct ieee80211_elem_defrag { elem: *const element, start: *const u8, len: usize }

#[repr(C)]
struct ieee80211_elems_parse {
    elems: ieee802_11_elems,
    ml_reconf: ieee80211_elem_defrag,
    ml_epcs: ieee80211_elem_defrag,
    ml_basic: ieee80211_elem_defrag,
    inside_multilink: bool,
    skip_vendor: bool,
    scratch_len: usize,
    scratch_pos: *mut u8,
    scratch: [u8; 0],
}

unsafe fn ieee80211_parse_extension_element(crc: *mut u32, elem: *const element,
    p: *mut ieee80211_elems_parse, params: *mut ieee80211_elems_parse_params) {
    let elems = &mut (*p).elems;
    let data = (*elem).data.add(1) as *const core::ffi::c_void;
    let mut calc_crc = false;
    if (*elem).datalen == 0 { return; }
    let len = (*elem).datalen - 1;
    match (*elem).data[0] {
        WLAN_EID_EXT_HE_MU_EDCA => { if (*params).mode < IEEE80211_CONN_MODE_HE { return; } calc_crc=true; if len >= core::mem::size_of_val(&elems.mu_edca_param_set) { elems.mu_edca_param_set=data; } }
        WLAN_EID_EXT_HE_CAPABILITY => { if (*params).mode >= IEEE80211_CONN_MODE_HE && ieee80211_he_capa_size_ok(data,len) { elems.he_cap=data; elems.he_cap_len=len; } }
        WLAN_EID_EXT_HE_OPERATION => { if (*params).mode >= IEEE80211_CONN_MODE_HE { calc_crc=true; if len >= core::mem::size_of_val(&elems.he_operation) && len >= ieee80211_he_oper_size(data)-1 { elems.he_operation=data; } } }
        WLAN_EID_EXT_UORA => { if (*params).mode >= IEEE80211_CONN_MODE_HE && len >= 1 { elems.uora_element=data; } }
        WLAN_EID_EXT_MAX_CHANNEL_SWITCH_TIME => { if len == 3 { elems.max_channel_switch_time=data; } }
        WLAN_EID_EXT_MULTIPLE_BSSID_CONFIGURATION => { if len >= core::mem::size_of_val(&elems.mbssid_config_ie) { elems.mbssid_config_ie=data; } }
        WLAN_EID_EXT_HE_SPR => { if (*params).mode >= IEEE80211_CONN_MODE_HE && len >= core::mem::size_of_val(&elems.he_spr) && len >= ieee80211_he_spr_size(data)-1 { elems.he_spr=data; } }
        WLAN_EID_EXT_HE_6GHZ_CAPA => { if (*params).mode >= IEEE80211_CONN_MODE_HE && len >= core::mem::size_of_val(&elems.he_6ghz_capa) { elems.he_6ghz_capa=data; } }
        WLAN_EID_EXT_EHT_CAPABILITY => { if (*params).mode >= IEEE80211_CONN_MODE_EHT && ieee80211_eht_capa_size_ok(elems.he_cap,data,len,(*params).from_ap) { elems.eht_cap=data; elems.eht_cap_len=len; } }
        WLAN_EID_EXT_EHT_OPERATION => { if (*params).mode >= IEEE80211_CONN_MODE_EHT { if ieee80211_eht_oper_size_ok(data,len) { elems.eht_operation=data; } calc_crc=true; } }
        WLAN_EID_EXT_EHT_MULTI_LINK => { if (*params).mode >= IEEE80211_CONN_MODE_EHT { calc_crc=true; if ieee80211_mle_size_ok(data,len) { let mle=data as *const ieee80211_multi_link_elem; match le16_get_bits((*mle).control,IEEE80211_ML_CONTROL_TYPE) { IEEE80211_ML_CONTROL_TYPE_BASIC => { if (*p).inside_multilink { elems.parse_error |= IEEE80211_PARSE_ERR_DUP_NEST_ML_BASIC; } else { (*p).ml_basic=ieee80211_elem_defrag{elem,start:(*params).start,len:(*params).len}; } }, IEEE80211_ML_CONTROL_TYPE_RECONF => (*p).ml_reconf=ieee80211_elem_defrag{elem,start:(*params).start,len:(*params).len}, IEEE80211_ML_CONTROL_TYPE_PRIO_ACCESS => (*p).ml_epcs=ieee80211_elem_defrag{elem,start:(*params).start,len:(*params).len}, _=>{} } } } }
        WLAN_EID_EXT_BANDWIDTH_INDICATION => { if (*params).mode >= IEEE80211_CONN_MODE_EHT { if ieee80211_bandwidth_indication_size_ok(data,len) { elems.bandwidth_indication=data; } calc_crc=true; } }
        WLAN_EID_EXT_TID_TO_LINK_MAPPING => { if (*params).mode >= IEEE80211_CONN_MODE_EHT { calc_crc=true; if ieee80211_tid_to_link_map_size_ok(data,len) && elems.ttlm_num < elems.ttlm.len() { elems.ttlm[elems.ttlm_num]=data; elems.ttlm_num+=1; } } }
        WLAN_EID_EXT_UHR_OPER => { if (*params).mode >= IEEE80211_CONN_MODE_UHR { calc_crc=true; if ieee80211_uhr_oper_size_ok(data,len) { elems.uhr_operation=data; elems.uhr_operation_len=len; } } }
        WLAN_EID_EXT_UHR_CAPA => { if (*params).mode >= IEEE80211_CONN_MODE_UHR { calc_crc=true; if ieee80211_uhr_capa_size_ok(data,len,true) { elems.uhr_cap=data; elems.uhr_cap_len=len; } } }
        _ => {}
    }
    if !crc.is_null() && calc_crc { *crc=crc32_be(*crc,elem as *mut _,(*elem).datalen+2); }
}

unsafe fn ieee80211_parse_tpe(tpe:*mut ieee80211_parsed_tpe,data:*const u8,len:u8) {
    let env=data as *const ieee80211_tx_pwr_env; if !ieee80211_valid_tpe_element(data,len){return;}
    let count=u8_get_bits((*env).info,IEEE80211_TX_PWR_ENV_INFO_COUNT); let interpret=u8_get_bits((*env).info,IEEE80211_TX_PWR_ENV_INFO_INTERPRET); let category=u8_get_bits((*env).info,IEEE80211_TX_PWR_ENV_INFO_CATEGORY);
    let (out,cnt,n,valid) = match interpret { IEEE80211_TPE_LOCAL_EIRP=>(&mut (*tpe).max_local[category],false), IEEE80211_TPE_REG_CLIENT_EIRP=>(&mut (*tpe).max_reg_client[category],false), IEEE80211_TPE_LOCAL_EIRP_PSD=>(&mut (*tpe).psd_local[category],true), IEEE80211_TPE_REG_CLIENT_EIRP_PSD=>(&mut (*tpe).psd_reg_client[category],true), _=>return };
    out.valid=true; if !valid { core::ptr::copy_nonoverlapping((*env).variable.as_ptr(),out.power.as_mut_ptr(),(count+1) as usize); out.count=count+1; if count==3 && len as usize>core::mem::size_of::<ieee80211_tx_pwr_env>()+count as usize+1 {out.power[4]=(*env).variable[4];out.count=5;} } else if count==0 { core::ptr::write_bytes(out.power.as_mut_ptr(),(*env).variable[0],IEEE80211_TPE_PSD_ENTRIES_320MHZ as usize); out.count=IEEE80211_TPE_PSD_ENTRIES_320MHZ; } else { let nval=1u8 << (count-1); core::ptr::copy_nonoverlapping((*env).variable.as_ptr(),out.power.as_mut_ptr(),nval as usize); out.count=nval; out.n=nval; if len as usize>core::mem::size_of::<ieee80211_tx_pwr_env>()+nval as usize { let k=min(u8_get_bits((*env).variable[nval as usize],IEEE80211_TX_PWR_ENV_EXT_COUNT),IEEE80211_TPE_PSD_ENTRIES_320MHZ-nval); core::ptr::copy_nonoverlapping((*env).variable.as_ptr().add(nval as usize+1),out.power.as_mut_ptr().add(nval as usize),k as usize); out.count+=k; } }
}

/* The following parser preserves the C loop and dispatch structure. */
unsafe fn _ieee802_11_parse_elems_full(params:*mut ieee80211_elems_parse_params, p:*mut ieee80211_elems_parse, inherit:*const element)->u32 {
    let elems=&mut (*p).elems; let mut crc=(*params).crc; let mut seen=[0u64;4]; let calc=(*params).filter!=0;
    for_each_element!(elem,(*params).start,(*params).len,{ let id=(*elem).id; let elen=(*elem).datalen; let pos=(*elem).data; if !inherit.is_null() && !cfg80211_is_element_inherited(elem,inherit){continue;}
        if calc && id<64 && ((*params).filter & (1u64<<id))!=0 {crc=crc32_be(crc,pos.sub(2) as *mut _,elen+2);}
        match id {
            WLAN_EID_SSID=>{elems.ssid=pos;elems.ssid_len=elen}, WLAN_EID_SUPP_RATES=>{elems.supp_rates=pos;elems.supp_rates_len=elen}, WLAN_EID_EXT_SUPP_RATES=>{elems.ext_supp_rates=pos;elems.ext_supp_rates_len=elen}, WLAN_EID_EXT_CAPABILITY=>{elems.ext_capab=pos;elems.ext_capab_len=elen}, WLAN_EID_RSN=>{elems.rsn=pos;elems.rsn_len=elen}, WLAN_EID_RSNX=>{elems.rsnx=pos;elems.rsnx_len=elen}, WLAN_EID_MESH_ID=>{elems.mesh_id=pos;elems.mesh_id_len=elen}, WLAN_EID_PEER_MGMT=>{elems.peering=pos;elems.peering_len=elen}, WLAN_EID_COUNTRY=>{elems.country_elem=pos;elems.country_elem_len=elen}, WLAN_EID_TIM=>{if elen>=core::mem::size_of::<ieee80211_tim_ie>(){elems.tim=pos as *const _;elems.tim_len=elen;}}, WLAN_EID_DS_PARAMS=>{if elen>=1{elems.ds_params=pos;}}, WLAN_EID_ERP_INFO=>{if elen>=1{elems.erp_info=pos;}}, WLAN_EID_TX_POWER_ENVELOPE=>{if (*params).mode>=IEEE80211_CONN_MODE_HE{ieee80211_parse_tpe(&mut elems.tpe,pos,elen);}}, WLAN_EID_EXTENSION=>{ieee80211_parse_extension_element(if calc{&mut crc}else{core::ptr::null_mut()},elem,p,params)}, _=>{}
        }
    }); crc
}

pub unsafe fn ieee802_11_parse_elems_full(params:*mut ieee80211_elems_parse_params)->*mut ieee802_11_elems { let p=kzalloc_flex::<ieee80211_elems_parse>((*params).len*3); if p.is_null(){return core::ptr::null_mut();} (*p).elems.frame_type=(*params).type_;(*p).elems.from_ap=(*params).from_ap;(*p).scratch_len=(*params).len*3;(*p).scratch_pos=(*p).scratch.as_mut_ptr();(*p).elems.ie_start=(*params).start;(*p).elems.total_len=(*params).len;ieee80211_clear_tpe(&mut (*p).elems.tpe);ieee80211_clear_tpe(&mut (*p).elems.csa_tpe);(*p).elems.crc=_ieee802_11_parse_elems_full(params,p,core::ptr::null());&mut (*p).elems }

pub unsafe fn ieee80211_parse_bitrates(sband:*const ieee80211_supported_band,srates:*const u8,srates_len:i32,rates:*mut u32)->i32 { *rates=0;let mut count=0;for i in 0..srates_len {let rate=*srates.add(i as usize)&0x7f;for j in 0..(*sband).n_bitrates {let br=&(*sband).bitrates[j as usize];if (br.bitrate+4)/5==rate as i32 {*rates|=1u32<<j;count+=1;break;}}}count }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
