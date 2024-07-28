# enum-macros
A collection of useful macros to make working with enums easier.

## Current features
The current set of macros is not fixed yet. More macros will likely be added or removed in the future.

Each macro can be enabled and disabled individually by its respective Cargo feature if it is not wanted.

* `custom_discriminant`: allows using arbitrary values as discriminants and converting from and to them
* `marker_type`: creates a copy of an enum where no field holds data 
* `next_variant`: allows moving forward through the variants of an enum
* `variant_amount`: adds a constant storing the amount of variants the enum has

## `no_std` compability
Disabling the `std` feature attempts to make the crate `no_std` compatible.