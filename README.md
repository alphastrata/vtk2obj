#vtk2obj

This is a pretty simple cli app that takes, one?(maybe only one) legacy `.vtk` and after parsing it will convert it into an `.obj` file.

That is all.

## Requirements:

1. [Rust](https://www.rust-lang.org/tools/install)
2. [This repo](https://github.com/alphastrata/vtk2obj/)

# Usage:

_assuming you've cloned the repo and got all the above installed properly_

```bash
cd <to-wherever-you-cloned-the-repo-to>

# building it.
cargo build --release 
./target/release/vtk2obj --help # for help.
./target/release/vtk2obj --input <your .vtk file> --output <what you wanna call it.obj>

# installing it more permanently, if you think you'll need that.
cargo install --path .
vtk2obj --input <your .vtk file> --output <what you wanna call it.obj> # you don't need the <> brackets it's just convention..
```

Now, that you've got an `.obj` file instead of that useless `.vtk` one you can import it into Blender.
- then export it to `.fbx`.
- then improt it into UNREAL.


# Contributions:
__welcome, but I didn't design this to be a long-lived big-ass tool. it's simple and does one thing_