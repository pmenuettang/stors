use rocstr::rocstr::RocStr;
use rust_decimal::Decimal;
use time::Date;
use uuid::Uuid;

use crate::Id;

// Workaround for `aquamarine` inner doc-comments.
// See [https://github.com/mersinvald/aquamarine/issues/5].
/// # Item
#[cfg_attr(doc, aquamarine::aquamarine)]
/// include_mmd!("diagrams/item.mmd")
pub mod item {
    pub use super::*;
}

/// A typed ID for `Item`
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ItemId(Uuid);

impl Id for ItemId {
    fn id(&self) -> uuid::Uuid {
        self.0
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Item {
    id: ItemId,
    name: RocStr<64>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PriceId(Uuid);

pub struct Price {
    id: PriceId,
    item: ItemId,
    start_date: Date,
    end_date: Date,
    amount: Decimal,
    vat: Decimal,
}

impl Price {
    pub fn new(
        id: PriceId,
        item: ItemId,
        start_date: Date,
        end_date: Date,
        amount: Decimal,
        vat: Decimal,
    ) -> Self {
        Self {
            id,
            item,
            start_date,
            end_date,
            amount,
            vat,
        }
    }

    pub fn id(&self) -> PriceId {
        self.id
    }
    pub fn item(&self) -> ItemId {
        self.item
    }
    pub fn start_date(&self) -> Date {
        self.start_date
    }
    pub fn end_date(&self) -> Date {
        self.end_date
    }
    pub fn amount(&self) -> Decimal {
        self.amount
    }
    pub fn vat(&self) -> Decimal {
        self.vat
    }
}

impl Id for PriceId {
    fn id(&self) -> uuid::Uuid {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_price_with_valid_values_should_contains_parameters_values() {
        let id = PriceId(Uuid::new_v4());
        let item = ItemId(Uuid::new_v4());
        let start_date = Date::MIN;
        let end_date = Date::MAX;
        let amount = Decimal::new(0, 0);
        let vat = Decimal::new(2, 1);

        let price = Price::new(id, item, start_date, end_date, amount, vat);

        assert_eq!(id, price.id, "❌ Price id differs from parameter.");
        assert_eq!(item, price.item, "❌ Price item differs from parameter.");
        assert_eq!(
            start_date, price.start_date,
            "❌ Price start_date differs from parameter."
        );
        assert_eq!(
            end_date, price.end_date,
            "❌ Price end_date differs from parameter."
        );
        assert_eq!(
            amount, price.amount,
            "❌ Price amount differs from parameter."
        );
        assert_eq!(vat, price.vat, "❌ Price vat differs from parameter.");
    }
}
