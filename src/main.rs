#[feature(trace_macros)]
macro_rules! my_assert_eq {
    // パターンを定義し、パターンマッチした対象に適用するテンプレートを書く
    // フラグメント型が「:expr」なので、$leftも$rightも引数に式が来ることを期待している
    ($left:expr , $right:expr) => ({
        // {} の中身がテンプレート
        match (&$left, &$right) {
            (left_val, right_val) => {
                // マクロ呼び出し時に所有権が移動しないよう参照で比較
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`)", left_val, right_val);
                }
            }
        }
    });
}

macro_rules! my_vec {
    ($elem:expr ; $n:expr) => {
        ::std::vec::from_elem($elem, $n)
    };
    // 0個以上のカンマで区切られた呼び出しにマッチ
    ( $( $x:expr ),* ) => {
        <[_]>::into_vec(Box::new([ $( $x ),* ]))
        // 以下と同義
        // let mut v = Vec::new();
        // $( v.push($x); )*
        // v
    };
    // 1個以上のカンマで区切られた呼び出しにマッチ
    // （最後に余分なカンマがあったら切り離して再起呼び出し）
    ( $( $x:expr ),+ ,) => {
        my_vec![ $( $x ),* ]
    };
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

use std::collections::HashMap;
#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

macro_rules! json {
    (null) => {
        Json::Null
    };
    ([ $( $element:expr ),* ]) => {
        Json::Array(vec![ $( $element ),* ])
    };
}

#[test]
fn json_null() {
    my_assert_eq!(json!(null), Json::Null);
}

fn main() {
    // trueで読みだすと定義したマクロ呼び出しの展開前と展開後のコードを出力する
    // trace_macros!(true);
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
    assert_eq!(vec_1, vec_2); // assert_eqマクロを()で呼ぶのも慣例で、実際は[]でも{}でも呼び出せる
    assert_eq![vec_1, vec_3];
    assert_eq!{vec_2, vec_3}

    let _buffer = my_vec![0_u8; 1000]; // 値を1,000回繰り返して生成
    let _numbers = my_vec!["udon", "ramen", "soba"]; // カンマで区切られた値のリストで生成
    let _recursive_call = my_vec!["udon", "ramen",]; // ケツカンマに対応したマクロ記述のため通る
    my_assert_eq!(_recursive_call, my_vec!["udon", "ramen"]);

    let version = env!("CARGO_PKG_VERSION");
    println!("CARGO_PKG_VERSION: {}", version);
    let undefined_variable = option_env!("NOT_DEFINED");
    my_assert_eq!(undefined_variable, None);

    const CARGO_TOML: &str = include_str!("../Cargo.toml");
    println!("Cargo.toml:\n\n {}", CARGO_TOML);

    let students = Json::Array(vec![
    Json::Object(Box::new(vec![
        ("name".to_string(), Json::String("Jim Blendy".to_string())),
        ("class_of".to_string(), Json::Number(1926.0)),
        ("major".to_string(), Json::String("Tibetan throat singing".to_string()))
    ].into_iter().collect()))
    ]);
}
