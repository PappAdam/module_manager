use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

static mut MODULE_ID: usize = 0;

#[proc_macro_derive(Module)]
pub fn module(input: TokenStream) -> TokenStream {
    let module_body = syn::parse_macro_input!(input as DeriveInput);

    let name = module_body.ident;

    let gen = unsafe {
        quote! {
            impl Module for #name {
                const ID: usize = #MODULE_ID;
            }
        }
    };

    unsafe { MODULE_ID += 1 };

    gen.into()
}

#[proc_macro_attribute]
pub fn module_handler(_: TokenStream, item: TokenStream) -> TokenStream {
    let module_body = syn::parse_macro_input!(item as syn::ItemStruct);

    let name = module_body.ident;
    let vis = module_body.vis;
    let fields = if let Some(fileds) = match module_body.fields {
        syn::Fields::Named(fileds_named) => Some(fileds_named),
        _ => None,
    } {
        Some(fileds.named)
    } else {
        None
    };

    unsafe { dbg!(MODULE_ID) };

    let gen = quote! {
        #vis struct #name {
            #fields
            _layout: std::alloc::Layout,
            _module_pointers: Vec<*mut u8>,
        }

        impl<'a> ModuleHandler<'a> for #name {
            fn drop(&mut self) {
                extern crate alloc;
                unsafe {
                    alloc::alloc::dealloc(self._module_pointers[0], self._layout)
                }
            }

            fn get_module<T: Module>(&self) -> &'a T {
                unsafe {&*((self._module_pointers[T::ID]) as *const T)}
            }

            fn get_module_mut<T: Module>(&self) -> &'a mut T {
                unsafe {&mut *((self._module_pointers[T::ID]) as *mut T)}
            }
        }


    };

    gen.into()
}
