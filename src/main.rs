use eyre::Result;
use serde::Serialize;
use erased_serde::Serialize as ErasedSerialize;

trait FarmersMarketStand {
    type Item: FarmersMarketStandItem;
    fn get_item(&self) -> Self::Item;
}

trait FarmersMarketStandItem {
    type SpecificData: Serialize + 'static;
    fn read_description(&self) -> Result<String>;
    fn read_price_in_cents(&self) -> Result<u64>;
    fn read_calories(&self) -> Result<f64>;
    fn read_grams_protein(&self) -> Result<f64>;
    fn read_grams_carbs(&self) -> Result<f64>;
    fn read_grams_fat(&self) -> Result<f64>;
    fn read_grams_alcohol(&self) -> Result<f64>;
    fn read_meat_status(&self) -> Result<MeatStatus>;
    fn read_halal(&self) -> Result<bool>;
    fn read_kosher(&self) -> Result<bool>;
    fn read_market_specific_data(&self) -> Result<Self::SpecificData>;
}

struct StandA;
struct ItemA;

#[derive(Serialize)]
struct AppleData {
    variety: String,
    doctors_kept_away: u32,
}

impl FarmersMarketStand for StandA {
    type Item = ItemA;

    fn get_item(&self) -> Self::Item {
        ItemA
    }
}

impl FarmersMarketStandItem for ItemA {
    type SpecificData = AppleData;

    fn read_description(&self) -> Result<String> {
        Ok("Apples".to_string())
    }

    fn read_price_in_cents(&self) -> Result<u64> {
        Ok(3)
    }

    fn read_calories(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_grams_protein(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_grams_carbs(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_grams_fat(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_grams_alcohol(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_meat_status(&self) -> Result<MeatStatus> {
        Ok(MeatStatus::Veg)
    }

    fn read_halal(&self) -> Result<bool> {
        Ok(true)
    }

    fn read_kosher(&self) -> Result<bool> {
        Ok(true)
    }

    fn read_market_specific_data(&self) -> Result<Self::SpecificData> {
        Ok(AppleData {
            variety: "Gala".to_string(),
            doctors_kept_away: 30,
        })
    }
}

struct StandB;
struct ItemB;

#[derive(Serialize)]
struct BaconData {
    farm_of_origin: String,
    breakfasts_served: u32,
}

impl FarmersMarketStand for StandB {
    type Item = ItemB;
    fn get_item(&self) -> Self::Item {
        ItemB
    }
}

impl FarmersMarketStandItem for ItemB {
    type SpecificData = BaconData;

    fn read_description(&self) -> Result<String> {
        Ok("Bacon".to_string())
    }

    fn read_price_in_cents(&self) -> Result<u64> {
        Ok(3000)
    }

    fn read_calories(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_grams_protein(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_grams_carbs(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_grams_fat(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_grams_alcohol(&self) -> Result<f64> {
        Ok(10.0)
    }

    fn read_meat_status(&self) -> Result<MeatStatus> {
        Ok(MeatStatus::Meat)
    }

    fn read_halal(&self) -> Result<bool> {
        Ok(false)
    }

    fn read_kosher(&self) -> Result<bool> {
        Ok(false)
    }

    fn read_market_specific_data(&self) -> Result<Self::SpecificData> {
        Ok(BaconData {
            farm_of_origin: "Stolzfus and Sons".to_string(),
            breakfasts_served: 15,
        })
    }
}

fn extract_grocery_data<T: FarmersMarketStand>(
    customer_id: CustomerId,
    item: &T::Item,
)-> Result<GroceryItem> {
    Ok(GroceryItem {
        description: item.read_description()?,
        customer_id,
        price_in_cents: item.read_price_in_cents()?,
        calories: item.read_calories()?,
        grams_protein: item.read_grams_protein()?,
        grams_carbs: item.read_grams_carbs()?,
        grams_fat: item.read_grams_fat()?,
        grams_alcohol: item.read_grams_alcohol()?,
        meat_status: item.read_meat_status()?,
        halal: item.read_halal()?,
        kosher: item.read_kosher()?,
        market_specific_data: Box::new(item.read_market_specific_data()?),
    })
}

#[derive(Serialize)]
pub enum MeatStatus {
    Veg,
    Fish,
    Meat,
}

#[derive(Serialize)]
pub struct CustomerId(pub u64);

#[derive(Serialize)]
pub struct GroceryItem {
    pub description: String,
    pub customer_id: CustomerId,
    pub price_in_cents: u64,
    pub calories: f64,
    pub grams_protein: f64,
    pub grams_carbs: f64,
    pub grams_fat: f64,
    pub grams_alcohol: f64,
    pub meat_status: MeatStatus,
    pub halal: bool,
    pub kosher: bool,
    pub market_specific_data: Box<dyn ErasedSerialize>,
}

fn main() -> Result<()> {
    let mut items = Vec::new();

    let stand_a = StandA;
    let item_a = extract_grocery_data::<StandA>(CustomerId(0), &stand_a.get_item())?;
    items.push(item_a);

    let stand_b = StandB;
    let item_b = extract_grocery_data::<StandB>(CustomerId(1), &stand_b.get_item())?;
    items.push(item_b);

    println!("{}", serde_json::to_string(&items)?);

    Ok(())
}
