#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int};
pub type u8 = ::core::ffi::c_uchar;
pub type u16 = ::core::ffi::c_ushort;
pub type u32 = ::core::ffi::c_uint;
pub type c_short = ::core::ffi::c_short;















	pub const HPI_FORMAT_MIXER_NATIVE: u32 = 0 as u32;


	pub const HPI_FORMAT_PCM8_UNSIGNED: u32 = 1 as u32;


	pub const HPI_FORMAT_PCM16_SIGNED: u32 = 2 as u32;


	pub const HPI_FORMAT_MPEG_L1: u32 = 3 as u32;


	pub const HPI_FORMAT_MPEG_L2: u32 = 4 as u32;


	pub const HPI_FORMAT_MPEG_L3: u32 = 5 as u32;


	pub const HPI_FORMAT_DOLBY_AC2: u32 = 6 as u32;


	pub const HPI_FORMAT_DOLBY_AC3: u32 = 7 as u32;


	pub const HPI_FORMAT_PCM16_BIGENDIAN: u32 = 8 as u32;


	pub const HPI_FORMAT_AA_TAGIT1_HITS: u32 = 9 as u32;


	pub const HPI_FORMAT_AA_TAGIT1_INSERTS: u32 = 10 as u32;


	pub const HPI_FORMAT_PCM32_SIGNED: u32 = 11 as u32;


	pub const HPI_FORMAT_RAW_BITSTREAM: u32 = 12 as u32;


	pub const HPI_FORMAT_AA_TAGIT1_HITS_EX1: u32 = 13 as u32;


	pub const HPI_FORMAT_PCM32_FLOAT: u32 = 14 as u32;


	pub const HPI_FORMAT_PCM24_SIGNED: u32 = 15 as u32;


	pub const HPI_FORMAT_OEM1: u32 = 16 as u32;


	pub const HPI_FORMAT_OEM2: u32 = 17 as u32;


	pub const HPI_FORMAT_UNDEFINED: u32 = 0xffff as u32





	
	pub const HPI_STATE_STOPPED: u32 = 1 as u32;

	
	pub const HPI_STATE_PLAYING: u32 = 2 as u32;

	
	pub const HPI_STATE_RECORDING: u32 = 3 as u32;

	
	pub const HPI_STATE_DRAINED: u32 = 4 as u32;

	
	pub const HPI_STATE_SINEGEN: u32 = 5 as u32;

	
	pub const HPI_STATE_WAIT: u32 = 6 as u32




	
	pub const HPI_SOURCENODE_NONE: u32 = 100 as u32;

	
	pub const HPI_SOURCENODE_OSTREAM: u32 = 101 as u32;

	
	pub const HPI_SOURCENODE_LINEIN: u32 = 102 as u32;

	pub const HPI_SOURCENODE_AESEBU_IN: u32 = 103 as u32;
	     
	pub const HPI_SOURCENODE_TUNER: u32 = 104 as u32;
	     
	pub const HPI_SOURCENODE_RF: u32 = 105 as u32;
	     
	pub const HPI_SOURCENODE_CLOCK_SOURCE: u32 = 106 as u32;
   
	pub const HPI_SOURCENODE_RAW_BITSTREAM: u32 = 107 as u32;
  
	pub const HPI_SOURCENODE_MICROPHONE: u32 = 108 as u32;
     
	
	pub const HPI_SOURCENODE_COBRANET: u32 = 109 as u32;

	pub const HPI_SOURCENODE_ANALOG: u32 = 110 as u32;
	     
	pub const HPI_SOURCENODE_ADAPTER: u32 = 111 as u32;
	     
	
	pub const HPI_SOURCENODE_RTP_DESTINATION: u32 = 112 as u32;

	pub const HPI_SOURCENODE_INTERNAL: u32 = 113 as u32;
	     
	pub const HPI_SOURCENODE_AVB: u32 = 114 as u32;
	     
	pub const HPI_SOURCENODE_BLULINK: u32 = 115 as u32;
	     
	
	pub const HPI_SOURCENODE_LAST_INDEX: u32 = 115	      as u32
		





	
	pub const HPI_DESTNODE_NONE: u32 = 200 as u32;

	
	pub const HPI_DESTNODE_ISTREAM: u32 = 201 as u32;

	pub const HPI_DESTNODE_LINEOUT: u32 = 202 as u32;
	     
	pub const HPI_DESTNODE_AESEBU_OUT: u32 = 203 as u32;
	     
	pub const HPI_DESTNODE_RF: u32 = 204 as u32;
		     
	pub const HPI_DESTNODE_SPEAKER: u32 = 205 as u32;
	     
	
	pub const HPI_DESTNODE_COBRANET: u32 = 206 as u32;

	pub const HPI_DESTNODE_ANALOG: u32 = 207 as u32;
	     
	
	pub const HPI_DESTNODE_RTP_SOURCE: u32 = 208 as u32;

	pub const HPI_DESTNODE_AVB: u32 = 209 as u32;
		     
	pub const HPI_DESTNODE_INTERNAL: u32 = 210 as u32;
	     
	pub const HPI_DESTNODE_BLULINK: u32 = 211 as u32;
	     
	
	pub const HPI_DESTNODE_LAST_INDEX: u32 = 211	      as u32
		





	pub const HPI_CONTROL_GENERIC: u32 = 0 as u32;
	
	pub const HPI_CONTROL_CONNECTION: u32 = 1 as u32;
 
	pub const HPI_CONTROL_VOLUME: u32 = 2 as u32;
	      
	pub const HPI_CONTROL_METER: u32 = 3 as u32;
	
	pub const HPI_CONTROL_MUTE: u32 = 4 as u32;
	
	pub const HPI_CONTROL_MULTIPLEXER: u32 = 5 as u32;
	

	pub const HPI_CONTROL_AESEBU_TRANSMITTER: u32 = 6 as u32;
 
	pub const HPI_CONTROL_AESEBUTX: u32 = 6 as u32;
	

	pub const HPI_CONTROL_AESEBU_RECEIVER: u32 = 7 as u32;
 
	pub const HPI_CONTROL_AESEBURX: u32 = 7 as u32;
	

	pub const HPI_CONTROL_LEVEL: u32 = 8 as u32;
 
	pub const HPI_CONTROL_TUNER: u32 = 9 as u32;
	

	pub const HPI_CONTROL_VOX: u32 = 11 as u32;
	



	pub const HPI_CONTROL_CHANNEL_MODE: u32 = 15 as u32;
	

	pub const HPI_CONTROL_BITSTREAM: u32 = 16 as u32;
	
	pub const HPI_CONTROL_SAMPLECLOCK: u32 = 17 as u32;
	
	pub const HPI_CONTROL_MICROPHONE: u32 = 18 as u32;
	
	pub const HPI_CONTROL_PARAMETRIC_EQ: u32 = 19 as u32;
	
	pub const HPI_CONTROL_EQUALIZER: u32 = 19 as u32;
	

	pub const HPI_CONTROL_COMPANDER: u32 = 20 as u32;
	
	pub const HPI_CONTROL_COBRANET: u32 = 21 as u32;
	
	pub const HPI_CONTROL_TONEDETECTOR: u32 = 22 as u32;
	
	pub const HPI_CONTROL_SILENCEDETECTOR: u32 = 23 as u32;
	
	pub const HPI_CONTROL_PAD: u32 = 24 as u32;
	
	pub const HPI_CONTROL_SRC: u32 = 25 as u32;
	
	pub const HPI_CONTROL_UNIVERSAL: u32 = 26 as u32;
	


	pub const HPI_CONTROL_LAST_INDEX: u32 = 26  as u32







	pub const HPI_ADAPTER_PROPERTY_ERRATA_1: u32 = 1 as u32;



	pub const HPI_ADAPTER_PROPERTY_GROUPING: u32 = 2 as u32;



	pub const HPI_ADAPTER_PROPERTY_ENABLE_SSX2: u32 = 3 as u32;



	pub const HPI_ADAPTER_PROPERTY_SSX2_SETTING: u32 = 4 as u32;



	pub const HPI_ADAPTER_PROPERTY_IRQ_RATE: u32 = 5 as u32;



	pub const HPI_ADAPTER_PROPERTY_READONLYBASE: u32 = 256 as u32;



	pub const HPI_ADAPTER_PROPERTY_LATENCY: u32 = 256 as u32;



	pub const HPI_ADAPTER_PROPERTY_GRANULARITY: u32 = 257 as u32;



	pub const HPI_ADAPTER_PROPERTY_CURCHANNELS: u32 = 258 as u32;



	pub const HPI_ADAPTER_PROPERTY_SOFTWARE_VERSION: u32 = 259 as u32;



	pub const HPI_ADAPTER_PROPERTY_MAC_ADDRESS_MSB: u32 = 260 as u32;



	pub const HPI_ADAPTER_PROPERTY_MAC_ADDRESS_LSB: u32 = 261 as u32;



	pub const HPI_ADAPTER_PROPERTY_EXTENDED_ADAPTER_TYPE: u32 = 262 as u32;



	pub const HPI_ADAPTER_PROPERTY_LOGTABLEN: u32 = 263 as u32;

	pub const HPI_ADAPTER_PROPERTY_LOGTABBEG: u32 = 264 as u32;



	pub const HPI_ADAPTER_PROPERTY_IP_ADDRESS: u32 = 265 as u32;



	pub const HPI_ADAPTER_PROPERTY_BUFFER_UPDATE_COUNT: u32 = 266 as u32;



	pub const HPI_ADAPTER_PROPERTY_INTERVAL: u32 = 267 as u32;


	pub const HPI_ADAPTER_PROPERTY_CAPS1: u32 = 268 as u32;


	pub const HPI_ADAPTER_PROPERTY_CAPS2: u32 = 269 as u32;



	pub const HPI_ADAPTER_PROPERTY_SYNC_HEADER_CONNECTIONS: u32 = 270 as u32;


	pub const HPI_ADAPTER_PROPERTY_SUPPORTS_SSX2: u32 = 271 as u32;


	pub const HPI_ADAPTER_PROPERTY_SUPPORTS_IRQ: u32 = 272 as u32;


	pub const HPI_ADAPTER_PROPERTY_SUPPORTS_FW_UPDATE: u32 = 273 as u32;


	pub const HPI_ADAPTER_PROPERTY_FIRMWARE_ID: u32 = 274 as u32




	
	pub const HPI_ADAPTER_MODE_SET: u32 = 0 as u32;

	
	pub const HPI_ADAPTER_MODE_QUERY: u32 = 1 as u32





	pub const HPI_ADAPTER_MODE_4OSTREAM: u32 = 1 as u32;



	pub const HPI_ADAPTER_MODE_6OSTREAM: u32 = 2 as u32;



	pub const HPI_ADAPTER_MODE_8OSTREAM: u32 = 3 as u32;



	pub const HPI_ADAPTER_MODE_16OSTREAM: u32 = 4 as u32;



	pub const HPI_ADAPTER_MODE_1OSTREAM: u32 = 5 as u32;



	pub const HPI_ADAPTER_MODE_1: u32 = 6 as u32;



	pub const HPI_ADAPTER_MODE_2: u32 = 7 as u32;



	pub const HPI_ADAPTER_MODE_3: u32 = 8 as u32;



	pub const HPI_ADAPTER_MODE_MULTICHANNEL: u32 = 9 as u32;



	pub const HPI_ADAPTER_MODE_12OSTREAM: u32 = 10 as u32;



	pub const HPI_ADAPTER_MODE_9OSTREAM: u32 = 11 as u32;



	pub const HPI_ADAPTER_MODE_MONO: u32 = 12 as u32;



	pub const HPI_ADAPTER_MODE_LOW_LATENCY: u32 = 13 as u32



