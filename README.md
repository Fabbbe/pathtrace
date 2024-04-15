# Pathtrace

A simple pathtracer written in rust.

Right now the image is output to stdout in ppm format which means saving the ouput is as easy as:

```
cargo run > out.ppm
```

And if you want the output in any other format use ffmpeg for now:

```
cargo run | ffmpeg -i - out.png
```

Or if your terminal supports it: 

```
cargo run | img2sixel
```

## TODO

Except for the task of developing the project it is necessary to:

* Write documentation for structs and functions
* Write tests for the core stuffs

