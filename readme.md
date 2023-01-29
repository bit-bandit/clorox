# Clorox
`clorox` is a command line utility for picking colors from an image.

## Synopsis
```
clorox [options...] [dimension]
```

### Options:
* `-h`, `--help`: Show help message and quit.
* `-f`, `--format`: Change color output display. Can be:
  * `rgb`: `255,255,255`
  * `hex`: `#FFFFFF`
  * `hsl`: ``

### Dimension 
MUST be the X Y coordinates of the pixel in question. 

## Examples
```sh
# Picking an image
clorox -i Pictures/test.png 255,255
```
