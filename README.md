# godot_io
Creating custom Godot resources with [godot-rust](https://github.com/godot-rust/gdext) and using them in the Godot Editor is fun and useful. There are some drawbacks to the process out of the box, though.

Godot default `ResourceSaver` and `ResourceLoader` can only handle `exported` fields of your resources. These needs to be recognized by Godot editor - so Godot types. This can be cumbersome if you want to save some more complex state inside your resource.

This crate is born from this frustration and its goal is to provide tools to save rust-created Resources straight to and from custom format.

## In Development
> **This crate is not production ready** ⚠️
>
> This crate is early in development and its API can certainly change. Contributions, discussions and informed opinions are very welcome.

Features that will be certainly expanded upon:
- integrate into unique identifiers functionality of Godot
- make current forced `BUNDLE_RESOURCES` behaviour optional, and if so save nested resources as their `Uid` (at least for `.gdron` files)
- add support for more compact formats, like binary and binary compressed
 

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
I presume that you saw the `GdRonResource` derive macro there, though. It implements currently main useful trait of this crate. It makes the Resource saveable with our custom saver straight to `.gdron` file.

`.gdron` is a very slightly modified `ron` file - it's only change is an inclusion of a header containing the struct identifier (or resource type identifier in Godot terms). For a random object of above structure it would look like that:

```
gd=[Statistics]=
(
    level: 0,
    stats: {
        Dex: 7,
        Agi: 11,
        Str: 9,
        Lck: 7,
        HP: 14,
        Mv: 0,
        Mag: 5,
        Def: 3,
        Res: 7,
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
File is recognizable by Godot editor, could be loaded through it and attached to a node or other Resource. Which lead to another question: what if you want to preserve the `statistics` field alongside the whole `CharacterData` resource, like below?
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
`godot_io::serde_gd::gd_option` is a provided module that allows serializing the nested resource. When saving the `CharacterData` we will now get a file as below: 
```
gd=[CharacterData]=
(
    affiliation: Player,
    statistics: Some((
        level: 0,
        stats: {
            Def: 5,
            Res: 11,
            Agi: 7,
            Mv: 0,
            HP: 5,
            Str: 11,
            Lck: 7,
            Mag: 7,
            Dex: 5,
        },
        exp: 0,
        bane: 6,
        effect_mods: {},
        item_mods: {},
        class_mods: (
            x: {},
        ),
    )),
)
```
There is also `godot_io::serde_gd::gd` module, handling `Gd<GodotClass>` fields.

## GdRonSaver and GdRonLoader macros
As we now have rust Resources fully Serializable to `.gdron`, we now need a tools for saving and loading them - default `ResourceSaver` and `ResourceLoader` don't know and won't recognize our `.gdron` files.

`godot_io` comes with `GdRonSaver` and `GdRonLoader` derive macro, that can be used to easily create `CustomFormatSaver` and `CustomFormatLoader` and register our Resources to them! Their syntax is very similiar:
```rust
// The derive itself
#[derive(GdRonSaver)]
// Macro attribute to provide UidMap to hold the identifiers of Resources. Same UID map should be provided for both Saver and Loader
#[uid_map(MY_UID_MAP)]
// Attribute to register the GdRonResources to be handled by given Saver/Loader
#[register(CharacterData, Statistics)]
// Multiple `register` macro attributes could be provided, all identifiers contained within will be registered
#[register(AnotherGdResource)]
```
Full example - defining both Saver and Loader:
```rust
/// Attribute macro included :)
#[godot_io_uid_map]
static RON_UID: UidMap;

#[derive(GodotClass, GdRonSaver)]
#[class(base=ResourceFormatSaver, init, tool)]
#[uid_map(RON_UID)]
#[register(CharacterData, Statistics)]
pub struct CustomRonSaver {}

#[godot_api]
impl CustomRonSaver {}

#[derive(GodotClass, GdRonLoader)]
#[class(base=ResourceFormatLoader, init, tool)]
#[uid_map(RON_UID)]
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