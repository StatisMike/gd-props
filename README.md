# godot_io
Creating custom Godot resources with [godot-rust](https://github.com/godot-rust/gdext) and using them in the Godot Editor is fun and useful. There are some drawbacks to the process out of the box, though.

Godot default `ResourceSaver` and `ResourceLoader` can only handle `exported` fields of your resources. These needs to be recognized by Godot editor - so Godot types. This can be cumbersome if you want to save some more complex state inside your resource.

This crate is born from this frustration and its goal is to provide tools to save rust-created Resources straight to and from custom format.

## In Development
> **This crate is not production ready** ⚠️
>
> This crate is early in development and its API can certainly change. Contributions, discussions and informed opinions are very welcome.

Features that will be certainly expanded upon:
- add support for more compact formats, like binary and binary compressed
- make the `gdron` format interchangeable with future binary/binary compressed (for release mode)
- make everything work smoothly in compiled game (especially pointers to `.tres` resources)

## GdRonResource macro
So, imagine that you have a Resouce with a structure similiar to one below. You could potentially transform `HashMap` into Godot's Dictionary, but you would also sacrifice some of its pros. Though other structs, like `StatModifiers` which you don't intend to handle like a `Resource` is sure to be lost if saving resource with Godot's resource saver.
```rust
#[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
#[class(base=Resource)]
pub struct Statistics {
  /// Current character level
  #[export]
  pub level: u32,
  /// All stats
  pub stats: HashMap<GeneralStat, usize>,
  /// Experience currently gained by the character. Every 100 experience points grants a level up with the chance of increasing stats
  pub exp: usize,
  /// Amount of bane needed to be applied to the character - the higher, the more *boons* it amassed
  pub bane: usize,
  /// Modifiers from [StatModEffect]. Key is the number of turns left, while value is the stat modifiers
  pub effect_mods: HashMap<usize, StatModifiers>,
  /// Modifiers from equipped items. Key is the index of the item
  pub item_mods: HashMap<usize, StatModifiers>,
  /// Modifiers from character class
  pub class_mods: StatModifiers,
}
```
I presume that you saw the `GdRonResource` derive macro there, though. It implements `GdRonResource` trait, and makes the Resource saveable with our custom saver straight to `.gdron` file.

`.gdron` is a very slightly modified `ron` file - it's only change is an inclusion of a header containing the struct identifier (or resource type identifier in Godot terms). For a random object of above structure it would look like that:

```
(gd_class:"Statistics",uid:"uid://bwgy4ec84b8xv")
(
    level: 3,
    stats: {
        Mv: 0,
        Lck: 7,
        Def: 7,
        Mag: 7,
        Agi: 9,
        HP: 28,
        Res: 3,
        Dex: 7,
        Str: 7,
    },
    exp: 0,
    bane: 4,
    effect_mods: {},
    item_mods: {},
    class_mods: (
        x: {},
    ),
)
```
File is recognizable by Godot editor, could be loaded through it and attached to a node or other Resource.

## Bundled resources
What if we have a Resource which contains another resource, which we would want to save as a bundled resource? There are two modules that handle this case: 
- `godot_io::serde_gd::gd_option` - for `Option<Gd<T>>` fields
- `godot_io::serde_gd::gd` - for `Gd<T>` fields.

There are some requirements for this to work:
- `T` needs to be User-defined `GodotClass` inheriting from `Resource`
- `T` needs to derive `Serialize` and `Deserialize`
  
### Example
```rust
#[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
#[class(base=Resource)]
pub struct CharacterData {
    #[export]
    affiliation: CharacterAffiliation,
    #[export]
    #[serde(with="godot_io::serde_gd::gd_option")]
    statistics: Option<Gd<Statistics>>,
}
```
Upon saving, we receive file as below:
```
(gd_class:"CharacterData",uid:"uid://dfa37uvpqlnhq")
(
    affiliation: Player,
    statistics: Some((
        level: 3,
        stats: {
            Def: 7,
            Dex: 7,
            Lck: 7,
            Mag: 7,
            Res: 3,
            Mv: 0,
            HP: 28,
            Agi: 9,
            Str: 7,
        },
        exp: 0,
        bane: 4,
        effect_mods: {},
        item_mods: {},
        class_mods: (
            x: {},
        ),
    )),
)
```

## External Resources
All right, but what if we would like to preserve the sub resource as an External Resource, just in a way that regular resource saving in Godot works? It is possible with two additional modules:

- `godot_io::serde_gd::ext_option` - for `Option<Gd<T>>` fields
- `godot_io::serde_gd::ext` - for `Gd<T>` fields.

There are some requirements for this to work:
- `T` needs to be a `Resource`
- `T` needs to be a standalone `Resource` (needs to be saved to a file and loadable from it)

This approach has numerous benefits:
- `T` doesn't need to be a User-defined `GodotClass` (so built-in resources will work)
- External Resource instance will be reused whenever it is referenced

### Example
```rust
#[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
#[class(base=Resource)]
pub struct CharacterData {
    #[export]
    affiliation: CharacterAffiliation,
    // As `statistics` is User-defined Resource, we could also use `gd_option` module
    #[export]
    #[serde(with="godot_io::serde_gd::ext_option")]
    statistics: Option<Gd<Statistics>>,
    #[export]
    #[serde(with="godot_io::serde_gd::ext_option")]
    nothing_is_here: Option<Gd<Resource>>,
    #[export]
    #[serde(with="godot_io::serde_gd::ext_option")]
    texture: Option<Gd<CompressedTexture2D>>,
}
```
Upon saving, we receive file as below:
```
(gd_class:"CharacterData",uid:"uid://dfa37uvpqlnhq")
(
    affiliation: Player,
    statistics: ExtResource((
        gd_class: "Statistics",
        uid: "uid://dixv2uvh8waug",
        path: "res://statistics.gdron",
    )),
    nothing_is_here: None,
    texture: ExtResource((
        gd_class: "CompressedTexture2D",
        uid: "uid://ci3y6557pn0o",
        path: "res://icon.svg",
    )),
)
```

## GdRonSaver and GdRonLoader macros
As we now have rust Resources fully Serializable to `.gdron`, we now need a tools for saving and loading them - default `ResourceSaver` and `ResourceLoader` don't know and won't recognize our `.gdron` files.

`godot_io` comes with `GdRonSaver` and `GdRonLoader` derive macro, that can be used to easily create `CustomFormatSaver` and `CustomFormatLoader` and register our Resources to them! Their syntax is very similiar:
```rust
// The derive itself
#[derive(GdRonSaver)]
// Attribute to register the GdRonResources to be handled by given Saver/Loader
#[register(CharacterData, Statistics)]
// Multiple `register` macro attributes could be provided, all identifiers contained within will be registered
#[register(AnotherGdResource)]
```
Full example - defining both Saver and Loader:
```rust
#[derive(GodotClass, GdRonSaver)]
#[class(base=ResourceFormatSaver, init, tool)]
#[register(CharacterData, Statistics)]
pub struct CustomRonSaver {}

#[godot_api]
impl CustomRonSaver {}

#[derive(GodotClass, GdRonLoader)]
#[class(base=ResourceFormatLoader, init, tool)]
#[register(CharacterData)]
#[register(Statistics)]
pub struct CustomRonLoader {}

#[godot_api]
impl CustomRonLoader {}
```
All that is left for Godot Editor to use our new `ResourceFormatSaver` and `ResourceFormatLoader` is to register them upon loading out `gdextension`:
```rust
// lib.rs
use godot_io::traits::{GdRonLoader, GdRonSaver};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(_level: InitLevel) {
        if _level == InitLevel::Scene {
            CustomRonLoader::register_loader();
            CustomRonSaver::register_saver();
        }
    }
}
```