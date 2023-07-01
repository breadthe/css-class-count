# Count unique classes in a CSS file

A command-line utility written in Rust that counts and displays a summary of unique classes in a CSS file.

**Sample output**

```
Class Counts
----------------------------------------
...
text-red-600: 1
focus: 4
shadow: 1
border-orange-400: 1
...
----------------------------------------
Total Unique Classes: 349
Full Path: /path/to/app.css
CSS File: app.css
File Size: 30.65 KB (31384 bytes)
```

**Build**

```shell
cargo build
```

You can copy and execute the binary `./target/debug/css-class-count` from anywhere you want.

**Run**

```shell
cargo run -- <path_to_css_file>

# or

cargo build

./target/debug/css-class-count <path_to_css_file>
```

**Write to a file**

```shell
cargo run -- <path_to_css_file> > class-count.txt

# or

cargo build

./target/debug/css-class-count <path_to_css_file> > class-count.txt
```
