use manager::{Module, ModuleBundle};

pub mod manager;

static mut MODULE_MANAGER: ModuleBundle = ModuleBundle::empty();

#[inline]
pub fn get_module<T: Module>() -> &'static T {
    unsafe { &*((MODULE_MANAGER.module_pointers[T::ID]) as *const T) }
}

#[inline]
pub fn get_module_mut<T: Module>() -> &'static mut T {
    unsafe { &mut *((MODULE_MANAGER.module_pointers[T::ID]) as *mut T) }
}

#[inline]
pub fn get_module_ptr_mut<T: Module>() -> *mut T {
    unsafe { (MODULE_MANAGER.module_pointers[T::ID]) as *mut T }
}

#[inline]
pub fn init(bundle: ModuleBundle) {
    unsafe {
        MODULE_MANAGER = bundle;
    }
}

#[inline]
pub fn change_bundle(bundle: ModuleBundle) {
    unsafe {
        MODULE_MANAGER = bundle;
    }
}
