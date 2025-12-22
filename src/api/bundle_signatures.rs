use crate::signature_module;

signature_module!(
    bundle_signature,
    bundle_signature,
    BundleSignature,
    CreateBundleSignatureParams,
    CreateBundleSignatureResponse,
    DeleteBundleSignatureParams,
    DeleteBundleSignatureResponse,
    CreateCommand,
    DeleteCommand,
    BundleSignaturesCommand,
    BundleSignaturesApi,
    bundle_prn,
    bundle_signature_prn,
    "/bundle_signatures"
);
