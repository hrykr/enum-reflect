use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident};

/// Derive reflection for enum fields to access name and value.
/// Use mut functions to get mutable fields.
/// 
/// # Implementing
/// 
/// ```
/// #[derive(EnumReflect)]
/// enum ExampleEnum;
/// ```
/// # Functions
/// 
/// - `fn get_fields() -> Vec<&mut dyn std::any::Any>` Return immutable field values
/// - `fn get_named_fields() -> Vec<(&'static str, &mut dyn std::any::Any)>` Return field names and immutable field values
/// - `fn get_fields_mut() -> Vec<&mut dyn std::any::Any>` Return mutable field values
/// - `fn get_named_fields_mut() -> Vec<(&'static str, &mut dyn std::any::Any)>` Return field names and mutable field values
///
/// # Example
/// 
/// ```
/// #[derive(EnumReflect)]
/// pub enum Example {
///     Empty,
///     Example1 {
///         var1: String,
///         var2: i32,
///     },
///     Example2 {
///         var1: String,
///         var2: i32,
///         var3: bool,
///     },
/// }
/// 
/// fn foo() {
///     let exml = Example::Example1 { var1: "Hello, World!".to_string(), var2: 32 };
///     
///     for (name, value) in exml.get_named_fields() {
///         println!("Field {} is {}", name, value.to_string());
///     }
/// }
/// ```
/// 
/// ## Output
///
/// - `Field var1 is Hello, World!`
/// - `Field var2 is 32`
///
#[proc_macro_derive(EnumReflect)]
pub fn enum_reflection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(name, "EnumReflect only works on enums")
            .to_compile_error()
            .into();
    };

    // For get_fields()
    let get_fields_arms = data_enum.variants.iter().map(|v| {
        let variant_ident = &v.ident;

        match &v.fields {
            Fields::Named(fields_named) => {
                let bindings: Vec<_> = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    quote! { #ident }
                }).collect();

                let refs: Vec<_> = bindings.iter().map(|ident| {
                    quote! { #ident as &dyn std::any::Any }
                }).collect();

                quote! {
                    #name::#variant_ident { #(#bindings),* } => vec![#(#refs),*],
                }
            }

            Fields::Unnamed(fields_unnamed) => {
                let bindings: Vec<_> = (0..fields_unnamed.unnamed.len())
                    .map(|i| syn::Ident::new(&format!("f{}", i), v.ident.span()))
                    .collect();

                let refs: Vec<_> = bindings.iter().map(|ident| {
                    quote! { #ident as &dyn std::any::Any }
                }).collect();

                quote! {
                    #name::#variant_ident( #(#bindings),* ) => vec![#(#refs),*],
                }
            }

            Fields::Unit => {
                quote! {
                    #name::#variant_ident => vec![],
                }
            }
        }
    });

    // For get_named_fields()
    let get_named_fields_arms = data_enum.variants.iter().map(|v| {
        let variant_ident = &v.ident;

        match &v.fields {
            Fields::Named(fields_named) => {
                let bindings: Vec<_> = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    quote! { #ident }
                }).collect();

                let pairs: Vec<_> = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    let name_str = ident.to_string();
                    quote! { (#name_str, #ident as &dyn std::any::Any) }
                }).collect();

                quote! {
                    #name::#variant_ident { #(#bindings),* } => vec![#(#pairs),*],
                }
            }

            Fields::Unnamed(_) => {
                // You can skip unnamed fields for `get_named_fields`
                quote! {
                    #name::#variant_ident(..) => vec![],
                }
            }

            Fields::Unit => {
                quote! {
                    #name::#variant_ident => vec![],
                }
            }
        }
    });

    // For get_fields_mut()
    let get_fields_mut_arms = data_enum.variants.iter().map(|v| {
        let variant_ident = &v.ident;

        match &v.fields {
            Fields::Named(fields_named) => {
                let bindings: Vec<_> = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    quote! { #ident }
                }).collect();

                let refs: Vec<_> = bindings.iter().map(|ident| {
                    quote! { #ident as &mut dyn std::any::Any }
                }).collect();

                quote! {
                    #name::#variant_ident { #(#bindings),* } => vec![#(#refs),*]
                }
            }

            Fields::Unnamed(fields_unnamed) => {
                let bindings: Vec<_> = (0..fields_unnamed.unnamed.len())
                    .map(|i| Ident::new(&format!("f{}", i), v.ident.span()))
                    .map(|ident| quote! { #ident })
                    .collect();

                let refs: Vec<_> = bindings.iter().map(|ident| {
                    quote! { #ident as &mut dyn std::any::Any }
                }).collect();

                quote! {
                    #name::#variant_ident( #(#bindings),* ) => vec![#(#refs),*]
                }
            }

            Fields::Unit => {
                quote! {
                    #name::#variant_ident => vec![]
                }
            }
        }
    });

    // For get_named_fields_mut()
    let get_named_fields_mut_arms = data_enum.variants.iter().map(|v| {
        let variant_ident = &v.ident;

        match &v.fields {
            Fields::Named(fields_named) => {
                let bindings: Vec<_> = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    quote! { #ident }
                }).collect();

                let pairs: Vec<_> = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    let name_str = ident.to_string();
                    quote! { (#name_str, #ident as &mut dyn std::any::Any) }
                }).collect();

                quote! {
                    #name::#variant_ident { #(#bindings),* } => vec![#(#pairs),*]
                }
            }

            Fields::Unnamed(_) => {
                quote! {
                    #name::#variant_ident(..) => vec![]
                }
            }

            Fields::Unit => {
                quote! {
                    #name::#variant_ident => vec![]
                }
            }
        }
    });

    let expanded = quote! {
        impl enum_reflect_extetn::EnumReflect for #name {
            pub fn get_fields(&self) -> Vec<&dyn std::any::Any> {
                match self {
                    #(#get_fields_arms)*
                }
            }

            pub fn get_named_fields(&self) -> Vec<(&'static str, &dyn std::any::Any)> {
                match self {
                    #(#get_named_fields_arms)*
                }
            }

            pub fn get_fields_mut(&mut self) -> Vec<&mut dyn std::any::Any> {
                match self {
                    #(#get_fields_mut_arms),*
                }
            }

            pub fn get_named_fields_mut(&mut self) -> Vec<(&'static str, &mut dyn std::any::Any)> {
                match self {
                    #(#get_named_fields_mut_arms),*
                }
            }
        }
    };

    expanded.into()
}

