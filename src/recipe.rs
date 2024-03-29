use serde::Deserialize;
use serde::export::fmt::Error;
use serde::export::Formatter;

const SEASON_TABLE: [(&'static str, [bool; 12]); 92] = [
    ("apple", [false, false, false, false, false, false, false, true, true, true, true, false]),
    ("apricot", [false, false, false, false, true, true, true, false, false, false, false, false]),
    ("artichoke", [false, false, true, true, true, true, false, false, false, true, true, false]),
    ("asian pear", [false, false, false, false, false, false, false, true, true, true, false, false]),
    ("asparagus", [false, false, true, true, false, false, false, false, false, false, false, false]),
    ("avocado", [false, true, true, true, true, true, true, false, false, false, false, false]),
    ("basil", [false, false, false, false, true, true, true, true, true, true, false, false]),
    ("beet", [true, true, true, false, false, false, false, false, false, false, true, true]),
    ("bell pepper", [false, false, false, false, false, false, false, false, true, true, false, false]),
    ("berry", [false, false, false, true, true, true, true, true, true, true, false, false]),
    ("black-eyed pea", [false, false, false, false, true, true, true, true, true, true, true, false]),
    ("blackberry", [false, false, false, true, true, true, true, true, true, true, false, false]),
    ("blood orange", [false, true, true, false, false, false, false, false, false, false, false, false]),
    ("blueberry", [false, false, false, true, true, true, true, true, true, true, false, false]),
    ("broccoli", [true, true, true, true, true, false, false, false, false, false, false, true]),
    ("brussels sprout", [true, true, false, false, false, false, false, false, false, false, true, true]),
    ("carambola", [true, false, false, false, false, false, false, false, false, true, true, false]),
    ("carrot", [false, true, true, true, true, true, false, false, false, false, false, false]),
    ("cauliflower", [true, true, true, true, true, false, false, false, false, false, false, true]),
    ("chard", [true, true, true, true, false, false, false, false, false, false, false, false]),
    ("cherimoyas", [true, false, false, false, false, false, false, false, false, true, true, false]),
    ("cherry", [false, false, false, false, true, true, true, false, false, false, false, false]),
    ("chili pepper", [false, false, false, false, false, false, false, true, true, true, true, true]),
    ("cilantro", [false, false, false, false, true, true, true, true, true, true, false, false]),
    ("collard green", [false, true, true, true, true, true, true, true, true, true, true, true]),
    ("corn", [false, false, false, false, false, true, true, true, true, false, false, false]),
    ("cucumber", [false, false, false, false, true, true, true, true, true, false, false, false]),
    ("cherry tomato", [false, false, false, true, true, true, true, false, false, false, false, false]),
    ("date", [true, true, true, true, true, false, false, false, false, false, false, false]),
    ("eggplant", [false, false, false, false, false, false, true, true, true, true, false, false]),
    ("english pea", [false, false, true, true, false, false, false, false, false, false, false, false]),
    ("fava bean", [false, false, true, true, true, true, false, false, false, false, false, false]),
    ("fennel", [true, true, true, false, false, false, false, false, false, false, false, false]),
    ("fig", [false, false, false, false, false, true, true, true, false, false, false, false]),
    ("grape", [false, false, false, false, false, false, true, true, true, true, false, false]),
    ("grape tomato", [false, false, false, true, true, true, true, false, false, false, false, false]),
    ("grapefruit", [false, false, true, true, true, true, true, false, false, false, false, false]),
    ("green bean", [false, false, false, false, true, true, true, true, false, false, false, false]),
    ("green garlic", [false, false, true, true, true, false, false, false, false, false, false, false]),
    ("green pea", [true, true, true, true, true, false, false, false, false, false, false, false]),
    ("guava", [false, false, false, false, false, false, false, false, true, true, true, true]),
    ("kale", [true, false, false, false, false, false, false, false, false, false, false, true]),
    ("kiwi", [false, false, false, false, false, false, false, false, false, true, true, true]),
    ("kumquat", [true, true, true, true, true, true, false, false, false, false, false, false]),
    ("lima bean", [false, false, false, false, false, false, false, false, false, true, true, false]),
    ("mandarin", [true, true, true, false, false, false, false, false, false, false, true, true]),
    ("mango", [false, false, false, false, false, false, false, true, true, false, false, false]),
    ("melon", [false, false, false, false, false, false, true, true, true, false, false, false]),
    ("meyer lemon", [true, true, true, true, false, false, false, false, false, false, false, false]),
    ("mint", [false, false, false, false, true, true, true, true, true, true, false, false]),
    ("mulberry", [false, false, false, false, false, true, true, false, false, false, false, false]),
    ("mushroom", [true, true, false, false, false, false, false, false, false, false, true, true]),
    ("navel orange", [true, true, true, true, false, false, false, false, false, false, false, true]),
    ("nectarine", [false, false, false, false, false, false, true, true, true, false, false, false]),
    ("new potato", [false, false, false, true, true, false, false, false, false, false, false, false]),
    ("okra", [false, false, false, false, false, true, true, true, true, true, false, false]),
    ("onion", [false, false, false, false, true, true, true, true, true, true, true, true]),
    ("oregano", [true, true, false, false, false, false, false, false, false, true, true, true]),
    ("parsley", [false, false, false, false, true, true, true, true, true, true, false, false]),
    ("parsnip", [true, false, false, false, false, false, false, false, false, false, true, true]),
    ("passion fruit", [false, false, false, false, false, false, true, true, true, false, false, false]),
    ("peach", [false, false, false, false, false, false, true, true, true, false, false, false]),
    ("pear", [false, false, false, false, false, false, false, true, true, true, false, false]),
    ("persimmon", [false, false, false, false, false, false, false, false, true, true, true, false]),
    ("plum", [false, false, false, false, true, true, true, true, false, false, false, false]),
    ("plumcot", [false, false, false, false, true, true, true, true, false, false, false, false]),
    ("pluot", [false, false, false, false, true, true, true, true, false, false, false, false]),
    ("pomegranate", [false, false, false, false, false, false, false, false, true, true, true, true]),
    ("pumpkin", [false, false, false, false, false, false, false, false, true, true, true, true]),
    ("radish", [true, true, true, true, true, false, false, false, false, false, false, false]),
    ("raspberry", [false, false, false, true, true, true, true, true, true, true, false, false]),
    ("root vegetable", [true, false, false, false, false, false, false, false, false, false, true, true]),
    ("rosemary", [true, true, false, false, false, false, false, false, false, true, true, true]),
    ("rutabaga", [true, false, false, false, false, false, false, false, false, false, true, true]),
    ("sapote", [false, false, false, false, false, false, true, true, true, true, true, false]),
    ("shelling bean", [false, false, false, false, false, false, false, true, true, false, false, false]),
    ("snow pea", [false, true, true, true, true, true, false, false, false, false, false, false]),
    ("soft herbs", [false, false, false, false, true, true, true, true, true, true, false, false]),
    ("special lettuce", [false, true, true, true, true, false, false, false, false, false, false, false]),
    ("spinach", [false, true, true, true, true, true, true, true, true, true, true, true]),
    ("strawberry", [false, false, true, true, true, false, false, false, false, false, false, false]),
    ("sugar snap pea", [false, true, true, true, true, true, false, false, false, false, false, false]),
    ("summer squash", [false, false, false, false, true, true, true, true, true, true, true, true]),
    ("sweet potato", [true, true, true, false, false, false, false, false, false, false, true, true]),
    ("tangerine", [true, true, true, true, false, false, false, false, false, true, true, false]),
    ("thyme", [true, true, false, false, false, false, false, false, false, true, true, true]),
    ("tomatillo", [false, true, true, true, false, false, false, true, true, true, false, false]),
    ("tomato", [false, false, false, false, false, false, true, true, true, true, false, false]),
    ("turnip", [true, false, false, false, false, false, false, false, false, false, true, true]),
    ("valencia orange", [false, false, false, false, true, true, true, true, true, true, true, true]),
    ("winter squash", [false, false, false, false, false, false, false, false, true, true, true, true]),
    ("zucchini", [false, false, false, false, false, false, true, true, true, false, false, false]),
];

#[derive(Deserialize, Debug, PartialEq)]
pub enum RecipeType {
    Meal,
    Dessert,
    Side,
    Snack,
    Bread,
    Component,
    Drink,
}

#[derive(Deserialize, Debug, PartialEq)]
struct SeasonableIngredient {
    name: String
}

trait Seasonable {
    fn in_season(&self, date: usize) -> bool;
}

impl Seasonable for SeasonableIngredient {
    fn in_season(&self, date: usize) -> bool {
        //TODO replace with binary search
        for i in SEASON_TABLE.iter() {
            let (name, table) = i;
            if name == &self.name.as_str() {
                return table[date];
            }
        }
        panic!("Unknown seasonable ingredient {}", self.name)
    }
}

impl Seasonable for Vec<SeasonableIngredient> {
    fn in_season(&self, date: usize) -> bool {
        self.iter().map(|n| n.in_season(date)).max().unwrap()
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub text: String,
    pub ingredients: String,
    page: u8,
    pub meat: bool,
    seafood: bool,
    protein: bool,
    grain: bool,
    fruit: bool,
    vegetables: bool,
    vegan: bool,
    serves: Option<u8>,
    pub category: RecipeType,
    seasonables: Vec<Vec<SeasonableIngredient>>,
    max_seasonable_percent: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct RecipeBook {
    pub recipes: Vec<Recipe>,
    name: Option<String>,
    author: Option<String>,
}

impl Recipe {
    pub fn seasonable_percent(&self, date: usize) -> f64 {
        if self.seasonables.len() == 0 {
            return 1.0;
        }
        let num_seasonable: f64 = self.seasonables.iter().map(|n| n.in_season(date)).filter(|n| *n).count() as f64;
        num_seasonable / self.seasonables.len() as f64
    }

    pub fn max_seasonable_percent(&self) -> f64 {
//        if self.max_seasonable_percent.is_none() {
        let mut max_seasonable: f64 = 0.0;
        for i in 0..12 {
            max_seasonable = max_seasonable.max(self.seasonable_percent(i));
        }
        max_seasonable
//            self.max_seasonable_percent = Some(max_seasonable);
//        }
//        self.max_seasonable_percent.unwrap()
    }

    pub fn filter(&self, month: Option<usize>, meat: Option<bool>, seafood: Option<bool>,
                  protein: Option<bool>, grain: Option<bool>, fruit: Option<bool>,
                  vegetables: Option<bool>, vegan: Option<bool>, category: Option<&RecipeType>)
                  -> bool {
        (!(month.is_some() && self.seasonable_percent(month.unwrap()) != self.max_seasonable_percent()) &&
            !(meat.is_some() && meat.unwrap() != self.meat) && !(seafood.is_some() && seafood.unwrap() != self.seafood)
            && !(protein.is_some() && protein.unwrap() != self.protein) && !(grain.is_some() && grain.unwrap() != self.grain)
            && !(fruit.is_some() && fruit.unwrap() != self.fruit) && !(vegetables.is_some() && vegetables.unwrap() != self.vegetables)
            && !(vegan.is_some() && vegan.unwrap() != self.vegan) && !(category.is_some() && category.unwrap() != &self.category))
    }
}

impl RecipeBook {
    pub fn deserialize(recipe_book: &str) -> RecipeBook {
        toml::from_str(recipe_book).unwrap()
    }

    pub fn filter(self, month: Option<usize>, meat: Option<bool>, seafood: Option<bool>,
                  protein: Option<bool>, grain: Option<bool>, fruit: Option<bool>,
                  vegetables: Option<bool>, vegan: Option<bool>, category: Option<RecipeType>) -> Vec<Recipe> {
        self.recipes.into_iter().filter(|r| r.filter(month, meat, seafood, protein, grain, fruit, vegetables, vegan, category.as_ref())).collect()
    }
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}\nIngredients:\n{}\nInstructions:\n{}", self.name, self.ingredients, self.text)
    }
}