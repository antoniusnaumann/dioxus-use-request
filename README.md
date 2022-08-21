# Dioxus - Use Request Hook
[![Build Status](https://github.com/antoniusnaumann/dioxus-use-request/actions/workflows/build.yml/badge.svg)](https://github.com/antoniusnaumann/dioxus-use-reqest/actions)
[![Publish Status](https://github.com/antoniusnaumann/dioxus-use-request/actions/workflows/publish.yml/badge.svg)](https://github.com/antoniusnaumann/dioxus-use-reqest/actions)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-F46623)](https://www.rust-lang.org)
[![Crates.io Version](https://img.shields.io/crates/v/dioxus-use-request)](https://crates.io/crates/dioxus-use-request)

A small crate that adds a hook for simple GET requests and [convienient macro]("#Macro") to [Dioxus](https://dioxuslabs.com).

## Example
### Function
Using the hook like this...
```Rust
let dog = use_request(&cx, (breed,), format!("https://dog.ceo/api/breed/{breed}/images/random"));

cx.render(match dog {
    Success(val) => rsx!(img { src: "{val.message}"}}),
    Failure(err) => rsx!("Failed to load dog"),
    Loading => rsx!("Loading dog image!"),
})
```
...achieves the same as this snippet which uses the [Use Future Hook](https://dioxuslabs.com/guide/async/use_future.html):

```Rust
let dog = use_future(&cx, (breed,), |(breed)| async move {
        reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await
            .unwrap()
            .json::<RandomDog>()
            .await
    });
    
cx.render(match dog.value() {
    Some(Ok(val)) => rsx!(img { src: "{val.message}"}}),
    Some(Err(err)) => rsx!("Failed to load dog"),
    None => rsx!("Loading dog image!"),
})
```
### Macro
For additional convienience, you can also enable the `use_request!` macro. 

```TOML
[dependencies]
dioxus-use-request = { version = "0.1.4", features = ["macro"] }
```

It automatically extract the hook's dependencies from the given URL string.
Using the macro, the example from above looks like this:
```Rust
// Macro automatically extracts dependencies in curly braces (e.g. {breed}) from the given string literal
let dog = use_request!(cx, "https://dog.ceo/api/breed/{breed}/images/random");

cx.render(match dog {
    Success(val) => rsx!(img { src: "{val.message}"}}),
    Failure(err) => rsx!("Failed to load dog"),
    Loading => rsx!("Loading dog image!"),
})
```

## Installation
For instructions on getting started with Dioxus, please refer to their [Getting Started Guide](https://dioxuslabs.com/guide/index.html). 
To use this crate, simply add it as dependency in your Cargo.toml. Make sure to enable the "macro" feature if you want to use the `use_request!` macro:

```TOML
[dependencies]
dioxus-use-request = { version = "0.1.4", features = ["macro"] }
```

## Features
- [x] Implement hook
- [x] Convienent macro with dependency extraction
- [ ] Return `UseRequest` handle to make request restartable
- [ ] Implement `.split()` on `UseRequest` handle which returns value and restart closure

If you encounter a bug or have an idea for a feature, feel free to [open an issue](https://github.com/antoniusnaumann/dioxus-use-request/issues/new)!

## License
This code is dual-licensed and availabe under MIT-License or Apache 2.0-License depending on what suits your needs best.

### Apache2.0-License
```
Copyright 2022 Antonius Naumann

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

### MIT-License
```
MIT License

Copyright (c) 2022 Antonius Naumann

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
