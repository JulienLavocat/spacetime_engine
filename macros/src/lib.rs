use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemStruct, parse_macro_input};

/// Usage:
///
/// ```rust
/// #[event_table]
/// pub struct MyTable {
///     pub my_field: u32,
/// }
/// ```
///
/// Generates:
/// ```rust
/// #[spacetimedb::table(name = my_table, scheduled(steng_remove_event_my_table))]
/// pub struct MyTable {
///     pub id: u64,
///     pub scheduled_at: spacetimedb::ScheduleAt,
///     pub my_field: u32,
/// }
///
/// #[spacetimedb::reducer]
/// pub fn steng_remove_event_my_table(ctx: &spacetimedb::ReducerContext, data: MyTable) {
/// }
/// ```
#[proc_macro_attribute]
pub fn event_table(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return quote! {
            compile_error!("`#[event_table]` does not take any arguments.");
        }
        .into();
    }

    let mut input = parse_macro_input!(item as ItemStruct);

    let struct_ident = input.ident.clone();

    let table_name_snake = heck::AsSnakeCase(struct_ident.to_string()).to_string();

    let table_name_ident = format_ident!("{}", table_name_snake);
    let remove_event_fn_ident = format_ident!("steng_remove_event_{}", table_name_snake);

    match &mut input.fields {
        Fields::Named(fields_named) => {
            use syn::parse_quote;

            fields_named.named.insert(
                0,
                parse_quote! { pub scheduled_at: spacetimedb::ScheduleAt },
            );
            fields_named.named.insert(
                0,
                parse_quote! {
                    #[primary_key]
                    #[auto_inc]
                    pub id: u64
                },
            );
        }
        Fields::Unit => {
            // Turn unit struct into a named-field struct with our fields.
            use syn::parse_quote;
            let mut named = syn::punctuated::Punctuated::new();
            named.push(parse_quote! { pub id: u64 });
            named.push(parse_quote! { pub schedule_at: spacetimedb::ScheduleAt });

            input.fields = Fields::Named(syn::FieldsNamed {
                brace_token: Default::default(),
                named,
            });
        }
        Fields::Unnamed(_) => {
            return quote! {
                compile_error!("`#[event_table]` requires a struct with named fields or a unit struct.");
            }
            .into();
        }
    }

    // Generate:
    //
    // #[spacetimedb::table(name = my_table, scheduled(tick_my_table))]
    // <struct with injected fields>
    //
    // #[spacetimedb::reducer]
    // pub fn tick_my_table(...)

    let expanded = quote! {
        #[::spacetimedb::table(name = #table_name_ident, scheduled(#remove_event_fn_ident))]
        #input

        #[::spacetimedb::reducer]
        pub fn #remove_event_fn_ident(
            ctx: &::spacetimedb::ReducerContext,
            data: #struct_ident,
        ) {
            use spacetimedb::Table;
            ctx.db.#table_name_ident().id().delete(data.id);
        }
    };

    TokenStream::from(expanded)
}