pub const HPI_CAPABILITY_NONE: u32 = (0) as u32;
pub const HPI_CAPABILITY_MPEG_LAYER3: u32 = (1) as u32;


pub const HPI_CAPABILITY_MAX: u32 = 1 as u32;






	
	pub const HPI_MPEG_ANC_HASENERGY: u32 = 0 as u32;

	
	pub const HPI_MPEG_ANC_RAW: u32 = 1 as u32




	
	pub const HPI_MPEG_ANC_ALIGN_LEFT: u32 = 0 as u32;

	
	pub const HPI_MPEG_ANC_ALIGN_RIGHT: u32 = 1 as u32





	pub const HPI_MPEG_MODE_DEFAULT: u32 = 0 as u32;

	
	pub const HPI_MPEG_MODE_STEREO: u32 = 1 as u32;

	
	pub const HPI_MPEG_MODE_JOINTSTEREO: u32 = 2 as u32;

	
	pub const HPI_MPEG_MODE_DUALCHANNEL: u32 = 3 as u32




pub const HPI_MIXER_GET_CONTROL_MULTIPLE_CHANGED: u32 = (0) as u32;
pub const HPI_MIXER_GET_CONTROL_MULTIPLE_RESET: u32 = (1) as u32;





	pub const HPI_MIXER_STORE_SAVE: u32 = 1 as u32;


	pub const HPI_MIXER_STORE_RESTORE: u32 = 2 as u32;


	pub const HPI_MIXER_STORE_DELETE: u32 = 3 as u32;


	pub const HPI_MIXER_STORE_ENABLE: u32 = 4 as u32;


	pub const HPI_MIXER_STORE_DISABLE: u32 = 5 as u32;


	pub const HPI_MIXER_STORE_SAVE_SINGLE: u32 = 6 as u32








	pub const HPI_SWITCH_OFF: u32 = 0 as u32;
	
	pub const HPI_SWITCH_ON: u32 = 1	 as u32





