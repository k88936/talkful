pub mod llama_refine_service;
use anyhow::Error;

pub trait RefineService {
    fn refine(&mut self, src: &str, prompt: &str) -> Result<String, Error>;
}

pub struct NoRefineService {}
impl NoRefineService {
    pub fn new() -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
impl RefineService for NoRefineService {
    fn refine(&mut self, src: &str, _prompt: &str) -> Result<String, Error> {
        Ok(src.into())
    }
}
