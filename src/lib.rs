#[cfg(test)]
mod peekable {
    #[test]
    fn basic_usage() {
        let v = vec![1, 2, 3];
        let mut iter = v.into_iter().peekable();
        // 查看下一个元素
        if let Some(&next) = iter.peek() {
            println!("Next value: {}", next); // 输出 1
            assert_eq!(next, 1);
        }

        // 消耗元素
        println!("Consumed: {:?}", iter.next()); // 输出 1

        // 再次查看
        if let Some(&next) = iter.peek() {
            println!("Next value: {}", next); // 输出 2
            assert_eq!(next, 2);
        }
    }
    
    #[test]
    fn skip_element() {
        let v = vec![1, 2, 3, 4, 5];
        let mut iter = v.into_iter().peekable();

        while let Some(&value) = iter.peek() {
            if value % 2 == 0 {
                println!("Skipping: {}", value);
                matches!(value, 2 | 4);
                iter.next(); // 消耗偶数
            } else {
                matches!(value, 1 | 3 | 5);
                println!("Processing: {}", value);
                iter.next(); // 消耗奇数
            }
        }
    }
    
    #[test]
    /// matches! 宏，但其模式匹配仅适用于结构解构或简单值匹配，而不能直接用于复杂的比较逻辑（如 String 的内容比较）
    /// assert_eq!(matches!(word, String::from("hello") | String::from("world")));
    /// assert!(word == "hello" || word == "world");
    /// assert!(["hello", "world"].contains(&word.as_str()));
    fn parse_symbol() {
        let input = "hello,world!";
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            if ch.is_alphabetic() {
                let mut word = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphabetic() {
                        word.push(ch);
                        chars.next(); // 消耗字母
                    } else {
                        break;
                    }
                }
                assert!(["hello", "world"].contains(&word.as_str()));
                println!("Word: {}", word);
            } else if ch.is_ascii_punctuation() {
                println!("Punctuation: {}", ch);
                assert!(matches!(ch, ',' | '!'));
                chars.next(); // 消耗标点符号
            } else {
                chars.next(); // 跳过其他字符
            }
        }
    }
    
    #[test]
    fn alter_next() {
        let v = vec![1, 2, 3];
        let mut iter = v.into_iter().peekable();

        if let Some(next) = iter.peek_mut() {
            *next *= 10; // 修改缓存的值
        }
        if let Some(next) = iter.next() {
            assert_eq!(next, 10);
            println!("First value after modification: {}", next); // 输出 10
        }
    }
}
