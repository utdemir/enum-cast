use enum_cast::EnumCast;

#[derive(Debug, EnumCast)]
enum Animal {
    Cat(Cat),
    Dog(Dog),
    Capybara(Capybara),
    Elephant(Elephant),
}

#[derive(Debug, EnumCast)]
enum DomesticAnimal {
    DomesticCat(Cat),
    DomesticDog(Dog),
}

fn simple() {
    // We can convert a DomesticAnimal to Animal
    let domestic = DomesticAnimal::DomesticCat(Cat);
    let animal = domestic.widen::<Animal>();

    // Which returns the Animal with the same payload
    if let Animal::Cat(cat) = &animal {
        println!("Converted to Animal: {:?}", cat);
    }

    // We can convert an Animal back to a DomesticAnimal. This
    // conversion might fail because not all Animals are DomesticAnimals.
    if let Some(domestic_animal) = animal.narrow::<DomesticAnimal>().ok() {
        println!("Converted back to DomesticAnimal: {:?}", domestic_animal);
    }
}

fn main() {
    simple();
}

#[derive(Clone, Debug)]
struct Cat;

#[derive(Clone, Debug)]
struct Dog;

#[derive(Clone, Debug)]
struct Capybara;

#[derive(Clone, Debug)]
struct Elephant;
