# Clorox
`clorox` is a command line utility for picking colors from an image.

## Synopsis
```
clorox [options...] [location]
```

### Options:
* `-h`, `--help`: Show help message and quit.
* `-f`, `--format`: Change color output display. Can be:
  * `rgb`: `255,255,255`
  * `hex`: `#FFFFFF`

### Dimension 
The X,Y location of the pixel in question. 

## Examples
```sh
# Picking an image
clorox -i Pictures/test.png 255,255

# Using `grim` and `slurp` to grab the colors from a screenshot
grim - | clorox $(slurp -p -f "%Xx%Y")
```
