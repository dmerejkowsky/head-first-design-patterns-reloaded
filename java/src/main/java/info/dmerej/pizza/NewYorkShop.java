package info.dmerej.pizza;

import java.security.InvalidParameterException;

public class NewYorkShop extends Shop {
    @Override
    public Pizza createPizza(String name) {
        var ingredientsFactory = new NewYorkIngredients();
        var pizza = switch (name) {
            case "cheese" -> new CheesePizza(ingredientsFactory);
            case "pepperoni" -> new PepperoniPizza(ingredientsFactory);
            default -> throw new InvalidParameterException("invalid name: " + name);
        };
        return pizza;
    }
}
