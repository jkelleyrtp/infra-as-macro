use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Field, FieldValue, Fields, FieldsNamed, FieldsUnnamed, Token,
};

#[proc_macro]
pub fn Postgres16(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(tokens as MyMacroInput);

    // make sure the deploy directory is all wired up
    _ = std::fs::create_dir("deploy");

    // todo: translate the input into a terraform file
    std::fs::write("deploy/test.tf", input.to_token_stream().to_string()).unwrap();

    // dump out the tokens so the config is ready at compile time
    // Will need to do translation of dynamic fields
    ToTokens::to_token_stream(&input).into()
}

struct MyMacroInput {
    fields: Vec<FieldValue>,
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fields = vec![];

        while !input.is_empty() {
            // partial expansion
            if let Ok(field) = input.parse() {
                fields.push(field)
            } else {
                break;
            }

            if input.is_empty() {
                break;
            }

            input.parse::<Token![,]>()?;
        }

        Ok(Self { fields })
    }
}

impl ToTokens for MyMacroInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let fields = self.fields.iter().map(|field| {
            let name = &field.member;
            let value = &field.expr;

            quote! {
                #name: #value
            }
        });

        let out = quote!(
            //
            Postgres {
                opts: PostgresOpts {
                    #(#fields),*
                }
            }
        );

        out.to_tokens(tokens);
    }

    // fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    //     let fields = &self.fields;

    //     tokens.extend(quote! {
    //         #(#fields),*
    //     });
    // }
}

#[proc_macro_derive(Table, attributes(props))]
pub fn derive_typed_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    quote! {
        impl Table for #name {
            type Id = i32;
        }
    }
    .into()
}
