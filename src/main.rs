macro_rules! my_assert_eq {
    // パターンを定義し、パターンマッチした対象に適用するテンプレートを書く
    ($left:expr , $right:expr) => ({
        // {} の中身がテンプレート
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`)", left_val, right_val);
                }
            }
        }
    });
}
fn main() {
    my_assert_eq!(1, 1);
}