#[deprecated(note = "Use #[derive(EnumReflect)] instead, now mut also included there")]
#[proc_macro_derive(EnumReflectMut)]
pub fn enum_reflection_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(name, "EnumReflectMut only works on enums")
            .to_compile_error()
            .into();
    };

    // For get_fields_mut()
    let get_fields_mut_arms = data_enum.variants.iter().map(|v| {
        let variant_ident = &v.ident;

        match &v.fields {
            Fields::Named(fields_named) => {
                let bindings: Vec<_> = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    quote! { #ident }
                }).collect();

                let refs: Vec<_> = bindings.iter().map(|ident| {
                    quote! { #ident as &mut dyn std::any::Any }
                }).collect();

                quote! {
                    #name::#variant_ident { #(#bindings),* } => vec![#(#refs),*]
                }
            }

            Fields::Unnamed(fields_unnamed) => {
                let bindings: Vec<_> = (0..fields_unnamed.unnamed.len())
                    .map(|i| Ident::new(&format!("f{}", i), v.ident.span()))
                    .map(|ident| quote! { #ident })
                    .collect();

                let refs: Vec<_> = bindings.iter().map(|ident| {
                    quote! { #ident as &mut dyn std::any::Any }
                }).collect();

                quote! {
                    #name::#variant_ident( #(#bindings),* ) => vec![#(#refs),*]
                }
            }

            Fields::Unit => {
                quote! {
                    #name::#variant_ident => vec![]
                }
            }
        }
    });

    // For get_named_fields_mut()
    let get_named_fields_mut_arms = data_enum.variants.iter().map(|v| {
        let variant_ident = &v.ident;

        match &v.fields {
            Fields::Named(fields_named) => {
                let bindings: Vec<_> = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    quote! { #ident }
                }).collect();

                let pairs: Vec<_> = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    let name_str = ident.to_string();
                    quote! { (#name_str, #ident as &mut dyn std::any::Any) }
                }).collect();

                quote! {
                    #name::#variant_ident { #(#bindings),* } => vec![#(#pairs),*]
                }
            }

            Fields::Unnamed(_) => {
                quote! {
                    #name::#variant_ident(..) => vec![]
                }
            }

            Fields::Unit => {
                quote! {
                    #name::#variant_ident => vec![]
                }
            }
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn get_fields_mut(&mut self) -> Vec<&mut dyn std::any::Any> {
                match self {
                    #(#get_fields_mut_arms),*
                }
            }

            pub fn get_named_fields_mut(&mut self) -> Vec<(&'static str, &mut dyn std::any::Any)> {
                match self {
                    #(#get_named_fields_mut_arms),*
                }
            }
        }
    };

    expanded.into()
}