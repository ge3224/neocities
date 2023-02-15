/// Args contains a command and its params
pub struct Args {
    pub command: String,
    pub params: Vec<String>,
}

impl Args {
    // add code here
    pub fn build(inputs: &[String]) -> Result<Args, &'static str> {
        if inputs.len() < 2 {
            return Err("not enough arguments");
        }

        let mut inputs_iter = inputs.iter();

        // skip the first argument
        inputs_iter.next();

        // TODO handle Option correctly
        let command = inputs_iter.next().unwrap().clone();

        let mut params = vec![];

        for param in inputs.iter() {
            params.push(param.clone());
        }

        Ok(Args { command, params })
    }

    /// params_size returns the number of params
    pub fn params_size(&self) -> usize {
        self.params.len()
    }

    /// is_params_empty checks if param size is 0
    pub fn is_params_empty(&self) -> bool {
        if self.params.len() != 0 {
            return false;
        }

        true
    }

    /// first_param returns an Some reference to the first param, or None.
    pub fn first_param(&self) -> Option<&String> {
        Some(&self.params[0])
    }
}
