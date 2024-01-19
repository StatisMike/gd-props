use std::io::Read;

use godot::builtin::{GString, PackedByteArray};
use godot::engine::file_access::ModeFlags;
use godot::engine::{
    EditorExportPlugin, GFile, IEditorExportPlugin, Object, ResourceLoader,
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
    fn _int_ron_to_bin_change_path(path: GString) -> GString {
        let mut stringified = path.to_string();

        stringified = stringified.replace(".gdron", ".gdbin");

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
        path: GString,
        _type_: GString,
    ) -> Option<PackedByteArray>
    where
        T: GdProp,
    {
        if let Some(res) = ResourceLoader::singleton().load(path.clone()) {
            let mut buf = Vec::new();
            let mut serializer = rmp_serde::Serializer::new(&mut buf);
            let result = res.cast::<T>().bind().serialize(&mut serializer);

            if let Err(err) = result {
                godot_error!("Error while serializing to gdbin: {err}");
            }

            let mut array = PackedByteArray::new();
            array.extend(buf);
            return Some(array);
        }
        None
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
}
