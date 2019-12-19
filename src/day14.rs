use std::collections::HashMap;

type RecipeBook = HashMap<String, (u32, Vec<(String, u32)>)>;
type ChemicalStore = HashMap<String, u32>;

#[derive(Debug)]
struct Reactor {
    recipes: RecipeBook,
    store: ChemicalStore,
}

impl Reactor {
    pub fn new() -> Reactor {
        Reactor {
            recipes: HashMap::new(),
            store: HashMap::new(),
        }
    }

    fn parse_ingredient(txt: &str) -> (String, u32) {
        let left_right: Vec<&str> = txt.split(" ").collect();
        (String::from(left_right[1]), left_right[0].parse().unwrap())
    }

    pub fn load_recipe_from_string(&mut self, txt: &str) {
        let left_right: Vec<&str> = txt.split("=>").collect();
        let input_list: Vec<&str> = left_right[0].split(",").map(|x| x.trim()).collect();
        let output = left_right[1].trim();

        let parsed_output = Self::parse_ingredient(output);

        let ingredients = input_list
            .iter()
            .map(|&x| Self::parse_ingredient(x))
            .collect();

        self.recipes
            .insert(parsed_output.0, (parsed_output.1, ingredients));
    }

    fn store_chemcial(store: &mut ChemicalStore, chemical: &str, count: u32) {
        if !store.contains_key(chemical) {
            store.insert(String::from(chemical), 0);
        }
        *store.get_mut(chemical).unwrap() += count;
    }

    fn count_stored_chemical(store: &ChemicalStore, chemical: &str) -> u32 {
        if !store.contains_key(chemical) {
            0
        } else {
            store[chemical]
        }
    }

    fn consume_chemical(store: &mut ChemicalStore, chemical: &str, count: u32) {
        if !store.contains_key(chemical) || store[chemical] < count {
            panic!();
        }
        *store.get_mut(chemical).unwrap() -= count;
    }

    fn produce_chemical(
        recipes: &RecipeBook,
        store: &mut ChemicalStore,
        chemical: &str,
        count: u32,
    ) -> u32 {
        if chemical == "ORE" {
            Self::store_chemcial(store, "ORE", count);
            return count;
        }

        let mut ore_used = 0u32;
        let (recipe_out_count, ingredients) = &recipes[chemical];

        while Self::count_stored_chemical(store, chemical) < count {
            for (in_chem, in_count) in ingredients {
                ore_used += Self::produce_chemical(recipes, store, &in_chem, *in_count);
            }
            Self::store_chemcial(store, chemical, *recipe_out_count);
        }

        Self::consume_chemical(store, chemical, count);

        ore_used
    }

    pub fn get_cost_in_ore(&mut self, chemical: &str, count: u32) -> u32 {
        self.store = HashMap::new();
        Self::produce_chemical(&self.recipes, &mut self.store, chemical, count)
    }
}

pub fn main() {
    let mut reactor = Reactor::new();

    std::fs::read_to_string("data/day14.txt")
        .unwrap()
        .lines()
        .for_each(|x| reactor.load_recipe_from_string(&x));

    let result0 = reactor.get_cost_in_ore("FUEL", 1);

    println!("{}", result0);
}
