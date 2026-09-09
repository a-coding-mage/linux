/* SPDX-License-Identifier: GPL-2.0-or-later */
/* ASN.1 Object identifier (OID) registry
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* OIDs are turned into these values if possible, or OID__NR if not held here.
 *
 * NOTE! Do not mess with the format of each line as this is read by
 * build_OID_registry.pl to generate the data for look_up_OID().
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OID {
    OID_id_dsa_with_sha1,
    OID_id_dsa,
    OID_id_ecPublicKey,
    OID_id_prime192v1,
    OID_id_prime256v1,
    OID_id_ecdsa_with_sha1,
    OID_id_ecdsa_with_sha224,
    OID_id_ecdsa_with_sha256,
    OID_id_ecdsa_with_sha384,
    OID_id_ecdsa_with_sha512,

    /* PKCS#1 {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-1(1)} */
    OID_rsaEncryption,
    OID_sha1WithRSAEncryption,
    OID_sha256WithRSAEncryption,
    OID_sha384WithRSAEncryption,
    OID_sha512WithRSAEncryption,
    OID_sha224WithRSAEncryption,
    /* PKCS#7 {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-7(7)} */
    OID_data,
    OID_signed_data,
    /* PKCS#9 {iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs-9(9)} */
    OID_email_address,
    OID_contentType,
    OID_messageDigest,
    OID_signingTime,
    OID_smimeCapabilites,
    OID_smimeAuthenticatedAttrs,

    OID_mskrb5,
    OID_krb5,
    OID_krb5u2u,
    /* Microsoft Authenticode & Software Publishing */
    OID_msIndirectData,
    OID_msStatementType,
    OID_msSpOpusInfo,
    OID_msPeImageDataObjId,
    OID_msIndividualSPKeyPurpose,
    OID_msOutlookExpress,
    OID_ntlmssp,
    OID_negoex,
    OID_spnego,
    OID_IAKerb,
    OID_PKU2U,
    OID_Scram,
    OID_certAuthInfoAccess,
    OID_sha1,
    OID_id_ansip384r1,
    OID_id_ansip521r1,
    OID_sha256,
    OID_sha384,
    OID_sha512,
    OID_sha224,
    /* Distinguished Name attribute IDs [RFC 2256] */
    OID_commonName,
    OID_surname,
    OID_countryName,
    OID_locality,
    OID_stateOrProvinceName,
    OID_organizationName,
    OID_organizationUnitName,
    OID_title,
    OID_description,
    OID_name,
    OID_givenName,
    OID_initials,
    OID_generationalQualifier,
    /* Certificate extension IDs */
    OID_subjectKeyIdentifier,
    OID_keyUsage,
    OID_subjectAltName,
    OID_issuerAltName,
    OID_basicConstraints,
    OID_crlDistributionPoints,
    OID_certPolicies,
    OID_authorityKeyIdentifier,
    OID_extKeyUsage,
    /* Heimdal mechanisms */
    OID_NetlogonMechanism,
    OID_appleLocalKdcSupported,
    /* EC-RDSA */
    OID_gostCPSignA,
    OID_gostCPSignB,
    OID_gostCPSignC,
    OID_gost2012PKey256,
    OID_gost2012PKey512,
    OID_gost2012Digest256,
    OID_gost2012Digest512,
    OID_gost2012Signature256,
    OID_gost2012Signature512,
    OID_gostTC26Sign256A,
    OID_gostTC26Sign256B,
    OID_gostTC26Sign256C,
    OID_gostTC26Sign256D,
    OID_gostTC26Sign512A,
    OID_gostTC26Sign512B,
    OID_gostTC26Sign512C,
    /* OSCCA */
    OID_sm2,
    OID_sm3,
    OID_SM2_with_SM3,
    OID_sm3WithRSAEncryption,
    /* TCG defined OIDS for TPM based keys */
    OID_TPMLoadableKey,
    OID_TPMImportableKey,
    OID_TPMSealedData,
    /* CSOR FIPS-202 SHA-3 */
    OID_sha3_256,
    OID_sha3_384,
    OID_sha3_512,
    OID_id_ecdsa_with_sha3_256,
    OID_id_ecdsa_with_sha3_384,
    OID_id_ecdsa_with_sha3_512,
    OID_id_rsassa_pkcs1_v1_5_with_sha3_256,
    OID_id_rsassa_pkcs1_v1_5_with_sha3_384,
    OID_id_rsassa_pkcs1_v1_5_with_sha3_512,
    /* NIST FIPS-204 ML-DSA */
    OID_id_ml_dsa_44,
    OID_id_ml_dsa_65,
    OID_id_ml_dsa_87,
    OID__NR,
}

unsafe extern "C" {
    pub fn look_up_OID(data: *const core::ffi::c_void, datasize: usize) -> OID;
    pub fn parse_OID(data: *const core::ffi::c_void, datasize: usize, oid: *mut OID) -> core::ffi::c_int;
    pub fn sprint_oid(data: *const core::ffi::c_void, datasize: usize, buffer: *mut core::ffi::c_char, buffersize: usize) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
