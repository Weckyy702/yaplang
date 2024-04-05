use log::info;
use yaplang::lexer::Lexer;

fn main() {
    env_logger::init();

    let l = Lexer::new(
        r#"            
# this is a comment
        := a 1
:= b 2
:= c (+ a b)
∈ c ℕ
"#
        .chars(),
    );

    for t in l {
        info!("{t:?}");
    }
}
