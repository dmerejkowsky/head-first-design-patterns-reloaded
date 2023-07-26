package info.dmerej.coffeeshop;

public abstract class Beverage {
    private int size = 1;

    public abstract String getDescription();

    abstract int cost();

    public int getSize() {
        return size;
    }

    public void setSize(int size) {
        this.size = size;
    }
}
