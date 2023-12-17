#[derive(Debug)] pub enum Ok { Passed }
#[derive(Debug)] pub enum Err { Failed }

pub struct CommandAction<F> where
    F: Fn(String) -> Result<Ok, Err>
{ prefix: String, action: F }

impl<F> CommandAction<F> where
    F: Fn(String) -> Result<Ok, Err>
{
    pub fn new(prefix: &str, action: F) -> Self {
        return CommandAction { prefix: prefix.to_string().to_lowercase(), action }
    }

    pub fn prefix(&self) -> &str {
        return self.prefix.as_str()
    }

    pub fn try_call(&self, param: &str) -> Result<Ok, Err> {
        let result = (self.action)(param.to_string());
        if result.is_err() {
            return Err(Err::Failed);
        }
        return Ok(Ok::Passed);
    }
}

pub type FPointer = Box<dyn Fn(String) -> Result<Ok, Err> + Sync>; 

pub fn use_command(target: &str, commands: &Vec<CommandAction<FPointer>>, param: &str) {
    let mut existent = false;
    let target = target.to_lowercase();

    commands.into_iter()
        .for_each(|command| {
            if command.prefix() == target {
                existent = true;
                if let Err(err) = command.try_call(param) {
                    panic!("error encountered '{:?}'", err);
                }
            }
        });


    if !existent {
        panic!("`{target}` not a command to use...");
    }
}
