use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    ffi::c_void,
    mem::size_of,
};

pub use module_manager_derive;

pub trait Module {
    const ID: usize;
}

pub trait ModuleHandler<'a> {
    fn drop(&mut self);
    fn get_module<T: Module>(&self) -> &'a T;
    fn get_module_mut<T: Module>(&self) -> &'a mut T;
    fn get_module_ptr<T: Module>(&self) -> *const T;
    fn get_module_ptr_mut<T: Module>(&self) -> *mut T;
}

#[derive(Debug)]
struct ModuleInfo {
    size: usize,
    module_id: usize,
}

pub struct ModuleBundleBuilder {
    module_infos: Vec<ModuleInfo>,
    allocation_size: usize,
    module_pointers: Vec<usize>,
    max_module_id: usize,
}

pub struct ModuleBundle {
    pub layout: Layout,
    pub module_pointers: Vec<usize>,
}

impl ModuleBundle {
    pub const fn empty() -> Self {
        Self {
            layout: Layout::new::<c_void>(),
            module_pointers: Vec::new(),
        }
    }
}

impl Drop for ModuleBundle {
    fn drop(&mut self) {
        if self.module_pointers.len() > 0 {
            unsafe { dealloc(self.module_pointers[0] as *mut u8, self.layout) }
        }

        println!("Module bundle has been dropped");
    }
}

impl ModuleBundleBuilder {
    pub fn new() -> Self {
        Self {
            module_infos: Vec::new(),
            allocation_size: 0,
            module_pointers: Vec::new(),
            max_module_id: 0,
        }
    }

    pub fn add_module<T: Module>(mut self) -> Self {
        self.module_infos.push(ModuleInfo {
            size: size_of::<T>(),
            module_id: T::ID,
        });

        if T::ID > self.max_module_id {
            self.max_module_id = T::ID
        }

        self.allocation_size += size_of::<T>();
        self
    }

    pub fn build(mut self) -> Result<ModuleBundle, String> {
        if self.module_infos.is_empty() {
            return Ok(ModuleBundle::empty());
        }

        let layout = Layout::from_size_align(self.allocation_size, 8).map_err(|e| e.to_string())?;
        let alloc_pointer = unsafe { alloc_zeroed(layout) };

        let mut current_pointer_pos = alloc_pointer as usize;
        self.module_pointers = vec![0; self.max_module_id + 1];

        for module_info in self.module_infos.iter() {
            self.module_pointers[module_info.module_id] = current_pointer_pos;

            current_pointer_pos += module_info.size;
        }

        Ok(ModuleBundle {
            layout,
            module_pointers: self.module_pointers,
        })
    }
}
