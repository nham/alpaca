#![crate_name = "alpaca"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![license = "MIT"]

use std::collections::{HashMap, HashSet};

// TODO: need a name
enum Program {
    Suite(Suite),
    Command(Command),
}

struct Suite {
    children: HashMap<String, Program>
}

// A command has some number of options and some number of arguments
struct Command {
    options: HashSet<Opt>,
    args: ArgSignature,
}

// An *option* has a name, zero or more arguments, and a multiplicity
// which dictates how many times the option can/must show up.
struct Opt {
    names: String,
    narg: uint,
    types: ArgSignature,
    multiplicity: OptMult,
}

struct OptMult {
    required: bool,
    multiple: bool,
}

enum ParamVal {
    ParamStr,
    ParamInt,
    ParamBool,
}

type ArgSignature = Vec<ParamVal>;

struct MatchedOpt {
    name: String,
    args: Vec<MatchedParamVal>,
}

enum MatchedParamVal {
    MParamStr(String),
    MParamInt(int),
    MParamBool(bool),
}

// Represents a successfully match of the parameters
// supplied to a command
struct ParamMatch {
    options: Vec<MatchedOpt>,
    args: Vec<MatchedParamVal>,
}

enum ParseFail {
    Borked(String),
}


pub fn getopts(args: &[String], prog: &Program) -> Result<ParamMatch, ParseFail> {
    // for now, assume all options come before any argument
    // we need to match every required option and every argument
    // also, if an option/argument is present, it must match up with each of its arguments
    // there's an ambiguity here. a program has "arguments", meaning its positional arguments
    // but an option can also have arguments.
}
