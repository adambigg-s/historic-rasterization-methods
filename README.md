# historic-rasterization-methods

non-scientific testing of ~100,000 arbitrary triangle rasterizations:

| Method | us Per Triangle |
| ------ | --------------- |
| Scanline | 51.29 |
| Barycentric | 190.47 |
| Raytraced | 15633.53 |

all methods are reasonably optimized and render an identical, watertight polygon with perspective correct interpolation that is spinning:
![alt text](https://github.com/adambigg-s/historic-rasterization-methods/blob/main/rasterizers/demo/spin.gif)
