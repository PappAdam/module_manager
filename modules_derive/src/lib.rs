use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

static mut MODULE_ID: usize = 0;
static mut HANDLER: bool = false;

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

#[deprecated]
#[proc_macro_attribute]
pub fn module_handler(_: TokenStream, item: TokenStream) -> TokenStream {
    if unsafe { HANDLER } {
        panic!("Only one handler object is allowed to be created yet");
    }

    let module_body = syn::parse_macro_input!(item as syn::ItemStruct);

    let name = module_body.ident;
    let vis = module_body.vis;
    let generics = module_body.generics.params;
    let fields = if let Some(fileds) = match module_body.fields {
        syn::Fields::Named(fileds_named) => Some(fileds_named),
        _ => None,
    } {
        Some(fileds.named)
    } else {
        None
    };

    let gen = quote! {
        #vis struct #name <#generics> {
            #fields
            _layout: std::alloc::Layout,
            _module_pointers: Vec<usize>,
        }

        impl<'handler, #generics> ModuleHandler<'handler> for #name <#generics> {
            #[inline]
            fn drop(&mut self) {
                extern crate alloc;
                unsafe {
                    alloc::alloc::dealloc(self._module_pointers[0] as *mut u8, self._layout)
                }
            }

            #[inline]
            fn get_module<T: Module>(&self) -> &'handler T {
                unsafe {&*((self._module_pointers[T::ID]) as *const T)}
            }

            #[inline]
            fn get_module_mut<T: Module>(&self) -> &'handler mut T {
                unsafe {&mut *((self._module_pointers[T::ID]) as *mut T)}
            }

            #[inline]
            fn get_module_ptr<T: Module>(&self) -> *const T {
                self._module_pointers[T::ID] as *const T
            }

            #[inline]
            fn get_module_ptr_mut<T: Module>(&self) -> *mut T {
                self._module_pointers[T::ID] as *mut T
            }

        }
    };

    unsafe {
        HANDLER = true;
    }

    gen.into()
}
