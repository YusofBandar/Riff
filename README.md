# Riff
Superfast and minimal dependency pixel image comparasion tool built with rust. 

```
riff <base_path> <diff_path>
```

## Command Line
```
<base_path>      Path to image (jpeg or png) to compare from
<diff_path>      Path to image (jpeg or png) to compare to
```

```
-o, --output        Path to output image (jpeg or png) [default: ./output.png]
-a, --alpha         Blending value of unchaged pixels, 0 alpha disables drawing of base image [default :0]
-t, --threshold     Matching threshold, smaller values makes pixel comparison more sensitive [default: 0.1]
-c, --diffColour    The color of differing pixels in [R, G, B, A] format [default: [218, 165, 32, 255]]
--viewPort          The region within base image to compare to in [x, y, width, height] format. Useful when comparing differently sized images

```
