package info.dmerej.pizza;

import java.security.InvalidParameterException;

public class PizzaFactory {
    public Pizza createPizza(String name) {
        switch (name) {
            case "cheese" -> {
                return new CheesePizza();
            }
            case "pepperoni" -> {
                return new PepperoniPizza();
            }
            default -> throw new InvalidParameterException("Unknown pizza: " + name);
        }
    }
}
