// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of chan.c; kernel-provided types, constants, and
 * helpers are intentionally left as external dependencies. */

unsafe fn cfg80211_valid_60g_freq(freq: u32) -> bool { freq >= 58320 && freq <= 70200 }

pub unsafe fn cfg80211_chandef_create(chandef: *mut cfg80211_chan_def, chan: *mut ieee80211_channel, typ: nl80211_channel_type) {
    if chan.is_null() { WARN_ON(true); return; }
    *chandef = core::mem::zeroed(); (*chandef).chan = chan;
    WARN_ON((*chan).band == NL80211_BAND_60GHZ || (*chan).band == NL80211_BAND_S1GHZ);
    match typ {
        NL80211_CHAN_NO_HT => { (*chandef).width=NL80211_CHAN_WIDTH_20_NOHT; (*chandef).center_freq1=(*chan).center_freq; }
        NL80211_CHAN_HT20 => { (*chandef).width=NL80211_CHAN_WIDTH_20; (*chandef).center_freq1=(*chan).center_freq; }
        NL80211_CHAN_HT40PLUS => { (*chandef).width=NL80211_CHAN_WIDTH_40; (*chandef).center_freq1=(*chan).center_freq+10; }
        NL80211_CHAN_HT40MINUS => { (*chandef).width=NL80211_CHAN_WIDTH_40; (*chandef).center_freq1=(*chan).center_freq-10; }
        _ => { WARN_ON(true); }
    }
}

unsafe fn cfg80211_get_start_freq(c: *const cfg80211_chan_def, cf: u32) -> u32 {
    let center=MHZ_TO_KHZ(if cf==1 {(*c).center_freq1} else {(*c).center_freq2}); let bw=MHZ_TO_KHZ(cfg80211_chandef_get_width(c));
    if bw<=MHZ_TO_KHZ(20) {center} else {center-bw/2+MHZ_TO_KHZ(10)}
}
unsafe fn cfg80211_get_end_freq(c: *const cfg80211_chan_def, cf: u32) -> u32 {
    let center=MHZ_TO_KHZ(if cf==1 {(*c).center_freq1} else {(*c).center_freq2}); let bw=MHZ_TO_KHZ(cfg80211_chandef_get_width(c));
    if bw<=MHZ_TO_KHZ(20) {center} else {center+bw/2-MHZ_TO_KHZ(10)}
}

#[repr(C)] struct cfg80211_per_bw_puncturing_values { len:u8, valid_values:*const u16 }
static P80:[u16;4]=[8,4,2,1]; static P160:[u16;12]=[0x80,0x40,0x20,0x10,8,4,2,1,0xc0,0x30,0xc,3];
static P320:[u16;24]=[0xc000,0x3000,0xc00,0x300,0xc0,0x30,0xc,3,0xf000,0xf00,0xf0,0xf,0xfc00,0xf300,0xf0c0,0xf030,0xf00c,0xf003,0xc00f,0x300f,0xc0f,0x30f,0xcf,0x3f];
static PBW:[cfg80211_per_bw_puncturing_values;3]=[
    cfg80211_per_bw_puncturing_values{len:4,valid_values:P80.as_ptr()},
    cfg80211_per_bw_puncturing_values{len:12,valid_values:P160.as_ptr()},
    cfg80211_per_bw_puncturing_values{len:24,valid_values:P320.as_ptr()}];

unsafe fn valid_puncturing_bitmap(c:*const cfg80211_chan_def, primary:u32, punct:u32)->bool {
    let (idx,start)=match (*c).width { NL80211_CHAN_WIDTH_80=>(0,(*c).center_freq1-40), NL80211_CHAN_WIDTH_160=>(1,(*c).center_freq1-80), NL80211_CHAN_WIDTH_320=>(2,(*c).center_freq1-160), _=>return punct==0 };
    if punct==0{return true} if punct & (1u16<<((primary-start)/20)) as u32 !=0{return false}
    for i in 0..PBW[idx].len {if *PBW[idx].valid_values.add(i as usize)==punct as u16{return true}} false
}

unsafe fn cfg80211_edmg_chandef_valid(c:*const cfg80211_chan_def)->bool {
    if (*c).edmg.channels==0 || (*c).edmg.bw_config==0 || !cfg80211_valid_60g_freq((*(*c).chan).center_freq){return false}
    let mut maxc=0;let mut n=0;let mut cur=0; for i in 0..6 {if (*c).edmg.channels & (1<<i)!=0 {cur+=1;n+=1}else{cur=0} if cur>maxc{maxc=cur}}
    let need=match (*c).edmg.bw_config {4|8|12=>1,5|9|13=>2,6|10|14=>3,7|11|15=>4,_=>return false}; if maxc<need{return false}
    match (*c).edmg.bw_config {4|5|6|7=>true,8|9|10|11=>n>=2,12|13|14|15=>n>=4&&maxc>=2,_=>false}
}

pub fn nl80211_chan_width_to_mhz(w:nl80211_chan_width)->i32 { match w { NL80211_CHAN_WIDTH_1=>1,NL80211_CHAN_WIDTH_2=>2,NL80211_CHAN_WIDTH_4=>4,NL80211_CHAN_WIDTH_8=>8,NL80211_CHAN_WIDTH_16=>16,NL80211_CHAN_WIDTH_20|NL80211_CHAN_WIDTH_20_NOHT=>20,NL80211_CHAN_WIDTH_40=>40,NL80211_CHAN_WIDTH_80P80|NL80211_CHAN_WIDTH_80=>80,NL80211_CHAN_WIDTH_160=>160,NL80211_CHAN_WIDTH_320=>320,_=>{WARN_ON_ONCE(true);-1}} }

