package info.dmerej.pizza;

public abstract class Shop {

    public abstract Pizza createPizza(String name);

    public Pizza orderPizza(String name) {
        Pizza pizza = createPizza(name);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.box();
        return pizza;
    }
}
