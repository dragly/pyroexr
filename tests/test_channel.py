import pyroexr
import numpy as np


def test_ocean_channels():
    image = pyroexr.load("tests/files/Ocean.exr")
    assert set(image.channel_names()) == {"B", "G", "R"}


def test_ocean_blue_channel():
    image = pyroexr.load("tests/files/Ocean.exr")
    np.save("tests/files/Ocean_B.npy", image.channel("B"))
