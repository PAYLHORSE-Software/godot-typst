<p align="center">
<img src="https://github.com/paylhorse/godot-typst/assets/74363924/61433620-8126-46a4-8deb-39c7eac1c5f1" alt="logo" width="200"/>
</p>
<p align="center">
<b>Render Typst expressions in <a href="https://github.com/godotengine/godot">Godot 4</a></b>
</p>
<p align="center">
<b>Requires <a href="https://github.com/godot-rust/gdext">godot-rust</a> and <a href="https://github.com/typst/typst">Typst</a></b>
</p>

## ABOUT
A robust $\TeX$ alternative, directly in your Godot application.

Inspired by [GodoTeX](https://github.com/file-acomplaint/GodoTeX). Works similarly by providing custom ```TextureRect``` and ```Sprite2D``` nodes that renders Typst expressions, continually updated at runtime.

![godot-typst](https://github.com/paylanon/godot-typst/assets/74363924/a1a0af08-8725-4c7d-8a80-f3adc60fd132)

## INSTALLATION
#### (1) Ensure Typst is installed to system.
```bash
$ typst --version
```
#### (2) Add this crate as a dependency to your godot-rust project. In ``Cargo.toml``:

```toml
[dependencies]
godot-typst = { git = "https://github.com/paylanon/godot-typst" }
```
#### (3) Import the TypstTextureRect and TypstSprite classes to automatically add them to Godot. Ignore warning. 

In ``lib.rs``:

```rs
use godot_typst::TypstTextureRect;
use godot_typst::TypstSprite;
```

**Done!**

Find the example project at example/typst_project in this repo.
