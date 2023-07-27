package info.dmerej.pizza;

import java.security.InvalidParameterException;

public class DefaultFactory {
    public static Pizza getPizza(String name) {
        var pizza = switch (name) {
            case "cheese" -> new CheesePizza();
            case "pepperoni" -> new PepperoniPizza();
            default -> throw new InvalidParameterException("invalid name: " + name);
        };
        return pizza;
    }
}
