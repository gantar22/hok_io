mod libs;
mod ball_game;

use bevy::asset::AssetContainer;
use pyo3::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalForce;
use pyo3::types::{PyInt, PyTuple};
use crate::libs::GamePlugin;

#[pyclass]
struct GameState;

#[pyclass]
struct TickOutput(u64);

#[pyclass(subclass)]
pub struct AiController;

#[pymethods]
impl AiController {
    #[staticmethod]
    fn new() -> Self {
        AiController
    }

    fn start_match(&mut self) -> PyResult<()> {
        Ok(())
    }

    fn tick_match(&mut self, game_state: &GameState, tick_output: &mut TickOutput) -> PyResult<()> {
        Ok(())
    }
}

#[derive(Resource)]
struct InputPyObject(PyObject);

#[pyfunction]
fn ad_hok(obj: PyObject) -> PyResult<()> {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_systems(Update,use_input)
        .add_plugins(GamePlugin::default());
    app.world_mut().commands().insert_resource(InputPyObject(obj));
    app.run();
    Ok(())
}

#[pyclass]
struct PyVec{x : f32, y : f32}

fn use_input(
    input_py: ResMut<InputPyObject>,
    mut force_q: Query<(&mut ExternalForce,&Transform)>
) {
    for (mut force, transform) in force_q.iter_mut() {
        Python::with_gil(|py| {
            let input = PyVec{ x: transform.translation.x, y: transform.translation.y };
            let output = input_py.0.call(py,PyTuple::new(py,[input]).unwrap(),None).unwrap();
            force.force.x = output.getattr(py,"x").unwrap().extract(py).unwrap();
            force.force.y = output.getattr(py,"y").unwrap().extract(py).unwrap();
        });
    }
}

#[pyfunction]
fn input_test(py: Python<'_>, controller: PyObject) -> PyResult<()> {
    let output = TickOutput(0);
    controller.call_method(py,"tick_match",(GameState,output),None)?;
    Ok(())
}


/// A Python module implemented in Rust.
#[pymodule]
fn hok_io(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ad_hok,m)?)?;
    m.add_function(wrap_pyfunction!(input_test,m)?)?;
    m.add_class::<GameState>()?;
    m.add_class::<TickOutput>()?;
    m.add_class::<AiController>()?;
    Ok(())
}
