# enum-macros
[![Crates.io Version](https://img.shields.io/crates/v/enum_macros)](https://crates.io/crates/enum_macros)
[![docs.rs (with version)](https://img.shields.io/docsrs/enum_macros/latest)](https://docs.rs/enum_macros/latest/enum_macros/)

A collection of useful macros to make working with enums easier.

## Current features
The current set of macros is not fixed yet. More macros will likely be added or removed in the future.

Each macro can be enabled and disabled individually by its respective Cargo feature if it is not wanted.
By default, all are enabled.

* `custom_discriminant`: allows using arbitrary values as discriminants and converting from and to them
* `marker_type`: creates a copy of an enum where no field holds data 
* `NextVariant`: allows moving forward through the variants of an enum
* `VariantAmount`: adds a constant storing the amount of variants the enum has
* `UnwrapVariant`: adds methods for unwrapping variants of an enum

## `no_std` compability
The crate is fully `#![no_std]` compatible at the moment. 
Future versions may introduce a subset of features that require `std` under a new `std` crate feature.
