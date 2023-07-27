package info.dmerej.pizza;

public class PepperoniPizza extends Pizza {
    @Override
    void prepare() {
        System.out.println("Adding pepperoni");
    }

    @Override
    public String name() {
        return "pepperoni";
    }
}
