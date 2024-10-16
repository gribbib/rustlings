// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> { 
        let mut return_vector: Vec<String> = Vec::new();
        for tuple in input {
            match tuple.1 {
                Command::Uppercase => return_vector.push(tuple.0.to_uppercase()),
                Command::Trim => return_vector.push(tuple.0.trim().to_string()),
                Command::Append(n) => return_vector.push(append(tuple.0, n))
            }
        }
        return return_vector;
     }

     fn append(input: String, count: usize) -> String {
        if count == 1 {
            input + "bar"
        } else if count > 1 {
            let input = input + "bar";
            append(input, count-1)
        }else{
            input
        }
        // let return_string: String = String::new;
        // for i in 1..count {
        //     return_string = return_string + "bar"
        // }
        // return_string
     }
}

fn main() {
    // You can optionally experiment here.
    use my_module::transformer;
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);
        println!("{:?}",output)
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
