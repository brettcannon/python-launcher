# Steps
## Execute
1. Manage CLI
1. Check extras/environment/context
   1. VIRTUAL_ENV
   1. Shebang
1. Search PATH
   1. Check every directory for all Python executables
   1. Select the best-fitting executable
1. Execute

## List
1. Manage CLI
1. Search PATH by checking every directory for all Python executables
1. Sort executables
1. Print table

## Help
1. Manage CLI
1. Print help


https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap12.html calls
CLI arguments "options" ('historically called "flags"') with potential "option-argument"

Sans-I/O for everything, e.g. no direct environment accessing. Probably need
to use generics/`impl Trait` to work around I/O-specific types, e.g. env::SplitPaths.
https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html
https://doc.rust-lang.org/book/ch10-00-generics.html
https://doc.rust-lang.org/std/convert/trait.TryFrom.html
https://doc.rust-lang.org/std/convert/trait.From.html

1. Start w/ cleaning up types
2. Implement various traits as appropriate
3. Fill out remaining functionality with functions
4. See where that leaves things in order to potentially organize into submodules
