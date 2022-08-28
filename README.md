# pyroexr

[![PyPI Downloads](https://img.shields.io/pypi/dm/pyroexr.svg)](
https://pypi.org/project/pyroexr/)

This is a minimal Python wrapper around the Rust [exr crate][exr],
which provides fast loading of [OpenEXR][openexr] files.

Note that this project only supports the functionality we currently need ourselves.
For instance, the package assumes you want to load the entire file into memory
and that there is only one layer in the file.
I have no current plans to extend it further, but contributions are of course welcome.

# Installation

The package can be installed directly from PyPI:

```bash
python -m pip install pyroexr
```

# Usage

A file can be opened and its channels printed as follows:

```python
import pyroexr

image = pyroexr.load("Ocean.exr")
print("Channels", image.channels())
```

```
Channels ['B', 'G', 'R']
```

Each channel can be accessed as a NumPy array, for instance to plot it in Matplotlib:

```python
import matplotlib.pyplot as plt
plt.imshow(image.channel("B"))
plt.show()
```
![Ocean output](/images/ocean.png)

# Development

If you want to develop this package locally, use the [maturin][maturin] tool
to build the Rust code and install the Python package in your current environment:

```bash
maturin develop
```

You can also use the maturin tool to build the wheels or publish the package.
See the maturin documentation for more details.

[exr]: https://crates.io/crates/exr
[openexr]: https://en.wikipedia.org/wiki/OpenEXR
[maturin]: https://maturin.rs
