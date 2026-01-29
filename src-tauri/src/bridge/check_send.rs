
#[cfg(test)]
mod tests {
    use crate::bridge::CodeBuddyAdapter;

    #[test]
    fn check_send_sync() {
        fn is_send<T: Send>() {}
        fn is_sync<T: Sync>() {}

        is_send::<CodeBuddyAdapter>();
        is_sync::<CodeBuddyAdapter>();
    }
}
