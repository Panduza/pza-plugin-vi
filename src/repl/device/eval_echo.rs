use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use panduza_platform_core::std::class::repl::ReplProtocol;
use panduza_platform_core::{Instance, Error};

#[derive(Default)]
///
/// Simple echo evaluation
/// 
pub struct EvalEcho { }

impl EvalEcho {
    pub fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    async fn attributes(
        &mut self,
        mut instance: Instance
    ) -> Result<(), Error> {

        let si_ro = instance
            .create_attribute("si_ro")
            .with_ro()
            .finish_as_si()
            .await?;
        // let si_wo = instance
        //     .create_attribute("si_wo")
        //     .with_wo()
        //     .finish_as_si()
        //     .await?;
        // let si_rw = instance
        //     .create_attribute("si_rw")
        //     .with_rw()
        //     .finish_as_si()
        //     .await?;


        // let bool_ro = instance
        //     .create_attribute("boolean_ro")
        //     .with_ro()
        //     .finish_as_boolean()
        //     .await?;
        // let bool_wo = instance
        //     .create_attribute("boolean_wo")
        //     .with_wo()
        //     .finish_as_si()
        //     .await?;
        // let bool_rw = instance
        //     .create_attribute("boolean_rw")
        //     .with_rw()
        //     .finish_as_si()
        //     .await?;


        // let str_ro = instance
        //     .create_attribute("string_ro")
        //     .with_ro()
        //     .finish_as_string()
        //     .await?;
        // let str_wo = instance
        //     .create_attribute("string_wo")
        //     .with_wo()
        //     .finish_as_string()
        //     .await?;
        // let str_rw = instance
        //     .create_attribute("string_rw")
        //     .with_rw()
        //     .finish_as_string()
        //     .await?;


        // let enum_ro = instance
        //     .create_attribute("enum_ro")
        //     .with_ro()
        //     .finish_as_enum()
        //     .await?;
        // let enum_wo = instance
        //     .create_attribute("enum_wo")
        //     .with_wo()
        //     .finish_as_enum()
        //     .await?;
        // let enum_rw = instance
        //     .create_attribute("enum_rw")
        //     .with_rw()
        //     .finish_as_enum()
        //     .await?;


        // let json_ro = instance
        //     .create_attribute("json_ro")
        //     .with_ro()
        //     .finish_as_json()
        //     .await?;
        // let json_wo = instance
        //     .create_attribute("json_wo")
        //     .with_wo()
        //     .finish_as_json()
        //     .await?;
        // let json_rw = instance
        //     .create_attribute("json_rw")
        //     .with_rw()
        //     .finish_as_json()
        //     .await?;


        // let num_ro = instance
        //     .create_attribute("si_ro")
        //     .with_ro()
        //     .finish_as_si()
        //     .await?;
        // let num_wo = instance
        //     .create_attribute("si_ro")
        //     .with_ro()
        //     .finish_as_si()
        //     .await?;
        // let num_rw = instance
        //     .create_attribute("si_ro")
        //     .with_ro()
        //     .finish_as_si()
        //     .await?;


        // let memcmd_ro = instance
        //     .create_attribute("number_ro")
        //     .with_ro()
        //     .finish_as_number()
        //     .await?;
        // let memcmd_wo = instance
        //     .create_attribute("number_wo")
        //     .with_wo()
        //     .finish_as_number()
        //     .await?;
        // let memcmd_rw = instance
        //     .create_attribute("number_rw")
        //     .with_rw()
        //     .finish_as_number()
        //     .await?;

        
    }
}

#[async_trait]
impl ReplProtocol for EvalEcho {
    async fn eval(&mut self, instance: Instance, command: String) -> Result<String, Error> {

        self.attributes(instance.clone())
            .await?;
        Ok(command)
    }
}
