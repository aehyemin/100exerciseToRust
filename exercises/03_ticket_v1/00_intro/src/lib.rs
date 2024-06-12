fn intro() -> &'static str {
    // TODO: fix me 👇
<<<<<<< HEAD
<<<<<<< HEAD
    "I'm ready to start modelling a software ticket!"
=======
    "I'm ready to start modelling a software ticket!
<<<<<<< HEAD
<<<<<<< HEAD
    git test3"
>>>>>>> 28aae8d (git test3)
=======
    git test4"
>>>>>>> 349fe73 (test4)
=======
    git test5"
>>>>>>> 0f69a6a (git test5)
=======
    "I'm ready to start modelling a software ticket!"
>>>>>>> 7e38abd (3-3 modules)
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to start modelling a software ticket!");
    }
}
