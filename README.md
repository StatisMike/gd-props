# gd-props

> Resources are akin to the versatile props that set the scene for an interactive masterpiece on the stage of game world. 
> Much like actors skillfully employ props to enrich their storytelling, game objects leverage resources to craft a compelling virtual 
> narrative. The mastery lies in the thoughtful selection and optimization of these digital tools, guaranteeing a captivating 
> performance as players step into the spotlight of the gaming world.

Custom resources created with [godot-rust](https://github.com/godot-rust/gdext) can be very useful. However, as the game data becomes 
complex, the workflow available out of the box can be quite limiting. By default, Godot saves all resources into `.tres` and `.res` files, 
preserving only the state of fields marked with `#[export]`. This limitation confines the saving of state only to fields converted to 
Godot-compatible types.

`gd-props` aims to address this issue by providing an alternative strategy for loading and saving resources, relying fully on `serde` 
serialization and deserialization. Resources can be saved in two formats:

- `.gdron`: Based on the `Ron` format from the `ron` crate. Intended for human-readable output during development.
- `.gdbin`: Based on the `MessagePack` format from the `rmp_serde` crate. Intended for faster serialization and deserialization 
- times, especially in exported games.

## Current Features

The following features are currently available. More will be listed in the `In Development` section.

- `GdProp` derive macro for custom `Resource`s, making them savable to `.gdron` and `.gdbin` formats.
- `GdPropSaver` and `GdPropLoader` macros for easily implementing `CustomFormatSaver` and `CustomFormatLoader` for `.gdron` and `.gdbin` formats.
- `serde_gd` module containing submodules to be used with `serde`, making it easier to implement `Serialize` and `Deserialize` for your 
  custom resources.

## In Development

> **This crate is not production-ready** ⚠️
>
> This crate is early in development, and its API may change. Contributions, discussions, and informed opinions are very welcome.

Features that will certainly be expanded upon:

- Provide more submodules in the `serde_gd` module to support iterable `Gd<Resouce>` collections.
- Make the `gdron` and `gdbin` formats interchangeable for release mode with a custom `EditorExportPlugin`.
- Ensure everything works smoothly in compiled games, especially pointers to `.tres` resources after they are changed into `.res` format.

## GdProp macro
Consider a scenario where you have a resource with a structure similar to the one below. You might contemplate transforming a `HashMap` 
into Godot's Dictionary, but this conversion could entail sacrificing some of its advantages. On the other hand, for structs like 
`StatModifiers` that you don't intend to handle as a `Resource`, there is a risk of loss when saving the resource with Godot's `ResourceSaver`.


```rust
#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(base=Resource)]
pub struct Statistics {
  /// Current character level - only available fully on Godot editor side. Rest can be accessed by other Rust GodotClasses.
  #[var]
  pub level: u32,
  /// All stats
  pub stats: HashMap<GeneralStat, usize>,
  /// Experience currently gained by the character. Every 100 experience points grants a level up with the chance of increasing stats.
  pub exp: usize,
  /// Amount of bane needed to be applied to the character - the higher, the more *boons* it amassed.
  pub bane: usize,
  /// Modifiers from [StatModEffect]. Key is the number of turns left, while value is the stat modifiers.
  pub effect_mods: HashMap<usize, StatModifiers>,
  /// Modifiers from equipped items. Key is the index of the item.
  pub item_mods: HashMap<usize, StatModifiers>,
  /// Modifiers from character class
  pub class_mods: StatModifiers,
}
```
`GdProp` derive macro implements `GdProp` trait, and makes the Resource saveable with our `gd-props` straight to `.gdron` and `.gdbin` file.

The `.gdron` format is a slightly modified `Ron` file, distinguished by the inclusion of a header containing the struct identifier or 
resource class name. For a random object of the above structure, the resulting file might look like this:


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

The header, in this case, contains `Statistics`, signifying the class name of the serialized struct. This format is designed for 
human-readable output during development, aiding in easy inspection and modification of the saved resources. Additionally,
Godot's `uid` path is also preserved there.

On the other hand, the `.gdbin` format is based on the `MessagePack` format from the `rmp_serde` crate. It is intended for faster 
serialization and deserialization times, especially in exported games, and in other aspects is analogous to `.gdron`.

Both formats, whether human-readable or optimized for performance, offer the flexibility to choose the serialization strategy 
that best suits your development and deployment needs.

Both file are recognizable by Godot editor, can be loaded through it and attached to some Godot class.

## Bundled resources
What if we have a Resource which contains another resource, which we would want to save as a bundled resource? There are two modules that handle this case: 
- `gd_props::serde_gd::gd_option` - for `Option<Gd<T>>` fields,
- `gd_props::serde_gd::gd` - for `Gd<T>` fields.

There are some requirements for this to work:
- `T` needs to be User-defined `GodotClass` inheriting from `Resource`,
- `T` needs to derive `Serialize` and `Deserialize`.
  
### Example
```rust
#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(base=Resource)]
pub struct CharacterData {
    #[export]
    affiliation: CharacterAffiliation,
    #[export]
    #[serde(with="gd_props::serde_gd::gd_option")]
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
If you desire to preserve a sub-resource as an External Resource, akin to regular resource saving in Godot, `gd-props` provides two additional modules:

- `gd_props::serde_gd::ext_option` - designed for `Option<Gd<T>>` fields.
- `gd_props::serde_gd::ext` - designed for `Gd<T>` fields.

To enable this functionality, a few requirements must be met:

- `T` needs to be a `Resource`.
- `T` must be a standalone `Resource` and be savable to and loadable from a file.

This approach offers several advantages:

- `T` doesn't necessarily need to be a User-defined `GodotClass`, making it compatible with built-in resources.
- External Resource instances are reused whenever they are referenced, enhancing efficiency and reducing redundancy in the game data.

### Example
```rust
#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(base=Resource)]
pub struct CharacterData {
    #[export]
    affiliation: CharacterAffiliation,
    // As `statistics` is User-defined Resource, so we could also use `gd_option` module to bundle the Resource.
    #[export]
    #[serde(with="gd_props::serde_gd::ext_option")]
    statistics: Option<Gd<Statistics>>,
    #[export]
    #[serde(with="gd_props::serde_gd::ext_option")]
    nothing_is_here: Option<Gd<Resource>>,
    #[export]
    #[serde(with="gd_props::serde_gd::ext_option")]
    texture: Option<Gd<CompressedTexture2D>>,
}
```
Upon saving to `.gdron` format we receive file as below:
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

## Custom Format Saving and Loading with `GdProp`

Now that we have Rust resources fully serializable to `.gdron` and `.gdprop`, the next step is to provide tools for saving and loading 
them within the Godot engine. The default `ResourceSaver` and `ResourceLoader` are unaware of our `.gdron` files.

`gd-props` introduces two powerful derive macros, `GdPropSaver` and `GdPropLoader`, designed to simplify the creation of `CustomFormatSaver` 
and `CustomFormatLoader` for formats it introduces. These macros enable the seamless registration of our resources to the Godot engine, 
ensuring compatibility with the `.gdron` and `.gdbin` formats.

The syntax for both macros is quite similar:
```rust
// The derive itself
#[derive(GdPropSaver)]
// Attribute to register the GdRonResources to be handled by given Saver/Loader
#[register(CharacterData, Statistics)]
// Multiple `register` macro attributes could be provided, all identifiers contained within will be registered
#[register(AnotherGodotResource)]
```
Full example - defining both Saver and Loader:
```rust
#[derive(GodotClass, GdPropSaver)]
#[class(base=ResourceFormatSaver, init, tool)]
#[register(CharacterData, Statistics)]
pub struct CustomRonSaver {}


#[derive(GodotClass, GdPropLoader)]
#[class(base=ResourceFormatLoader, init, tool)]
#[register(CharacterData)]
#[register(Statistics)]
pub struct CustomPropLoader {}
```
All that is left for Godot Editor to use our new `ResourceFormatSaver` and `ResourceFormatLoader` is to register them upon loading out 
`gdextension` to Godot's `ResourceSaver` and `ResourceLoader`, respectively. It can be achieved with provided associated methods
in `GdPropSaver` and `GdPropLoader` traits.
```rust
// lib.rs
use godot_io::traits::{GdPropLoader, GdPropSaver};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(_level: InitLevel) {
        if _level == InitLevel::Scene {
            CustomPropLoader::register_loader();
            CustomPropSaver::register_saver();
        }
    }
}
```