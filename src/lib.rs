pub fn search<'a>(query : & 'a str, file_data : & 'a str) -> Vec<& 'a str>{
    let mut result_vec = Vec::new();
    for line in file_data.lines(){
        if line.contains(query){
            result_vec.push(line);
        }
    }

    return result_vec;
}

pub fn search_case_insensitive<'a>(query : & 'a str, file_data : & 'a str) -> Vec<& 'a str>{
    let mut result_vec = Vec::new();
    let query = query.to_lowercase();
    for line in file_data.lines(){
        if line.to_lowercase().contains(&query){
            result_vec.push(line);
        }
    }

    return result_vec;
}

#[cfg(test)]
mod test{
    use crate::{search, search_case_insensitive};

    #[test]
    fn test_one(){
        let query = "English";
        let content = "William Shakespeare was an English playwright, \npoet and actor. He is widely regarded as the greatest writer in the \nEnglish language and the world's pre-eminent dramatist.";

        assert_eq!(vec!["William Shakespeare was an English playwright, ",
        "English language and the world's pre-eminent dramatist."], search(query, content));
    }

    #[test]
    fn test_two_case_insensitive(){
        let query = "Tree";
        let content = "Tree House, \nTree frame, \nOrange tree";

        assert_eq!(vec!["Tree House, ", "Tree frame, ", "Orange tree"], search_case_insensitive(query, content));
    }
}