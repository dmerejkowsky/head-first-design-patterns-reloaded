package info.dmerej.pizza;

public class ChicagoIngredients implements IngredientsFactory {
    @Override
    public Dough createDough() {
        return new ThickCrustDough();
    }

    @Override
    public Cheese createCheese() {
        return new RegianoCheese();
    }

    @Override
    public Sauce createSauce() {
        return new PlumTomatoSauce();
    }

    @Override
    public Veggies[] createVeggies() {
        return new Veggies[]{
            new Eggplant(),
            new Spinach(),
        };
    }

    @Override
    public Clams createClams() {
        return new FrozenClams();
    }
}
