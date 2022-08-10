#[cfg(test)]
use crate::tokenizer;


#[test]
fn test_tokenizer() {
    let tests = [ "
    fn average(a: Num, b: Num): Num {
        set = (a+b)/2;
    }
    ", "
    proc onJoin(e: OnJoinEvent) {
        select ALL;
        sendMessage(e.player & ' joined!');
    }
    " ];

    for t in tests {
        println!("{}\n{:?}\n", t, tokenizer::tokenizer(t));
    }
}