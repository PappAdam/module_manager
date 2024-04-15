mod tests;

use std::{
    alloc::{alloc_zeroed, Layout},
    mem::size_of,
};

pub trait Module {
    const ID: usize;
}

pub trait ModuleHandler<'a> {
    fn drop(&mut self);
    fn get_module<T: Module>(&self) -> &'a T;
    fn get_module_mut<T: Module>(&self) -> &'a mut T;
}

struct ModuleInfo {
    size: usize,
    module_id: usize,
}

pub struct ModuleBuilder {
    module_infos: Vec<ModuleInfo>,
    allocation_size: usize,
    module_pointers: Vec<*mut u8>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            module_infos: Vec::new(),
            allocation_size: 0,
            module_pointers: Vec::new(),
        }
    }

    pub fn add_module<T: Module>(mut self) -> Self {
        self.module_infos.push(ModuleInfo {
            size: size_of::<T>(),
            module_id: T::ID,
        });

        self.allocation_size += size_of::<T>();

        self
    }

    pub fn build(mut self) -> Result<(Layout, Vec<*mut u8>), String> {
        let layout = Layout::from_size_align(self.allocation_size, 8).map_err(|e| e.to_string())?;
        let alloc_pointer = unsafe { alloc_zeroed(layout) };

        self.module_infos
            .sort_by(|mia, mib| mia.module_id.partial_cmp(&mib.module_id).unwrap());

        let mut current_pointer_pos = alloc_pointer as usize;

        for module_info in self.module_infos.iter() {
            self.module_pointers.push(current_pointer_pos as *mut u8);
            current_pointer_pos += module_info.size;
        }

        Ok((layout, self.module_pointers))
    }
}
