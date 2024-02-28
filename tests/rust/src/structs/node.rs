use gd_rehearse::bench::BenchContext;
use godot::engine::{INode, Node};
use godot::obj::{Base, Gd, NewAlloc};
use godot::register::{godot_api, GodotClass};

use super::resource::{TestResource, WithBundledGd, WithExtGd};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ExportTestNode {
    #[export]
    bundle_res: Gd<WithBundledGd>,
    #[export]
    ext_res: Gd<WithExtGd>,
}

#[godot_api]
impl INode for ExportTestNode {
    fn init(_base: Base<Node>) -> Self {
        // "res://export_test/test_resource.gdron"
        let bundle_res = Gd::<WithBundledGd>::default();
        // "res://export_test/with_ext_gd.gdron"
        let ext_res = Gd::<WithExtGd>::default();

        Self {
            bundle_res,
            ext_res,
        }
    }
}

#[godot_api]
impl ExportTestNode {
    #[func]
    fn get_bundle(&self) -> Gd<WithBundledGd> {
        self.bundle_res.clone()
    }

    #[func]
    fn get_ext(&self) -> Gd<WithExtGd> {
        self.ext_res.clone()
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct TestResourceNode {
    pub(crate) res: Gd<TestResource>,
}

#[godot_api]
impl INode for TestResourceNode {
    fn init(_base: Base<Node>) -> Self {
        Self {
            res: TestResource::new_random(100, 100),
        }
    }
}

pub(crate) fn test_resource_setup(ctx: &mut BenchContext) {
    ctx.setup_add_node(TestResourceNode::new_alloc().upcast(), "TestResourceNode");
}
