pub mod functions {
    use chrono::Duration;
    use rand::{thread_rng, Rng};

    const CODES: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    pub fn gen_random_code(len: i8) -> String {
        let mut res: Vec<char> = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..len {
            let code = CODES[rng.gen_range(0..CODES.len())];
            res.push(code)
        }
        String::from_iter(res)
    }
    
    pub fn gen_expire_time(timestamp: Duration) -> usize {
        let now = chrono::Local::now() + timestamp;
        now.timestamp() as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::functions::gen_random_code;

    #[test]
    fn test_random_code() {
        let code = gen_random_code(6);
        assert_eq!(code.len(), 6);
    }
}
