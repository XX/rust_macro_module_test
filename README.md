# The problem 

## Macro undefined when test module is moved to a separate file

*Sebastian*

*Published in 2017-02-14 00:07:22Z*

I am writing a test for a macro I want to export. The test works as long as I keep my tests in a single file, but as soon as I put the tests module in a separate file, I get an error.

**export/src/lib.rs**
```rust
pub mod my_mod {
    #[macro_export]
    macro_rules! my_macro {
        ( $x:expr ) => { $x + 1 };
    }
    
    pub fn my_func(x: isize) -> isize {
        my_macro!(x)
    }
}
```
**export/tests/lib.rs**
```rust
#[macro_use]
extern crate export;

mod my_test_mod {
    use export::my_mod;

    #[test]
    fn test_func() {
        assert_eq!(my_mod::my_func(1), 2);
    }

    #[test]
    fn test_macro() {
        assert_eq!(my_macro!(1), 2);
    }
}
```
Running `cargo test` indicates that both tests passed. If I extract `my_test_mod` to a file it no longer compiles.

**export/src/lib.rs**

Unchanged

**export/tests/lib.rs**
```rust
#[macro_use]
extern crate export;

mod my_test_mod;
```
**export/tests/my_test_mod.rs**
```rust
use export::my_mod;

#[test]
fn test_func() {
    assert_eq!(my_mod::my_func(1), 2);
}

#[test]
fn test_macro() {
    assert_eq!(my_macro!(1), 2); // error: macro undefined: 'my_macro!'
}
```
This gives me an error that the macro is undefined.

# Solution

*Shepmaster*

*Reply to 2017-02-14 00:38:31Z*

The problem here is that you aren't compiling what you think you are compiling. Check it out:
```
$ cargo test --verbose
   Compiling export v0.1.0 (file:///private/tmp/export)
     Running `rustc --crate-name my_test_mod tests/my_test_mod.rs ...`
```
When you run `cargo test`, it assumes that every .rs file is a test to be run. It doesn't know that **my_test_mod.rs** should only be compiled as part of another test!

The easiest solution is to move your module to the other valid module location, in a separate directory: **tests/my_test_mod/mod.rs**. Cargo will not recursively look inside the directory for test files.

# References

http://webcache.googleusercontent.com/search?q=cache:qA42sx3lrcIJ:demo101.phpcaiji.com/article/eccbfhbe-macro-undefined-when-test-module-is-moved-to-a-separate-file.html

http://demo101.phpcaiji.com/article/eccbfhbe-macro-undefined-when-test-module-is-moved-to-a-separate-file.html
