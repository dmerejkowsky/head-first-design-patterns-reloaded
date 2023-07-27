package info.dmerej.pizza;

public class CheezePizza extends Pizza {
    @Override
    public void prepare() {
        System.out.println("Adding some cheese");
    }
}
