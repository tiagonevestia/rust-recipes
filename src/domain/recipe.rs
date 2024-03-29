use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::Entity;

impl Entity for Recipe {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeId(Option<String>);

impl RecipeId {
    pub fn value(&self) -> &Option<String> {
        &self.0
    }
}

impl TryFrom<String> for RecipeId {
    type Error = &'static str;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        if id.is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(id)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecipeName(String);

impl RecipeName {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for RecipeName {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("A receita precisa ter um nome")
        } else {
            Ok(RecipeName(value))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecipeTags(Vec<String>);

impl RecipeTags {
    pub fn value(&self) -> &Vec<String> {
        &self.0
    }
}

impl TryFrom<Vec<String>> for RecipeTags {
    type Error = &'static str;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("A receita precisa pelo menos de uma tag")
        } else {
            Ok(RecipeTags(value))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecipeIngredients(Vec<String>);

impl RecipeIngredients {
    pub fn value(&self) -> &Vec<String> {
        &self.0
    }
}

impl TryFrom<Vec<String>> for RecipeIngredients {
    type Error = &'static str;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("A receita precisa pelo menos de um ingrediente")
        } else {
            Ok(RecipeIngredients(value))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecipeInstructions(Vec<String>);

impl RecipeInstructions {
    pub fn value(&self) -> &Vec<String> {
        &self.0
    }
}

impl TryFrom<Vec<String>> for RecipeInstructions {
    type Error = &'static str;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("A receita precisa pelo menos de uma instrução")
        } else {
            Ok(RecipeInstructions(value))
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: RecipeName,
    pub tags: RecipeTags,
    pub ingredients: RecipeIngredients,
    pub instructions: RecipeInstructions,
    pub published_at: Option<DateTime<Local>>,
}

impl Recipe {
    pub fn new(
        id: String,
        name: String,
        tags: Vec<String>,
        ingredients: Vec<String>,
        instructions: Vec<String>,
    ) -> Result<Self, String> {
        let recipe_id = RecipeId::try_from(id)?;
        let recipe_name = RecipeName::try_from(name)?;
        let recipe_tags = RecipeTags::try_from(tags)?;
        let recipe_ingredients = RecipeIngredients::try_from(ingredients)?;
        let recipe_instructions = RecipeInstructions::try_from(instructions)?;

        Ok(Recipe {
            id: recipe_id,
            name: recipe_name,
            tags: recipe_tags,
            ingredients: recipe_ingredients,
            instructions: recipe_instructions,
            published_at: Some(Local::now()),
        })
    }

    pub fn id(&self) -> &RecipeId {
        &self.id
    }

    pub fn name(&self) -> &RecipeName {
        &self.name
    }

    pub fn tags(&self) -> &RecipeTags {
        &self.tags
    }

    pub fn ingredients(&self) -> &RecipeIngredients {
        &self.ingredients
    }

    pub fn instructions(&self) -> &RecipeInstructions {
        &self.instructions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_the_expected_recipe() {
        let tags = vec!["main".to_string(), "chicken".to_string()];
        let ingredients = vec!["4 (6 to 7-ounce) boneless skinless chicken breasts\r".to_string()];
        let instructions = vec!["To marinate the chicken: In a non-reactive dish, combine the lemon juice, olive oil, oregano, salt, and pepper and mix together".to_string()];
        let name: &str = "Oregano Marinated Chicken";

        let new_recipe = Recipe::new(
            "10".to_string(),
            name.to_string(),
            tags.clone(),
            ingredients.clone(),
            instructions.clone(),
        )
        .unwrap();

        assert_eq!(new_recipe.name.value(), name);
        assert_eq!(tags.len(), new_recipe.tags.0.len());
        assert_eq!(ingredients.len(), new_recipe.ingredients.0.len());
        for (i, exp_ins) in instructions.into_iter().enumerate() {
            assert_eq!(exp_ins, new_recipe.instructions.value()[i])
        }
    }

    #[test]
    fn should_fail_without_a_name_or_ingredients_or_tags_or_instructions() {
        let tags = vec!["main".to_string(), "chicken".to_string()];
        let ingredients = vec!["4 (6 to 7-ounce) boneless skinless chicken breasts\r".to_string()];
        let instructions = vec!["To marinate the chicken: In a non-reactive dish, combine the lemon juice, olive oil, oregano, salt, and pepper and mix together".to_string()];
        let name = "Oregano Marinated Chicken";

        let err_recipe = Recipe::new(
            "10".to_string(),
            "".to_string(),
            tags.clone(),
            ingredients.clone(),
            instructions.clone(),
        );
        assert_eq!(err_recipe.is_err(), true);
        assert_eq!(err_recipe.unwrap_err(), "A receita precisa ter um nome");

        let err_recipe = Recipe::new(
            "10".to_string(),
            name.to_string(),
            vec![],
            ingredients.clone(),
            instructions.clone(),
        );
        assert_eq!(err_recipe.is_err(), true);
        assert_eq!(
            err_recipe.unwrap_err(),
            "A receita precisa pelo menos de uma tag"
        );

        let err_recipe = Recipe::new(
            "10".to_string(),
            name.to_string(),
            tags.clone(),
            vec![],
            instructions.clone(),
        );
        assert_eq!(err_recipe.is_err(), true);
        assert_eq!(
            err_recipe.unwrap_err(),
            "A receita precisa pelo menos de um ingrediente"
        );

        let err_recipe = Recipe::new(
            "10".to_string(),
            name.to_string(),
            tags.clone(),
            ingredients.clone(),
            vec![],
        );
        assert_eq!(err_recipe.is_err(), true);
        assert_eq!(
            err_recipe.unwrap_err(),
            "A receita precisa pelo menos de uma instrução"
        );
    }
}
