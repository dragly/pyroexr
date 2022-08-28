use ::exr::prelude::{AnyChannels, FlatSamples, Image, Layer, ReadChannels, ReadLayers, SmallVec};
use numpy::{PyArray, PyArray2};
use pyo3::{
    exceptions::{PyKeyError, PyRuntimeError},
    prelude::*,
};

#[pyclass]
struct ImageWrapper {
    image: Image<SmallVec<[Layer<AnyChannels<FlatSamples>>; 2]>>,
}

#[pymethods]
impl ImageWrapper {
    fn channels(&self) -> PyResult<Vec<String>> {
        let layer = match self.image.layer_data.first() {
            Some(l) => l,
            None => {
                return Err(PyRuntimeError::new_err("Image contains no layers".to_string()));
            }
        };
        Ok(layer
            .channel_data
            .list
            .iter()
            .map(|c| c.name.to_string())
            .collect())
    }

    fn channel<'a>(&self, py: Python<'a>, name: &str) -> PyResult<&'a PyArray2<f32>> {
        let layer = match self.image.layer_data.first() {
            Some(l) => l,
            None => {
                return Err(PyRuntimeError::new_err("Image contains no layers".to_string()));
            }
        };
        let channel = match layer
            .channel_data
            .list
            .iter()
            .find(|channel| channel.name.eq(name))
        {
            Some(c) => c,
            None => {
                return Err(PyKeyError::new_err(format!(
                    "Channel '{name}' not found in image"
                )));
            }
        };
        let size = [layer.size.1, layer.size.0];
        let array = PyArray::from_iter(py, channel.sample_data.values_as_f32()).reshape(size);

        array
    }
}

#[pyfunction]
fn load(filename: &str) -> PyResult<ImageWrapper> {
    let image = match exr::prelude::read::read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()
        .all_layers()
        .all_attributes()
        .from_file(filename)
    {
        Ok(img) => img,
        Err(err) => {
            return Err(PyRuntimeError::new_err(format!(
                "Could not load file '{filename}' due to error: '{err}'"
            )));
        }
    };

    Ok(ImageWrapper { image })
}

#[pymodule]
fn pyroexr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    Ok(())
}
