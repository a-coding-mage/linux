/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of cistpl.h. */

pub type cisdata_t = u8;

macro_rules! c { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
c!(CISTPL_NULL=0x00,CISTPL_DEVICE=0x01,CISTPL_LONGLINK_CB=0x02,CISTPL_INDIRECT=0x03,CISTPL_CONFIG_CB=0x04,CISTPL_CFTABLE_ENTRY_CB=0x05,CISTPL_LONGLINK_MFC=0x06,CISTPL_BAR=0x07,CISTPL_PWR_MGMNT=0x08,CISTPL_EXTDEVICE=0x09,CISTPL_CHECKSUM=0x10,CISTPL_LONGLINK_A=0x11,CISTPL_LONGLINK_C=0x12,CISTPL_LINKTARGET=0x13,CISTPL_NO_LINK=0x14,CISTPL_VERS_1=0x15,CISTPL_ALTSTR=0x16,CISTPL_DEVICE_A=0x17,CISTPL_JEDEC_C=0x18,CISTPL_JEDEC_A=0x19,CISTPL_CONFIG=0x1a,CISTPL_CFTABLE_ENTRY=0x1b,CISTPL_DEVICE_OC=0x1c,CISTPL_DEVICE_OA=0x1d,CISTPL_DEVICE_GEO=0x1e,CISTPL_DEVICE_GEO_A=0x1f,CISTPL_MANFID=0x20,CISTPL_FUNCID=0x21,CISTPL_FUNCE=0x22,CISTPL_SWIL=0x23,CISTPL_END=0xff,CISTPL_VERS_2=0x40,CISTPL_FORMAT=0x41,CISTPL_GEOMETRY=0x42,CISTPL_BYTEORDER=0x43,CISTPL_DATE=0x44,CISTPL_BATTERY=0x45,CISTPL_FORMAT_A=0x47,CISTPL_ORG=0x46,CISTPL_SPCL=0x90);

#[repr(C)] pub struct cistpl_longlink_t { pub addr:u32 }
#[repr(C)] pub struct cistpl_checksum_t { pub addr:u16,pub len:u16,pub sum:u8 }
pub const CISTPL_MAX_FUNCTIONS:usize=8; pub const CISTPL_MFC_ATTR:u32=0; pub const CISTPL_MFC_COMMON:u32=1;
#[repr(C)] pub struct cistpl_longlink_mfc_t { pub nfn:u8, pub fn_: [CistplLonglinkFn;8] }
#[repr(C)] pub struct CistplLonglinkFn { pub space:u8,pub addr:u32 }
pub const CISTPL_MAX_ALTSTR_STRINGS:usize=4;
#[repr(C)] pub struct cistpl_altstr_t { pub ns:u8,pub ofs:[u8;4],pub str_:[i8;254] }

c!(CISTPL_DTYPE_NULL=0,CISTPL_DTYPE_ROM=1,CISTPL_DTYPE_OTPROM=2,CISTPL_DTYPE_EPROM=3,CISTPL_DTYPE_EEPROM=4,CISTPL_DTYPE_FLASH=5,CISTPL_DTYPE_SRAM=6,CISTPL_DTYPE_DRAM=7,CISTPL_DTYPE_FUNCSPEC=0x0d,CISTPL_DTYPE_EXTEND=0x0e);
pub const CISTPL_MAX_DEVICES:usize=4;
#[repr(C)] pub struct CistplDevice { pub type_:u8,pub wp:u8,pub speed:u32,pub size:u32 }
#[repr(C)] pub struct cistpl_device_t { pub ndev:u8,pub dev:[CistplDevice;4] }
c!(CISTPL_DEVICE_MWAIT=1,CISTPL_DEVICE_3VCC=2);
#[repr(C)] pub struct cistpl_device_o_t { pub flags:u8,pub device:cistpl_device_t }
pub const CISTPL_VERS_1_MAX_PROD_STRINGS:usize=4;
#[repr(C)] pub struct cistpl_vers_1_t { pub major:u8,pub minor:u8,pub ns:u8,pub ofs:[u8;4],pub str_:[i8;254] }
#[repr(C)] pub struct CistplJedecId { pub mfr:u8,pub info:u8 }
#[repr(C)] pub struct cistpl_jedec_t { pub nid:u8,pub id:[CistplJedecId;4] }
#[repr(C)] pub struct cistpl_manfid_t { pub manf:u16,pub card:u16 }
c!(CISTPL_FUNCID_MULTI=0,CISTPL_FUNCID_MEMORY=1,CISTPL_FUNCID_SERIAL=2,CISTPL_FUNCID_PARALLEL=3,CISTPL_FUNCID_FIXED=4,CISTPL_FUNCID_VIDEO=5,CISTPL_FUNCID_NETWORK=6,CISTPL_FUNCID_AIMS=7,CISTPL_FUNCID_SCSI=8,CISTPL_SYSINIT_POST=1,CISTPL_SYSINIT_ROM=2);
#[repr(C)] pub struct cistpl_funcid_t { pub func:u8,pub sysinit:u8 }
#[repr(C)] pub struct cistpl_funce_t { pub type_:u8,pub data:[u8;0] }

c!(CISTPL_FUNCE_SERIAL_IF=0,CISTPL_FUNCE_SERIAL_CAP=1,CISTPL_FUNCE_SERIAL_SERV_DATA=2,CISTPL_FUNCE_SERIAL_SERV_FAX=3,CISTPL_FUNCE_SERIAL_SERV_VOICE=4,CISTPL_FUNCE_SERIAL_CAP_DATA=5,CISTPL_FUNCE_SERIAL_CAP_FAX=6,CISTPL_FUNCE_SERIAL_CAP_VOICE=7,CISTPL_FUNCE_SERIAL_IF_DATA=8,CISTPL_FUNCE_SERIAL_IF_FAX=9,CISTPL_FUNCE_SERIAL_IF_VOICE=0xa,CISTPL_SERIAL_UART_8250=0,CISTPL_SERIAL_UART_16450=1,CISTPL_SERIAL_UART_16550=2,CISTPL_SERIAL_UART_8251=3,CISTPL_SERIAL_UART_8530=4,CISTPL_SERIAL_UART_85230=5,CISTPL_SERIAL_UART_SPACE=1,CISTPL_SERIAL_UART_MARK=2,CISTPL_SERIAL_UART_ODD=4,CISTPL_SERIAL_UART_EVEN=8,CISTPL_SERIAL_UART_5BIT=1,CISTPL_SERIAL_UART_6BIT=2,CISTPL_SERIAL_UART_7BIT=4,CISTPL_SERIAL_UART_8BIT=8,CISTPL_SERIAL_UART_1STOP=0x10,CISTPL_SERIAL_UART_MSTOP=0x20,CISTPL_SERIAL_UART_2STOP=0x40);
#[repr(C)] pub struct cistpl_serial_t { pub uart_type:u8,pub uart_cap_0:u8,pub uart_cap_1:u8 }
#[repr(C)] pub struct cistpl_modem_cap_t { pub flow:u8,pub cmd_buf:u8,pub rcv_buf_0:u8,pub rcv_buf_1:u8,pub rcv_buf_2:u8,pub xmit_buf_0:u8,pub xmit_buf_1:u8,pub xmit_buf_2:u8 }
c!(CISTPL_SERIAL_MOD_103=1,CISTPL_SERIAL_MOD_V21=2,CISTPL_SERIAL_MOD_V23=4,CISTPL_SERIAL_MOD_V22=8,CISTPL_SERIAL_MOD_212A=0x10,CISTPL_SERIAL_MOD_V22BIS=0x20,CISTPL_SERIAL_MOD_V26=0x40,CISTPL_SERIAL_MOD_V26BIS=0x80,CISTPL_SERIAL_MOD_V27BIS=1,CISTPL_SERIAL_MOD_V29=2,CISTPL_SERIAL_MOD_V32=4,CISTPL_SERIAL_MOD_V32BIS=8,CISTPL_SERIAL_MOD_V34=0x10,CISTPL_SERIAL_ERR_MNP2_4=1,CISTPL_SERIAL_ERR_V42_LAPM=2,CISTPL_SERIAL_CMPR_V42BIS=1,CISTPL_SERIAL_CMPR_MNP5=2,CISTPL_SERIAL_CMD_AT1=1,CISTPL_SERIAL_CMD_AT2=2,CISTPL_SERIAL_CMD_AT3=4,CISTPL_SERIAL_CMD_MNP_AT=8,CISTPL_SERIAL_CMD_V25BIS=0x10,CISTPL_SERIAL_CMD_V25A=0x20,CISTPL_SERIAL_CMD_DMCL=0x40);
#[repr(C)] pub struct cistpl_data_serv_t { pub max_data_0:u8,pub max_data_1:u8,pub modulation_0:u8,pub modulation_1:u8,pub error_control:u8,pub compression:u8,pub cmd_protocol:u8,pub escape:u8,pub encrypt:u8,pub misc_features:u8,pub ccitt_code:[u8;0] }
#[repr(C)] pub struct cistpl_fax_serv_t { pub max_data_0:u8,pub max_data_1:u8,pub modulation:u8,pub encrypt:u8,pub features_0:u8,pub features_1:u8,pub ccitt_code:[u8;0] }
#[repr(C)] pub struct cistpl_voice_serv_t { pub max_data_0:u8,pub max_data_1:u8 }

c!(CISTPL_FUNCE_LAN_TECH=1,CISTPL_FUNCE_LAN_SPEED=2,CISTPL_FUNCE_LAN_MEDIA=3,CISTPL_FUNCE_LAN_NODE_ID=4,CISTPL_FUNCE_LAN_CONNECTOR=5,CISTPL_LAN_TECH_ARCNET=1,CISTPL_LAN_TECH_ETHERNET=2,CISTPL_LAN_TECH_TOKENRING=3,CISTPL_LAN_TECH_LOCALTALK=4,CISTPL_LAN_TECH_FDDI=5,CISTPL_LAN_TECH_ATM=6,CISTPL_LAN_TECH_WIRELESS=7);
#[repr(C)] pub struct cistpl_lan_tech_t { pub tech:u8 } #[repr(C)] pub struct cistpl_lan_speed_t { pub speed:u32 }
c!(CISTPL_LAN_MEDIA_UTP=1,CISTPL_LAN_MEDIA_STP=2,CISTPL_LAN_MEDIA_THIN_COAX=3,CISTPL_LAN_MEDIA_THICK_COAX=4,CISTPL_LAN_MEDIA_FIBER=5,CISTPL_LAN_MEDIA_900MHZ=6,CISTPL_LAN_MEDIA_2GHZ=7,CISTPL_LAN_MEDIA_5GHZ=8,CISTPL_LAN_MEDIA_DIFF_IR=9,CISTPL_LAN_MEDIA_PTP_IR=0xa);
#[repr(C)] pub struct cistpl_lan_media_t { pub media:u8 } #[repr(C)] pub struct cistpl_lan_node_id_t { pub nb:u8,pub id:[u8;16] } #[repr(C)] pub struct cistpl_lan_connector_t { pub code:u8 }
c!(CISTPL_IDE_INTERFACE=1,CISTPL_IDE_SILICON=4,CISTPL_IDE_UNIQUE=8,CISTPL_IDE_DUAL=0x10,CISTPL_IDE_HAS_SLEEP=1,CISTPL_IDE_HAS_STANDBY=2,CISTPL_IDE_HAS_IDLE=4,CISTPL_IDE_LOW_POWER=8,CISTPL_IDE_REG_INHIBIT=0x10,CISTPL_IDE_HAS_INDEX=0x20,CISTPL_IDE_IOIS16=0x40,CISTPL_FUNCE_IDE_IFACE=1,CISTPL_FUNCE_IDE_MASTER=2,CISTPL_FUNCE_IDE_SLAVE=3);
#[repr(C)] pub struct cistpl_ide_interface_t { pub interface:u8 } #[repr(C)] pub struct cistpl_ide_feature_t { pub feature1:u8,pub feature2:u8 }
c!(CISTPL_BAR_SPACE=7,CISTPL_BAR_SPACE_IO=0x10,CISTPL_BAR_PREFETCH=0x20,CISTPL_BAR_CACHEABLE=0x40,CISTPL_BAR_1MEG_MAP=0x80);
#[repr(C)] pub struct cistpl_bar_t { pub attr:u8,pub size:u32 } #[repr(C)] pub struct cistpl_config_t { pub last_idx:u8,pub base:u32,pub rmask:[u32;4],pub subtuples:u8 }
c!(CISTPL_POWER_VNOM=0,CISTPL_POWER_VMIN=1,CISTPL_POWER_VMAX=2,CISTPL_POWER_ISTATIC=3,CISTPL_POWER_IAVG=4,CISTPL_POWER_IPEAK=5,CISTPL_POWER_IDOWN=6,CISTPL_POWER_HIGHZ_OK=1,CISTPL_POWER_HIGHZ_REQ=2);
#[repr(C)] pub struct cistpl_power_t { pub present:u8,pub flags:u8,pub param:[u32;7] }
#[repr(C)] pub struct cistpl_timing_t { pub wait:u32,pub waitscale:u32,pub ready:u32,pub rdyscale:u32,pub reserved:u32,pub rsvscale:u32 }
c!(CISTPL_IO_LINES_MASK=0x1f,CISTPL_IO_8BIT=0x20,CISTPL_IO_16BIT=0x40,CISTPL_IO_RANGE=0x80);
pub const CISTPL_IO_MAX_WIN:usize=16; #[repr(C)] pub struct CistplIoWin { pub base:u32,pub len:u32 }
#[repr(C)] pub struct cistpl_io_t { pub flags:u8,pub nwin:u8,pub win:[CistplIoWin;16] } #[repr(C)] pub struct cistpl_irq_t { pub IRQInfo1:u32,pub IRQInfo2:u32 }
pub const CISTPL_MEM_MAX_WIN:usize=8; #[repr(C)] pub struct CistplMemWin { pub len:u32,pub card_addr:u32,pub host_addr:u32 }
#[repr(C)] pub struct cistpl_mem_t { pub flags:u8,pub nwin:u8,pub win:[CistplMemWin;8] }
c!(CISTPL_CFTABLE_DEFAULT=1,CISTPL_CFTABLE_BVDS=2,CISTPL_CFTABLE_WP=4,CISTPL_CFTABLE_RDYBSY=8,CISTPL_CFTABLE_MWAIT=0x10,CISTPL_CFTABLE_AUDIO=0x800,CISTPL_CFTABLE_READONLY=0x1000,CISTPL_CFTABLE_PWRDOWN=0x2000);
#[repr(C)] pub struct cistpl_cftable_entry_t { pub index:u8,pub flags:u16,pub interface:u8,pub vcc:cistpl_power_t,pub vpp1:cistpl_power_t,pub vpp2:cistpl_power_t,pub timing:cistpl_timing_t,pub io:cistpl_io_t,pub irq:cistpl_irq_t,pub mem:cistpl_mem_t,pub subtuples:u8 }
c!(CISTPL_CFTABLE_MASTER=0x100,CISTPL_CFTABLE_INVALIDATE=0x200,CISTPL_CFTABLE_VGA_PALETTE=0x400,CISTPL_CFTABLE_PARITY=0x800,CISTPL_CFTABLE_WAIT=0x1000,CISTPL_CFTABLE_SERR=0x2000,CISTPL_CFTABLE_FAST_BACK=0x4000,CISTPL_CFTABLE_BINARY_AUDIO=0x10000,CISTPL_CFTABLE_PWM_AUDIO=0x20000);
#[repr(C)] pub struct cistpl_cftable_entry_cb_t { pub index:u8,pub flags:u32,pub vcc:cistpl_power_t,pub vpp1:cistpl_power_t,pub vpp2:cistpl_power_t,pub io:u8,pub irq:u8,pub mem:u8,pub subtuples:u8 }
#[repr(C)] pub struct CistplGeo { pub buswidth:u8,pub erase_block:u32,pub read_block:u32,pub write_block:u32,pub partition:u32,pub interleave:u32 }
#[repr(C)] pub struct cistpl_device_geo_t { pub ngeo:u8,pub geo:[CistplGeo;4] }
#[repr(C)] pub struct cistpl_vers_2_t { pub vers:u8,pub comply:u8,pub dindex:u16,pub vspec8:u8,pub vspec9:u8,pub nhdr:u8,pub vendor:u8,pub info:u8,pub str_:[i8;244] }
#[repr(C)] pub struct cistpl_org_t { pub data_org:u8,pub desc:[i8;30] } c!(CISTPL_ORG_FS=0,CISTPL_ORG_APPSPEC=1,CISTPL_ORG_XIP=2);
#[repr(C)] pub struct cistpl_format_t { pub type_:u8,pub edc:u8,pub offset:u32,pub length:u32 } c!(CISTPL_FORMAT_DISK=0,CISTPL_FORMAT_MEM=1,CISTPL_EDC_NONE=0,CISTPL_EDC_CKSUM=1,CISTPL_EDC_CRC=2,CISTPL_EDC_PCC=3);
#[repr(C)] pub union cisparse_t { pub device:cistpl_device_t,pub checksum:cistpl_checksum_t,pub longlink:cistpl_longlink_t,pub longlink_mfc:cistpl_longlink_mfc_t,pub version_1:cistpl_vers_1_t,pub altstr:cistpl_altstr_t,pub jedec:cistpl_jedec_t,pub manfid:cistpl_manfid_t,pub funcid:cistpl_funcid_t,pub funce:cistpl_funce_t,pub bar:cistpl_bar_t,pub config:cistpl_config_t,pub cftable_entry:cistpl_cftable_entry_t,pub cftable_entry_cb:cistpl_cftable_entry_cb_t,pub device_geo:cistpl_device_geo_t,pub vers_2:cistpl_vers_2_t,pub org:cistpl_org_t,pub format:cistpl_format_t }
#[repr(C)] pub struct tuple_t { pub Attributes:u32,pub DesiredTuple:cisdata_t,pub Flags:u32,pub LinkOffset:u32,pub CISOffset:u32,pub TupleCode:cisdata_t,pub TupleLink:cisdata_t,pub TupleOffset:cisdata_t,pub TupleDataMax:cisdata_t,pub TupleDataLen:cisdata_t,pub TupleData:*mut cisdata_t }
pub const RETURN_FIRST_TUPLE:u32=0xff; pub const TUPLE_RETURN_LINK:u32=1; pub const TUPLE_RETURN_COMMON:u32=2; pub const CISTPL_MAX_CIS_SIZE:u32=0x200;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
