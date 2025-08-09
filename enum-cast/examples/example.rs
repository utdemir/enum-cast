use enum_cast::{EnumCast, EnumVariantIds};

#[derive(Debug, EnumCast, EnumVariantIds)]
enum Animal {
    Cat(Cat),
    Dog(Dog),
    Capybara(Capybara),
    Elephant(Elephant),
}

#[derive(Debug, EnumCast, EnumVariantIds)]
enum DomesticAnimal {
    DomesticCat(Cat),
    DomesticDog(Dog),
}

fn simple() {
    // We can convert a DomesticAnimal to an Animal. As `Animal` contains all
    // the variants of `DomesticAnimal`, this conversion is not fallible.
    let domestic = DomesticAnimal::DomesticCat(Cat);
    let animal = domestic.upcast::<Animal>();
    if let Animal::Cat(cat) = &animal {
        println!("Converted to Animal: {:?}", cat);
    }

    // We also can convert an Animal back to a DomesticAnimal. This
    // conversion might fail because not all Animals are DomesticAnimals.
    if let Some(domestic_animal) = animal.downcast::<DomesticAnimal>().ok() {
        println!("Converted back to DomesticAnimal: {:?}", domestic_animal);
        if let DomesticAnimal::DomesticCat(cat) = domestic_animal {
            println!("It's a DomesticCat: {:?}", cat);
        }
    }
}

fn variant_ids() {
    // An (admittedly obscure) use case of this crate is to figure out whether
    // a particular variant is present in a target enum. Here's how:
    let cat_animal = Animal::Cat(Cat);
    let capybara_animal = Animal::Capybara(Capybara);

    let domestic_animal_ids = DomesticAnimal::VARIANT_TYPE_IDS;

    let is_cat_domestic = domestic_animal_ids.contains(&cat_animal.current_variant_id());
    let is_capybara_domestic = domestic_animal_ids.contains(&capybara_animal.current_variant_id());

    println!("Is cat domestic animal: {:?}", is_cat_domestic);
    println!("Is capybara domestic animal: {:?}", is_capybara_domestic);
}

fn main() {
    simple();
    variant_ids();
}

#[derive(Clone, Debug)]
struct Cat;

#[derive(Clone, Debug)]
struct Dog;

#[derive(Clone, Debug)]
struct Capybara;

#[derive(Clone, Debug)]
struct Elephant;