pub const HPI_UNITS_PER_dB: u32 = 100 as u32;

pub const HPI_GAIN_OFF: u32 = (-100 * HPI_UNITS_PER_dB) as u32;


pub const HPI_BITMASK_ALL_CHANNELS: u32 = (0xFFFFFFFF) as u32;


pub const HPI_METER_MINIMUM: u32 = (-150 * HPI_UNITS_PER_dB) as u32;




	pub const HPI_VOLUME_AUTOFADE_LOG: u32 = 2 as u32;


	pub const HPI_VOLUME_AUTOFADE_LINEAR: u32 = 3 as u32





	pub const HPI_AESEBU_FORMAT_AESEBU: u32 = 1 as u32;


	pub const HPI_AESEBU_FORMAT_SPDIF: u32 = 2 as u32





	pub const HPI_AESEBU_ERROR_NOT_LOCKED: u32 = 0x01 as u32;


	pub const HPI_AESEBU_ERROR_POOR_QUALITY: u32 = 0x02 as u32;


	pub const HPI_AESEBU_ERROR_PARITY_ERROR: u32 = 0x04 as u32;


	pub const HPI_AESEBU_ERROR_BIPHASE_VIOLATION: u32 = 0x08 as u32;


	pub const HPI_AESEBU_ERROR_VALIDITY: u32 = 0x10 as u32;


	pub const HPI_AESEBU_ERROR_CRC: u32 = 0x20 as u32




