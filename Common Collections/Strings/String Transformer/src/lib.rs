pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}


pub mod transformer {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input:Vec<(String,Command)>) -> Vec<String> {

        let l=input.len();
        let mut output = vec![String::from("dd");l];
        for i in 0..l{
            match &input[i]{
                (s,Command::Uppercase) => {output[i]=s[..].to_uppercase()}
                (s,Command::Trim)=>{output[i]=s.trim().to_string()}
                (s,Command::Append(n))=>{output[i]=(*s).clone();
                for j in 0..*n as i32{
                    output[i].push_str("bar");
                }
                }
            }
        }

        output
    }
}