package info.dmerej.pizza;

public class CheesePizza extends Pizza {

    @Override
    public void prepare() {
        System.out.println("Adding some cheese");
    }

    @Override
    public String name() {
        return "cheese";
    }
}
