package info.dmerej.menus;

import java.util.Objects;

public final class MenuItem implements MenuComponent {
    private final String name;
    private final int price;
    private final boolean vegetarian;

    public MenuItem(String name, int price, boolean vegetarian) {
        this.name = name;
        this.price = price;
        this.vegetarian = vegetarian;
    }

    @Override
    public String getName() {
        return name;
    }

    @Override
    public int getPrice() {
        return price;
    }

    @Override
    public boolean isVegetarian() {
        return vegetarian;
    }

    @Override
    public void add(MenuComponent component) {
        throw new UnsupportedOperationException();
    }

    @Override
    public MenuComponent getChild(int index) {
        throw new UnsupportedOperationException();
    }

    @Override
    public void print() {
        System.out.println(this);
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == this) return true;
        if (obj == null || obj.getClass() != this.getClass()) return false;
        var that = (MenuItem) obj;
        return Objects.equals(this.name, that.name) &&
            this.price == that.price &&
            this.vegetarian == that.vegetarian;
    }

    @Override
    public int hashCode() {
        return Objects.hash(name, price, vegetarian);
    }

    @Override
    public String toString() {
        return "MenuItem[" +
            "name=" + name + ", " +
            "price=" + price + ", " +
            "vegetarian=" + vegetarian + ']';
    }

}
