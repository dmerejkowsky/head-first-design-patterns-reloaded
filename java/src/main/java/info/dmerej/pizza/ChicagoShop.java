package info.dmerej.pizza;

public class ChicagoShop extends Shop {
    @Override
    public Pizza createPizza(String name) {
        var pizza = DefaultFactory.getPizza(name);
        pizza.setCrust("thick");
        return pizza;
    }
}
