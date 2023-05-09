use rusty_money::Money;
use rusty_money::iso::Currency;
use chrono::naive::NaiveDate;

#[derive(Debug)]
pub struct ErstelleRechnungMitGrunddaten<'a> {
    pub rechnungs_nummer: String,
    pub rechnungs_datum: Option<NaiveDate>,
    pub betrag: Money<'a, Currency>,
}

impl<'a> ErstelleRechnungMitGrunddaten<'a> {
    pub fn new(rechnungs_nummer: String, rechnungs_datum: Option<NaiveDate>, betrag: Money<Currency>) -> ErstelleRechnungMitGrunddaten {
        ErstelleRechnungMitGrunddaten {
            rechnungs_nummer,
            rechnungs_datum,
            betrag,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Betrag {}\nRechnung # {}", &self.rechnungs_nummer, &self.betrag)
    }

    pub fn to_string_verbose(&self) -> String {
        format!("{:?}", &self)
    }
}

#[cfg(test)]
mod tests {
    use rusty_money::{Money, iso};
    use chrono::{Datelike, naive::NaiveDate};

    use crate::commands::erstelle_rechnung_command::ErstelleRechnungMitGrunddaten;

    #[test]
    fn erstelle_rechnung_command_test() {
        let datum = NaiveDate::from_ymd_opt(2023, 5, 3);

        // let re = ErstelleRechnung {
        //     rechnungs_nummer: String::from("RE-12"),
        //     rechnungs_datum: datum,
        //     betrag: Money::from_str("4009,09", iso::EUR).unwrap()
        // };

        let re = ErstelleRechnungMitGrunddaten::new(
            String::from("RE-12"),
            datum,
            Money::from_str("4009,09", iso::EUR).unwrap()
        );

        assert_eq!("RE-12", re.rechnungs_nummer);
        assert_eq!("€4.009,09", re.betrag.to_string());

        // assert_eq!(re.datum.to_string(), "34");

        let from_ymd_opt = NaiveDate::from_ymd_opt;

        assert!(from_ymd_opt(
            re.rechnungs_datum.unwrap().year(),
            re.rechnungs_datum.unwrap().month(),
            re.rechnungs_datum.unwrap().day())
            .is_some());

        // TODO: Move to app somewhere
        println!("{}",re.to_string());
        println!("{}",re.to_string_verbose());
    }
}