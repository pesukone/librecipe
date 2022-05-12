use fraction::Fraction;

#[derive(Debug)]
pub enum Unit {
    Tbsp,
    Tsp,
    Cup,
    Dl,
    L 
}

#[derive(Debug)]
pub struct Ingredient {
    pub name: String,
}

#[derive(Debug)]
pub struct IngredientAmount {
    pub ingredient: Ingredient,
    pub unit: Unit,
    pub amount: Fraction,
}

#[derive(Debug)]
pub struct Recipe {
    pub ingredients: Vec<IngredientAmount>,
    pub steps: Vec<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
