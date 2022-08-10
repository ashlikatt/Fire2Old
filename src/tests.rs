#[cfg(test)]
use crate::tokenizer;


#[test]
fn test_tokenizer() {
    let tests = [ r"
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

    for t in tests {
        println!("{}\n{:?}\n", t, tokenizer::tokenizer(t));
    }
}