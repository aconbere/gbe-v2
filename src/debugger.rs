
struct Debugger {
    break_points: Vec<u16>,
}

impl Debugger {
    pub fn new(sender: Sender<Target>) -> Debugger {
        Debugger {
            break_points: Vec::new(),
        }
    }

    pub fn set(&mut self, pc: u16) {
        if !self.break_points.contains(&pc) {
            self.break_points.push(pc);
        }
    }

    pub fn list(&self) -> Vec<u16> {
        self.break_points.clone()
    }

    pub fn run(&self) {

    }

    pub fn step(&self) {
    }

    pub fn next(&self) {
    }

    pub fn finish(&self) {
    }

    pub fn delete(&mut self, pc: u16) {
        self.break_points.retain(|e| *e != pc);
    }

    pub fn print_all(&self) -> String {
        String::from("wtf")
    }

    pub fn print_register(&self, r: Register) -> String {
        match r {
            Register::AF => format!("{}", "AF"),
            _ => format!("{}", "??")
        }
    }

    pub fn print_flag(&self, f: Flag) -> String {
        match f {
            Flag::Z => format!("{}", "fZ"),
            _ => format!("{}", "f?")
        }
    }

}

pub fn start(debugger_sender: SyncSender<DebugTarget>) {
    let stdin = io::stdin();

    let mut input_handle = stdin.lock();
    let mut output_handle = stdout();
    let mut debugger = Debugger::new();

    loop {
        output_handle.write(prompt().as_bytes()).unwrap();
        output_handle.flush().unwrap();

        match read(&mut input_handle) {
            Ok(tokens) => {
                println!("Tokens: {:?}", tokens);
                let output = debugger.eval(tokens).unwrap();
                println!("output: {:?}", output);
            },
            e => println!("Error: {:?}", e)
        }
    }

}

