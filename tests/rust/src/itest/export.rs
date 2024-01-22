use gd_rehearse::{itest::gditest, CaseContext};
use godot::{
    builtin::{GString, NodePath},
    engine::NodeExt,
};

use crate::structs::node::ExportTestNode;

#[gditest(scene_path = "res://export_test.tscn")]
fn exported_can_retrieve_node(ctx: &CaseContext) {
    let node = ctx
        .scene_tree()
        .try_get_node_as::<ExportTestNode>(NodePath::from("ExportTestNode"));
    assert!(node.is_some());
}

#[gditest(scene_path = "res://export_test.tscn")]
fn exported_bundled_works(ctx: &CaseContext) {
    let node = ctx
        .scene_tree()
        .try_get_node_as::<ExportTestNode>(NodePath::from("ExportTestNode"))
        .unwrap();

    let bundled = node.bind().get_bundle_res().clone();

    let first = bundled.bind().get_first().clone();
    let set = first.bind().get_set().clone();
    let set_vec = set.iter().collect::<Vec<_>>();
    assert_eq!(set_vec.len(), 1, "wrong first bundled set length!");

    let inner_thing = set_vec.get(0).unwrap();
    assert_eq!(inner_thing.character, 'N');
    assert_eq!(inner_thing.int, -125);

    let vec = first.bind().get_vec().clone();
    assert_eq!(vec.len(), 1, "wrong first bundled vec length!");

    let inner_thing = vec.get(0).unwrap();
    assert_eq!(inner_thing.character, 'K');
    assert_eq!(inner_thing.int, -173);

    let second = bundled
        .bind()
        .get_second()
        .clone()
        .expect("cannot unwrap second resource");
    let set = second.bind().get_set().clone();
    let set_vec = set.iter().collect::<Vec<_>>();
    assert_eq!(set_vec.len(), 1, "wrong second bundled set length!");

    let inner_thing = set_vec.get(0).unwrap();
    assert_eq!(inner_thing.character, 'A');
    assert_eq!(inner_thing.int, 2137);

    let vec = second.bind().get_vec().clone();
    assert_eq!(vec.len(), 1, "wrong second bundled vec length!");

    let inner_thing = vec.get(0).unwrap();
    assert_eq!(inner_thing.character, 'Z');
    assert_eq!(inner_thing.int, -2137);
}

#[gditest(scene_path = "res://export_test.tscn")]
fn exported_ext_works(ctx: &CaseContext) {
    let node = ctx
        .scene_tree()
        .try_get_node_as::<ExportTestNode>(NodePath::from("ExportTestNode"))
        .unwrap();

    let bundled = node.bind().get_ext_res().clone();

    let first = bundled.bind().get_first().clone();
    let set = first.bind().get_set().clone();
    let set_vec = set.iter().collect::<Vec<_>>();
    assert_eq!(set_vec.len(), 1, "wrong first bundled set length!");

    let inner_thing = set_vec.get(0).unwrap();
    assert_eq!(inner_thing.character, 'A');
    assert_eq!(inner_thing.int, 2137);

    let vec = first.bind().get_vec().clone();
    assert_eq!(vec.len(), 1, "wrong first bundled vec length!");

    let inner_thing = vec.get(0).unwrap();
    assert_eq!(inner_thing.character, 'Z');
    assert_eq!(inner_thing.int, -2137);

    let second = bundled
        .bind()
        .get_second()
        .clone()
        .expect("cannot unwrap second resource");
    assert_eq!(second.bind().get_int(), -623);
    assert_eq!(second.bind().get_str(), GString::from("XEJLHWGHHB"));
}
