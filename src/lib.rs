use ::exr::prelude::{AnyChannels, FlatSamples, Image, Layer, ReadChannels, ReadLayers, SmallVec};
use numpy::{PyArray, PyArray2};
use pyo3::prelude::*;

#[pyclass]
struct ImageWrapper {
    image: Image<SmallVec<[Layer<AnyChannels<FlatSamples>>; 2]>>,
}

#[pymethods]
impl ImageWrapper {
    fn channels(&self) -> PyResult<Vec<String>> {
        Ok(self
            .image
            .layer_data
            .first()
            .unwrap()
            .channel_data
            .list
            .iter()
            .map(|c| c.name.to_string())
            .collect())
    }

    fn channel<'a>(&self, py: Python<'a>, name: &str) -> PyResult<&'a PyArray2<f32>> {
        let layer = self.image.layer_data.first().unwrap();
        let channel = layer
            .channel_data
            .list
            .iter()
            .find(|channel| channel.name.eq(name))
            .unwrap();
        let size = [layer.size.1, layer.size.0];
        let array = PyArray::from_iter(py, channel.sample_data.values_as_f32()).reshape(size);

        array
    }
}

#[pyfunction]
fn load(filename: &str) -> PyResult<ImageWrapper> {
    let image = exr::prelude::read::read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()
        .all_layers()
        .all_attributes()
        .from_file(filename)
        .unwrap();

    Ok(ImageWrapper { image })
}

#[pymodule]
fn pyroexr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    Ok(())
}
