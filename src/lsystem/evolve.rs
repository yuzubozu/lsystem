use std::collections::HashMap;

pub fn evolve(gen:String,rules:HashMap<String,String>)->String{
    return gen.chars().map(|c| match rules.get(&(c.to_string())
){
        Some(s) => s.to_string(),
        None => {
            c.to_string()
        }
    }).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evolve_test() {
        let premise = String::from("abcabc");
        let mut rules = HashMap::new();
        rules.insert(String::from("a"),String::from("ab"));
        rules.insert(String::from("c"),String::from("ca"));
        let result = evolve(premise,rules.clone());
        assert_eq!("abbcaabbca", result);
        let result2 = evolve(result,rules.clone());
        assert_eq!("abbbcaababbbcaab", result2);
    }
}