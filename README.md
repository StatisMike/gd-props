# gd-props
![tests workflow](https://github.com/StatisMike/gd-props/actions/workflows/tests.yaml/badge.svg)
[![Latest compatible gdext](https://byob.yarr.is/StatisMike/gd-props/gdext_latest_success)](https://github.com/godot-rust/gdext)

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
- `serde_gd` module containing submodules to be used with `serde`, making it easier to implement `Serialize` and `Deserialize` for your 
  custom resources.
- `gd_props_plugin` macro, which handles:
  - setting up `ResourceFormatSaver` and `ResourceFormatSaver` to handle `.gdron` and `.gdbin` formats.
  - setting up `EditorPlugin` and `EditorExportPlugin` to handle export of `.gdron` and `.gdbin` formats.
    - during export, all `.gdron` files are transformed into `.gdbin`, as the later is more compact and much faster to load. 

## In Development

> **This crate is not production-ready** ⚠️
>
> This crate is early in development, and its API may change. Contributions, discussions, and informed opinions are very welcome.

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
- `gd_props::serde_gd::gd` - for `Gd<T>` fields,
- `gd_props::serde_gd::gd_array` - for `Array<Gd<T>>` fields,
- `gd_props::serde_gd::gd_hashmap` - for `HashMap<K, Gd<T>` fields.

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

- `gd_props::serde_gd::ext_option` - designed for `Option<Gd<T>>` fields,
- `gd_props::serde_gd::ext` - designed for `Gd<T>` fields,
- `gd_props::serde_gd::ext_array` - for `Array<Gd<T>>` fields,
- `gd_props::serde_gd::ext_hashmap` - for `HashMap<K, Gd<T>` fields.

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

## GdProp tooling

Now that we have Rust resources fully serializable to `.gdron` and `.gdprop`, the next step is to provide tools for saving and loading 
them within the Godot engine. The default `ResourceSaver` and `ResourceLoader` are unaware of our `.gdron` and `.gdbin` files.

Also, by default the files won't be included into exported game, and actually we want to have a say in how they should be
exported.

To automatically define all tool GodotClass needed to handle introduced formats, the `#[gd_props_plugin]` macro should be used.
Below example that creates all needed tools and register two `GdProps`-annotated resources to be recognized by them.

```rust
use godot::prelude::*;
use gd_props::gd_props_plugin;

// Macro creates four different GodotClasses and registers two resources implementing `GdProp`
#[gd_props_plugin]
#[register(CharacterData, Statistics)]
pub(crate) struct PropPlugin;
 
// Plugin and Exporter are only available in-editor for exporting resources.
assert_eq!(PropPlugin::INIT_LEVEL, InitLevel::Editor);
assert_eq!(PropPluginExporter::INIT_LEVEL, InitLevel::Editor);

// Loader and Saver are available in scenes for loading/saving registered resources.
assert_eq!(PropPluginSaver::INIT_LEVEL, InitLevel::Scene);
assert_eq!(PropPluginLoader::INIT_LEVEL, InitLevel::Scene);
```

### Custom Format Saving and Loading with `GdProp`

After above, all that is left for Godot Editor to use our new `ResourceFormatSaver` and `ResourceFormatLoader` is to register them upon loading out `gdextension` to Godot's `ResourceSaver` and `ResourceLoader`, respectively. It can be achieved with provided associated methods
in `GdPropSaver` and `GdPropLoader` traits.

```rust
// lib.rs
use godot_io::traits::{GdPropLoader, GdPropSaver};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(_level: InitLevel) {
        if _level == InitLevel::Scene {
            PropPluginLoader::register_loader();
            PropPluginSaver::register_saver();
        }
    }
    // And we need to unregister them when editor is closing!
    fn on_level_deinit(deinit: InitLevel) {
       if deinit == InitLevel::Scene {
           PropPluginLoader::unregister_loader();
           PropPluginSaver::unregister_saver();
       }
    }
}
```

### Custom format export

Contrary to Loader and Saver, just a definition of `EditorPlugin` GodotClass is enough to handle the resources
on export and no extra steps are needed. Besides adding the custom format resources into the exported executable, 
all resources in `.gdron` format will be translated into `.gdbin`, as main reason for the former (being human-readible)
isn't needed anymore, and the later is more concise and faster to load. 

As comparison from `gd-rehearse` run shows, the difference is meaningiful, so currently there is no way
to opt-out of the conversion.

On debug (both Godot and Rust) build, where resources with paths ending with `.gdron` are saved/loaded as `.gdron` files.

```
--------------------------------------------------------------------------------
   Running Rust benchmarks
--------------------------------------------------------------------------------
                                              min       median
   gdbin.rs:
   -- serialize                  ...     27.260μs     29.396μs
   -- deserialize                ...     73.402μs     73.855μs
   -- gdbin_save                 ...    350.893μs    360.174μs
   -- gdbin_load                 ...    228.172μs    230.172μs

   gdron.rs:
   -- serialize                  ...     37.871μs     40.088μs
   -- deserialize                ...     82.897μs     83.356μs
   -- gdron_save                 ...    492.388μs    502.603μs
   -- gdron_load                 ...    979.330μs    988.216μs
```

On release (both Godot and Rust) build, where resources with paths ending with `.gdron` are remapped to `.gdbin`, the 
times are similiar for both formats: slightly higher times on `gdron` are probably caused by Godot's remap system.

> Saving was omitted, as the `res://` path is unavailable while exported

```
--------------------------------------------------------------------------------
   Running Rust benchmarks
--------------------------------------------------------------------------------
                                              min       median
   gdbin.rs:
   -- serialize                  ...      3.853μs      5.548μs
   -- deserialize                ...      7.514μs      9.500μs
   -- gdbin_load                 ...    120.190μs    177.116μs

   gdron.rs:
   -- serialize                  ...      5.105μs      6.051μs
   -- deserialize                ...      7.113μs      8.318μs
   -- gdron_load                 ...    140.451μs    205.902μs
