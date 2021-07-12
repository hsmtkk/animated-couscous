use anyhow::{bail, Result};

const SPACE:&str =  " ";
const GO_TO:&str = "go to";
const BLANK_PAGE:&str = "blank page";
const GO_TO_BLANK_PAGE:&str = "go to blank page";
const USE_THE_BUCK_BUTTON:&str = "use the back button";

fn surf(inputs:Vec<&str>) -> Vec<String>{
    let mut proc = Processor::new();
    let mut outputs = Vec::new();
    for input in inputs {
        let inst = parse_line(input).unwrap();
        let got = proc.process_instruction(inst);
        outputs.push(got);
    }
    outputs
}

struct Processor {
    stack: Vec<String>,
}

impl Processor {
    fn new() -> Processor {
        let stack = Vec::new();
        Processor{stack}
    }

    fn process_instruction(&mut self, inst:Instruction) -> String {
        match inst.command {
            Command::GoTo => {
                let arg = inst.argument.clone();
                self.stack.push(arg);
                return inst.argument;
            },
            Command::UseTheBackButton => {
                let val = self.stack.pop().unwrap();
                return val;
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    GoTo,
    UseTheBackButton,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    command: Command,
    argument: String,
}

impl Instruction {
    fn new(command:Command, argument:String) -> Instruction {
        Instruction{command, argument}
    }
}

fn parse_line(line:&str) -> Result<Instruction> {
    if USE_THE_BUCK_BUTTON == line {
        return Ok(Instruction::new(Command::UseTheBackButton, "".to_string()));
    }
    if line.starts_with(GO_TO){
        let arg = &line[GO_TO.len()+1..];
        return Ok(Instruction::new(Command::GoTo, arg.to_string()));
    }
    bail!("unknown line; {}", line)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0(){
        let inputs: Vec<&str> = vec![
            super::GO_TO_BLANK_PAGE,
            "go to bja n",
            "go to va",
            super::USE_THE_BUCK_BUTTON,
            super::USE_THE_BUCK_BUTTON,
        ];
        let want: Vec<&str> = vec![
            super::BLANK_PAGE,
            "bja n",
            "va",
            "bja n",
            super::BLANK_PAGE,
        ];
        let got = super::surf(inputs);
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let inputs: Vec<&str>  = vec![
            super::GO_TO_BLANK_PAGE,
            "go to nkah",
            super::USE_THE_BUCK_BUTTON,
            "go to gi",
            "go to in",
            "go to nkah",
            super::USE_THE_BUCK_BUTTON,
        ];
        let want: Vec<&str> = vec![            
            super::BLANK_PAGE,
            "nkah",
            super::BLANK_PAGE,
            "gi",
            "in",
            "nkah",
            "in",
        ];
        let got = super::surf(inputs);
        assert_eq!(want, got);
    }

    #[test]
    fn test_parse_line0(){
        let input = "go to blank page";
        let want = super::Instruction{
            command: super::Command::GoTo,
            argument: super::BLANK_PAGE.to_string(),
        };
        let got = super::parse_line(input).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_parse_line1(){
        let input = "go to nkah";
        let want = super::Instruction{
            command: super::Command::GoTo,
            argument: "nkah".to_string(),
        };
        let got = super::parse_line(input).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_parse_line2(){
        let input = "use the back button";
        let want = super::Instruction{
            command: super::Command::UseTheBackButton,
            argument: "".to_string(),
        };
        let got = super::parse_line(input).unwrap();
        assert_eq!(want, got);
    }
}
