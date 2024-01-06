trait Frame<T> {
    fn serialize(&self) -> &[u8] {
        return "hello".as_bytes();
    }

    fn deserialize(&self, frame: T) -> T {}
}
