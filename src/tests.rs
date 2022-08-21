use std::collections::VecDeque;

#[cfg(test)]
use crate::tokenizer;
use crate::{parser, expression_parser};
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
    "#, r#"
    import something
    import something2 as s2;

    @Save
    let allPlayers = Set<UUID>();

    fn func1() return 2;

    @Unsafe
    fn func2(a: Num, b: Num = 0): Int {
        let c = 2                           // a is type Int
        let d: Num = 1;

        if (c > a || c > b) c = a;
        else if (d > a) d = a;
        else a = c + d;

        return a + b * c / d;
    }

    fn func3(a: List<Num>): Num {
        let smallest = 0;
        for (elem in a)
            if (elem > smallest)
                smallest = elem;
        return smallest;
    }



    trait Iterator<T> {
        fn hasNext(self): bool;
        fn next(self): T;
    }

    @Iterator(/* Stuff~! */)
    struct Grid : Iterator<Loc> {
        curX: Int = startX,
        curY: Int = startY,
        curZ: Int = startZ,
        startX: Int,
        startY: Int,
        startZ: Int,
        endX: Int,
        endY: Int,
        endZ: Int

        fn hasNext(self): bool {
            return self.curX<self.endX && self.curY<self.endY && self.curZ<self.endZ;
        }

        fn next(self): Loc {
            let out = Loc(self.curX, self.curY, self.curZ);
            self.curX += 1;
            if (self.curX > self.endX) {
                self.curX = self.startX;
                self.curY += 1;
                if (self.curY > self.endY) {
                    self.curY = self.startY;
                    self.curZ += 1;
                    if (self.curZ > self.endZ) {
                        self.curZ = 0;
                    }
                }
            }
            return out;
        }

        fn from(a: Loc, b: Loc): Grid {
            return Grid {
                startX : floor(min(a.x, b.x)),
                startY : floor(min(a.y, b.y)),
                startZ : floor(min(a.z, b.z)),
                endX : floor(max(a.x, b.x)),
                endY : floor(max(a.y, b.y)),
                endZ : floor(max(a.z, b.z))
            }
        }
    }

    enum Gender: String {
        F = "Female",
        M = "Male",
        O = "Other",
    }
"#];

const EXPR_TESTS: &[&str] = &[ "
    2 + 2
", "
    1 + 2 * 3
", "
    2 * 3 + 1
", "
    myList[2]
", r#"
    myDictionary["apple" & "2"]
"#, "
    var
", "
    var.inside
", "
    var.method()
","
    function()
", "
    []
", "
    [2 + 2]
", "
    [2, 3, 4]
", "
    {}
", "
    { a: 5 }
", "
    { b : 2, c : 3 }
", "
    { nestedArray: [ 2 + 2 ], nestedDict: { b : 9 } }
" ];


#[test]
fn test_tokenizer() {
    for t in TESTS {
        let mut tokens = tokenizer::tokenizer(t).unwrap();
        let table: Vec<String> = tokens.iter().map(|f| {
            format!("{:#?}", f)
        }).collect();
        println!("{}", table.to_table().join("\n"));
    }
}

#[test]
fn test_parser() {
    for t in EXPR_TESTS {
        println!("{}\n{:?}\n", t, expression_parser::parse_expression(&mut VecDeque::from(tokenizer::tokenizer(t).unwrap())).unwrap());
    }
}