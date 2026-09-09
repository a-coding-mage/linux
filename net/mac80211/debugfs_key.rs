// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2003-2005 Devicescape Software, Inc.
 * Copyright (c) 2006 Jiri Benc <jbenc@suse.cz>
 * Copyright 2007 Johannes Berg <johannes@sipsolutions.net>
 * Copyright (C) 2015 Intel Deutschland GmbH
 * Copyright (C) 2021-2023 Intel Corporation
 */

// Declarations supplied by the surrounding mac80211/kernel translation.

unsafe fn key_keylen_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let key = (*file).private_data as *mut ieee80211_key;
    mac80211_format_buffer(userbuf, count, ppos, "%d\n", (*key).conf.keylen)
}
unsafe fn key_keyidx_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let key = (*file).private_data as *mut ieee80211_key;
    mac80211_format_buffer(userbuf, count, ppos, "%d\n", (*key).conf.keyidx)
}
unsafe fn key_hw_key_idx_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let key = (*file).private_data as *mut ieee80211_key;
    mac80211_format_buffer(userbuf, count, ppos, "%d\n", (*key).conf.hw_key_idx)
}
unsafe fn key_flags_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let key = (*file).private_data as *mut ieee80211_key;
    mac80211_format_buffer(userbuf, count, ppos, "0x%x\n", (*key).flags)
}
unsafe fn key_ifindex_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let key = (*file).private_data as *mut ieee80211_key;
    mac80211_format_buffer(userbuf, count, ppos, "%s\n", (*key).sdata.name)
}

unsafe fn key_algorithm_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let mut buf = [0u8; 15];
    let key = (*file).private_data as *mut ieee80211_key;
    let c: u32 = (*key).conf.cipher;
    let len = sprintf(buf.as_mut_ptr(), "%.2x-%.2x-%.2x:%d\n", c >> 24, (c >> 16) & 0xff, (c >> 8) & 0xff, c & 0xff);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), len)
}

unsafe fn key_tx_spec_write(file: *mut file, userbuf: *const u8, count: usize, _ppos: *mut loff_t) -> isize {
    let key = (*file).private_data as *mut ieee80211_key;
    let mut pn: u64 = 0;
    let ret: isize;
    match (*key).conf.cipher {
        WLAN_CIPHER_SUITE_WEP40 | WLAN_CIPHER_SUITE_WEP104 => return -EINVAL,
        WLAN_CIPHER_SUITE_TKIP => return -EOPNOTSUPP,
        WLAN_CIPHER_SUITE_CCMP | WLAN_CIPHER_SUITE_CCMP_256 |
        WLAN_CIPHER_SUITE_AES_CMAC | WLAN_CIPHER_SUITE_BIP_CMAC_256 |
        WLAN_CIPHER_SUITE_BIP_GMAC_128 | WLAN_CIPHER_SUITE_BIP_GMAC_256 |
        WLAN_CIPHER_SUITE_GCMP | WLAN_CIPHER_SUITE_GCMP_256 => {
            ret = kstrtou64_from_user(userbuf, count, 16, &mut pn);
            if ret != 0 { return ret; }
            if pn >= (1u64 << 48) { return -ERANGE; }
            atomic64_set(&mut (*key).conf.tx_pn, pn);
            count as isize
        }
        _ => 0,
    }
}

