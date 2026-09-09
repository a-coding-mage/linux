/* SPDX-License-Identifier: GPL-2.0 */
// Translated from tpm_command.h. The crypto/sha2.h dependency is supplied externally.

pub const TPM_MAX_ORDINAL: u32 = 243;

pub const TPM_TAG_RQU_COMMAND: u32 = 193;
pub const TPM_TAG_RQU_AUTH1_COMMAND: u32 = 194;
pub const TPM_TAG_RQU_AUTH2_COMMAND: u32 = 195;
pub const TPM_TAG_RSP_COMMAND: u32 = 196;
pub const TPM_TAG_RSP_AUTH1_COMMAND: u32 = 197;
pub const TPM_TAG_RSP_AUTH2_COMMAND: u32 = 198;

pub const TPM_ORD_CONTINUE_SELFTEST: u32 = 83;
pub const TPM_ORD_GET_CAP: u32 = 101;
pub const TPM_ORD_GET_RANDOM: u32 = 70;
pub const TPM_ORD_PCR_EXTEND: u32 = 20;
pub const TPM_ORD_PCR_READ: u32 = 21;
pub const TPM_ORD_OSAP: u32 = 11;
pub const TPM_ORD_OIAP: u32 = 10;
pub const TPM_ORD_SAVESTATE: u32 = 152;
pub const TPM_ORD_SEAL: u32 = 23;
pub const TPM_ORD_STARTUP: u32 = 153;
pub const TPM_ORD_UNSEAL: u32 = 24;

pub const TPM_CAP_FLAG: u32 = 4;
pub const TPM_CAP_PROP: u32 = 5;
pub const TPM_CAP_VERSION_1_1: u32 = 0x06;
pub const TPM_CAP_VERSION_1_2: u32 = 0x1A;
pub const TPM_CAP_PROP_PCR: u32 = 0x101;
pub const TPM_CAP_PROP_MANUFACTURER: u32 = 0x103;
pub const TPM_CAP_FLAG_PERM: u32 = 0x108;
pub const TPM_CAP_FLAG_VOL: u32 = 0x109;
pub const TPM_CAP_PROP_OWNER: u32 = 0x111;
pub const TPM_CAP_PROP_TIS_TIMEOUT: u32 = 0x115;
pub const TPM_CAP_PROP_TIS_DURATION: u32 = 0x120;

pub const TPM_BASE_MASK: u32 = 0;
pub const TPM_NON_FATAL_MASK: u32 = 0x00000800;
pub const TPM_SUCCESS: u32 = TPM_BASE_MASK + 0;
pub const TPM_ERR_DEACTIVATED: u32 = TPM_BASE_MASK + 6;
pub const TPM_ERR_DISABLED: u32 = TPM_BASE_MASK + 7;
pub const TPM_ERR_FAIL: u32 = TPM_BASE_MASK + 9;
pub const TPM_ERR_FAILEDSELFTEST: u32 = TPM_BASE_MASK + 28;
pub const TPM_ERR_INVALID_POSTINIT: u32 = TPM_BASE_MASK + 38;
pub const TPM_ERR_INVALID_FAMILY: u32 = TPM_BASE_MASK + 55;
pub const TPM_WARN_RETRY: u32 = TPM_BASE_MASK + TPM_NON_FATAL_MASK;
pub const TPM_WARN_DOING_SELFTEST: u32 = TPM_BASE_MASK + TPM_NON_FATAL_MASK + 2;

