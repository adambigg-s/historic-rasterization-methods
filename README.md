# historic-rasterization-methods

non-scientific testing of ~100,000 arbitrary triangle rasterizations:

| Method | us Per Triangle |
| ------ | --------------- |
| Scanline | 53.78 |
| Barycentric | 204.04 |
| Raytraced | 16162.85 |

all methods are reasonably optimized and render an identical, watertight polygon with perspective correct interpolation that is spinning:
![til](https://github.com/adambigg-s/historic-rasterization-methods/tree/main/rasterizers/demo/spin.gif)
