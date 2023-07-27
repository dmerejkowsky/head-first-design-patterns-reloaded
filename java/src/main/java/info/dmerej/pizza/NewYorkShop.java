package info.dmerej.pizza;

public class NewYorkShop extends Shop {
    @Override
    public Pizza createPizza(String name) {
        var pizza = DefaultFactory.getPizza(name);
        pizza.setCrust("thin");
        return pizza;
    }
}
