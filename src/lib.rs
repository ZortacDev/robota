mod fsm;
mod job;
mod runner;
mod publishers;
mod context;

type Error = ();
type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
