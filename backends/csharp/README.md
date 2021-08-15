Generates C# bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).

## Usage

Assuming you have written a crate containing your FFI logic called `example_library_ffi` and
want to generate **C# bindings**, follow the instructions below.

#### Inside Your Library

Add [**Interoptopus**](https://crates.io/crates/interoptopus) attributes to the library you have
written, and define an [**inventory**](https://docs.rs/interoptopus/latest/interoptopus/macro.inventory.html)
function listing all symbols you wish to export. An overview of all supported constructs can be found in the
[**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/interoptopus_reference_project/src).

```rust
use interoptopus::{ffi_function, ffi_type};

#[ffi_type]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn my_function(input: Vec2) -> Vec2 {
    input
}

interoptopus::inventory!(my_inventory, [], [my_function], [], []);
```


Add these to your `Cargo.toml` so the attributes and the binding generator can be found
(replace `...` with the latest version):

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
interoptopus = "..."
interoptopus_backend_csharp = "..."
```

Create a unit test in `tests/bindings.rs` which will generate your bindings when run
with `cargo test`. In real projects you might want to add this code to another crate instead:

```rust
use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            dll_name: "example_library".to_string(),
            namespace_mappings: NamespaceMappings::new("My.Company"),
            ..Config::default()
        },
        example_library_ffi::my_inventory(),
    ).write_file("bindings/csharp/Interop.cs")?;

    Ok(())
}
```

Now run `cargo test`.

If anything is unclear you can find a [**working sample on Github**](https://github.com/ralfbiedert/interoptopus/tree/master/examples/hello_world).

#### Generated Output

The output below is what this backend might generate. Have a look at the [`Config`] struct
if you want to customize something. If you really don't like how something is generated it is
easy to [**create your own**](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md#new-backends).

```csharp
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using My.Company;

namespace My.Company
{
    public static partial class InteropClass
    {
        public const string NativeLib = "example_library";

        /// Function using the type.
        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "my_function")]
        public static extern Vec2 my_function(Vec2 input);
    }

    /// A simple type in our FFI layer.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Vec2
    {
        public float x;
        public float y;
    }
}
```