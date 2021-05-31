use semantics::expression::Expression;
use semantics::machine::Machine;
use semantics::statement::Statement;
use std::collections::HashMap;

fn main() {
    let expression = Expression::Add {
        left: Box::new(Expression::Variable(String::from("x"))),
        right: Box::new(Expression::Number(1)),
    };
    let statement = Statement::Assignment {
        name: String::from("x"),
        expression: expression,
    };

    let mut environment = HashMap::new();
    environment.insert(String::from("x"), Expression::Number(2));

    let mut machine = Machine {
        statement: statement,
        environment: environment,
    };

    machine.run();

    println!("--");

    let mut environment = HashMap::new();
    environment.insert(String::from("x"), Expression::Boolean(true));
    let mut machine = Machine {
        statement: Statement::If {
            condition: Expression::Variable(String::from("x")),
            consequence: Box::new(Statement::Assignment {
                name: String::from("y"),
                expression: Expression::Number(1),
            }),
            alternative: Box::new(Statement::Assignment {
                name: String::from("y"),
                expression: Expression::Number(2),
            }),
        },
        environment: environment,
    };
    machine.run();
}