unsafe fn cfg80211_valid_center_freq(center:u32,width:nl80211_chan_width)->bool { if center<5955||center>7215{return true} let bw=nl80211_chan_width_to_mhz(width);if bw<0{return false} if center-bw as u32/2<5945||center+bw as u32/2>7225{return false} let step=if bw==320{160}else{bw as u32};(center-bw as u32/2-5945)%step==0 }
unsafe fn cfg80211_chandef_valid_control_freq(c:*const cfg80211_chan_def, f:u32)->bool { match (*c).width { NL80211_CHAN_WIDTH_20|NL80211_CHAN_WIDTH_20_NOHT|NL80211_CHAN_WIDTH_1|NL80211_CHAN_WIDTH_2|NL80211_CHAN_WIDTH_4|NL80211_CHAN_WIDTH_8|NL80211_CHAN_WIDTH_16=>true, NL80211_CHAN_WIDTH_320 if [150,130,110,90].iter().any(|d|(*c).center_freq1==f+*d||(*c).center_freq1==f-*d)=>true, NL80211_CHAN_WIDTH_160 if [70,50].iter().any(|d|(*c).center_freq1==f+*d||(*c).center_freq1==f-*d)=>true, NL80211_CHAN_WIDTH_80P80|NL80211_CHAN_WIDTH_80 if (*c).center_freq1==f+30||(*c).center_freq1==f-30=>true, NL80211_CHAN_WIDTH_40 if (*c).center_freq1==f+10||(*c).center_freq1==f-10=>true, _=>false } }

pub unsafe fn cfg80211_chandef_valid(c:*const cfg80211_chan_def)->bool {
    if (*c).chan.is_null()||(*c).freq1_offset>=1000{return false} let f=(*(*c).chan).center_freq;
    if cfg80211_chandef_is_s1g(c)&&!matches!((*c).width,NL80211_CHAN_WIDTH_1|NL80211_CHAN_WIDTH_2|NL80211_CHAN_WIDTH_4|NL80211_CHAN_WIDTH_8|NL80211_CHAN_WIDTH_16){return false}
    match (*c).width { NL80211_CHAN_WIDTH_20|NL80211_CHAN_WIDTH_20_NOHT=>{if ieee80211_chandef_to_khz(c)!=ieee80211_channel_to_khz((*c).chan)||(*c).center_freq2!=0{return false}}, NL80211_CHAN_WIDTH_80P80=>{if (*c).center_freq2==0||(*c).center_freq1.abs_diff((*c).center_freq2)==80{return false}}, _=>if (*c).center_freq2!=0{return false} }
    if !cfg80211_chandef_valid_control_freq(c,f)||!cfg80211_valid_center_freq((*c).center_freq1,(*c).width){return false}
    if (*c).width==NL80211_CHAN_WIDTH_80P80&&!cfg80211_valid_center_freq((*c).center_freq2,(*c).width){return false}
    if (*c).center_freq1==2484&&(*c).width!=NL80211_CHAN_WIDTH_20_NOHT{return false}
    if cfg80211_chandef_is_edmg(c)&&!cfg80211_edmg_chandef_valid(c){return false}
    if !cfg80211_chandef_is_s1g(c)&&(*c).s1g_primary_2mhz{return false} valid_puncturing_bitmap(c,f,(*c).punctured)
}

pub unsafe fn cfg80211_chandef_primary(c:*const cfg80211_chan_def,pw:nl80211_chan_width,punct:*mut u16)->i32 {let mut p=0u16;let out=if punct.is_null(){&mut p}else{&mut *punct};let mut pri=nl80211_chan_width_to_mhz(pw);let mut w=cfg80211_chandef_get_width(c);if pri<0||w<0||pri>w{return -1}*out=(*c).punctured;let mut center=(*c).center_freq1;let control=(*(*c).chan).center_freq;while w>pri{let b=w/20/2;if control>center{center+=w/4;*out>>=b}else{center-=w/4;*out&=((1<<b)-1) as u16}w/=2}center as i32}

pub unsafe fn cfg80211_is_sub_chan(c:*mut cfg80211_chan_def,chan:*mut ieee80211_channel,primary:bool)->bool {if (*c).chan.is_null(){return false}if (*(*c).chan).center_freq==(*chan).center_freq{return true}if primary{return false}let w=cfg80211_chandef_get_width(c);if w<=20{return false}for f in ((*c).center_freq1-w/2+10..=(*c).center_freq1+w/2-10).step_by(20){if (*chan).center_freq==f{return true}}if (*c).center_freq2==0{return false}for f in ((*c).center_freq2-w/2+10..=(*c).center_freq2+w/2-10).step_by(20){if (*chan).center_freq==f{return true}}false}

// Remaining declarations are supplied by the surrounding kernel translation.
extern "C" {
    pub fn cfg80211_chandef_dfs_required(w:*mut wiphy,c:*const cfg80211_chan_def,i:nl80211_iftype)->i32;
    pub fn cfg80211_chandef_dfs_usable(w:*mut wiphy,c:*const cfg80211_chan_def)->bool;
    pub fn cfg80211_chandef_usable(w:*mut wiphy,c:*const cfg80211_chan_def,p:u32)->bool;
    pub fn cfg80211_any_usable_channels(w:*mut wiphy,m:usize,p:u32)->bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
