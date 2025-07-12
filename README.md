# historic-rasterization-methods

non-scientific testing of ~1,000,000 arbitrary triangle rasterizations:

| Method      | Time per Triangle (us) |
| ------      | ---------------        |
| Scanline    | 51.29                  |
| Barycentric | 189.63                 |
| Raytraced   | 8,318.76               |

all methods are reasonably optimized and render an identical, watertight polygon with perspective correct interpolation that is spinning:
![alt text](https://github.com/adambigg-s/historic-rasterization-methods/blob/main/rasterizers/demo/spin.gif)
