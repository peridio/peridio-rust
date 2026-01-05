use crate::signature_module;

signature_module!(
    binary_signatures,
    binary_signature,
    BinarySignature,
    CreateBinarySignatureParams,
    CreateBinarySignatureResponse,
    DeleteBinarySignatureParams,
    DeleteBinarySignatureResponse,
    ListBinarySignaturesParams,
    ListBinarySignaturesResponse,
    CreateCommand,
    DeleteCommand,
    BinarySignaturesCommand,
    BinarySignaturesApi,
    binary_prn,
    binary_signature_prn,
    "/binary_signatures"
);
