package info.dmerej.coffeeshop;

public class DarkRoast extends Beverage {
    @Override
    public String getDescription() {
        return "dark roast";
    }

    @Override
    int cost() {
        return 10 * getSize();
    }
}
