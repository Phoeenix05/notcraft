**NOTE**: This crate is currently on hold as using the generated bindings don't seem to be 
as easily usable as I expected them to be.

**Lua** in **NotCraft** is used for modding the game.

**NOTE**: This crate only includes the bindings for lua not the actual implementations.


Maybe something liek this could work.

A global Lua Scope something like this
```rust
lazy_static! {
    pub static LUA_SCOPE: Mutex<Lua> = ...;
}
```

And then having a derive macro that adds the functions needed in lua to the scope
and also generates a lua file for intellisense.
