use crate::signature_module;

signature_module!(
    bundle_signatures,
    bundle_signature,
    BundleSignature,
    CreateBundleSignatureParams,
    CreateBundleSignatureResponse,
    DeleteBundleSignatureParams,
    DeleteBundleSignatureResponse,
    ListBundleSignaturesParams,
    ListBundleSignaturesResponse,
    CreateCommand,
    DeleteCommand,
    BundleSignaturesCommand,
    BundleSignaturesApi,
    bundle_prn,
    bundle_signature_prn,
    "/bundle_signatures"
);
