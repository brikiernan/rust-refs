use std::error::Error;
use std::fmt;

// ------------ TextTransformer Trait ------------

trait TextTransformer {
    fn transform(&self, input: &str) -> Result<String, Box<dyn Error>>;
}

// ------------ Custom Error Types ------------

#[derive(Debug)]
struct PizzaError;

impl fmt::Display for PizzaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hey, there's a pizza emoji in the text. So cheesy.")
    }
}

impl Error for PizzaError {}

#[derive(Debug)]
struct EmptyStringError;

impl fmt::Display for EmptyStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The string has nothing left in it.")
    }
}

impl Error for EmptyStringError {}

// ------------ WhitespaceTransformer ------------

struct WhitespaceTransformer {
    start: bool,
    end: bool,
}

impl TextTransformer for WhitespaceTransformer {
    fn transform(&self, input: &str) -> Result<String, Box<dyn Error>> {
        if input.contains('🍕') {
            return Err(Box::new(PizzaError));
        }

        let transformed = if self.start && self.end {
            input.trim()
        } else if self.start {
            input.trim_start()
        } else if self.end {
            input.trim_end()
        } else {
            input
        };

        if transformed.is_empty() {
            return Err(Box::new(EmptyStringError));
        }

        Ok(transformed.to_string())
    }
}

// ------------ CaseTransformer ------------

enum Case {
    Uppercase,
    Lowercase,
}

struct CaseTransformer {
    case: Case,
}

impl TextTransformer for CaseTransformer {
    fn transform(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let transformed = match self.case {
            Case::Uppercase => input.to_uppercase(),
            Case::Lowercase => input.to_lowercase(),
        };
        Ok(transformed)
    }
}

// ------------ apply_transformations Function ------------

fn apply_transformations(text: String, pipeline: Vec<Box<dyn TextTransformer>>) -> String {
    let mut current_text = text;
    for transformer in pipeline {
        match transformer.transform(&current_text) {
            Ok(transformed) => {
                current_text = transformed;
            }
            Err(e) => {
                eprintln!("Error Message: Something went wrong: {e}. Moving on to next transform");
            }
        }
    }
    current_text
}

// ------------ apply_transformations_fold Function ------------

fn apply_transformations_fold(text: String, pipeline: Vec<Box<dyn TextTransformer>>) -> String {
    pipeline.into_iter().fold(text, |accumulator, transformer| {
        match transformer.transform(&accumulator) {
            Ok(transformed) => transformed,
            Err(e) => {
                eprintln!("Error Message: Something went wrong: {e}. Moving on to next transform");
                accumulator
            }
        }
    })
}

fn main() {
    // Input
    let text = String::from("  homer simpson  ");
    // Output
    // Content: "HOMER SIMPSON"

    let pipeline: Vec<Box<dyn TextTransformer>> = vec![
        Box::new(WhitespaceTransformer {
            start: true,
            end: true,
        }),
        Box::new(CaseTransformer {
            case: Case::Uppercase,
        }),
    ];

    let transformed_text = apply_transformations(text, pipeline);
    println!("Output: {transformed_text}");

    // Input
    let text = String::from("  data  🍕  ");
    // Output
    // Error Message: Something went wrong: Hey, there's a pizza emoji in the text. So cheesy. Moving on to next transform
    // Content: "  DATA  🍕  "

    let pipeline: Vec<Box<dyn TextTransformer>> = vec![
        Box::new(WhitespaceTransformer {
            start: true,
            end: true,
        }),
        Box::new(CaseTransformer {
            case: Case::Uppercase,
        }),
    ];

    let transformed_text = apply_transformations(text, pipeline);
    println!("Output: {transformed_text}");

    // Input
    let text = String::from("    ");
    // Output:
    // Error Message: Something went wrong: The string has nothing left in it. Moving on to next transform
    // Content: "    "

    let pipeline: Vec<Box<dyn TextTransformer>> = vec![
        Box::new(WhitespaceTransformer {
            start: true,
            end: true,
        }),
        Box::new(CaseTransformer {
            case: Case::Uppercase,
        }),
    ];

    let transformed_text = apply_transformations(text, pipeline);
    println!("Output: {transformed_text}");
}
