#[cfg(test)]
use crate::tokenizer;
use crate::parser;
use formatted_debug::table_formatting::StringTable;


const TESTS: &[&str] = &[ r"
    fn average(a: Num, b: Num): Num {
        set = (a+b)/2;
    }
    ", r"
    proc onJoin(e: OnJoinEvent) {
        select ALL;
        sendMessage(e.player & ' joined!');
    }
    ", r#"
    "Hello!",
    "\"Hello!\"",
    'UwU',
    '"OwO"',
    "Yes~!\nYess~!\nYesss~!"
    "#, r#"
    @Save uniquePlayers: List = emptyList();

    let GEAR_SWORD: Item = createItem("stone_sword").setName("Evil Sword").hideflags().unbreakable();

    // Handled by event
    proc onJoin(e: OnJoinEvent) {
        select DEFAULT; // Unnecessary
        giveGear();
        if (uniquePlayers.contains(e.uuid)) {
            uniquePlayers.add(e.uuid);
        }
    }

    fn giveGear() {
        TARGET::giveItems(GEAR_SWORD);
    }
    "# ];


#[test]
fn test_tokenizer() {
    for t in TESTS {
        let mut tokens = tokenizer::tokenizer(t).unwrap();
        let table: Vec<String> = tokens.iter().map(|f| {
            format!("{:?}", f)
        }).collect();
        println!("{}", table.to_table().join("\n"));
    }
}

#[test]
fn test_parser() {
    for t in TESTS {
        //println!("{}\n{:?}\n", t, parser::parse_tokens(&tokenizer::tokenizer(t).unwrap()));
    }
}