# import_fields
Proc Macro to import fields from another struct

## How does it works?
Given any number of file paths like `"path/to/file.rs"`, the Proc Macro imports into the current Struct all fields from every struct it finds in given paths.

It is possible to specify a filter in the form `"path/to/file.rs::StructName"` to import only fields from matching Struct.

Take a look at [the tests](tests/path.rs) for a better explanation.
