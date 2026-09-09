/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/sound/soc-dapm.h. */

/* Included C types and macros are supplied by other translated headers. */

pub const SND_SOC_NOPM: i32 = -1;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct regulator { _private: [u8; 0] }
#[repr(C)]
pub struct soc_enum { _private: [u8; 0] }
#[repr(C)]
pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_pcm_runtime { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)]
pub struct snd_ctl_elem_value { _private: [u8; 0] }
#[repr(C)]
pub struct snd_ctl_elem_info { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dai { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_card { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)]
pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dobj { _private: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct pinctrl { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum snd_soc_bias_level { SND_SOC_BIAS_OFF=0, SND_SOC_BIAS_STANDBY=1, SND_SOC_BIAS_PREPARE=2, SND_SOC_BIAS_ON=3 }

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum snd_soc_dapm_type {
    snd_soc_dapm_input=0, snd_soc_dapm_output, snd_soc_dapm_mux, snd_soc_dapm_mux_named_ctl,
    snd_soc_dapm_demux, snd_soc_dapm_mixer, snd_soc_dapm_mixer_named_ctl, snd_soc_dapm_pga,
    snd_soc_dapm_out_drv, snd_soc_dapm_adc, snd_soc_dapm_dac, snd_soc_dapm_micbias,
    snd_soc_dapm_mic, snd_soc_dapm_hp, snd_soc_dapm_spk, snd_soc_dapm_line, snd_soc_dapm_switch,
    snd_soc_dapm_vmid, snd_soc_dapm_pre, snd_soc_dapm_post, snd_soc_dapm_supply,
    snd_soc_dapm_pinctrl, snd_soc_dapm_regulator_supply, snd_soc_dapm_clock_supply,
    snd_soc_dapm_aif_in, snd_soc_dapm_aif_out, snd_soc_dapm_siggen, snd_soc_dapm_sink,
    snd_soc_dapm_dai_in, snd_soc_dapm_dai_out, snd_soc_dapm_dai_link, snd_soc_dapm_kcontrol,
    snd_soc_dapm_buffer, snd_soc_dapm_scheduler, snd_soc_dapm_effect, snd_soc_dapm_src,
    snd_soc_dapm_asrc, snd_soc_dapm_encoder, snd_soc_dapm_decoder, SND_SOC_DAPM_TYPE_COUNT,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum snd_soc_dapm_direction { SND_SOC_DAPM_DIR_IN=0, SND_SOC_DAPM_DIR_OUT=1 }

pub const SND_SOC_DAPM_STREAM_NOP:u32=0x0; pub const SND_SOC_DAPM_STREAM_START:u32=0x1;
pub const SND_SOC_DAPM_STREAM_STOP:u32=0x2; pub const SND_SOC_DAPM_STREAM_SUSPEND:u32=0x4;
pub const SND_SOC_DAPM_STREAM_RESUME:u32=0x8; pub const SND_SOC_DAPM_STREAM_PAUSE_PUSH:u32=0x10;
pub const SND_SOC_DAPM_STREAM_PAUSE_RELEASE:u32=0x20;
pub const SND_SOC_DAPM_PRE_PMU:u32=0x1; pub const SND_SOC_DAPM_POST_PMU:u32=0x2;
pub const SND_SOC_DAPM_PRE_PMD:u32=0x4; pub const SND_SOC_DAPM_POST_PMD:u32=0x8;
pub const SND_SOC_DAPM_PRE_REG:u32=0x10; pub const SND_SOC_DAPM_POST_REG:u32=0x20;
pub const SND_SOC_DAPM_WILL_PMU:u32=0x40; pub const SND_SOC_DAPM_WILL_PMD:u32=0x80;
pub const SND_SOC_DAPM_PRE_POST_PMD:u32=SND_SOC_DAPM_PRE_PMD|SND_SOC_DAPM_POST_PMD;
pub const SND_SOC_DAPM_PRE_POST_PMU:u32=SND_SOC_DAPM_PRE_PMU|SND_SOC_DAPM_POST_PMU;
pub const SND_SOC_DAPM_REGULATOR_BYPASS:u32=1;

#[repr(C)] pub struct snd_soc_dapm_route { pub sink:*const i8, pub control:*const i8, pub source:*const i8, pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget,*mut snd_soc_dapm_widget)->i32>, pub dobj:snd_soc_dobj }
#[repr(C)] pub struct snd_soc_dapm_path { pub name:*const i8, pub source:*mut snd_soc_dapm_widget, pub sink:*mut snd_soc_dapm_widget, pub node:[*mut snd_soc_dapm_widget;2], pub connect:u32, pub walking:u32, pub is_supply:u32, pub connected:Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget,*mut snd_soc_dapm_widget)->i32>, pub list_node:[list_head;2], pub list_kcontrol:list_head, pub list:list_head }
#[repr(C)] pub struct snd_soc_dapm_widget { pub id:snd_soc_dapm_type, pub name:*const i8, pub sname:*const i8, pub list:list_head, pub dapm:*mut snd_soc_dapm_context, pub priv_:*mut core::ffi::c_void, pub regulator:*mut regulator, pub pinctrl:*mut pinctrl, pub reg:i32, pub shift:u8, pub mask:u32, pub on_val:u32, pub off_val:u32, pub power:u8, pub active:u8, pub connected:u8, pub new_:u8, pub force:u8, pub ignore_suspend:u8, pub new_power:u8, pub power_checked:u8, pub is_supply:u8, pub is_ep:u8, pub no_wname_in_kcontrol_name:u8, pub subseq:i32, pub power_check:Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget)->i32>, pub event_flags:u16, pub event:Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget,*mut snd_kcontrol,i32)->i32>, pub num_kcontrols:i32, pub kcontrol_news:*const snd_kcontrol_new, pub kcontrols:*mut *mut snd_kcontrol, pub dobj:snd_soc_dobj, pub edges:[list_head;2], pub work_list:list_head, pub power_list:list_head, pub dirty:list_head, pub endpoints:[i32;2], pub clk:*mut clk, pub channel:i32 }
#[repr(C)] pub struct snd_soc_dapm_update { pub kcontrol:*mut snd_kcontrol,pub reg:i32,pub mask:i32,pub val:i32,pub reg2:i32,pub mask2:i32,pub val2:i32,pub has_second_set:bool }
#[repr(C)] pub struct snd_soc_dapm_widget_list { pub num_widgets:i32, pub widgets:[*mut snd_soc_dapm_widget;0] }
#[repr(C)] pub struct snd_soc_dapm_stats { pub power_checks:i32,pub path_checks:i32,pub neighbour_checks:i32 }
#[repr(C)] pub struct snd_soc_dapm_pinctrl_priv { pub active_state:*const i8,pub sleep_state:*const i8 }

#[macro_export] macro_rules! SND_SOC_DAPM_EVENT_ON { ($e:expr) => { ($e & (SND_SOC_DAPM_PRE_PMU|SND_SOC_DAPM_POST_PMU)) }; }
#[macro_export] macro_rules! SND_SOC_DAPM_EVENT_OFF { ($e:expr) => { ($e & (SND_SOC_DAPM_PRE_PMD|SND_SOC_DAPM_POST_PMD)) }; }
#[macro_export] macro_rules! SND_SOC_DAPM_DIR_TO_EP { ($x:expr) => { 1u32 << ($x as u32) }; }
pub const SND_SOC_DAPM_EP_SOURCE:u32=1; pub const SND_SOC_DAPM_EP_SINK:u32=2;

/* C initializer macros are represented as Rust macro_rules initializers. */
#[macro_export] macro_rules! SND_SOC_DAPM_INIT_REG_VAL { ($wreg:expr,$wshift:expr,$winvert:expr) => { reg:$wreg, mask:1, shift:$wshift, on_val:if $winvert {0} else {1}, off_val:if $winvert {1} else {0} }; }
#[macro_export] macro_rules! SND_SOC_DAPM_REG { ($wid:expr,$wname:expr,$wreg:expr,$wshift:expr,$wmask:expr,$won:expr,$woff:expr) => { snd_soc_dapm_widget { id:$wid,name:$wname,sname::core::ptr::null(),list:list_head{_private:[]},dapm::core::ptr::null_mut(),priv_:::core::ptr::null_mut(),regulator::core::ptr::null_mut(),pinctrl::core::ptr::null_mut(),reg:$wreg,shift:$wshift,mask:$wmask,on_val:$won,off_val:$woff,..unsafe{::core::mem::zeroed()} } }; }

extern "C" {
    pub fn snd_soc_dapm_alloc(dev:*mut device)->*mut snd_soc_dapm_context;
    pub fn snd_soc_dapm_regulator_event(w:*mut snd_soc_dapm_widget,k:*mut snd_kcontrol,event:i32)->i32;
    pub fn snd_soc_dapm_clock_event(w:*mut snd_soc_dapm_widget,k:*mut snd_kcontrol,event:i32)->i32;
    pub fn snd_soc_dapm_pinctrl_event(w:*mut snd_soc_dapm_widget,k:*mut snd_kcontrol,event:i32)->i32;
    pub fn snd_soc_dapm_put_volsw(k:*mut snd_kcontrol,u:*mut snd_ctl_elem_value)->i32;
    pub fn snd_soc_dapm_get_volsw(k:*mut snd_kcontrol,u:*mut snd_ctl_elem_value)->i32;
    pub fn snd_soc_dapm_get_enum_double(k:*mut snd_kcontrol,u:*mut snd_ctl_elem_value)->i32;
    pub fn snd_soc_dapm_put_enum_double(k:*mut snd_kcontrol,u:*mut snd_ctl_elem_value)->i32;
    pub fn snd_soc_dapm_new_widgets(card:*mut snd_soc_card)->i32;
    pub fn snd_soc_dapm_free(dapm:*mut snd_soc_dapm_context);
    pub fn snd_soc_dapm_sync(dapm:*mut snd_soc_dapm_context)->i32;
    pub fn snd_soc_dapm_stream_event(rtd:*mut snd_soc_pcm_runtime,stream:i32,event:i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
