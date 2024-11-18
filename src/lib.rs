mod libs;
mod ball_game;

use pyo3::prelude::*;
use bevy::prelude::*;
use pyo3::types::PyInt;
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

#[pyfunction]
fn ad_hok() -> PyResult<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin::default())
        .run();
    Ok(())
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
