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

    println!("--");

    let mut machine = Machine {
        statement: Statement::Sequence {
            first: Box::new(Statement::Assignment {
                name: String::from("x"),
                expression: Expression::Add {
                    left: Box::new(Expression::Number(1)),
                    right: Box::new(Expression::Number(1)),
                },
            }),
            second: Box::new(Statement::Assignment {
                name: String::from("y"),
                expression: Expression::Add {
                    left: Box::new(Expression::Variable(String::from("x"))),
                    right: Box::new(Expression::Number(3)),
                },
            }),
        },
        environment: HashMap::new(),
    };
    machine.run();

    println!("--");

    let mut environment = HashMap::new();
    environment.insert(String::from("x"), Expression::Number(1));
    let mut machine = Machine {
        statement: Statement::While {
            condition: Expression::LessThan {
                left: Box::new(Expression::Variable(String::from("x"))),
                right: Box::new(Expression::Number(5)),
            },
            body: Box::new(Statement::Assignment {
                name: String::from("x"),
                expression: Expression::Multiply {
                    left: Box::new(Expression::Variable(String::from("x"))),
                    right: Box::new(Expression::Number(3)),
                },
            }),
        },
        environment: environment,
    };
    machine.run();

    println!("-- big step --");

    println!("{}", Expression::Number(23).evaluate(&HashMap::new()));

    let mut environment = HashMap::new();
    environment.insert(String::from("x"), Expression::Number(23));
    println!(
        "{}",
        Expression::Variable(String::from("x")).evaluate(&environment)
    );

    let mut environment = HashMap::new();
    environment.insert(String::from("x"), Expression::Number(2));
    environment.insert(String::from("y"), Expression::Number(5));

    let exp = Expression::LessThan {
        left: Box::new(Expression::Add {
            left: Box::new(Expression::Variable(String::from("x"))),
            right: Box::new(Expression::Number(2)),
        }),
        right: Box::new(Expression::Variable(String::from("y"))),
    };
    println!("{}", exp.evaluate(&environment));

    println!("--");

    let statement = Statement::Sequence {
        first: Box::new(Statement::Assignment {
            name: String::from("x"),
            expression: Expression::Add {
                left: Box::new(Expression::Number(1)),
                right: Box::new(Expression::Number(1)),
            },
        }),
        second: Box::new(Statement::Assignment {
            name: String::from("y"),
            expression: Expression::Add {
                left: Box::new(Expression::Variable(String::from("x"))),
                right: Box::new(Expression::Number(3)),
            },
        }),
    };
    println!("{:?}", statement.evaluate(&mut HashMap::new()));

    let statement = Statement::While {
        condition: Expression::LessThan {
            left: Box::new(Expression::Variable(String::from("x"))),
            right: Box::new(Expression::Number(5)),
        },
        body: Box::new(Statement::Assignment {
            name: String::from("x"),
            expression: Expression::Multiply {
                left: Box::new(Expression::Variable(String::from("x"))),
                right: Box::new(Expression::Number(3)),
            },
        }),
    };

    let mut environment = HashMap::new();
    environment.insert(String::from("x"), Expression::Number(1));
    println!("{:?}", statement.evaluate(&mut environment));
}
