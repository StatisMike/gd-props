use std::io::Read;

use godot::builtin::{GString, PackedByteArray};
use godot::engine::file_access::ModeFlags;
use godot::engine::{
    save, DirAccess, EditorExportPlugin, FileAccess, GFile, IEditorExportPlugin, Object,
    ResourceLoader, ResourceUid,
};
use godot::log::godot_error;
use godot::obj::bounds::MemRefCounted;
use godot::obj::cap::GodotDefault;
use godot::obj::{Bounds, GodotClass, Inherits, UserClass};

use crate::gdprop::GdProp;

#[derive(Default)]
#[doc(hidden)]
pub struct ExporterState {
    remaps: Vec<RemapData>,
    debug: bool
}

/// Trait containing most of the logic necessary for [EditorExportPlugin] to be able to handle
/// `gd_props` custom resource formats upon `Godot` project export.
/// 
/// Not necessary for users to implement this trait themselves. Use `#[gd_props_plugin]` macro
/// for implementation of all necessary structs. 
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
    #[doc(hidden)]
    fn _int_state_mut(&mut self) -> &mut ExporterState;
    
    #[doc(hidden)]
    fn _int_remaps(&mut self) -> &mut Vec<RemapData> {
        &mut self._int_state_mut().remaps
    }

    #[doc(hidden)]
    fn _int_debug(&mut self) -> &mut bool {
        &mut self._int_state_mut().debug
    }

    #[doc(hidden)]
    fn _int_ron_to_bin_change_path(path: GString) -> GString {
        let stringified = path.to_string();
        let replace_index = stringified.len() - ".gdron".len();
        GString::from(&format!(
            "{}{}",
            &stringified[..replace_index],
            "_from_ron.gdbin"
        ))
    }

    #[doc(hidden)]
    fn _int_is_gdron(path: GString) -> bool {
        path.to_string().ends_with(".gdron")
    }

    #[doc(hidden)]
    fn _int_is_gdbin(path: GString) -> bool {
        path.to_string().ends_with(".gdbin")
    }

    // #[doc(hidden)]
    // fn _int_verify_processable(
    //     &self,
    //     type_: GString,
    //     resource: Gd<Resource>
    // ) {
    //     let loader_extensions = ResourceLoader::singleton().get_recognized_extensions_for_type(type_.clone());
    //     let saver_extensions = ResourceSaver::singleton().get_recognized_extensions(resource);

    //     if !GdPropFormat::verify_supported_extensions(&loader_extensions) {
    //         godot_warn!("ResourceLoader doesn't recognize GdProp file extensions for resource of type: {type_}. Make sure that custom format loader have been registered");
    //     }
    //     if !GdPropFormat::verify_supported_extensions(&saver_extensions) {
    //         godot_warn!("ResourceSaver doesn't recognize GdProp file extensions for resource of type: {type_}. Make sure that custom format saver have been registered");
    //     }
    // }

    #[doc(hidden)]
    fn _int_process_ron_file<T>(&mut self, ron_path: GString, bin_path: GString) -> PackedByteArray
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

    #[doc(hidden)]
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

    #[doc(hidden)]
    fn _int_export_begin(&mut self, is_debug: bool) {
        self._int_remaps().clear();
        *self._int_debug() = is_debug;
    }

    #[doc(hidden)]
    fn _int_export_end(&mut self) {
        while let Some(remap) = self._int_remaps().pop() {
            remap.undo_uid();
            DirAccess::remove_absolute(remap.bin_path.clone());
        }
    }
}

#[doc(hidden)]
pub struct RemapData {
    ron_path: GString,
    bin_path: GString,
    uid: i64,
}

impl RemapData {
    pub(crate) fn new(ron_path: &GString, bin_path: &GString) -> Self {
        let mut loader = ResourceLoader::singleton();
        let uid = loader.get_resource_uid(ron_path.clone());

        Self {
            ron_path: ron_path.clone(),
            bin_path: bin_path.clone(),
            uid,
        }
    }

    pub(crate) fn transfer_uid(&self) {
        self.change_uid(self.bin_path.clone())
    }

    pub(crate) fn undo_uid(&self) {
        self.change_uid(self.ron_path.clone())
    }

    fn change_uid(&self, path: GString) {
        let mut resource_uid = ResourceUid::singleton();

        let existing_id = resource_uid.has_id(self.uid);

        if existing_id {
            resource_uid.set_id(self.uid, path)
        } else {
            resource_uid.add_id(self.uid, path)
        }
    }
}