unsafe fn key_tx_spec_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let key = (*file).private_data as *mut ieee80211_key;
    let mut pn: u64 = 0;
    let mut buf = [0u8; 20];
    let len: isize;
    match (*key).conf.cipher {
        WLAN_CIPHER_SUITE_WEP40 | WLAN_CIPHER_SUITE_WEP104 => len = scnprintf(buf.as_mut_ptr(), buf.len(), "\n"),
        WLAN_CIPHER_SUITE_TKIP => {
            pn = atomic64_read(&(*key).conf.tx_pn);
            len = scnprintf(buf.as_mut_ptr(), buf.len(), "%08x %04x\n", TKIP_PN_TO_IV32(pn), TKIP_PN_TO_IV16(pn));
        }
        WLAN_CIPHER_SUITE_CCMP | WLAN_CIPHER_SUITE_CCMP_256 | WLAN_CIPHER_SUITE_AES_CMAC |
        WLAN_CIPHER_SUITE_BIP_CMAC_256 | WLAN_CIPHER_SUITE_BIP_GMAC_128 | WLAN_CIPHER_SUITE_BIP_GMAC_256 |
        WLAN_CIPHER_SUITE_GCMP | WLAN_CIPHER_SUITE_GCMP_256 => {
            pn = atomic64_read(&(*key).conf.tx_pn);
            len = scnprintf(buf.as_mut_ptr(), buf.len(), "%02x%02x%02x%02x%02x%02x\n",
                (pn >> 40) as u8, (pn >> 32) as u8, (pn >> 24) as u8, (pn >> 16) as u8, (pn >> 8) as u8, pn as u8);
        }
        _ => return 0,
    }
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), len)
}

unsafe fn key_rx_spec_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let key = (*file).private_data as *mut ieee80211_key;
    let mut buf = [0u8; 14 * IEEE80211_NUM_TIDS + 1];
    let mut p = buf.as_mut_ptr();
    let mut len: isize;
    match (*key).conf.cipher {
        WLAN_CIPHER_SUITE_WEP40 | WLAN_CIPHER_SUITE_WEP104 => len = scnprintf(p, buf.len(), "\n"),
        WLAN_CIPHER_SUITE_TKIP => { for i in 0..IEEE80211_NUM_TIDS { p = p.add(scnprintf(p, buf.len() - p.offset_from(buf.as_ptr()) as usize, "%08x %04x\n", (*key).u.tkip.rx[i].iv32, (*key).u.tkip.rx[i].iv16) as usize); } len = p.offset_from(buf.as_ptr()); }
        WLAN_CIPHER_SUITE_CCMP | WLAN_CIPHER_SUITE_CCMP_256 => { for i in 0..(IEEE80211_NUM_TIDS + 1) { let rpn = (*key).u.ccmp.rx_pn[i]; p = p.add(scnprintf(p, buf.len() - p.offset_from(buf.as_ptr()) as usize, "%02x%02x%02x%02x%02x%02x\n", rpn[0],rpn[1],rpn[2],rpn[3],rpn[4],rpn[5]) as usize); } len = p.offset_from(buf.as_ptr()); }
        WLAN_CIPHER_SUITE_AES_CMAC | WLAN_CIPHER_SUITE_BIP_CMAC_256 => { let rpn = (*key).u.aes_cmac.rx_pn; p = p.add(scnprintf(p, buf.len(), "%02x%02x%02x%02x%02x%02x\n", rpn[0],rpn[1],rpn[2],rpn[3],rpn[4],rpn[5]) as usize); len = p.offset_from(buf.as_ptr()); }
        WLAN_CIPHER_SUITE_BIP_GMAC_128 | WLAN_CIPHER_SUITE_BIP_GMAC_256 => { let rpn = (*key).u.aes_gmac.rx_pn; p = p.add(scnprintf(p, buf.len(), "%02x%02x%02x%02x%02x%02x\n", rpn[0],rpn[1],rpn[2],rpn[3],rpn[4],rpn[5]) as usize); len = p.offset_from(buf.as_ptr()); }
        WLAN_CIPHER_SUITE_GCMP | WLAN_CIPHER_SUITE_GCMP_256 => { for i in 0..(IEEE80211_NUM_TIDS + 1) { let rpn = (*key).u.gcmp.rx_pn[i]; p = p.add(scnprintf(p, buf.len() - p.offset_from(buf.as_ptr()) as usize, "%02x%02x%02x%02x%02x%02x\n", rpn[0],rpn[1],rpn[2],rpn[3],rpn[4],rpn[5]) as usize); } len = p.offset_from(buf.as_ptr()); }
        _ => return 0,
    }
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), len)
}

