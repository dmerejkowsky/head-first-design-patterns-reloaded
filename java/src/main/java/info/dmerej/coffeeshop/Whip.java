package info.dmerej.coffeeshop;

public class Whip implements CondimentDecorator {
    private final Beverage beverage;

    public Whip(Beverage beverage) {
        this.beverage = beverage;
    }

    @Override
    public String getDescription() {
        return "whipped " + beverage.getDescription();
    }

    public int cost() {
        return 1 + beverage.cost();
    }
}
