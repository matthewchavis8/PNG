// mod args;
// mod chunk;
pub mod chunk_type;
// mod commands;
// mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// fn main() -> Result<()> {}
fn main() {}