package info.dmerej.coffeeshop;

public class Decaf extends Beverage {


    @Override
    public String getDescription() {
        return "Decaf";
    }

    @Override
    int cost() {
        return 8;
    }
}
