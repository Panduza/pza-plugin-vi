use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use panduza_platform_core::{std::class::repl::ReplProtocol, Error};

#[derive(Default)]
///
/// Simple echo evaluation
/// 
pub struct EvalEcho { }

impl EvalEcho {
    pub fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl ReplProtocol for EvalEcho {
    async fn eval(&mut self, command: String) -> Result<String, Error> {
        Ok(command)
    }
}