use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DataSource)]
pub fn data_source_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_data_source_macro(&ast)
}

#[proc_macro_derive(OAuth)]
pub fn oauth_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_oauth_macro(&ast)
}

fn impl_data_source_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
    #[async_trait]
    impl DataSource<#name> for #name {
        fn as_persist_hashmap(&self) -> HashMap<&str, String> {
            HashMap::from([
                ("uuid", self.uuid.clone()),
                ("name", self.name.clone()),
                ("authentication_type", self.authentication_type.clone()),
                ("data_source_type", self.data_source_type.clone()),
            ])
        }
        fn authenticate(&mut self) {
            self.access_token = self.refresh_token();
        }

        async fn create(
            conn: &DatabaseConnection,
            name: String,
            oauth_authorization_code: String,
        ) -> Option<#name> {
            let uuid = uuid::Uuid::new_v4();

            let (access_token, refresh_token) = Self::get_access_tokens(&oauth_authorization_code)?;

            let data_source = Self::new(
                uuid.to_string(),
                name,
                String::from("oauth"),
                String::from("gitlab"),
                access_token,
                refresh_token,
            )?;

            if let Err(_) = Mutation::create_data_source(conn, data_source.as_persist_hashmap()).await {
                return None;
            }

            Some(data_source)
        }

        fn new(
            uuid: String,
            name: String,
            authentication_type: String,
            data_source_type: String,
            access_token: String,
            refresh_token: String,
        ) -> Option<#name> {
            Some(#name {
                uuid,
                name,
                authentication_type,
                data_source_type,
                access_token,
                refresh_token,
            })
        }

        fn properties(&self) {
            // HashMap
        }
    }
    };
    gen.into()
}

fn impl_oauth_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl OAuth for #name {
            fn refresh_token(&self) -> String {
                // TODO
                String::from("xxxx-xxx-xxx-xxxx")
            }

            fn get_access_tokens(_oauth_authorization_code: &String) -> Option<(String, String)> {
                // TODO
                Some((
                    String::from("xxxx-xxx-xxx-xxxx"),
                    String::from("xxxx-xxx-xxx-xxxx"),
                ))
            }
        }
    };
    gen.into()
}
