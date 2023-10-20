use godot::{prelude::{GodotClass, Object, Inherits, Gd}, obj::dom, engine::Engine};

pub trait SingletonGodotClass
where Self: GodotClass<Declarer = dom::UserDomain> + Inherits<Object>
{
    const SINGLETON_NAME: &'static str;

    fn struct_init() -> Gd<Self>;

    fn singleton() -> Gd<Self> {
        if Engine::singleton().has_singleton(Self::SINGLETON_NAME.into()) {

            Engine::singleton().get_singleton(Self::SINGLETON_NAME.into()).unwrap().cast::<Self>()

        } else {

            let object = Self::struct_init();
            Engine::singleton().register_singleton(Self::SINGLETON_NAME.into(),object.clone().upcast());
            object
        }
    }
}