#[macro_export]
macro_rules! signature_module {
    (
        $module_name:ident,
        $response_field:ident,
        $signature_struct:ident,
        $create_params:ident,
        $create_response:ident,
        $delete_params:ident,
        $delete_response:ident,
        $create_command:ident,
        $delete_command:ident,
        $signatures_command:ident,
        $signatures_api:ident,
        $prn_field:ident,
        $prn_param:ident,
        $endpoint:expr
    ) => {
        use reqwest::Method;
        use serde::{Deserialize, Serialize};

        use $crate::{json_body, Api};

        use super::Error;
        use snafu::ResultExt;

        #[derive(Debug, Deserialize, Serialize)]
        pub struct $signature_struct {
            pub $prn_field: String,
            pub inserted_at: String,
            pub keyid: String,
            pub organization_prn: String,
            pub prn: String,
            pub signature: String,
            pub signing_key_prn: String,
            pub updated_at: String,
        }

        #[derive(Debug, Serialize)]
        pub struct $create_params {
            pub $prn_field: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub signing_key_prn: Option<String>,
            pub signature: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub signing_key_keyid: Option<String>,
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct $create_response {
            pub $response_field: $signature_struct,
        }

        #[derive(Debug, Serialize)]
        pub struct $delete_params {
            pub $prn_param: String,
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct $delete_response {}

        // Command types for the command pattern
        #[derive(Debug)]
        pub struct $create_command {
            pub params: $create_params,
        }

        #[derive(Debug)]
        pub struct $delete_command {
            pub params: $delete_params,
        }

        #[derive(Debug)]
        pub struct Command<T> {
            pub inner: T,
            // Additional command metadata could go here
            pub metadata: Option<String>,
        }

        #[derive(Debug)]
        pub enum $signatures_command {
            Create(Box<Command<$create_command>>),
            Delete(Command<$delete_command>),
        }

        pub struct $signatures_api<'a>(pub &'a Api);

        impl<'a> $signatures_api<'a> {
            pub async fn create(
                &'a self,
                params: $create_params,
            ) -> Result<Option<$create_response>, Error> {
                self.0
                    .execute(Method::POST, $endpoint, Some(json_body!(&params)))
                    .await
            }

            pub async fn delete(
                &'a self,
                params: $delete_params,
            ) -> Result<Option<$delete_response>, Error> {
                let prn: String = params.$prn_param;
                self.0
                    .execute(Method::DELETE, format!("{}/{prn}", $endpoint), None)
                    .await
            }
        }
    };
}