#[repr(C, packed)]
pub struct stclear_flags_t { pub tag: u16, pub deactivated: u8, pub disableForceClear: u8, pub physicalPresence: u8, pub physicalPresenceLock: u8, pub bGlobalLock: u8 }
#[repr(C, packed)]
pub struct tpm1_version { pub major: u8, pub minor: u8, pub rev_major: u8, pub rev_minor: u8 }
#[repr(C, packed)]
pub struct tpm1_version2 { pub tag: u16, pub version: tpm1_version }
#[repr(C, packed)]
pub struct timeout_t { pub a: u32, pub b: u32, pub c: u32, pub d: u32 }
#[repr(C, packed)]
pub struct duration_t { pub tpm_short: u32, pub tpm_medium: u32, pub tpm_long: u32 }
#[repr(C, packed)]
pub struct permanent_flags_t {
    pub tag: u16, pub disable: u8, pub ownership: u8, pub deactivated: u8, pub readPubek: u8,
    pub disableOwnerClear: u8, pub allowMaintenance: u8, pub physicalPresenceLifetimeLock: u8,
    pub physicalPresenceHWEnable: u8, pub physicalPresenceCMDEnable: u8, pub CEKPUsed: u8,
    pub TPMpost: u8, pub TPMpostLock: u8, pub FIPS: u8, pub operator: u8, pub enableRevokeEK: u8,
    pub nvLocked: u8, pub readSRKPub: u8, pub tpmEstablished: u8, pub maintenanceDone: u8,
    pub disableFullDALogicInfo: u8,
}
#[repr(C)] pub union cap_t { pub perm_flags: permanent_flags_t, pub stclear_flags: stclear_flags_t, pub owned: u8, pub num_pcrs: u32, pub version1: tpm1_version, pub version2: tpm1_version2, pub manufacturer_id: u32, pub timeout: timeout_t, pub duration: duration_t }

pub const TPM_MAX_RNG_DATA: usize = 128;
#[repr(C, packed)] pub struct tpm1_get_random_out { pub rng_data_len: u32, pub rng_data: [u8; TPM_MAX_RNG_DATA] }
pub const SRKHANDLE: u32 = 0x40000000;
pub const TPM_NONCE_SIZE: usize = 20;
pub const TPM_ST_CLEAR: u32 = 1;

pub const TPM2_SPACE_BUFFER_SIZE: usize = 16384;
pub const TPM2_SE_HMAC: u32 = 0; pub const TPM2_SE_POLICY: u32 = 1; pub const TPM2_SE_TRIAL: u32 = 2;
pub const TPM2_TIMEOUT_A: u32 = 750; pub const TPM2_TIMEOUT_B: u32 = 4000; pub const TPM2_TIMEOUT_C: u32 = 200; pub const TPM2_TIMEOUT_D: u32 = 30;
pub const TPM2_DURATION_SHORT: u32 = 20; pub const TPM2_DURATION_MEDIUM: u32 = 750; pub const TPM2_DURATION_LONG: u32 = 2000; pub const TPM2_DURATION_LONG_LONG: u32 = 300000; pub const TPM2_DURATION_DEFAULT: u32 = 120000;
pub const TPM2_ST_NO_SESSIONS: u32 = 0x8001; pub const TPM2_ST_SESSIONS: u32 = 0x8002; pub const TPM2_ST_CREATION: u32 = 0x8021;
pub const TSS2_RC_LAYER_SHIFT: u32 = 16; pub const TSS2_RESMGR_TPM_RC_LAYER: u32 = 11 << TSS2_RC_LAYER_SHIFT;
pub const TPM2_RC_SUCCESS: u32 = 0x0000; pub const TPM2_RC_HASH: u32 = 0x0083; pub const TPM2_RC_HANDLE: u32 = 0x008B; pub const TPM2_RC_INTEGRITY: u32 = 0x009F; pub const TPM2_RC_INITIALIZE: u32 = 0x0100; pub const TPM2_RC_FAILURE: u32 = 0x0101; pub const TPM2_RC_DISABLED: u32 = 0x0120; pub const TPM2_RC_UPGRADE: u32 = 0x012D; pub const TPM2_RC_COMMAND_CODE: u32 = 0x0143; pub const TPM2_RC_TESTING: u32 = 0x090A; pub const TPM2_RC_REFERENCE_H0: u32 = 0x0910; pub const TPM2_RC_RETRY: u32 = 0x0922; pub const TPM2_RC_SESSION_MEMORY: u32 = 0x0903;

