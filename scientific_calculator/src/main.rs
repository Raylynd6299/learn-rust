use regex::Regex;

fn make_operation(reg: Regex, mut expression: String, operation: &str) -> String {
    if operation.is_empty() {
        return "".to_string();
    }

    loop {
        let captures = reg.captures(expression.as_str());

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();

        let cap_expression = captures.get(0).unwrap().as_str();

        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

        let result = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };

        expression = expression.replace(cap_expression, &result.to_string());
    }

    expression
}

fn operate_expression(mut expression: String) -> String {
    // Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_res = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();

    expression = make_operation(re_mult, expression, "*");
    expression = make_operation(re_div, expression, "/");
    expression = make_operation(re_add, expression, "+");
    expression = make_operation(re_res, expression, "-");

    expression
}

fn main() {
    // User data
    let mut expression = String::new();
    println!("Ingrese su calculo");

    std::io::stdin().read_line(&mut expression).unwrap();

    expression = operate_expression(expression);

    println!("Result: {expression}");
}