pub const HPI_PAD_CHANNEL_NAME_LEN: u32 = 16 as u32;

pub const HPI_PAD_ARTIST_LEN: u32 = 64 as u32;

pub const HPI_PAD_TITLE_LEN: u32 = 64 as u32;

pub const HPI_PAD_COMMENT_LEN: u32 = 256 as u32;

pub const HPI_PAD_PROGRAM_TYPE_INVALID: u32 = 0xffff as u32;




	pub const HPI_RDS_DATATYPE_RDS: u32 = 0 as u32;
	
	pub const HPI_RDS_DATATYPE_RBDS: u32 = 1	 as u32




	pub const HPI_TUNER_BAND_AM: u32 = 1 as u32;
	 
	pub const HPI_TUNER_BAND_FM: u32 = 2 as u32;
	 
	pub const HPI_TUNER_BAND_TV_NTSC_M: u32 = 3 as u32;
	 
	pub const HPI_TUNER_BAND_TV: u32 = 3 as u32;
	
	pub const HPI_TUNER_BAND_FM_STEREO: u32 = 4 as u32;
	 
	pub const HPI_TUNER_BAND_AUX: u32 = 5 as u32;
	 
	pub const HPI_TUNER_BAND_TV_PAL_BG: u32 = 6 as u32;
	 
	pub const HPI_TUNER_BAND_TV_PAL_I: u32 = 7 as u32;
	 
	pub const HPI_TUNER_BAND_TV_PAL_DK: u32 = 8 as u32;
	 
	pub const HPI_TUNER_BAND_TV_SECAM_L: u32 = 9 as u32;
	 
	pub const HPI_TUNER_BAND_DAB: u32 = 10 as u32;

	pub const HPI_TUNER_BAND_LAST: u32 = 10  as u32




	pub const HPI_TUNER_MODE_RSS: u32 = 1 as u32;
	
	pub const HPI_TUNER_MODE_RDS: u32 = 2	 as u32





	pub const HPI_TUNER_MODE_RSS_DISABLE: u32 = 0 as u32;
	
	pub const HPI_TUNER_MODE_RSS_ENABLE: u32 = 1 as u32;
	


	pub const HPI_TUNER_MODE_RDS_DISABLE: u32 = 0 as u32;
	
	pub const HPI_TUNER_MODE_RDS_RDS: u32 = 1 as u32;
  
	pub const HPI_TUNER_MODE_RDS_RBDS: u32 = 2  as u32




	pub const HPI_TUNER_VIDEO_COLOR_PRESENT: u32 = 0x0001 as u32;
	
	pub const HPI_TUNER_VIDEO_IS_60HZ: u32 = 0x0020 as u32;
 
	pub const HPI_TUNER_VIDEO_HORZ_SYNC_MISSING: u32 = 0x0040 as u32;
 
	pub const HPI_TUNER_VIDEO_STATUS_VALID: u32 = 0x0100 as u32;
 
	pub const HPI_TUNER_DIGITAL: u32 = 0x0200 as u32;
 
	pub const HPI_TUNER_MULTIPROGRAM: u32 = 0x0400 as u32;
 
	pub const HPI_TUNER_PLL_LOCKED: u32 = 0x1000 as u32;
 
	pub const HPI_TUNER_FM_STEREO: u32 = 0x2000  as u32





	pub const HPI_CHANNEL_MODE_NORMAL: u32 = 1 as u32;


	pub const HPI_CHANNEL_MODE_SWAP: u32 = 2 as u32;


	pub const HPI_CHANNEL_MODE_LEFT_TO_STEREO: u32 = 3 as u32;


	pub const HPI_CHANNEL_MODE_RIGHT_TO_STEREO: u32 = 4 as u32;


	pub const HPI_CHANNEL_MODE_STEREO_TO_LEFT: u32 = 5 as u32;


	pub const HPI_CHANNEL_MODE_STEREO_TO_RIGHT: u32 = 6 as u32;

	pub const HPI_CHANNEL_MODE_LAST: u32 = 6 as u32





	pub const HPI_SAMPLECLOCK_SOURCE_LOCAL: u32 = 1 as u32;


	pub const HPI_SAMPLECLOCK_SOURCE_AESEBU_SYNC: u32 = 2 as u32;


	pub const HPI_SAMPLECLOCK_SOURCE_WORD: u32 = 3 as u32;


	pub const HPI_SAMPLECLOCK_SOURCE_WORD_HEADER: u32 = 4 as u32;


	pub const HPI_SAMPLECLOCK_SOURCE_SMPTE: u32 = 5 as u32;


	pub const HPI_SAMPLECLOCK_SOURCE_AESEBU_INPUT: u32 = 6 as u32;


	pub const HPI_SAMPLECLOCK_SOURCE_NETWORK: u32 = 8 as u32;


	pub const HPI_SAMPLECLOCK_SOURCE_PREV_MODULE: u32 = 10 as u32;


	pub const HPI_SAMPLECLOCK_SOURCE_BLULINK: u32 = 11 as u32;


	pub const HPI_SAMPLECLOCK_SOURCE_LAST: u32 = 11 as u32




	pub const HPI_FILTER_TYPE_BYPASS: u32 = 0 as u32;
	

	pub const HPI_FILTER_TYPE_LOWSHELF: u32 = 1 as u32;
	
	pub const HPI_FILTER_TYPE_HIGHSHELF: u32 = 2 as u32;
	
	pub const HPI_FILTER_TYPE_EQ_BAND: u32 = 3 as u32;
	

	pub const HPI_FILTER_TYPE_LOWPASS: u32 = 4 as u32;
	
	pub const HPI_FILTER_TYPE_HIGHPASS: u32 = 5 as u32;
	
	pub const HPI_FILTER_TYPE_BANDPASS: u32 = 6 as u32;
	
	pub const HPI_FILTER_TYPE_BANDSTOP: u32 = 7	 as u32




	pub const HPI_ASYNC_EVENT_GPIO: u32 = 1 as u32;
	
	pub const HPI_ASYNC_EVENT_SILENCE: u32 = 2 as u32;
	
	pub const HPI_ASYNC_EVENT_TONE: u32 = 3	 as u32




	
	pub const HPI_ERROR_INVALID_TYPE: u32 = 100 as u32;

	
	pub const HPI_ERROR_INVALID_OBJ: u32 = 101 as u32;

	
	pub const HPI_ERROR_INVALID_FUNC: u32 = 102 as u32;

	
	pub const HPI_ERROR_INVALID_OBJ_INDEX: u32 = 103 as u32;

	
	pub const HPI_ERROR_OBJ_NOT_OPEN: u32 = 104 as u32;

	
	pub const HPI_ERROR_OBJ_ALREADY_OPEN: u32 = 105 as u32;

	
	pub const HPI_ERROR_INVALID_RESOURCE: u32 = 106 as u32;

	
	
	pub const HPI_ERROR_INVALID_RESPONSE: u32 = 108 as u32;

	
	pub const HPI_ERROR_PROCESSING_MESSAGE: u32 = 109 as u32;

	
	pub const HPI_ERROR_NETWORK_TIMEOUT: u32 = 110 as u32;

	
	pub const HPI_ERROR_INVALID_HANDLE: u32 = 111 as u32;

	
	pub const HPI_ERROR_UNIMPLEMENTED: u32 = 112 as u32;

	
	pub const HPI_ERROR_NETWORK_TOO_MANY_CLIENTS: u32 = 113 as u32;

	
	pub const HPI_ERROR_RESPONSE_BUFFER_TOO_SMALL: u32 = 114 as u32;

	
	pub const HPI_ERROR_RESPONSE_MISMATCH: u32 = 115 as u32;

	
	pub const HPI_ERROR_CONTROL_CACHING: u32 = 116 as u32;

	
	pub const HPI_ERROR_MESSAGE_BUFFER_TOO_SMALL: u32 = 117 as u32;


	
	
	pub const HPI_ERROR_BAD_ADAPTER: u32 = 201 as u32;

	
	pub const HPI_ERROR_BAD_ADAPTER_NUMBER: u32 = 202 as u32;

	
	pub const HPI_ERROR_DUPLICATE_ADAPTER_NUMBER: u32 = 203 as u32;

	
	pub const HPI_ERROR_DSP_BOOTLOAD: u32 = 204 as u32;

	
	pub const HPI_ERROR_DSP_FILE_NOT_FOUND: u32 = 206 as u32;

	
	pub const HPI_ERROR_DSP_HARDWARE: u32 = 207 as u32;

	
	pub const HPI_ERROR_MEMORY_ALLOC: u32 = 208 as u32;

	
	pub const HPI_ERROR_PLD_LOAD: u32 = 209 as u32;

	
	pub const HPI_ERROR_DSP_FILE_FORMAT: u32 = 210 as u32;


	
	pub const HPI_ERROR_DSP_FILE_ACCESS_DENIED: u32 = 211 as u32;

	
	pub const HPI_ERROR_DSP_FILE_NO_HEADER: u32 = 212 as u32;

	
	
	pub const HPI_ERROR_DSP_SECTION_NOT_FOUND: u32 = 214 as u32;

	
	pub const HPI_ERROR_DSP_FILE_OTHER_ERROR: u32 = 215 as u32;

	
	pub const HPI_ERROR_DSP_FILE_SHARING_VIOLATION: u32 = 216 as u32;

	
	pub const HPI_ERROR_DSP_FILE_NULL_HEADER: u32 = 217 as u32;


	

	
	pub const HPI_ERROR_BAD_CHECKSUM: u32 = 221 as u32;

	pub const HPI_ERROR_BAD_SEQUENCE: u32 = 222 as u32;

	pub const HPI_ERROR_FLASH_ERASE: u32 = 223 as u32;

	pub const HPI_ERROR_FLASH_PROGRAM: u32 = 224 as u32;

	pub const HPI_ERROR_FLASH_VERIFY: u32 = 225 as u32;

	pub const HPI_ERROR_FLASH_TYPE: u32 = 226 as u32;

	pub const HPI_ERROR_FLASH_START: u32 = 227 as u32;

	pub const HPI_ERROR_FLASH_READ: u32 = 228 as u32;

	pub const HPI_ERROR_FLASH_READ_NO_FILE: u32 = 229 as u32;

	pub const HPI_ERROR_FLASH_SIZE: u32 = 230 as u32;


	
	pub const HPI_ERROR_RESERVED_1: u32 = 290 as u32;


	
	
	pub const HPI_ERROR_INVALID_FORMAT: u32 = 301 as u32;

	
	pub const HPI_ERROR_INVALID_SAMPLERATE: u32 = 302 as u32;

	
	pub const HPI_ERROR_INVALID_CHANNELS: u32 = 303 as u32;

	
	pub const HPI_ERROR_INVALID_BITRATE: u32 = 304 as u32;

	
	pub const HPI_ERROR_INVALID_DATASIZE: u32 = 305 as u32;

	
	
	
	pub const HPI_ERROR_INVALID_DATA_POINTER: u32 = 308 as u32;

	
	pub const HPI_ERROR_INVALID_PACKET_ORDER: u32 = 309 as u32;


	
	pub const HPI_ERROR_INVALID_OPERATION: u32 = 310 as u32;


	
	pub const HPI_ERROR_INCOMPATIBLE_SAMPLERATE: u32 = 311 as u32;

	
	pub const HPI_ERROR_BAD_ADAPTER_MODE: u32 = 312 as u32;


	
	pub const HPI_ERROR_TOO_MANY_CAPABILITY_CHANGE_ATTEMPTS: u32 = 313 as u32;

	
	pub const HPI_ERROR_NO_INTERADAPTER_GROUPS: u32 = 314 as u32;

	
	pub const HPI_ERROR_NO_INTERDSP_GROUPS: u32 = 315 as u32;

	
	pub const HPI_ERROR_WAIT_CANCELLED: u32 = 316 as u32;

	
	pub const HPI_ERROR_INVALID_STRING: u32 = 317 as u32;


	
	pub const HPI_ERROR_INVALID_NODE: u32 = 400 as u32;

	
	pub const HPI_ERROR_INVALID_CONTROL: u32 = 401 as u32;

	
	pub const HPI_ERROR_INVALID_CONTROL_VALUE: u32 = 402 as u32;

	
	pub const HPI_ERROR_INVALID_CONTROL_ATTRIBUTE: u32 = 403 as u32;

	
	pub const HPI_ERROR_CONTROL_DISABLED: u32 = 404 as u32;

	
	pub const HPI_ERROR_CONTROL_I2C_MISSING_ACK: u32 = 405 as u32;

	pub const HPI_ERROR_I2C_MISSING_ACK: u32 = 405 as u32;

	
	pub const HPI_ERROR_CONTROL_NOT_READY: u32 = 407 as u32;


	
	pub const HPI_ERROR_NVMEM_BUSY: u32 = 450 as u32;

	pub const HPI_ERROR_NVMEM_FULL: u32 = 451 as u32;

	pub const HPI_ERROR_NVMEM_FAIL: u32 = 452 as u32;


	
	pub const HPI_ERROR_I2C_BAD_ADR: u32 = 460 as u32;


	
	pub const HPI_ERROR_ENTITY_TYPE_MISMATCH: u32 = 470 as u32;

	
	pub const HPI_ERROR_ENTITY_ITEM_COUNT: u32 = 471 as u32;

	
	pub const HPI_ERROR_ENTITY_TYPE_INVALID: u32 = 472 as u32;

	
	pub const HPI_ERROR_ENTITY_ROLE_INVALID: u32 = 473 as u32;

	
	pub const HPI_ERROR_ENTITY_SIZE_MISMATCH: u32 = 474 as u32;


	

	
	pub const HPI_ERROR_CUSTOM: u32 = 600 as u32;


	
	pub const HPI_ERROR_MUTEX_TIMEOUT: u32 = 700 as u32;


	
	pub const HPI_ERROR_BACKEND_BASE: u32 = 900 as u32;


	
	pub const HPI_ERROR_DSP_COMMUNICATION: u32 = 900 as u32
		




pub const HPI_MAX_ADAPTERS: u32 = 20 as u32;

pub const HPI_MAX_STREAMS: u32 = 16 as u32;
pub const HPI_MAX_CHANNELS: u32 = 2 as u32;	
pub const HPI_MAX_NODES: u32 = 8 as u32;	
pub const HPI_MAX_CONTROLS: u32 = 4 as u32;	

pub const HPI_MAX_ANC_BYTES_PER_FRAME: u32 = (64) as u32;
pub const HPI_STRING_LEN: u32 = 16 as u32;


pub const HPI_MIN_NETWORK_ADAPTER_IDX: u32 = 100 as u32;


pub const HPI_OSTREAM_VELOCITY_UNITS: u32 = 4096 as u32;

pub const HPI_OSTREAM_TIMESCALE_UNITS: u32 = 10000 as u32;

pub const HPI_OSTREAM_TIMESCALE_PASSTHROUGH: u32 = 99999 as u32;






// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