pub const TPM2_CC_FIRST: u32 = 0x011F; pub const TPM2_CC_HIERARCHY_CONTROL: u32 = 0x0121; pub const TPM2_CC_HIERARCHY_CHANGE_AUTH: u32 = 0x0129; pub const TPM2_CC_CREATE_PRIMARY: u32 = 0x0131; pub const TPM2_CC_SEQUENCE_COMPLETE: u32 = 0x013E; pub const TPM2_CC_SELF_TEST: u32 = 0x0143; pub const TPM2_CC_STARTUP: u32 = 0x0144; pub const TPM2_CC_SHUTDOWN: u32 = 0x0145; pub const TPM2_CC_NV_READ: u32 = 0x014E; pub const TPM2_CC_CREATE: u32 = 0x0153; pub const TPM2_CC_LOAD: u32 = 0x0157; pub const TPM2_CC_SEQUENCE_UPDATE: u32 = 0x015C; pub const TPM2_CC_UNSEAL: u32 = 0x015E; pub const TPM2_CC_CONTEXT_LOAD: u32 = 0x0161; pub const TPM2_CC_CONTEXT_SAVE: u32 = 0x0162; pub const TPM2_CC_FLUSH_CONTEXT: u32 = 0x0165; pub const TPM2_CC_READ_PUBLIC: u32 = 0x0173; pub const TPM2_CC_START_AUTH_SESS: u32 = 0x0176; pub const TPM2_CC_VERIFY_SIGNATURE: u32 = 0x0177; pub const TPM2_CC_GET_CAPABILITY: u32 = 0x017A; pub const TPM2_CC_GET_RANDOM: u32 = 0x017B; pub const TPM2_CC_PCR_READ: u32 = 0x017E; pub const TPM2_CC_PCR_EXTEND: u32 = 0x0182; pub const TPM2_CC_EVENT_SEQUENCE_COMPLETE: u32 = 0x0185; pub const TPM2_CC_HASH_SEQUENCE_START: u32 = 0x0186; pub const TPM2_CC_CREATE_LOADED: u32 = 0x0191; pub const TPM2_CC_LAST: u32 = 0x0193;

