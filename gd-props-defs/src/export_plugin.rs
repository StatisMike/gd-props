use std::io::Read;

use godot::builtin::meta::ClassName;
use godot::builtin::{GString, PackedByteArray};
use godot::engine::file_access::ModeFlags;
use godot::engine::{
    load, save, try_load, DirAccess, EditorExportPlugin, FileAccess, GFile, IEditorExportPlugin, Object, Resource, ResourceLoader, ResourceSaver, ResourceUid
};
use godot::log::godot_error;
use godot::obj::bounds::MemRefCounted;
use godot::obj::cap::GodotDefault;
use godot::obj::{Bounds, GodotClass, Inherits, UserClass};

use crate::gdprop::GdProp;

pub trait GdPropExporter
where
    Self: GodotClass
        + UserClass
        + Inherits<EditorExportPlugin>
        + Inherits<Object>
        + IEditorExportPlugin
        + Bounds<Memory = MemRefCounted>
        + GodotDefault,
{
    fn _int_remaps(&mut self) -> &mut Vec<RemapData>;

    fn _int_ron_to_bin_change_path(path: GString) -> GString {
        let mut stringified = path.to_string();

        stringified = stringified.replace(".gdron", "_ron_remap");
        stringified.push_str(".gdbin");

        GString::from(stringified)
    }

    fn _int_is_gdron(path: GString) -> bool {
        path.to_string().ends_with(".gdron")
    }

    fn _int_is_gdbin(path: GString) -> bool {
        path.to_string().ends_with(".gdbin")
    }

    fn _int_process_ron_file<T>(
        &mut self,
        ron_path: GString,
        bin_path: GString,
    ) -> PackedByteArray
    where
        T: GdProp,
    {
        let mut loader = ResourceLoader::singleton();
        let res = loader.load(ron_path.clone()).expect("can't get ron file");

        let remap_data = RemapData::new(&ron_path, &bin_path);
        remap_data.transfer_uid();
        save(res, bin_path.clone());

        self._int_remaps().push(remap_data);

        FileAccess::get_file_as_bytes(bin_path.clone())
    }

    fn _int_read_file_to_bytes(path: GString) -> Option<PackedByteArray> {
        if let Ok(mut file) = GFile::open(path, ModeFlags::READ) {
            let mut buf = Vec::with_capacity(file.length() as usize);
            let result = file.read_to_end(&mut buf);

            if let Err(err) = result {
                godot_error!("Error while reading file: {err}");
                return None;
            }

            let mut array = PackedByteArray::new();
            array.extend(buf);
            return Some(array);
        }
        None
    }

    fn _int_export_begin(&mut self) {
        self._int_remaps().clear();
    }

    fn _int_export_end(&mut self) {
        while let Some(remap) = self._int_remaps().pop() {
            remap.undo_uid();
            DirAccess::remove_absolute(remap.bin_path.clone());
        }
    }
}

pub struct RemapData {
    ron_path: GString,
    bin_path: GString,
    uid: i64,
}

impl RemapData {
    pub (crate) fn new(ron_path: &GString, bin_path: &GString) -> Self {
        let mut loader = ResourceLoader::singleton();
        let uid = loader.get_resource_uid(ron_path.clone());

        Self {
            ron_path: ron_path.clone(),
            bin_path: bin_path.clone(),
            uid
        }
    }

    pub (crate) fn transfer_uid(&self) {
        self.change_uid(self.bin_path.clone())
    }

    pub (crate) fn undo_uid(&self) {
        self.change_uid(self.ron_path.clone())
    }

    fn change_uid(&self, path: GString) {
        let mut resource_uid = ResourceUid::singleton();

        let existing_id = resource_uid.has_id(self.uid);
    
        if existing_id {
            resource_uid.add_id(self.uid, path)
        } else {
            resource_uid.set_id(self.uid, path)
        }
    }
}