img-convert
===

A thin wrapper around the nice [image crate](https://github.com/image-rs/image/tree/master) to make converting between the various image types easier.

## Usage

### Help Text

Usage: img-convert [OPTIONS] <SOURCE> <DESTINATION>

Arguments:
  <SOURCE>       Path to the image you wish to convert
  <DESTINATION>  The destination, image conversion done based on file extension (.png will convert to png)

Options:
  -f               If set, will overwrite destination (if it exists)
  -r <RESIZE>      If set, will resize the image to given dimentions. Format is wxh, e.g. 1080x720
  -h, --help       Print help
  -V, --version    Print version

### Examples

Convert from jpg to png:

```
img-convert image.jpg image.png
```

Resize to 100x100:
```
img-convert -r 100x100 image.jpg image-small.jpg
```

Reize to a width of 100, and make the height proportional:
```
img-convert -r 100x image.jpg image-small.jpg
```

Reize to a height of 100, and make the width proportional:
```
img-convert -r x100 image.jpg image-small.jpg
```