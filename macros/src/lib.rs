use convert_case::{Case, Casing};
use darling::{FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemFn};

#[derive(FromField, Clone, Debug)]
#[darling(attributes(model), forward_attrs(allow, doc, cfg))]
struct ModelField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

impl ToTokens for ModelField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.ident;
        let ty = &self.ty;
        quote!(#name: #ty).to_tokens(tokens)
    }
}

#[derive(FromDeriveInput, Clone, Debug)]
#[darling(
    attributes(model),
    supports(struct_named),
    forward_attrs(allow, doc, cfg)
)]
struct ModelOptions {
    ident: syn::Ident,
    vis: syn::Visibility,
    data: darling::ast::Data<darling::util::Ignored, ModelField>,

    name: String,
}

#[proc_macro_derive(Model, attributes(model))]
pub fn model(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let opts = ModelOptions::from_derive_input(&input).unwrap();

    let name = &opts.ident;

    let stct = opts.data.take_struct().unwrap();

    let id_field = stct
        .fields
        .iter()
        .find(|f| f.ident.clone().unwrap() == "id")
        .unwrap();

    let id_type = &id_field.ty;
    let repo_name = format_ident!("{}Repo", name);
    let table_name = opts.name;
    let vis = &opts.vis;

    let insertion_name = &format_ident!("{}Insertion", name);
    let insertion_fields = stct
        .fields
        .iter()
        .filter(|f| f.ident.clone().unwrap() != "id");

    let insertion_fields_names = stct
        .fields
        .iter()
        .filter(|f| f.ident.clone().unwrap() != "id")
        .map(|f| f.ident.clone());
    let insertion_fields_names_1 = insertion_fields_names.clone();
    let in_query_field_names = insertion_fields_names_1
        .clone()
        .map(|f| f.unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
    let in_query_fields_tag = insertion_fields_names_1
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect::<Vec<String>>()
        .join(",");
    let insert_query = format!(
        r#"--sql
        INSERT INTO {} ({}) VALUES ({})
        RETURNING id
        "#,
        table_name, in_query_field_names, in_query_fields_tag
    );

    let insertion_fields_names_2 = insertion_fields_names.clone();

    let updating_name = &format_ident!("{}Updating", name);
    let updating_fields = insertion_fields.clone().map(|f| {
        let name = &f.ident;
        let _ty = &f.ty;
        quote!(#name: crate::database::repo::Value<#_ty>)
    });
    let updating_get_query_fields = insertion_fields.clone().map(|f| {
        let name = &f.ident;
        let _ty = &f.ty;
        let format_string = format!("{} = {{}}", name.clone().unwrap());

        quote!(
            match self.#name {
                crate::database::repo::Value::Set(value) => {
                    set_part.push(format!(#format_string, value));
                }
                crate::database::repo::Value::Unset => (),
            };
        )
    });
    let query_string = format!("UPDATE {} SET {{}}", table_name);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        #[derive(Debug, Default)]
        #vis struct #insertion_name {
            #(#insertion_fields,)*
        }

        impl From<#name> for #insertion_name {
            fn from(value: #name) -> Self {
                #insertion_name {
                    #(#insertion_fields_names: value.#insertion_fields_names,)*
                }
            }
        }

        impl crate::database::repo::Model<#id_type> for #name {
            fn get_id(&self) -> &#id_type {
                &self.id
            }
        }

        #[derive(Debug)]
        #vis struct #repo_name<'r> {
            pool: &'r sqlx::PgPool,
        }

        impl<'r> From<&'r sqlx::PgPool> for #repo_name<'r> {
            fn from(pool: &'r sqlx::PgPool) -> Self {
                #repo_name { pool }
            }
        }

        impl<'r> crate::database::Db<'r> for #repo_name<'r> {
            fn get_pool(&self) -> &sqlx::PgPool {
                &self.pool
            }
        }

        impl<'r> crate::database::repo::Repo<'r, #name, #id_type> for #repo_name<'r> {
            const TABLE: &'static str = #table_name;

            fn new(pool: &'r sqlx::PgPool) -> Self {
                Self { pool }
            }
        }

        impl<'r> #repo_name<'r> {
            pub async fn insert(&self, model: impl Into<#insertion_name>) -> anyhow::Result<#id_type> {
                let model = model.into();
                let inserted = sqlx::query!(
                    #insert_query,
                    #(model.#insertion_fields_names_2,)*
                )
                .fetch_one(self.get_pool())
                .await?;

                Ok(inserted.id)
            }
        }

        #[derive(Debug, Default)]
        #vis struct #updating_name {
            #(#updating_fields,)*
        }

        impl crate::database::repo::Updating for #updating_name {
            fn get_raw_update_query(&self) -> String {
                let mut set_part = vec![];
                #(#updating_get_query_fields)*
                format!(#query_string, set_part.join(","))
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(rename = "for")]
    for_update: String,
    #[darling(multiple, rename = "param")]
    extra_params: Vec<String>,
}

#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let vis = &input.vis;
    let struct_name = format_ident!(
        "{}{}Handler",
        input.sig.ident.to_string().to_case(Case::Pascal),
        args.for_update
    );
    let update_type: proc_macro2::TokenStream = args.for_update.parse().unwrap();
    let update_path = quote!(teloxide::types::#update_type);
    let func_name = &input.sig.ident;

    let extra_params_ty = args.extra_params.iter().map(|f| {
        let token: proc_macro2::TokenStream = f.parse().unwrap();
        token
    });
    let extra_params_name = args
        .extra_params
        .iter()
        .map(|f| format_ident!("{}", f.split("::").last().unwrap().to_case(Case::Snake)));

    let extra_params_ty_1 = extra_params_ty.clone();
    let extra_params_name_1 = extra_params_name.clone();
    let extra_params_name_2 = extra_params_name.clone();

    let tokens = quote! {
        #input

        #[derive(Debug, Clone)]
        #vis struct #struct_name {
            bot: teloxide::Bot,
            update: #update_path,
            #(#extra_params_name: #extra_params_ty,)*
        }

        impl #struct_name {
            pub async fn handle(bot: teloxide::Bot, update: #update_path, #(#extra_params_name_1: #extra_params_ty_1,)*) -> anyhow::Result<()> {
                let handler  = Self {
                    bot,
                    update,
                    #(#extra_params_name_2,)*
                };

                let res = #func_name(handler).await;
                res
            }
        }

        #[async_trait::async_trait(?Send)]
        impl crate::handlers::Handler<#update_path> for #struct_name {
            fn bot(&self) -> &teloxide::Bot {
                &self.bot
            }

            fn update(&self) -> &#update_path{
                &self.update
            }
        }
    };

    TokenStream::from(tokens)
}
