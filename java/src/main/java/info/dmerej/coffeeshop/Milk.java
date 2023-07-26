package info.dmerej.coffeeshop;

public class Milk extends Beverage implements CondimentDecorator {
    private final Beverage beverage;

    public Milk(Beverage beverage) {
        this.beverage = beverage;
    }

    @Override
    public String getDescription() {
        return this.beverage.getDescription() + " with milk";
    }

    public int cost() {
        return beverage.cost() + 2;
    }
}
