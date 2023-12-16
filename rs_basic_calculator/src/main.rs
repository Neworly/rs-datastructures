use std::{cell::Cell, collections::HashMap};

/// defines standard operations for our calculator type to inherit. 
trait Tools {
    type NOne;
    type NTwo;
    type NResult;

    fn addiction(n1: Self::NOne, n2: Self::NTwo) -> Self::NResult;
    fn subtract(n1: Self::NOne, n2: Self::NTwo) -> Self::NResult;
    fn product(n1: Self::NOne, n2: Self::NTwo) -> Self::NResult;
    fn divide(n1: Self::NOne, n2: Self::NTwo) -> Self::NResult;
}

struct Calculator { 
    sum: Cell<f64>, 
    accumulate: Cell<bool>, 
    operators: HashMap<String, fn(<Calculator as Tools>::NOne, <Calculator as Tools>::NTwo) -> <Calculator as Tools>::NResult> 
}

impl Tools for Calculator {
    type NOne = f64;
    type NTwo = f64;
    type NResult = f64;

    fn addiction(n1: Self::NOne, n2: Self::NTwo) -> Self::NResult {
        return n1 + n2;
    }

    fn subtract(n1: Self::NOne, n2: Self::NTwo) -> Self::NResult {
        return n1 - n2;
    }

    fn product(n1: Self::NOne, n2: Self::NTwo) -> Self::NResult {
        return n1 * n2;
    }    

    fn divide(n1: Self::NOne, n2: Self::NTwo) -> Self::NResult {
        return n1 / n2
    }

}

impl Calculator {
    /// Initialise default values and 'operators'.
    /// 
    /// Each operator is mapped in a 'key/pointer' format
    fn default() -> Self {
        let mut operators: HashMap<String, fn(<Calculator as Tools>::NOne, <Calculator as Tools>::NTwo) -> <Calculator as Tools>::NResult> = HashMap::new();
        
        operators.insert("add".to_string(), Calculator::addiction);
        operators.insert("sub".to_string(), Calculator::subtract);
        operators.insert("pro".to_string(), Calculator::product);
        operators.insert("div".to_string(), Calculator::divide);

        return Calculator { sum: Cell::new(0.0), accumulate: Cell::new(false), operators }
    }

    /// Search for a specified operator.
    fn try_operator(&self, operator: &str) -> Option<&fn(<Calculator as Tools>::NOne, <Calculator as Tools>::NTwo) -> <Calculator as Tools>::NResult> {
        return self.operators.get(operator)
    }

    /// Explicitely call a specified operator and return back their value.
    fn use_this(&self, operator: &str, x: f64, y: f64) -> f64 {
        return self.operators[operator](x, y)
    }    
}


fn main() {
    let calc: Calculator = Calculator::default();

    let stop_when = 273498327.3;

    loop {
        let user_operator_choice = "pro";

        let x = 3.24;
        let y = 3.35;

        if calc.try_operator(user_operator_choice).is_none() {
            println!("{user_operator_choice}' is not valid.");
            continue;
        }

        if calc.sum.get() >= stop_when {
            break;
        }

        if calc.accumulate.get() == true  {
            calc.sum.set(calc.use_this(user_operator_choice, calc.sum.get(), x));
            println!("consecutive sum sum: {}", calc.sum.get());
            continue
        }        

        calc.sum.set(calc.use_this(user_operator_choice, x, y));
        calc.accumulate.set(true);

        println!("start sum: {}", calc.sum.get());
    }
}
