use std::collections::HashMap;
use std::io::BufWriter;

use gd_props::types::GdResVec;
use gd_rehearse::itest::gditest;
use godot::builtin::GString;
use godot::engine::ResourceLoader;
use godot::obj::Gd;

use ron::Serializer;
use serde::Serialize;

use crate::remove_file;
use crate::structs::resource::*;

#[gditest]
fn serde_bundled() {
    let resource = WithBundledGd::new();

    let mut buffer = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(BufWriter::new(&mut buffer));

    let result = resource.serialize(&mut serializer);
    assert!(result.is_ok());
    drop(serializer);

    let result = rmp_serde::from_slice::<WithBundledGd>(&buffer);
    assert!(result.is_ok());
    let deserialized = result.unwrap();

    assert!(TestResource::check_set_eq(
        resource.first.bind().get_set(),
        deserialized.first.bind().get_set()
    ));
    assert!(TestResource::check_vec_eq(
        resource.first.bind().get_vec(),
        deserialized.first.bind().get_vec()
    ));

    assert!(TestResource::check_set_eq(
        resource.second.clone().unwrap().bind().get_set(),
        deserialized.second.clone().unwrap().bind().get_set()
    ));
    assert!(TestResource::check_vec_eq(
        resource.second.unwrap().bind().get_vec(),
        deserialized.second.unwrap().bind().get_vec()
    ));
}

#[gditest]
fn serde_bundled_recvec() {
    let resource = WithBundleHashMap::new(5);
    let mut buffer = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(BufWriter::new(&mut buffer));

    let result = resource.serialize(&mut serializer);
    assert!(result.is_ok());
    drop(serializer);

    let result = rmp_serde::from_slice::<WithBundleHashMap>(&buffer);
    assert!(result.is_ok());
    let deserialized = result.unwrap();

    let keys = resource.map.keys();
    for key in keys {
        assert!(TestResource::check_set_eq(
            resource.map.get(key).unwrap().bind().get_set(),
            deserialized.map.get(key).unwrap().bind().get_set()
        ));
        assert!(TestResource::check_vec_eq(
            resource.map.get(key).unwrap().bind().get_vec(),
            deserialized.map.get(key).unwrap().bind().get_vec()
        ));
    }
}

#[gditest]
fn serde_bundled_hashmap() {
    let resource = WithBundleHashMap::new(5);
    let mut buffer = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(BufWriter::new(&mut buffer));

    let result = resource.serialize(&mut serializer);
    assert!(result.is_ok());
    drop(serializer);

    let result = rmp_serde::from_slice::<WithBundleHashMap>(&buffer);
    assert!(result.is_ok());
    let deserialized = result.unwrap();

    let keys = resource.map.keys();
    for key in keys {
        assert!(TestResource::check_set_eq(
            resource.map.get(key).unwrap().bind().get_set(),
            deserialized.map.get(key).unwrap().bind().get_set()
        ));
        assert!(TestResource::check_vec_eq(
            resource.map.get(key).unwrap().bind().get_vec(),
            deserialized.map.get(key).unwrap().bind().get_vec()
        ));
    }
}

#[gditest]
fn serde_external() {
    let mut loader = ResourceLoader::singleton();

    let test_res = loader
        .load(GString::from("res://ext_test/test_resource.gdron"))
        .unwrap()
        .cast::<TestResource>();
    let test_godot_res = loader
        .load(GString::from("res://ext_test/test_godot_res.tres"))
        .unwrap()
        .cast::<TestGodotResource>();

    let resource = Gd::<WithExtGd>::from_object(WithExtGd {
        first: test_res,
        second: Some(test_godot_res),
    });

    let mut buffer = Vec::new();
    let mut serializer = ron::Serializer::new(BufWriter::new(&mut buffer), None).unwrap();

    let result = resource.bind().serialize(&mut serializer);
    if let Err(error) = &result {
        println!("{}", error);
    }
    assert!(result.is_ok());
    drop(serializer);

    let result = ron::de::from_bytes::<WithExtGd>(&buffer);
    if let Err(error) = &result {
        println!("{}", error);
    }
    assert!(result.is_ok());
    let deserialized = result.unwrap();

    assert_eq!(
        deserialized.first.get_path(),
        GString::from("res://ext_test/test_resource.gdron")
    );
    assert_eq!(
        deserialized.second.unwrap().get_path(),
        GString::from("res://ext_test/test_godot_res.tres")
    );
}

#[gditest]
fn serde_external_resvec() {
    let path = "res://";
    let subresources = TestGodotResource::new_saved_multiple(path, 5);
    let mut vec = GdResVec::default();
    for (_, subresource) in subresources.iter() {
        vec.push(subresource.clone());
    }
    let resource = WithExtResVec { vec };

    let mut buffer = Vec::new();
    let mut serializer = Serializer::new(&mut buffer, None).unwrap();

    let result = resource.serialize(&mut serializer);
    assert!(result.is_ok());
    drop(serializer);

    let result = ron::de::from_bytes::<WithExtResVec>(&buffer);
    assert!(result.is_ok());
    let deserialized = result.unwrap();

    let mut deserialized_hash = HashMap::new();

    for subresource in deserialized.vec.iter() {
        let res_path = subresource.get_path().to_string();
        let trimmed = res_path.trim_start_matches(path.clone());
        deserialized_hash.insert(trimmed.to_owned(), subresource.clone());
    }

    for (file, subresource) in subresources.iter() {
        let de_res = deserialized_hash.get(file).unwrap();

        assert_eq!(de_res.bind().int, subresource.bind().int);
        assert_eq!(de_res.bind().str, subresource.bind().str);

        remove_file(path, file);
    }
}

#[gditest]
fn serde_external_hashmap() {
    let path = "res://";
    let subresources = TestGodotResource::new_saved_multiple(path, 5);
    let mut map = HashMap::new();
    for (key, subresource) in subresources.iter() {
        map.insert(key.clone(), subresource.clone());
    }
    let resource = WithExtHashMap { map };

    let mut buffer = Vec::new();
    let mut serializer = Serializer::new(&mut buffer, None).unwrap();

    let result = resource.serialize(&mut serializer);
    assert!(result.is_ok());
    drop(serializer);

    let result = ron::de::from_bytes::<WithExtHashMap>(&buffer);
    assert!(result.is_ok());
    let deserialized = result.unwrap();

    for (file, subresource) in subresources.iter() {
        let de_res = deserialized.map.get(file).unwrap();

        assert_eq!(de_res.bind().int, subresource.bind().int);
        assert_eq!(de_res.bind().str, subresource.bind().str);

        remove_file(path, file);
    }
}
