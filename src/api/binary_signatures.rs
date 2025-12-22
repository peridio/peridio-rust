use crate::signature_module;

signature_module!(
    binary_signature,
    binary_signature,
    BinarySignature,
    CreateBinarySignatureParams,
    CreateBinarySignatureResponse,
    DeleteBinarySignatureParams,
    DeleteBinarySignatureResponse,
    CreateCommand,
    DeleteCommand,
    BinarySignaturesCommand,
    BinarySignaturesApi,
    binary_prn,
    binary_signature_prn,
    "/binary_signatures"
);
