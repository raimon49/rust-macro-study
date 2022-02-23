macro_rules! my_assert_eq {
    // パターンを定義し、パターンマッチした対象に適用するテンプレートを書く
    // フラグメント型が「:expr」なので、$leftも$rightも引数に式が来ることを期待している
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

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    my_assert_eq!(1, 1);

    // マクロを呼び出す時の括弧には慣例で()が使われるが、[]でも{}でも呼び出せる
    my_assert_eq!(gcd(6, 10), 2);
    my_assert_eq![gcd(6, 10), 2];
    my_assert_eq!{gcd(6, 10), 2};
    // 中括弧呼び出しの時だけセミコロンを省略可能
    my_assert_eq!{gcd(6, 10), 2}

    let vec_1 = vec![1, 2, 3];
    let vec_2 = vec!(1, 2, 3);
    let vec_3 = vec!{1, 2, 3};
    assert_eq!(vec_1, vec_2);
    assert_eq![vec_1, vec_3];
    assert_eq!{vec_2, vec_3}
}
