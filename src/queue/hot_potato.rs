fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut queue = names.clone();
    queue.reverse();

    while queue.len() > 1 {
        for _i in 0..num {
            let name = queue.pop().unwrap();
            queue.insert(0, name);
        }

        let _ = queue.pop();
    }
    return queue.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hot_potato_should_work() {
        let names = vec!["Mon", "Tom", "Kew", "Lisa", "Marry", "Bob"];
        let survivor = hot_potato(names, 8);
        assert_eq!(survivor, "Marry");
    }
}
