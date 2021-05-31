use crate::expression::Environment;
use crate::statement::Statement;

pub struct Machine {
    pub statement: Statement,
    pub environment: Environment,
}

impl Machine {
    fn step(&mut self) {
        // 以下のようには現バージョンだと書けない？
        // (self.statement, self.environment) = self.statement.reduce(&self.environment)
        let (new_statement, new_env) = self.statement.reduce(&mut self.environment);
        self.statement = new_statement;
        self.environment = new_env;
    }
    pub fn run(&mut self) {
        while self.statement.is_reducible() {
            println!("{}, {:?}", self.statement, self.environment);
            self.step();
        }
        println!("{}, {:?}", self.statement, self.environment);
    }
}