pub const TPM2_CAP_HANDLES: u32 = 1; pub const TPM2_CAP_COMMANDS: u32 = 2; pub const TPM2_CAP_PCRS: u32 = 5; pub const TPM2_CAP_TPM_PROPERTIES: u32 = 6; pub const TPM_PT_TOTAL_COMMANDS: u32 = 0x0129;
pub const TPM2_SU_CLEAR: u32 = 0; pub const TPM2_SU_STATE: u32 = 1; pub const TPM2_CC_ATTR_CHANDLES: u32 = 25; pub const TPM2_CC_ATTR_RHANDLE: u32 = 28; pub const TPM2_CC_ATTR_VENDOR: u32 = 29; pub const TPM2_RH_NULL: u32 = 0x40000007; pub const TPM2_RS_PW: u32 = 0x40000009;
pub const TPM2_MSO_NVRAM: u32 = 1; pub const TPM2_MSO_SESSION: u32 = 2; pub const TPM2_MSO_POLICY: u32 = 3; pub const TPM2_MSO_PERMANENT: u32 = 0x40; pub const TPM2_MSO_VOLATILE: u32 = 0x80; pub const TPM2_MSO_PERSISTENT: u32 = 0x81;
pub const TPM2_ECC_NONE: u32 = 0; pub const TPM2_ECC_NIST_P256: u32 = 3;
pub const TPM2_PT_NONE: u32 = 0x00000000; pub const TPM2_PT_GROUP: u32 = 0x00000100; pub const TPM2_PT_FIXED: u32 = TPM2_PT_GROUP * 1;
pub const TPM2_PT_FAMILY_INDICATOR: u32 = TPM2_PT_FIXED + 0; pub const TPM2_PT_LEVEL: u32 = TPM2_PT_FIXED + 1; pub const TPM2_PT_REVISION: u32 = TPM2_PT_FIXED + 2; pub const TPM2_PT_DAY_OF_YEAR: u32 = TPM2_PT_FIXED + 3; pub const TPM2_PT_YEAR: u32 = TPM2_PT_FIXED + 4; pub const TPM2_PT_MANUFACTURER: u32 = TPM2_PT_FIXED + 5; pub const TPM2_PT_VENDOR_STRING_1: u32 = TPM2_PT_FIXED + 6; pub const TPM2_PT_VENDOR_STRING_2: u32 = TPM2_PT_FIXED + 7; pub const TPM2_PT_VENDOR_STRING_3: u32 = TPM2_PT_FIXED + 8; pub const TPM2_PT_VENDOR_STRING_4: u32 = TPM2_PT_FIXED + 9; pub const TPM2_PT_VENDOR_TPM_TYPE: u32 = TPM2_PT_FIXED + 10; pub const TPM2_PT_FIRMWARE_VERSION_1: u32 = TPM2_PT_FIXED + 11; pub const TPM2_PT_FIRMWARE_VERSION_2: u32 = TPM2_PT_FIXED + 12; pub const TPM2_PT_INPUT_BUFFER: u32 = TPM2_PT_FIXED + 13; pub const TPM2_PT_HR_TRANSIENT_MIN: u32 = TPM2_PT_FIXED + 14; pub const TPM2_PT_HR_PERSISTENT_MIN: u32 = TPM2_PT_FIXED + 15; pub const TPM2_PT_HR_LOADED_MIN: u32 = TPM2_PT_FIXED + 16; pub const TPM2_PT_ACTIVE_SESSIONS_MAX: u32 = TPM2_PT_FIXED + 17; pub const TPM2_PT_PCR_COUNT: u32 = TPM2_PT_FIXED + 18; pub const TPM2_PT_PCR_SELECT_MIN: u32 = TPM2_PT_FIXED + 19; pub const TPM2_PT_CONTEXT_GAP_MAX: u32 = TPM2_PT_FIXED + 20; pub const TPM2_PT_NV_COUNTERS_MAX: u32 = TPM2_PT_FIXED + 22; pub const TPM2_PT_NV_INDEX_MAX: u32 = TPM2_PT_FIXED + 23; pub const TPM2_PT_MEMORY: u32 = TPM2_PT_FIXED + 24; pub const TPM2_PT_CLOCK_UPDATE: u32 = TPM2_PT_FIXED + 25; pub const TPM2_PT_CONTEXT_HASH: u32 = TPM2_PT_FIXED + 26; pub const TPM2_PT_CONTEXT_SYM: u32 = TPM2_PT_FIXED + 27; pub const TPM2_PT_CONTEXT_SYM_SIZE: u32 = TPM2_PT_FIXED + 28; pub const TPM2_PT_ORDERLY_COUNT: u32 = TPM2_PT_FIXED + 29; pub const TPM2_PT_MAX_COMMAND_SIZE: u32 = TPM2_PT_FIXED + 30; pub const TPM2_PT_MAX_RESPONSE_SIZE: u32 = TPM2_PT_FIXED + 31; pub const TPM2_PT_MAX_DIGEST: u32 = TPM2_PT_FIXED + 32; pub const TPM2_PT_MAX_OBJECT_CONTEXT: u32 = TPM2_PT_FIXED + 33; pub const TPM2_PT_MAX_SESSION_CONTEXT: u32 = TPM2_PT_FIXED + 34; pub const TPM2_PT_PS_FAMILY_INDICATOR: u32 = TPM2_PT_FIXED + 35; pub const TPM2_PT_PS_LEVEL: u32 = TPM2_PT_FIXED + 36; pub const TPM2_PT_PS_REVISION: u32 = TPM2_PT_FIXED + 37; pub const TPM2_PT_PS_DAY_OF_YEAR: u32 = TPM2_PT_FIXED + 38; pub const TPM2_PT_PS_YEAR: u32 = TPM2_PT_FIXED + 39; pub const TPM2_PT_SPLIT_MAX: u32 = TPM2_PT_FIXED + 40; pub const TPM2_PT_TOTAL_COMMANDS: u32 = TPM2_PT_FIXED + 41; pub const TPM2_PT_LIBRARY_COMMANDS: u32 = TPM2_PT_FIXED + 42; pub const TPM2_PT_VENDOR_COMMANDS: u32 = TPM2_PT_FIXED + 43; pub const TPM2_PT_NV_BUFFER_MAX: u32 = TPM2_PT_FIXED + 44; pub const TPM2_PT_MODES: u32 = TPM2_PT_FIXED + 45; pub const TPM2_PT_MAX_CAP_BUFFER: u32 = TPM2_PT_FIXED + 46;
pub const TPM2_PT_VAR: u32 = TPM2_PT_GROUP * 2; pub const TPM2_PT_PERMANENT: u32 = TPM2_PT_VAR; pub const TPM2_PT_STARTUP_CLEAR: u32 = TPM2_PT_VAR + 1; pub const TPM2_PT_HR_NV_INDEX: u32 = TPM2_PT_VAR + 2; pub const TPM2_PT_HR_LOADED: u32 = TPM2_PT_VAR + 3; pub const TPM2_PT_HR_LOADED_AVAIL: u32 = TPM2_PT_VAR + 4; pub const TPM2_PT_HR_ACTIVE: u32 = TPM2_PT_VAR + 5; pub const TPM2_PT_HR_ACTIVE_AVAIL: u32 = TPM2_PT_VAR + 6; pub const TPM2_PT_HR_TRANSIENT_AVAIL: u32 = TPM2_PT_VAR + 7; pub const TPM2_PT_HR_PERSISTENT: u32 = TPM2_PT_VAR + 8; pub const TPM2_PT_HR_PERSISTENT_AVAIL: u32 = TPM2_PT_VAR + 9; pub const TPM2_PT_NV_COUNTERS: u32 = TPM2_PT_VAR + 10; pub const TPM2_PT_NV_COUNTERS_AVAIL: u32 = TPM2_PT_VAR + 11; pub const TPM2_PT_ALGORITHM_SET: u32 = TPM2_PT_VAR + 12; pub const TPM2_PT_LOADED_CURVES: u32 = TPM2_PT_VAR + 13; pub const TPM2_PT_LOCKOUT_COUNTER: u32 = TPM2_PT_VAR + 14; pub const TPM2_PT_MAX_AUTH_FAIL: u32 = TPM2_PT_VAR + 15; pub const TPM2_PT_LOCKOUT_INTERVAL: u32 = TPM2_PT_VAR + 16; pub const TPM2_PT_LOCKOUT_RECOVERY: u32 = TPM2_PT_VAR + 17; pub const TPM2_PT_NV_WRITE_RECOVERY: u32 = TPM2_PT_VAR + 18; pub const TPM2_PT_AUDIT_COUNTER_0: u32 = TPM2_PT_VAR + 19; pub const TPM2_PT_AUDIT_COUNTER_1: u32 = TPM2_PT_VAR + 20;
pub const TPM2_OA_FIXED_TPM: u32 = 1 << 1; pub const TPM2_OA_ST_CLEAR: u32 = 1 << 2; pub const TPM2_OA_FIXED_PARENT: u32 = 1 << 4; pub const TPM2_OA_SENSITIVE_DATA_ORIGIN: u32 = 1 << 5; pub const TPM2_OA_USER_WITH_AUTH: u32 = 1 << 6; pub const TPM2_OA_ADMIN_WITH_POLICY: u32 = 1 << 7; pub const TPM2_OA_NO_DA: u32 = 1 << 10; pub const TPM2_OA_ENCRYPTED_DUPLICATION: u32 = 1 << 11; pub const TPM2_OA_RESTRICTED: u32 = 1 << 16; pub const TPM2_OA_DECRYPT: u32 = 1 << 17; pub const TPM2_OA_SIGN: u32 = 1 << 18;
pub const TPM2_SA_CONTINUE_SESSION: u32 = 1; pub const TPM2_SA_AUDIT_EXCLUSIVE: u32 = 2; pub const TPM2_SA_AUDIT_RESET: u32 = 1 << 3; pub const TPM2_SA_DECRYPT: u32 = 1 << 5; pub const TPM2_SA_ENCRYPT: u32 = 1 << 6; pub const TPM2_SA_AUDIT: u32 = 1 << 7;
pub const TPM2_PLATFORM_PCR: u32 = 24; pub const TPM2_PCR_SELECT_MIN: usize = ((TPM2_PLATFORM_PCR + 7) / 8) as usize;
pub const TPM2_HT_HMAC_SESSION: u32 = 0x02000000; pub const TPM2_HT_POLICY_SESSION: u32 = 0x03000000; pub const TPM2_HT_TRANSIENT: u32 = 0x80000000;

