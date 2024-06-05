use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq)]
enum Chorus {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eight,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
}

impl Chorus {
    fn as_str(&self) -> &str {
        match self {
            Chorus::First => "first",
            Chorus::Second => "second",
            Chorus::Third => "third",
            Chorus::Fourth => "fourth",
            Chorus::Fifth => "fifth",
            Chorus::Sixth => "sixth",
            Chorus::Seventh => "seventh",
            Chorus::Eight => "eight",
            Chorus::Ninth => "ninth",
            Chorus::Tenth => "tenth",
            Chorus::Eleventh => "eleventh",
            Chorus::Twelfth => "twelfth",
        }
    }

    fn phrase(&self) -> &str {
        match self {
            Chorus::First => "A partridge in a pear tree",
            Chorus::Second => "Two turtle doves",
            Chorus::Third => "Three French hens",
            Chorus::Fourth => "Four calling birds",
            Chorus::Fifth => "Five golden rings",
            Chorus::Sixth => "Six geese a-laying",
            Chorus::Seventh => "Seven swans a-swimming",
            Chorus::Eight => "Eight maids a-milking",
            Chorus::Ninth => "Nine ladies dancing",
            Chorus::Tenth => "Ten lords a-leaping",
            Chorus::Eleventh => "Eleven pipers piping",
            Chorus::Twelfth => "Twelve drummers drumming",
        }
    }
}

pub fn sing() {
    let mut repeat: Vec<String> = vec![];
    for chorus in Chorus::iter() {
        println!("For the {} day of Christmas,", chorus.as_str());
        println!("My true love gave to me");
        if chorus == Chorus::First {
            println!("A partridge in a pear tree.");
        } else {
            repeat.push(String::from(chorus.phrase()));
            for phrase in repeat.iter().rev() {
                println!("{},", phrase);
            }
            println!("And a partridge in a pear tree.");
        }
        println!()
    }
}
