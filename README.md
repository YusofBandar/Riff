# Riff
Superfast and minimal dependecy pixel image comparasion tool built with rust. 

```
riff <base_path> <diff_path> <output_path>
```

## Command Line
```
<base_path>      Path to image (jpeg or png) to compare from
<diff_path>      Path to image (jpeg or png) to compare to
<output_path>    Path to to save output image (jpeg or png)
```

```
--alpha          Blending value of unchaged pixels, 0 alpha disables drawing of base image [default :0]
--threshold      Matching threshold, smaller values makes pixel comparison more sensitive [default: 0.1]
```
