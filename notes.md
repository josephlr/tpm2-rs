Idea for TPMT_PUBLIC types:


Have TPMT_PUBLIC_PARMS include the unique field, just say that it's ignored when marshalling that type.
Then, only use the unique field/fields when unmarshalling  

TPML types inside non cmd/response structures
- Everything about capabilities (inside TPMS_CAPABILITY_DATA / TPMU_CAPABILITIES) only out
- TPMS_QUOTE_INFO (inside TPMU_ATTEST / TPMS_ATTEST / TPM2B_ATTEST) only out
- TPMS_CREATION_DATA (inside TPM2B_CREATION_DATA) only out

TPML IN/OUT should work

TPM2B_ types that are just buffers:

TPM2B_DIGEST
TPM2B_DATA
TPM2B_NONCE
TPM2B_AUTH
TPM2B_OPERAND
TPM2B_EVENT
TPM2B_MAX*
TPM2B_IV
TPM2B_SYM_KEY
TPM2B_LABEL
TPM2B_PUBLIC_KEY_RSA
TPM2B_PRIVATE_KEY_RSA
TPM2B_ECC_PARAMETER
TPM2B_ENCRYPTED_SECRET
TPM2B_PRIVATE_VENDOR_SPECIFIC
TPM2B_CONTEXT_SENSITIVE
TPM2B_TIMEOUT ?? (what is it used for)
TPM2B_PRIVATE ?? (is it just a opaque buffer)
TPM2B_SENSITIVE_DATA ?? (seems to just be opaque bytes)

TPM2B_ types holding structured data:

TPM2B_NAME (maybe handled specially??)
TPM2B_ATTEST
TPM2B_DERIVE
TPM2B_SENSITIVE_CREATE
TPM2B_ECC_POINT
TPM2B_PUBLIC
TPM2B_TEMPLATE ?? (why not just PUBLIC)
TPM2B_SENSITIVE
TPM2B_ID_OBJECT
TPM2B_NV_PUBLIC
TPM2B_CONTEXT_DATA
TPM2B_CREATION_DATA