package info.dmerej.pizza;

public class CheesePizza extends Pizza {
    private final IngredientsFactory ingredientsFactory;

    public CheesePizza(IngredientsFactory ingredientsFactory) {
        this.ingredientsFactory = ingredientsFactory;
    }

    @Override
    public void prepare() {
        dough = ingredientsFactory.createDough();
        cheese = ingredientsFactory.createCheese();
        System.out.printf("Adding %s and %s\n", dough.getClass().getSimpleName(), cheese.getClass().getSimpleName());
    }

    @Override
    public String name() {
        return "cheese";
    }
}
