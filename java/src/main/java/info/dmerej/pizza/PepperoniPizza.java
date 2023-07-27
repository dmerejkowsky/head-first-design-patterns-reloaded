package info.dmerej.pizza;

public class PepperoniPizza extends Pizza {
    private final IngredientsFactory ingredientsFactory;

    public PepperoniPizza(IngredientsFactory ingredientsFactory) {
        this.ingredientsFactory = ingredientsFactory;
    }

    @Override
    void prepare() {
        dough = ingredientsFactory.createDough();
        pepperoni = ingredientsFactory.createPepperoni();
        System.out.printf("Adding %s and %s\n", dough.getClass().getSimpleName(), pepperoni.getClass().getSimpleName());

    }

    @Override
    public String name() {
        return "pepperoni";
    }
}