unsafe fn key_replays_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let key = (*file).private_data as *mut ieee80211_key; let mut buf=[0u8;20]; let len;
    match (*key).conf.cipher { WLAN_CIPHER_SUITE_CCMP|WLAN_CIPHER_SUITE_CCMP_256 => len=scnprintf(buf.as_mut_ptr(),20,"%u\n",(*key).u.ccmp.replays), WLAN_CIPHER_SUITE_AES_CMAC|WLAN_CIPHER_SUITE_BIP_CMAC_256=>len=scnprintf(buf.as_mut_ptr(),20,"%u\n",(*key).u.aes_cmac.replays), WLAN_CIPHER_SUITE_BIP_GMAC_128|WLAN_CIPHER_SUITE_BIP_GMAC_256=>len=scnprintf(buf.as_mut_ptr(),20,"%u\n",(*key).u.aes_gmac.replays), WLAN_CIPHER_SUITE_GCMP|WLAN_CIPHER_SUITE_GCMP_256=>len=scnprintf(buf.as_mut_ptr(),20,"%u\n",(*key).u.gcmp.replays), _=>return 0 }; simple_read_from_buffer(userbuf,count,ppos,buf.as_ptr(),len)
}
unsafe fn key_icverrors_read(file:*mut file,userbuf:*mut u8,count:usize,ppos:*mut loff_t)->isize { let key=(*file).private_data as *mut ieee80211_key; let mut buf=[0u8;20]; let len; match (*key).conf.cipher { WLAN_CIPHER_SUITE_AES_CMAC|WLAN_CIPHER_SUITE_BIP_CMAC_256=>len=scnprintf(buf.as_mut_ptr(),20,"%u\n",(*key).u.aes_cmac.icverrors), WLAN_CIPHER_SUITE_BIP_GMAC_128|WLAN_CIPHER_SUITE_BIP_GMAC_256=>len=scnprintf(buf.as_mut_ptr(),20,"%u\n",(*key).u.aes_gmac.icverrors), _=>return 0 }; simple_read_from_buffer(userbuf,count,ppos,buf.as_ptr(),len) }
unsafe fn key_mic_failures_read(file:*mut file,userbuf:*mut u8,count:usize,ppos:*mut loff_t)->isize { let key=(*file).private_data as *mut ieee80211_key; if (*key).conf.cipher != WLAN_CIPHER_SUITE_TKIP{return -EINVAL;} let mut buf=[0u8;20]; let len=scnprintf(buf.as_mut_ptr(),20,"%u\n",(*key).u.tkip.mic_failures); simple_read_from_buffer(userbuf,count,ppos,buf.as_ptr(),len) }
unsafe fn key_key_read(file:*mut file,userbuf:*mut u8,count:usize,ppos:*mut loff_t)->isize { let key=(*file).private_data as *mut ieee80211_key; let bufsize=2*(*key).conf.keylen+2; let buf=kmalloc(bufsize,GFP_KERNEL); if buf.is_null(){return -ENOMEM;} let mut p=buf; for i in 0..(*key).conf.keylen { p=p.add(scnprintf(p,bufsize+(buf as usize)-(p as usize),"%02x",(*key).conf.key[i]) as usize); } p=p.add(scnprintf(p,bufsize+(buf as usize)-(p as usize),"\n") as usize); let res=simple_read_from_buffer(userbuf,count,ppos,buf,(p as usize)-(buf as usize) as isize); kfree(buf); res }

