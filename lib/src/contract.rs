use crate::{crypto::Hashable, executable::Executable, state::GlobalState};

pub struct State {
    pub variables: Vec<String>, // the varibles, in hex
}

pub struct SmartContract {
    hash: String,    // the hash of the byte code + account address
    hex: String,     // the byte code of this account, as hex
    account: String, // the address of this account
    state: State,    // the contracts 'state
}

impl Hashable for SmartContract {
    fn bytes(&self) -> Vec<u8> {
        let mut out = vec![];

        out.extend(hex::decode(self.hex.clone()).unwrap());
        out.extend(self.account.bytes());
        for var in &self.state.variables {
            out.extend(var.as_bytes())
        }

        out
    }
}

impl Executable for SmartContract {
    /// # Execute
    /// Runs the smart contract, with conext being the block hash it was called by
    /// Returns the new state hash if this smart contract ran correctly or the error code if it failed
    fn execute(&self, context: &String, state: Option<&mut GlobalState>) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }
    /// # Evaluate
    /// Checks if this smart contract is correct byte code
    fn evalute(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    /// # Cost
    /// Estimates the cost of an execution of this contract given a block hash as context
    fn cost(&self, context: &String) -> u64 {
        todo!()
    }
}

pub fn compile_into_bytecode(code: String, code_type: u8) -> String {
    let mut out: Vec<u8> = vec![];
    let mut found_start_code = false;

    match code_type {
        0 => {
            // redcode, redstones native smart contract language
            let commands = code.split(";");
            for command in commands {
                let subcommand = command.split(" ").collect::<Vec<&str>>();
                match subcommand[0] {
                    "START" => {
                        if found_start_code {
                            return String::default();
                        }
                        out.push(100); // start program command
                        if subcommand.len() == 2 {
                            // START takes an optional name as the seccond varible
                            out.push(0); // varible delimniter
                            out.extend(hex::encode(subcommand[1]).into_bytes());
                        } else if subcommand.len() != 1 {
                            return String::default(); // START takes at most 1 varible
                        }
                        out.push(1); // command delimniter
                        found_start_code = true
                    }
                    "SET" => {
                        if !found_start_code || subcommand.len() != 3 {
                            return String::default();
                        } else {
                            // sets a value of the conracts state (eg SET 0 1 will set the first var of the contracts state to 1)
                            let index: i64 =
                                i64::from_str_radix(subcommand[1].trim_start_matches("0x"), 16)
                                    .unwrap_or(-2);
                            if index == -2 {
                                return String::default();
                            } else {
                                let val =
                                    i64::from_str_radix(subcommand[2].trim_start_matches("0x"), 16)
                                        .unwrap_or(-2);
                                if val == -1 {
                                    return String::default();
                                }
                                out.push(101);
                                out.push(0);
                                out.extend((index as u64).to_string().into_bytes());
                                out.push(0);
                                out.extend(val.to_string().into_bytes());
                                out.push(1);
                            }
                        }
                    }
                    "END" => {
                        // check that START has been issued
                        if !found_start_code {
                            return String::default();
                        } else {
                            // return the compiled byte code
                            return hex::encode(out);
                        }
                    }
                    _ => {
                        return String::default();
                    }
                }
            }
        }
        _ => return String::default(),
    }
    return String::default();
}
