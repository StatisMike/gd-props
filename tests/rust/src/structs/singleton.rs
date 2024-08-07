use godot::classes::{Engine, Object};
use godot::obj::bounds::MemRefCounted;
use godot::obj::{Bounds, Gd, Inherits, UserClass};
use godot::prelude::GodotClass;

// Currently unused in tests as benchmarking was improved.
pub(crate) trait GodotSingleton
where
    Self: GodotClass + UserClass + Inherits<Object> + Bounds<Memory = MemRefCounted>,
{
    const SINGLETON_NAME: &'static str;

    fn singleton_instance() -> Gd<Self>;

    fn singleton() -> Gd<Self> {
        let mut engine = Engine::singleton();
        if engine.has_singleton(Self::SINGLETON_NAME.into()) {
            engine
                .get_singleton(Self::SINGLETON_NAME.into())
                .expect("no singleton found")
                .cast()
        } else {
            let object = Self::singleton_instance();
            engine.register_singleton(Self::SINGLETON_NAME.into(), object.clone().upcast());
            // TODO: Remove after https://github.com/godot-rust/gdext/issues/522 is fixed.
            std::mem::forget(object);
            engine
                .get_singleton(Self::SINGLETON_NAME.into())
                .expect("no singleton found")
                .cast()
        }
    }
}