pub unsafe fn ieee80211_debugfs_key_add(key:*mut ieee80211_key) { static mut KEYCOUNT:i32=0; let mut buf=[0u8;100]; let sta; if (*(*key).local).debugfs.keys.is_null(){return;} sprintf(buf.as_mut_ptr(),"%d",KEYCOUNT); (*key).debugfs.cnt=KEYCOUNT; KEYCOUNT+=1; (*key).debugfs.dir=debugfs_create_dir(buf.as_ptr(),(*(*key).local).debugfs.keys); sta=(*key).sta; if !sta.is_null(){sprintf(buf.as_mut_ptr(),"../../netdev:%s/stations/%pM",(*sta).sdata.name,(*sta).sta.addr.as_ptr());(*key).debugfs.stalink=debugfs_create_symlink("station",(*key).debugfs.dir,buf.as_ptr());} debugfs_create_file("keylen",0o400,(*key).debugfs.dir,key,&key_keylen_ops); debugfs_create_file("flags",0o400,(*key).debugfs.dir,key,&key_flags_ops); debugfs_create_file("keyidx",0o400,(*key).debugfs.dir,key,&key_keyidx_ops); debugfs_create_file("hw_key_idx",0o400,(*key).debugfs.dir,key,&key_hw_key_idx_ops); debugfs_create_file("algorithm",0o400,(*key).debugfs.dir,key,&key_algorithm_ops); debugfs_create_file("tx_spec",0o600,(*key).debugfs.dir,key,&key_tx_spec_ops); debugfs_create_file("rx_spec",0o400,(*key).debugfs.dir,key,&key_rx_spec_ops); debugfs_create_file("replays",0o400,(*key).debugfs.dir,key,&key_replays_ops); debugfs_create_file("icverrors",0o400,(*key).debugfs.dir,key,&key_icverrors_ops); debugfs_create_file("mic_failures",0o400,(*key).debugfs.dir,key,&key_mic_failures_ops); debugfs_create_file("key",0o400,(*key).debugfs.dir,key,&key_key_ops); debugfs_create_file("ifindex",0o400,(*key).debugfs.dir,key,&key_ifindex_ops); }
pub unsafe fn ieee80211_debugfs_key_remove(key:*mut ieee80211_key){if key.is_null(){return;}debugfs_remove_recursive((*key).debugfs.dir);(*key).debugfs.dir=core::ptr::null_mut();}
pub unsafe fn ieee80211_debugfs_key_update_default(sdata:*mut ieee80211_sub_if_data){let mut buf=[0u8;50];let key; if (*sdata).vif.debugfs_dir.is_null(){return;} lockdep_assert_wiphy((*(*sdata).local).hw.wiphy); debugfs_remove((*sdata).debugfs.default_unicast_key);(*sdata).debugfs.default_unicast_key=core::ptr::null_mut();if !(*sdata).default_unicast_key.is_null(){key=wiphy_dereference((*(*sdata).local).hw.wiphy,(*sdata).default_unicast_key);sprintf(buf.as_mut_ptr(),"../keys/%d",(*key).debugfs.cnt);(*sdata).debugfs.default_unicast_key=debugfs_create_symlink("default_unicast_key",(*sdata).vif.debugfs_dir,buf.as_ptr());}debugfs_remove((*sdata).debugfs.default_multicast_key);(*sdata).debugfs.default_multicast_key=core::ptr::null_mut();if !(*sdata).deflink.default_multicast_key.is_null(){key=wiphy_dereference((*(*sdata).local).hw.wiphy,(*sdata).deflink.default_multicast_key);sprintf(buf.as_mut_ptr(),"../keys/%d",(*key).debugfs.cnt);(*sdata).debugfs.default_multicast_key=debugfs_create_symlink("default_multicast_key",(*sdata).vif.debugfs_dir,buf.as_ptr());}}
pub unsafe fn ieee80211_debugfs_key_remove_mgmt_default(sdata:*mut ieee80211_sub_if_data){if sdata.is_null(){return;}debugfs_remove((*sdata).debugfs.default_mgmt_key);(*sdata).debugfs.default_mgmt_key=core::ptr::null_mut();}
pub unsafe fn ieee80211_debugfs_key_remove_beacon_default(sdata:*mut ieee80211_sub_if_data){if sdata.is_null(){return;}debugfs_remove((*sdata).debugfs.default_beacon_key);(*sdata).debugfs.default_beacon_key=core::ptr::null_mut();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