pub const TPM_DIGEST_SIZE: usize = 20; pub const TPM_BUFSIZE: usize = 4096;
// SHA512_DIGEST_SIZE is supplied by crypto/sha2.h.
pub const TPM2_MAX_DIGEST_SIZE: usize = SHA512_DIGEST_SIZE;
pub const TPM2_MAX_NAME_SIZE: usize = TPM2_MAX_DIGEST_SIZE + 2; pub const TPM2_NULL_NAME_SIZE: usize = 34; pub const TPM2_MAX_PCR_BANKS: usize = 8;
pub const TPM_ALG_ERROR: u32 = 0x0000; pub const TPM_ALG_SHA1: u32 = 0x0004; pub const TPM_ALG_AES: u32 = 0x0006; pub const TPM_ALG_KEYEDHASH: u32 = 0x0008; pub const TPM_ALG_SHA256: u32 = 0x000B; pub const TPM_ALG_SHA384: u32 = 0x000C; pub const TPM_ALG_SHA512: u32 = 0x000D; pub const TPM_ALG_NULL: u32 = 0x0010; pub const TPM_ALG_SM3_256: u32 = 0x0012; pub const TPM_ALG_ECC: u32 = 0x0023; pub const TPM_ALG_CFB: u32 = 0x0043;
pub const TPM_LOCALITY_0: u32 = 0; pub const TPM_LOCALITY_1: u32 = 1; pub const TPM_LOCALITY_2: u32 = 2; pub const TPM_LOCALITY_3: u32 = 3; pub const TPM_LOCALITY_4: u32 = 4; pub const TPM_MAX_LOCALITY: u32 = TPM_LOCALITY_4;

