# Rand derive 2

[![Latest Version](https://img.shields.io/crates/v/rand_derive2.svg)](https://crates.io/crates/rand_derive2)
[![Build Status](https://img.shields.io/github/workflow/status/jasperav/rand_derive2/CI/master)](https://github.com/jasperav/rand_derive2/actions)

Derive macro for generating random types with the `rand` crate. 
It will implement the `rand::distributions::Standard` for a given type.

## Usage

Check out the example crate or follow the instructions below.

1. Add this to your Cargo.toml file:

```toml
[dependencies]
rand_derive2 = "0.1"
rand = "0.8"
```

2. Import the macro somewhere in your file where your type is:
```rust
use rand_derive2::RandGen;
```

Alternatively, use this to global import the macro:
```rust
#[macro_use]
extern crate rand_derive2;
```

3. Add the RandGen derive macro for your type:
```rust
#[derive(RandGen)]
struct MyStruct {}
```

4. Generate your struct:
```rust
fn generate_my_struct() -> MyStruct { 
    rand::random()
}

// Or like this
fn generate_my_struct_direct() -> MyStruct { 
    MyStruct::generate_random()
}
```

### Customization
Note: all things that can be customized is covered in the example crate
#### Options
To make sure an option is never generated with `None`, add the `rand_derive(none)` attribute on top of the property.
To make sure an option is never generated with `Some`, add the `rand_derive(some)` attribute on top of the property.
#### Skip enum variant
If a variant should never be generated, add the `rand_derive(skip)` attribute on the variant. 
#### Custom value
If you want a custom value for one of the properties, add the `rand_derive(custom)` attribute.
A trait is created called TestDataProviderFor$TYPE$. 
This trait will require the user to provider the values.
#### No rand
Panic implementation of the property, making the type unable to be random generated.
Add the `rand_derive(panic)` attribute for this case.
#### Default value
Place `rand_derive(default)` above a field to make it generate the default value. Note: for a Vec, this will create a vec
with 1 element inside it which holds the default value.
#### Empty
Can be placed above types which holds a Vec. It will generate an empty vec.
#### Fixed value
Place `rand_derive(fixed = "MY_VALUE")` above a field to make it generate the fixed value.

### How it works 
#### Structs
It calls `rng.gen()` on all the fields.
#### Enums 
It will generate a random variant.

### Types with lifetimes
It doesn't make sense to generate a type with a lifetime attached, because than a type with the owned values must be
created somewhere, but where? It could be _maybe_ done by leaking the type that owns the values, but that isn't implemented at the moment.

### TODO
- Recursion for e.g. vec in vec
- More types from the standard library covered
- Functions documented
- Custom trait type/method names
- Weighted randomization (currently only supports `rand::distributions::Standard`)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
