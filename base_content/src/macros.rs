// 定义一个宏
macro_rules! vec {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            ${
                temp_vec.push($x);
            }*
            temp_vec
        }
    };
}

macro_rules! o_O {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
        &[ $($( $x + $y ),*),* ]
    }
}

fn main() {
    assert_eq!(vec![1, 2, 3], [1, 2, 3]);
    let a: &[i32] = o_O!(10; [1, 2, 3];
                         20; [4, 5, 6]);
    assert_eq!(a, [11, 12, 13, 24, 25, 26]);
}

macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

fn main() {
    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
        html[
            head[title["Macros guide"]]
            body[h1["Macros are the best!"]]
        ]);
    
    assert_eq!(out, 
        "<html><head><title>Macros guide</title></head>\
        <body><h1>Macros are the best!</h1></body></html>");
}