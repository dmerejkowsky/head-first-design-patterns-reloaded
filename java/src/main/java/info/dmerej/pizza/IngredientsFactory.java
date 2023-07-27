package info.dmerej.pizza;

public interface IngredientsFactory {
    Dough createDough();

    Cheese createCheese();

    Sauce createSauce();

    Veggies[] createVeggies();

    Clams createClams();

    default Pepperoni createPepperoni() {
        return new SlicedPepperoni();
    }
}
