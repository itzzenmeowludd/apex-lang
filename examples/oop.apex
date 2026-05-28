#!/usr/bin/env apex
# ========================================
# OBJECT-ORIENTED PROGRAMMING
# Classes, inheritance, encapsulation
# ========================================

# Define a class
class Animal:
    """Base class for all animals."""
    
    name: str
    age: i32
    species: str
    
    fn new(name: str, age: i32, species: str):
        """Constructor."""
        return {
            name: name,
            age: age,
            species: species
        }
    
    fn speak():
        """Make a sound - overridable."""
        print("${this.name} makes a sound")
    
    fn get_info() -> str:
        """Get animal information."""
        return "${this.name} is a ${this.age}-year-old ${this.species}"
    
    fn have_birthday():
        """Increment age."""
        this.age = this.age + 1
        print("🎂 ${this.name} is now ${this.age} years old")

# Subclass - inherit from Animal
class Dog(Animal):
    """A dog that extends Animal."""
    
    breed: str
    
    fn new(name: str, age: i32, breed: str):
        """Constructor for Dog."""
        let dog = {
            name: name,
            age: age,
            species: "Canis familiaris",
            breed: breed
        }
        return dog
    
    fn speak():
        """Dogs bark!"""
        print("🐕 ${this.name} barks: Woof woof! 🐕")
    
    fn fetch(item: str):
        """Dogs can fetch."""
        print("${this.name} fetches the ${item}")

# Another subclass
class Cat(Animal):
    """A cat that extends Animal."""
    
    indoor: bool
    
    fn new(name: str, age: i32, indoor: bool = true):
        """Constructor for Cat."""
        return {
            name: name,
            age: age,
            species: "Felis catus",
            indoor: indoor
        }
    
    fn speak():
        """Cats meow!"""
        print("🐱 ${this.name} meows: Meow meow! 🐱")
    
    fn scratch(object: str):
        """Cats love to scratch."""
        print("${this.name} scratches the ${object}")

# Another class - Person
class Person:
    """Represent a person."""
    
    first_name: str
    last_name: str
    age: i32
    email: str
    pets: []
    
    fn new(first_name: str, last_name: str, age: i32, email: str):
        """Constructor."""
        return {
            first_name: first_name,
            last_name: last_name,
            age: age,
            email: email,
            pets: []
        }
    
    fn get_full_name() -> str:
        """Get full name."""
        return "${this.first_name} ${this.last_name}"
    
    fn add_pet(pet):
        """Add a pet."""
        this.pets.push(pet)
        print("${this.get_full_name()} adopted ${pet.name}!")
    
    fn list_pets():
        """List all pets."""
        if len(this.pets) == 0:
            print("${this.get_full_name()} has no pets")
            return
        
        print("${this.get_full_name()}'s pets:")
        for pet in this.pets:
            print("  • ${pet.name} (${pet.species})")
    
    fn is_adult() -> bool:
        """Check if person is adult."""
        return this.age >= 18

# ========================================
fn main():
    print("=" * 60)
    print("Object-Oriented Programming in Apex")
    print("=" * 60)
    
    # Create instances
    print("\n1️⃣  Creating Objects (Instances)")
    let dog1 = Dog.new("Buddy", 3, "Golden Retriever")
    let cat1 = Cat.new("Whiskers", 5, true)
    let generic_animal = Animal.new("Unknown", 1, "Unknown")
    
    print("Created dog: ${dog1.name}")
    print("Created cat: ${cat1.name}")
    
    # Access properties
    print("\n2️⃣  Accessing Properties")
    print("dog1.name = ${dog1.name}")
    print("dog1.breed = ${dog1.breed}")
    print("dog1.age = ${dog1.age}")
    print("cat1.indoor = ${cat1.indoor}")
    
    # Call methods
    print("\n3️⃣  Calling Methods")
    dog1.speak()
    cat1.speak()
    generic_animal.speak()
    
    # Polymorphism
    print("\n4️⃣  Polymorphism (Same method, different behavior)")
    let animals = [dog1, cat1, generic_animal]
    for animal in animals:
        animal.speak()
    
    # More methods
    print("\n5️⃣  Using Object Methods")
    print("Info: ${dog1.get_info()}")
    print("Info: ${cat1.get_info()}")
    
    dog1.fetch("ball")
    cat1.scratch("couch")
    
    # Modify state
    print("\n6️⃣  Modifying Object State")
    dog1.have_birthday()
    cat1.have_birthday()
    
    # Work with Person class
    print("\n7️⃣  Complex Object: Person with Pets")
    let alice = Person.new("Alice", "Smith", 28, "alice@example.com")
    print("Name: ${alice.get_full_name()}")
    print("Adult: ${alice.is_adult()}")
    
    alice.add_pet(dog1)
    alice.add_pet(cat1)
    alice.list_pets()
    
    # Create more people
    print("\n8️⃣  More Examples")
    let bob = Person.new("Bob", "Johnson", 16, "bob@example.com")
    print("${bob.get_full_name()} is adult: ${bob.is_adult()}")
    
    # Array of objects
    print("\n9️⃣  Array of Objects")
    let people = [alice, bob]
    for person in people:
        print("${person.get_full_name()} (Age: ${person.age})")
    
    # Object comparison
    print("\n🔟 Object Information")
    print("Dog 1: ${dog1}")
    print("Cat 1: ${cat1}")
    
    print("\n✅ OOP in Apex mastered! 🎉")