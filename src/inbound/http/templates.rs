use axum_template::engine::Engine;
use minijinja::Environment;

pub type AppEngine = Engine<Environment<'static>>;