#[repr(C)] pub struct tpm_bank_info { pub alg_id: u16, pub digest_size: u16, pub crypto_id: u16 }
pub const TPM_MAX_HASHES: usize = 5;
#[repr(C, packed)] pub struct tpm_digest { pub alg_id: u16, pub digest: [u8; TPM2_MAX_DIGEST_SIZE] }
pub const TPM_HEADER_SIZE: usize = 10;
#[repr(C)] pub union tpm_header__bindgen_ty_1 { pub ordinal: u32, pub return_code: u32 }
#[repr(C, packed)] pub struct tpm_header { pub tag: u16, pub length: u32, pub _bindgen_union: tpm_header__bindgen_ty_1 }

#[repr(C, packed)] pub struct tpm2_pcr_read_out { pub update_cnt: u32, pub pcr_selects_cnt: u32, pub hash_alg: u16, pub pcr_select_size: u8, pub pcr_select: [u8; TPM2_PCR_SELECT_MIN], pub digests_cnt: u32, pub digest_size: u16, pub digest: [u8; 0] }
#[repr(C, packed)] pub struct tpm2_get_random_out { pub size: u16, pub buffer: [u8; TPM_MAX_RNG_DATA] }
#[repr(C, packed)] pub struct tpm2_get_cap_out { pub more_data: u8, pub subcap_id: u32, pub property_cnt: u32, pub property_id: u32, pub value: u32 }
#[repr(C, packed)] pub struct tpm2_pcr_selection { pub hash_alg: u16, pub size_of_select: u8, pub pcr_select: [u8; 3] }
#[repr(C, packed)] pub struct tpm2_context { pub sequence: u64, pub saved_handle: u32, pub hierarchy: u32, pub blob_size: u16 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
