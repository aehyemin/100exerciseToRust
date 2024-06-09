fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to start modelling a software ticket!
    git test4"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to start modelling a software ticket!");
    }
}
