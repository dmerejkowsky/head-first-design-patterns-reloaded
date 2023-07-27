package info.dmerej.pizza;

public class NewYorkIngredients implements IngredientsFactory {
    @Override
    public Dough createDough() {
        return new ThinCrustDough();
    }

    @Override
    public Cheese createCheese() {
        return new Mozarella();
    }

    @Override
    public Sauce createSauce() {
        return new MarinaSauce();
    }

    @Override
    public Veggies[] createVeggies() {
        return new Veggies[]{
            new Garlic(),
            new Oignon(),
            new Pepper(),
        };
    }

    @Override
    public Clams createClams() {
        return new FreshClams();
    }
}
